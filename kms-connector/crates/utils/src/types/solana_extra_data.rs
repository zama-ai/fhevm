//! The Solana public-decrypt proof carrier: a versioned `extraData` container.
//!
//! Solana public decrypt has no live on-chain "is public" flag — public-ness is only provable by a
//! `PublicDecryptLeaf` MMR inclusion proof. The gateway interface has no typed fields for such a
//! proof, so it travels in this versioned `extraData` blob. This is transitional transport for one
//! flow, not protocol surface:
//!
//! - Removal condition: typed proof carriage for public decrypt — a `gateway-contracts` interface
//!   change, owned outside this workstream. Once the gateway carries the proof in typed fields,
//!   this container, its strict parser and its encoders are deleted; the container must not
//!   outlive that change.
//! - Until then it must not be torn down either: without it a Solana public decrypt fails closed —
//!   silently, for every request. `kms-worker/tests/solana_public_decrypt_carrier.rs` pins the
//!   carrier and the full public-decrypt path from outside the modules that consume it.
//!
//! The blob is pure bytes and can be shared and unit-tested in isolation; the proof decode and MMR
//! verification live in the `kms-worker` connector, which owns the vetted crypto dependency.

/// `extraData` version byte carrying only the KMS context id (32 bytes), no MMR proof.
pub const SOLANA_EXTRA_DATA_VERSION_CONTEXT_ONLY: u8 = 0x01;
/// `extraData` version byte carrying the KMS context id PLUS the MMR-proof tail
/// (`acl_value_key ‖ proof_slot ‖ mmr_proof_len ‖ mmr_proof_bytes`).
///
/// TEMPORARY PROOF CARRIER, OWNED BY PUBLIC DECRYPT — see the module docs for its status and
/// removal condition.
pub const SOLANA_EXTRA_DATA_VERSION_MMR_PROOF: u8 = 0x03;

/// The parsed Solana `extraData` carrier: a KMS context id and an optional MMR-proof tail.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SolanaExtraData {
    /// The 32-byte KMS context id (zero when absent).
    pub context_id: [u8; 32],
    /// The encrypted value ID for a current or MMR-proof decrypt; all-zero only when omitted.
    pub acl_value_key: [u8; 32],
    /// The encrypted value account leaf_count the proof was built against; 0 for a current-ACL request.
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
///
/// Part of the temporary proof carrier — see [`SOLANA_EXTRA_DATA_VERSION_MMR_PROOF`] for its
/// ownership, status and removal condition.
///
/// The client-side encoder is `buildSolanaPublicDecryptMmrProofExtraData` in
/// `sdk/js-sdk/src/solana/actions/publicDecryptCertificate.ts` — a hand-mirrored codec across
/// languages (TypeScript there, Rust here); the two layouts must change together.
pub fn parse_solana_mmr_proof_extra_data(extra_data: &[u8]) -> Option<SolanaExtraData> {
    if extra_data.len() < 33 || extra_data[0] != SOLANA_EXTRA_DATA_VERSION_MMR_PROOF {
        return None;
    }
    // version(1) ‖ context_id(32) ‖ acl_value_key(32) ‖ proof_slot(8 BE) ‖ len(4 BE) ‖ proof
    if extra_data.len() < 33 + 32 + 8 + 4 {
        return None;
    }

    let mut out = SolanaExtraData::default();
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
    fn mmr_proof_carrier_round_trips_through_the_strict_parser() {
        let ctx = [7u8; 32];
        let value_key = [9u8; 32];
        let proof = vec![0x01u8, 0x02, 0x03];
        let blob = encode_solana_extra_data_mmr_proof(ctx, value_key, 42, &proof);

        let parsed = parse_solana_mmr_proof_extra_data(&blob).expect("the carrier decodes");
        assert_eq!(parsed.context_id, ctx);
        assert_eq!(parsed.acl_value_key, value_key);
        assert_eq!(parsed.proof_slot, 42);
        assert_eq!(parsed.mmr_proof_bytes, proof);
    }

    #[test]
    fn an_empty_proof_tail_keeps_the_acl_value_key() {
        let ctx = [7u8; 32];
        let value_key = [9u8; 32];
        let blob = encode_solana_extra_data_mmr_proof(ctx, value_key, 0, &[]);

        let parsed = parse_solana_mmr_proof_extra_data(&blob).expect("the carrier decodes");
        assert_eq!(parsed.context_id, ctx);
        assert_eq!(parsed.acl_value_key, value_key);
        assert_eq!(parsed.proof_slot, 0);
        assert!(parsed.mmr_proof_bytes.is_empty());
    }

    #[test]
    fn the_strict_parser_requires_v3_and_an_exact_length() {
        let ctx = [7u8; 32];
        let value_key = [9u8; 32];
        let proof = vec![0x01u8, 0x02, 0x03];
        let blob = encode_solana_extra_data_mmr_proof(ctx, value_key, 42, &proof);

        // A context-only blob and an empty blob are the wrong version, not a proof tail.
        assert!(parse_solana_mmr_proof_extra_data(&[]).is_none());
        assert!(
            parse_solana_mmr_proof_extra_data(&encode_solana_extra_data_context_only(ctx))
                .is_none()
        );

        // A trailing byte and a truncated body both break the exact-length rule.
        let mut trailing = blob.clone();
        trailing.push(0);
        assert!(parse_solana_mmr_proof_extra_data(&trailing).is_none());
        assert!(parse_solana_mmr_proof_extra_data(&blob[..blob.len() - 1]).is_none());
    }

    #[test]
    fn a_proof_length_lie_is_rejected_not_truncated_read() {
        let mut lied = vec![SOLANA_EXTRA_DATA_VERSION_MMR_PROOF];
        lied.extend_from_slice(&[3u8; 32]); // context_id
        lied.extend_from_slice(&[4u8; 32]); // acl_value_key
        lied.extend_from_slice(&5u64.to_be_bytes()); // proof_slot
        lied.extend_from_slice(&100u32.to_be_bytes()); // claims 100 bytes of proof
        lied.extend_from_slice(&[0xffu8; 3]); // only 3 actually present
        assert!(parse_solana_mmr_proof_extra_data(&lied).is_none());
    }
}
