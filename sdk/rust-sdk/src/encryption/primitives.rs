//! SDK-specific encryption primitives.
//!
//! Re-exports [`EncryptionType`] from client-core and adds filesystem-dependent
//! key loading and proof verification functions.

pub use fhevm_client_core::encryption::primitives::EncryptionType;

use crate::{FhevmError, Result, utils};
use std::path::Path;
use tfhe::zk::CompactPkeCrs;
use tfhe::{ProvenCompactCiphertextList, set_server_key};
use tracing::info;

/// Verifies and expands an encrypted boolean with its proof
///
/// # Arguments
/// * `serialized_proven_compact_list` - Serialized proven compact ciphertext list
/// * `public_key` - The compact public key used for encryption
/// * `server_key` - The server key for FHE operations
/// * `crs` - Common reference string for zero-knowledge proofs
/// * `aux_data` - Auxiliary data used during proof generation
pub fn verify_expand(
    serialized_proven_compact_list: Vec<u8>,
    public_key: &tfhe::CompactPublicKey,
    server_key: tfhe::ServerKey,
    crs: &CompactPkeCrs,
    aux_data: &[u8],
) -> Result<tfhe::CompactCiphertextListExpander> {
    set_server_key(server_key);
    let deserialized_proven_compact_list: ProvenCompactCiphertextList =
        tfhe::safe_serialization::safe_deserialize(
            serialized_proven_compact_list.as_slice(),
            1 << 20,
        )
        .map_err(|e| FhevmError::DecryptionError(format!("Failed to deserialize: {e}")))?;
    let expander = deserialized_proven_compact_list
        .verify_and_expand(crs, public_key, aux_data)
        .map_err(|e| FhevmError::DecryptionError(format!("Failed to verify: {e}")))?;
    Ok(expander)
}

/// Creates or loads encryption parameters
///
/// If the keys directory exists at `keys_path`, it will load the keys from there.
/// Otherwise, it will generate new keys and save them to the specified path.
///
/// # Arguments
///
/// * `keys_path` - Path where keys should be loaded from or saved to
///
/// # Returns
///
/// Returns a tuple of (public_key, client_key, server_key, crs) if successful
pub fn create_encryption_parameters(
    keys_path: &Path,
) -> Result<(
    tfhe::CompactPublicKey,
    tfhe::ClientKey,
    tfhe::ServerKey,
    tfhe::zk::CompactPkeCrs,
)> {
    if keys_path.exists()
        && keys_path.join("public_key.bin").exists()
        && keys_path.join("client_key.bin").exists()
        && keys_path.join("server_key.bin").exists()
        && keys_path.join("crs.bin").exists()
    {
        info!("Loading existing keys from: {}", keys_path.display());
        return utils::load_fhe_keyset(keys_path);
    }

    info!("Generating new keys and saving to: {}", keys_path.display());

    if !keys_path.exists() {
        std::fs::create_dir_all(keys_path)
            .map_err(|e| FhevmError::FileError(format!("Failed to create directory: {e}")))?;
    }

    utils::generate_fhe_keyset(keys_path)?;
    utils::load_fhe_keyset(keys_path)
}
