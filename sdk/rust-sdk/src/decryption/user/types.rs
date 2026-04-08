use alloy::primitives::Address;
use kms_grpc::kms::v1::TypedPlaintext;

/// Result of a user decryption operation
#[derive(Debug, Clone)]
pub struct UserDecryptionResult {
    /// The decrypted plaintexts
    pub plaintexts: Vec<TypedPlaintext>,
    /// Metadata about the decryption
    pub metadata: DecryptionMetadata,
}

/// Metadata about a decryption operation
#[derive(Debug, Clone)]
pub struct DecryptionMetadata {
    /// Number of handles decrypted
    pub handle_count: usize,
    /// User who requested decryption
    pub user_address: Address,
    /// Whether signatures were verified
    pub signatures_verified: bool,
}
