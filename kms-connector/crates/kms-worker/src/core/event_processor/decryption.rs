use crate::core::{
    config::{Config, HostChainKind},
    event_processor::{ProcessingError, context::ContextManager, s3::S3Service},
    solana_acl::{SolanaAclVerifier, SolanaPubkeyBytes, decode_acl_record_witness},
};
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Address, Bytes, FixedBytes, U256, map::DefaultHashBuilder},
    providers::Provider,
    sol_types::{Eip712Domain, SolCall},
};
use anyhow::anyhow;
use base64::Engine as _;
use connector_utils::types::{
    KmsGrpcRequest, extra_data::parse_extra_data, handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, HandleEntry, SnsCiphertextMaterial,
    UserDecryptionRequest_1 as UserDecryptionRequestV2, delegatedUserDecryptionRequestCall,
    userDecryptionRequest_1Call as userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use futures::future::{join_all, try_join_all};
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::info;
use user_decryption_signature::{compute_user_decrypt_digest, verify_signature};

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider, HP: Provider, C> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The entity used to validate KMS context.
    context_manager: C,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The instances of the host chains `ACL` contracts used to check the decryption ACL.
    acl_contracts: HashMap<u64, ACLInstance<HP>>,

    /// The ACL backend configured for each host chain.
    host_chain_kinds: HashMap<u64, HostChainKind>,

    /// Per-Solana-host RPC + verifier used to authorize decryption by reading the zama-host
    /// ACL record directly from the validator (the trusted source), rather than trusting the
    /// gateway-conveyed `extraData` witness.
    solana_acl: HashMap<u64, SolanaAclChainConfig>,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<GP>,

    /// Gas cap for the `IERC1271.isValidSignature` static call (RFC-012).
    erc1271_gas_limit: u64,
}

/// RPC endpoint + ACL verifier for a single Solana host chain.
#[derive(Clone)]
struct SolanaAclChainConfig {
    /// Validator JSON-RPC endpoint the worker reads ACL records from.
    rpc_url: String,
    /// `zama-host` program id, base58-encoded for `getProgramAccounts`.
    program_id_base58: String,
    /// Verifier bound to the expected `zama-host` program id.
    verifier: SolanaAclVerifier,
}

