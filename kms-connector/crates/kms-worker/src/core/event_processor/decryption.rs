use crate::core::{
    config::{Config, HostChainKind},
    event_processor::{ProcessingError, context::ContextManager, s3::S3Service},
    solana_acl::{
        AclPermissionWitness, AclRecordWitness, HandleBytes, HandleMaterialCommitmentWitness,
        SolanaAclVerifier, SolanaPubkeyBytes, UserDecryptionDelegationWitness,
        WILDCARD_APP_CONTEXT, decode_acl_permission_witness, decode_acl_record_witness,
        decode_handle_material_commitment_witness, decode_user_decryption_delegation_witness,
    },
};
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
    extra_data::{
        EXTRA_DATA_V1_LENGTH, EXTRA_DATA_V1_VERSION, EXTRA_DATA_V2_LENGTH, EXTRA_DATA_V2_VERSION,
        parse_extra_data,
    },
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

const SOLANA_GATEWAY_ACL_POC_MAGIC: &[u8] = b"ZAMA_SOLANA_ACL_GATEWAY_POC_V0";
const SOLANA_GATEWAY_ACL_MODE_PUBLIC: u8 = 0;
const SOLANA_GATEWAY_ACL_MODE_DIRECT: u8 = 1;
const SOLANA_GATEWAY_ACL_MODE_DELEGATED: u8 = 2;

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

    /// The native Solana ACL verifiers configured for each Solana host chain.
    solana_acl_verifiers: HashMap<u64, SolanaAclVerifier>,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<GP>,
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
        let solana_acl_verifiers = config
            .host_chains
            .iter()
            .filter(|host_chain| host_chain.chain_kind == HostChainKind::Solana)
            .filter_map(|host_chain| {
                host_chain
                    .solana_host_program_id
                    .map(|program_id| (host_chain.chain_id, SolanaAclVerifier::new(program_id)))
            })
            .collect();

        Self {
            domain,
            context_manager,
            decryption_contract,
            acl_contracts,
            host_chain_kinds,
            solana_acl_verifiers,
            s3_service,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        extra_data: &Bytes,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        let solana_payload =
            self.solana_gateway_acl_payload_if_needed(sns_ciphertexts, extra_data)?;
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            if self.host_chain_kind(ct_chain_id) == HostChainKind::Solana {
                let payload = solana_payload.as_ref().ok_or_else(|| {
                    ProcessingError::Recoverable(anyhow!(
                        "Solana ACL verification for chain id {ct_chain_id} requires a Gateway-PoC \
                        Solana ACL witness payload in extraData"
                    ))
                })?;
                self.check_solana_public_decryption(ct_chain_id, ct, payload)?;
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

        let (ct_handle_contract_pairs, delegator_address, extra_data) =
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

        let solana_payload =
            self.solana_gateway_acl_payload_if_needed(sns_ciphertexts, &extra_data)?;
        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            if self.host_chain_kind(ct_chain_id) == HostChainKind::Solana {
                let payload = solana_payload.as_ref().ok_or_else(|| {
                    ProcessingError::Recoverable(anyhow!(
                        "Solana ACL verification for chain id {ct_chain_id} requires a Gateway-PoC \
                        Solana ACL witness payload in extraData"
                    ))
                })?;
                self.check_solana_user_decryption(
                    ct_chain_id,
                    ct,
                    payload,
                    delegator_address.is_some(),
                )?;
                continue;
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

    fn host_chain_kind(&self, chain_id: u64) -> HostChainKind {
        self.host_chain_kinds
            .get(&chain_id)
            .copied()
            .unwrap_or(HostChainKind::Evm)
    }

    fn solana_gateway_acl_payload_if_needed(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        extra_data: &Bytes,
    ) -> Result<Option<SolanaGatewayAclPayload>, ProcessingError> {
        let solana_handles = sns_ciphertexts
            .iter()
            .filter_map(|ct| {
                let chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice()).ok()?;
                (self.host_chain_kind(chain_id) == HostChainKind::Solana)
                    .then(|| fixed_bytes_to_handle(ct.ctHandle))
            })
            .collect::<Vec<_>>();
        if solana_handles.is_empty() {
            return Ok(None);
        }

        let payload = parse_solana_gateway_acl_payload(extra_data)?;
        if payload.entries.len() != solana_handles.len() {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana ACL witness entry count {} does not match Solana ciphertext count {}",
                payload.entries.len(),
                solana_handles.len()
            )));
        }

        let mut entries_by_handle = HashMap::<HandleBytes, ()>::new();
        for entry in &payload.entries {
            if entries_by_handle.insert(entry.handle, ()).is_some() {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "Solana ACL witness payload contains duplicate handle {}",
                    hex::encode(entry.handle)
                )));
            }
        }
        for handle in solana_handles {
            if !entries_by_handle.contains_key(&handle) {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "Solana ACL witness payload is missing handle {}",
                    hex::encode(handle)
                )));
            }
        }

        Ok(Some(payload))
    }

    fn check_solana_public_decryption(
        &self,
        chain_id: u64,
        ct: &SnsCiphertextMaterial,
        payload: &SolanaGatewayAclPayload,
    ) -> Result<(), ProcessingError> {
        if payload.mode != SOLANA_GATEWAY_ACL_MODE_PUBLIC {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana public decryption requires Gateway-PoC ACL mode {}, got {}",
                SOLANA_GATEWAY_ACL_MODE_PUBLIC,
                payload.mode
            )));
        }

        let verifier = self.solana_acl_verifier(chain_id)?;
        let handle = fixed_bytes_to_handle(ct.ctHandle);
        let entry = payload.entry_for_handle(handle)?;
        verify_solana_material_matches_sns(ct, &entry.material)?;
        verifier
            .verify_public_decrypt_with_material(&entry.acl_record, &entry.material, handle)
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!("Solana public ACL verification failed: {e}"))
            })
    }

    fn check_solana_user_decryption(
        &self,
        chain_id: u64,
        ct: &SnsCiphertextMaterial,
        payload: &SolanaGatewayAclPayload,
        delegated: bool,
    ) -> Result<(), ProcessingError> {
        let expected_mode = if delegated {
            SOLANA_GATEWAY_ACL_MODE_DELEGATED
        } else {
            SOLANA_GATEWAY_ACL_MODE_DIRECT
        };
        if payload.mode != expected_mode {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana user decryption requires Gateway-PoC ACL mode {expected_mode}, got {}",
                payload.mode
            )));
        }
        if is_zero_pubkey(payload.subject) {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana user decryption witness has an empty subject"
            )));
        }
        if is_zero_pubkey(payload.app_account) {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana user decryption witness has an empty app account"
            )));
        }
        if payload.app_account == WILDCARD_APP_CONTEXT {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana user decryption witness uses the reserved wildcard app-context sentinel"
            )));
        }
        if payload.subject == payload.app_account {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana user decryption witness subject must differ from app account"
            )));
        }

        let verifier = self.solana_acl_verifier(chain_id)?;
        let handle = fixed_bytes_to_handle(ct.ctHandle);
        let entry = payload.entry_for_handle(handle)?;
        verify_solana_material_matches_sns(ct, &entry.material)?;

        if delegated {
            let delegation = payload.delegation.as_ref().ok_or_else(|| {
                ProcessingError::Recoverable(anyhow!(
                    "Solana delegated user decryption witness is missing a delegation account"
                ))
            })?;
            if is_zero_pubkey(payload.delegate)
                || payload.delegate == WILDCARD_APP_CONTEXT
                || payload.delegation_counter == 0
            {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "Solana delegated user decryption witness has an empty or reserved delegate or counter"
                )));
            }
            if payload.delegate == payload.subject || payload.delegate == payload.app_account {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "Solana delegated user decryption witness has invalid role equality"
                )));
            }
            verifier
                .verify_delegated_user_decrypt_with_material(
                    &entry.acl_record,
                    &entry.overflow_permissions,
                    delegation,
                    &entry.material,
                    handle,
                    payload.subject,
                    payload.delegate,
                    payload.app_account,
                    payload.delegation_counter,
                    payload.observed_slot,
                    &payload.allowed_acl_domain_keys,
                )
                .map_err(|e| {
                    ProcessingError::Recoverable(anyhow!(
                        "Solana delegated ACL verification failed: {e}"
                    ))
                })?;
        } else {
            verifier
                .verify_user_decrypt_with_material(
                    &entry.acl_record,
                    &entry.overflow_permissions,
                    &entry.material,
                    handle,
                    payload.subject,
                    &payload.allowed_acl_domain_keys,
                )
                .map_err(|e| {
                    ProcessingError::Recoverable(anyhow!(
                        "Solana direct ACL verification failed: {e}"
                    ))
                })?;
        }

        verifier
            .verify_user_decrypt(
                &entry.acl_record,
                &entry.overflow_permissions,
                handle,
                payload.app_account,
                &payload.allowed_acl_domain_keys,
            )
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!(
                    "Solana app-context ACL verification failed: {e}"
                ))
            })
    }

    fn solana_acl_verifier(&self, chain_id: u64) -> Result<&SolanaAclVerifier, ProcessingError> {
        self.solana_acl_verifiers.get(&chain_id).ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana host chain {chain_id} is missing solana_host_program_id; refusing ACL \
                verification"
            ))
        })
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

