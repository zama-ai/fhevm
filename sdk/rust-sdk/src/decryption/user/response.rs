use super::deserializer::UserDecryptionDeserializer;
use super::types::ResponseConfig;
use crate::blockchain::bindings::Decryption::CtHandleContractPair;
use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{FhevmError, Result};
use alloy::primitives::Address;
use alloy::signers::Signature;
use kms_grpc::kms::v1::{Eip712DomainMsg, TypedPlaintext};
use kms_lib::client::js_api::{
    new_client, new_server_id_addr, process_user_decryption_resp, u8vec_to_ml_kem_pke_pk, u8vec_to_ml_kem_pke_sk
};
use kms_lib::client::{CiphertextHandle, ParsedUserDecryptionRequest};
use tracing::{debug, info};

/// Builder for processing user decryption responses
///
/// This builder provides a fluent API for configuring and executing
/// user decryption operations with proper validation at processing time.
///
/// # Example
///
/// ```no_run
/// # use gateway_sdk::decryption::user::process_user_decryption_response;
/// # use gateway_sdk::blockchain::bindings::Decryption::CtHandleContractPair;
/// # use gateway_sdk::FhevmError;
/// # use alloy::primitives::{Address, U256};
/// # use std::str::FromStr;
/// #
/// # fn example() -> Result<(), FhevmError> {
/// # let handle_pairs = vec![]; // Your handle-contract pairs
/// # let json_response = "{}"; // Response from gateway
/// #
/// let results = process_user_decryption_response()
///     .kms_signers(vec!["0x67F6A11ADf13CEDdB8319Fe12705809563611703".to_string()])
///     .user_address("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80")
///     .gateway_chain_id(54321)
///     .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
///     .signature("791e8a06dab85d960745c4c5dea65fdc250e0d42...")
///     .public_key("2000000000000000750f4e54713eae622dfeb01809290183...")
///     .private_key("2000000000000000321387e7b579a16d9bcb17d14625dc28...")
///     .handle_contract_pairs(handle_pairs)
///     .json_response(json_response)
///     .verify_signatures(true) // Optional
///     .process()?;
///
/// println!("Decrypted {} values", results.len());
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
    pub fn kms_signers(mut self, signers: Vec<String>) -> Self {
        self.config.kms_signers = Some(signers);
        self
    }

    /// Add a single KMS signer (convenience method)
    pub fn add_kms_signer(mut self, signer: String) -> Self {
        self.config
            .kms_signers
            .get_or_insert_with(Vec::new)
            .push(signer);
        self
    }

    /// Set user address (required)
    pub fn user_address(mut self, address: &str) -> Self {
        self.config.user_address = Some(address.to_string());
        self
    }

    /// Set gateway chain ID (required)
    pub fn gateway_chain_id(mut self, chain_id: u64) -> Self {
        self.config.gateway_chain_id = Some(chain_id);
        self
    }

    /// Set verifying contract address (required)
    pub fn verifying_contract_address(mut self, address: &str) -> Self {
        self.config.verifying_contract_address = Some(address.to_string());
        self
    }

    /// Set signature (required)
    pub fn signature(mut self, signature: &str) -> Self {
        self.config.signature = Some(signature.to_string());
        self
    }

    /// Set public key (required)
    pub fn public_key(mut self, key: &str) -> Self {
        self.config.public_key = Some(key.to_string());
        self
    }

    /// Set private key (required)
    pub fn private_key(mut self, key: &str) -> Self {
        self.config.private_key = Some(key.to_string());
        self
    }

    /// Set handle-contract pairs (required)
    pub fn handle_contract_pairs(mut self, pairs: Vec<CtHandleContractPair>) -> Self {
        self.config.handle_contract_pairs = Some(pairs);
        self
    }

    /// Set JSON response (required)
    pub fn json_response(mut self, response: &str) -> Self {
        self.config.json_response = Some(response.to_string());
        self
    }

    /// Enable or disable signature verification (optional, default: false)
    pub fn verify_signatures(mut self, verify: bool) -> Self {
        self.config.verify_signatures = verify;
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
        let mut client = create_kms_client(&kms_signers, &user_address)?;

        // Build EIP-712 domain
        let eip712_domain = build_eip712_domain(gateway_chain_id, &verifying_contract_address);

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

        // Convert keys for cryptobox
        let crypto_pub_key = u8vec_to_ml_kem_pke_pk(&public_key_bytes)
            .map_err(|e| FhevmError::DecryptionError(format!("Invalid public key: {:?}", e)))?;

        let crypto_priv_key = u8vec_to_ml_kem_pke_sk(&private_key_bytes)
            .map_err(|e| FhevmError::DecryptionError(format!("Invalid private key: {:?}", e)))?;

        // Process decryption
        let decryption_result = process_user_decryption_resp(
            &mut client,
            Some(payload),
            Some(eip712_domain),
            responses,
            &crypto_pub_key,
            &crypto_priv_key,
            self.config.verify_signatures,
        )
        .map_err(|e| FhevmError::DecryptionError(format!("KMS decryption failed: {:?}", e)))?;

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
    let kms_signers = kms_signers.iter()
        .enumerate()
        .map(|(i, s)| {
            new_server_id_addr(i as u32, s.clone())
                .map_err(|_| FhevmError::AddressError(s.clone()))
        })
        .collect::<Result<Vec<_>>>()?;

    new_client(kms_signers, user_address, "default")
        .map_err(|_| FhevmError::DecryptionError("Failed to create KMS client".to_string()))
}

