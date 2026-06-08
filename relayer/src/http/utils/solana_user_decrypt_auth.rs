//! RFC-021 Solana user-decryption authorization — the sanctioned ed25519 `signMessage` auth seam.
//!
//! On EVM the gateway verifies a secp256k1 EIP-712 signature over the user-decryption request
//! (`ecrecover(digest) == userAddress`). Solana users hold ed25519 keys and authorize via
//! `signMessage`, which an EVM contract cannot `ecrecover`. The relayer therefore verifies the
//! user's ed25519 signature over a canonical, domain-separated message that binds the request,
//! before forwarding to the gateway's `userDecryptionRequestSolana`. This is the one Solana-only
//! divergence on the user-decryption request path; everything downstream is the EVM-parity flow.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::core::event::UserDecryptRequest;

/// Domain separator for the Solana user-decryption authorization message.
const SOLANA_USER_DECRYPT_AUTH_DOMAIN: &[u8] = b"fhevm-solana-user-decryption-auth-v0";

/// Builds the canonical message the user signs (and the relayer verifies) to authorize a Solana
/// user-decryption request. Binds every field that scopes the decryption: the host chain, the
/// reencryption public key, the requested ciphertext handles, and the validity window. The SDK
/// MUST construct the identical preimage when signing with the user's ed25519 key.
pub fn solana_user_decrypt_auth_message(request: &UserDecryptRequest) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.extend_from_slice(SOLANA_USER_DECRYPT_AUTH_DOMAIN);
    msg.extend_from_slice(&request.contracts_chain_id.to_le_bytes());
    msg.extend_from_slice(&(request.public_key.len() as u32).to_le_bytes());
    msg.extend_from_slice(&request.public_key);
    msg.extend_from_slice(&(request.ct_handle_contract_pairs.len() as u32).to_le_bytes());
    for pair in &request.ct_handle_contract_pairs {
        msg.extend_from_slice(&pair.ct_handle.to_be_bytes::<32>());
    }
    msg.extend_from_slice(&request.request_validity.start_timestamp.to_be_bytes::<32>());
    msg.extend_from_slice(&request.request_validity.duration_days.to_be_bytes::<32>());
    msg
}

/// Verifies the user's ed25519 `signMessage` authorization over [`solana_user_decrypt_auth_message`].
/// `pubkey` is the 32-byte Solana user identity; `signature` is the 64-byte ed25519 signature
/// carried in the request's `signature` field.
pub fn verify_solana_user_decrypt_auth(
    request: &UserDecryptRequest,
    pubkey: &[u8; 32],
    signature: &[u8],
) -> Result<(), SolanaUserDecryptAuthError> {
    let verifying_key = VerifyingKey::from_bytes(pubkey)
        .map_err(|_| SolanaUserDecryptAuthError::InvalidPublicKey)?;
    let sig_bytes: [u8; 64] = signature
        .try_into()
        .map_err(|_| SolanaUserDecryptAuthError::InvalidSignatureLength(signature.len()))?;
    let signature = Signature::from_bytes(&sig_bytes);
    let message = solana_user_decrypt_auth_message(request);
    verifying_key
        .verify(&message, &signature)
        .map_err(|_| SolanaUserDecryptAuthError::SignatureVerificationFailed)
}

/// Errors from verifying a Solana user-decryption ed25519 authorization.
#[derive(Debug, thiserror::Error)]
pub enum SolanaUserDecryptAuthError {
    #[error("invalid Solana ed25519 public key")]
    InvalidPublicKey,
    #[error("invalid ed25519 signature length: {0} (expected 64)")]
    InvalidSignatureLength(usize),
    #[error("Solana user-decryption ed25519 signature verification failed")]
    SignatureVerificationFailed,
}
