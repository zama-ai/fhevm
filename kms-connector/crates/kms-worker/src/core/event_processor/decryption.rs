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
    sol_types::{Eip712Domain, SolValue},
};
use anyhow::anyhow;
use connector_utils::types::KmsGrpcRequest;
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use kms_grpc::kms::v1::{
    PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::borrow::Cow;
use tracing::info;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712Domain,

    /// The entity used to collect ciphertexts from S3 buckets.
    s3_service: S3Service,
}

impl DecryptionProcessor {
    pub fn new(config: &Config, s3_service: S3Service) -> Self {
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
        extra_data: Vec<u8>,
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

        let ciphertexts = self
            .prepare_ciphertexts(decryption_id, &key_id, sns_materials, &extra_data)
            .await?;

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
                extra_data,
            };

            verify_user_decryption_eip712(&user_decryption_request)?;
            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(RequestId { request_id: key_id }),
                domain: Some(domain_msg),
                extra_data,
            };
            Ok(public_decryption_request.into())
        }
    }

    async fn prepare_ciphertexts(
        &self,
        decryption_id: U256,
        key_id: &str,
        sns_materials: Vec<SnsCiphertextMaterial>,
        extra_data: &[u8],
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        let s3_url_matrix = decode_s3_url_matrix(extra_data)?;
        let sns_ciphertext_materials = self
            .s3_service
            .retrieve_sns_ciphertext_materials(sns_materials, s3_url_matrix)
            .await;

        if sns_ciphertext_materials.is_empty() {
            return Err(anyhow!("Failed to retrieve any ciphertext materials"));
        }

        // Extract and log FHE types for all ciphertexts
        let fhe_types: Vec<_> = sns_ciphertext_materials
            .iter()
            .map(|ct| ct.fhe_type)
            .collect();

        info!(
            "Processing {} with {} ciphertexts, key_id: {}, FHE types: {:?}",
            decryption_id,
            sns_ciphertext_materials.len(),
            key_id,
            fhe_types,
        );

        Ok(sns_ciphertext_materials)
    }
}

fn decode_s3_url_matrix(extra_data: &[u8]) -> anyhow::Result<Vec<Vec<String>>> {
    let (_version, urls): (u32, Vec<Vec<String>>) = SolValue::abi_decode_sequence(extra_data)?;
    Ok(urls)
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

    #[test]
    fn test_urls_decoding() {
        let url_matrix = vec![
            vec![
                String::from("http://localhost"),
                String::from("https://127.0.0.1:80/"),
            ],
            vec![String::from("http://localhost:81/test")],
        ];

        let extra_data = hex::decode(REMIX_GENERATED_EXTRA_DATA).unwrap();
        let decoded_url_matrix = decode_s3_url_matrix(&extra_data).unwrap();
        assert_eq!(url_matrix, decoded_url_matrix);
    }

    // Encoded bytes retrieved with the following function in Remix
    // ```
    // function encode() public pure returns (bytes memory) {
    //     string[] memory url_array1 = new string[](2);
    //     url_array1[0] = "http://localhost";
    //     url_array1[1] = "https://127.0.0.1:80/";
    //     string[] memory url_array2 = new string[](1);
    //     url_array2[0] = "http://localhost:81/test";
    //     string[][] memory url_matrix =  new string[][](2);
    //     url_matrix[0] = url_array1;
    //     url_matrix[1] = url_array2;
    //     uint32 version = 1;
    //     return abi.encode(version, url_matrix);
    // }
    // ```
    const REMIX_GENERATED_EXTRA_DATA: &str = "0x000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000010687474703a2f2f6c6f63616c686f737400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001568747470733a2f2f3132372e302e302e313a38302f0000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000018687474703a2f2f6c6f63616c686f73743a38312f746573740000000000000000";
}
