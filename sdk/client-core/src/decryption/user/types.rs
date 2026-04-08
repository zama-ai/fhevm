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
