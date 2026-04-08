use super::deserializer::deserialize_decrypted_result;
use super::types::{DecryptedResults, ResponseConfig};
use super::verification::verify_signatures;
use crate::{ClientCoreError, Result};
use alloy::primitives::FixedBytes;
use tracing::{debug, info};

/// Builder for processing public decryption responses.
pub struct PublicDecryptionResponseBuilder {
    config: ResponseConfig,
}

impl Default for PublicDecryptionResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PublicDecryptionResponseBuilder {
    pub fn new() -> Self {
        Self {
            config: ResponseConfig::default(),
        }
    }

    pub fn with_kms_signers(mut self, signers: Vec<String>) -> Self {
        self.config.kms_signers = Some(signers);
        self
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.config.threshold = Some(threshold);
        self
    }

    pub fn with_kms_signer(mut self, signer: String) -> Self {
        self.config
            .kms_signers
            .get_or_insert_with(Vec::new)
            .push(signer);
        self
    }

    pub fn with_gateway_chain_id(mut self, chain_id: u64) -> Self {
        self.config.gateway_chain_id = Some(chain_id);
        self
    }

    pub fn with_verifying_contract_address(mut self, address: &str) -> Self {
        self.config.verifying_contract_address = Some(address.to_string());
        self
    }

    pub fn with_ct_handles(mut self, handles: Vec<String>) -> Self {
        self.config.ct_handles = Some(handles);
        self
    }

    pub fn with_ct_handles_from_fixed_bytes(mut self, handles: &[FixedBytes<32>]) -> Self {
        let hex_handles = handles
            .iter()
            .map(|h| format!("0x{}", hex::encode(h)))
            .collect();
        self.config.ct_handles = Some(hex_handles);
        self
    }

    pub fn with_json_response(mut self, response: &str) -> Self {
        self.config.json_response = Some(response.to_string());
        self
    }

    /// Process the response: verify signatures and deserialize decrypted values.
    pub fn process(self) -> Result<DecryptedResults> {
        validate_config(&self.config)?;

        let kms_signers = self.config.kms_signers.ok_or_else(|| {
            ClientCoreError::InvalidParams("KMS signers not configured".into())
        })?;
        let threshold = self.config.threshold.ok_or_else(|| {
            ClientCoreError::InvalidParams("Threshold not configured".into())
        })?;
        let gateway_chain_id = self.config.gateway_chain_id.ok_or_else(|| {
            ClientCoreError::InvalidParams("Gateway chain ID not configured".into())
        })?;
        let verifying_contract_address =
            self.config.verifying_contract_address.ok_or_else(|| {
                ClientCoreError::InvalidParams("Verifying contract address not configured".into())
            })?;
        let ct_handles = self.config.ct_handles.ok_or_else(|| {
            ClientCoreError::InvalidParams("Ciphertext handles not configured".into())
        })?;
        let json_response = self.config.json_response.ok_or_else(|| {
            ClientCoreError::InvalidParams("JSON response not configured".into())
        })?;

        info!("Processing public decryption response");
        info!("   KMS signers: {} signers", kms_signers.len());
        info!("   Threshold: {}", threshold);
        info!("   Handles to decrypt: {}", ct_handles.len());

        let response_data = parse_json_response(&json_response)?;

        debug!("Parsed JSON response successfully: {}", response_data);

        let (decrypted_value, signatures) = extract_response_data(&response_data)?;

        debug!("Decrypted value: {}", decrypted_value);
        debug!("Number of signatures: {}", signatures.len());

        info!(
            "Verifying {} signatures against threshold of {}",
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

        info!("Deserializing {} decrypted values", ct_handles.len());
        let results = deserialize_decrypted_result(&ct_handles, &decrypted_value)?;

        info!(
            "Public decryption processed successfully: {} values decrypted",
            results.len()
        );

        Ok(results)
    }
}

fn parse_json_response(json_response: &str) -> Result<serde_json::Value> {
    serde_json::from_str(json_response)
        .map_err(|e| ClientCoreError::DecryptionError(format!("JSON parse error: {e}")))
}

fn extract_response_data(response_data: &serde_json::Value) -> Result<(String, Vec<String>)> {
    let responses = response_data
        .get("response")
        .and_then(|r| r.as_array())
        .ok_or_else(|| {
            ClientCoreError::DecryptionError("No response array in JSON".to_string())
        })?;

    if responses.is_empty() {
        return Err(ClientCoreError::DecryptionError(
            "No responses in JSON response array".to_string(),
        ));
    }

    if responses.len() > 1 {
        return Err(ClientCoreError::DecryptionError(format!(
            "Expected exactly 1 response entry, got {}. Multi-response batches are not supported.",
            responses.len()
        )));
    }

    let result = &responses[0];

    let decrypted_value = result
        .get("decrypted_value")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            ClientCoreError::DecryptionError("Missing decrypted_value in response".to_string())
        })?;

    let signatures = result
        .get("signatures")
        .and_then(|s| s.as_array())
        .ok_or_else(|| {
            ClientCoreError::DecryptionError("Missing signatures in response".to_string())
        })?
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.as_str()
                .ok_or_else(|| {
                    ClientCoreError::DecryptionError(format!("Signature {i} is not a string"))
                })
                .map(|s| s.to_string())
        })
        .collect::<Result<Vec<_>>>()?;

    if signatures.is_empty() {
        return Err(ClientCoreError::DecryptionError(
            "No signatures provided in response".to_string(),
        ));
    }

    let decrypted_result = if decrypted_value.starts_with("0x") {
        decrypted_value.to_string()
    } else {
        format!("0x{decrypted_value}")
    };

    Ok((decrypted_result, signatures))
}

