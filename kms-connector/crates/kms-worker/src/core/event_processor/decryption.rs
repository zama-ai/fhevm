use crate::core::{
    config::Config,
    event_processor::{ProcessingError, context::ContextManager, s3::S3Service},
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
    KmsGrpcRequest, extra_data::parse_extra_data, handle::extract_chain_id_from_handle,
    u256_to_request_id,
};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, HandleEntry, SnsCiphertextMaterial,
    UserDecryptionRequest_1 as UserDecryptionRequestV2, delegatedUserDecryptionRequestCall,
    userDecryptionRequest_1Call as userDecryptionRequestCall,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use futures::future::join_all;
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use sqlx::types::chrono::Utc;
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
                        ProcessingError::Irrecoverable(anyhow!(
                            "Was not able to parse calldata for both userDecryptionRequestCall {e2} \
                            and delegatedUserDecryptionRequestCall ({e})!"
                        ))
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
                .map_err(ProcessingError::Irrecoverable)?;
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

    /// RFC016 unified user decryption check — verifies the full ACL authorization for a
    /// `UserDecryptionRequestV2` payload.
    ///
    /// 1. (deferred — see TODO below) signature invalidation check
    /// 2. validity window (`startTimestamp <= now <= startTimestamp + durationSeconds`)
    /// 3. `userAddress ∉ allowedContracts` when `allowedContracts` is non-empty
    /// 4. per-handle ownership (direct `isAllowed` if `ownerAddress == userAddress`, else
    ///    `isHandleDelegatedForUserDecryption`)
    /// 5. per-handle contract allowance (any `isAllowed(handle, c)` for `c ∈ allowedContracts`,
    ///    no-op in permissive mode)
    #[tracing::instrument(skip_all)]
    pub async fn check_user_decryption_request_v2(
        &self,
        request: &UserDecryptionRequestV2,
    ) -> Result<(), ProcessingError> {
        info!(
            "Starting RFC016 check for {} handles...",
            request.handles.len()
        );

        let payload = &request.payload;

        // TODO(RFC016 signature invalidation): fetch
        // `ACL.decryptionSignatureInvalidatedBefore(userAddress)` once the `fhevm-host-bindings`
        // `ACL` binding exposes it, and reject when `validity.startTimestamp < invalidationTs`.

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

        for entry in &request.handles {
            let chain_id = extract_chain_id_from_handle(entry.handle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            let acl_contract = self.acl_contracts.get(&chain_id).ok_or_else(|| {
                ProcessingError::Recoverable(anyhow!(
                    "No ACL contract config found for chain id {chain_id}"
                ))
            })?;

            tokio::try_join!(
                self.inner_ownership_check_for_user_decryption_v2(
                    acl_contract,
                    entry,
                    payload.userAddress,
                ),
                self.inner_allowed_contracts_check_for_user_decryption_v2(
                    acl_contract,
                    entry.handle,
                    &payload.allowedContracts,
                ),
            )?;
        }

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
        if results.into_iter().any(|r| matches!(r, Ok(true))) {
            Ok(())
        } else {
            Err(ProcessingError::Recoverable(anyhow!(
                "No contract in allowedContracts is allowed to decrypt handle {}",
                hex::encode(handle)
            )))
        }
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
    use connector_utils::tests::rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256};
    use fhevm_gateway_bindings::decryption::{
        Decryption::CtHandleContractPair,
        IDecryption::{RequestValiditySeconds, UserDecryptionRequestPayload},
    };
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;

    enum ExpectedOutcome {
        Ok,
        Recoverable,
        #[allow(unused)]
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

    fn make_v2_request(
        sns_ct: &SnsCiphertextMaterial,
        owner_address: Address,
        user_address: Address,
        allowed_contracts: Vec<Address>,
        start_offset_secs: i64,
        duration_secs: u64,
    ) -> UserDecryptionRequestV2 {
        let start = (Utc::now().timestamp() + start_offset_secs) as u64;
        UserDecryptionRequestV2 {
            decryptionId: rand_u256(),
            snsCtMaterials: vec![sns_ct.clone()],
            handles: vec![HandleEntry {
                handle: sns_ct.ctHandle,
                contractAddress: rand_address(),
                ownerAddress: owner_address,
            }],
            payload: UserDecryptionRequestPayload {
                userAddress: user_address,
                publicKey: Bytes::from(rand_public_key()),
                allowedContracts: allowed_contracts,
                requestValidity: RequestValiditySeconds {
                    startTimestamp: U256::from(start),
                    durationSeconds: U256::from(duration_secs),
                },
                extraData: Bytes::default(),
                signature: Bytes::default(),
            },
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
        let user_address = rand_address();
        let processor = setup_test_processor(Asserter::new(), &sns_ct);
        let request = make_v2_request(
            &sns_ct,
            user_address,
            user_address,
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
        let user_address = rand_address();
        let processor = setup_test_processor(Asserter::new(), &sns_ct);
        let request = make_v2_request(
            &sns_ct,
            user_address,
            user_address,
            vec![user_address],
            -3600,
            86400,
        );

        let result = processor.check_user_decryption_request_v2(&request).await;
        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
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
        let user_address = rand_address();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        let (owner_address, acl_response) = match mock {
            OwnershipMock::DirectPath(r) => (user_address, r),
            OwnershipMock::DelegatedPath(r) => (rand_address(), r),
        };
        match acl_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(&sns_ct, owner_address, user_address, vec![], -3600, 86400);
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
    // serves responses in FIFO order, but poll ordering between the two futures is not
    // guaranteed. The test design is robust to either ordering: ownership and contracts
    // failures are both Recoverable, so swapping which future receives which response
    // doesn't change the expected outcome.
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
        let user_address = rand_address();
        let processor = setup_test_processor(asserter.clone(), &sns_ct);

        asserter.push_success(&true.abi_encode()); // ownership always passes
        match contract_response {
            Some(v) => asserter.push_success(&v.abi_encode()),
            None => asserter.push_failure_msg("transport error"),
        }

        let request = make_v2_request(
            &sns_ct,
            user_address,
            user_address,
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
}
