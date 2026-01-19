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
use connector_utils::types::{KmsGrpcRequest, fhe::extract_chain_id_from_handle};
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, SnsCiphertextMaterial,
};
use fhevm_host_bindings::acl::ACL::ACLInstance;
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::collections::HashMap;
use tracing::info;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider, HP: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The instances of the host chains `ACL` contracts used to check the decryption ACL.
    acl_contracts: HashMap<u64, ACLInstance<HP>>,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<GP>,
}

impl<GP, HP> DecryptionProcessor<GP, HP>
where
    GP: Provider,
    HP: Provider,
{
    pub fn new(
        config: &Config,
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
            decryption_contract,
            acl_contracts,
            s3_service,
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

    pub async fn check_ciphertexts_allowed_for_public_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
    ) -> Result<(), ProcessingError> {
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            let Some(acl_contract) = self.acl_contracts.get(&ct_chain_id) else {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "No ACL contract config found for chain id {ct_chain_id}"
                )));
            };

            if !acl_contract
                .isAllowedForDecryption(ct.ctHandle)
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?
            {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "{} is not allowed for decrypt!",
                    hex::encode(ct.ctHandle)
                )));
            }
        }

        Ok(())
    }

    pub async fn check_ciphertexts_allowed_for_user_decryption(
        &self,
        sns_ciphertexts: &[SnsCiphertextMaterial],
        user_address: Address,
    ) -> Result<(), ProcessingError> {
        for ct in sns_ciphertexts {
            let ct_chain_id = extract_chain_id_from_handle(ct.ctHandle.as_slice())
                .map_err(ProcessingError::Irrecoverable)?;
            let Some(acl_contract) = self.acl_contracts.get(&ct_chain_id) else {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "No ACL contract config found for chain id {ct_chain_id}"
                )));
            };

            if !acl_contract
                .isAllowed(ct.ctHandle, user_address)
                .call()
                .await
                .map_err(|e| ProcessingError::Recoverable(anyhow::Error::from(e)))?
            {
                return Err(ProcessingError::Irrecoverable(anyhow!(
                    "{} is not allowed for decrypt!",
                    hex::encode(ct.ctHandle)
                )));
            }
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
    use connector_utils::tests::rand::rand_sns_ct;
    use fhevm_host_bindings::acl::ACL;

    #[tokio::test]
    async fn check_decryption_not_already_done() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        let acl_contracts_mock = HashMap::from([(
            u64::default(),
            ACL::new(Address::default(), mock_provider.clone()),
        )]);

        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor =
            DecryptionProcessor::new(&config, mock_provider, acl_contracts_mock, s3_service);

        asserter.push_failure_msg("Transport Error");
        assert!(matches!(
            decryption_processor
                .check_decryption_not_already_done(U256::ZERO)
                .await
                .unwrap_err(),
            ProcessingError::Recoverable(_)
        ));

        asserter.push_success(&false.abi_encode());
        decryption_processor
            .check_decryption_not_already_done(U256::ZERO)
            .await
            .unwrap();

        asserter.push_success(&true.abi_encode());
        assert!(matches!(
            decryption_processor
                .check_decryption_not_already_done(U256::ZERO)
                .await
                .unwrap_err(),
            ProcessingError::Irrecoverable(_)
        ));
    }

    #[tokio::test]
    async fn check_ciphertexts_allowed_for_decryption() {
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
        let decryption_processor =
            DecryptionProcessor::new(&config, mock_provider, acl_contracts_mock, s3_service);

        asserter.push_failure_msg("Transport Error");
        assert!(matches!(
            decryption_processor
                .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts)
                .await
                .unwrap_err(),
            ProcessingError::Recoverable(_)
        ));

        asserter.push_success(&true.abi_encode());
        decryption_processor
            .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts)
            .await
            .unwrap();

        asserter.push_success(&false.abi_encode());
        assert!(matches!(
            decryption_processor
                .check_ciphertexts_allowed_for_public_decryption(&sns_ciphertexts)
                .await
                .unwrap_err(),
            ProcessingError::Irrecoverable(_)
        ));
    }
}
