//! Canonical Solana user-decryption `extraData` layout and ed25519 signing preimage.
//!
//! On EVM, the Gateway verifies on-chain the EIP-712 signature that binds the re-encryption
//! `publicKey` to the requesting `userAddress`, so the relayer is untrusted. The Solana host
//! has no such on-chain binding, so each KMS party's connector MUST re-derive and verify the
//! user's ed25519 signature itself. This module is the single source of truth for:
//!
//! 1. the byte layout of the Solana `extraData` blob carried in
//!    `UserDecryptionRequestPayload.extraData`, and
//! 2. the exact bytes the user's ed25519 key must sign (the "signing preimage").
//!
//! The relayer, the client SDK, and kms-core must reproduce this byte-for-byte. Everything is
//! pure (`no I/O`, `no crypto`) so it can be shared and unit-tested in isolation; the actual
//! ed25519 verification lives in the `kms-worker` connector, which owns the vetted crypto
//! dependency.
//!
//! # `extraData` layout (version `0x03`, [`EXTRA_DATA_SOLANA_V1_VERSION`])
//!
//! All multi-byte integers are big-endian. The `version ‖ context_id` prefix is shared with the
//! EVM v1/v2 layouts so the generic [`super::extra_data::parse_extra_data`] dispatcher can
//! surface `context_id` without knowing the Solana-specific tail.
//!
//! ```text
//! offset  size            field
//! 0       1               version (0x03)
//! 1       32              context_id (big-endian U256)
//! 33      32              ed25519 identity public key
//! 65      32              per-request nonce (anti-replay)
//! 97      4               domain_key_count = N (big-endian u32)
//! 101     32 * N          allowed ACL domain keys (each a 32-byte Solana pubkey)
//! ```
//!
//! The allowed ACL domain keys are the Solana analog of the EVM `allowedContracts` scope: they
//! enumerate the on-chain ACL domains the user authorizes this decryption for. They live in the
//! signed `extraData` (not in the EVM `allowedContracts` field, which cannot hold 32-byte
//! pubkeys) so that the scope itself is committed to by the ed25519 signature.
//!
//! # Signing preimage
//!
//! The ed25519 signature in `UserDecryptionRequestPayload.signature` MUST cover
//! [`solana_user_decrypt_signing_preimage`], whose layout is:
//!
//! ```text
//! SOLANA_USER_DECRYPT_DOMAIN_TAG                 (constant ASCII tag)
//! ‖ contracts_chain_id                           (8 bytes BE)
//! ‖ public_key_len ‖ public_key                  (4 bytes BE length, then the re-encryption key)
//! ‖ handle_count ‖ handle[0] ‖ ... ‖ handle[k-1] (4 bytes BE count, then 32 bytes each)
//! ‖ identity                                     (32 bytes)
//! ‖ context_id                                   (32 bytes BE; zero when no explicit context)
//! ‖ nonce                                        (32 bytes)
//! ‖ domain_key_count ‖ key[0] ‖ ... ‖ key[N-1]   (4 bytes BE count, then 32 bytes each)
//! ‖ start_timestamp                              (8 bytes BE)
//! ‖ duration_seconds                             (8 bytes BE)
//! ```
//!
//! Binding `public_key` here is what closes the substitution attack: an attacker cannot swap in
//! their own re-encryption key without invalidating the user's signature.

use crate::types::extra_data::EXTRA_DATA_SOLANA_V1_VERSION;
use anyhow::{anyhow, bail};

/// Domain-separation tag for the Solana user-decryption signing preimage. Versioned so a future
/// layout change forces signatures to a fresh domain.
pub const SOLANA_USER_DECRYPT_DOMAIN_TAG: &[u8] = b"zama-solana-user-decrypt-v1";

/// Length of a Solana ed25519 public key / ACL domain key / nonce, in bytes.
pub const SOLANA_PUBKEY_LEN: usize = 32;
/// Length of a ciphertext handle, in bytes.
pub const HANDLE_LEN: usize = 32;
/// Length of an ed25519 signature, in bytes.
pub const ED25519_SIGNATURE_LEN: usize = 64;

