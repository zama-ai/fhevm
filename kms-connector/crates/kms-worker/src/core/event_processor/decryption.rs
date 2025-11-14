use crate::core::{
    config::Config,
    event_processor::{ProcessingError, eip712::alloy_to_protobuf_domain, s3::S3Service},
};
use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    sol_types::Eip712Domain,
};
use anyhow::anyhow;
use connector_utils::types::KmsGrpcRequest;
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, SnsCiphertextMaterial,
};
use kms_grpc::kms::v1::{
    PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::borrow::Cow;
use tracing::info;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<P: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712Domain,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<P>,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<P>,
}

impl<P> DecryptionProcessor<P>
where
    P: Provider,
{
    pub fn new(config: &Config, provider: P, s3_service: S3Service<P>) -> Self {
        let domain = Eip712Domain {
            name: Some(Cow::Owned(config.decryption_contract.domain_name.clone())),
            version: Some(Cow::Owned(
                config.decryption_contract.domain_version.clone(),
            )),
            chain_id: Some(U256::from(config.chain_id)),
            verifying_contract: Some(config.decryption_contract.address),
            salt: None,
        };
        let decryption_contract = Decryption::new(config.decryption_contract.address, provider);

        Self {
            decryption_contract,
            s3_service,
            domain,
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

        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;
        info!("Eip712Domain constructed: {domain_msg:?}",);

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
                domain: Some(domain_msg),
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
                domain: Some(domain_msg),
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

    #[tokio::test]
    async fn check_decryption_not_already_done() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());

        let config = Config::default();
        let s3_service = S3Service::new(&config, mock_provider.clone(), reqwest::Client::new());
        let decryption_processor = DecryptionProcessor::new(&config, mock_provider, s3_service);

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
}
