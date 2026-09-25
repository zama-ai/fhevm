//! Ciphertext-material management for incoming decryption requests.

pub mod manager;
pub mod s3;

pub use manager::CiphertextManager;

use alloy::primitives::U256;
use kms_grpc::kms::v1::TypedCiphertext;

/// The ciphertexts of a decryption request, resolved and verified off-chain.
///
/// `key_id` comes from the attestation consensus (not the event payload) and is shared by every
/// handle of the request — a request whose handles resolve to different key ids is rejected.
pub struct VerifiedCiphertexts {
    pub ciphertexts: Vec<TypedCiphertext>,
    pub key_id: U256,
}