#[derive(Clone, Debug)]
struct SolanaGatewayAclPayload {
    mode: u8,
    subject: SolanaPubkeyBytes,
    delegate: SolanaPubkeyBytes,
    app_account: SolanaPubkeyBytes,
    delegation_counter: u64,
    observed_slot: u64,
    allowed_acl_domain_keys: Vec<SolanaPubkeyBytes>,
    delegation: Option<UserDecryptionDelegationWitness>,
    entries: Vec<SolanaGatewayAclHandleEntry>,
}

impl SolanaGatewayAclPayload {
    fn entry_for_handle(
        &self,
        handle: HandleBytes,
    ) -> Result<&SolanaGatewayAclHandleEntry, ProcessingError> {
        self.entries
            .iter()
            .find(|entry| entry.handle == handle)
            .ok_or_else(|| {
                ProcessingError::Recoverable(anyhow!(
                    "Solana ACL witness payload is missing handle {}",
                    hex::encode(handle)
                ))
            })
    }
}

#[derive(Clone, Debug)]
struct SolanaGatewayAclHandleEntry {
    handle: HandleBytes,
    acl_record: AclRecordWitness,
    material: HandleMaterialCommitmentWitness,
    overflow_permissions: Vec<AclPermissionWitness>,
}

struct RawSolanaAccountWitness {
    account_key: SolanaPubkeyBytes,
    owner: SolanaPubkeyBytes,
    data: Vec<u8>,
}

