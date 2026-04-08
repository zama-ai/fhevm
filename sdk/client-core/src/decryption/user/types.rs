use alloy::primitives::{Address, Bytes};
use fhevm_gateway_bindings::decryption::{
    Decryption::CtHandleContractPair, IDecryption::RequestValidity,
};
use serde::{Deserialize, Serialize};

/// Represents a user decryption request with all necessary data.
///
/// Constructed via [`UserDecryptRequestBuilder`](super::UserDecryptRequestBuilder).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDecryptRequest {
    pub ct_handle_contract_pairs: Vec<CtHandleContractPair>,
    pub request_validity: RequestValidity,
    pub contract_addresses: Vec<Address>,
    pub user_address: Address,
    pub signature: Bytes,
    pub public_key: Bytes,
}

/// A single decrypted value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedValue {
    pub handle: String,
    pub value: Vec<u8>,
    pub fhe_type: i32,
}

/// Configuration for response processing (builder internal state).
#[derive(Debug)]
pub(crate) struct ResponseConfig {
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
