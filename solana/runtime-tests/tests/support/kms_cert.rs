//! Shared KMS / secp256k1 EIP-712 certificate helpers for the Mollusk suites.
//!
//! Both `host_mollusk` (the stateless `verify_public_decrypt`) and `token_mollusk` (the
//! disclosure/redeem consumes) need to mint the same secp256k1 certificates the KMS produces, so the
//! signing key, address recovery, signature encoding, and `PublicDecryptVerification` digest live
//! here once rather than duplicated per binary.

// Test-support is compiled into every test binary; the operator suites include it without
// consuming these helpers, and CI denies dead-code warnings.
#![allow(dead_code)]

use k256::ecdsa::SigningKey;
use zama_host as host;

/// KMS signing key backing `PublicDecryptVerification` certs; its EVM address is the sole signer of
/// the fixtures' pinned KMS context.
pub fn kms_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[0x55u8; 32].into()).unwrap()
}

/// Recovers the EVM address (keccak(pubkey)[12..]) for a signing key.
pub fn secp_evm_address(key: &SigningKey) -> [u8; 20] {
    let encoded = key.verifying_key().to_encoded_point(false);
    let hash = solana_program::keccak::hash(&encoded.as_bytes()[1..]).to_bytes();
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// 65-byte `[r || s || v]` recoverable signature over an EIP-712 digest.
pub fn secp_sign(key: &SigningKey, digest: &[u8; 32]) -> [u8; 65] {
    let (signature, recovery_id) = key.sign_prehash_recoverable(digest).unwrap();
    let mut out = [0u8; 65];
    out[..64].copy_from_slice(&signature.to_bytes());
    out[64] = 27 + recovery_id.to_byte();
    out
}

/// Builds a KMS `PublicDecryptVerification` secp256k1 cert over `handle` / `cleartext` (a 32-byte
/// big-endian `uint256`), signed by [`kms_signing_key`]. `extra_data` binds the KMS context (empty /
/// `[0x00]` selects the current context; a version-1 payload commits an explicit context id).
pub fn kms_public_decrypt_cert(
    handle: [u8; 32],
    cleartext: [u8; 32],
    gateway_chain_id: u64,
    decryption_contract: &[u8; 20],
    extra_data: &[u8],
) -> Vec<[u8; 65]> {
    kms_public_decrypt_cert_signed_by(
        handle,
        cleartext,
        gateway_chain_id,
        decryption_contract,
        extra_data,
        &[kms_signing_key()],
    )
}

/// A distinct KMS signing key derived from a seed byte (for t-of-n KMS-context tests).
pub fn kms_signing_key_n(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32].into()).unwrap()
}

/// Like [`kms_public_decrypt_cert`], but produces one signature per key in `keys` — a t-of-n cert.
/// The carried signature payload scales with the threshold t (t x 65 bytes), independent of how many
/// signers are registered in the context.
pub fn kms_public_decrypt_cert_signed_by(
    handle: [u8; 32],
    cleartext: [u8; 32],
    gateway_chain_id: u64,
    decryption_contract: &[u8; 20],
    extra_data: &[u8],
    keys: &[SigningKey],
) -> Vec<[u8; 65]> {
    let digest = host::eip712::typed_data_digest(
        &host::eip712::domain_separator(b"Decryption", b"1", gateway_chain_id, decryption_contract),
        &host::eip712::public_decrypt_struct_hash(&[handle], &cleartext, extra_data),
    );
    keys.iter().map(|key| secp_sign(key, &digest)).collect()
}

/// Version-1 `extra_data` committing an explicit KMS context id in `[1..33]` (EVM `_extractContextId`
/// parity). Used to mint a cert bound to a rotated-out context id.
pub fn context_extra_data_v1(context_id: u64) -> Vec<u8> {
    let mut extra_data = vec![1u8];
    extra_data.extend_from_slice(&[0u8; 24]);
    extra_data.extend_from_slice(&context_id.to_be_bytes());
    extra_data
}

/// The 32-byte big-endian `uint256` encoding of a u64 cleartext (the KMS-signed decrypted result).
pub fn cleartext_u256(value: u64) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[24..].copy_from_slice(&value.to_be_bytes());
    out
}
