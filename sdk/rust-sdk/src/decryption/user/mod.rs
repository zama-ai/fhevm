//! User decryption module for FHEVM SDK
//!
//! Re-exports platform-agnostic decryption types and builders from
//! [`fhevm_client_core`] and adds SDK-specific result types.

mod types;

// Re-export core types and builders
pub use fhevm_client_core::decryption::user::{
    DecryptedValue, UserDecryptRequest, UserDecryptRequestBuilder,
    UserDecryptionResponseBuilder, process_user_decryption_response,
};

// Re-export SDK-specific types
pub use self::types::UserDecryptionResult;

// Re-export convenience function at module level
pub use fhevm_client_core::decryption::user::process_user_decryption_response as user_decrypt_response;
