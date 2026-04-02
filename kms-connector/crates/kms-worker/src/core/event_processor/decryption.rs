use crate::core::{
    config::Config,
    event_processor::{ProcessingError, context::ContextManager, s3::S3Service},
    solana_state::{
        EvmAddress as SolanaEvmAddress, Handle as SolanaHandle, Pubkey as SolanaHostPubkey,
        SolanaStateClient, host_identity_from_evm_address,
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
    extra_data::{parse_extra_data_context, parse_extra_data_identities},
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
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Clone)]
pub enum HostAclBackend<HP: Provider> {
    Evm(ACLInstance<HP>),
    Solana { client: SolanaStateClient },
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

    /// The instances of the host chains `ACL` contracts used to check the decryption ACL.
    acl_contracts: HashMap<u64, HostAclBackend<HP>>,

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
        acl_contracts: HashMap<u64, HostAclBackend<HP>>,
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

        Self {
            domain,
            context_manager,
            decryption_contract,
            acl_contracts,
            s3_service,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting ACL check for {} handles...",
            sns_ciphertexts.len()
        );

        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            let Some(backend) = self.acl_contracts.get(&ct_chain_id) else {
                return Err(ProcessingError::Recoverable(anyhow!(
                    "No ACL contract config found for chain id {ct_chain_id}"
                )));
            };

            if !self
                .is_allowed_for_public_decryption(backend, ct.ctHandle)
                .await?
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