fn parse_solana_gateway_acl_payload(
    extra_data: &[u8],
) -> Result<SolanaGatewayAclPayload, ProcessingError> {
    let payload_offset = solana_gateway_acl_payload_offset(extra_data)
        .map_err(|e| ProcessingError::Recoverable(anyhow!("{e}")))?;
    let payload_bytes = &extra_data[payload_offset..];
    if !payload_bytes.starts_with(SOLANA_GATEWAY_ACL_POC_MAGIC) {
        return Err(ProcessingError::Recoverable(anyhow!(
            "Solana ACL verification requires Gateway-PoC witness magic {} after the standard \
            extraData context prefix",
            String::from_utf8_lossy(SOLANA_GATEWAY_ACL_POC_MAGIC)
        )));
    }

    let mut cursor =
        SolanaGatewayAclCursor::new(&payload_bytes[SOLANA_GATEWAY_ACL_POC_MAGIC.len()..]);
    let mode = cursor.read_u8()?;
    if !matches!(
        mode,
        SOLANA_GATEWAY_ACL_MODE_PUBLIC
            | SOLANA_GATEWAY_ACL_MODE_DIRECT
            | SOLANA_GATEWAY_ACL_MODE_DELEGATED
    ) {
        return Err(ProcessingError::Recoverable(anyhow!(
            "unsupported Solana Gateway-PoC ACL mode {mode}"
        )));
    }
    let subject = cursor.read_bytes_32()?;
    let delegate = cursor.read_bytes_32()?;
    let app_account = cursor.read_bytes_32()?;
    let delegation_counter = cursor.read_u64()?;
    let observed_slot = cursor.read_u64()?;

    let allowed_domain_count = cursor.read_u16()? as usize;
    let mut allowed_acl_domain_keys = Vec::with_capacity(allowed_domain_count);
    for _ in 0..allowed_domain_count {
        allowed_acl_domain_keys.push(cursor.read_bytes_32()?);
    }

    let delegation = if mode == SOLANA_GATEWAY_ACL_MODE_DELEGATED {
        let witness = cursor.read_account_witness()?;
        Some(
            decode_user_decryption_delegation_witness(
                witness.account_key,
                witness.owner,
                &witness.data,
            )
            .map_err(|e| {
                ProcessingError::Recoverable(anyhow!(
                    "invalid Solana delegation account witness: {e}"
                ))
            })?,
        )
    } else {
        None
    };

    let entry_count = cursor.read_u16()? as usize;
    let mut entries = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        entries.push(cursor.read_handle_entry()?);
    }
    if cursor.remaining() != 0 {
        return Err(ProcessingError::Recoverable(anyhow!(
            "Solana Gateway-PoC ACL witness payload has {} trailing bytes",
            cursor.remaining()
        )));
    }

    Ok(SolanaGatewayAclPayload {
        mode,
        subject,
        delegate,
        app_account,
        delegation_counter,
        observed_slot,
        allowed_acl_domain_keys,
        delegation,
        entries,
    })
}

