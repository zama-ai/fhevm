use crate::core::{
    config::Config,
    event_processor::{
        CiphertextManager, ProcessingError, RequestCheckError, RequestCheckKind,
        context::ContextManager,
        solana_user_decrypt::{
            SolanaHost, check_solana_handles_acl, check_solana_handles_public_decrypt,
            verify_solana_user_decrypt_signature,
        },
    },
    solana_acl::HandleBytes,
};
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Address, Bytes, FixedBytes, U256, map::DefaultHashBuilder},
    providers::Provider,
    sol_types::{Eip712Domain, SolCall},
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest, extra_data::parse_extra_data, handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, HandleEntry, SnsCiphertextMaterial,
    UserDecryptionRequest_1 as UserDecryptionRequestV2, UserDecryptionRequestSolana,
    delegatedUserDecryptionRequestCall, userDecryptionRequest_1Call as userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use futures::future::{join_all, try_join_all};
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use sqlx::types::chrono::Utc;
use std::collections::HashMap;
use tracing::info;
use user_decryption_signature::{compute_user_decrypt_digest, verify_signature};

#[derive(Clone)]
pub enum HostChainAclBackend<HP: Provider> {
    Evm(ACLInstance<HP>),
    Solana(SolanaHost),
}

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider, HP: Provider, C> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The entity used to validate KMS context.
    context_manager: C,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The configured ACL backend for each host chain.
    host_chain_backends: HashMap<u64, HostChainAclBackend<HP>>,

    /// The entity used to verify and collect the ciphertexts of decryption requests.
    ciphertext_manager: CiphertextManager<GP>,

    /// Gas cap for the `IERC1271.isValidSignature` static call (RFC-012).
    erc1271_gas_limit: u64,
}