const CONTEXT_ID_OFFSET: usize = 1;
const IDENTITY_OFFSET: usize = CONTEXT_ID_OFFSET + 32;
const NONCE_OFFSET: usize = IDENTITY_OFFSET + SOLANA_PUBKEY_LEN;
const DOMAIN_KEY_COUNT_OFFSET: usize = NONCE_OFFSET + SOLANA_PUBKEY_LEN;
const DOMAIN_KEYS_OFFSET: usize = DOMAIN_KEY_COUNT_OFFSET + 4;

/// The minimum length of a Solana `extraData` blob (header with zero domain keys).
pub const SOLANA_EXTRA_DATA_MIN_LEN: usize = DOMAIN_KEYS_OFFSET;

/// Decoded Solana `extraData` contents (everything beyond the shared `context_id`, which the
/// generic parser already exposes).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaExtraData {
    /// 32-byte big-endian context id (zero when no explicit context was supplied).
    pub context_id: [u8; 32],
    /// The user's 32-byte ed25519 identity public key.
    pub identity: [u8; SOLANA_PUBKEY_LEN],
    /// Per-request anti-replay nonce.
    pub nonce: [u8; SOLANA_PUBKEY_LEN],
    /// The allowed ACL domain keys (Solana analog of EVM `allowedContracts`).
    pub allowed_acl_domain_keys: Vec<[u8; SOLANA_PUBKEY_LEN]>,
}

/// Encodes [`SolanaExtraData`] into the canonical `extraData` byte layout (version `0x03`).
pub fn encode_solana_extra_data(data: &SolanaExtraData) -> anyhow::Result<Vec<u8>> {
    let count = u32::try_from(data.allowed_acl_domain_keys.len())
        .map_err(|_| anyhow!("too many allowed ACL domain keys"))?;

    let mut out = Vec::with_capacity(DOMAIN_KEYS_OFFSET + data.allowed_acl_domain_keys.len() * 32);
    out.push(EXTRA_DATA_SOLANA_V1_VERSION);
    out.extend_from_slice(&data.context_id);
    out.extend_from_slice(&data.identity);
    out.extend_from_slice(&data.nonce);
    out.extend_from_slice(&count.to_be_bytes());
    for key in &data.allowed_acl_domain_keys {
        out.extend_from_slice(key);
    }
    Ok(out)
}

/// Decodes the canonical Solana `extraData` byte layout. Rejects a wrong version byte, a short
/// buffer, a domain-key count that overflows the buffer, and any trailing bytes (the encoding is
/// exact, so a length mismatch signals a malformed or tampered blob).
pub fn decode_solana_extra_data(bytes: &[u8]) -> anyhow::Result<SolanaExtraData> {
    if bytes.len() < SOLANA_EXTRA_DATA_MIN_LEN {
        bail!(
            "solana extra_data too short: {} bytes, expected at least {}",
            bytes.len(),
            SOLANA_EXTRA_DATA_MIN_LEN
        );
    }
    if bytes[0] != EXTRA_DATA_SOLANA_V1_VERSION {
        bail!(
            "unexpected solana extra_data version: 0x{:02x}, expected 0x{:02x}",
            bytes[0],
            EXTRA_DATA_SOLANA_V1_VERSION
        );
    }

    let context_id = read_array::<32>(bytes, CONTEXT_ID_OFFSET);
    let identity = read_array::<SOLANA_PUBKEY_LEN>(bytes, IDENTITY_OFFSET);
    let nonce = read_array::<SOLANA_PUBKEY_LEN>(bytes, NONCE_OFFSET);

    let count = u32::from_be_bytes(read_array::<4>(bytes, DOMAIN_KEY_COUNT_OFFSET)) as usize;
    let expected_len = DOMAIN_KEYS_OFFSET
        .checked_add(count.checked_mul(SOLANA_PUBKEY_LEN).ok_or_else(|| {
            anyhow!("solana extra_data domain-key count overflows: {count}")
        })?)
        .ok_or_else(|| anyhow!("solana extra_data length overflows"))?;
    if bytes.len() != expected_len {
        bail!(
            "solana extra_data length mismatch: {} bytes, expected exactly {} for {} domain keys",
            bytes.len(),
            expected_len,
            count
        );
    }

    let allowed_acl_domain_keys = (0..count)
        .map(|i| read_array::<SOLANA_PUBKEY_LEN>(bytes, DOMAIN_KEYS_OFFSET + i * SOLANA_PUBKEY_LEN))
        .collect();

    Ok(SolanaExtraData {
        context_id,
        identity,
        nonce,
        allowed_acl_domain_keys,
    })
}

