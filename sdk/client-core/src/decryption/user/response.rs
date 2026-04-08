use super::deserializer::UserDecryptionDeserializer;
use super::types::ResponseConfig;
use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{ClientCoreError, Result};
use alloy::primitives::Address;
use alloy::signers::Signature;
use fhevm_gateway_bindings::decryption::Decryption::CtHandleContractPair;
use kms_grpc::kms::v1::{Eip712DomainMsg, TypedPlaintext};
use kms_grpc::rpc_types::protobuf_to_alloy_domain;
use kms_lib::client::js_api::{new_client, new_server_id_addr};
use kms_lib::client::user_decryption_wasm::{CiphertextHandle, ParsedUserDecryptionRequest};
use kms_lib::consts::SAFE_SER_SIZE_LIMIT;
use kms_lib::cryptography::encryption::{PrivateEncKey, UnifiedPrivateEncKey, UnifiedPublicEncKey};
use tracing::{debug, info};

/// Builder for processing user decryption responses.
///
/// Provides a fluent API for configuring and executing user decryption
/// operations with proper validation at processing time.
pub struct UserDecryptionResponseBuilder {
    config: ResponseConfig,
}

impl Default for UserDecryptionResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserDecryptionResponseBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            config: ResponseConfig::default(),
        }
    }

    /// Set KMS signers (required)
    pub fn with_kms_signers(mut self, signers: Vec<String>) -> Self {
        self.config.kms_signers = Some(signers);
        self
    }

    /// Add a single KMS signer (convenience method)
    pub fn with_kms_signer(mut self, signer: String) -> Self {
        self.config
            .kms_signers
            .get_or_insert_with(Vec::new)
            .push(signer);
        self
    }

    /// Set user address (required)
    pub fn with_user_address(mut self, address: &str) -> Self {
        self.config.user_address = Some(address.to_string());
        self
    }

    /// Set gateway chain ID (required)
    pub fn with_gateway_chain_id(mut self, chain_id: u64) -> Self {
        self.config.gateway_chain_id = Some(chain_id);
        self
    }

    /// Set verifying contract address (required)
    pub fn with_verifying_contract_address(mut self, address: &str) -> Self {
        self.config.verifying_contract_address = Some(address.to_string());
        self
    }

    /// Set signature (required)
    pub fn with_signature(mut self, signature: &str) -> Self {
        self.config.signature = Some(signature.to_string());
        self
    }

    /// Set public key (required)
    pub fn with_public_key(mut self, key: &str) -> Self {
        self.config.public_key = Some(key.to_string());
        self
    }

    /// Set private key (required)
    pub fn with_private_key(mut self, key: &str) -> Self {
        self.config.private_key = Some(key.to_string());
        self
    }

    /// Set handle-contract pairs (required)
    pub fn with_handle_contract_pairs(mut self, pairs: Vec<CtHandleContractPair>) -> Self {
        self.config.handle_contract_pairs = Some(pairs);
        self
    }

    /// Set JSON response (required)
    pub fn with_json_response(mut self, response: &str) -> Self {
        self.config.json_response = Some(response.to_string());
        self
    }

    /// Enable or disable signature verification (optional, default: false)
    pub fn with_verification(mut self, verify: bool) -> Self {
        self.config.verify_signatures = verify;
        self
    }

    /// Set domain (optional, default: "Decryption")
    pub fn with_domain(mut self, domain: &str) -> Self {
        self.config.domain = Some(domain.to_string());
        self
    }

    /// Process the user decryption response
    ///
    /// # Returns
    /// Vector of decrypted plaintexts with their types
    ///
    /// # Flow
    /// 1. **Validation**: Ensures all required fields are set
    /// 2. **KMS Client**: Creates client with signers
    /// 3. **EIP-712 Domain**: Builds domain for verification
    /// 4. **Payload Creation**: Prepares decryption request
    /// 5. **Response Processing**: Decrypts via KMS
    /// 6. **Verification**: Optional signature verification
    pub fn process(self) -> Result<Vec<TypedPlaintext>> {
        // Validate configuration
        validate_config(&self.config)?;

        // Extract and process
        let processor = ResponseProcessor::new(self.config);
        processor.process()
    }
}

