//! Ciphertext-material management for incoming decryption requests.
//!
//! On every decryption request the [`CiphertextManager`]:
//! 1. verifies the off-chain ciphertext attestations of the requested handles against the
//!    authoritative on-chain `SnsCiphertextMaterial` snapshot (shadow mode: it never changes
//!    decryption behavior, a failed verification is only logged),
//! 2. retrieves the ciphertexts from the Coprocessors' S3 buckets.
//!
//! Both steps rely on the same periodically-synced [`CoprocessorRegistry`]: the attestation
//! fan-out needs the signer set, the bucket URLs and the majority threshold, while the
//! ciphertext retrieval needs the bucket URLs.

pub mod manager;
pub mod registry;
pub mod s3;

use alloy::primitives::U256;
pub use manager::CiphertextManager;
pub use registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot};

/// Hardcoded to `1` per RFC 023; bound by the attestation signature.
pub const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;
