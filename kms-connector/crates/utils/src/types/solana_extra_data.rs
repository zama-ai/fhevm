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
//! ‖ acl_value_key                                (32 bytes; zero only when no lineage is named)
//! ‖ proof_slot                                   (8 bytes BE; 0 for current/no-proof requests)
//! ‖ mmr_proof_len ‖ mmr_proof_bytes              (4 bytes BE length, then the verbatim proof blob)
//! ```
//!
//! Binding `public_key` here is what closes the substitution attack: an attacker cannot swap in
//! their own re-encryption key without invalidating the user's signature.
//!
//! The MMR-proof tail (`acl_value_key`, `proof_slot`, `mmr_proof_bytes`) carries a historical or
//! public confidential-value decrypt's inclusion proof. `mmr_proof_bytes` is the full transport
//! blob (a 1-byte mode prefix followed by the Borsh-encoded proof) committed **verbatim** — it is
//! NOT re-encoded or normalized here, so the sign side and the verify side hash identical bytes.

/// Domain-separation tag for the Solana user-decryption signing preimage. Versioned so a future
/// layout change forces signatures to a fresh domain. Bumped to `v2` when the MMR-proof tail
/// (`acl_value_key`, `proof_slot`, `mmr_proof_bytes`) was appended: ed25519 is non-malleable and
/// the tag is the first bytes signed, so a `v1` signature can never verify against a `v2` preimage.
/// This is the relayer-bypass security fix: an in-worker re-verification of this signature over
/// the v2 preimage (see `event_processor::solana_user_decrypt`) binds the MMR proof, the lineage
/// value key, and the proof slot to the user's identity, so a relayer cannot substitute any of
/// them after the user signs.
pub const SOLANA_USER_DECRYPT_DOMAIN_TAG: &[u8] = b"zama-solana-user-decrypt-v2";

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
    /// The lineage value key for a current/historical/public decrypt; all-zero only when no
    /// lineage is named. Flat `&[u8; 32]` (not a typed key) because this crate has no
    /// `zama-solana-acl` dependency — the kms-worker owns the proof decode.
    pub acl_value_key: &'a [u8; 32],
    /// The full MMR-proof transport blob (1-byte mode prefix ‖ Borsh proof) committed verbatim;
    /// empty for a current-ACL request. NOT re-Borsh'd here so sign and verify hash identical bytes.
    pub mmr_proof_bytes: &'a [u8],
    /// The lineage leaf_count the proof was built against (staleness marker); 0 for current-ACL.
    pub proof_slot: u64,
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
            + 16
            + 32
            + 8
            + 4
            + input.mmr_proof_bytes.len(),
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

    // MMR-proof tail. acl_value_key is fixed-width (no prefix); proof_slot is fixed 8 BE bytes;
    // mmr_proof_bytes is length-prefixed (empty = 0x00000000, unambiguous). Committed verbatim:
    // the kms-worker decodes the proof, never this crate, so sign/verify hash identical bytes.
    preimage.extend_from_slice(input.acl_value_key);
    preimage.extend_from_slice(&input.proof_slot.to_be_bytes());
    preimage.extend_from_slice(&(input.mmr_proof_bytes.len() as u32).to_be_bytes());
    preimage.extend_from_slice(input.mmr_proof_bytes);

    preimage
}

/// `extraData` version byte carrying only the KMS context id (32 bytes), no MMR proof.
pub const SOLANA_EXTRA_DATA_VERSION_CONTEXT_ONLY: u8 = 0x01;
/// `extraData` version byte carrying the KMS context id PLUS the MMR-proof tail
/// (`acl_value_key ‖ proof_slot ‖ mmr_proof_len ‖ mmr_proof_bytes`).
///
/// DEVIATION FROM THE REFERENCE DESIGN: the reference design carried `aclValueKey` / `mmrProof` /
/// `proofSlot` as typed `UserDecryptionRequestSolanaPayload` fields (RFC-021-style). Adding those
/// fields is a `gateway-contracts` Solidity + codegen change, and this workstream is scoped to
/// `kms-connector/` + `sdk/js-sdk/` only (gateway-contracts is owned by a different, in-flight
/// workstream and is not even read-only-listed here). Packing the MMR tail into the existing
/// `extraData` blob — versioned so a `v0x01` (context-only) request is unambiguous from a `v0x03`
/// (MMR-proof) request — reuses the one Solana-specific "escape hatch" field the gateway interface
/// already has, at zero gateway-contracts cost. `mmr_proof_bytes` is still committed **verbatim**
/// into the [`SOLANA_USER_DECRYPT_DOMAIN_TAG`] preimage, so the signature-binding property is
/// unaffected by this transport choice; only the origin of `acl_value_key` / `proof_slot` /
/// `mmr_proof_bytes` on the decode side differs from the reference (extraData here, typed fields
/// there). If/when `gateway-contracts` grows the typed fields, this module's preimage builder is
/// unchanged and only [`parse_solana_user_decrypt_extra_data`] (and its call site) should move.
pub const SOLANA_EXTRA_DATA_VERSION_MMR_PROOF: u8 = 0x03;