impl<GP, HP, C> DecryptionProcessor<GP, HP, C>
where
    GP: Provider,
    HP: Provider,
    C: ContextManager,
{
    pub fn new(
        config: &Config,
        context_manager: C,
        gateway_provider: GP,
        acl_contracts: HashMap<u64, ACLInstance<HP>>,
        s3_service: S3Service<GP>,
    ) -> Self {
        let domain = Eip712DomainMsg {
            name: config.decryption_contract.domain_name.clone(),
            version: config.decryption_contract.domain_version.clone(),
            chain_id: U256::from(config.gateway_chain_id).to_be_bytes_vec(),
            verifying_contract: config.decryption_contract.address.to_string(),
            salt: None,
        };
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, gateway_provider);
        let host_chain_kinds = config
            .host_chains
            .iter()
            .map(|host_chain| (host_chain.chain_id, host_chain.chain_kind))
            .collect();
        // A Solana host chain authorizes decryption via its on-chain ACL record; record the
        // validator RPC + the expected program id so the worker can read and verify it.
        let solana_acl = config
            .host_chains
            .iter()
            .filter(|hc| hc.chain_kind == HostChainKind::Solana)
            .filter_map(|hc| {
                hc.solana_host_program_id.map(|program_id| {
                    (
                        hc.chain_id,
                        SolanaAclChainConfig {
                            rpc_url: hc.url.to_string(),
                            program_id_base58: solana_pubkey::Pubkey::new_from_array(program_id)
                                .to_string(),
                            verifier: SolanaAclVerifier::new(program_id),
                        },
                    )
                })
            })
            .collect();
        Self {
            domain,
            context_manager,
            decryption_contract,
            acl_contracts,
            host_chain_kinds,
            solana_acl,
            s3_service,
            erc1271_gas_limit: config.erc1271_gas_limit,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        _extra_data: &Bytes,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            // Solana authorizes public decryption via its on-chain ACL record, read directly
            // from the validator (the trusted source) and verified with secp/account-witness
            // semantics — not the gateway-conveyed `extraData`.
            if self.host_chain_kind(ct_chain_id) == HostChainKind::Solana {
                self.verify_solana_public_decrypt_allowed(ct_chain_id, ct.ctHandle.0)
                    .await?;
                continue;
            }

            let Some(acl_contract) = self.acl_contracts.get(&ct_chain_id) else {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "No ACL contract config found for chain id {ct_chain_id}"
                )));
            };

            if !acl_contract
                .isAllowedForDecryption(ct.ctHandle)
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?
            {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "{} is not allowed for decrypt!",
                    hex::encode(ct.ctHandle)
                )));
            }
        }

        info!("ACL check passed for {} handles!", sns_ciphertexts.len());
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_user_decryption(
        &self,
        calldata: Vec<u8>,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        user_address: Address,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        let (ct_handle_contract_pairs, delegator_address, _extra_data) =
            match delegatedUserDecryptionRequestCall::abi_decode(calldata.as_slice()) {
                Ok(parsed_calldata) => (
                    parsed_calldata.ctHandleContractPairs,
                    Some(parsed_calldata.delegationAccounts.delegatorAddress),
                    parsed_calldata.extraData,
                ),
                Err(e) => {
                    let parsed_calldata = userDecryptionRequestCall::abi_decode(
                        calldata.as_slice(),
                    )
                    .map_err(|e2| {
                        ProcessingError::Irrecoverable(anyhow!(
                            "Was not able to parse calldata for both userDecryptionRequestCall {e2} \
                            and delegatedUserDecryptionRequestCall ({e})!"
                        ))
                    })?;
                    (
                        parsed_calldata.ctHandleContractPairs,
                        None,
                        parsed_calldata.extraData,
                    )
                }
            };

        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            if self.host_chain_kind(ct_chain_id) == HostChainKind::Solana {
                return Err(Self::reject_solana_gateway_decryption(ct_chain_id));
            }

            let acl_contract = self.acl_contracts.get(&ct_chain_id).ok_or_else(|| {
                ProcessingError::Recoverable(anyhow!(
                    "No ACL contract config found for chain id {ct_chain_id}"
                ))
            })?;
            let contract_address = contracts_map.get(ct.ctHandle.as_slice()).ok_or_else(|| {
                ProcessingError::Irrecoverable(anyhow!(
                    "Could not find contract address for handle {}",
                    hex::encode(ct.ctHandle)
                ))
            })?;

            if let Some(delegator_addr) = delegator_address {
                self.inner_acl_check_for_delegated_user_decryption(
                    acl_contract,
                    ct.ctHandle,
                    user_address,
                    *contract_address,
                    delegator_addr,
                )
                .await?;
            } else {
                self.inner_acl_check_for_user_decryption(
                    acl_contract,
                    ct.ctHandle,
                    user_address,
                    *contract_address,
                )
                .await?;
            }
        }

        info!("ACL check passed for {} handles!", sns_ciphertexts.len());
        Ok(())
    }

    /// Authorizes a Solana public decryption by reading the handle's `zama-host` ACL record
    /// directly from the validator (trusted) and verifying it permits public decrypt with the
    /// program-bound [`SolanaAclVerifier`] — the RPC-verified authorization the gateway path
    /// cannot supply via `extraData`.
    async fn verify_solana_public_decrypt_allowed(
        &self,
        chain_id: u64,
        handle: SolanaPubkeyBytes,
    ) -> Result<(), ProcessingError> {
        let cfg = self.solana_acl.get(&chain_id).ok_or_else(|| {
            ProcessingError::Irrecoverable(anyhow!(
                "No Solana ACL config (rpc/program id) for host chain {chain_id}"
            ))
        })?;
        let (account_key, owner, data) = Self::fetch_solana_acl_record(cfg, handle)
            .await
            .map_err(ProcessingError::Recoverable)?;
        let record = decode_acl_record_witness(account_key, owner, &data)
            .map_err(|e| ProcessingError::Recoverable(anyhow!("decode Solana ACL record: {e}")))?;
        cfg.verifier
            .verify_public_decrypt(&record, handle)
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!(
                    "{} is not allowed for Solana public decrypt: {e}",
                    hex::encode(handle)
                ))
            })?;
        Ok(())
    }

    /// ACL check for Solana user decryption: every handle must grant `subject` (the requesting
    /// ed25519 user) the USE role on its on-chain `zama-host` ACL record, read directly from the
    /// validator and verified with the program-bound [`SolanaAclVerifier`]. The relayer's ed25519
    /// `signMessage` proves request ownership, not decrypt permission, and the gateway `extraData`
    /// witness is attacker-controlled, so authorization must be enforced here. Fail-closed: a
    /// non-Solana handle, missing ACL config, or an absent grant rejects the whole request.
    pub async fn check_ciphertexts_allowed_for_solana_user_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        subject: SolanaPubkeyBytes,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting Solana user-decrypt ACL check for {} handles...",
            sns_ciphertexts.len()
        );
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            if self.host_chain_kind(ct_chain_id) != HostChainKind::Solana {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "UserDecryptionSolana handle {} is not on a Solana host chain ({ct_chain_id})",
                    hex::encode(ct.ctHandle)
                )));
            }
            self.verify_solana_user_decrypt_allowed(ct_chain_id, ct.ctHandle.0, subject)
                .await?;
        }
        info!(
            "Solana user-decrypt ACL check passed for {} handles!",
            sns_ciphertexts.len()
        );
        Ok(())
    }

    /// Authorizes a Solana user decryption by reading the handle's `zama-host` ACL record directly
    /// from the validator (trusted) and verifying it grants `subject` the USE role with the
    /// program-bound [`SolanaAclVerifier`] — the RPC-verified authorization the gateway `extraData`
    /// cannot supply, mirroring [`Self::verify_solana_public_decrypt_allowed`].
    async fn verify_solana_user_decrypt_allowed(
        &self,
        chain_id: u64,
        handle: SolanaPubkeyBytes,
        subject: SolanaPubkeyBytes,
    ) -> Result<(), ProcessingError> {
        let cfg = self.solana_acl.get(&chain_id).ok_or_else(|| {
            ProcessingError::Irrecoverable(anyhow!(
                "No Solana ACL config (rpc/program id) for host chain {chain_id}"
            ))
        })?;
        let (account_key, owner, data) = Self::fetch_solana_acl_record(cfg, handle)
            .await
            .map_err(ProcessingError::Recoverable)?;
        let record = decode_acl_record_witness(account_key, owner, &data)
            .map_err(|e| ProcessingError::Recoverable(anyhow!("decode Solana ACL record: {e}")))?;
        cfg.verifier
            .verify_user_decrypt_subject(&record, &[], handle, subject)
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!(
                    "user {} is not allowed to user-decrypt {} on Solana: {e}",
                    hex::encode(subject),
                    hex::encode(handle)
                ))
            })?;
        Ok(())
    }

    /// Fetches the canonical `zama-host` ACL record for `handle` via `getProgramAccounts`,
    /// filtering on the record's `handle` field (offset 8, after the 8-byte Anchor
    /// discriminator). Returns (account key, owner, raw account data).
    async fn fetch_solana_acl_record(
        cfg: &SolanaAclChainConfig,
        handle: SolanaPubkeyBytes,
    ) -> anyhow::Result<(SolanaPubkeyBytes, SolanaPubkeyBytes, Vec<u8>)> {
        let handle_b64 = base64::engine::general_purpose::STANDARD.encode(handle);
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getProgramAccounts",
            "params": [cfg.program_id_base58, {
                "encoding": "base64",
                "filters": [{"memcmp": {"offset": 8, "bytes": handle_b64, "encoding": "base64"}}]
            }]
        });
        let resp: serde_json::Value = reqwest::Client::new()
            .post(&cfg.rpc_url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        let account = resp
            .get("result")
            .and_then(|r| r.as_array())
            .and_then(|a| a.first())
            .ok_or_else(|| {
                anyhow!(
                    "no zama-host ACL record found for handle {}",
                    hex::encode(handle)
                )
            })?;
        let pubkey = account
            .get("pubkey")
            .and_then(|p| p.as_str())
            .ok_or_else(|| anyhow!("getProgramAccounts: missing pubkey"))?;
        let inner = account
            .get("account")
            .ok_or_else(|| anyhow!("getProgramAccounts: missing account"))?;
        let owner = inner
            .get("owner")
            .and_then(|o| o.as_str())
            .ok_or_else(|| anyhow!("getProgramAccounts: missing owner"))?;
        let data_b64 = inner
            .get("data")
            .and_then(|d| d.as_array())
            .and_then(|a| a.first())
            .and_then(|s| s.as_str())
            .ok_or_else(|| anyhow!("getProgramAccounts: missing account data"))?;
        let data = base64::engine::general_purpose::STANDARD.decode(data_b64)?;
        let account_key = solana_pubkey::Pubkey::from_str(pubkey)
            .map_err(|e| anyhow!("invalid ACL record pubkey {pubkey}: {e}"))?
            .to_bytes();
        let owner = solana_pubkey::Pubkey::from_str(owner)
            .map_err(|e| anyhow!("invalid ACL record owner {owner}: {e}"))?
            .to_bytes();
        Ok((account_key, owner, data))
    }

    fn host_chain_kind(&self, chain_id: u64) -> HostChainKind {
        self.host_chain_kinds
            .get(&chain_id)
            .copied()
            .unwrap_or(HostChainKind::Evm)
    }

    /// Fails Solana decryption authorization closed on the Gateway request path.
    ///
    /// The Gateway request carries ACL/material account witnesses inside the
    /// requester-controlled `extraData`. Those bytes are not on-chain truth: an
    /// attacker can set the witness `owner` field to the host program id and
    /// hand-craft an `AclRecord`/`HandleMaterialCommitment` body that names
    /// themselves as a subject (or sets `public_decrypt = true`), passing every
    /// self-consistency check (canonical PDA, nonce key, material hash) because
    /// they are all recomputed from the same attacker-supplied fields. Trusting
    /// this payload therefore authorizes decryption of any Solana handle.
    ///
    /// Solana authorization must instead be verified against account state read
    /// from the chain by the native-v0 flow (`solana_live`/`solana_flow`), which
    /// derives `owner`/`data`/`observed_slot` from a finalized RPC snapshot. Until
    /// that flow is wired into this processor, Solana decryption is refused rather
    /// than authorized from untrusted witnesses. This mirrors the existing
    /// fail-closed behavior for Solana host chains configured without a host
    /// program id (see `KmsWorker::from_config`).
    fn reject_solana_gateway_decryption(chain_id: u64) -> ProcessingError {
        ProcessingError::Irrecoverable(anyhow!(
            "Refusing decryption for Solana host chain {chain_id}: Gateway `extraData` ACL \
             witnesses are not a trusted authorization source. Solana handles must be authorized \
             via the RPC-verified native-v0 flow, which is not wired into this processor. Failing \
             closed."
        ))
    }

    async fn inner_acl_check_for_delegated_user_decryption(
        &self,
        acl_contract: &ACLInstance<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
        delegator_address: Address,
    ) -> Result<(), ProcessingError> {
        let handle_hex = hex::encode(handle);
        let is_delegated = acl_contract
            .isHandleDelegatedForUserDecryption(
                delegator_address,
                user_address,
                contract_address,
                handle,
            )
            .call()
            .await
            .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;

        if !is_delegated {
            return Err(ProcessingError::Recoverable(anyhow!(
                "{user_address} is not a delegate of {delegator_address} for contract \
                    {contract_address} and handle {handle_hex}!",
            )));
        }

        Ok(())
    }

    /// Verify that a `UserDecryptionRequestV2` is internally consistent before the ACL phase:
    /// `handles` and `snsCtMaterials` are pairwise aligned, and every handle resolves to the
    /// same host chain id. Returns that shared chain id.
    fn validate_handles_and_extract_chain_id(
        request: &UserDecryptionRequestV2,
    ) -> Result<u64, ProcessingError> {
        if request.handles.len() != request.snsCtMaterials.len() {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "handles/snsCtMaterials length mismatch: {} vs {}",
                request.handles.len(),
                request.snsCtMaterials.len(),
            )));
        }

        let chain_id = request
            .handles
            .first()
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("request contains no handles")))
            .map(|h| extract_chain_id_from_handle(h.handle.as_slice()))?
            .map_err(ProcessingError::Irrecoverable)?;

        for (i, (h, m)) in request
            .handles
            .iter()
            .zip(request.snsCtMaterials.iter())
            .enumerate()
        {
            if h.handle != m.ctHandle {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "handles[{i}].handle ({}) != snsCtMaterials[{i}].ctHandle ({})",
                    h.handle,
                    m.ctHandle,
                )));
            }
            match extract_chain_id_from_handle(h.handle.as_slice()) {
                Ok(id) if id == chain_id => (),
                Ok(other) => {
                    return Err(ProcessingError::Irrecoverable(anyhow!(
                        "user decryption request handles span multiple chains ({chain_id}, {other})",
                    )));
                }
                Err(e) => {
                    return Err(ProcessingError::Irrecoverable(anyhow!(
                        "Failed to extract chain_id from handle {}: {e}",
                        hex::encode(h.handle),
                    )));
                }
            }
        }

        Ok(chain_id)
    }

    /// RFC016 unified user decryption check — verifies the full ACL authorization for a
    /// `UserDecryptionRequestV2` payload.
    ///
    /// 1. validity window (`startTimestamp <= now <= startTimestamp + durationSeconds`)
    /// 2. `userAddress ∉ allowedContracts` when `allowedContracts` is non-empty
    /// 3. concurrent host-chain checks (one RPC round-trip wave):
    ///    - EIP-712 signature verification with `ecrecover` → ERC-1271 fallback (RFC-012)
    ///    - signature invalidation: `startTimestamp >= ACL.decryptionSignatureInvalidatedBefore(userAddress)`
    ///    - per-handle ownership (direct `isAllowed` if `ownerAddress == userAddress`, else
    ///      `isHandleDelegatedForUserDecryption`)
    ///    - per-handle contract allowance (any `isAllowed(handle, c)` for
    ///      `c ∈ allowedContracts`, no-op in permissive mode)
    #[tracing::instrument(skip_all)]
    pub async fn check_user_decryption_request_v2(
        &self,
        request: &UserDecryptionRequestV2,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting RFC016 check for {} handles...",
            request.handles.len()
        );

        let chain_id = Self::validate_handles_and_extract_chain_id(request)?;

        let payload = &request.payload;

        // Validity window
        let start = payload.requestValidity.startTimestamp;
        let now = U256::from(Utc::now().timestamp() as u64);
        let end = start.saturating_add(payload.requestValidity.durationSeconds);
        if now < start {
            return Err(ProcessingError::Recoverable(anyhow!(
                "RFC016 user decryption request not yet valid: now {now} < startTimestamp {start}",
            )));
        }
        if now > end {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "RFC016 user decryption request validity window expired: now {now} > end {end}"
            )));
        }

        // `userAddress` must not appear in a non-empty `allowedContracts` list.
        if payload.allowedContracts.contains(&payload.userAddress) {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "userAddress {} is listed in allowedContracts — request rejected",
                payload.userAddress
            )));
        }

        let acl_contract = self.acl_contracts.get(&chain_id).ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "No ACL contract config found for chain id {chain_id}"
            ))
        })?;

        // RFC-012: EIP-712 signature verification with ecrecover → ERC-1271 fallback.
        // The domain takes name/version/verifyingContract from `self.domain` (already validated
        // at startup) but substitutes the host `contractsChainId` for the Gateway chain id —
        // `self.domain` targets KMS gRPC requests, the user-decryption signature targets the
        // host chain.
        let domain = Eip712Domain {
            name: Some(self.domain.name.clone().into()),
            version: Some(self.domain.version.clone().into()),
            chain_id: Some(U256::from(chain_id)),
            verifying_contract: Some(*self.decryption_contract.address()),
            salt: None,
        };
        let digest = compute_user_decrypt_digest(payload, &domain);

        // Signature verification, invalidation, and per-handle ACL checks are all independent
        // host-chain reads. Fire them concurrently so the smart-account happy path is faster.
        // `biased;` polls branches in order so tests can deterministically craft the mock-queue
        // order.
        tokio::try_join!(
            biased;
            async {
                verify_signature(
                    acl_contract.provider(),
                    payload.userAddress,
                    digest,
                    payload.signature.as_ref(),
                    self.erc1271_gas_limit,
                )
                .await
                .map_err(ProcessingError::from)
            },
            self.inner_invalidation_check_for_user_decryption_v2(
                acl_contract,
                payload.userAddress,
                start,
            ),
            try_join_all(request.handles.iter().map(|handle_entry| async move {
                tokio::try_join!(
                    biased;
                    self.inner_ownership_check_for_user_decryption_v2(
                        acl_contract,
                        handle_entry,
                        payload.userAddress,
                    ),
                    self.inner_allowed_contracts_check_for_user_decryption_v2(
                        acl_contract,
                        handle_entry.handle,
                        &payload.allowedContracts,
                    ),
                )
            })),
        )?;

        info!(
            "RFC016 ACL check passed for {} handles!",
            request.handles.len()
        );
        Ok(())
    }

    /// RFC016 per-handle ownership check. Direct path (`ownerAddress == userAddress`) calls
    /// `isAllowed(handle, userAddress)`; delegated path calls
    /// `isHandleDelegatedForUserDecryption(ownerAddress, userAddress, contractAddress, handle)`.
    async fn inner_ownership_check_for_user_decryption_v2(
        &self,
        acl_contract: &ACLInstance<HP>,
        entry: &HandleEntry,
        user_address: Address,
    ) -> Result<(), ProcessingError> {
        let handle_hex = hex::encode(entry.handle);
        if entry.ownerAddress == user_address {
            let user_allowed = acl_contract
                .isAllowed(entry.handle, user_address)
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;
            if !user_allowed {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "{user_address} is not allowed to decrypt {handle_hex}",
                )));
            }
        } else {
            let is_delegated = acl_contract
                .isHandleDelegatedForUserDecryption(
                    entry.ownerAddress,
                    user_address,
                    entry.contractAddress,
                    entry.handle,
                )
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;
            if !is_delegated {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "{user_address} is not a delegate of {} for contract {} and handle {handle_hex}",
                    entry.ownerAddress,
                    entry.contractAddress,
                )));
            }
        }
        Ok(())
    }

    /// RFC016 per-handle `allowedContracts` check — succeeds if at least one contract in the list
    /// has `isAllowed(handle, contract)` returning true. Returns `Ok(())` without any RPC call in
    /// permissive mode (empty list) so callers can invoke it unconditionally.
    async fn inner_allowed_contracts_check_for_user_decryption_v2(
        &self,
        acl_contract: &ACLInstance<HP>,
        handle: FixedBytes<32>,
        allowed_contracts: &[Address],
    ) -> Result<(), ProcessingError> {
        if allowed_contracts.is_empty() {
            return Ok(());
        }

        let calls = allowed_contracts
            .iter()
            .map(|c| async move { acl_contract.isAllowed(handle, *c).call().await });
        let results = join_all(calls).await;

        // Short-circuit on first positive. Individual transport errors are tolerated as long as at
        // least one contract returns true.
        if results.iter().any(|r| matches!(r, Ok(true))) {
            Ok(())
        } else {
            Err(ProcessingError::Recoverable(anyhow!(
                "No contract in allowedContracts is allowed to decrypt handle {} ({:?})",
                hex::encode(handle),
                results,
            )))
        }
    }

    /// RFC016 signature invalidation check. Rejects if `startTimestamp < invalidationTs`, meaning
    /// the user has invalidated all signatures issued before `invalidationTs`.
    async fn inner_invalidation_check_for_user_decryption_v2(
        &self,
        acl_contract: &ACLInstance<HP>,
        user_address: Address,
        start_timestamp: U256,
    ) -> Result<(), ProcessingError> {
        let invalidation_ts = acl_contract
            .decryptionSignatureInvalidatedBefore(user_address)
            .call()
            .await
            .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;
        if start_timestamp < invalidation_ts {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "RFC016 signature invalidated: startTimestamp {start_timestamp} < \
                 invalidatedBefore {invalidation_ts} for userAddress {user_address}"
            )));
        }
        Ok(())
    }

    async fn inner_acl_check_for_user_decryption(
        &self,
        acl_contract: &ACLInstance<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
    ) -> Result<(), ProcessingError> {
        let handle_hex = hex::encode(handle);
        let user_allowed_call = acl_contract.isAllowed(handle, user_address);
        let contract_allowed_call = acl_contract.isAllowed(handle, contract_address);

        let (user_allowed, contract_allowed) =
            tokio::try_join!(biased; user_allowed_call.call(), contract_allowed_call.call())
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;

        if !user_allowed {
            return Err(ProcessingError::Recoverable(anyhow!(
                "{user_address} is not allowed to decrypt {handle_hex}!",
            )));
        }
        if !contract_allowed {
            return Err(ProcessingError::Recoverable(anyhow!(
                "{contract_address} is not allowed to decrypt {handle_hex}!",
            )));
        }

        Ok(())
    }

    pub async fn prepare_decryption_request(
        &self,
        decryption_id: U256,
        sns_materials: &[SnsCiphertextMaterial],
        extra_data: &Bytes,
        user_decrypt_data: Option<UserDecryptionExtraData>,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        // Extract keyId from the first SNS ciphertext material if available
        let key_id = sns_materials
            .first()
            .map(|m| hex::encode(m.keyId.to_be_bytes::<32>()))
            .ok_or_else(|| {
                ProcessingError::Irrecoverable(anyhow!(
                    "No snsCtMaterials found, cannot proceed without a valid key_id"
                ))
            })?;
        info!("Extracted key_id {key_id} from snsCtMaterials[0]");

        let parsed_extra_data =
            parse_extra_data(extra_data).map_err(ProcessingError::Irrecoverable)?;
        if let Some(context_id) = parsed_extra_data.context_id {
            self.context_manager.validate_context(context_id).await?;
        }
        // TODO: validation of epoch_id during RFC-005 implementation

        let ciphertexts = self.prepare_ciphertexts(&key_id, sns_materials).await?;

        let request_id = Some(u256_to_request_id(decryption_id));
        let kms_extra_data = kms_decryption_extra_data(extra_data);

        if let Some(user_decrypt_data) = user_decrypt_data {
            // RFC-021: a Solana user identity is encoded as "solana:<hex pubkey>", which the KMS
            // parses to route through its Solana branch (compute_link_solana + solana_acl). EVM
            // identities use the checksummed 20-byte address.
            let client_address = match user_decrypt_data.solana_user_address {
                Some(solana_user_address) => {
                    format!("solana:{}", hex::encode(solana_user_address))
                }
                None => user_decrypt_data.user_address.to_checksum(None),
            };
            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                enc_key,
                typed_ciphertexts: ciphertexts,
                extra_data: kms_extra_data,
                epoch_id: None,
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            };

            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                extra_data: kms_extra_data,
                epoch_id: None,
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
            };
            Ok(public_decryption_request.into())
        }
    }

    async fn prepare_ciphertexts(
        &self,
        key_id: &str,
        sns_materials: &[SnsCiphertextMaterial],
    ) -> Result<Vec<TypedCiphertext>, ProcessingError> {
        let sns_ciphertext_materials = self
            .s3_service
            .retrieve_sns_ciphertext_materials(sns_materials)
            .await
            .map_err(ProcessingError::Recoverable)?;

        if sns_ciphertext_materials.is_empty() {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "Failed to retrieve any ciphertext materials"
            )));
        }

        // Extract and log FHE types for all ciphertexts
        let fhe_types: Vec<_> = sns_ciphertext_materials
            .iter()
            .map(|ct| ct.fhe_type)
            .collect();

        info!(
            "Processing {} ciphertexts, key_id: {}, FHE types: {:?}",
            sns_ciphertext_materials.len(),
            key_id,
            fhe_types,
        );

        Ok(sns_ciphertext_materials)
    }

    pub async fn fetch_calldata(
        &self,
        tx_hash: FixedBytes<32>,
    ) -> Result<Vec<u8>, ProcessingError> {
        self.decryption_contract
            .provider()
            .get_transaction_by_hash(tx_hash)
            .await
            .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?
            .ok_or_else(|| {
                ProcessingError::Irrecoverable(anyhow!("No transaction found with hash {tx_hash}!"))
            })
            .map(|tx| tx.input().to_vec())
    }
}

