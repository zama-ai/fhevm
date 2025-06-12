use super::deserializer::deserialize_decrypted_result;
use super::types::{DecryptedResults, ResponseConfig};
use super::verification::verify_signatures;
use crate::{FhevmError, Result};
use alloy::primitives::FixedBytes;
use tracing::{debug, info};

/// Builder for processing public decryption responses
///
/// This builder provides a fluent API for configuring and executing
/// public decryption operations, matching the JavaScript implementation.
///
/// # Public Decryption Flow (from JS analysis)
///
/// 1. Check each handle is allowed for public decryption via ACL contract (this step is not done currently)
/// 2. Send handles to relayer at `/v1/public-decrypt` (This step will no longer be needed with this sdk)
/// 3. Receive response with `decrypted_value` and `signatures`
/// 4. Verify signatures using EIP-712 with KMS signers
/// 5. Deserialize the decrypted result using ABI decoding
///
/// # Example
///
/// ```no_run
/// # use gateway_sdk::decryption::public::{process_public_decryption_response, DecryptedResults};
/// # use gateway_sdk::FhevmError;
/// #
/// # fn example() -> Result<DecryptedResults, FhevmError> {
/// // Process a public decryption response from the gateway
/// let results = process_public_decryption_response()
///     .kms_signers(vec![
///         "0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string()
///     ])
///     .threshold(1)
///     .gateway_chain_id(54321)
///     .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
///     .ct_handles(vec![
///         "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()
///     ])
///     .json_response(r#"{
///         "response": [{
///             "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
///             "signatures": ["1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"]
///         }]
///     }"#)
///     .process()?;
///
/// // Access decrypted values
/// for (handle, value) in &results {
///     println!("Handle {}: {:?}", handle, value);
/// }
/// # Ok(results)
/// # }
/// ```
pub struct PublicDecryptionResponseBuilder {
    config: ResponseConfig,
}

impl Default for PublicDecryptionResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicDecryptionResponseBuilder {
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

    /// Set threshold for KMS signers (required)
    pub fn threshold(mut self, threshold: usize) -> Self {
        self.config.threshold = Some(threshold);
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

    /// Set ciphertext handles as hex strings (required)
    pub fn ct_handles(mut self, handles: Vec<String>) -> Self {
        self.config.ct_handles = Some(handles);
        self
    }

    /// Set ciphertext handles from FixedBytes
    pub fn ct_handles_from_fixed_bytes(mut self, handles: &[FixedBytes<32>]) -> Self {
        let hex_handles = handles
            .iter()
            .map(|h| format!("0x{}", hex::encode(h)))
            .collect();
        self.config.ct_handles = Some(hex_handles);
        self
    }

    /// Set JSON response (required)
    pub fn json_response(mut self, response: &str) -> Self {
        self.config.json_response = Some(response.to_string());
        self
    }

    /// Process the public decryption response
    ///
    /// # Returns
    /// A map of handle to decrypted value
    ///
    /// # Flow
    ///
    /// 1. **Validation**: Ensures all required fields are set
    /// 2. **JSON Parsing**: Parses the gateway response
    /// 3. **Status Check**: Verifies the decryption succeeded
    /// 4. **Signature Verification**: Validates KMS signatures using EIP-712
    /// 5. **Result Deserialization**: Decodes the ABI-encoded results
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required fields are missing
    /// - JSON parsing fails
    /// - Status indicates failure
    /// - Signature verification fails
    /// - Threshold is not reached
    /// - Deserialization fails
    pub fn process(self) -> Result<DecryptedResults> {
        // Validate all required fields
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

    fn process(self) -> Result<DecryptedResults> {
        // Extract fields (safe unwraps after validation)
        let kms_signers = self.config.kms_signers.unwrap();
        let threshold = self.config.threshold.unwrap();
        let gateway_chain_id = self.config.gateway_chain_id.unwrap();
        let verifying_contract_address = self.config.verifying_contract_address.unwrap();
        let ct_handles = self.config.ct_handles.unwrap();
        let json_response = self.config.json_response.unwrap();

        info!("🔓 Processing public decryption response");
        info!("   KMS signers: {} signers", kms_signers.len());
        info!("   Threshold: {}", threshold);
        info!("   Gateway chain ID: {}", gateway_chain_id);
        info!("   Handles to decrypt: {}", ct_handles.len());

        // Parse and process the response
        let response_data = parse_json_response(&json_response)?;

        debug!("Parsed JSON response successfully: {}", response_data);

        // Extract decrypted value and signatures
        let (decrypted_value, signatures) = extract_response_data(&response_data)?;

        debug!("Decrypted value: {}", decrypted_value);
        debug!("Number of signatures: {}", signatures.len());

        // Verify signatures using EIP-712
        info!(
            "🔐 Verifying {} signatures against threshold of {}",
            signatures.len(),
            threshold
        );
        verify_signatures(
            &kms_signers,
            threshold,
            gateway_chain_id,
            &verifying_contract_address,
            &ct_handles,
            &decrypted_value,
            &signatures,
        )?;

        // Deserialize the decrypted result based on handle types
        info!("📦 Deserializing {} decrypted values", ct_handles.len());
        let results = deserialize_decrypted_result(&ct_handles, &decrypted_value)?;

        info!(
            "✅ Public decryption processed successfully: {} values decrypted",
            results.len()
        );

        // Log individual results at debug level
        for (handle, value) in &results {
            debug!("   Handle {}: {:?}", &handle[..16], value);
        }

        Ok(results)
    }
}

fn parse_json_response(json_response: &str) -> Result<serde_json::Value> {
    serde_json::from_str(json_response)
        .map_err(|e| FhevmError::DecryptionError(format!("JSON parse error: {}", e)))
}

fn extract_response_data(response_data: &serde_json::Value) -> Result<(String, Vec<String>)> {
    // Extract response array
    let responses = response_data
        .get("response")
        .and_then(|r| r.as_array())
        .ok_or_else(|| FhevmError::DecryptionError("No response array in JSON".to_string()))?;

    if responses.is_empty() {
        return Err(FhevmError::DecryptionError(
            "No responses in JSON response array".to_string(),
        ));
    }

    // Process the first response (matching JS behavior)
    let result = &responses[0];

    // Extract decrypted value
    let decrypted_value = result
        .get("decrypted_value")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            FhevmError::DecryptionError("Missing decrypted_value in response".to_string())
        })?;

