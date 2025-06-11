//! Public decryption module for FHEVM SDK
//!
//! This module provides public decryption functionality following the same
//! builder pattern as user decryption for consistency and ease of use.

use crate::blockchain::calldata::public_decryption_req;
use crate::utils::{parse_hex_string, validate_address_from_str};
use crate::{FhevmError, Result};
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::sol_types::SolCall;
use alloy::sol_types::SolStruct;
use std::collections::HashMap;
use tracing::{debug, info};

/// Result of a public decryption - maps handle to decrypted value
pub type DecryptedResults = HashMap<String, serde_json::Value>;

/// Public decrypt request structure
#[derive(Debug, Clone)]
pub struct PublicDecryptRequest {
    /// Ciphertext handles to decrypt
    pub ct_handles: Vec<FixedBytes<32>>,
}

#[derive(Debug, Clone)]
pub struct PublicDecryptRequestBuilder {
    ct_handles: Vec<FixedBytes<32>>,
}

impl PublicDecryptRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            ct_handles: Vec::new(),
        }
    }

    /// Add handles from byte arrays
    ///
    /// # Arguments
    /// * `handles` - Array of 32-byte handles
    ///
    /// # Errors
    /// * If no handles are provided
    /// * If any handle is not exactly 32 bytes
    /// * If more than 256 handles are provided (protocol limit)
    pub fn add_handles_from_bytes(mut self, handles: &[Vec<u8>]) -> Result<Self> {
        if handles.is_empty() {
            return Err(FhevmError::InvalidParams(
                "At least one ciphertext handle is required".to_string(),
            ));
        }

        if handles.len() > 256 {
            return Err(FhevmError::InvalidParams(
                "Maximum 256 handles allowed in a single public decryption request".to_string(),
            ));
        }

        for (i, handle) in handles.iter().enumerate() {
            if handle.len() != 32 {
                return Err(FhevmError::InvalidParams(format!(
                    "Handle {} must be exactly 32 bytes, got {}",
                    i,
                    handle.len()
                )));
            }

            let fixed_bytes = FixedBytes::<32>::from_slice(handle);
            self.ct_handles.push(fixed_bytes);
        }

        Ok(self)
    }

    /// Add handles from hex strings
    ///
    /// # Arguments
    /// * `hex_handles` - Array of hex strings (with or without 0x prefix)
    ///
    /// # Example
    /// ```ignore
    /// builder.add_handles_from_hex(&[
    ///     "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200",
    ///     "a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890"
    /// ])?;
    /// ```
    pub fn add_handles_from_hex(mut self, hex_handles: &[&str]) -> Result<Self> {
        if hex_handles.is_empty() {
            return Err(FhevmError::InvalidParams(
                "At least one ciphertext handle is required".to_string(),
            ));
        }

        if hex_handles.len() > 256 {
            return Err(FhevmError::InvalidParams(
                "Maximum 256 handles allowed in a single public decryption request".to_string(),
            ));
        }

        for (i, hex_handle) in hex_handles.iter().enumerate() {
            let handle_bytes = parse_hex_string(hex_handle, &format!("handle {}", i))?;

            if handle_bytes.len() != 32 {
                return Err(FhevmError::InvalidParams(format!(
                    "Handle {} must be exactly 32 bytes, got {}",
                    i,
                    handle_bytes.len()
                )));
            }

            let fixed_bytes = FixedBytes::<32>::from_slice(&handle_bytes);
            self.ct_handles.push(fixed_bytes);
        }

        Ok(self)
    }

    /// Add a single handle from bytes
    pub fn add_handle(mut self, handle: &[u8]) -> Result<Self> {
        if handle.len() != 32 {
            return Err(FhevmError::InvalidParams(format!(
                "Handle must be exactly 32 bytes, got {}",
                handle.len()
            )));
        }

        let fixed_bytes = FixedBytes::<32>::from_slice(handle);
        self.ct_handles.push(fixed_bytes);
        Ok(self)
    }

    /// Clear all handles (useful for reusing the builder)
    pub fn clear(mut self) -> Self {
        self.ct_handles.clear();
        self
    }

    /// Get the number of handles currently added
    pub fn handle_count(&self) -> usize {
        self.ct_handles.len()
    }

    /// Build the request and generate calldata
    ///
    /// This is the final step that creates the transaction calldata ready to send to the blockchain.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - The encoded calldata bytes ready for your transaction
    /// * `Err(FhevmError)` - If no handles were added or validation fails
    pub fn build_and_generate_calldata(self) -> Result<Vec<u8>> {
        let request = self.build()?;
        let calldata = public_decryption_req(request.ct_handles)?;
        Ok(calldata.to_vec())
    }

    /// Build just the request object (without calldata generation)
    ///
    /// Use this method if you need the structured request object for other purposes
    /// besides generating calldata.
    pub fn build(self) -> Result<PublicDecryptRequest> {
        if self.ct_handles.is_empty() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing handles: Call `add_handles_from_bytes()` or `add_handles_from_hex()` first.\n\
                 💡 Tip: You need at least one encrypted handle to decrypt publicly."
                    .to_string(),
            ));
        }

        let request = PublicDecryptRequest {
            ct_handles: self.ct_handles,
        };

        debug!("✅ PublicDecryptRequest built successfully");
        debug!("   📊 Handles: {}", request.ct_handles.len());

        Ok(request)
    }
}