/// Handles the actual processing logic
struct ResponseProcessor {
    config: ResponseConfig,
}

impl ResponseProcessor {
    fn new(config: ResponseConfig) -> Self {
        Self { config }
    }

    fn process(self) -> Result<Vec<TypedPlaintext>> {
        // Extract fields (safe after validation)
        let kms_signers = self.config.kms_signers.unwrap();
        let user_address = self.config.user_address.unwrap();
        let gateway_chain_id = self.config.gateway_chain_id.unwrap();
        let verifying_contract_address = self.config.verifying_contract_address.unwrap();
        let signature = self.config.signature.unwrap();
        let public_key = self.config.public_key.unwrap();
        let private_key = self.config.private_key.unwrap();
        let handle_contract_pairs = self.config.handle_contract_pairs.unwrap();
        let json_response = self.config.json_response.unwrap();
        let domain = self.config.domain.unwrap();

        info!("Processing user decryption response");
        info!("   KMS signers: {} signers", kms_signers.len());
        info!("   User address: {}", user_address);
        info!("   Gateway chain ID: {}", gateway_chain_id);
        info!("   Handles to decrypt: {}", handle_contract_pairs.len());

        // Verify addresses
        let user_address_verified = validate_address_from_str(&user_address)?;
        let verifying_contract_verified = validate_address_from_str(&verifying_contract_address)?;

        // Parse keys
        let public_key_bytes = parse_hex_string(&public_key, "public_key")?;
        let private_key_bytes = parse_hex_string(&private_key, "private_key")?;

        // Create KMS client
        let client = create_kms_client(&kms_signers, &user_address)?;

        // Build EIP-712 domain
        let eip712_domain =
            build_eip712_domain(&domain, gateway_chain_id, &verifying_contract_address);

        // Prepare payload
        let payload = create_decryption_payload(
            &signature,
            user_address_verified,
            &public_key_bytes,
            &handle_contract_pairs,
            verifying_contract_verified,
        )?;

        // Convert response to KMS format
        let responses = UserDecryptionDeserializer::json_to_responses(&json_response)?;

        // Convert keys for ML-KEM
        let ml_kem_pub_key = tfhe::safe_serialization::safe_deserialize::<UnifiedPublicEncKey>(
            std::io::Cursor::new(public_key_bytes),
            SAFE_SER_SIZE_LIMIT,
        )
        .map_err(|e| ClientCoreError::DecryptionError(format!("Invalid public key: {e:?}")))?;

        let ml_kem_priv_key =
            bc2wrap::deserialize_safe::<PrivateEncKey<ml_kem::MlKem512>>(&private_key_bytes)
                .map(UnifiedPrivateEncKey::MlKem512)
                .map_err(|e| {
                    ClientCoreError::DecryptionError(format!("Invalid private key: {e:?}"))
                })?;

        let eip712_domain = protobuf_to_alloy_domain(&eip712_domain).map_err(|e| {
            ClientCoreError::DecryptionError(format!("Invalid EIP-712 domain: {e:?}"))
        })?;

        let decryption_result = if self.config.verify_signatures {
            client
                .process_user_decryption_resp(
                    &payload,
                    &eip712_domain,
                    &responses,
                    &ml_kem_pub_key,
                    &ml_kem_priv_key,
                )
                .map_err(|e| {
                    ClientCoreError::DecryptionError(format!("KMS decryption failed: {e:?}"))
                })?
        } else {
            client
                .insecure_process_user_decryption_resp(&responses, &ml_kem_priv_key)
                .map_err(|e| {
                    ClientCoreError::DecryptionError(format!("KMS decryption failed: {e:?}"))
                })?
        };

        info!(
            "User decryption processed successfully: {} results",
            decryption_result.len()
        );

        // Log individual results at debug level
        for (i, plaintext) in decryption_result.iter().enumerate() {
            debug!(
                "   Result {}: type={}, {} bytes",
                i,
                plaintext.fhe_type,
                plaintext.bytes.len()
            );
        }

        Ok(decryption_result)
    }
}