/// Fields of a Solana user-decryption request that the ed25519 signature must commit to. Held by
/// reference so callers build the preimage without copying the (potentially large) handle list or
/// public key.
#[derive(Clone, Copy, Debug)]
pub struct SolanaUserDecryptSigningInput<'a> {
    /// The host chain id the handles belong to.
    pub contracts_chain_id: u64,
    /// The re-encryption public key the plaintext will be sealed to. Binding this is the core of
    /// the fix: a substituted key cannot reuse the user's signature.
    pub public_key: &'a [u8],
    /// The requested handles, each 32 bytes.
    pub handles: &'a [[u8; HANDLE_LEN]],
    /// The user's ed25519 identity public key.
    pub identity: &'a [u8; SOLANA_PUBKEY_LEN],
    /// The 32-byte big-endian context id (zero when none).
    pub context_id: &'a [u8; 32],
    /// Per-request anti-replay nonce.
    pub nonce: &'a [u8; SOLANA_PUBKEY_LEN],
    /// The authorized ACL domain keys (the signed `allowedContracts` scope).
    pub allowed_acl_domain_keys: &'a [[u8; SOLANA_PUBKEY_LEN]],
    /// Validity window start (unix seconds).
    pub start_timestamp: u64,
    /// Validity window duration (seconds).
    pub duration_seconds: u64,
}

/// Builds the exact bytes the user's ed25519 key must sign. See the module docs for the layout.
pub fn solana_user_decrypt_signing_preimage(input: &SolanaUserDecryptSigningInput<'_>) -> Vec<u8> {
    let mut preimage = Vec::with_capacity(
        SOLANA_USER_DECRYPT_DOMAIN_TAG.len()
            + 8
            + 4
            + input.public_key.len()
            + 4
            + input.handles.len() * HANDLE_LEN
            + SOLANA_PUBKEY_LEN
            + 32
            + SOLANA_PUBKEY_LEN
            + 4
            + input.allowed_acl_domain_keys.len() * SOLANA_PUBKEY_LEN
            + 16,
    );

    preimage.extend_from_slice(SOLANA_USER_DECRYPT_DOMAIN_TAG);
    preimage.extend_from_slice(&input.contracts_chain_id.to_be_bytes());

    // Length-prefix variable-length fields so distinct (publicKey, handles) splits can never
    // collide into the same preimage.
    preimage.extend_from_slice(&(input.public_key.len() as u32).to_be_bytes());
    preimage.extend_from_slice(input.public_key);

    preimage.extend_from_slice(&(input.handles.len() as u32).to_be_bytes());
    for handle in input.handles {
        preimage.extend_from_slice(handle);
    }

    preimage.extend_from_slice(input.identity);
    preimage.extend_from_slice(input.context_id);
    preimage.extend_from_slice(input.nonce);

    preimage.extend_from_slice(&(input.allowed_acl_domain_keys.len() as u32).to_be_bytes());
    for key in input.allowed_acl_domain_keys {
        preimage.extend_from_slice(key);
    }

    preimage.extend_from_slice(&input.start_timestamp.to_be_bytes());
    preimage.extend_from_slice(&input.duration_seconds.to_be_bytes());

    preimage
}