/// The Solana user-decrypt auth fields carried in `extraData`, beyond the typed gateway fields.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaUserDecryptExtraData {
    /// The 32-byte KMS context id (zero when absent).
    pub context_id: [u8; 32],
    /// The lineage value key for a current or MMR-proof decrypt; all-zero only when omitted.
    pub acl_value_key: [u8; 32],
    /// The lineage leaf_count the proof was built against; 0 for a current-ACL request.
    pub proof_slot: u64,
    /// The full MMR-proof transport blob (1-byte mode prefix ‖ Borsh proof); empty for a
    /// current-ACL request.
    pub mmr_proof_bytes: Vec<u8>,
}

/// Parses the MMR-proof-tail `extraData` format strictly:
/// `0x03 ‖ context_id(32) ‖ acl_value_key(32) ‖ proof_slot(8 BE) ‖ proof_len(4 BE) ‖ proof`.
///
/// Returns `None` unless the blob is exactly the proof-tail version and its length prefix matches
/// the full body. Public decrypt uses this strict form because a missing or malformed proof must
/// fail closed instead of silently routing to a no-proof path.
pub fn parse_solana_mmr_proof_extra_data(extra_data: &[u8]) -> Option<SolanaUserDecryptExtraData> {
    if extra_data.len() < 33 || extra_data[0] != SOLANA_EXTRA_DATA_VERSION_MMR_PROOF {
        return None;
    }
    // version(1) ‖ context_id(32) ‖ acl_value_key(32) ‖ proof_slot(8 BE) ‖ len(4 BE) ‖ proof
    if extra_data.len() < 33 + 32 + 8 + 4 {
        return None;
    }

    let mut out = SolanaUserDecryptExtraData::default();
    out.context_id.copy_from_slice(&extra_data[1..33]);

    let mut offset = 33;
    out.acl_value_key
        .copy_from_slice(&extra_data[offset..offset + 32]);
    offset += 32;
    out.proof_slot = u64::from_be_bytes(extra_data[offset..offset + 8].try_into().ok()?);
    offset += 8;
    let proof_len = u32::from_be_bytes(extra_data[offset..offset + 4].try_into().ok()?) as usize;
    offset += 4;
    if extra_data.len() != offset + proof_len {
        return None;
    }
    out.mmr_proof_bytes = extra_data[offset..].to_vec();
    Some(out)
}

/// Parses a Solana `extraData` blob per [`SOLANA_EXTRA_DATA_VERSION_CONTEXT_ONLY`] /
/// [`SOLANA_EXTRA_DATA_VERSION_MMR_PROOF`]. Unknown versions, and malformed `v0x03` bodies, decode
/// as the all-zero/empty default (context-only, no proof) — the caller's dispatch on
/// `acl_value_key == [0; 32]` then naturally routes to the current-ACL (no-proof) path, matching
/// the "absent tail" behavior of a `v0x01` blob. This function is intentionally infallible: a
/// malformed extraData tail must never crash request processing, only fail to grant a proof-gated
/// decrypt (a fail-closed, not fail-open, outcome — the current-ACL path has its own membership
/// check).
pub fn parse_solana_user_decrypt_extra_data(extra_data: &[u8]) -> SolanaUserDecryptExtraData {
    let mut out = SolanaUserDecryptExtraData::default();
    if extra_data.len() < 33 {
        return out;
    }
    out.context_id.copy_from_slice(&extra_data[1..33]);
    if extra_data[0] != SOLANA_EXTRA_DATA_VERSION_MMR_PROOF {
        return out;
    }
    parse_solana_mmr_proof_extra_data(extra_data).unwrap_or_default()
}