// Helper functions

fn create_kms_client(
    kms_signers: &[String],
    user_address: &str,
) -> Result<kms_lib::client::client_wasm::Client> {
    // Convert string addresses to ServerIdAddr objects
    let server_id_addrs = kms_signers
        .iter()
        .enumerate()
        .map(|(index, addr)| {
            new_server_id_addr((index + 1) as u32, addr.clone()).map_err(|e| {
                ClientCoreError::DecryptionError(format!(
                    "Invalid KMS signer address {addr}: {e:?}"
                ))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    new_client(server_id_addrs, user_address, "default").map_err(|e| {
        ClientCoreError::DecryptionError(format!("Failed to create KMS client: {e:?}"))
    })
}

fn build_eip712_domain(
    domain: &str,
    gateway_chain_id: u64,
    verifying_contract: &str,
) -> Eip712DomainMsg {
    // Reuse the shared chain_id_to_bytes utility (32 bytes, big-endian)
    let chain_id_buffer = crate::utils::chain_id_to_bytes(gateway_chain_id);

    Eip712DomainMsg {
        name: domain.to_string(),
        version: "1".to_string(),
        chain_id: chain_id_buffer.to_vec(),
        verifying_contract: verifying_contract.to_string(),
        salt: None,
    }
}

fn create_decryption_payload(
    signature: &str,
    user_address: Address,
    public_key_bytes: &[u8],
    handle_contract_pairs: &[CtHandleContractPair],
    verifying_contract: Address,
) -> Result<ParsedUserDecryptionRequest> {
    // Parse signature
    let sig_bytes = parse_hex_string(signature, "signature")?;
    let sig = Signature::from_raw(&sig_bytes)
        .map_err(|e| ClientCoreError::DecryptionError(format!("Invalid signature format: {e}")))?;

    // Convert handles
    let ct_handles: Vec<CiphertextHandle> = handle_contract_pairs
        .iter()
        .map(|pair| {
            let handle_bytes = pair.ctHandle.as_slice().to_vec();
            CiphertextHandle::new(handle_bytes)
        })
        .collect();

    debug!("Prepared {} ciphertext handles", ct_handles.len());

    Ok(ParsedUserDecryptionRequest::new(
        Some(sig),
        user_address,
        public_key_bytes.to_vec(),
        ct_handles,
        verifying_contract,
    ))
}

fn validate_config(config: &ResponseConfig) -> Result<()> {
    if config.kms_signers.as_ref().is_none_or(|s| s.is_empty()) {
        return Err(ClientCoreError::InvalidParams(
            "Missing KMS signers".to_string(),
        ));
    }

    if config.user_address.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing user address".to_string(),
        ));
    }

    if config.gateway_chain_id.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing gateway chain ID".to_string(),
        ));
    }

    if config.verifying_contract_address.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing verifying contract address".to_string(),
        ));
    }

    if config.signature.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing signature".to_string(),
        ));
    }

    if config.public_key.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing public key".to_string(),
        ));
    }

    if config.private_key.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing private key".to_string(),
        ));
    }

    if config
        .handle_contract_pairs
        .as_ref()
        .is_none_or(|p| p.is_empty())
    {
        return Err(ClientCoreError::InvalidParams(
            "Missing handle-contract pairs".to_string(),
        ));
    }

    if config.json_response.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing JSON response".to_string(),
        ));
    }

    Ok(())
}

/// Convenience function to create a user decryption response builder
pub fn process_user_decryption_response() -> UserDecryptionResponseBuilder {
    UserDecryptionResponseBuilder::new()
}