impl<GP, HP, C> DecryptionProcessor<GP, HP, C>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
    C: ContextManager,
{
    pub fn new(
        config: &Config,
        context_manager: C,
        gateway_provider: GP,
        host_chain_backends: HashMap<u64, HostChainAclBackend<HP>>,
        ciphertext_manager: CiphertextManager<GP>,
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

        Self {
            domain,
            context_manager,
            decryption_contract,
            host_chain_backends,
            ciphertext_manager,
            erc1271_gas_limit: config.erc1271_gas_limit,
        }
    }

    fn host_chain_backend(
        &self,
        chain_id: u64,
    ) -> Result<&HostChainAclBackend<HP>, RequestCheckError> {
        self.host_chain_backends.get(&chain_id).ok_or_else(|| {
            RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                anyhow!("No host-chain ACL backend configured for chain id {chain_id}"),
            )
        })
    }

    fn evm_acl_backend(&self, chain_id: u64) -> Result<&ACLInstance<HP>, RequestCheckError> {
        match self.host_chain_backend(chain_id)? {
            HostChainAclBackend::Evm(acl) => Ok(acl),
            HostChainAclBackend::Solana(_) => Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Acl,
                anyhow!(
                    "Host chain {chain_id} uses the Solana ACL backend, but this request requires EVM"
                ),
            )),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        extra_data: &[u8],
    ) -> Result<(), RequestCheckError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(|e| RequestCheckError::irrecoverable(RequestCheckKind::Acl, e))?;

            match self.host_chain_backend(ct_chain_id)? {
                HostChainAclBackend::Solana(host) => {
                    // Public access is proven by a PublicDecryptLeaf MMR proof and verified
                    // against the live confirmed lineage account.
                    check_solana_handles_public_decrypt(host, &[ct.ctHandle.0], extra_data)
                        .await
                        .map_err(|e| {
                            RequestCheckError::from_processing(RequestCheckKind::Acl, e)
                        })?;
                }
                HostChainAclBackend::Evm(acl_contract) => {
                    if !acl_contract
                        .isAllowedForDecryption(ct.ctHandle)
                        .call()
                        .await
                        .map_err(RequestCheckError::network)?
                    {
                        return Err(RequestCheckError::recoverable(
                            RequestCheckKind::Acl,
                            anyhow!("Decryption is not allowed for {}", ct.ctHandle),
                        ));
                    }
                }
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
    ) -> Result<(), RequestCheckError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        let (ct_handle_contract_pairs, delegator_address) =
            match delegatedUserDecryptionRequestCall::abi_decode(calldata.as_slice()) {
                Ok(parsed_calldata) => (
                    parsed_calldata.ctHandleContractPairs,
                    Some(parsed_calldata.delegationAccounts.delegatorAddress),
                ),
                Err(e) => {
                    let parsed_calldata = userDecryptionRequestCall::abi_decode(
                        calldata.as_slice(),
                    )
                    .map_err(|e2| {
                        RequestCheckError::irrecoverable(
                            RequestCheckKind::Acl,
                            anyhow!(
                                "Was not able to parse calldata for both userDecryptionRequestCall \
                                {e2} and delegatedUserDecryptionRequestCall ({e})!"
                            ),
                        )
                    })?;
                    (parsed_calldata.ctHandleContractPairs, None)
                }
            };

        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(|e| RequestCheckError::irrecoverable(RequestCheckKind::Acl, e))?;
            let acl_contract = self.evm_acl_backend(ct_chain_id)?;
            let contract_address = contracts_map.get(ct.ctHandle.as_slice()).ok_or_else(|| {
                RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    anyhow!("Could not find contract address for handle {}", ct.ctHandle),
                )
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

    async fn inner_acl_check_for_delegated_user_decryption(
        &self,
        acl_contract: &ACLInstance<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
        delegator_address: Address,
    ) -> Result<(), RequestCheckError> {
        let is_delegated = acl_contract
            .isHandleDelegatedForUserDecryption(
                delegator_address,
                user_address,
                contract_address,
                handle,
            )
            .call()
            .await
            .map_err(RequestCheckError::network)?;

        if !is_delegated {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                anyhow!(
                    "{user_address} is not a delegate of {delegator_address} for contract \
                    {contract_address} and handle {handle}!",
                ),
            ));
        }

        Ok(())
    }

    /// Verify that a `UserDecryptionRequestV2` is internally consistent before the ACL phase:
    /// `handles` and `snsCtMaterials` are pairwise aligned, and every handle resolves to the
    /// same host chain id. Returns that shared chain id.
    /// Shared by the EVM (`UserDecryptionRequestV2`) and Solana (`UserDecryptionRequestSolana`)
    /// paths — both carry the same `handles` / `snsCtMaterials` shapes.
    fn validate_handles_and_extract_chain_id(
        handles: &[HandleEntry],
        sns_ct_materials: &[SnsCiphertextMaterial],
    ) -> Result<u64, RequestCheckError> {
        if handles.len() != sns_ct_materials.len() {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Acl,
                anyhow!(
                    "handles/snsCtMaterials length mismatch: {} vs {}",
                    handles.len(),
                    sns_ct_materials.len(),
                ),
            ));
        }

        let chain_id = handles
            .first()
            .ok_or_else(|| {
                RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    anyhow!("request contains no handles"),
                )
            })
            .map(|h| extract_chain_id_from_handle(h.handle.as_slice()))?
            .map_err(|e| RequestCheckError::irrecoverable(RequestCheckKind::Acl, e))?;

        for (i, (h, m)) in handles.iter().zip(sns_ct_materials.iter()).enumerate() {
            if h.handle != m.ctHandle {
                return Err(RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    anyhow!(
                        "handles[{i}].handle ({}) != snsCtMaterials[{i}].ctHandle ({})",
                        h.handle,
                        m.ctHandle,
                    ),
                ));
            }
            match extract_chain_id_from_handle(h.handle.as_slice()) {
                Ok(id) if id == chain_id => (),
                Ok(other) => {
                    return Err(RequestCheckError::irrecoverable(
                        RequestCheckKind::Acl,
                        anyhow!(
                            "user decryption request handles span multiple chains ({chain_id}, {other})",
                        ),
                    ));
                }
                Err(e) => {
                    return Err(RequestCheckError::irrecoverable(
                        RequestCheckKind::Acl,
                        anyhow!(
                            "Failed to extract chain_id from handle {}: {e}",
                            hex::encode(h.handle),
                        ),
                    ));
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
    ) -> Result<(), RequestCheckError> {
        info!(
            "Starting RFC016 check for {} handles...",
            request.handles.len()
        );

        let chain_id =
            Self::validate_handles_and_extract_chain_id(&request.handles, &request.snsCtMaterials)?;

        // Solana user-decryptions are a distinct event kind (`UserDecryptionSolana`) handled by
        // `check_user_decryption_request_solana`; the V2 path below is EVM-only.
        let payload = &request.payload;

        // Validity window
        let start = payload.requestValidity.startTimestamp;
        let now = U256::from(Utc::now().timestamp() as u64);
        let end = start.saturating_add(payload.requestValidity.durationSeconds);
        if now < start {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Signature,
                anyhow!(
                    "RFC016 user decryption request not yet valid: now {now} < startTimestamp {start}",
                ),
            ));
        }
        if now > end {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Signature,
                anyhow!(
                    "RFC016 user decryption request validity window expired: now {now} > end {end}"
                ),
            ));
        }

        // `userAddress` must not appear in a non-empty `allowedContracts` list.
        if payload.allowedContracts.contains(&payload.userAddress) {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Signature,
                anyhow!(
                    "userAddress {} is listed in allowedContracts — request rejected",
                    payload.userAddress
                ),
            ));
        }

        let acl_contract = self.evm_acl_backend(chain_id)?;

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
                .map_err(RequestCheckError::from)
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

    /// Picks the `client_address` for an EVM V2 user-decryption KMS gRPC request: the checksummed
    /// `userAddress`. (Solana uses [`Self::user_decryption_extra_data_for_solana`].)
    pub fn user_decryption_extra_data_for_v2(
        request: &UserDecryptionRequestV2,
    ) -> UserDecryptionExtraData {
        let payload = &request.payload;
        UserDecryptionExtraData::new(payload.userAddress, payload.publicKey.clone())
    }

    /// Picks the `client_address` for a Solana user-decryption KMS gRPC request: `solana:<hex
    /// identity>`, with the ed25519 identity taken from the typed request payload (the authorization
    /// check runs before this, so the identity is trustworthy here).
    pub fn user_decryption_extra_data_for_solana(
        request: &UserDecryptionRequestSolana,
    ) -> UserDecryptionExtraData {
        let payload = &request.payload;
        UserDecryptionExtraData::new_solana(payload.userIdentity.0, payload.publicKey.clone())
    }

    /// Solana user-decryption authorization check (RFC-021). The ed25519 auth fields are TYPED on
    /// the request (no extraData blob — extraData carries only the KMS context).
    ///
    /// Unlike the EVM path, the re-encryption `publicKey ↔ identity` binding is NOT verified
    /// on-chain by the Gateway, so this connector verifies it itself:
    ///
    /// 1. validity window (shared semantics with EVM; rejects expired / not-yet-valid windows),
    /// 2. ed25519 signature over the canonical preimage — binds `publicKey`, handles, identity,
    ///    nonce, allowed domains, and validity window to the claimed Solana identity (closes the
    ///    publicKey-substitution / relayer-bypass bug),
    /// 3. per-handle ACL read at `confirmed` commitment with owner + canonical-PDA checks and the
    ///    domain-scoped verifier (identity as subject).
    #[tracing::instrument(skip_all)]
    pub async fn check_user_decryption_request_solana(
        &self,
        request: &UserDecryptionRequestSolana,
    ) -> Result<(), RequestCheckError> {
        let chain_id =
            Self::validate_handles_and_extract_chain_id(&request.handles, &request.snsCtMaterials)?;
        info!("Starting Solana user-decryption check for chain {chain_id}...");

        let payload = &request.payload;

        // Validity window (same semantics as the EVM path).
        let start = payload.requestValidity.startTimestamp;
        let now = U256::from(Utc::now().timestamp() as u64);
        let end = start.saturating_add(payload.requestValidity.durationSeconds);
        if now < start {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Signature,
                anyhow!(
                    "Solana user decryption request not yet valid: now {now} < startTimestamp {start}",
                ),
            ));
        }
        if now > end {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::Signature,
                anyhow!(
                    "Solana user decryption request validity window expired: now {now} > end {end}"
                ),
            ));
        }

        let host = match self.host_chain_backend(chain_id)? {
            HostChainAclBackend::Solana(host) => host,
            HostChainAclBackend::Evm(_) => {
                return Err(RequestCheckError::irrecoverable(
                    RequestCheckKind::Acl,
                    anyhow!(
                        "Host chain {chain_id} uses the EVM ACL backend, but this request requires Solana"
                    ),
                ));
            }
        };

        // ed25519 binding — the check that closes the substitution bug. Pure, no I/O.
        let auth = verify_solana_user_decrypt_signature(request, chain_id)
            .map_err(|e| RequestCheckError::from_processing(RequestCheckKind::Signature, e))?;

        // ACL phase: read each handle's record at confirmed commitment and run the domain-scoped
        // verifier with the identity as subject.
        let handles: Vec<HandleBytes> = request.handles.iter().map(|e| e.handle.0).collect();
        check_solana_handles_acl(host, &handles, &auth)
            .await
            .map_err(|e| RequestCheckError::from_processing(RequestCheckKind::Acl, e))?;

        info!(
            "Solana user-decryption ACL check passed for {} handles!",
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
    ) -> Result<(), RequestCheckError> {
        let handle_hex = hex::encode(entry.handle);
        if entry.ownerAddress == user_address {
            let user_allowed = acl_contract
                .isAllowed(entry.handle, user_address)
                .call()
                .await
                .map_err(RequestCheckError::network)?;
            if !user_allowed {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    anyhow!("{user_address} is not allowed to decrypt {handle_hex}"),
                ));
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
                .map_err(RequestCheckError::network)?;
            if !is_delegated {
                return Err(RequestCheckError::recoverable(
                    RequestCheckKind::Acl,
                    anyhow!(
                        "{user_address} is not a delegate of {} for contract {} and handle {handle_hex}",
                        entry.ownerAddress,
                        entry.contractAddress,
                    ),
                ));
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
    ) -> Result<(), RequestCheckError> {
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
            // This branch covers both a genuine denial and an all-RPC-failed wave; the two can't
            // be cleanly separated here, so it counts as a single ACL rejection.
            Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                anyhow!(
                    "No contract in allowedContracts is allowed to decrypt handle {handle} ({results:?})",
                ),
            ))
        }
    }

    /// RFC016 signature invalidation check. Rejects if `startTimestamp < invalidationTs`, meaning
    /// the user has invalidated all signatures issued before `invalidationTs`.
    async fn inner_invalidation_check_for_user_decryption_v2(
        &self,
        acl_contract: &ACLInstance<HP>,
        user_address: Address,
        start_timestamp: U256,
    ) -> Result<(), RequestCheckError> {
        let invalidation_ts = acl_contract
            .decryptionSignatureInvalidatedBefore(user_address)
            .call()
            .await
            .map_err(RequestCheckError::network)?;
        if start_timestamp < invalidation_ts {
            return Err(RequestCheckError::irrecoverable(
                // TODO: reconsider Signature naming
                RequestCheckKind::Signature,
                anyhow!(
                    "RFC016 signature invalidated: startTimestamp {start_timestamp} < \
                     invalidatedBefore {invalidation_ts} for userAddress {user_address}"
                ),
            ));
        }
        Ok(())
    }

    async fn inner_acl_check_for_user_decryption(
        &self,
        acl_contract: &ACLInstance<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        contract_address: Address,
    ) -> Result<(), RequestCheckError> {
        let user_allowed_call = acl_contract.isAllowed(handle, user_address);
        let contract_allowed_call = acl_contract.isAllowed(handle, contract_address);

        let (user_allowed, contract_allowed) =
            tokio::try_join!(biased; user_allowed_call.call(), contract_allowed_call.call())
                .map_err(RequestCheckError::network)?;

        if !user_allowed {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                anyhow!("{user_address} is not allowed to decrypt {handle}!"),
            ));
        }
        if !contract_allowed {
            return Err(RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                anyhow!("{contract_address} is not allowed to decrypt {handle}!"),
            ));
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
        self.context_manager
            .validate_context(&parsed_extra_data)
            .await
            .map_err(RequestCheckError::record)?;

        let ciphertexts = self.prepare_ciphertexts(&key_id, sns_materials).await?;

        let request_id = Some(u256_to_request_id(decryption_id));
        let kms_extra_data = kms_decryption_extra_data(extra_data);

        if let Some(user_decrypt_data) = user_decrypt_data {
            let client_address = user_decrypt_data.client_address;
            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                enc_key,
                typed_ciphertexts: ciphertexts,
                extra_data: kms_extra_data,
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
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
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
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
            .ciphertext_manager
            .retrieve_verified_ciphertexts(sns_materials)
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
    /// The `client_address` set on the KMS gRPC request. For EVM this is the checksummed
    /// `userAddress`; for Solana it is `solana:<hex identity>` (kms#637's parser keys on the
    /// `solana:` prefix).
    pub client_address: String,
    pub public_key: Bytes,
}

