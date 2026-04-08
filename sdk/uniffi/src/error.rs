//! Error types for the UniFFI binding layer.

use fhevm_client_core::ClientCoreError;

/// Errors exposed to Swift / Kotlin / React Native via UniFFI.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FhevmError {
    #[error("Encryption failed: {reason}")]
    EncryptionError { reason: String },

    #[error("Signing failed: {reason}")]
    SigningError { reason: String },

    #[error("Decryption failed: {reason}")]
    DecryptionError { reason: String },

    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },

    #[error("Internal error: {reason}")]
    InternalError { reason: String },
}

impl From<ClientCoreError> for FhevmError {
    fn from(err: ClientCoreError) -> Self {
        match err {
            ClientCoreError::EncryptionError(msg) => FhevmError::EncryptionError { reason: msg },
            ClientCoreError::DecryptionError(msg) => FhevmError::DecryptionError { reason: msg },
            ClientCoreError::InvalidParams(msg) => FhevmError::InvalidInput { reason: msg },
            ClientCoreError::SignatureError(msg) => FhevmError::SigningError { reason: msg },
            ClientCoreError::KeyError(msg) => FhevmError::InvalidInput { reason: msg },
            ClientCoreError::HexError(e) => FhevmError::InvalidInput {
                reason: format!("Hex decoding error: {e}"),
            },
            ClientCoreError::AlloyParseError(e) => FhevmError::InvalidInput {
                reason: format!("Address parse error: {e}"),
            },
        }
    }
}
