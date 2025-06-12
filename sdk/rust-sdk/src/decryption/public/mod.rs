//! Public decryption module for FHEVM SDK
//!
//! This module provides public decryption functionality following the same
//! builder pattern as user decryption for consistency and ease of use.

// Internal modules
mod builder;
mod deserializer;
mod response;
mod types;
mod verification;

// Re-export main types and functions
pub use self::builder::{PublicDecryptRequest, PublicDecryptRequestBuilder};
pub use self::response::{PublicDecryptionResponseBuilder, process_public_decryption_response};
pub use self::types::{DecryptedResults, PublicDecryptionResponse, PublicDecryptionResult};

// Re-export convenience function at module level
pub use self::response::process_public_decryption_response as public_decrypt_response;
