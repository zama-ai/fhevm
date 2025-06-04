//! Decryption module for FHEVM SDK

use crate::blockchain::calldata::user_decryption_req;
use crate::utils::validate_address_from_str;
use crate::utils::{JsonConverter, parse_hex_string};
use crate::{FhevmError, Result, types::DecryptedValue};
use alloy::primitives::{Address, Bytes, U256};
use alloy::signers::Signature;
use kms_grpc::kms::v1::TypedPlaintext;
use kms_lib::client::{CiphertextHandle, ParsedUserDecryptionRequest};

use crate::blockchain::bindings::Decryption::CtHandleContractPair;
use crate::blockchain::bindings::IDecryption::RequestValidity;
use kms_lib::client::js_api::{
    new_client, process_user_decryption_resp, u8vec_to_cryptobox_pk, u8vec_to_cryptobox_sk,
};

use log::{debug, info};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDecryptRequest {
    pub ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    pub request_validity: RequestValidity,
    pub contracts_chain_id: u64,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub signature: Bytes,
    pub public_key: Bytes,
}

/// Builder pattern for creating UserDecryptRequest instances
pub struct UserDecryptRequestBuilder {
    ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    contract_addresses: Vec<Address>,
    user_address: Option<Address>,
    signature: Option<Bytes>,
    public_key: Option<Bytes>,
    start_timestamp: Option<u64>,
    duration_days: Option<u64>,
    contracts_chain_id: Option<u64>,
}

