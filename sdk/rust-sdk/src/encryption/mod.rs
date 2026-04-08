//! Encryption module for FHEVM SDK
//!
//! Re-exports platform-agnostic encryption types from [`fhevm_client_core`]
//! and adds SDK-specific functionality (key loading from filesystem, proof verification).

// Re-export all core encryption types
pub use fhevm_client_core::encryption::{
    CIPHERTEXT_VERSION, IntoU256,
    input::{EncryptedInput, EncryptedInputBuilder, InputBuilderFactory},
    primitives::EncryptionType,
};

// SDK-specific sub-modules
pub mod primitives;

// Re-export the input module path for backward compatibility
pub mod input {
    pub use fhevm_client_core::encryption::input::*;

    use crate::Result;

    /// Legacy function to maintain backward compatibility
    /// This will create a default keys directory if it doesn't exist
    pub fn get_default_encryption_parameters() -> Result<(
        tfhe::CompactPublicKey,
        tfhe::ClientKey,
        tfhe::ServerKey,
        tfhe::zk::CompactPkeCrs,
    )> {
        let default_path = std::path::PathBuf::from("./keys");
        super::primitives::create_encryption_parameters(&default_path)
    }
}