fn solana_gateway_acl_payload_offset(extra_data: &[u8]) -> Result<usize, String> {
    let Some(version) = extra_data.first().copied() else {
        return Err(
            "extraData is empty; Solana Gateway-PoC ACL witnesses require a standard \
            v1/v2 extraData context prefix"
                .to_string(),
        );
    };

    match version {
        EXTRA_DATA_V1_VERSION => {
            if extra_data.len() < EXTRA_DATA_V1_LENGTH {
                Err(format!(
                    "extraData too short for v1 Solana witness prefix: {} bytes, expected at \
                    least {}",
                    extra_data.len(),
                    EXTRA_DATA_V1_LENGTH
                ))
            } else {
                Ok(EXTRA_DATA_V1_LENGTH)
            }
        }
        EXTRA_DATA_V2_VERSION => {
            if extra_data.len() < EXTRA_DATA_V2_LENGTH {
                Err(format!(
                    "extraData too short for v2 Solana witness prefix: {} bytes, expected at \
                    least {}",
                    extra_data.len(),
                    EXTRA_DATA_V2_LENGTH
                ))
            } else {
                Ok(EXTRA_DATA_V2_LENGTH)
            }
        }
        _ => Err(format!(
            "unsupported extraData version 0x{version:02x}; Solana Gateway-PoC ACL witnesses \
            require v1 or v2 context-prefixed extraData"
        )),
    }
}

struct SolanaGatewayAclCursor<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> SolanaGatewayAclCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn read_handle_entry(&mut self) -> Result<SolanaGatewayAclHandleEntry, ProcessingError> {
        let handle = self.read_bytes_32()?;

        let record_witness = self.read_account_witness()?;
        let acl_record = decode_acl_record_witness(
            record_witness.account_key,
            record_witness.owner,
            &record_witness.data,
        )
        .map_err(|e| {
            ProcessingError::Recoverable(anyhow!("invalid Solana ACL record witness: {e}"))
        })?;

        let material_witness = self.read_account_witness()?;
        let material = decode_handle_material_commitment_witness(
            material_witness.account_key,
            material_witness.owner,
            &material_witness.data,
        )
        .map_err(|e| {
            ProcessingError::Recoverable(anyhow!("invalid Solana material commitment witness: {e}"))
        })?;

        let permission_count = self.read_u16()? as usize;
        let mut overflow_permissions = Vec::with_capacity(permission_count);
        for _ in 0..permission_count {
            let permission_witness = self.read_account_witness()?;
            overflow_permissions.push(
                decode_acl_permission_witness(
                    permission_witness.account_key,
                    permission_witness.owner,
                    &permission_witness.data,
                )
                .map_err(|e| {
                    ProcessingError::Recoverable(anyhow!(
                        "invalid Solana overflow permission witness: {e}"
                    ))
                })?,
            );
        }

        Ok(SolanaGatewayAclHandleEntry {
            handle,
            acl_record,
            material,
            overflow_permissions,
        })
    }

    fn read_account_witness(&mut self) -> Result<RawSolanaAccountWitness, ProcessingError> {
        let account_key = self.read_bytes_32()?;
        let owner = self.read_bytes_32()?;
        let data_len = self.read_u32()? as usize;
        let data = self.read_exact(data_len)?.to_vec();
        Ok(RawSolanaAccountWitness {
            account_key,
            owner,
            data,
        })
    }

    fn read_bytes_32(&mut self) -> Result<[u8; 32], ProcessingError> {
        let bytes = self.read_exact(32)?;
        let mut output = [0; 32];
        output.copy_from_slice(bytes);
        Ok(output)
    }

    fn read_u64(&mut self) -> Result<u64, ProcessingError> {
        let bytes = self.read_exact(8)?;
        Ok(u64::from_le_bytes(
            bytes.try_into().expect("slice has 8 bytes"),
        ))
    }

    fn read_u32(&mut self) -> Result<u32, ProcessingError> {
        let bytes = self.read_exact(4)?;
        Ok(u32::from_le_bytes(
            bytes.try_into().expect("slice has 4 bytes"),
        ))
    }

    fn read_u16(&mut self) -> Result<u16, ProcessingError> {
        let bytes = self.read_exact(2)?;
        Ok(u16::from_le_bytes(
            bytes.try_into().expect("slice has 2 bytes"),
        ))
    }

    fn read_u8(&mut self) -> Result<u8, ProcessingError> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], ProcessingError> {
        let end = self.offset.checked_add(len).ok_or_else(|| {
            ProcessingError::Recoverable(anyhow!(
                "Solana Gateway-PoC ACL witness payload offset overflow"
            ))
        })?;
        if end > self.data.len() {
            return Err(ProcessingError::Recoverable(anyhow!(
                "Solana Gateway-PoC ACL witness payload is truncated at byte {}, needed {} more \
                bytes",
                self.offset,
                end - self.data.len()
            )));
        }
        let slice = &self.data[self.offset..end];
        self.offset = end;
        Ok(slice)
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }
}

