//! Encryption module for FHEVM client core.
//!
//! Provides encrypted input building with ZK proofs, handle computation,
//! and encryption type definitions. All operations are platform-agnostic —
//! key material is accepted as pre-loaded `Arc` references, not file paths.

use crate::{ClientCoreError, Result};

/// Trait for types that can be converted to a 256-bit value for encryption.
pub trait IntoU256 {
    fn into_u256_bytes(self) -> Result<[u8; 32]>;
}

impl IntoU256 for &[u8] {
    fn into_u256_bytes(self) -> Result<[u8; 32]> {
        if self.len() > 32 {
            return Err(ClientCoreError::EncryptionError(
                "Value exceeds 256 bits".to_string(),
            ));
        }

        let mut result = [0u8; 32];
        let start = result.len().saturating_sub(self.len());
        result[start..].copy_from_slice(&self[..self.len()]);
        Ok(result)
    }
}

impl IntoU256 for alloy::primitives::U256 {
    fn into_u256_bytes(self) -> Result<[u8; 32]> {
        Ok(self.to_be_bytes::<32>())
    }
}

/// Current ciphertext version used for handle generation.
pub const CIPHERTEXT_VERSION: u8 = 0;

pub use self::input::{EncryptedInput, EncryptedInputBuilder, InputBuilderFactory};
pub use self::primitives::EncryptionType;

pub mod input;
pub mod primitives;