        let (ct_handle_contract_pairs, contract_addresses, extra_data, delegator_address) =
            match delegatedUserDecryptionRequestCall::abi_decode(calldata.as_slice()) {
                Ok(parsed_calldata) => (
                    parsed_calldata.ctHandleContractPairs,
                    parsed_calldata.contractsInfo.addresses,
                    parsed_calldata.extraData,
                    Some(parsed_calldata.delegationAccounts.delegatorAddress),
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
                        parsed_calldata.contractsInfo.addresses,
                        parsed_calldata.extraData,
                        None,
                    )
                }
            };
        let identity_overrides =
            parse_user_identity_overrides(&contract_addresses, extra_data.as_ref())?;

        let contracts_map = HashMap::<FixedBytes<32>, Address, DefaultHashBuilder>::from_iter(
            ct_handle_contract_pairs
                .iter()
                .map(|c| (c.ctHandle, c.contractAddress)),
        );
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            let backend = self.acl_contracts.get(&ct_chain_id).ok_or_else(|| {
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
            let native_contract_identity = identity_overrides
                .contract_ids
                .as_ref()
                .and_then(|overrides| overrides.get(contract_address).copied());

            if let Some(delegator_addr) = delegator_address {
                self.inner_acl_check_for_delegated_user_decryption(
                    backend,
                    ct.ctHandle,
                    user_address,
                    identity_overrides.delegate_id,
                    *contract_address,
                    native_contract_identity,
                    delegator_addr,
                    identity_overrides.delegator_id,
                )
                .await?;
            } else {
                self.inner_acl_check_for_user_decryption(
                    backend,
                    ct.ctHandle,
                    user_address,
                    identity_overrides.user_id,
                    *contract_address,
                    native_contract_identity,
                )
                .await?;
            }
        }

        info!("ACL check passed for {} handles!", sns_ciphertexts.len());
        Ok(())
    }

    async fn inner_acl_check_for_delegated_user_decryption(
        &self,
        backend: &HostAclBackend<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        native_user_identity: Option<SolanaHostPubkey>,
        contract_address: Address,
        native_contract_identity: Option<SolanaHostPubkey>,
        delegator_address: Address,
        native_delegator_identity: Option<SolanaHostPubkey>,
    ) -> Result<(), ProcessingError> {
        let handle_hex = hex::encode(handle);
        let is_delegated = self
            .is_handle_delegated_for_user_decryption(
                backend,
                delegator_address,
                user_address,
                native_delegator_identity,
                native_user_identity,
                contract_address,
                native_contract_identity,
                handle,
            )
            .await?;

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
        backend: &HostAclBackend<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        native_user_identity: Option<SolanaHostPubkey>,
        contract_address: Address,
        native_contract_identity: Option<SolanaHostPubkey>,
    ) -> Result<(), ProcessingError> {
        let handle_hex = hex::encode(handle);
        let (user_allowed, contract_allowed) = self
            .is_allowed_for_user_decryption(
                backend,
                handle,
                user_address,
                native_user_identity,
                contract_address,
                native_contract_identity,
            )
            .await?;

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

    async fn is_allowed_for_public_decryption(
        &self,
        backend: &HostAclBackend<HP>,
        handle: FixedBytes<32>,
    ) -> Result<bool, ProcessingError> {
        match backend {
            HostAclBackend::Evm(acl_contract) => acl_contract
                .isAllowedForDecryption(handle)
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e))),
            HostAclBackend::Solana { client } => {
                let state = client
                    .fetch_state()
                    .await
                    .map_err(ProcessingError::Recoverable)?;
                Ok(state
                    .acl()
                    .is_allowed_for_decryption(SolanaHandle::from(fixed_bytes_to_array(handle))))
            }
        }
    }

    async fn is_allowed_for_user_decryption(
        &self,
        backend: &HostAclBackend<HP>,
        handle: FixedBytes<32>,
        user_address: Address,
        native_user_identity: Option<SolanaHostPubkey>,
        contract_address: Address,
        native_contract_identity: Option<SolanaHostPubkey>,
    ) -> Result<(bool, bool), ProcessingError> {
        match backend {
            HostAclBackend::Evm(acl_contract) => {
                let user_allowed_call = acl_contract.isAllowed(handle, user_address);
                let contract_allowed_call = acl_contract.isAllowed(handle, contract_address);
                tokio::try_join!(user_allowed_call.call(), contract_allowed_call.call())
                    .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))
            }
            HostAclBackend::Solana { client } => {
                let state = client
                    .fetch_state()
                    .await
                    .map_err(ProcessingError::Recoverable)?;
                let handle = SolanaHandle::from(fixed_bytes_to_array(handle));
                let user_identity = native_user_identity
                    .unwrap_or_else(|| solana_host_identity_from_evm_address(user_address));
                let contract_identity = native_contract_identity
                    .unwrap_or_else(|| solana_host_identity_from_evm_address(contract_address));
                Ok((
                    state.acl().persist_allowed(handle, user_identity),
                    state.acl().persist_allowed(handle, contract_identity),
                ))
            }
        }
    }

    async fn is_handle_delegated_for_user_decryption(
        &self,
        backend: &HostAclBackend<HP>,
        delegator_address: Address,
        user_address: Address,
        native_delegator_identity: Option<SolanaHostPubkey>,
        native_user_identity: Option<SolanaHostPubkey>,
        contract_address: Address,
        native_contract_identity: Option<SolanaHostPubkey>,
        handle: FixedBytes<32>,
    ) -> Result<bool, ProcessingError> {
        match backend {
            HostAclBackend::Evm(acl_contract) => acl_contract
                .isHandleDelegatedForUserDecryption(
                    delegator_address,
                    user_address,
                    contract_address,
                    handle,
                )
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e))),
            HostAclBackend::Solana { client } => {
                let state = client
                    .fetch_state()
                    .await
                    .map_err(ProcessingError::Recoverable)?;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|err| ProcessingError::Recoverable(anyhow!(err)))?
                    .as_secs();
                Ok(state.acl().is_handle_delegated_for_user_decryption(
                    native_delegator_identity
                        .unwrap_or_else(|| solana_host_identity_from_evm_address(delegator_address)),
                    native_user_identity
                        .unwrap_or_else(|| solana_host_identity_from_evm_address(user_address)),
                    native_contract_identity
                        .unwrap_or_else(|| solana_host_identity_from_evm_address(contract_address)),
                    SolanaHandle::from(fixed_bytes_to_array(handle)),
                    now,
                ))
            }
        }
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

        let context_id = self.extract_and_validate_context(extra_data).await?;
        let ciphertexts = self.prepare_ciphertexts(&key_id, sns_materials).await?;

        let request_id = Some(u256_to_request_id(decryption_id));

        // TODO(https://github.com/zama-ai/fhevm-internal/issues/1167):
        // Workaround for backward compatibility with relayer-sdk <=0.4.2.
        // The SDK sends extraData=0x00 in the user decryption request, but does not pass extraData
        // to the TKMS library during response signature verification (reconstruction step),
        // effectively verifying against empty bytes. We normalize 0x00 → vec![] here so the KMS
        // signs over empty extraData, matching what the SDK expects during verification.
        // This is fixed in relayer-sdk v0.5.0.
        let extra_data = if extra_data.as_ref() == [0x00] {
            vec![]
        } else {
            extra_data.to_vec()
        };

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
                extra_data,
                epoch_id: None,
                context_id,
            };

            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(self.domain.clone()),
                extra_data,
                epoch_id: None,
                context_id,
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

    /// Parses `extraData` for a context ID and validates it if present.
    async fn extract_and_validate_context(
        &self,
        extra_data: &[u8],
    ) -> Result<Option<RequestId>, ProcessingError> {
        match parse_extra_data_context(extra_data) {
            Err(e) => Err(ProcessingError::Irrecoverable(e)),
            Ok(None) => Ok(None),
            Ok(Some(context_id)) => {
                self.context_manager.validate_context(context_id).await?;
                Ok(Some(u256_to_request_id(context_id)))
            }
        }
    }
}

