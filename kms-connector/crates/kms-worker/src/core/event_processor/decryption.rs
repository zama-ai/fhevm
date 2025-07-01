use crate::core::{
    config::Config,
    event_processor::{
        eip712::{alloy_to_protobuf_domain, verify_user_decryption_eip712},
        s3::S3Service,
    },
};
use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    sol_types::Eip712Domain,
};
use anyhow::anyhow;
use connector_utils::types::{
    KmsGrpcRequest,
    fhe::{extract_fhe_type_from_handle, fhe_type_to_string},
};
use fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial;
use kms_grpc::kms::v1::{
    CiphertextFormat, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::borrow::Cow;
use tracing::info;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<P: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712Domain,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service<P>,
}

impl<P> DecryptionProcessor<P>
where
    P: Provider,
{
    pub fn new(config: &Config, s3_service: S3Service<P>) -> Self {
        // Create EIP-712 domain using alloy primitives
        let domain = Eip712Domain {
            name: Some(Cow::Owned(config.decryption_contract.domain_name.clone())),
            version: Some(Cow::Owned(
                config.decryption_contract.domain_version.clone(),
            )),
            chain_id: Some(U256::from(config.chain_id)),
            verifying_contract: Some(config.decryption_contract.address),
            salt: None,
        };

        Self { s3_service, domain }
    }

    pub async fn prepare_decryption_request(
        &self,
        decryption_id: U256,
        sns_materials: Vec<SnsCiphertextMaterial>,
        user_decrypt_data: Option<UserDecryptionExtraData>,
    ) -> anyhow::Result<KmsGrpcRequest> {
        let decryption_type = if user_decrypt_data.is_some() {
            "UserDecryptionRequest"
        } else {
            "PublicDecryptionRequest"
        };

        info!("Processing {decryption_type}: {decryption_id}");

        // Extract keyId from the first SNS ciphertext material if available
        let key_id = sns_materials
            .first()
            .map(|m| hex::encode(m.keyId.to_be_bytes::<32>()))
            .ok_or_else(|| anyhow!(
                "No snsCtMaterials found for {decryption_type} {decryption_id}, cannot proceed without a valid key_id"
            ))?;
        info!(
            "Extracted key_id {key_id} from snsCtMaterials[0] for {decryption_type} {decryption_id}"
        );

        let ciphertexts = self
            .prepare_ciphertexts(decryption_id, &key_id, sns_materials)
            .await
            .map_err(|e| anyhow!("{decryption_type} {decryption_id}: {e}"))?;

        // Convert alloy domain to protobuf domain
        let domain_msg = alloy_to_protobuf_domain(&self.domain)?;

        info!(
            "Eip712Domain constructed: name={} version={} chain_id={} verifying_contract={} salt=None",
            domain_msg.name,
            domain_msg.version,
            U256::from_be_slice(&domain_msg.chain_id).to_string(),
            domain_msg.verifying_contract,
        );

        let request_id = Some(RequestId {
            request_id: hex::encode(decryption_id.to_be_bytes::<32>()),
        });

        if let Some(user_decrypt_data) = user_decrypt_data {
            let client_address = user_decrypt_data.user_address.to_checksum(None);
            info!("Proceeding with Client address: {client_address}");

            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(domain_msg),
                enc_key,
                typed_ciphertexts: ciphertexts,
            };

            verify_user_decryption_eip712(&user_decryption_request)?;
            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(domain_msg),
            };
            Ok(public_decryption_request.into())
        }
    }

    async fn prepare_ciphertexts(
        &self,
        decryption_id: U256,
        key_id: &str,
        sns_materials: Vec<SnsCiphertextMaterial>,
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        // Retrieve ciphertext materials from S3
        let sns_ciphertext_materials = self
            .s3_service
            .retrieve_sns_ciphertext_materials(sns_materials)
            .await;

        // If we couldn't retrieve any materials, fail the request
        if sns_ciphertext_materials.is_empty() {
            return Err(anyhow!("Failed to retrieve any ciphertext materials"));
        }

        // Extract and log FHE types for all ciphertexts
        let fhe_types: Vec<String> = sns_ciphertext_materials
            .iter()
            .map(|(handle, _)| {
                let fhe_type = extract_fhe_type_from_handle(handle);
                fhe_type_to_string(fhe_type).to_string()
            })
            .collect();

        info!(
            "Processing {} with {} ciphertexts, key_id: {}, FHE types: [{}]",
            decryption_id,
            sns_ciphertext_materials.len(),
            key_id,
            fhe_types.join(", ")
        );

        // Prepare typed ciphertexts for the user decryption request
        let typed_ciphertexts = sns_ciphertext_materials
            .into_iter()
            .map(|(handle, ciphertext)| {
                let fhe_type = extract_fhe_type_from_handle(&handle);
                info!(
                    "Handle: {}\nRetrieved S3 ciphertext of length: {}, FHE Type: {}",
                    hex::encode(&handle),
                    ciphertext.len(),
                    fhe_type
                );
                TypedCiphertext {
                    ciphertext,
                    external_handle: handle,
                    fhe_type,
                    ciphertext_format: CiphertextFormat::BigExpanded.into(),
                }
            })
            .collect();

        Ok(typed_ciphertexts)
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
