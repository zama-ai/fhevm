//! Canonical Solana user-decryption ed25519 signing preimage.
//!
//! On EVM, the Gateway verifies on-chain the EIP-712 signature that binds the re-encryption
//! `publicKey` to the requesting `userAddress`, so the relayer is untrusted. The Solana host has no
//! such on-chain binding, so each KMS party's connector MUST re-derive and verify the user's
//! ed25519 signature itself. This module is the single source of truth for the exact bytes the
//! user's ed25519 key signs (the "signing preimage").
//!
//! The auth fields (identity, nonce, allowed ACL domain keys, context) travel as TYPED gateway
//! fields (RFC-021) — there is no `extraData` blob. The relayer, the client SDK, and kms-core must
//! reproduce this preimage byte-for-byte. It is pure (no I/O, no crypto) so it can be shared and
//! unit-tested in isolation; the ed25519 verification lives in the `kms-worker` connector, which
//! owns the vetted crypto dependency.
//!
//! # Signing preimage
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

/// Domain-separation tag for the Solana user-decryption signing preimage. Versioned so a future
/// layout change forces signatures to a fresh domain.
pub const SOLANA_USER_DECRYPT_DOMAIN_TAG: &[u8] = b"zama-solana-user-decrypt-v1";

/// Length of a Solana ed25519 public key / ACL domain key / nonce, in bytes.
pub const SOLANA_PUBKEY_LEN: usize = 32;
/// Length of a ciphertext handle, in bytes.
pub const HANDLE_LEN: usize = 32;
/// Length of an ed25519 signature, in bytes.
pub const ED25519_SIGNATURE_LEN: usize = 64;

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
    /// Per-request nonce bound into the signed preimage (not dedup-enforced; replay is bounded by
    /// the validity window, matching EVM).
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

#[cfg(test)]
mod tests {
    use super::*;

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