fn kms_decryption_extra_data(extra_data: &Bytes) -> Vec<u8> {
    // relayer-sdk <=0.4.2 sends 0x00 but verifies the KMS signature against empty extraData.
    if extra_data.as_ref() == [0x00] {
        Vec::new()
    } else {
        extra_data.to_vec()
    }
}

pub struct UserDecryptionExtraData {
    pub user_address: Address,
    pub public_key: Bytes,
    /// Set for RFC-021 (Solana) user decryptions: the 32-byte Solana pubkey. When present, the
    /// gRPC `client_address` is built as `"solana:<hex>"` so the KMS routes through its Solana
    /// branch (compute_link_solana + solana_acl); the 20-byte `user_address` is then unused.
    pub solana_user_address: Option<FixedBytes<32>>,
}

impl UserDecryptionExtraData {
    pub fn new(user_address: Address, public_key: Bytes) -> Self {
        Self {
            user_address,
            public_key,
            solana_user_address: None,
        }
    }

    /// Constructor for an RFC-021 (Solana) user decryption carrying the 32-byte Solana pubkey.
    pub fn new_solana(solana_user_address: FixedBytes<32>, public_key: Bytes) -> Self {
        Self {
            user_address: Address::ZERO,
            public_key,
            solana_user_address: Some(solana_user_address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        signers::{SignerSync, local::PrivateKeySigner},
        sol_types::SolValue,
        transports::http::reqwest,
    };
    use connector_utils::tests::rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256};
    use fhevm_gateway_bindings::decryption::{
        Decryption::CtHandleContractPair,
        IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
    };
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;
    use user_decryption_signature::{
        ERC1271_MAGIC_VALUE, compute_user_decrypt_digest, default_user_decrypt_domain,
    };

    const TEST_HOST_PROGRAM_ID: [u8; 32] = [42; 32];

    fn solana_config_for(chain_id: u64) -> Config {
        let mut config = Config::default();
        config.host_chains[0].chain_id = chain_id;
        config.host_chains[0].chain_kind = HostChainKind::Solana;
        config.host_chains[0].solana_host_program_id = Some(TEST_HOST_PROGRAM_ID);
        config
    }

    enum ExpectedOutcome {
        Ok,
        Recoverable,
        Irrecoverable,
    }

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _context_id: U256) -> Result<(), ProcessingError> {
            Ok(())
        }
    }

    fn setup_test_processor(
        asserter: Asserter,
        sns_ct: &SnsCiphertextMaterial,
    ) -> DecryptionProcessor<impl Provider + use<>, impl Provider + use<>, MockContextManager> {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let acl_contracts = HashMap::from([(
            chain_id,
            ACL::new(Address::default(), mock_provider.clone()),
        )]);
        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts,
            s3_service,
        )
    }

    #[test]
    fn kms_decryption_extra_data_normalizes_legacy_zero_marker() {
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x00])),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn kms_decryption_extra_data_keeps_empty_and_versioned_values() {
        assert_eq!(kms_decryption_extra_data(&Bytes::new()), Vec::<u8>::new());
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x01, 0x02])),
            vec![0x01, 0x02]
        );
    }

    enum PubDecryptACLMock {
        Failure(&'static str),
        Success(bool),
    }

    #[rstest]
    #[case::transport_error(
        PubDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable
    )]
    #[case::allowed(PubDecryptACLMock::Success(true), ExpectedOutcome::Ok)]
    #[case::not_allowed(PubDecryptACLMock::Success(false), ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_public_decryption(
        #[case] mock_response: PubDecryptACLMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let decryption_processor = setup_test_processor(asserter.clone(), &sns_ct);
        let sns_ciphertexts = vec![sns_ct];

        match mock_response {
            PubDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            PubDecryptACLMock::Success(val) => asserter.push_success(&val.abi_encode()),
        }

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts, &Bytes::from(vec![]))
            .await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    enum UserDecryptACLMock {
        Failure(&'static str),
        Success {
            user_allowed: bool,
            contract_allowed: bool,
        },
    }

    #[rstest]
    #[case::transport_error(
        UserDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable
    )]
    #[case::allowed(
        UserDecryptACLMock::Success { user_allowed: true, contract_allowed: true },
        ExpectedOutcome::Ok
    )]
    #[case::not_allowed(
        UserDecryptACLMock::Success { user_allowed: false, contract_allowed: false },
        ExpectedOutcome::Recoverable
    )]
    #[case::user_allowed_contract_not_allowed(
        UserDecryptACLMock::Success { user_allowed: true, contract_allowed: false },
        ExpectedOutcome::Recoverable
    )]
    #[case::user_not_allowed_contract_allowed(
        UserDecryptACLMock::Success { user_allowed: false, contract_allowed: true },
        ExpectedOutcome::Recoverable
    )]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_user_decryption(
        #[case] mock_response: UserDecryptACLMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let decryption_processor = setup_test_processor(asserter.clone(), &sns_ct);

        // Use non-delegated userDecryptionRequestCall (requires only 2 ACL checks)
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let sns_ciphertexts = vec![sns_ct];
        let user_address = Address::default();

        match mock_response {
            UserDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            UserDecryptACLMock::Success {
                user_allowed,
                contract_allowed,
            } => {
                asserter.push_success(&user_allowed.abi_encode());
                asserter.push_success(&contract_allowed.abi_encode());
            }
        }

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &sns_ciphertexts, user_address)
            .await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    enum DelegatedUserDecryptACLMock {
        Failure(&'static str),
        Success { is_delegated: bool },
    }

    #[rstest]
    #[case::transport_error(
        DelegatedUserDecryptACLMock::Failure("Transport Error"),
        ExpectedOutcome::Recoverable,
        None
    )]
    #[case::allowed(
        DelegatedUserDecryptACLMock::Success { is_delegated: true },
        ExpectedOutcome::Ok,
        None
    )]
    #[case::not_delegated(
        DelegatedUserDecryptACLMock::Success { is_delegated: false },
        ExpectedOutcome::Recoverable,
        Some("is not a delegate of")
    )]
    #[tokio::test]
    async fn check_ciphertexts_allowed_for_delegated_user_decryption(
        #[case] mock_response: DelegatedUserDecryptACLMock,
        #[case] expected: ExpectedOutcome,
        #[case] expected_error_msg: Option<&str>,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let decryption_processor = setup_test_processor(asserter.clone(), &sns_ct);

        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let sns_ciphertexts = vec![sns_ct];
        let user_address = Address::default();

        match mock_response {
            DelegatedUserDecryptACLMock::Failure(msg) => asserter.push_failure_msg(msg),
            DelegatedUserDecryptACLMock::Success { is_delegated } => {
                asserter.push_success(&is_delegated.abi_encode());
            }
        }

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &sns_ciphertexts, user_address)
            .await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => match result {
                Err(ProcessingError::Irrecoverable(e)) => {
                    let expected_msg = expected_error_msg.unwrap();
                    assert!(
                        e.to_string().contains(expected_msg),
                        "Expected error message to contain '{expected_msg}', got: {e}",
                    );
                }
                _ => panic!("Expected Irrecoverable error, got: {:?}", result),
            },
        }
    }

    /// Builds a processor whose only host chain is a Solana chain.
    fn solana_processor(
        ct_chain_id: u64,
    ) -> DecryptionProcessor<
        impl alloy::providers::Provider + Clone,
        impl alloy::providers::Provider + Clone,
        MockContextManager,
    > {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        )
    }

    fn assert_fails_closed(result: Result<(), ProcessingError>) {
        match result {
            // Must be irrecoverable: retrying an unsupported authorization path
            // cannot succeed, and we must never authorize from untrusted witnesses.
            Err(ProcessingError::Irrecoverable(e)) => assert!(
                e.to_string().contains("not a trusted authorization source"),
                "unexpected error: {e}"
            ),
            other => panic!("expected fail-closed Irrecoverable refusal, got {other:?}"),
        }
    }

    // The Gateway `extraData` witness path is not a trusted source for Solana ACL
    // state: an attacker controls those bytes (including the account `owner`
    // field), so trusting them would authorize decryption of any Solana handle.
    // These tests pin the fail-closed behavior for the USER-decrypt gateway path:
    // even a non-empty, attacker-shaped witness payload must be refused without
    // being inspected. Public decryption, by contrast, is authorized from the
    // RPC-verified on-chain ACL record (the trusted source) — see below.

    // Public decryption does NOT use the gateway `extraData` witnesses: it authorizes
    // against the handle's `zama-host` ACL record read directly from the validator. With
    // a Solana host chain configured, the check therefore takes the RPC-verified path
    // (`verify_solana_public_decrypt_allowed`) and — with no reachable validator in this
    // unit test — surfaces the RPC/ACL fetch error rather than the gateway fail-closed
    // refusal. The point is that it is NOT refused as an untrusted-witness path.
    #[tokio::test]
    async fn solana_public_decryption_uses_rpc_verified_acl_not_gateway_witness() {
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let processor = solana_processor(ct_chain_id);

        let result = processor
            .check_ciphertexts_allowed_for_public_decryption(
                &[sns_ct],
                &Bytes::from(vec![1u8; 256]),
            )
            .await;

        let err = result.expect_err("no reachable validator, so the RPC-verified ACL check errors");
        assert!(
            !err.to_string()
                .contains("not a trusted authorization source"),
            "public decrypt must take the RPC-verified ACL path, not the gateway fail-closed \
             refusal; got: {err}"
        );
    }

    #[tokio::test]
    async fn solana_user_decryption_is_refused_fail_closed() {
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let processor = solana_processor(ct_chain_id);
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();

        let result = processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        assert_fails_closed(result);
    }

    #[tokio::test]
    async fn solana_delegated_user_decryption_is_refused_fail_closed() {
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let processor = solana_processor(ct_chain_id);
        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();

        let result = processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        assert_fails_closed(result);
    }

    /// Builds a `UserDecryptionRequestV2` whose payload carries a valid 65-byte ECDSA signature
    /// over the EIP-712 digest.
    ///
    /// `user_address` and `signing_key` are intentionally decoupled: the EOA-direct case
    /// passes `signing_key.address()` for both, and the smart-account case passes the
    /// contract address as `user_address` while `signing_key` plays the role of the wallet's
    /// owner EOA — its signature recovers to a different address, forcing the ERC-1271
    /// fallback in `verify_signature`.
    ///
    /// The digest is computed against `Config::default().decryption_contract.address` — the
    /// same gateway address `setup_test_processor` configures the processor with.
    fn make_v2_request(
        sns_ct: &SnsCiphertextMaterial,
        owner_address: Address,
        user_address: Address,
        signing_key: &PrivateKeySigner,
        allowed_contracts: Vec<Address>,
        start_offset_secs: i64,
        duration_secs: u64,
    ) -> UserDecryptionRequestV2 {
        let start = (Utc::now().timestamp() + start_offset_secs) as u64;
        let mut payload = UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: Bytes::from(rand_public_key()),
            allowedContracts: allowed_contracts,
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(start),
                durationSeconds: U256::from(duration_secs),
            },
            extraData: Bytes::default(),
            signature: Bytes::default(),
        };

        let chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let gateway_addr = Config::default().decryption_contract.address;
        let domain = default_user_decrypt_domain(chain_id, gateway_addr);
        let digest = compute_user_decrypt_digest(&payload, &domain);
        let sig = signing_key.sign_hash_sync(&digest).unwrap();
        payload.signature = Bytes::from(sig.as_bytes().to_vec());

        UserDecryptionRequestV2 {
            decryptionId: rand_u256(),
            snsCtMaterials: vec![sns_ct.clone()],
            handles: vec![HandleEntry {
                handle: sns_ct.ctHandle,
                contractAddress: rand_address(),
                ownerAddress: owner_address,
            }],
            payload,
        }
    }

    #[rstest]
    #[case::not_yet_valid(3600_i64, 86400_u64, ExpectedOutcome::Recoverable)]
    #[case::expired(-(2 * 3600_i64), 3600_u64, ExpectedOutcome::Irrecoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_validity_window(
        #[case] start_offset_secs: i64,
        #[case] duration_secs: u64,
        #[case] expected: ExpectedOutcome,
    ) {
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let processor = setup_test_processor(Asserter::new(), &sns_ct);
        let request = make_v2_request(
            &sns_ct,
            user_address,
            user_address,
            &user_signer,
            vec![],
            start_offset_secs,
            duration_secs,
        );

        let result = processor.check_user_decryption_request_v2(&request).await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    // Test userAddress ∈ allowedContracts
    #[tokio::test]
    async fn check_user_decryption_request_v2_user_in_allowed_contracts() {
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let processor = setup_test_processor(Asserter::new(), &sns_ct);
        let request = make_v2_request(
            &sns_ct,
            user_address,
            user_address,
            &user_signer,
            vec![user_address],
            -3600,
            86400,
        );

        let result = processor.check_user_decryption_request_v2(&request).await;
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    // -------------------------------------------------------------------------
    // Invalidation check (validity window passes, empty allowedContracts, direct ownership)
    // -------------------------------------------------------------------------
    enum InvalidationMock {
        Zero,         // invalidation_ts = 0 → start (≈ now-3600) >= 0 → passes
        AboveStart,   // invalidation_ts = u64::MAX → start < u64::MAX → fails
        EqualToStart, // invalidation_ts = start → start < start is false → passes
        TransportError,
    }

    #[rstest]
    #[case::not_invalidated(InvalidationMock::Zero, ExpectedOutcome::Ok)]
    #[case::invalidated(InvalidationMock::AboveStart, ExpectedOutcome::Irrecoverable)]
    #[case::boundary_passes(InvalidationMock::EqualToStart, ExpectedOutcome::Ok)]
    #[case::transport_error(InvalidationMock::TransportError, ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_invalidation(
        #[case] mock: InvalidationMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        const START_OFFSET_SECS: i64 = -3600;
        let start = U256::from((Utc::now().timestamp() + START_OFFSET_SECS) as u64);

        let passes = match mock {
            InvalidationMock::Zero => {
                asserter.push_success(&U256::ZERO.abi_encode());
                true
            }
            InvalidationMock::AboveStart => {
                asserter.push_success(&U256::from(u64::MAX).abi_encode());
                false
            }
            InvalidationMock::EqualToStart => {
                asserter.push_success(&start.abi_encode());
                true
            }
            InvalidationMock::TransportError => {
                asserter.push_failure_msg("transport error");
                false
            }
        };

        if passes {
            asserter.push_success(&true.abi_encode()); // ownership: direct path passes
        }

        let request = make_v2_request(
            &sns_ct,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![],
            START_OFFSET_SECS,
            86400,
        );
        let result = processor.check_user_decryption_request_v2(&request).await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    // -------------------------------------------------------------------------
    // Ownership check (empty allowedContracts → 1 RPC per test)
    // -------------------------------------------------------------------------
    enum OwnershipMock {
        DirectPath(Option<bool>),
        DelegatedPath(Option<bool>),
    }

    #[rstest]
    #[case::direct_transport_error(OwnershipMock::DirectPath(None), ExpectedOutcome::Recoverable)]
    #[case::direct_allowed(OwnershipMock::DirectPath(Some(true)), ExpectedOutcome::Ok)]
    #[case::direct_not_allowed(
        OwnershipMock::DirectPath(Some(false)),
        ExpectedOutcome::Recoverable
    )]
    #[case::delegated_transport_error(
        OwnershipMock::DelegatedPath(None),
        ExpectedOutcome::Recoverable
    )]
    #[case::delegated_yes(OwnershipMock::DelegatedPath(Some(true)), ExpectedOutcome::Ok)]
    #[case::delegated_no(
        OwnershipMock::DelegatedPath(Some(false)),
        ExpectedOutcome::Recoverable
    )]
    #[tokio::test]
    async fn check_user_decryption_request_v2_ownership(
        #[case] mock: OwnershipMock,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let user_address = user_signer.address();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        let (owner_address, acl_response) = match mock {
            OwnershipMock::DirectPath(r) => (user_address, r),
            OwnershipMock::DelegatedPath(r) => (rand_address(), r),
        };
        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation check: not invalidated
        match acl_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(
            &sns_ct,
            owner_address,
            user_address,
            &user_signer,
            vec![],
            -3600,
            86400,
        );
        let result = processor.check_user_decryption_request_v2(&request).await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    // -------------------------------------------------------------------------
    // Allowed contracts check (direct ownership always passes → 2 RPCs)
    //
    // Two `isAllowed` calls are made concurrently via `tokio::try_join!`. The Asserter
    // serves responses in FIFO order, and poll ordering between the two futures is
    // guaranteed by the `biased` annotation.
    // -------------------------------------------------------------------------
    #[rstest]
    #[case::transport_error(None, ExpectedOutcome::Recoverable)]
    #[case::at_least_one_allowed(Some(true), ExpectedOutcome::Ok)]
    #[case::none_allowed(Some(false), ExpectedOutcome::Recoverable)]
    #[tokio::test]
    async fn check_user_decryption_request_v2_allowed_contracts(
        #[case] contract_response: Option<bool>,
        #[case] expected: ExpectedOutcome,
    ) {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation check: not invalidated
        asserter.push_success(&true.abi_encode()); // ownership always passes
        match contract_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(
            &sns_ct,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![rand_address()],
            -3600,
            86400,
        );
        let result = processor.check_user_decryption_request_v2(&request).await;

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            ExpectedOutcome::Recoverable => {
                assert!(matches!(result, Err(ProcessingError::Recoverable(_))))
            }
            ExpectedOutcome::Irrecoverable => {
                assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))))
            }
        }
    }

    // -------------------------------------------------------------------------
    // RFC-012: signature verification wired into check_user_decryption_request_v2
    // -------------------------------------------------------------------------

    /// A flipped byte in `payload.signature` makes ecrecover return some other address; with
    /// no contract code at `userAddress`, the ERC-1271 fallback rejects with Irrecoverable.
    /// No invalidation/ACL RPC is reached.
    #[tokio::test]
    async fn check_user_decryption_request_v2_signature_mismatch() {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let user_signer = PrivateKeySigner::random();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        // STATICCALL to a no-code address returns empty returndata at the EVM level →
        // `EoaMismatchNoCode` rejection.
        asserter.push_success(&Bytes::default());

        let mut request = make_v2_request(
            &sns_ct,
            user_signer.address(),
            user_signer.address(),
            &user_signer,
            vec![],
            -3600,
            86400,
        );
        // Flip a byte in the signature
        let mut sig = request.payload.signature.to_vec();
        sig[0] ^= 0xFF;
        request.payload.signature = Bytes::from(sig);

        let result = processor.check_user_decryption_request_v2(&request).await;
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    /// A smart-account user (Safe-style) whose contract returns the ERC-1271 magic value
    /// passes the signature check, then the rest of the pipeline (invalidation + ownership)
    /// proceeds normally.
    #[tokio::test]
    async fn check_user_decryption_request_v2_smart_account_accepts() {
        let asserter = Asserter::new();
        let sns_ct = rand_sns_ct();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        // Random "smart account" address; no off-chain key controls it, so ecrecover will
        // never match — verification only succeeds via the ERC-1271 fallback.
        let smart_account = rand_address();
        // The wallet's owner EOA: produces real 65-byte signature bytes whose recovered
        // address is *not* `smart_account`, forcing the ERC-1271 path.
        let owner = PrivateKeySigner::random();
        let request = make_v2_request(
            &sns_ct,
            smart_account, // owner == userAddress: direct path
            smart_account,
            &owner,
            vec![],
            -3600,
            86400,
        );

        // Mock the host RPC sequence:
        //   1. isValidSignature → magic value (left-aligned in a 32-byte word)
        //   2. invalidation → 0
        //   3. ownership: isAllowed → true
        let mut magic_word = [0u8; 32];
        magic_word[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&magic_word); // isValidSignature
        asserter.push_success(&U256::ZERO.abi_encode()); // invalidation
        asserter.push_success(&true.abi_encode()); // ownership

        processor
            .check_user_decryption_request_v2(&request)
            .await
            .unwrap();
    }
}
