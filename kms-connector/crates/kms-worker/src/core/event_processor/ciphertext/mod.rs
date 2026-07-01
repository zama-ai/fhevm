//! Ciphertext-material management for incoming decryption requests.

pub mod manager;
pub mod registry;
pub mod s3;

use alloy::primitives::U256;
pub use manager::CiphertextManager;
pub use registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot};

/// Hardcoded to `1` per RFC 023; bound by the attestation signature.
pub const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;