fn validate_config(config: &ResponseConfig) -> Result<()> {
    if config.kms_signers.as_ref().is_none_or(|s| s.is_empty()) {
        return Err(ClientCoreError::InvalidParams(
            "Missing KMS signers: Call `with_kms_signers()` first.".to_string(),
        ));
    }

    if config.threshold.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing threshold: Call `with_threshold()` first.".to_string(),
        ));
    }

    if config.gateway_chain_id.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing gateway chain ID: Call `with_gateway_chain_id()` first.".to_string(),
        ));
    }

    if config.verifying_contract_address.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing verifying contract address: Call `with_verifying_contract_address()` first."
                .to_string(),
        ));
    }

    if config.ct_handles.as_ref().is_none_or(|h| h.is_empty()) {
        return Err(ClientCoreError::InvalidParams(
            "Missing ciphertext handles: Call `with_ct_handles()` first.".to_string(),
        ));
    }

    if config.json_response.is_none() {
        return Err(ClientCoreError::InvalidParams(
            "Missing JSON response: Call `with_json_response()` first.".to_string(),
        ));
    }

    Ok(())
}

/// Create a new builder for processing public decryption responses.
pub fn process_public_decryption_response() -> PublicDecryptionResponseBuilder {
    PublicDecryptionResponseBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_validation() {
        let builder = PublicDecryptionResponseBuilder::new();
        let result = builder.process();

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Missing"));
    }

    #[test]
    fn test_full_public_decrypt_response_flow() {
        let json_response = serde_json::json!({
            "response": [{
                "decrypted_value": "00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060",
                "signatures": [
                    "1a320a79075486da76ba3e9009200d298e35b7b4f0cd17364830a6793d74609a5862922b0e4af639e34de9dc40a5a48b390fb5c27fa8523d402e9a167c3444d61c"
                ]
            }]
        }).to_string();

        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";

        let builder = PublicDecryptionResponseBuilder::new()
            .with_kms_signers(vec![
                "0xF7a67027C94d141e5ebC7AeEE03FDF5fa8E0580C".to_string(),
            ])
            .with_threshold(1)
            .with_gateway_chain_id(54321)
            .with_verifying_contract_address("0xc9bAE822fE6793e3B456144AdB776D5A318CB71e")
            .with_ct_handles(vec![handle.to_string()])
            .with_json_response(&json_response);

        let result = builder.process();

        assert!(result.is_ok(), "Processing should succeed");
        let decrypted = result.unwrap();

        assert_eq!(decrypted.len(), 1);
        assert_eq!(decrypted[handle], serde_json::json!("242"));
    }
}
