use alloy::primitives::{keccak256, B256};

/// Computes the Keccak-256 hash of a given string and returns it as a hex string.
pub fn keccak256_hex(input: &str) -> String {
    let hash: B256 = keccak256(input);
    format!("0x{:x}", hash)
}
