use crate::{FhevmError, Result, utils};
use std::path::Path;
use tfhe::zk::CompactPkeCrs;
use tfhe::{ProvenCompactCiphertextList, set_server_key};
use tracing::info;

/// Defines the bit width for different encryption types
#[derive(Debug, Clone, Copy)]
pub enum EncryptionType {
    Bit1,   // Boolean
    Bit8,   // uint8
    Bit16,  // uint16
    Bit32,  // uint32
    Bit64,  // uint64
    Bit128, // uint128
    Bit160, // address
    Bit256, // uint256
}

impl EncryptionType {
    /// Get the number of bits for this encryption type
    pub fn bit_width(&self) -> usize {
        match self {
            Self::Bit1 => 1,
            Self::Bit8 => 8,
            Self::Bit16 => 16,
            Self::Bit32 => 32,
            Self::Bit64 => 64,
            Self::Bit128 => 128,
            Self::Bit160 => 160,
            Self::Bit256 => 256,
        }
    }

    /// Get the discriminant value used in handle computation
    pub fn discriminant(&self) -> u8 {
        match self {
            Self::Bit1 => 0,   // ebool
            Self::Bit8 => 2,   // euint8
            Self::Bit16 => 3,  // euint16
            Self::Bit32 => 4,  // euint32
            Self::Bit64 => 5,  // euint64
            Self::Bit128 => 6, // euint128
            Self::Bit160 => 7, // eaddress
            Self::Bit256 => 8, // euint256
        }
    }

    /// Get the encryption type from a bit width
    pub fn from_bit_width(bit_width: usize) -> Result<Self> {
        match bit_width {
            1 => Ok(Self::Bit1),
            8 => Ok(Self::Bit8),
            16 => Ok(Self::Bit16),
            32 => Ok(Self::Bit32),
            64 => Ok(Self::Bit64),
            128 => Ok(Self::Bit128),
            160 => Ok(Self::Bit160),
            256 => Ok(Self::Bit256),
            _ => Err(FhevmError::InvalidParams(format!(
                "Unsupported bit width: {}",
                bit_width
            ))),
        }
    }
}

/// Verifies and expands an encrypted boolean with its proof
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
        .map_err(|e| FhevmError::DecryptionError(format!("Failed to deserialize: {}", e)))?;
    let expander = deserialized_proven_compact_list
        .verify_and_expand(crs, public_key, aux_data)
        .map_err(|e| FhevmError::DecryptionError(format!("Failed to verify: {}", e)))?;
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
    // Check if keys_path exists and has the necessary key files
    if keys_path.exists()
        && keys_path.join("public_key.bin").exists()
        && keys_path.join("client_key.bin").exists()
        && keys_path.join("server_key.bin").exists()
        && keys_path.join("crs.bin").exists()
    {
        // Load the keys from the existing directory
        info!("Loading existing keys from: {}", keys_path.display());
        return utils::load_fhe_keyset(keys_path);
    }

    // If path doesn't exist or is missing files, generate new keys
    info!("Generating new keys and saving to: {}", keys_path.display());

    // First, ensure the directory exists
    if !keys_path.exists() {
        std::fs::create_dir_all(keys_path)
            .map_err(|e| FhevmError::FileError(format!("Failed to create directory: {}", e)))?;
    }

    // Generate the keys and save them
    utils::generate_fhe_keyset(keys_path)?;

    // Load and return the newly generated keys
    utils::load_fhe_keyset(keys_path)
}
