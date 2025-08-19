use super::deserializer::UserDecryptionDeserializer;
use super::types::ResponseConfig;
use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{FhevmError, Result};
use alloy::primitives::Address;
use alloy::signers::Signature;
use fhevm_gateway_rust_bindings::decryption::Decryption::CtHandleContractPair;
use kms_grpc::kms::v1::{Eip712DomainMsg, TypedPlaintext};
use kms_grpc::rpc_types::protobuf_to_alloy_domain;
use kms_lib::client::js_api::{new_client, new_server_id_addr};
use kms_lib::client::{CiphertextHandle, ParsedUserDecryptionRequest};
use kms_lib::consts::SAFE_SER_SIZE_LIMIT;
use kms_lib::cryptography::internal_crypto_types::{
    PrivateEncKey, UnifiedPrivateEncKey, UnifiedPublicEncKey,
};
use tracing::{debug, info};

/// Builder for processing user decryption responses
///
/// This builder provides a fluent API for configuring and executing
/// user decryption operations with proper validation at processing time.
///
/// # Example
///
/// ```no_run
/// # use fhevm_gateway_rust_bindings::decryption::Decryption::CtHandleContractPair;
/// # use gateway_sdk::decryption::user::process_user_decryption_response;
/// # use gateway_sdk::FhevmError;
/// # use alloy::primitives::{Address, U256};
/// # use std::str::FromStr;
/// #
/// # fn example() -> Result<(), FhevmError> {
/// # let handle_pairs = vec![]; // Your handle-contract pairs
/// # let json_response = "{}"; // Response from gateway
/// #
/// let results = process_user_decryption_response()
///     .with_kms_signers(vec!["0x67F6A11ADf13CEDdB8319Fe12705809563611703".to_string()])
///     .with_user_address("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80")
///     .with_gateway_chain_id(54321)
///     .with_verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
///     .with_signature("791e8a06dab85d960745c4c5dea65fdc250e0d42...")
///     .with_public_key("2000000000000000750f4e54713eae622dfeb01809290183...")
///     .with_private_key("2000000000000000321387e7b579a16d9bcb17d14625dc28...")
///     .with_handle_contract_pairs(handle_pairs)
///     .with_json_response(json_response)
///     .with_verification(true) // Optional
///     .process()?;
///
/// # Ok(())
/// # }
/// ```
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

        info!("🔐 Processing user decryption response");
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
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid public key: {e:?}")))?;

        let ml_kem_priv_key =
            bc2wrap::deserialize::<PrivateEncKey<ml_kem::MlKem512>>(&private_key_bytes)
                .map(UnifiedPrivateEncKey::MlKem512)
                .map_err(|e| FhevmError::DecryptionError(format!("Invalid private key: {e:?}")))?;

        let eip712_domain = protobuf_to_alloy_domain(&eip712_domain).unwrap();

        let decryption_result = if self.config.verify_signatures {
            client
                .process_user_decryption_resp(
                    &payload,
                    &eip712_domain,
                    &responses,
                    &ml_kem_pub_key,
                    &ml_kem_priv_key,
                )
                .map_err(|e| FhevmError::DecryptionError(format!("KMS decryption failed: {e:?}")))?
        } else {
            client
                .insecure_process_user_decryption_resp(
                    &responses,
                    &ml_kem_pub_key,
                    &ml_kem_priv_key,
                )
                .map_err(|e| FhevmError::DecryptionError(format!("KMS decryption failed: {e:?}")))?
        };

        info!(
            "✅ User decryption processed successfully: {} results",
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
) -> Result<kms_lib::client::Client> {
    // Convert string addresses to ServerIdAddr objects
    let server_id_addrs = kms_signers
        .iter()
        .enumerate()
        .map(|(index, addr)| {
            new_server_id_addr((index + 1) as u32, addr.clone()).map_err(|e| {
                FhevmError::DecryptionError(format!("Invalid KMS signer address {addr}: {e:?}"))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    new_client(server_id_addrs, user_address, "default")
        .map_err(|e| FhevmError::DecryptionError(format!("Failed to create KMS client: {e:?}")))
}

fn build_eip712_domain(
    domain: &str,
    gateway_chain_id: u64,
    verifying_contract: &str,
) -> Eip712DomainMsg {
    // Create chain ID buffer (32 bytes, big-endian)
    let mut chain_id_buffer = [0u8; 32];
    if gateway_chain_id <= u32::MAX as u64 {
        chain_id_buffer[28..32].copy_from_slice(&(gateway_chain_id as u32).to_be_bytes());
    } else {
        chain_id_buffer[24..32].copy_from_slice(&gateway_chain_id.to_be_bytes());
    }

    debug!("🔢 Chain ID buffer: {}", hex::encode(chain_id_buffer));

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
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid signature format: {e}")))?;

    // Convert handles
    let ct_handles: Vec<CiphertextHandle> = handle_contract_pairs
        .iter()
        .map(|pair| {
            let handle_bytes = pair.ctHandle.as_slice().to_vec();
            CiphertextHandle::new(handle_bytes)
        })
        .collect();

    debug!("🔗 Prepared {} ciphertext handles", ct_handles.len());

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
        return Err(FhevmError::InvalidParams(
            "❌ Missing KMS signers: Call `kms_signers()` or `add_kms_signer()` first.\n\
             💡 Tip: Add at least one KMS signer address that will participate in the decryption."
                .to_string(),
        ));
    }

    if config.user_address.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing user address: Call `user_address()` first.\n\
             💡 Tip: This should be the address of the user requesting the decryption."
                .to_string(),
        ));
    }

    if config.gateway_chain_id.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing gateway chain ID: Call `gateway_chain_id()` first.\n\
             💡 Tip: This is the chain ID where the gateway contracts are deployed."
                .to_string(),
        ));
    }

    if config.verifying_contract_address.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing verifying contract address: Call `verifying_contract_address()` first.\n\
             💡 Tip: This is the address of the contract that will verify the decryption request."
                .to_string(),
        ));
    }

    if config.signature.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing signature: Call `signature()` first.\n\
             💡 Tip: This should be the EIP-712 signature for the decryption request."
                .to_string(),
        ));
    }

    if config.public_key.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing public key: Call `public_key()` first.\n\
             💡 Tip: This is the user's ML-KEM public key for receiving encrypted responses."
                .to_string(),
        ));
    }

    if config.private_key.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing private key: Call `private_key()` first.\n\
             💡 Tip: This is the user's ML-KEM private key for decrypting responses.\n\
             ⚠️  Security: Never share or log this key!"
                .to_string(),
        ));
    }

    if config
        .handle_contract_pairs
        .as_ref()
        .is_none_or(|p| p.is_empty())
    {
        return Err(FhevmError::InvalidParams(
            "❌ Missing handle-contract pairs: Call `handle_contract_pairs()` first.\n\
             💡 Tip: Add the ciphertext handles and their associated contract addresses to decrypt."
                .to_string(),
        ));
    }

    if config.json_response.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing JSON response: Call `json_response()` first.\n\
             💡 Tip: This should be the decryption response received from the gateway/relayer."
                .to_string(),
        ));
    }

    Ok(())
}

