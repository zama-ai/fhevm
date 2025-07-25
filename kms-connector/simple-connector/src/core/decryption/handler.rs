use alloy::{
    hex,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    sol_types::Eip712Domain,
};
use kms_grpc::kms::v1::{
    CiphertextFormat, PublicDecryptionRequest, RequestId, TypedCiphertext, UserDecryptionRequest,
};
use std::{borrow::Cow, sync::Arc};
use tonic::Request;
use tracing::{debug, error, info, warn};

use crate::{
    core::{
        config::Config,
        types::fhe_types::{
            abi_encode_plaintexts, extract_fhe_type_from_handle, fhe_type_to_string,
            log_and_extract_result,
        },
        utils::eip712::{alloy_to_protobuf_domain, verify_user_decryption_eip712},
    },
    error::Result,
    gw_adapters::decryption::DecryptionAdapter,
    kms_core_adapters::service::{KmsService, KmsServiceImpl},
};

/// Handle decryption requests and responses
#[derive(Clone)]
pub struct DecryptionHandler<P> {
    decryption: DecryptionAdapter<P>,
    kms_client: Arc<KmsServiceImpl>,
    #[allow(dead_code)]
    config: Config,
}

impl<P: Provider + Clone + 'static> DecryptionHandler<P> {
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

    /// Get the gateway config address from the config
    pub fn gateway_config_address(&self) -> Address {
        self.config.gateway_config_address
    }

    /// Get the provider from the decryption adapter
    pub fn provider(&self) -> Arc<P> {
        self.decryption.provider().clone()
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
        let request_type = if client_addr.is_some() {
            "User"
        } else {
            "Public"
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
            "[PROCESSING] {}DecryptionRequest-{} with {} ciphertexts, key_id: {}, FHE types: [{}]",
            request_type,
            request_id.to_string(),
            sns_ciphertext_materials.len(),
            key_id_hex,
            fhe_types.join(", ")
        );

        // Convert request_id to proper 32-byte format for KMS-Core
        let request_id_bytes = request_id.to_be_bytes::<32>();
        let request_id_obj = RequestId {
            request_id: hex::encode(request_id_bytes),
        };

        // Validate key_id format according to protobuf spec:
        // Must be 32-byte/256-bit lowercase hex string without 0x prefix (exactly 64 chars)
        if key_id_hex.len() != 64 {
            error!(
                "CRITICAL: key_id_hex has invalid length {} (expected 64 chars for 32-byte hex). Value: '{}'",
                key_id_hex.len(),
                key_id_hex
            );
        }

        if key_id_hex
            .chars()
            .any(|c| !c.is_ascii_hexdigit() || c.is_ascii_uppercase())
        {
            error!(
                "CRITICAL: key_id_hex contains invalid characters (must be lowercase hex). Value: '{}'",
                key_id_hex
            );
        }

        let key_id_obj = RequestId {
            request_id: key_id_hex.clone(),
        };

        // Create EIP-712 domain using alloy primitives
        let domain_name = self.kms_client.config().decryption_domain_name.clone();
        let domain_version = self.kms_client.config().decryption_domain_version.clone();
        let domain = Eip712Domain {
            name: Some(Cow::Owned(domain_name)),
            version: Some(Cow::Owned(domain_version)),
            chain_id: Some(U256::from(self.kms_client.config().chain_id)),
            verifying_contract: Some(self.kms_client.config().decryption_address),
            salt: None, // TODO: verify policy on this
        };

        // Convert alloy domain to protobuf domain
        let domain_msg = alloy_to_protobuf_domain(&domain)
            .map_err(|e| crate::error::Error::Config(format!("Failed to convert domain: {e}")))?;

        debug!(
            "Eip712Domain constructed: name={} version={} chain_id={} verifying_contract={} salt=None",
            domain_msg.name,
            domain_msg.version,
            alloy::primitives::U256::from_be_slice(&domain_msg.chain_id).to_string(),
            domain_msg.verifying_contract,
        );

        match client_addr {
            // Public decryption
            None => {
                // Prepare ciphertexts for the public decryption request
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

                let request = Request::new(PublicDecryptionRequest {
                    ciphertexts,
                    key_id: Some(key_id_obj.clone()),
                    domain: Some(domain_msg),
                    request_id: Some(request_id_obj.clone()),
                });

                let response = self.kms_client.request_public_decryption(request).await?;
                info!(
                    "[RECEIVED] PublicDecryptionResponse-{}",
                    request_id.to_string()
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

                    // Send response back to the Gateway
                    info!(
                        "[EMBEDDING] PublicDecryptionResponse-{} with {} plaintexts into the Gateway",
                        request_id,
                        payload.plaintexts.len()
                    );

                    let decryption_adapter = self.decryption.clone();

                    tokio::spawn(async move {
                        match decryption_adapter
                            .send_public_decryption_response(request_id, result, signature)
                            .await
                        {
                            Ok(_) => {
                                info!(
                                    "PublicDecryptionResponse-{} transaction submitted successfully",
                                    request_id
                                );
                            }
                            Err(e) => {
                                error!(
                                    "Failed to submit PublicDecryptionResponse-{}: {}",
                                    request_id, e
                                );
                            }
                        }
                    });
                } else {
                    error!(
                        "Received empty payload for PublicDecryptionRequest-{}",
                        request_id
                    );
                    return Err(crate::error::Error::Contract(
                        "Empty payload received from KMS Core".into(),
                    ));
                }

                Ok(())
            }
            // User decryption
            Some(client_addr) => {
                // Prepare typed ciphertexts for the user decryption request
                let typed_ciphertexts = sns_ciphertext_materials
                    .iter()
                    .map(|(handle, ciphertext)| {
                        let hexed_handle = hex::encode(handle);
                        let fhe_type = extract_fhe_type_from_handle(handle);
                        info!(
                            "UserDecryptionRequest handle: {}, retrieved S3 ciphertext of length: {}, FHE Type: {}",
                            hexed_handle,
                            ciphertext.len(),
                            fhe_type_to_string(fhe_type)
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

                debug!(
                    "Proceeding with Client address: {} (length: {} bytes)",
                    client_address_str,
                    client_addr.len()
                );

                let user_decryption_request = UserDecryptionRequest {
                    request_id: Some(request_id_obj.clone()),
                    client_address: client_address_str.clone(),
                    key_id: Some(key_id_obj.clone()), // Use key_id_obj wrapped in Some
                    domain: Some(domain_msg),
                    enc_key: public_key
                        .expect("Couldn't parse public_key aka enc_key")
                        .to_vec(),
                    typed_ciphertexts,
                };

                verify_user_decryption_eip712(&user_decryption_request)?;

                let request = Request::new(user_decryption_request.clone());

                // Only log detailed ciphertext info at debug level
                if tracing::enabled!(tracing::Level::DEBUG) {
                    for (i, ct) in request.get_ref().typed_ciphertexts.iter().enumerate() {
                        debug!(
                            "Ciphertext[{}]: fhe_type={}, handle_len={}, ct_len={}",
                            i,
                            fhe_type_to_string(ct.fhe_type),
                            ct.external_handle.len(),
                            ct.ciphertext.len()
                        );
                    }
                }

                let response = self.kms_client.request_user_decryption(request).await?;
                info!(
                    "[RECEIVED] UserDecryptionResponse-{}",
                    request_id.to_string()
                );
                let user_decryption_response = response.into_inner();

                // Get the external signature (non-optional in UserDecryptionResponsePayload)
                let signature = user_decryption_response.external_signature;

                if let Some(payload) = user_decryption_response.payload {
                    // Serialize all signcrypted ciphertexts
                    let serialized_response_payload =
                        bincode::serialize(&payload).map_err(|e| {
                            crate::error::Error::InvalidResponse(format!(
                                "Failed to serialize user decryption payload: {e}"
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

                    // Send response back to the Gateway
                    info!(
                        "[EMBEDDING] UserDecryptionResponse-{} into the Gateway",
                        request_id
                    );

                    let decryption_adapter = self.decryption.clone();

                    tokio::spawn(async move {
                        match decryption_adapter
                            .send_user_decryption_response(
                                request_id,
                                Bytes::from(serialized_response_payload),
                                signature,
                            )
                            .await
                        {
                            Ok(_) => {
                                info!(
                                    "UserDecryptionResponse-{} trx submitted successfully",
                                    request_id
                                );
                            }
                            Err(e) => {
                                error!(
                                    "Failed to submit UserDecryptionResponse-{} trx: {}",
                                    request_id, e
                                );
                            }
                        }
                    });
                } else {
                    error!(
                        "Received empty payload for UserDecryptionRequest-{}",
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