    // Extract signatures
    let signatures = result
        .get("signatures")
        .and_then(|s| s.as_array())
        .ok_or_else(|| FhevmError::DecryptionError("Missing signatures in response".to_string()))?
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.as_str()
                .ok_or_else(|| {
                    FhevmError::DecryptionError(format!("Signature {} is not a string", i))
                })
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<_>>>()?;

    if signatures.is_empty() {
        return Err(FhevmError::DecryptionError(
            "No signatures provided in response".to_string(),
        ));
    }

    // Normalize decrypted result (add 0x prefix if missing)
    let decrypted_result = if decrypted_value.starts_with("0x") {
        decrypted_value.to_string()
    } else {
        format!("0x{}", decrypted_value)
    };

    debug!(
        "Extracted decrypted result: {} bytes",
        decrypted_result.len()
    );
    debug!("Extracted {} signatures", signatures.len());

    Ok((decrypted_result, signatures))
}

fn validate_config(config: &ResponseConfig) -> Result<()> {
    if config.kms_signers.as_ref().map_or(true, |s| s.is_empty()) {
        return Err(FhevmError::InvalidParams(
            "❌ Missing KMS signers: Call `kms_signers()` or `add_kms_signer()` first.\n\
             💡 Tip: Add at least one KMS signer address that participates in the decryption."
                .to_string(),
        ));
    }

    if config.threshold.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing threshold: Call `threshold()` first.\n\
             💡 Tip: Set the minimum number of KMS signers required for decryption."
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
             💡 Tip: This is the address of the Decryption contract on the gateway."
                .to_string(),
        ));
    }

    if config.ct_handles.as_ref().map_or(true, |h| h.is_empty()) {
        return Err(FhevmError::InvalidParams(
            "❌ Missing ciphertext handles: Call `ct_handles()` or `ct_handles_from_fixed_bytes()` first.\n\
             💡 Tip: Add the handles you want to decrypt publicly."
                .to_string()
        ));
    }

    if config.json_response.is_none() {
        return Err(FhevmError::InvalidParams(
            "❌ Missing JSON response: Call `json_response()` first.\n\
             💡 Tip: This should be the decryption response from the gateway/relayer."
                .to_string(),
        ));
    }

    Ok(())
}

/// Convenience function to create a public decryption response builder
pub fn process_public_decryption_response() -> PublicDecryptionResponseBuilder {
    PublicDecryptionResponseBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_response() -> String {
        serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
                ]
            }]
        }).to_string()
    }

    #[test]
    fn test_builder_validation() {
        let builder = PublicDecryptionResponseBuilder::new();
        let result = builder.process();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("❌ Missing"));
    }

    #[test]
    fn test_full_public_decrypt_response_flow() {
        // Create a full test response matching the actual output
        let json_response = create_test_response();

        // Build response processor with test data
        let kms_signers = vec!["0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string()];
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400"; // uint32

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(kms_signers)
            .threshold(1)
            .gateway_chain_id(54321)
            .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .ct_handles(vec![handle.to_string()])
            .json_response(&json_response);

        let result = builder.process();

        assert!(result.is_ok(), "Processing should succeed");
        let decrypted = result.unwrap();

        assert_eq!(decrypted.len(), 1);
        assert_eq!(decrypted[handle], serde_json::json!("242"));
    }

    #[test]
    fn test_response_with_failure_status() {
        let json_response = serde_json::json!({
            "status": "failure",
            "error": "ACL check failed",
            "response": []
        })
        .to_string();

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(vec!["0xtest".to_string()])
            .threshold(1)
            .gateway_chain_id(1)
            .verifying_contract_address("0x1234567890123456789012345678901234567890")
            .ct_handles(vec!["0x".to_string()])
            .json_response(&json_response);

        let result = builder.process();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ACL check failed"));
    }

    #[test]
    fn test_missing_signatures() {
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "0x1234",
                "signatures": []
            }]
        })
        .to_string();

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(vec!["0xtest".to_string()])
            .threshold(1)
            .gateway_chain_id(1)
            .verifying_contract_address("0x1234567890123456789012345678901234567890")
            .ct_handles(vec!["0xhandle".to_string()])
            .json_response(&json_response);

        let result = builder.process();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No signatures"));
    }

    #[test]
    fn test_normalize_decrypted_value() {
        // Test with 0x prefix
        let response_data = serde_json::json!({
            "response": [{
                "decrypted_value": "0xabcd",
                "signatures": ["sig1"]
            }]
        });

        let (value, _) = extract_response_data(&response_data).unwrap();
        assert_eq!(value, "0xabcd");

        // Test without 0x prefix
        let response_data = serde_json::json!({
            "response": [{
                "decrypted_value": "abcd",
                "signatures": ["sig1"]
            }]
        });

        let (value, _) = extract_response_data(&response_data).unwrap();
        assert_eq!(value, "0xabcd");
    }
}
