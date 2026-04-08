//! Signature verification utilities for EIP-712.

use crate::{ClientCoreError, Result};
use alloy::primitives::{Address, B256};

/// Verify an EIP-712 signature against an expected signer.
pub fn verify_signature(signature: &[u8], hash: B256, expected_signer: Address) -> Result<bool> {
    let recovered = recover_signer(signature, hash)?;
    Ok(recovered == expected_signer)
}

/// Recover the signer address from an EIP-712 signature.
pub fn recover_signer(signature: &[u8], hash: B256) -> Result<Address> {
    use alloy::primitives::Signature;

    let sig = Signature::from_raw(signature)
        .map_err(|e| ClientCoreError::SignatureError(format!("Invalid signature: {e}")))?;

    let recovered = sig
        .recover_address_from_prehash(&hash)
        .map_err(|e| ClientCoreError::SignatureError(format!("Failed to recover address: {e}")))?;

    Ok(recovered)
}
