//! The relayer's encoder for the canonical Solana `hostPayload` bytes.
//!
//! The gateway's host-generic `userDecryptionRequest` overload carries the whole Solana
//! request as one opaque `hostPayload = 0x01 ‖ borsh(body)`; the gateway never reads a byte
//! of it. The relayer is the party that submits that calldata, so it is the party that
//! builds these bytes; each KMS party's connector decodes them.
//!
//! There is deliberately no shared codec crate. The byte layout is frozen by the normative
//! connector-auth vectors (spec §12), and `host_payload_bytes_match_the_fixture` pins this
//! encoder against the very vector the connector's decoder is pinned against — so the two
//! independent implementations cannot drift. The field order below IS the canonical layout
//! and mirrors the connector's `host_payload` module field for field.

use borsh::BorshSerialize;
use zama_solana_permit::PermitWireFields;

/// The one known `hostPayload` version byte.
pub const HOST_PAYLOAD_VERSION: u8 = 0x01;

/// The `hostKind` dispatch value for a Solana host chain, forwarded verbatim into the
/// gateway calldata (mirrors the gateway contract's `HOST_KIND_SOLANA`).
pub const HOST_KIND_SOLANA: u8 = 2;

/// One handle entry, as the relayer forwards it into the canonical payload. None of these
/// fields are signed: they are evidence, self-authenticating against host state.
pub struct SolanaHandleWire {
    /// 32-byte ciphertext handle.
    pub handle: Vec<u8>,
    /// 32-byte ciphertext owner (the signer for a direct entry, the delegator for a
    /// delegated one).
    pub owner: Vec<u8>,
    /// 32-byte encrypted value ID (`encryptedValueId` in the request JSON).
    pub encrypted_value_id: Vec<u8>,
    /// The `leaf_count` the access proof was built against; 0 in current mode.
    pub proof_leaf_count: u64,
    /// Empty for current access; otherwise the borsh `MmrProof` blob.
    pub access_proof: Vec<u8>,
}

/// The borsh body of a `hostPayload`, mirroring the connector's `HostPayloadBody` field for
/// field over primitives. The field order here IS the canonical layout.
#[derive(BorshSerialize)]
struct HostPayloadBody {
    user_pubkey: Vec<u8>,
    transport_key: Vec<u8>,
    allowed_acl_domain_keys: Vec<Vec<u8>>,
    start_timestamp: u64,
    duration_seconds: u64,
    verifying_program_id: Vec<u8>,
    chain_id: u64,
    extra_data: Vec<u8>,
    signature: Vec<u8>,
    handles: Vec<HostPayloadEntry>,
}

/// One handle entry of the body, mirroring the connector's `HostPayloadEntry`.
#[derive(BorshSerialize)]
struct HostPayloadEntry {
    handle: Vec<u8>,
    owner: Vec<u8>,
    encrypted_value_id: Vec<u8>,
    proof_leaf_count: u64,
    access_proof: Vec<u8>,
}

/// Why the canonical `hostPayload` bytes could not be produced.
#[derive(Debug, thiserror::Error)]
pub enum HostPayloadEncodeError {
    /// The borsh body refused to serialize. The `Vec` writer itself cannot fail, so in
    /// practice this is a collection longer than borsh's `u32` length prefix.
    #[error("host payload body does not serialize: {reason}")]
    BodySerialization {
        /// What the serializer tripped over.
        reason: String,
    },
}

/// Encodes the permit fields, the ed25519 signature and the handle entries into the
/// canonical `hostPayload` bytes: `0x01 ‖ borsh(body)`.
///
/// The only failure borsh can produce for this body is a collection longer than its
/// `u32` length prefix. Every list is bounded far below that upstream, but the bound
/// lives in other modules, so the failure is propagated rather than assumed away.
pub fn encode_host_payload(
    permit: &PermitWireFields,
    signature: &[u8],
    handles: &[SolanaHandleWire],
) -> Result<Vec<u8>, HostPayloadEncodeError> {
    // Exhaustive destructuring, not field access: a field added to `PermitWireFields` or
    // `SolanaHandleWire` then fails to compile here instead of silently vanishing from the
    // signed bytes. This is the relayer's half of the drift guard the connector applies when
    // it destructures the decoded body on its side.
    let PermitWireFields {
        user_pubkey,
        transport_key,
        allowed_acl_domain_keys,
        start_timestamp,
        duration_seconds,
        verifying_program_id,
        chain_id,
        extra_data,
    } = permit;

    let body = HostPayloadBody {
        user_pubkey: user_pubkey.clone(),
        transport_key: transport_key.clone(),
        allowed_acl_domain_keys: allowed_acl_domain_keys.clone(),
        start_timestamp: *start_timestamp,
        duration_seconds: *duration_seconds,
        verifying_program_id: verifying_program_id.clone(),
        chain_id: *chain_id,
        extra_data: extra_data.clone(),
        signature: signature.to_vec(),
        handles: handles
            .iter()
            .map(|entry| {
                let SolanaHandleWire {
                    handle,
                    owner,
                    encrypted_value_id,
                    proof_leaf_count,
                    access_proof,
                } = entry;
                HostPayloadEntry {
                    handle: handle.clone(),
                    owner: owner.clone(),
                    encrypted_value_id: encrypted_value_id.clone(),
                    proof_leaf_count: *proof_leaf_count,
                    access_proof: access_proof.clone(),
                }
            })
            .collect(),
    };
    let mut bytes = vec![HOST_PAYLOAD_VERSION];
    borsh::to_writer(&mut bytes, &body).map_err(|source| {
        HostPayloadEncodeError::BodySerialization {
            reason: source.to_string(),
        }
    })?;
    Ok(bytes)
}