/// Convenience function to create a user decryption response builder
pub fn process_user_decryption_response() -> UserDecryptionResponseBuilder {
    UserDecryptionResponseBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;
    use serde_json::Value;
    use std::str::FromStr;

    #[test]
    fn test_builder_validation() {
        let builder = UserDecryptionResponseBuilder::new();
        let result = builder.process();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("❌ Missing"));
    }

    //  Load test data from JSON file
    fn load_test_data() -> Value {
        let test_data_json = include_str!("../../../test_data/user_decryption_test_data.json");
        serde_json::from_str(test_data_json).expect("Failed to parse test data JSON")
    }

    //  Helper function to create handle pairs from JSON
    fn create_handle_pairs_from_json(data: &Value) -> Vec<CtHandleContractPair> {
        data["handle_contract_pairs"]
            .as_array()
            .expect("handle_contract_pairs should be an array")
            .iter()
            .map(|pair| {
                let ct_handle = U256::from_str(
                    pair["ct_handle"]
                        .as_str()
                        .expect("ct_handle should be a string"),
                )
                .expect("Invalid ct_handle format");

                let contract_address = Address::from_str(
                    pair["contract_address"]
                        .as_str()
                        .expect("contract_address should be a string"),
                )
                .expect("Invalid contract_address format");

                CtHandleContractPair {
                    ctHandle: ct_handle.into(),
                    contractAddress: contract_address,
                }
            })
            .collect()
    }

    // Helper function to build test from JSON data
    fn build_test_from_json(test_name: &str) -> UserDecryptionResponseBuilder {
        let test_data = load_test_data();
        let test_case = &test_data["user_decryption_test_data"][test_name];
        let input = &test_case["input"];
        let json_response = &test_case["json_response"];

        // Extract KMS signers
        let kms_signers: Vec<String> = input["kms_signers"]
            .as_array()
            .expect("kms_signers should be an array")
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("KMS signer should be a string")
                    .to_string()
            })
            .collect();

        // Create handle pairs
        let handle_pairs = create_handle_pairs_from_json(input);

        // Build the test
        process_user_decryption_response()
            .with_kms_signers(kms_signers)
            .with_user_address(
                input["user_address"]
                    .as_str()
                    .expect("user_address required"),
            )
            .with_gateway_chain_id(
                input["gateway_chain_id"]
                    .as_u64()
                    .expect("gateway_chain_id required"),
            )
            .with_verifying_contract_address(
                input["verifying_contract_address"]
                    .as_str()
                    .expect("verifying_contract_address required"),
            )
            .with_signature(input["signature"].as_str().expect("signature required"))
            .with_public_key(input["public_key"].as_str().expect("public_key required"))
            .with_private_key(input["private_key"].as_str().expect("private_key required"))
            .with_handle_contract_pairs(handle_pairs)
            .with_domain(input["domain"].as_str().unwrap_or("Decryption")) // Default domain
            .with_json_response(
                &serde_json::to_string(json_response).expect("Failed to serialize json_response"),
            )
    }

    #[test]
    fn test_process_with_complete_data_kms_rc26() {
        let builder = build_test_from_json("rc26_complete_test");
        let result = builder.process();

        // Load expected output from test data
        let test_data = load_test_data();
        let expected =
            &test_data["user_decryption_test_data"]["rc26_complete_test"]["expected_output"];

        if expected["success"].as_bool().unwrap_or(false) {
            assert!(result.is_ok(), "KMS RC26 test should succeed");

            let plaintexts = result.unwrap();
            let expected_values = expected["decrypted_values"].as_array().unwrap();

            assert_eq!(plaintexts.len(), expected_values.len());

            for (i, expected_value) in expected_values.iter().enumerate() {
                let plaintext = &plaintexts[i];

                // Check FHE type
                assert_eq!(
                    plaintext.fhe_type,
                    expected_value["fhe_type"].as_i64().unwrap() as i32
                );

                // Check decrypted value
                if let Some(expected_u64) = expected_value["as_u64"].as_u64() {
                    assert_eq!(plaintext.as_u64(), expected_u64);
                }
            }
        } else {
            // Test is expected to fail - check the reason
            println!(
                "Test expected to fail: {}",
                expected["reason"].as_str().unwrap_or("Unknown")
            );
        }
    }

    #[test]
    fn test_json_data_integrity() {
        let test_data = load_test_data();

        // Verify structure of test data
        assert!(test_data["user_decryption_test_data"].is_object());
        assert!(test_data["user_decryption_test_data"]["rc26_complete_test"].is_object());

        // Verify required fields exist
        let rc17_test = &test_data["user_decryption_test_data"]["rc26_complete_test"];
        assert!(rc17_test["input"].is_object());
        assert!(rc17_test["json_response"].is_object());
        assert!(rc17_test["expected_output"].is_object());

        // Verify arrays have expected lengths
        let input = &rc17_test["input"];
        assert!(!input["kms_signers"].as_array().unwrap().is_empty());
        assert!(
            !input["handle_contract_pairs"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_builder_domain() {
        let builder = UserDecryptionResponseBuilder::new();
        assert_eq!(builder.config.domain, Some("Decryption".to_string()));

        let builder = builder.with_domain("CustomDomain");
        assert_eq!(builder.config.domain, Some("CustomDomain".to_string()));
    }
}
