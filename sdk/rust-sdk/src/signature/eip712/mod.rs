//! EIP-712 signature generation module
//!
//! This module provides a builder pattern for creating EIP-712 typed signatures
//! for user decryption operations in the FHEVM ecosystem.

mod builder;
mod types;
mod verification;

pub use self::builder::Eip712SignatureBuilder;
pub use self::types::{Eip712Config, Eip712Result};
pub use self::verification::{recover_signer, verify_signature};

// Re-export from parent signature module for backward compatibility
pub use crate::signature::{generate_keypair, validate_private_key_format};
