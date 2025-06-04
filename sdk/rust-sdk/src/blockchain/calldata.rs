//! Calldata module for FHEVM SDK

use crate::Result;
use crate::blockchain::bindings::Decryption::userDecryptionRequestCall;
use crate::decryption::user::UserDecryptRequest;
use alloy::primitives::{Bytes, U256};
use alloy::sol_types::SolCall;
use log::info;

/// Function selectors (first 4 bytes of keccak256 hash of function signature)
pub mod selectors {
    /// User decrypt function selector (dummy value)
    pub const USER_DECRYPT: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

    /// Delegated decrypt function selector (dummy value)
    pub const DELEGATED_DECRYPT: [u8; 4] = [0x23, 0x45, 0x67, 0x89];

    /// Public decrypt function selector (dummy value)
    pub const PUBLIC_DECRYPT: [u8; 4] = [0x34, 0x56, 0x78, 0x9A];

    /// Input function selector (dummy value)
    pub const INPUT: [u8; 4] = [0x45, 0x67, 0x89, 0xAB];
}

/// Generate calldata for user decryptÒ
pub fn user_decryption_req(user_decrypt_request: UserDecryptRequest) -> Result<Bytes> {
    info!("Generating user decryption request calldata");

    // Create the userDecryptionRequest call
    let call = userDecryptionRequestCall::new((
        user_decrypt_request.ct_handle_contract_pairs,
        user_decrypt_request.request_validity,
        U256::from(user_decrypt_request.contracts_chain_id),
        user_decrypt_request.contract_addresses,
        user_decrypt_request.user_address,
        user_decrypt_request.public_key,
        user_decrypt_request.signature,
    ));

    // Encode the call to get the calldata
    let calldata = userDecryptionRequestCall::abi_encode(&call);

    Ok(Bytes::from(calldata))
}

/// Generate calldata for delegated decrypt
pub fn generate_delegated_decrypt(
    _ct_handles: &[Vec<u8>],
    _user_address: &str,
    _delegate_address: &str,
    _chain_id: u64,
) -> Result<Vec<u8>> {
    // Simple placeholder implementation
    let mut calldata = Vec::new();
    calldata.extend_from_slice(&selectors::DELEGATED_DECRYPT);

    Ok(calldata)
}

/// Generate calldata for public decrypt
pub fn generate_public_decrypt(_ct_handles: &[Vec<u8>], _chain_id: u64) -> Result<Vec<u8>> {
    // Simple placeholder implementation
    let mut calldata = Vec::new();
    calldata.extend_from_slice(&selectors::PUBLIC_DECRYPT);

    Ok(calldata)
}

/// Generate calldata for input
pub fn generate_input(_ciphertext: &[u8], _proof: &[u8]) -> Result<Vec<u8>> {
    // Simple placeholder implementation
    let mut calldata = Vec::new();
    calldata.extend_from_slice(&selectors::INPUT);

    Ok(calldata)
}
