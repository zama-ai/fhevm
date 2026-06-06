use crate::core::{
    config::{Config, HostChainKind},
    event_processor::{ProcessingError, context::ContextManager, s3::S3Service},
    solana_acl::{SolanaAclVerifier, SolanaPubkeyBytes, decode_acl_record_witness},
};
use base64::Engine as _;
use std::str::FromStr;
use alloy::{
    consensus::Transaction,
    hex,
    primitives::{Address, Bytes, FixedBytes, U256, map::DefaultHashBuilder},
    providers::Provider,
    sol_types::SolCall,
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest,
    extra_data::parse_extra_data,
    handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, SnsCiphertextMaterial, delegatedUserDecryptionRequestCall,
    userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::collections::HashMap;
use tracing::info;

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
            tokio::try_join!(user_allowed_call.call(), contract_allowed_call.call())
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

        let context_id = parse_extra_data(extra_data)
            .map_err(ProcessingError::Irrecoverable)?
            .context_id;
        // TODO: validation of epoch_id during RFC-005 implementation
        self.context_manager.validate_context(context_id).await?;

        let ciphertexts = self.prepare_ciphertexts(&key_id, sns_materials).await?;

        let request_id = Some(u256_to_request_id(decryption_id));

        if let Some(user_decrypt_data) = user_decrypt_data {
            let client_address = user_decrypt_data.user_address.to_checksum(None);
            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                enc_key,
                typed_ciphertexts: ciphertexts,
                extra_data: extra_data.to_vec(),
                epoch_id: None,
                context_id: Some(u256_to_request_id(context_id)),
            };

            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                extra_data: extra_data.to_vec(),
                epoch_id: None,
                context_id: Some(u256_to_request_id(context_id)),
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

pub struct UserDecryptionExtraData {
    pub user_address: Address,
    pub public_key: Bytes,
}

impl UserDecryptionExtraData {
    pub fn new(user_address: Address, public_key: Bytes) -> Self {
        Self {
            user_address,
            public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
        transports::http::reqwest,
    };
    use connector_utils::tests::rand::{rand_address, rand_sns_ct};
    use fhevm_gateway_bindings::decryption::Decryption::CtHandleContractPair;
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;

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
        #[allow(unused)]
        Irrecoverable,
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
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        let sns_ct = rand_sns_ct();
        let acl_contracts_mock = HashMap::from([(
            extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap(),
            ACL::new(Address::default(), mock_provider.clone()),
        )]);

        let sns_ciphertexts = vec![sns_ct];
        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts_mock,
            s3_service,
        );

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
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());

        let sns_ct = rand_sns_ct();
        let acl_contracts_mock = HashMap::from([(
            extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap(),
            ACL::new(Address::default(), mock_provider.clone()),
        )]);

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
        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts_mock,
            s3_service,
        );

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
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());

        let sns_ct = rand_sns_ct();
        let acl_contracts_mock = HashMap::from([(
            extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap(),
            ACL::new(Address::default(), mock_provider.clone()),
        )]);

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
        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts_mock,
            s3_service,
        );

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
            .check_ciphertexts_allowed_for_public_decryption(&[sns_ct], &Bytes::from(vec![1u8; 256]))
            .await;

        let err = result.expect_err("no reachable validator, so the RPC-verified ACL check errors");
        assert!(
            !err.to_string().contains("not a trusted authorization source"),
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

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _context_id: U256) -> Result<(), ProcessingError> {
            Ok(())
        }
    }
}
