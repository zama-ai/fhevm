use alloy::primitives::{Address, Bytes};
use fhevm_gateway_rust_bindings::decryption::{
    Decryption::CtHandleContractPair, IDecryption::RequestValidity,
};
use kms_grpc::kms::v1::TypedPlaintext;
use serde::{Deserialize, Serialize};

/// Represents a user decryption request with all necessary data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDecryptRequest {
    pub ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    pub request_validity: RequestValidity,
    pub contracts_chain_id: u64,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub signature: Bytes,
    pub public_key: Bytes,
}

/// Result of a user decryption operation
#[derive(Debug, Clone)]
pub struct UserDecryptionResult {
    /// The decrypted plaintexts
    pub plaintexts: Vec<TypedPlaintext>,
    /// Metadata about the decryption
    pub metadata: DecryptionMetadata,
}

/// Metadata about a decryption operation
#[derive(Debug, Clone)]
pub struct DecryptionMetadata {
    /// Number of handles decrypted
    pub handle_count: usize,
    /// User who requested decryption
    pub user_address: Address,
    /// Whether signatures were verified
    pub signatures_verified: bool,
}

/// A single decrypted value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedValue {
    /// The handle that was decrypted
    pub handle: String,
    /// The decrypted value
    pub value: Vec<u8>,
    /// The FHE type of the value
    pub fhe_type: i32,
}

/// Configuration for response processing
#[derive(Debug)]
pub(super) struct ResponseConfig {
    pub kms_signers: Option<Vec<String>>,
    pub user_address: Option<String>,
    pub gateway_chain_id: Option<u64>,
    pub verifying_contract_address: Option<String>,
    pub signature: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub domain: Option<String>,
    pub handle_contract_pairs: Option<Vec<CtHandleContractPair>>,
    pub json_response: Option<String>,
    pub verify_signatures: bool,
}

impl Default for ResponseConfig {
    fn default() -> Self {
        Self {
            kms_signers: None,
            user_address: None,
            gateway_chain_id: None,
            verifying_contract_address: None,
            signature: None,
            public_key: None,
            private_key: None,
            domain: Some("Decryption".to_string()),
            handle_contract_pairs: None,
            json_response: None,
            verify_signatures: false,
        }
    }
}