/// Reads a fixed-size array from `bytes` at `offset`. Callers guarantee the slice is long enough
/// (length is validated up front in [`decode_solana_extra_data`]).
fn read_array<const N: usize>(bytes: &[u8], offset: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes[offset..offset + N]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::extra_data::{ExtraData, parse_extra_data};
    use alloy::primitives::U256;

    fn sample() -> SolanaExtraData {
        SolanaExtraData {
            context_id: U256::from(0x1234u64).to_be_bytes(),
            identity: [7u8; 32],
            nonce: [9u8; 32],
            allowed_acl_domain_keys: vec![[1u8; 32], [2u8; 32]],
        }
    }

    #[test]
    fn extra_data_round_trips() {
        let data = sample();
        let encoded = encode_solana_extra_data(&data).unwrap();
        assert_eq!(decode_solana_extra_data(&encoded).unwrap(), data);
    }

    #[test]
    fn extra_data_round_trips_with_no_domain_keys() {
        let mut data = sample();
        data.allowed_acl_domain_keys.clear();
        let encoded = encode_solana_extra_data(&data).unwrap();
        assert_eq!(encoded.len(), SOLANA_EXTRA_DATA_MIN_LEN);
        assert_eq!(decode_solana_extra_data(&encoded).unwrap(), data);
    }

    #[test]
    fn generic_parser_surfaces_context_id() {
        let data = sample();
        let encoded = encode_solana_extra_data(&data).unwrap();
        assert_eq!(
            parse_extra_data(&encoded).unwrap(),
            ExtraData {
                context_id: Some(U256::from(0x1234u64)),
                epoch_id: None,
            }
        );
    }

    #[test]
    fn decode_rejects_wrong_version() {
        let mut encoded = encode_solana_extra_data(&sample()).unwrap();
        encoded[0] = 0x02;
        assert!(decode_solana_extra_data(&encoded).is_err());
    }

    #[test]
    fn decode_rejects_trailing_bytes() {
        let mut encoded = encode_solana_extra_data(&sample()).unwrap();
        encoded.push(0xff);
        assert!(decode_solana_extra_data(&encoded).is_err());
    }

    #[test]
    fn decode_rejects_truncated_domain_keys() {
        let mut encoded = encode_solana_extra_data(&sample()).unwrap();
        encoded.truncate(encoded.len() - 1);
        assert!(decode_solana_extra_data(&encoded).is_err());
    }

    #[test]
    fn preimage_is_deterministic_and_binds_public_key() {
        let handles = [[3u8; HANDLE_LEN]];
        let domain_keys = [[1u8; 32]];
        let identity = [7u8; 32];
        let nonce = [9u8; 32];
        let context_id = [0u8; 32];
        let base = SolanaUserDecryptSigningInput {
            contracts_chain_id: 42,
            public_key: b"public-key-bytes",
            handles: &handles,
            identity: &identity,
            context_id: &context_id,
            nonce: &nonce,
            allowed_acl_domain_keys: &domain_keys,
            start_timestamp: 1000,
            duration_seconds: 3600,
        };

        let a = solana_user_decrypt_signing_preimage(&base);
        let b = solana_user_decrypt_signing_preimage(&base);
        assert_eq!(a, b, "preimage must be deterministic");

        let mut other = base;
        other.public_key = b"different-public";
        let c = solana_user_decrypt_signing_preimage(&other);
        assert_ne!(a, c, "preimage must change when the public key changes");
    }

    #[test]
    fn preimage_avoids_length_extension_collisions() {
        // (publicKey="ab", one handle) vs (publicKey="abXX...", zero handles) must differ thanks
        // to the explicit length prefixes.
        let identity = [7u8; 32];
        let nonce = [9u8; 32];
        let context_id = [0u8; 32];
        let handle = [0xaau8; HANDLE_LEN];

        let with_handle = SolanaUserDecryptSigningInput {
            contracts_chain_id: 1,
            public_key: b"ab",
            handles: std::slice::from_ref(&handle),
            identity: &identity,
            context_id: &context_id,
            nonce: &nonce,
            allowed_acl_domain_keys: &[],
            start_timestamp: 0,
            duration_seconds: 0,
        };
        let mut merged_key = b"ab".to_vec();
        merged_key.extend_from_slice(&handle);
        let without_handle = SolanaUserDecryptSigningInput {
            public_key: &merged_key,
            handles: &[],
            ..with_handle
        };

        assert_ne!(
            solana_user_decrypt_signing_preimage(&with_handle),
            solana_user_decrypt_signing_preimage(&without_handle),
        );
    }
}
