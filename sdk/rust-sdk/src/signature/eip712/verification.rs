//! Signature verification utilities for EIP-712

use crate::{FhevmError, Result};
use alloy::primitives::{Address, B256};

/// Verify an EIP-712 signature
///
/// Checks if the signature was created by the expected signer for the given hash
pub fn verify_signature(signature: &[u8], hash: B256, expected_signer: Address) -> Result<bool> {
    let recovered = recover_signer(signature, hash)?;
    Ok(recovered == expected_signer)
}

/// Recover the signer address from an EIP-712 signature
///
/// Returns the address that created the signature for the given hash
pub fn recover_signer(signature: &[u8], hash: B256) -> Result<Address> {
    use alloy::primitives::Signature;

    // Parse the signature from bytes
    let sig = Signature::from_raw(signature)
        .map_err(|e| FhevmError::SignatureError(format!("Invalid signature: {}", e)))?;

    // Recover the address
    let recovered = sig
        .recover_address_from_prehash(&hash)
        .map_err(|e| FhevmError::SignatureError(format!("Failed to recover address: {}", e)))?;

    Ok(recovered)
}