impl UserDecryptRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            ct_handle_contract_pairs: Vec::new(),
            contract_addresses: Vec::new(),
            user_address: None,
            signature: None,
            public_key: None,
            start_timestamp: None,
            duration_days: None,
            contracts_chain_id: None,
        }
    }

    pub fn add_handles_from_bytes(
        mut self,
        handles: &[Vec<u8>],
        contract_addresses: &[Address],
    ) -> Result<Self> {
        // Validate inputs upfront
        if handles.is_empty() {
            return Err(FhevmError::InvalidParams(
                "At least one ciphertext handle is required".to_string(),
            ));
        }

        if contract_addresses.is_empty() {
            return Err(FhevmError::InvalidParams(
                "At least one contract address is required".to_string(),
            ));
        }

        if contract_addresses.len() > 10 {
            return Err(FhevmError::InvalidParams(
                "Maximum 10 contract addresses allowed".to_string(),
            ));
        }

        // Process handles with validation
        for (i, handle) in handles.iter().enumerate() {
            if handle.len() != 32 {
                return Err(FhevmError::InvalidParams(format!(
                    "Handle {} must be exactly 32 bytes, got {}",
                    i,
                    handle.len()
                )));
            }

            let handle_u256 = U256::from_be_slice(handle);
            let contract_addr = contract_addresses[i % contract_addresses.len()];

            self.ct_handle_contract_pairs.push(CtHandleContractPair {
                ctHandle: handle_u256.into(),
                contractAddress: contract_addr,
            });
        }

        // Store unique contract addresses
        for &addr in contract_addresses {
            if !self.contract_addresses.contains(&addr) {
                self.contract_addresses.push(addr);
            }
        }

        Ok(self)
    }

    pub fn user_address_from_str(mut self, address: &str) -> Result<Self> {
        if address.trim().is_empty() {
            return Err(FhevmError::InvalidParams(
                "User address cannot be empty".to_string(),
            ));
        }

        let addr = validate_address_from_str(address)?;

        if addr.is_zero() {
            return Err(FhevmError::InvalidParams(
                "Zero address is not allowed for user".to_string(),
            ));
        }

        self.user_address = Some(addr);
        Ok(self)
    }

    pub fn signature_from_hex(mut self, signature: &str) -> Result<Self> {
        let signature_bytes = parse_hex_string(signature, "signature")?;

        // Validate signature length (should be 65 bytes for ECDSA)
        if signature_bytes.len() != 65 {
            return Err(FhevmError::InvalidParams(format!(
                "Invalid signature length: expected 65 bytes, got {}",
                signature_bytes.len()
            )));
        }

        self.signature = Some(signature_bytes);
        Ok(self)
    }

    pub fn public_key_from_hex(mut self, public_key: &str) -> Result<Self> {
        let public_key_bytes = parse_hex_string(public_key, "public key")?;

        if public_key_bytes.is_empty() {
            return Err(FhevmError::InvalidParams(
                "Public key cannot be empty".to_string(),
            ));
        }

        self.public_key = Some(public_key_bytes);
        Ok(self)
    }

    pub fn validity(mut self, start_timestamp: u64, duration_days: u64) -> Result<Self> {
        if duration_days == 0 {
            return Err(FhevmError::InvalidParams(
                "Duration days cannot be zero".to_string(),
            ));
        }

        if duration_days > 365 {
            return Err(FhevmError::InvalidParams(
                "Duration days cannot exceed 365".to_string(),
            ));
        }

        // Validate timestamp is not in the future (with 5 minute tolerance)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if start_timestamp > now + 300 {
            // 5 minute tolerance
            return Err(FhevmError::InvalidParams(
                "Start timestamp cannot be more than 5 minutes in the future".to_string(),
            ));
        }

        self.start_timestamp = Some(start_timestamp);
        self.duration_days = Some(duration_days);
        Ok(self)
    }

    pub fn contracts_chain_id(mut self, chain_id: u64) -> Self {
        self.contracts_chain_id = Some(chain_id);
        self
    }

    /// ## **Build the request and generate calldata**
    ///
    /// This is the final step that creates the transaction calldata ready to send to the blockchain.
    /// All required fields must be set before calling this method.
    ///
    /// ## Returns
    /// * `Ok(Vec<u8>)` - The encoded calldata bytes ready for your transaction
    /// * `Err(FhevmError)` - If any required fields are missing or validation fails
    ///
    ///
    /// ## Example
    /// ```ignore
    /// // Complete flow ending with calldata generation
    /// let calldata = UserDecryptRequestBuilder::new()
    ///     .add_handles_from_bytes(&handles, &contracts)?
    ///     .user_address_from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")?
    ///     .signature_from_hex("0x1234567890abcdef...")?
    ///     .public_key_from_hex("0x2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331")?
    ///     .validity(1640995200, 30)?
    ///     .build_and_generate_calldata()?;
    ///
    /// // Now you can use the calldata in your transaction
    /// println!("Calldata ready: {} bytes", calldata.len());
    /// send_transaction_with_calldata(calldata)?;
    /// ```
    ///
    /// # Required Fields Check
    /// This method will return helpful errors if you're missing any required fields:
    /// - "❌ Missing handles: Call `add_handles_from_bytes()` first"
    /// - "❌ Missing user address: Call `user_address_from_str()` first"  
    /// - "❌ Missing signature: Call `signature_from_hex()` first"
    /// - "❌ Missing public key: Call `public_key_from_hex()` first"
    /// - "❌ Missing validity: Call `validity()` first"
    ///
    /// # Typical Usage Pattern
    /// ```ignore
    /// match builder.build_and_generate_calldata() {
    ///     Ok(calldata) => {
    ///         // Success! Send the transaction
    ///         send_transaction(calldata).await?;
    ///     }
    ///     Err(e) => {
    ///         // The error message will tell you exactly what's missing
    ///         println!("Build failed: {}", e);
    ///         // Example: "Missing signature: Call `signature_from_hex()` first"
    ///     }
    /// }
    /// ```
    pub fn build_and_generate_calldata(self) -> Result<Vec<u8>> {
        let request = self.build()?;
        let calldata = user_decryption_req(request)?;
        Ok(calldata.to_vec())
    }

    /// **Alternative: Build just the request object (without calldata generation)**
    ///
    /// Use this method if you need the structured request object for other purposes
    /// besides generating calldata (e.g., serialization, inspection, or custom encoding).
    ///
    /// # Returns
    /// * `Ok(UserDecryptRequest)` - The structured request object
    /// * `Err(FhevmError)` - If any required fields are missing or validation fails
    ///
    /// # When to Use This
    /// - **Debugging**: Inspect the request structure before encoding
    /// - **Custom encoding**: You want to encode the request differently
    /// - **Multiple outputs**: Generate both calldata and other formats
    /// - **Testing**: Validate request structure in tests
    ///
    /// # Example
    /// ```ignore
    /// // Build the request object
    /// let request = UserDecryptRequestBuilder::new()
    ///     .add_handles_from_bytes(&handles, &contracts)?
    ///     .user_address_from_str("0x742d35Cc6634C0532925a3b8D8d8E4C9B4c5D2B1")?
    ///     .signature_from_hex("0x1234...")?
    ///     .public_key_from_hex("0xabcd...")?
    ///     .validity(1640995200, 30)?
    ///     .build()?;
    ///
    /// // Now you can:
    /// println!("Request has {} handles", request.ct_handle_contract_pairs.len());
    /// println!("User address: {}", request.user_address);
    /// println!("Valid for {} days", request.request_validity.durationDays);
    ///
    /// // Generate calldata separately if needed
    /// let calldata = user_decryption_req(request)?;
    /// ```
    ///
    /// # Request Structure
    /// The returned `UserDecryptRequest` contains:
    /// - `ct_handle_contract_pairs`: Handle-contract mappings
    /// - `request_validity`: Start time and duration
    /// - `contracts_chain_id`: Chain ID for contracts
    /// - `contract_addresses`: List of contract addresses
    /// - `user_address`: User who can decrypt
    /// - `signature`: EIP-712 signature
    /// - `public_key`: User's public key for decryption
    pub fn build(self) -> Result<UserDecryptRequest> {
        // Check handles
        if self.ct_handle_contract_pairs.is_empty() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing handles: Call `add_handles_from_bytes()` first.\n\
                 💡 Tip: You need at least one encrypted handle to decrypt."
                    .to_string(),
            ));
        }

        // Check user address
        let user_address = self.user_address.ok_or_else(|| {
            FhevmError::InvalidParams(
                "❌ Missing user address: Call `user_address_from_str()` first.\n\
                 💡 Tip: Specify who can decrypt the data with their Ethereum address."
                    .to_string(),
            )
        })?;

        // Check signature
        let signature = self.signature.ok_or_else(|| {
            FhevmError::InvalidParams(
                "❌ Missing signature: Call `signature_from_hex()` first.\n\
                 💡 Tip: Add the EIP-712 signature that proves authorization."
                    .to_string(),
            )
        })?;

        // Check public key
        let public_key = self.public_key.ok_or_else(|| {
            FhevmError::InvalidParams(
                "❌ Missing public key: Call `public_key_from_hex()` first.\n\
                 💡 Tip: Add the user's public key for decryption."
                    .to_string(),
            )
        })?;

        // Check validity
        let start_timestamp = self.start_timestamp.ok_or_else(|| {
            FhevmError::InvalidParams(
                "❌ Missing validity: Call `validity()` first.\n\
                 💡 Tip: Set when and for how long the decryption permission is valid."
                    .to_string(),
            )
        })?;

        let duration_days = self.duration_days.ok_or_else(|| {
            FhevmError::InvalidParams(
                "❌ Missing validity: Call `validity()` first.\n\
                 💡 Tip: Set when and for how long the decryption permission is valid."
                    .to_string(),
            )
        })?;

        // Use default chain ID if not set (usually set by SDK)
        let contracts_chain_id = self.contracts_chain_id.unwrap_or(1);

        // Additional validation
        if self.contract_addresses.is_empty() {
            return Err(FhevmError::InvalidParams(
                "❌ No contract addresses found. This should not happen if you called `add_handles_from_bytes()`.\n\
                 💡 Tip: Make sure you added handles with valid contract addresses.".to_string(),
            ));
        }

        // Create the request validity object
        let request_validity = RequestValidity {
            startTimestamp: U256::from(start_timestamp),
            durationDays: U256::from(duration_days),
        };

        // Build the final request
        let request = UserDecryptRequest {
            ct_handle_contract_pairs: self.ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses: self.contract_addresses,
            user_address,
            signature,
            public_key,
        };

        log::debug!("✅ UserDecryptRequest built successfully");
        log::debug!("   📊 Handles: {}", request.ct_handle_contract_pairs.len());
        log::debug!("   👤 User: {}", request.user_address);
        log::debug!("   🏢 Contracts: {}", request.contract_addresses.len());
        log::debug!(
            "   ⏰ Duration: {} days",
            request.request_validity.durationDays
        );

        Ok(request)
    }
}