fn build_eip712_domain(gateway_chain_id: u64, verifying_contract: &str) -> Eip712DomainMsg {
    // Create chain ID buffer (32 bytes, big-endian)
    let mut chain_id_buffer = [0u8; 32];
    if gateway_chain_id <= u32::MAX as u64 {
        chain_id_buffer[28..32].copy_from_slice(&(gateway_chain_id as u32).to_be_bytes());
    } else {
        chain_id_buffer[24..32].copy_from_slice(&gateway_chain_id.to_be_bytes());
    }

    debug!("🔢 Chain ID buffer: {}", hex::encode(&chain_id_buffer));

    Eip712DomainMsg {
        name: "Decryption".to_string(),
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
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid signature format: {}", e)))?;

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
        public_key_bytes.to_vec().into(),
        ct_handles,
        verifying_contract,
    ))
}

fn validate_config(config: &ResponseConfig) -> Result<()> {
    if config.kms_signers.as_ref().map_or(true, |s| s.is_empty()) {
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
             💡 Tip: This is the user's cryptobox public key for receiving encrypted responses."
                .to_string(),
        ));
    }

    if config.private_key.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing private key: Call `private_key()` first.\n\
             💡 Tip: This is the user's cryptobox private key for decrypting responses.\n\
             ⚠️  Security: Never share or log this key!"
                .to_string(),
        ));
    }

    if config
        .handle_contract_pairs
        .as_ref()
        .map_or(true, |p| p.is_empty())
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
    use std::str::FromStr;

    fn create_test_handle_pairs() -> Vec<CtHandleContractPair> {
        vec![CtHandleContractPair {
            ctHandle: U256::from_str(
                "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200",
            )
            .unwrap()
            .into(),
            contractAddress: Address::from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46")
                .unwrap(),
        }]
    }

    fn create_mock_json_response() -> String {
        serde_json::json!({
            "response": [
                {
                    "payload": "29000000000000002100000000000000029395c8ff9ca2d768dd40bf9fb6dfc834874487da26218ee57a929228b807ff2b20000000000000002a92056afa790a38b17237730d08ef686c04a2e0dac55aec05b97f26c79a95a50100000000000000020000001501000000000000c5000000000000003b62b10c9abb1f9c4caef03543917fa093758c0b6eb22444293172d287415966f72a4bb1c352aacf7c0003652653aefedb05871dbf068643e8f57baa56a631b579ea0d062921c178e9a73ca341d8a983687a84cd1690af7f4679642a5e3209f8d902c9fcde4a18d8c2dc5bd06d30cdae4ae26c838d35199db8497d454fa4dfc6ec43315254b901d4262fb07f0a039b9523ea0aa658ea583ed29fe54ce22d9fa361502be74746c993e814e6685e7ba723cfcd7b590fa394efbd9068156dfc17d9d3c8c5fa7f1800000000000000717abaaaeb83db7e49cac2168789d3184de51040f7205936200000000000000031604bdf7defdf92477633d530e37899aa12b94dcf132fb6d717aad48b8b625d2000000000000000f2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200010000000100000000000000",
                    "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }).to_string()
    }

    #[test]
    fn test_builder_validation() {
        let builder = UserDecryptionResponseBuilder::new();
        let result = builder.process();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("❌ Missing"));
    }

    #[test]
    fn test_process_with_complete_data() {
        let builder = process_user_decryption_response()
            .kms_signers(vec!["0x67F6A11ADf13CEDdB8319Fe12705809563611703".to_string()])
            .user_address("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80")
            .gateway_chain_id(54321)
            .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .signature("791e8a06dab85d960745c4c5dea65fdc250e0d42cbfbd2037ae221d2baa980c062f8b46f023c11bba8ba49c17e9e73a8ce0556040c567849b62b675678c3bc071c")
            .public_key("2000000000000000750f4e54713eae622dfeb01809290183a447e2b277e89d2c6a681af1aa5b2c2b")
            .private_key("2000000000000000321387e7b579a16d9bcb17d14625dc2841314c05f7c266418a9576091178902d")
            .handle_contract_pairs(create_test_handle_pairs())
            .json_response(&create_mock_json_response());

        let result = builder.process();
        assert!(result.is_ok());

        let plaintexts = result.unwrap();
        assert_eq!(plaintexts.len(), 1);
        assert_eq!(plaintexts[0].as_u8(), 42);
    }
}