impl Default for PublicDecryptRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for processing public decryption responses
///
/// This builder provides a fluent API for configuring and executing
/// public decryption operations, matching the JavaScript implementation.
///
/// # Public Decryption Flow (from JS analysis)
///
/// 1. Check each handle is allowed for public decryption via ACL contract
/// 2. Send handles to relayer at `/v1/public-decrypt`
/// 3. Receive response with `decrypted_value` and `signatures`
/// 4. Verify signatures using EIP-712 with KMS signers
/// 5. Deserialize the decrypted result using ABI decoding
pub struct PublicDecryptionResponseBuilder {
    kms_signers: Option<Vec<String>>,
    threshold: Option<usize>,
    gateway_chain_id: Option<u64>,
    verifying_contract_address: Option<String>,
    ct_handles: Option<Vec<String>>,
    json_response: Option<String>,
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
            kms_signers: None,
            threshold: None,
            gateway_chain_id: None,
            verifying_contract_address: None,
            ct_handles: None,
            json_response: None,
        }
    }

    /// Set KMS signers (required)
    pub fn kms_signers(mut self, signers: Vec<String>) -> Self {
        self.kms_signers = Some(signers);
        self
    }

    /// Set threshold for KMS signers (required)
    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = Some(threshold);
        self
    }

    /// Add a single KMS signer (convenience method)
    pub fn add_kms_signer(mut self, signer: String) -> Self {
        self.kms_signers.get_or_insert_with(Vec::new).push(signer);
        self
    }

    /// Set gateway chain ID (required)
    pub fn gateway_chain_id(mut self, chain_id: u64) -> Self {
        self.gateway_chain_id = Some(chain_id);
        self
    }

    /// Set verifying contract address (required)
    pub fn verifying_contract_address(mut self, address: &str) -> Self {
        self.verifying_contract_address = Some(address.to_string());
        self
    }

    /// Set ciphertext handles as hex strings (required)
    pub fn ct_handles(mut self, handles: Vec<String>) -> Self {
        self.ct_handles = Some(handles);
        self
    }

    /// Set ciphertext handles from FixedBytes
    pub fn ct_handles_from_fixed_bytes(mut self, handles: &[FixedBytes<32>]) -> Self {
        let hex_handles = handles
            .iter()
            .map(|h| format!("0x{}", hex::encode(h)))
            .collect();
        self.ct_handles = Some(hex_handles);
        self
    }

    /// Set JSON response (required)
    pub fn json_response(mut self, response: &str) -> Self {
        self.json_response = Some(response.to_string());
        self
    }

    /// Validate that all required fields are set
    fn validate(&self) -> Result<()> {
        if self.kms_signers.as_ref().map_or(true, |s| s.is_empty()) {
            return Err(FhevmError::InvalidParams(
                "❌ Missing KMS signers: Call `kms_signers()` or `add_kms_signer()` first.\n\
                 💡 Tip: Add at least one KMS signer address that participates in the decryption."
                    .to_string(),
            ));
        }

        if self.threshold.is_none() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing threshold: Call `threshold()` first.\n\
                 💡 Tip: Set the minimum number of KMS signers required for decryption."
                    .to_string(),
            ));
        }

        if self.gateway_chain_id.is_none() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing gateway chain ID: Call `gateway_chain_id()` first.\n\
                 💡 Tip: This is the chain ID where the gateway contracts are deployed."
                    .to_string(),
            ));
        }

        if self.verifying_contract_address.is_none() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing verifying contract address: Call `verifying_contract_address()` first.\n\
                 💡 Tip: This is the address of the Decryption contract on the gateway."
                    .to_string()
            ));
        }

        if self.ct_handles.as_ref().map_or(true, |h| h.is_empty()) {
            return Err(FhevmError::InvalidParams(
                "❌ Missing ciphertext handles: Call `ct_handles()` or `ct_handles_from_fixed_bytes()` first.\n\
                 💡 Tip: Add the handles you want to decrypt publicly."
                    .to_string()
            ));
        }

        if self.json_response.is_none() {
            return Err(FhevmError::InvalidParams(
                "❌ Missing JSON response: Call `json_response()` first.\n\
                 💡 Tip: This should be the decryption response from the gateway/relayer."
                    .to_string(),
            ));
        }

        Ok(())
    }

    /// Process the public decryption response
    ///
    /// # Returns
    /// A map of handle to decrypted value
    pub fn process(self) -> Result<DecryptedResults> {
        // Validate all required fields
        self.validate()?;

        // Extract fields (safe unwraps after validation)
        let kms_signers = self.kms_signers.unwrap();
        let threshold = self.threshold.unwrap();
        let gateway_chain_id = self.gateway_chain_id.unwrap();
        let verifying_contract_address = self.verifying_contract_address.unwrap();
        let ct_handles = self.ct_handles.unwrap();
        let json_response = self.json_response.unwrap();

        info!("🔓 Processing public decryption response");
        info!("   KMS signers: {} signers", kms_signers.len());
        info!("   Threshold: {}", threshold);
        info!("   Gateway chain ID: {}", gateway_chain_id);
        info!("   Handles to decrypt: {}", ct_handles.len());

        // Parse the JSON response
        let response_data: serde_json::Value = serde_json::from_str(&json_response)
            .map_err(|e| FhevmError::DecryptionError(format!("JSON parse error: {}", e)))?;

        // Check for failure status
        if let Some(status) = response_data.get("status") {
            if status == "failure" {
                return Err(FhevmError::DecryptionError(
                    "Public decrypt failed: the public decrypt didn't succeed for an unknown reason".to_string()
                ));
            }
        }

        // Extract response array
        let responses = response_data
            .get("response")
            .and_then(|r| r.as_array())
            .ok_or_else(|| FhevmError::DecryptionError("No response array in JSON".to_string()))?;

        if responses.is_empty() {
            return Err(FhevmError::DecryptionError(
                "No responses in JSON".to_string(),
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
            .ok_or_else(|| {
                FhevmError::DecryptionError("Missing signatures in response".to_string())
            })?;

        // Normalize decrypted result (add 0x prefix if missing)
        let decrypted_result = if decrypted_value.starts_with("0x") {
            decrypted_value.to_string()
        } else {
            format!("0x{}", decrypted_value)
        };

        // Verify signatures using EIP-712
        let domain = alloy::sol_types::eip712_domain! {
            name: "Decryption",
            version: "1",
            chain_id: gateway_chain_id,
            verifying_contract: validate_address_from_str(&verifying_contract_address)?,
        };

        // Define the EIP-712 types
        alloy::sol! {
            struct PublicDecryptVerification {
                bytes32[] ctHandles;
                bytes decryptedResult;
            }
        }

        // Convert handles to bytes32
        let ct_handles_bytes32: Vec<FixedBytes<32>> = ct_handles
            .iter()
            .map(|h| {
                let bytes = if h.starts_with("0x") {
                    hex::decode(&h[2..])
                } else {
                    hex::decode(h)
                }
                .map_err(|e| FhevmError::InvalidParams(format!("Invalid handle hex: {}", e)))?;

                if bytes.len() != 32 {
                    return Err(FhevmError::InvalidParams(
                        "Handle must be 32 bytes".to_string(),
                    ));
                }

                Ok(FixedBytes::<32>::from_slice(&bytes))
            })
            .collect::<Result<Vec<_>>>()?;

        let decrypted_result_bytes = if decrypted_result.starts_with("0x") {
            hex::decode(&decrypted_result[2..])
        } else {
            hex::decode(&decrypted_result)
        }
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid decrypted result hex: {}", e)))?;

        let message = PublicDecryptVerification {
            ctHandles: ct_handles_bytes32.clone(),
            decryptedResult: Bytes::from(decrypted_result_bytes.clone()),
        };

        let signing_hash = message.eip712_signing_hash(&domain);

        // Verify each signature and collect recovered addresses
        let mut recovered_addresses = Vec::new();

        for (i, sig_value) in signatures.iter().enumerate() {
            let sig_str = sig_value.as_str().ok_or_else(|| {
                FhevmError::DecryptionError(format!("Signature {} is not a string", i))
            })?;

            let sig_bytes = if sig_str.starts_with("0x") {
                hex::decode(&sig_str[2..])
            } else {
                hex::decode(sig_str)
            }
            .map_err(|e| {
                FhevmError::DecryptionError(format!("Invalid signature {} hex: {}", i, e))
            })?;

            let signature = alloy::primitives::Signature::from_raw(&sig_bytes).map_err(|e| {
                FhevmError::DecryptionError(format!("Invalid signature {} format: {}", i, e))
            })?;

            let recovered = signature
                .recover_address_from_prehash(&signing_hash)
                .map_err(|e| {
                    FhevmError::DecryptionError(format!(
                        "Failed to recover address from signature {}: {}",
                        i, e
                    ))
                })?;

            debug!("Signature {} recovered address: {}", i, recovered);
            recovered_addresses.push(recovered.to_string());
        }

        // Check threshold is reached
        if !is_threshold_reached(&kms_signers, &recovered_addresses, threshold)? {
            return Err(FhevmError::DecryptionError(
                "KMS signers threshold is not reached".to_string(),
            ));
        }

        info!(
            "✅ Signature verification passed: {}/{} signers",
            recovered_addresses.len(),
            threshold
        );

        // Deserialize the decrypted result
        let results = deserialize_decrypted_result(&ct_handles, &decrypted_result)?;

        info!(
            "✅ Public decryption processed: {} values decrypted",
            results.len()
        );

        Ok(results)
    }
}

/// Check if threshold is reached with proper validation
fn is_threshold_reached(
    kms_signers: &[String],
    recovered_addresses: &[String],
    threshold: usize,
) -> Result<bool> {
    // Check for duplicates
    let mut seen = std::collections::HashSet::new();
    for addr in recovered_addresses {
        if !seen.insert(addr) {
            return Err(FhevmError::DecryptionError(format!(
                "Duplicate KMS signer address found: {} appears multiple times",
                addr
            )));
        }
    }

    // Normalize addresses for comparison (lowercase)
    let kms_signers_lower: Vec<String> = kms_signers.iter().map(|s| s.to_lowercase()).collect();

    // Check all recovered addresses are valid KMS signers
    for addr in recovered_addresses {
        let addr_lower = addr.to_lowercase();
        if !kms_signers_lower.contains(&addr_lower) {
            return Err(FhevmError::DecryptionError(format!(
                "Invalid address found: {} is not in the list of KMS signers",
                addr
            )));
        }
    }

    Ok(recovered_addresses.len() >= threshold)
}

/// Deserialize decrypted result using ABI decoding (matching JS implementation)
fn deserialize_decrypted_result(
    handles: &[String],
    decrypted_result: &str,
) -> Result<DecryptedResults> {
    use alloy::sol_types::SolValue;

    // Extract types from handles
    let mut types_list = Vec::new();
    for handle in handles {
        if handle.len() < 4 {
            return Err(FhevmError::DecryptionError("Handle too short".to_string()));
        }

        // Get the type discriminant from the handle (3rd and 4th last hex chars)
        let hex_pair = &handle[handle.len() - 4..handle.len() - 2];
        let type_discriminant = u8::from_str_radix(hex_pair, 16)
            .map_err(|e| FhevmError::DecryptionError(format!("Invalid handle type: {}", e)))?;
        types_list.push(type_discriminant);
    }

    // Decode the hex string
    let decrypted_bytes = if decrypted_result.starts_with("0x") {
        hex::decode(&decrypted_result[2..])
    } else {
        hex::decode(decrypted_result)
    }
    .map_err(|e| FhevmError::DecryptionError(format!("Invalid hex in decrypted result: {}", e)))?;

    // Create the restored encoded data (matching JS implementation)
    let mut restored_encoded = Vec::new();
    restored_encoded.extend_from_slice(&[0u8; 32]); // dummy requestID (ignored)
    restored_encoded.extend_from_slice(&decrypted_bytes);
    restored_encoded.extend_from_slice(&[0u8; 32]); // dummy empty bytes[] length (ignored)

    let mut results = DecryptedResults::new();

    // Simple ABI decoding for common cases
    // Full ABI decoding would be complex, so we handle the most common cases
    if handles.len() == 1 && restored_encoded.len() >= 96 {
        // Single value case - the actual value is between bytes 32-64
        let handle = &handles[0];
        let type_disc = types_list[0];
        let value_bytes = &restored_encoded[32..64];

        match type_disc {
            0 => {
                // bool - check if last byte is 1
                let is_true = value_bytes[31] == 1;
                results.insert(handle.clone(), serde_json::json!(is_true));
            }
            2..=6 | 8 => {
                // Numeric types - extract as big-endian uint256
                let value = alloy::primitives::U256::from_be_slice(value_bytes);
                results.insert(handle.clone(), serde_json::json!(value.to_string()));
            }
            7 => {
                // Address - last 20 bytes of the 32-byte slot
                let addr_bytes = &value_bytes[12..32];
                let addr = format!("0x{}", hex::encode(addr_bytes));
                results.insert(handle.clone(), serde_json::json!(addr));
            }
            9..=11 => {
                // Bytes types - would need full dynamic ABI decoding
                // For now, return the hex representation
                results.insert(
                    handle.clone(),
                    serde_json::json!(format!("0x{}", hex::encode(value_bytes))),
                );
            }
            _ => {
                return Err(FhevmError::DecryptionError(format!(
                    "Unknown type discriminant: {}",
                    type_disc
                )));
            }
        }
    } else if handles.len() > 1 {
        // Multiple values case
        // Each static value takes 32 bytes, starting after the dummy requestID
        let mut offset = 32; // Skip dummy requestID

        for (i, handle) in handles.iter().enumerate() {
            if offset + 32 > restored_encoded.len() {
                return Err(FhevmError::DecryptionError(
                    "Not enough data for all handles".to_string(),
                ));
            }

            let type_disc = types_list[i];
            let value_bytes = &restored_encoded[offset..offset + 32];

            match type_disc {
                0 => {
                    // bool
                    let is_true = value_bytes[31] == 1;
                    results.insert(handle.clone(), serde_json::json!(is_true));
                }
                2..=6 | 8 => {
                    // Numeric types
                    let value = alloy::primitives::U256::from_be_slice(value_bytes);
                    results.insert(handle.clone(), serde_json::json!(value.to_string()));
                }
                7 => {
                    // Address
                    let addr_bytes = &value_bytes[12..32];
                    let addr = format!("0x{}", hex::encode(addr_bytes));
                    results.insert(handle.clone(), serde_json::json!(addr));
                }
                9..=11 => {
                    // Bytes types - for static layout, just return hex of the slot
                    results.insert(
                        handle.clone(),
                        serde_json::json!(format!("0x{}", hex::encode(value_bytes))),
                    );
                }
                _ => {
                    return Err(FhevmError::DecryptionError(format!(
                        "Unknown type discriminant: {}",
                        type_disc
                    )));
                }
            }

            offset += 32;
        }
    } else {
        return Err(FhevmError::DecryptionError(
            "Invalid decrypted data format".to_string(),
        ));
    }

    Ok(results)
}

/// JSON response structure for public decryption
#[derive(Debug, serde::Deserialize)]
struct PublicDecryptionResponse {
    response: Vec<PublicDecryptionResult>,
    status: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct PublicDecryptionResult {
    decrypted_value: String,
    signatures: Vec<String>,
}

/// Convenience function to create a public decryption response builder
pub fn process_public_decryption_response() -> PublicDecryptionResponseBuilder {
    PublicDecryptionResponseBuilder::new()
}

// Re-export the convenience function at module level
pub use self::process_public_decryption_response as public_decrypt_response;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handles() -> Vec<Vec<u8>> {
        vec![vec![1u8; 32], vec![2u8; 32], vec![3u8; 32]]
    }

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
    fn test_public_decrypt_request_builder() {
        let handles = create_test_handles();

        let result = PublicDecryptRequestBuilder::new()
            .add_handles_from_bytes(&handles)
            .unwrap()
            .build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.ct_handles.len(), 3);
    }

    #[test]
    fn test_public_decrypt_request_builder_with_hex() {
        let hex_handles = vec![
            "0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390299",
            "a1b2c3d4e5f67890123456789012345678901234567890123456789012345600",
        ];

        let result = PublicDecryptRequestBuilder::new()
            .add_handles_from_hex(&hex_handles.iter().map(|s| *s).collect::<Vec<_>>())
            .unwrap()
            .build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.ct_handles.len(), 2);
    }

    #[test]
    fn test_public_decrypt_builder_validates_empty_handles() {
        let result = PublicDecryptRequestBuilder::new().build();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("❌ Missing handles"));
    }

    #[test]
    fn test_public_decrypt_builder_validates_handle_size() {
        let bad_handles = vec![vec![1u8; 31]]; // Wrong size

        let result = PublicDecryptRequestBuilder::new().add_handles_from_bytes(&bad_handles);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("32 bytes"));
    }

    #[test]
    fn test_public_decrypt_response_builder_validation() {
        let builder = PublicDecryptionResponseBuilder::new();
        let result = builder.process();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("❌ Missing"));
    }

    #[test]
    fn test_builder_clear_method() {
        let handles = create_test_handles();

        let builder = PublicDecryptRequestBuilder::new()
            .add_handles_from_bytes(&handles)
            .unwrap()
            .clear();

        assert_eq!(builder.handle_count(), 0);
    }

    #[test]
    fn test_add_single_handle() {
        let handle = vec![42u8; 32];

        let result = PublicDecryptRequestBuilder::new()
            .add_handle(&handle)
            .unwrap()
            .build();

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.ct_handles.len(), 1);
        assert_eq!(request.ct_handles[0].as_slice(), &handle);
    }

    #[test]
    fn test_calldata_generation() {
        let handles = create_test_handles();

        let result = PublicDecryptRequestBuilder::new()
            .add_handles_from_bytes(&handles)
            .unwrap()
            .build_and_generate_calldata();

        assert!(result.is_ok());
        let calldata = result.unwrap();
        assert!(!calldata.is_empty());
    }

    #[test]
    fn test_deserialize_single_uint32_value() {
        // Exact handle from your test output - uint32 (type 04)
        let handles =
            vec!["0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()];

        // Exact decrypted result from your test output for value 242
        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060";

        let results = deserialize_decrypted_result(&handles, decrypted_result).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[&handles[0]], serde_json::json!("242"));
    }

    #[test]
    fn test_deserialize_single_uint_value() {
        let handles =
            vec!["0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390500".to_string()];
        // This is a euint64 (type 05)

        // Simulated decrypted result for value 242
        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060";

        let results = deserialize_decrypted_result(&handles, decrypted_result).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[&handles[0]], serde_json::json!("242"));
    }

    #[test]
    fn test_deserialize_single_bool_value() {
        let handles =
            vec!["0xf2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390000".to_string()];
        // This is a ebool (type 00)

        // Simulated decrypted result for true - note the extra 0x60 at the end!
        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000060";

        let results = deserialize_decrypted_result(&handles, decrypted_result).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[&handles[0]], serde_json::json!(true));
    }

    #[test]
    fn test_full_public_decrypt_response_flow() {
        // Create a full test response matching your actual output
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
                ]
            }]
        }).to_string();

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
    fn test_public_decrypt_response_wrong_kms_signer() {
        // Same response as successful test
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
                ]
            }]
        }).to_string();

        // Use WRONG KMS signers that don't match the signature
        let wrong_kms_signers = vec![
            "0x1111111111111111111111111111111111111111".to_string(),
            "0x2222222222222222222222222222222222222222".to_string(),
        ];
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(wrong_kms_signers)
            .threshold(1)
            .gateway_chain_id(54321)
            .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .ct_handles(vec![handle.to_string()])
            .json_response(&json_response);

        let result = builder.process();

        assert!(
            result.is_err(),
            "Processing should fail with wrong KMS signers"
        );
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("is not in the list of KMS signers"),
            "Error should mention invalid KMS signer, got: {}",
            error
        );
    }

    #[test]
    fn test_public_decrypt_response_invalid_signature() {
        // Response with an invalid signature (wrong format)
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    // Invalid signature - too short (should be 65 bytes = 130 hex chars)
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d"
                ]
            }]
        }).to_string();

        let kms_signers = vec!["0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string()];
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(kms_signers)
            .threshold(1)
            .gateway_chain_id(54321)
            .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .ct_handles(vec![handle.to_string()])
            .json_response(&json_response);

        let result = builder.process();

        assert!(
            result.is_err(),
            "Processing should fail with invalid signature"
        );
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("Invalid signature") || error.contains("signature"),
            "Error should mention invalid signature format, got: {}",
            error
        );
    }

    #[test]
    fn test_public_decrypt_response_threshold_not_met() {
        // Response with only one signature but threshold requires 2
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
                ]
            }]
        }).to_string();

        let kms_signers = vec![
            "0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string(),
            "0xAnotherKMSSigner11111111111111111111111".to_string(),
        ];
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";

        let builder = PublicDecryptionResponseBuilder::new()
            .kms_signers(kms_signers)
            .threshold(2) // Require 2 signatures but only have 1
            .gateway_chain_id(54321)
            .verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .ct_handles(vec![handle.to_string()])
            .json_response(&json_response);

        let result = builder.process();

        assert!(
            result.is_err(),
            "Processing should fail when threshold not met"
        );
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("threshold is not reached"),
            "Error should mention threshold not reached, got: {}",
            error
        );
    }
}