impl UserDecryptionExtraData {
    /// EVM user-decryption: `client_address` is the checksummed EVM `userAddress`.
    pub fn new(user_address: Address, public_key: Bytes) -> Self {
        Self {
            client_address: user_address.to_checksum(None),
            public_key,
        }
    }

    /// Solana user-decryption: `client_address` is `solana:<hex identity>` — lowercase, exactly
    /// 64 hex chars, no `0x`.
    pub fn new_solana(identity: [u8; 32], public_key: Bytes) -> Self {
        Self {
            client_address: format!("solana:{}", hex::encode(identity)),
            public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::solana_v2_fetcher::SolanaV2Fetcher;
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        signers::{SignerSync, local::PrivateKeySigner},
        sol_types::SolValue,
        transports::http::reqwest,
    };
    use connector_utils::{
        tests::rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256},
        types::extra_data::ExtraData,
    };
    use fhevm_gateway_bindings::decryption::{
        Decryption::CtHandleContractPair,
        IDecryption::{
            RequestValiditySeconds, UserDecryptionRequestPayload,
            UserDecryptionRequestSolanaPayload,
        },
    };
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;
    use user_decryption_signature::{
        ERC1271_MAGIC_VALUE, compute_user_decrypt_digest, default_user_decrypt_domain,
    };

