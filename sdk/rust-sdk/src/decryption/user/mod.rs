//! User decryption module for FHEVM SDK
//!
//! This module provides user decryption functionality with a clean builder pattern
//! for both request construction and response processing.

// Internal modules
mod deserializer;
mod request;
mod response;
mod types;

// Re-export main types and functions
pub use self::request::UserDecryptRequestBuilder;
pub use self::response::{UserDecryptionResponseBuilder, process_user_decryption_response};
pub use self::types::{DecryptedValue, UserDecryptRequest, UserDecryptionResult};

// Re-export convenience function at module level
pub use self::response::process_user_decryption_response as user_decrypt_response;
