//! Encryption module for FHEVM SDK
//!
//! This module provides functionalities for encrypting data using TFHE and generating zero-knowledge proofs
//! for encrypted inputs. It's designed to work with the FHEVM (Fully Homomorphic Encryption Virtual Machine).

use crate::{FhevmError, Result};
/// Trait for types that can be converted to a 256-bit value for encryption
pub trait IntoU256 {
    /// Convert this type to a byte array suitable for adding as a u256
    fn into_u256_bytes(self) -> Result<[u8; 32]>;
}

// Implement for byte slices with the validation check
impl IntoU256 for &[u8] {
    fn into_u256_bytes(self) -> Result<[u8; 32]> {
        if self.len() > 32 {
            return Err(FhevmError::EncryptionError(
                "Value exceeds 256 bits".to_string(),
            ));
        }

        let mut result = [0u8; 32];
        let start = result.len().saturating_sub(self.len());
        result[start..].copy_from_slice(&self[..self.len()]);
        Ok(result)
    }
}

// Implement for alloy U256 (which is always 32 bytes)
impl IntoU256 for alloy::primitives::U256 {
    fn into_u256_bytes(self) -> Result<[u8; 32]> {
        Ok(self.to_be_bytes::<32>())
    }
}

/// Current ciphertext version used for handle generation
pub const CIPHERTEXT_VERSION: u8 = 0;

pub use self::input::{EncryptedInput, EncryptedInputBuilder, InputBuilderFactory};
pub use self::primitives::EncryptionType;

// Sub-modules
pub mod input;
pub mod primitives;
