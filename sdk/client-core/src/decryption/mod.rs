//! Decryption modules for FHEVM client core.
//!
//! Provides request builders and response processing for both public
//! and user decryption flows. KMS-specific response processing (which
//! requires network access and KMS-specific key material) is not included
//! in this platform-agnostic crate.

pub mod public;
pub mod user;