impl Default for UserDecryptRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Process user decryption using KMS library
///
/// This function replicates the JavaScript functionality:
/// 1. Creates a KMS client with signers and user address
/// 2. Constructs the EIP-712 domain with gateway chain ID
/// 3. Builds the verification payload
/// 4. Processes the decryption response from the gateway
/// 5. Returns the decrypted results as a vector
pub fn process_user_decryption_response(
    kms_signers: &[String],
    user_address: &str,
    gateway_chain_id: u64,
    verifying_contract_address: &str,
    signature: &str,
    public_key: &str,
    private_key: &str,
    handle_contract_pairs: &[CtHandleContractPair],
    json_response: &str,
) -> Result<Vec<TypedPlaintext>> {
    info!("🔐 Processing user decryption with KMS api");
    info!("   KMS signers: {:?}", kms_signers);
    info!("   User address: {}", user_address);
    info!("   Gateway chain ID: {}", gateway_chain_id);

    debug!("📋 Verification of inputs");
    let user_address_verified = validate_address_from_str(user_address)?;
    let public_key_bytes = parse_hex_string(public_key, "public_key")?;
    let private_key_bytes = parse_hex_string(private_key, "private_key")?;
    let verifying_contract_address_checked = validate_address_from_str(verifying_contract_address)?;

    if kms_signers.is_empty() {
        return Err(FhevmError::DecryptionError(
            "KMS signers cannot be empty".to_string(),
        ));
    }

    // Step 1: Create KMS client
    let mut client = new_client(kms_signers.to_vec(), user_address, "default")
        .map_err(|_| FhevmError::DecryptionError(format!("Failed to create KMS client")))?;

    debug!("✅ KMS client created successfully");

    // Step 2: Create the gateway chain ID buffer (32 bytes, big-endian)
    let mut chain_id_buffer = [0u8; 32];
    // Set the gateway chain ID in the last 4 bytes (positions 28-31)
    let chain_id_bytes = gateway_chain_id.to_be_bytes();
    if gateway_chain_id <= u32::MAX as u64 {
        // For values that fit in u32, place them at position 28
        chain_id_buffer[28..32].copy_from_slice(&(gateway_chain_id as u32).to_be_bytes());
    } else {
        // For larger values, place the full u64 at the end
        chain_id_buffer[24..32].copy_from_slice(&chain_id_bytes);
    }

    debug!("🔢 Chain ID buffer: {}", hex::encode(&chain_id_buffer));

    // Step 3: Build EIP-712 domain
    let eip712_domain = kms_grpc::kms::v1::Eip712DomainMsg {
        name: "Decryption".to_string(),
        version: "1".to_string(),
        chain_id: chain_id_buffer.to_vec(),
        verifying_contract: verifying_contract_address.to_string(),
        salt: None,
    };

    debug!("📝 EIP-712 domain constructed");

    // Step 4: Prepare handles
    let ciphertext_handles: Vec<String> = handle_contract_pairs
        .iter()
        .map(|pair| hex::encode(pair.ctHandle))
        .collect();

    debug!(
        "🔗 Prepared {} ciphertext handles",
        ciphertext_handles.len()
    );

    // Prepare signature for payload
    let sig = parse_hex_string(signature, "signature")?;
    let sign = Signature::from_raw(sig.iter().as_slice())
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid signature format: {}", e)))?;

    // Prepare handles into specific type, .i.e. `kms_lib::client::CiphertextHandle`
    let ct_handles: Vec<CiphertextHandle> = ciphertext_handles
        .iter()
        .map(|h| parse_hex_string(h, "handle").map(|bytes| CiphertextHandle::new(bytes.to_vec())))
        .collect::<Result<Vec<_>>>()?;

    let payload = ParsedUserDecryptionRequest::new(
        Some(sign),
        user_address_verified,
        public_key_bytes.clone().into(),
        ct_handles,
        verifying_contract_address_checked,
    );

    // Convert an array of user decryption response into kms
    // friendly type, i.e. `kms_grpc::kms::v1::UserDecryptionResponse;`
    let responses = JsonConverter::json_to_responses(json_response)?;

    // Step 6: Convert keys for KMS processing

    let crypto_pub_key = u8vec_to_cryptobox_pk(&public_key_bytes).map_err(|e| {
        FhevmError::DecryptionError(format!("Failed to convert public key: {:?}", e.to_owned()))
    })?;

    let crypto_priv_key = u8vec_to_cryptobox_sk(&private_key_bytes).map_err(|e| {
        FhevmError::DecryptionError(format!("Failed to convert private key: {:?}", e.to_owned()))
    })?;

    // Process the decryption response using KMS library
    let decryption_result = process_user_decryption_resp(
        &mut client,
        Some(payload),
        Some(eip712_domain),
        responses,
        &crypto_pub_key,
        &crypto_priv_key,
        false, // verify signatures
    )
    .map_err(|e| {
        FhevmError::DecryptionError(format!("KMS decryption failed: {:?}", e.to_owned()))
    })?;

    log::info!(
        "✅ User decryption processed successfully {:?}",
        decryption_result
    );

    Ok(decryption_result)
}