/// Encodes a context-only (`v0x01`) `extraData` blob.
pub fn encode_solana_extra_data_context_only(context_id: [u8; 32]) -> Vec<u8> {
    let mut data = vec![SOLANA_EXTRA_DATA_VERSION_CONTEXT_ONLY];
    data.extend_from_slice(&context_id);
    data
}

/// Encodes an MMR-proof-tail (`v0x03`) `extraData` blob.
pub fn encode_solana_extra_data_mmr_proof(
    context_id: [u8; 32],
    acl_value_key: [u8; 32],
    proof_slot: u64,
    mmr_proof_bytes: &[u8],
) -> Vec<u8> {
    let mut data = vec![SOLANA_EXTRA_DATA_VERSION_MMR_PROOF];
    data.extend_from_slice(&context_id);
    data.extend_from_slice(&acl_value_key);
    data.extend_from_slice(&proof_slot.to_be_bytes());
    data.extend_from_slice(&(mmr_proof_bytes.len() as u32).to_be_bytes());
    data.extend_from_slice(mmr_proof_bytes);
    data
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
            acl_value_key: &[0u8; 32],
            mmr_proof_bytes: &[],
            proof_slot: 0,
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
            acl_value_key: &[0u8; 32],
            mmr_proof_bytes: &[],
            proof_slot: 0,
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

    fn base_input<'a>(
        identity: &'a [u8; 32],
        context_id: &'a [u8; 32],
        nonce: &'a [u8; 32],
        acl_value_key: &'a [u8; 32],
        mmr_proof_bytes: &'a [u8],
        proof_slot: u64,
    ) -> SolanaUserDecryptSigningInput<'a> {
        SolanaUserDecryptSigningInput {
            contracts_chain_id: 1,
            public_key: b"pk",
            handles: &[],
            identity,
            context_id,
            nonce,
            allowed_acl_domain_keys: &[],
            start_timestamp: 0,
            duration_seconds: 0,
            acl_value_key,
            mmr_proof_bytes,
            proof_slot,
        }
    }

    // The fixed-width acl_value_key + the 4-byte length prefix on mmr_proof_bytes prevent a tail
    // collision: a 1-byte proof with an empty value_key cannot hash the same as an empty proof
    // with a value_key whose bytes overlap the proof byte.
    #[test]
    fn tail_avoids_length_extension_collisions() {
        let id = [7u8; 32];
        let ctx = [0u8; 32];
        let nonce = [9u8; 32];

        let proof_one_byte = base_input(&id, &ctx, &nonce, &[0u8; 32], &[0xab], 0);

        let mut value_key = [0u8; 32];
        value_key[0] = 0xab;
        let empty_proof = base_input(&id, &ctx, &nonce, &value_key, &[], 0);

        assert_ne!(
            solana_user_decrypt_signing_preimage(&proof_one_byte),
            solana_user_decrypt_signing_preimage(&empty_proof),
        );
    }

    // The new MMR-proof fields are load-bearing in the preimage: changing the proof_slot, the
    // proof bytes, or the value_key changes the signed bytes (so a mutated request fails verify).
    #[test]
    fn tail_fields_bind_into_preimage() {
        let id = [7u8; 32];
        let ctx = [0u8; 32];
        let nonce = [9u8; 32];
        let proof = [0x01u8, 0x02, 0x03];
        let value_key = [0x55u8; 32];

        let base = base_input(&id, &ctx, &nonce, &value_key, &proof, 42);
        let baseline = solana_user_decrypt_signing_preimage(&base);

        let mut diff_slot = base;
        diff_slot.proof_slot = 43;
        assert_ne!(baseline, solana_user_decrypt_signing_preimage(&diff_slot));

        let other_proof = [0x01u8, 0x02, 0x04];
        let mut diff_proof = base;
        diff_proof.mmr_proof_bytes = &other_proof;
        assert_ne!(baseline, solana_user_decrypt_signing_preimage(&diff_proof));

        let other_key = [0x66u8; 32];
        let mut diff_key = base;
        diff_key.acl_value_key = &other_key;
        assert_ne!(baseline, solana_user_decrypt_signing_preimage(&diff_key));
    }

    #[test]
    fn extra_data_context_only_round_trips() {
        let ctx = [7u8; 32];
        let blob = encode_solana_extra_data_context_only(ctx);
        let parsed = parse_solana_user_decrypt_extra_data(&blob);
        assert_eq!(parsed.context_id, ctx);
        assert_eq!(parsed.acl_value_key, [0u8; 32]);
        assert_eq!(parsed.proof_slot, 0);
        assert!(parsed.mmr_proof_bytes.is_empty());
    }

    #[test]
    fn extra_data_mmr_proof_round_trips() {
        let ctx = [7u8; 32];
        let value_key = [9u8; 32];
        let proof = vec![0x01u8, 0x02, 0x03];
        let blob = encode_solana_extra_data_mmr_proof(ctx, value_key, 42, &proof);
        let parsed = parse_solana_user_decrypt_extra_data(&blob);
        assert_eq!(parsed.context_id, ctx);
        assert_eq!(parsed.acl_value_key, value_key);
        assert_eq!(parsed.proof_slot, 42);
        assert_eq!(parsed.mmr_proof_bytes, proof);
    }

    #[test]
    fn strict_mmr_proof_extra_data_requires_v3_and_exact_length() {
        let ctx = [7u8; 32];
        let value_key = [9u8; 32];
        let proof = vec![0x01u8, 0x02, 0x03];
        let blob = encode_solana_extra_data_mmr_proof(ctx, value_key, 42, &proof);

        let parsed = parse_solana_mmr_proof_extra_data(&blob).unwrap();
        assert_eq!(parsed.context_id, ctx);
        assert_eq!(parsed.acl_value_key, value_key);
        assert_eq!(parsed.proof_slot, 42);
        assert_eq!(parsed.mmr_proof_bytes, proof);

        assert!(parse_solana_mmr_proof_extra_data(&[]).is_none());
        assert!(
            parse_solana_mmr_proof_extra_data(&encode_solana_extra_data_context_only(ctx))
                .is_none()
        );

        let mut trailing = blob.clone();
        trailing.push(0);
        assert!(parse_solana_mmr_proof_extra_data(&trailing).is_none());

        assert!(parse_solana_mmr_proof_extra_data(&blob[..blob.len() - 1]).is_none());
    }

    #[test]
    fn extra_data_malformed_or_unknown_version_decodes_as_default() {
        assert_eq!(
            parse_solana_user_decrypt_extra_data(&[]),
            SolanaUserDecryptExtraData::default()
        );
        // Unknown version byte: context_id still parsed, no MMR tail.
        let mut unknown = vec![0x09u8];
        unknown.extend_from_slice(&[1u8; 32]);
        let parsed = parse_solana_user_decrypt_extra_data(&unknown);
        assert_eq!(parsed.context_id, [1u8; 32]);
        assert!(parsed.mmr_proof_bytes.is_empty());
        // v0x03 with a truncated tail: falls back to all-default (not even context_id), fail-closed.
        let mut truncated = vec![SOLANA_EXTRA_DATA_VERSION_MMR_PROOF];
        truncated.extend_from_slice(&[2u8; 32]);
        truncated.extend_from_slice(&[0u8; 4]); // way short of value_key+slot+len
        assert_eq!(
            parse_solana_user_decrypt_extra_data(&truncated),
            SolanaUserDecryptExtraData::default()
        );
        // v0x03 with a proof-length lie: rejected, not truncated-read.
        let mut lied = vec![SOLANA_EXTRA_DATA_VERSION_MMR_PROOF];
        lied.extend_from_slice(&[3u8; 32]); // context_id
        lied.extend_from_slice(&[4u8; 32]); // acl_value_key
        lied.extend_from_slice(&5u64.to_be_bytes()); // proof_slot
        lied.extend_from_slice(&100u32.to_be_bytes()); // claims 100 bytes of proof
        lied.extend_from_slice(&[0xffu8; 3]); // only 3 actually present
        assert_eq!(
            parse_solana_user_decrypt_extra_data(&lied),
            SolanaUserDecryptExtraData::default()
        );
    }
}