fn verify_solana_material_matches_sns(
    ct: &SnsCiphertextMaterial,
    material: &HandleMaterialCommitmentWitness,
) -> Result<(), ProcessingError> {
    let key_id = ct.keyId.to_be_bytes::<32>();
    if material.key_id != key_id {
        return Err(ProcessingError::Recoverable(anyhow!(
            "Solana material witness key_id does not match Gateway SNS material for handle {}",
            hex::encode(ct.ctHandle)
        )));
    }

    let sns_digest = fixed_bytes_to_handle(ct.snsCiphertextDigest);
    if material.sns_ciphertext_digest != sns_digest {
        return Err(ProcessingError::Recoverable(anyhow!(
            "Solana material witness SNS digest does not match Gateway SNS material for handle {}",
            hex::encode(ct.ctHandle)
        )));
    }

    Ok(())
}

fn fixed_bytes_to_handle(bytes: FixedBytes<32>) -> HandleBytes {
    let mut output = [0; 32];
    output.copy_from_slice(bytes.as_slice());
    output
}

fn is_zero_pubkey(pubkey: SolanaPubkeyBytes) -> bool {
    pubkey == [0; 32]
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
    use crate::core::solana_acl::{
        ACL_ROLE_PUBLIC_DECRYPT, ACL_ROLE_USE, HANDLE_MATERIAL_STATE_COMMITTED, SubjectRole,
        WILDCARD_APP_CONTEXT, acl_nonce_key, acl_record_address, anchor_account_discriminator,
        handle_material_address, handle_material_commitment_hash,
        user_decryption_delegation_address,
    };
    use alloy::{
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
        transports::http::reqwest,
    };
    use connector_utils::tests::rand::{rand_address, rand_sns_ct};
    use fhevm_gateway_bindings::decryption::Decryption::CtHandleContractPair;
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;

    const TEST_HOST_PROGRAM_ID: SolanaPubkeyBytes = [42; 32];
    const TEST_DOMAIN: SolanaPubkeyBytes = [1; 32];
    const TEST_APP_ACCOUNT: SolanaPubkeyBytes = [2; 32];
    const TEST_OWNER: SolanaPubkeyBytes = [3; 32];
    const TEST_DELEGATE: SolanaPubkeyBytes = [4; 32];
    const TEST_OBSERVED_SLOT: u64 = 500;
    const TEST_LABEL: [u8; 32] = *b"balance_________________________";
    const TEST_MAX_ACL_SUBJECTS: usize = 8;

    fn solana_config_for(chain_id: u64) -> Config {
        let mut config = Config::default();
        config.host_chains[0].chain_id = chain_id;
        config.host_chains[0].chain_kind = HostChainKind::Solana;
        config.host_chains[0].solana_host_program_id = Some(TEST_HOST_PROGRAM_ID);
        config
    }

    fn test_acl_record(handle: HandleBytes) -> AclRecordWitness {
        let nonce_key = acl_nonce_key(TEST_DOMAIN, TEST_APP_ACCOUNT, TEST_LABEL);
        let (account_key, bump) = acl_record_address(TEST_HOST_PROGRAM_ID, nonce_key, 8);
        AclRecordWitness {
            account_key,
            owner: TEST_HOST_PROGRAM_ID,
            handle,
            nonce_key,
            nonce_sequence: 8,
            acl_domain_key: TEST_DOMAIN,
            app_account: TEST_APP_ACCOUNT,
            encrypted_value_label: TEST_LABEL,
            subjects: vec![
                SubjectRole {
                    subject: TEST_OWNER,
                    role_flags: ACL_ROLE_USE | ACL_ROLE_PUBLIC_DECRYPT,
                },
                SubjectRole {
                    subject: TEST_APP_ACCOUNT,
                    role_flags: ACL_ROLE_USE,
                },
            ],
            overflow_subject_count: 0,
            public_decrypt: true,
            material_commitment: [0; 32],
            material_commitment_hash: [0; 32],
            material_key_id: [0; 32],
            created_slot: TEST_OBSERVED_SLOT,
            bump,
        }
    }

    fn test_material(
        ct: &SnsCiphertextMaterial,
        record: &AclRecordWitness,
    ) -> HandleMaterialCommitmentWitness {
        let (account_key, bump) = handle_material_address(TEST_HOST_PROGRAM_ID, record.account_key);
        let key_id = ct.keyId.to_be_bytes::<32>();
        let ciphertext_digest = [22; 32];
        let sns_ciphertext_digest = fixed_bytes_to_handle(ct.snsCiphertextDigest);
        let coprocessor_set_digest = [24; 32];
        let material_commitment_hash = handle_material_commitment_hash(
            TEST_HOST_PROGRAM_ID,
            account_key,
            record.account_key,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
        );
        HandleMaterialCommitmentWitness {
            account_key,
            owner: TEST_HOST_PROGRAM_ID,
            acl_record: record.account_key,
            handle: record.handle,
            key_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            coprocessor_set_digest,
            material_commitment_hash,
            created_slot: TEST_OBSERVED_SLOT,
            state: HANDLE_MATERIAL_STATE_COMMITTED,
            bump,
        }
    }

    fn seal_record_to_material(
        record: &mut AclRecordWitness,
        material: &HandleMaterialCommitmentWitness,
    ) {
        record.material_commitment = material.account_key;
        record.material_commitment_hash = material.material_commitment_hash;
        record.material_key_id = material.key_id;
    }

    fn test_delegation() -> UserDecryptionDelegationWitness {
        let (account_key, bump) = user_decryption_delegation_address(
            TEST_HOST_PROGRAM_ID,
            TEST_OWNER,
            TEST_DELEGATE,
            TEST_APP_ACCOUNT,
        );
        UserDecryptionDelegationWitness {
            account_key,
            owner: TEST_HOST_PROGRAM_ID,
            delegator: TEST_OWNER,
            delegate: TEST_DELEGATE,
            app_account: TEST_APP_ACCOUNT,
            expiration_slot: TEST_OBSERVED_SLOT + 20,
            delegation_counter: 9,
            last_update_slot: TEST_OBSERVED_SLOT - 1,
            revoked: false,
            bump,
        }
    }

    fn encode_acl_record(record: &AclRecordWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("AclRecord").to_vec();
        data.extend_from_slice(&record.handle);
        data.extend_from_slice(&record.nonce_key);
        data.extend_from_slice(&record.nonce_sequence.to_le_bytes());
        data.extend_from_slice(&record.acl_domain_key);
        data.extend_from_slice(&record.app_account);
        data.extend_from_slice(&record.encrypted_value_label);
        for index in 0..TEST_MAX_ACL_SUBJECTS {
            let subject = record
                .subjects
                .get(index)
                .map(|entry| entry.subject)
                .unwrap_or([0; 32]);
            data.extend_from_slice(&subject);
        }
        for index in 0..TEST_MAX_ACL_SUBJECTS {
            let role_flags = record
                .subjects
                .get(index)
                .map(|entry| entry.role_flags)
                .unwrap_or(0);
            data.push(role_flags);
        }
        data.push(record.subjects.len() as u8);
        data.extend_from_slice(&record.overflow_subject_count.to_le_bytes());
        data.push(record.public_decrypt as u8);
        data.extend_from_slice(&record.material_commitment);
        data.extend_from_slice(&record.material_commitment_hash);
        data.extend_from_slice(&record.material_key_id);
        data.extend_from_slice(&record.created_slot.to_le_bytes());
        data.push(record.bump);
        data
    }

    fn encode_material(material: &HandleMaterialCommitmentWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("HandleMaterialCommitment").to_vec();
        data.extend_from_slice(&material.acl_record);
        data.extend_from_slice(&material.handle);
        data.extend_from_slice(&material.key_id);
        data.extend_from_slice(&material.ciphertext_digest);
        data.extend_from_slice(&material.sns_ciphertext_digest);
        data.extend_from_slice(&material.coprocessor_set_digest);
        data.extend_from_slice(&material.material_commitment_hash);
        data.extend_from_slice(&material.created_slot.to_le_bytes());
        data.push(material.state);
        data.push(material.bump);
        data
    }

    fn encode_delegation(delegation: &UserDecryptionDelegationWitness) -> Vec<u8> {
        let mut data = anchor_account_discriminator("UserDecryptionDelegation").to_vec();
        data.extend_from_slice(&delegation.delegator);
        data.extend_from_slice(&delegation.delegate);
        data.extend_from_slice(&delegation.app_account);
        data.extend_from_slice(&delegation.expiration_slot.to_le_bytes());
        data.extend_from_slice(&delegation.delegation_counter.to_le_bytes());
        data.extend_from_slice(&delegation.last_update_slot.to_le_bytes());
        data.push(delegation.revoked as u8);
        data.push(delegation.bump);
        data
    }

    fn push_account_witness(
        data: &mut Vec<u8>,
        account_key: SolanaPubkeyBytes,
        owner: SolanaPubkeyBytes,
        account_data: Vec<u8>,
    ) {
        data.extend_from_slice(&account_key);
        data.extend_from_slice(&owner);
        data.extend_from_slice(&(account_data.len() as u32).to_le_bytes());
        data.extend_from_slice(&account_data);
    }

    fn solana_gateway_acl_extra_data(
        mode: u8,
        record: &AclRecordWitness,
        material: &HandleMaterialCommitmentWitness,
        delegation: Option<&UserDecryptionDelegationWitness>,
    ) -> Bytes {
        solana_gateway_acl_extra_data_with_roles(
            mode,
            TEST_OWNER,
            delegation.map_or(TEST_DELEGATE, |d| d.delegate),
            TEST_APP_ACCOUNT,
            record,
            material,
            delegation,
        )
    }

    fn solana_gateway_acl_extra_data_with_roles(
        mode: u8,
        subject: SolanaPubkeyBytes,
        delegate: SolanaPubkeyBytes,
        app_account: SolanaPubkeyBytes,
        record: &AclRecordWitness,
        material: &HandleMaterialCommitmentWitness,
        delegation: Option<&UserDecryptionDelegationWitness>,
    ) -> Bytes {
        let mut data = vec![EXTRA_DATA_V1_VERSION];
        data.extend_from_slice(&U256::from(1u64).to_be_bytes::<32>());
        data.extend_from_slice(SOLANA_GATEWAY_ACL_POC_MAGIC);
        data.push(mode);
        data.extend_from_slice(&subject);
        data.extend_from_slice(&delegate);
        data.extend_from_slice(&app_account);
        data.extend_from_slice(&delegation.map_or(0, |d| d.delegation_counter).to_le_bytes());
        data.extend_from_slice(&TEST_OBSERVED_SLOT.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&TEST_DOMAIN);
        if let Some(delegation) = delegation {
            push_account_witness(
                &mut data,
                delegation.account_key,
                delegation.owner,
                encode_delegation(delegation),
            );
        }
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&record.handle);
        push_account_witness(
            &mut data,
            record.account_key,
            record.owner,
            encode_acl_record(record),
        );
        push_account_witness(
            &mut data,
            material.account_key,
            material.owner,
            encode_material(material),
        );
        data.extend_from_slice(&0u16.to_le_bytes());
        Bytes::from(data)
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

    #[tokio::test]
    async fn check_solana_chain_public_decryption_fails_closed_before_evm_acl() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let mut config = Config::default();
        config.host_chains[0].chain_id = ct_chain_id;
        config.host_chains[0].chain_kind = HostChainKind::Solana;
        let acl_contracts_mock = HashMap::from([(
            ct_chain_id,
            ACL::new(Address::default(), mock_provider.clone()),
        )]);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts_mock,
            s3_service,
        );

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[sns_ct], &Bytes::from(vec![]))
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => {
                assert!(e.to_string().contains("Solana Gateway-PoC ACL witnesses"))
            }
            other => panic!("expected recoverable Solana ACL error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn check_solana_chain_public_decryption_with_gateway_poc_witness() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let extra_data =
            solana_gateway_acl_extra_data(SOLANA_GATEWAY_ACL_MODE_PUBLIC, &record, &material, None);

        decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[sns_ct], &extra_data)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn check_solana_chain_public_decryption_rejects_wrong_material_key() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let mut material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        material.key_id = [99; 32];
        let extra_data =
            solana_gateway_acl_extra_data(SOLANA_GATEWAY_ACL_MODE_PUBLIC, &record, &material, None);

        let result = decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&[sns_ct], &extra_data)
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => assert!(e.to_string().contains("key_id")),
            other => panic!("expected material key mismatch, got {other:?}"),
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

    #[tokio::test]
    async fn check_solana_chain_user_decryption_fails_closed_before_evm_acl() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let mut config = Config::default();
        config.host_chains[0].chain_id = ct_chain_id;
        config.host_chains[0].chain_kind = HostChainKind::Solana;
        let acl_contracts_mock = HashMap::from([(
            ct_chain_id,
            ACL::new(Address::default(), mock_provider.clone()),
        )]);
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            ..Default::default()
        }
        .abi_encode();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider,
            acl_contracts_mock,
            s3_service,
        );

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => {
                assert!(e.to_string().contains("Solana Gateway-PoC ACL witnesses"))
            }
            other => panic!("expected recoverable Solana ACL error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn check_solana_chain_user_decryption_with_gateway_poc_witness() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let extra_data =
            solana_gateway_acl_extra_data(SOLANA_GATEWAY_ACL_MODE_DIRECT, &record, &material, None);
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn check_solana_chain_user_decryption_rejects_subject_app_equality() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let extra_data = solana_gateway_acl_extra_data_with_roles(
            SOLANA_GATEWAY_ACL_MODE_DIRECT,
            TEST_APP_ACCOUNT,
            TEST_DELEGATE,
            TEST_APP_ACCOUNT,
            &record,
            &material,
            None,
        );
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => assert!(
                e.to_string()
                    .contains("subject must differ from app account"),
                "unexpected error: {e}"
            ),
            other => panic!("expected subject/app equality rejection, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn check_solana_chain_user_decryption_rejects_wildcard_app_context() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let extra_data = solana_gateway_acl_extra_data_with_roles(
            SOLANA_GATEWAY_ACL_MODE_DIRECT,
            TEST_OWNER,
            TEST_DELEGATE,
            WILDCARD_APP_CONTEXT,
            &record,
            &material,
            None,
        );
        let calldata = userDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => assert!(
                e.to_string()
                    .contains("reserved wildcard app-context sentinel"),
                "unexpected error: {e}"
            ),
            other => panic!("expected wildcard app-context rejection, got {other:?}"),
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

    #[tokio::test]
    async fn check_solana_chain_delegated_user_decryption_with_gateway_poc_witness() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let delegation = test_delegation();
        let extra_data = solana_gateway_acl_extra_data(
            SOLANA_GATEWAY_ACL_MODE_DELEGATED,
            &record,
            &material,
            Some(&delegation),
        );
        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn check_solana_chain_delegated_user_decryption_rejects_wildcard_delegate() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let mut delegation = test_delegation();
        delegation.delegate = WILDCARD_APP_CONTEXT;
        let (account_key, bump) = user_decryption_delegation_address(
            TEST_HOST_PROGRAM_ID,
            delegation.delegator,
            delegation.delegate,
            delegation.app_account,
        );
        delegation.account_key = account_key;
        delegation.bump = bump;
        let extra_data = solana_gateway_acl_extra_data(
            SOLANA_GATEWAY_ACL_MODE_DELEGATED,
            &record,
            &material,
            Some(&delegation),
        );
        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => assert!(
                e.to_string()
                    .contains("empty or reserved delegate or counter"),
                "unexpected error: {e}"
            ),
            other => panic!("expected wildcard delegate rejection, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn check_solana_chain_delegated_user_decryption_rejects_delegate_app_equality() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let sns_ct = rand_sns_ct();
        let ct_chain_id = extract_chain_id_from_handle(sns_ct.ctHandle.as_slice()).unwrap();
        let config = solana_config_for(ct_chain_id);
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(
            &config,
            MockContextManager,
            mock_provider.clone(),
            HashMap::from([(ct_chain_id, ACL::new(Address::default(), mock_provider))]),
            s3_service,
        );
        let mut record = test_acl_record(fixed_bytes_to_handle(sns_ct.ctHandle));
        let material = test_material(&sns_ct, &record);
        seal_record_to_material(&mut record, &material);
        let mut delegation = test_delegation();
        delegation.delegate = TEST_APP_ACCOUNT;
        let (account_key, bump) = user_decryption_delegation_address(
            TEST_HOST_PROGRAM_ID,
            delegation.delegator,
            delegation.delegate,
            delegation.app_account,
        );
        delegation.account_key = account_key;
        delegation.bump = bump;
        let extra_data = solana_gateway_acl_extra_data(
            SOLANA_GATEWAY_ACL_MODE_DELEGATED,
            &record,
            &material,
            Some(&delegation),
        );
        let calldata = delegatedUserDecryptionRequestCall {
            ctHandleContractPairs: vec![CtHandleContractPair {
                ctHandle: sns_ct.ctHandle,
                contractAddress: rand_address(),
            }],
            extraData: extra_data,
            ..Default::default()
        }
        .abi_encode();

        let result = decryption_processor
            .check_ciphertexts_allowed_for_user_decryption(calldata, &[sns_ct], Address::default())
            .await;

        match result {
            Err(ProcessingError::Recoverable(e)) => assert!(
                e.to_string().contains("invalid role equality"),
                "unexpected error: {e}"
            ),
            other => panic!("expected delegate/app equality rejection, got {other:?}"),
        }
    }

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _context_id: U256) -> Result<(), ProcessingError> {
            Ok(())
        }
    }
}