fn solana_host_identity_from_evm_address(address: Address) -> SolanaHostPubkey {
    host_identity_from_evm_address(SolanaEvmAddress::from(address.into_array()))
}

#[derive(Default)]
struct UserIdentityOverrides {
    user_id: Option<SolanaHostPubkey>,
    delegate_id: Option<SolanaHostPubkey>,
    delegator_id: Option<SolanaHostPubkey>,
    contract_ids: Option<HashMap<Address, SolanaHostPubkey>>,
}

fn parse_user_identity_overrides(
    contract_addresses: &[Address],
    extra_data: &[u8],
) -> Result<UserIdentityOverrides, ProcessingError> {
    let Some(identities) =
        parse_extra_data_identities(extra_data).map_err(ProcessingError::Irrecoverable)?
    else {
        return Ok(UserIdentityOverrides::default());
    };

    if identities.len() == contract_addresses.len() {
        return Ok(UserIdentityOverrides {
            contract_ids: Some(zip_contract_identities(contract_addresses, identities)?),
            ..UserIdentityOverrides::default()
        });
    }

    if identities.len() == contract_addresses.len() + 1 {
        let user_id = identities
            .first()
            .copied()
            .map(SolanaHostPubkey::new)
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("missing v2 user identity")))?;
        return Ok(UserIdentityOverrides {
            user_id: Some(user_id),
            contract_ids: Some(zip_contract_identities(
                contract_addresses,
                identities.into_iter().skip(1).collect(),
            )?),
            ..UserIdentityOverrides::default()
        });
    }

    if identities.len() == contract_addresses.len() + 2 {
        let delegator_id = identities
            .first()
            .copied()
            .map(SolanaHostPubkey::new)
            .ok_or_else(|| {
                ProcessingError::Irrecoverable(anyhow!("missing v2 delegator identity"))
            })?;
        let delegate_id = identities
            .get(1)
            .copied()
            .map(SolanaHostPubkey::new)
            .ok_or_else(|| ProcessingError::Irrecoverable(anyhow!("missing v2 delegate identity")))?;
        return Ok(UserIdentityOverrides {
            delegator_id: Some(delegator_id),
            delegate_id: Some(delegate_id),
            contract_ids: Some(zip_contract_identities(
                contract_addresses,
                identities.into_iter().skip(2).collect(),
            )?),
            ..UserIdentityOverrides::default()
        });
    }

    Err(ProcessingError::Irrecoverable(anyhow!(
        "v2 decryption extra_data contains {} identities, but calldata contains {} contract addresses",
        identities.len(),
        contract_addresses.len()
    )))
}

fn zip_contract_identities(
    contract_addresses: &[Address],
    contract_ids: Vec<[u8; 32]>,
) -> Result<HashMap<Address, SolanaHostPubkey>, ProcessingError> {
    if contract_ids.len() != contract_addresses.len() {
        return Err(ProcessingError::Irrecoverable(anyhow!(
            "v2 decryption extra_data contains {} contract ids, but calldata contains {} contract addresses",
            contract_ids.len(),
            contract_addresses.len()
        )));
    }

    Ok(HashMap::from_iter(
        contract_addresses
            .iter()
            .copied()
            .zip(contract_ids.into_iter().map(SolanaHostPubkey::new)),
    ))
}

fn fixed_bytes_to_array(bytes: FixedBytes<32>) -> [u8; 32] {
    let mut out = [0_u8; 32];
    out.copy_from_slice(bytes.as_slice());
    out
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
            .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts)
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

    struct MockContextManager;

    impl ContextManager for MockContextManager {
        async fn validate_context(&self, _context_id: U256) -> Result<(), ProcessingError> {
            Ok(())
        }
    }
}