/// Public decrypt operation
pub fn public_decrypt(ciphertext: &[u8], _public_key: &[u8]) -> Result<DecryptedValue> {
    // Placeholder implementation
    if ciphertext.is_empty() {
        return Err(FhevmError::DecryptionError(
            "No ciphertext provided".to_string(),
        ));
    }

    // Return mock decrypted value
    Ok(DecryptedValue(vec![42]))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::bindings::Decryption::CtHandleContractPair;
    use alloy::primitives::{Address, U256};
    use std::str::FromStr;

    /// Creates test data matching the JavaScript reference implementation
    fn create_test_data() -> TestData {
        TestData {
            kms_signers: vec!["0x67F6A11ADf13CEDdB8319Fe12705809563611703".to_string()],
            user_address: "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80".to_string(),
            gateway_chain_id: 54321,
            verifying_contract_address: "0xc9bAE822fE6793e3B456144AdB776D5A318CB71e".to_string(),
            signature: "791e8a06dab85d960745c4c5dea65fdc250e0d42cbfbd2037ae221d2baa980c062f8b46f023c11bba8ba49c17e9e73a8ce0556040c567849b62b675678c3bc071c".to_string(),
            public_key: "2000000000000000750f4e54713eae622dfeb01809290183a447e2b277e89d2c6a681af1aa5b2c2b".to_string(),
            private_key: "2000000000000000321387e7b579a16d9bcb17d14625dc2841314c05f7c266418a9576091178902d".to_string(),
            handle_contract_pairs: vec![
                CtHandleContractPair {
                    ctHandle: U256::from_str("0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200").unwrap().into(),
                    contractAddress: Address::from_str("0xa3f4D50ebfea1237316b4377F0fff4831F2D1c46").unwrap(),
                }
            ],
            json_response: create_mock_json_response(),
        }
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

    struct TestData {
        kms_signers: Vec<String>,
        user_address: String,
        gateway_chain_id: u64,
        verifying_contract_address: String,
        signature: String,
        public_key: String,
        private_key: String,
        handle_contract_pairs: Vec<CtHandleContractPair>,
        json_response: String,
    }

    /// Helper function to call process_user_decryption with test data
    fn call_process_user_decryption(test_data: &TestData) -> Result<Vec<TypedPlaintext>> {
        process_user_decryption_response(
            &test_data.kms_signers,
            &test_data.user_address,
            test_data.gateway_chain_id,
            &test_data.verifying_contract_address,
            &test_data.signature,
            &test_data.public_key,
            &test_data.private_key,
            &test_data.handle_contract_pairs,
            &test_data.json_response,
        )
    }

    /// Helper to assert error contains expected keywords
    fn assert_error_contains(result: &Result<Vec<TypedPlaintext>>, keywords: &[&str]) {
        let error = result
            .as_ref()
            .expect_err("Expected an error but got success");
        let error_msg = error.to_string().to_lowercase();

        let found = keywords
            .iter()
            .any(|&keyword| error_msg.contains(&keyword.to_lowercase()));

        assert!(
            found,
            "Error '{}' should contain one of: {:?}",
            error, keywords
        );
    }

    #[test]
    fn test_process_user_decryption_success() {
        let test_data = create_test_data();
        let result = call_process_user_decryption(&test_data);

        match result {
            Ok(decrypted_values) => {
                // Based on the test data, we expect the decrypted value to be 42
                let first_value = &decrypted_values[0];
                assert_eq!(first_value.as_u8(), 42, "Decrypted value should be 42");
            }
            Err(e) => {
                panic!("Expected success but got error: {}", e);
            }
        }
    }

    #[test]
    fn test_process_user_decryption_invalid_signature() {
        let mut test_data = create_test_data();
        test_data.signature = "invalid_signature".to_string();

        let result = call_process_user_decryption(&test_data);

        assert_error_contains(&result, &["signature", "hex"]);
    }

    #[test]
    fn test_process_user_decryption_invalid_address() {
        let mut test_data = create_test_data();
        test_data.user_address = "invalid_address".to_string();

        let result = call_process_user_decryption(&test_data);

        assert_error_contains(&result, &["address"]);
    }

    #[test]
    fn test_process_user_decryption_malformed_json() {
        let mut test_data = create_test_data();
        test_data.json_response = "{ malformed json".to_string();

        let result = call_process_user_decryption(&test_data);

        assert_error_contains(&result, &["json", "parse"]);
    }

    #[test]
    fn test_process_user_decryption_empty_signers() {
        let mut test_data = create_test_data();
        test_data.kms_signers = vec![];

        let result = call_process_user_decryption(&test_data);

        assert_error_contains(&result, &["kms", "client"]);
    }

    #[test]
    fn test_process_user_decryption_invalid_public_key() {
        let mut test_data = create_test_data();
        test_data.public_key = "invalid_key".to_string();

        let result = call_process_user_decryption(&test_data);

        assert_error_contains(&result, &["public key", "hex", "key"]);
    }

    #[test]
    fn test_multiple_error_scenarios() {
        let test_cases: &[(&str, Box<dyn Fn(&mut TestData)>, &[&str])] = &[
            (
                "invalid_sig",
                Box::new(|data| data.signature = "bad".to_string()),
                &["signature", "hex"],
            ),
            (
                "bad_address",
                Box::new(|data| data.user_address = "bad".to_string()),
                &["address"],
            ),
            (
                "bad_json",
                Box::new(|data| data.json_response = "{bad}".to_string()),
                &["json"],
            ),
            (
                "empty_signers",
                Box::new(|data| data.kms_signers = vec![]),
                &["kms", "client"],
            ),
            (
                "bad_key",
                Box::new(|data| data.public_key = "bad".to_string()),
                &["key", "hex"],
            ),
        ];

        for (name, modify_data, expected_keywords) in test_cases {
            let mut test_data = create_test_data();
            modify_data(&mut test_data);

            let result = call_process_user_decryption(&test_data);

            assert!(result.is_err(), "Test case '{}' should fail", name);
            assert_error_contains(&result, expected_keywords);
        }
    }

    #[test]
    fn test_chain_id_buffer_creation() {
        let gateway_chain_id = 54321u64;
        let mut chain_id_buffer = [0u8; 32];

        if gateway_chain_id <= u32::MAX as u64 {
            chain_id_buffer[28..32].copy_from_slice(&(gateway_chain_id as u32).to_be_bytes());
        } else {
            chain_id_buffer[24..32].copy_from_slice(&gateway_chain_id.to_be_bytes());
        }

        // Verify against JS reference: [0,0,...,0,0,212,49]
        assert_eq!(chain_id_buffer[30], 212);
        assert_eq!(chain_id_buffer[31], 49);
        assert_eq!(&chain_id_buffer[..28], &[0u8; 28]);

        let expected_hex = "000000000000000000000000000000000000000000000000000000000000d431";
        assert_eq!(hex::encode(&chain_id_buffer), expected_hex);
    }

    #[test]
    fn test_handle_preparation() {
        let test_data = create_test_data();

        let ciphertext_handles: Vec<String> = test_data
            .handle_contract_pairs
            .iter()
            .map(|pair| hex::encode(pair.ctHandle))
            .collect();

        assert_eq!(ciphertext_handles.len(), 1);

        let expected_handle = "f2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200";
        assert_eq!(ciphertext_handles[0], expected_handle);
    }
}
