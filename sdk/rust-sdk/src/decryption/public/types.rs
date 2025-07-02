use serde::Deserialize;
use std::collections::HashMap;

/// Result of a public decryption - maps handle to decrypted value
pub type DecryptedResults = HashMap<String, serde_json::Value>;

/// JSON response structure for public decryption
#[derive(Debug, Deserialize)]
pub struct PublicDecryptionResponse {
    pub response: Vec<PublicDecryptionResult>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PublicDecryptionResult {
    pub decrypted_value: String,
    pub signatures: Vec<String>,
}

/// Configuration for response processing
#[derive(Default)]
pub(super) struct ResponseConfig {
    pub kms_signers: Option<Vec<String>>,
    pub threshold: Option<usize>,
    pub gateway_chain_id: Option<u64>,
    pub verifying_contract_address: Option<String>,
    pub ct_handles: Option<Vec<String>>,
    pub json_response: Option<String>,
}
