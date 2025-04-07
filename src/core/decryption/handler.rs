use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    sol_types::Eip712Domain,
};
use kms_grpc::kms::v1::{
    CiphertextFormat, DecryptionRequest, ReencryptionRequest, RequestId, TypedCiphertext,
};
use std::{borrow::Cow, sync::Arc};
use tonic::Request;
use tracing::{debug, error, info, warn};

use crate::{
    core::{
        config::Config,
        types::fhe_types::{
            abi_encode_plaintexts, extract_fhe_type_from_handle, fhe_type_to_string,
            format_request_id, log_and_extract_result,
        },
        utils::eip712::{alloy_to_protobuf_domain, verify_reencryption_eip712},
    },
    error::Result,
    gwl2_adapters::decryption::DecryptionAdapter,
    kms_core_adapters::service::{KmsService, KmsServiceImpl},
};

/// Handle decryption requests and responses
#[derive(Clone)]
pub struct DecryptionHandler<P: Provider + Clone> {
    decryption: DecryptionAdapter<P>,
    kms_client: Arc<KmsServiceImpl>,
    #[allow(dead_code)]
    config: Config,
}

impl<P: Provider + Clone + std::fmt::Debug + 'static> DecryptionHandler<P> {
    /// Create a new decryption handler
    pub fn new(
        decryption: DecryptionAdapter<P>,
        kms_client: Arc<KmsServiceImpl>,
        config: Config,
    ) -> Self {
        Self {
            decryption,
            kms_client,
            config,
        }
    }

    /// Handle a decryption request (both public and user)
    pub async fn handle_decryption_request_response(
        &self,
        request_id: U256,
        key_id_hex: String,
        sns_ciphertext_materials: Vec<(Vec<u8>, Vec<u8>)>, // (handle, ciphertext) pairs
        client_addr: Option<Address>,
        public_key: Option<Bytes>,
    ) -> Result<()> {
        let request_id_hex = format_request_id(request_id);

        let request_type = if client_addr.is_some() {
            "user"
        } else {
            "public"
        };

        // Extract and log FHE types for all ciphertexts
        let fhe_types: Vec<String> = sns_ciphertext_materials
            .iter()
            .map(|(handle, _)| {
                let fhe_type = extract_fhe_type_from_handle(handle);
                fhe_type_to_string(fhe_type).to_string()
            })
            .collect();

        info!(
            "Processing {} decryption request {} with {} ciphertexts, key_id: {}, FHE types: [{}]",
            request_type,
            request_id_hex,
            sns_ciphertext_materials.len(),
            key_id_hex,
            fhe_types.join(", ")
        );

        let request_id_obj = RequestId {
            request_id: request_id_hex.clone(),
        };

        let key_id_obj = RequestId {
            request_id: key_id_hex.clone(),
        };

        // Create EIP-712 domain using alloy primitives
        let domain_name = self
            .kms_client
            .config()
            .decryption_manager_domain_name
            .clone();
        let domain_version = self
            .kms_client
            .config()
            .decryption_manager_domain_version
            .clone();
        let domain = Eip712Domain {
            name: Some(Cow::Owned(domain_name)),
            version: Some(Cow::Owned(domain_version)),
            chain_id: Some(U256::from(self.kms_client.config().chain_id)),
            verifying_contract: Some(
                self.kms_client
                    .config()
                    .decryption_manager_address
                    .parse()
                    .map_err(|e| {
                        crate::error::Error::Config(format!(
                            "Invalid decryption manager address: {}",
                            e
                        ))
                    })?,
            ),
            salt: None, // TODO: verify policy on this
        };

        // Convert alloy domain to protobuf domain
        let domain_msg = alloy_to_protobuf_domain(&domain)
            .map_err(|e| crate::error::Error::Config(format!("Failed to convert domain: {}", e)))?;

        info!(
            "Eip712Domain constructed: name={} version={} chain_id={} verifying_contract={} salt=None",
            domain_msg.name,
            domain_msg.version,
            alloy_primitives::U256::from_be_slice(&domain_msg.chain_id).to_string(),
            domain_msg.verifying_contract,
        );

        match client_addr {
            // Public decryption
            None => {
                // Prepare ciphertexts for the decryption request
                let ciphertexts = sns_ciphertext_materials
                    .iter()
                    .map(|(handle, ciphertext)| {
                        let fhe_type = extract_fhe_type_from_handle(handle);
                        TypedCiphertext {
                            ciphertext: ciphertext.clone(),
                            fhe_type,
                            external_handle: handle.clone(),
                            ciphertext_format: CiphertextFormat::BigExpanded.into(),
                        }
                    })
                    .collect();

                let request = Request::new(DecryptionRequest {
                    ciphertexts,
                    key_id: Some(key_id_obj.clone()),
                    domain: Some(domain_msg),
                    request_id: Some(request_id_obj.clone()),
                });

                let response = self.kms_client.request_decryption(request).await?;
                info!(
                    "[IN] 📡 PublicDecryptionResponse({}) received",
                    request_id_hex
                );
                let decryption_response = response.into_inner();

                // Check if we have a valid payload
                if let Some(payload) = decryption_response.payload {
                    // Log all plaintexts for debugging
                    for pt in &payload.plaintexts {
                        log_and_extract_result(&pt.bytes, pt.fhe_type, request_id, None);
                    }

                    // Encode all plaintexts using ABI encoding
                    let result = abi_encode_plaintexts(&payload.plaintexts);

                    // Get the external signature
                    let signature = payload.external_signature.ok_or_else(|| {
                        crate::error::Error::Contract(
                            "KMS Core did not provide required EIP-712 signature".into(),
                        )
                    })?;

                    // Send response back to L2
                    info!(
                        "Sending public decryption response for request {} with {} plaintexts",
                        request_id,
                        payload.plaintexts.len()
                    );
                    self.decryption
                        .send_public_decryption_response(request_id, result, signature)
                        .await?;
                } else {
                    error!(
                        "Received empty payload for decryption request {}",
                        request_id
                    );
                    return Err(crate::error::Error::Contract(
                        "Empty payload received from KMS Core".into(),
                    ));
                }

                Ok(())
            }
            // User decryption aka reencryption
            Some(client_addr) => {
                // Prepare typed ciphertexts for the reencryption request
                let typed_ciphertexts = sns_ciphertext_materials
                    .iter()
                    .map(|(handle, ciphertext)| {
                        let hexed_handle = hex::encode(handle);
                        let fhe_type = extract_fhe_type_from_handle(handle);
                        info!(
                            "Handle: {}\nRetrieved S3 ciphertext of length: {}, FHE Type: {}",
                            hexed_handle,
                            ciphertext.len(),
                            fhe_type
                        );
                        TypedCiphertext {
                            ciphertext: ciphertext.clone(),
                            external_handle: handle.clone(),
                            fhe_type,
                            ciphertext_format: CiphertextFormat::BigExpanded.into(),
                        }
                    })
                    .collect();

                // Convert user_addr to an Ethereum address
                // The public key might not be a valid Ethereum address (it's 40 bytes instead of 20)
                // We need to handle this gracefully to maintain the non-failable design
                let client_address_str = if client_addr.len() == 20 {
                    // If it's a valid 20-byte Ethereum address, convert it properly
                    client_addr.to_checksum(None)
                } else {
                    // If it's not 20 bytes, we can't create a valid Ethereum address
                    // Thus, we'll use its hex representation instead
                    warn!(
                        "Client address has invalid length for Ethereum address: {} bytes (expected 20), using hex representation",
                        client_addr.len()
                    );
                    format!("0x{}", alloy::hex::encode(client_addr))
                };

                info!(
                    "Proceeding with Client address: {} (length: {} bytes)",
                    client_address_str,
                    client_addr.len()
                );

                let reencryption_request = ReencryptionRequest {
                    request_id: Some(request_id_obj.clone()),
                    client_address: client_address_str.clone(),
                    key_id: Some(key_id_obj.clone()),
                    domain: Some(domain_msg),
                    enc_key: public_key
                        .expect("Couldn't parse public_key aka enc_key")
                        .to_vec(),
                    typed_ciphertexts,
                };

                verify_reencryption_eip712(&reencryption_request)?;

                let request = Request::new(reencryption_request.clone());

                // Log a more concise version of the request with hex representations
                info!(
                    "ReencryptionRequest constructed with: request_id={}, client_address={}, key_id={}, typed_ciphertexts.len={}, domain.chain_id={}",
                    request.get_ref().request_id.as_ref().map(|id| id.request_id.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    request.get_ref().client_address,
                    request.get_ref().key_id.as_ref().map(|id| id.request_id.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    request.get_ref().typed_ciphertexts.len(),
                    request.get_ref().domain.as_ref().map(|x| {
                        let chain_id_bytes = &x.chain_id;
                        if !chain_id_bytes.is_empty() {
                            // Parse bytes back to U256 and display as decimal
                            let chain_id_u256 = alloy_primitives::U256::from_be_slice(chain_id_bytes);
                            chain_id_u256.to_string()
                        } else {
                            "unknown".to_string()
                        }
                    }).unwrap_or_else(|| "unknown".to_string())
                );

                // TODO: revert to DEBUG
                // Only log detailed ciphertext info at debug level
                if tracing::enabled!(tracing::Level::DEBUG) {
                    for (i, ct) in request.get_ref().typed_ciphertexts.iter().enumerate() {
                        debug!(
                            "Ciphertext[{}]: fhe_type={}, handle_len={}, ct_len={}",
                            i,
                            ct.fhe_type,
                            ct.external_handle.len(),
                            ct.ciphertext.len()
                        );
                    }
                }

                let response = self.kms_client.request_reencryption(request).await?;
                info!("[IN] 📡 ReencryptionResponse({}) received", request_id_hex);
                let reencryption_response = response.into_inner();

                // Get the external signature (non-optional in ReencryptionResponsePayload)
                let signature = reencryption_response.external_signature;

                if let Some(payload) = reencryption_response.payload {
                    // Serialize all signcrypted ciphertexts
                    let reencrypted_share_buf = bincode::serialize(&payload).map_err(|e| {
                        crate::error::Error::InvalidResponse(format!(
                            "Failed to serialize user decryption payload: {}",
                            e
                        ))
                    })?;

                    // Log each ciphertext for debugging
                    for ct in &payload.signcrypted_ciphertexts {
                        log_and_extract_result(
                            &ct.signcrypted_ciphertext,
                            ct.fhe_type,
                            request_id,
                            Some(client_addr),
                        );
                    }

                    // Send response back to L2
                    info!("Sending userDecryptionResponse for request {}", request_id);
                    self.decryption
                        .send_user_decryption_response(
                            request_id,
                            Bytes::from(reencrypted_share_buf),
                            signature,
                        )
                        .await?;
                } else {
                    error!(
                        "Received empty payload for reencryption request {}",
                        request_id
                    );
                    return Err(crate::error::Error::Contract(
                        "Empty payload received from KMS Core".into(),
                    ));
                }

                Ok(())
            }
        }
    }
}
