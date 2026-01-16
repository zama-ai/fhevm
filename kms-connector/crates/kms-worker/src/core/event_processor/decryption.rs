use crate::core::{
    config::Config,
    event_processor::{ProcessingError, s3::S3Service},
};
use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::types::KmsGrpcRequest;
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, SnsCiphertextMaterial,
};
use fhevm_host_bindings::acl::ACL::{self, ACLInstance};
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use tracing::info;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<P: Provider, H: Provider = P> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<P>,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<P>,

    /// The instance of the ACL contract on the host chain used for authorization checks.
    acl_contract: ACLInstance<H>,
}

impl<P, H> DecryptionProcessor<P, H>
where
    P: Provider,
    H: Provider,
{
    pub fn new(
        config: &Config,
        gateway_provider: P,
        host_chain_provider: H,
        s3_service: S3Service<P>,
    ) -> Self {
        let domain = Eip712DomainMsg {
            name: config.decryption_contract.domain_name.clone(),
            version: config.decryption_contract.domain_version.clone(),
            chain_id: U256::from(config.chain_id).to_be_bytes_vec(),
            verifying_contract: config.decryption_contract.address.to_string(),
            salt: None,
        };
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, gateway_provider);
        let acl_contract =
            ACL::new(config.host_chain.acl_contract_address, host_chain_provider);

        Self {
            decryption_contract,
            s3_service,
            domain,
            acl_contract,
        }
    }

    pub async fn check_decryption_not_already_done(
        &self,
        decryption_id: U256,
    ) -> Result<(), ProcessingError> {
        let is_decryption_done = self
            .decryption_contract
            .isDecryptionDone(decryption_id)
            .call()
            .await
            .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?;

        if is_decryption_done {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "Decryption already done on the Gateway"
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
    ) -> anyhow::Result<KmsGrpcRequest> {
        // Extract keyId from the first SNS ciphertext material if available
        let key_id = sns_materials
            .first()
            .map(|m| hex::encode(m.keyId.to_be_bytes::<32>()))
            .ok_or_else(|| {
                anyhow!("No snsCtMaterials found, cannot proceed without a valid key_id")
            })?;
        info!("Extracted key_id {key_id} from snsCtMaterials[0]");

        let ciphertexts = self.prepare_ciphertexts(&key_id, sns_materials).await?;

        let request_id = Some(RequestId {
            request_id: hex::encode(decryption_id.to_be_bytes::<32>()),
        });
        let extra_data = extra_data.to_vec();

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
                context_id: None,
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
                context_id: None,
            };
            Ok(public_decryption_request.into())
        }
    }

    async fn prepare_ciphertexts(
        &self,
        key_id: &str,
        sns_materials: &[SnsCiphertextMaterial],
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        let sns_ciphertext_materials = self
            .s3_service
            .retrieve_sns_ciphertext_materials(sns_materials)
            .await?;

        if sns_ciphertext_materials.is_empty() {
            return Err(anyhow!("Failed to retrieve any ciphertext materials"));
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

    /// Checks that all handles are allowed for public decryption on the host chain ACL.
    ///
    /// For public decryption, each handle must have been explicitly allowed for decryption
    /// by calling `allowForDecryption` on the ACL contract.
    pub async fn check_acl_for_public_decryption(
        &self,
        sns_materials: &[SnsCiphertextMaterial],
    ) -> Result<(), ProcessingError> {
        for material in sns_materials {
            let handle = material.ctHandle;
            let is_allowed: bool = self
                .acl_contract
                .isAllowedForDecryption(handle)
                .call()
                .await
                .map_err(|e| {
                    ProcessingError::Recoverable(anyhow!(
                        "Failed to check ACL for public decryption: {}",
                        e
                    ))
                })?;

            if !is_allowed {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "Handle {} is not allowed for public decryption",
                    handle
                )));
            }
        }

        info!(
            "ACL check passed for public decryption of {} handles",
            sns_materials.len()
        );
        Ok(())
    }

    /// Checks that all handles are allowed for user decryption on the host chain ACL.
    ///
    /// For user decryption, the user address must be allowed to access each handle.
    /// This is checked via the `isAllowed` function on the ACL contract.
    pub async fn check_acl_for_user_decryption(
        &self,
        sns_materials: &[SnsCiphertextMaterial],
        user_address: Address,
    ) -> Result<(), ProcessingError> {
        for material in sns_materials {
            let handle = material.ctHandle;
            let is_allowed: bool = self
                .acl_contract
                .isAllowed(handle, user_address)
                .call()
                .await
                .map_err(|e| {
                    ProcessingError::Recoverable(anyhow!(
                        "Failed to check ACL for user decryption: {}",
                        e
                    ))
                })?
                .into();

            if !is_allowed {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "User {} is not allowed to decrypt handle {}",
                    user_address,
                    handle
                )));
            }
        }

        info!(
            "ACL check passed for user decryption of {} handles for user {}",
            sns_materials.len(),
            user_address
        );
        Ok(())
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
        primitives::FixedBytes,
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
        transports::http::reqwest,
    };
    use rstest::rstest;

    fn create_test_processor(
        gateway_asserter: Asserter,
        host_chain_asserter: Asserter,
    ) -> DecryptionProcessor<impl Provider + Clone, impl Provider + Clone> {
        let gateway_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(gateway_asserter);
        let host_chain_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(host_chain_asserter);

        let config = Config::default();
        let s3_service = S3Service::new(&config, gateway_provider.clone(), reqwest::Client::new());
        DecryptionProcessor::new(&config, gateway_provider, host_chain_provider, s3_service)
    }

    fn create_test_sns_materials() -> Vec<SnsCiphertextMaterial> {
        vec![SnsCiphertextMaterial {
            ctHandle: FixedBytes::ZERO,
            keyId: U256::ZERO,
            snsCiphertextDigest: FixedBytes::ZERO,
            coprocessorTxSenderAddresses: vec![],
        }]
    }

    #[tokio::test]
    async fn check_decryption_not_already_done() {
        let gateway_asserter = Asserter::new();
        let host_chain_asserter = Asserter::new();
        let decryption_processor =
            create_test_processor(gateway_asserter.clone(), host_chain_asserter);

        gateway_asserter.push_failure_msg("Transport Error");
        assert!(matches!(
            decryption_processor
                .check_decryption_not_already_done(U256::ZERO)
                .await
                .unwrap_err(),
            ProcessingError::Recoverable(_)
        ));

        gateway_asserter.push_success(&false.abi_encode());
        decryption_processor
            .check_decryption_not_already_done(U256::ZERO)
            .await
            .unwrap();

        gateway_asserter.push_success(&true.abi_encode());
        assert!(matches!(
            decryption_processor
                .check_decryption_not_already_done(U256::ZERO)
                .await
                .unwrap_err(),
            ProcessingError::Irrecoverable(_)
        ));
    }

    /// Tests for ACL checks - parametrized by decryption type and expected result
    #[derive(Debug, Clone, Copy)]
    enum AclCheckType {
        Public,
        User,
    }

    #[derive(Debug, Clone, Copy)]
    enum MockResponse {
        Allowed,
        NotAllowed,
        TransportError,
    }

    #[derive(Debug, Clone, Copy)]
    enum ExpectedResult {
        Success,
        Irrecoverable,
        Recoverable,
    }

    #[rstest]
    #[case::public_allowed(AclCheckType::Public, MockResponse::Allowed, ExpectedResult::Success)]
    #[case::public_not_allowed(AclCheckType::Public, MockResponse::NotAllowed, ExpectedResult::Irrecoverable)]
    #[case::public_transport_error(AclCheckType::Public, MockResponse::TransportError, ExpectedResult::Recoverable)]
    #[case::user_allowed(AclCheckType::User, MockResponse::Allowed, ExpectedResult::Success)]
    #[case::user_not_allowed(AclCheckType::User, MockResponse::NotAllowed, ExpectedResult::Irrecoverable)]
    #[case::user_transport_error(AclCheckType::User, MockResponse::TransportError, ExpectedResult::Recoverable)]
    #[tokio::test]
    async fn check_acl_authorization(
        #[case] check_type: AclCheckType,
        #[case] mock_response: MockResponse,
        #[case] expected: ExpectedResult,
    ) {
        let gateway_asserter = Asserter::new();
        let host_chain_asserter = Asserter::new();
        let decryption_processor =
            create_test_processor(gateway_asserter, host_chain_asserter.clone());
        let sns_materials = create_test_sns_materials();

        // Setup mock response
        match mock_response {
            MockResponse::Allowed => host_chain_asserter.push_success(&true.abi_encode()),
            MockResponse::NotAllowed => host_chain_asserter.push_success(&false.abi_encode()),
            MockResponse::TransportError => host_chain_asserter.push_failure_msg("Transport Error"),
        }

        // Execute the appropriate check
        let result = match check_type {
            AclCheckType::Public => {
                decryption_processor
                    .check_acl_for_public_decryption(&sns_materials)
                    .await
            }
            AclCheckType::User => {
                decryption_processor
                    .check_acl_for_user_decryption(&sns_materials, Address::ZERO)
                    .await
            }
        };

        // Verify expected outcome
        match expected {
            ExpectedResult::Success => result.unwrap(),
            ExpectedResult::Irrecoverable => {
                assert!(matches!(result.unwrap_err(), ProcessingError::Irrecoverable(_)))
            }
            ExpectedResult::Recoverable => {
                assert!(matches!(result.unwrap_err(), ProcessingError::Recoverable(_)))
            }
        }
    }
}
