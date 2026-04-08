//! Signature module for FHEVM SDK
//!
//! Re-exports platform-agnostic signature types and ML-KEM keypair generation
//! from [`fhevm_client_core`] and adds SDK-specific signing utilities.

use crate::{FhevmError, Result};
use alloy::primitives::{B256, Bytes};

// Sub-modules
pub mod eip712;

// Re-export core types and functions
pub use fhevm_client_core::signature::{
    Eip712Config, Eip712Result, Eip712SignatureBuilder, Keypair, derive_address_from_private_key,
    generate_keypair, validate_private_key_format,
};

/// Sign an EIP-712 hash with a private key
///
/// Signs the provided hash using ECDSA with the given private key
pub(crate) fn sign_eip712_hash(hash: B256, private_key: &str) -> Result<Bytes> {
    use alloy::signers::{Signer, local::PrivateKeySigner};
    use std::str::FromStr;

    // Parse the private key (remove 0x prefix if present)
    let private_key_str = private_key.strip_prefix("0x").unwrap_or(private_key);

    // Create the signer
    let signer = PrivateKeySigner::from_str(private_key_str)
        .map_err(|e| FhevmError::SignatureError(format!("Invalid private key: {e}")))?;

    // Try to use existing runtime, fallback to blocking if needed
    let signature = if let Ok(handle) = tokio::runtime::Handle::try_current() {
        // We're already in a tokio runtime
        handle.block_on(async { signer.sign_hash(&hash).await })
    } else {
        // No runtime exists, create a minimal one
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| FhevmError::SignatureError(format!("Failed to create runtime: {e}")))?;

        rt.block_on(async { signer.sign_hash(&hash).await })
    };

    let signature =
        signature.map_err(|e| FhevmError::SignatureError(format!("Failed to sign: {e}")))?;

    Ok(Bytes::from(signature.as_bytes().to_vec()))
}