    enum ExpectedOutcome {
        Ok,
        Recoverable,
        Irrecoverable,
    }

    fn assert_irrecoverable_contains(result: Result<(), ProcessingError>, expected: &str) {
        match result {
            Err(ProcessingError::Irrecoverable(error)) => {
                assert!(
                    error.to_string().contains(expected),
                    "unexpected error: {error}"
                );
            }
            other => panic!("expected irrecoverable error containing '{expected}', got {other:?}"),
        }
    }

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _extra_data: &ExtraData) -> Result<(), RequestCheckError> {
            Ok(())
        }
    }

    #[derive(Clone, Copy)]
    enum TestHostBackend {
        Evm,
        Solana,
        Missing,
    }

    fn setup_test_processor(
        asserter: Asserter,
        sns_ct: &SnsCiphertextMaterial,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>, MockContextManager>
    {
        setup_test_processor_with_backend(asserter, sns_ct, TestHostBackend::Evm)
    }

    fn setup_test_processor_with_backend(
        asserter: Asserter,
        sns_ct: &SnsCiphertextMaterial,
        backend: TestHostBackend,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>, MockContextManager>
    {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = Config::default();
        let host_chain_backends = match backend {
            TestHostBackend::Evm => HashMap::from([(
                chain_id,
                HostChainAclBackend::Evm(ACL::new(Address::default(), mock_provider.clone())),
            )]),
            TestHostBackend::Solana => HashMap::from([(
                chain_id,
                HostChainAclBackend::Solana(SolanaHost {
                    program_id: [7; 32],
                    fetcher: SolanaV2Fetcher::new(
                        config.host_chains[0].url.clone(),
                        reqwest::Client::new(),
                    ),
                }),
            )]),
            TestHostBackend::Missing => HashMap::new(),
        };
        let ciphertext_manager =
            CiphertextManager::disabled(mock_provider.clone(), reqwest::Client::new());
        DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            host_chain_backends,
            ciphertext_manager,
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
            .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts, &[0u8])
            .await
            .map_err(RequestCheckError::record);

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
            .await
            .map_err(RequestCheckError::record);

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
            .await
            .map_err(RequestCheckError::record);

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

    fn make_solana_request(sns_ct: &SnsCiphertextMaterial) -> UserDecryptionRequestSolana {
        let payload = UserDecryptionRequestSolanaPayload {
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from((Utc::now().timestamp() - 60) as u64),
                durationSeconds: U256::from(3_600),
            },
            ..Default::default()
        };
        UserDecryptionRequestSolana {
            decryptionId: U256::from(1),
            snsCtMaterials: vec![sns_ct.clone()],
            handles: vec![HandleEntry {
                handle: sns_ct.ctHandle,
                contractAddress: Address::ZERO,
                ownerAddress: Address::ZERO,
            }],
            payload,
        }
    }

    #[tokio::test]
    async fn public_decryption_dispatches_to_solana_backend() {
        let sns_ct = rand_sns_ct();
        let processor =
            setup_test_processor_with_backend(Asserter::new(), &sns_ct, TestHostBackend::Solana);

        let result = processor
            .check_ciphertexts_allowed_for_public_decryption(&[sns_ct], &[0])
            .await
            .map_err(RequestCheckError::record);

        match result {
            Err(ProcessingError::Irrecoverable(error)) => {
                assert!(
                    error
                        .to_string()
                        .contains("requires a PublicDecryptLeaf MMR proof")
                );
            }
            other => panic!("expected Solana public-decrypt rejection, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn legacy_user_decryption_rejects_solana_backend() {
        let sns_ct = rand_sns_ct();
        let processor =
            setup_test_processor_with_backend(Asserter::new(), &sns_ct, TestHostBackend::Solana);
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();

        let result = processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::ZERO)
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires EVM");
    }

    #[tokio::test]
    async fn rfc016_user_decryption_rejects_solana_backend() {
        let sns_ct = rand_sns_ct();
        let processor =
            setup_test_processor_with_backend(Asserter::new(), &sns_ct, TestHostBackend::Solana);
        let signer = PrivateKeySigner::random();
        let request = make_v2_request(
            &sns_ct,
            signer.address(),
            signer.address(),
            &signer,
            vec![],
            -60,
            3_600,
        );

        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires EVM");
    }

    #[tokio::test]
    async fn solana_user_decryption_rejects_evm_backend() {
        let sns_ct = rand_sns_ct();
        let processor = setup_test_processor(Asserter::new(), &sns_ct);
        let request = make_solana_request(&sns_ct);

        let result = processor
            .check_user_decryption_request_solana(&request)
            .await
            .map_err(RequestCheckError::record);

        assert_irrecoverable_contains(result, "request requires Solana");
    }

    #[tokio::test]
    async fn unknown_backend_is_recoverable_for_all_decryption_families() {
        let sns_ct = rand_sns_ct();
        let processor =
            setup_test_processor_with_backend(Asserter::new(), &sns_ct, TestHostBackend::Missing);
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let signer = PrivateKeySigner::random();
        let evm_v2_request = make_v2_request(
            &sns_ct,
            signer.address(),
            signer.address(),
            &signer,
            vec![],
            -60,
            3_600,
        );
        let solana_request = make_solana_request(&sns_ct);

        let public = processor
            .check_ciphertexts_allowed_for_public_decryption(std::slice::from_ref(&sns_ct), &[0])
            .await
            .map_err(RequestCheckError::record);
        let legacy = processor
            .check_ciphertexts_allowed_for_user_decryption(
                calldata,
                std::slice::from_ref(&sns_ct),
                Address::ZERO,
            )
            .await
            .map_err(RequestCheckError::record);
        let evm_v2 = processor
            .check_user_decryption_request_v2(&evm_v2_request)
            .await
            .map_err(RequestCheckError::record);
        let solana = processor
            .check_user_decryption_request_solana(&solana_request)
            .await
            .map_err(RequestCheckError::record);

        for result in [public, legacy, evm_v2, solana] {
            match result {
                Err(ProcessingError::Recoverable(error)) => assert!(
                    error
                        .to_string()
                        .contains("No host-chain ACL backend configured")
                ),
                other => panic!("expected recoverable unknown-backend error, got {other:?}"),
            }
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

        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

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

        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);
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
        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

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
        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

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
        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);

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

        let result = processor
            .check_user_decryption_request_v2(&request)
            .await
            .map_err(RequestCheckError::record);
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
