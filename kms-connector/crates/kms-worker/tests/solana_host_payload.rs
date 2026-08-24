//! The canonical `hostPayload` layout and its parity with the event's typed handles.
//!
//! The gateway's host-generic entry carries everything host-specific as one opaque blob;
//! this suite pins the blob's Rust side from the outside: the round trip through the
//! canonical bytes, the strictness of the decoder (one version, no trailing bytes, no
//! length lies), the fact that every wire field reaches the bytes (a field the encoder
//! forgot would round-trip fine on the reference fixture and corrupt real requests
//! silently), and the parity rule that admits a payload only when its handle list is
//! exactly the event's typed one.
//!
//! Why parity is load-bearing: the gateway enforces the bit budget on the TYPED handles,
//! and the KMS response linker binds their exact order and count. A payload free to name
//! other handles would be budgeted on one list and authorized on another.

mod solana_support;

use kms_worker::core::solana::host_payload::{
    HOST_PAYLOAD_VERSION, HostPayloadError, check_handle_list_parity, decode_host_payload,
    encode_host_payload,
};
use kms_worker::core::solana::request::SolanaUserDecryptRequestWire;
use kms_worker::core::solana_acl::HandleBytes;
use solana_support::{
    DOMAIN, EncryptedValueAccountFixture, FHE_TYPE_UINT64, PermitBuilder, RequestBuilder, Wallet,
    handle,
};

/// A reference request exercising the whole width of the wire form: two entries, one
/// current and one historical (with a real MMR proof), under a scoped permit.
fn reference_wire() -> SolanaUserDecryptRequestWire {
    let wallet = Wallet::new(7);
    let current = handle(10, FHE_TYPE_UINT64);
    let replaced = handle(11, FHE_TYPE_UINT64);
    let replacement = handle(12, FHE_TYPE_UINT64);

    let current_account = EncryptedValueAccountFixture::new(current, &[wallet.pubkey()]);
    let mut historical_account = EncryptedValueAccountFixture::new(replaced, &[wallet.pubkey()]);
    historical_account.update(replacement);
    let proof = historical_account.proof(0);

    RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).scope(&[DOMAIN]))
        .direct_current(&current_account, current)
        .historical(&historical_account, replaced, wallet.pubkey(), &proof, 1)
        .wire()
}

/// The typed handle list the gateway event would carry for `wire`.
fn typed_handles(wire: &SolanaUserDecryptRequestWire) -> Vec<HandleBytes> {
    wire.handles
        .iter()
        .map(|entry| {
            let handle: HandleBytes = entry
                .handle
                .as_slice()
                .try_into()
                .expect("fixture handles are 32 bytes");
            handle
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The canonical bytes
// ---------------------------------------------------------------------------

#[test]
fn a_host_payload_roundtrips_through_its_canonical_bytes() {
    let wire = reference_wire();

    let bytes = encode_host_payload(&wire).expect("the wire request serializes");
    assert_eq!(
        bytes[0], HOST_PAYLOAD_VERSION,
        "the payload leads with its version"
    );

    let decoded = decode_host_payload(&bytes).expect("the canonical bytes decode");
    assert_eq!(decoded, wire, "the round trip is exact");
}

/// Every wire field must reach the canonical bytes. A field the encoder forgot would pass
/// the round trip on any one fixture; changing each field in isolation and demanding
/// different bytes is what makes the omission visible.
#[test]
fn every_wire_field_reaches_the_canonical_bytes() {
    let base = reference_wire();
    let baseline = encode_host_payload(&base).expect("the base wire serializes");

    let mut variants: Vec<(&str, SolanaUserDecryptRequestWire)> = Vec::new();

    let mut wire = base.clone();
    wire.permit.user_pubkey[0] ^= 1;
    variants.push(("permit.user_pubkey", wire));

    let mut wire = base.clone();
    wire.permit.transport_key[0] ^= 1;
    variants.push(("permit.transport_key", wire));

    let mut wire = base.clone();
    wire.permit.allowed_acl_domain_keys[0][0] ^= 1;
    variants.push(("permit.allowed_acl_domain_keys", wire));

    let mut wire = base.clone();
    wire.permit.start_timestamp += 1;
    variants.push(("permit.start_timestamp", wire));

    let mut wire = base.clone();
    wire.permit.duration_seconds += 1;
    variants.push(("permit.duration_seconds", wire));

    let mut wire = base.clone();
    wire.permit.verifying_program_id[0] ^= 1;
    variants.push(("permit.verifying_program_id", wire));

    let mut wire = base.clone();
    wire.permit.chain_id ^= 1;
    variants.push(("permit.chain_id", wire));

    let mut wire = base.clone();
    wire.permit.extra_data[0] ^= 1;
    variants.push(("permit.extra_data", wire));

    let mut wire = base.clone();
    wire.signature[0] ^= 1;
    variants.push(("signature", wire));

    let mut wire = base.clone();
    wire.handles[0].handle[0] ^= 1;
    variants.push(("entry.handle", wire));

    let mut wire = base.clone();
    wire.handles[0].owner[0] ^= 1;
    variants.push(("entry.owner", wire));

    let mut wire = base.clone();
    wire.handles[0].encrypted_value_id[0] ^= 1;
    variants.push(("entry.encrypted_value_id", wire));

    let mut wire = base.clone();
    wire.handles[1].proof_leaf_count += 1;
    variants.push(("entry.proof_leaf_count", wire));

    let mut wire = base.clone();
    wire.handles[1].access_proof[0] ^= 1;
    variants.push(("entry.access_proof", wire));

    for (field, variant) in variants {
        let bytes = encode_host_payload(&variant).expect("every variant serializes");
        assert_ne!(
            bytes, baseline,
            "changing {field} left the canonical bytes unchanged"
        );
        assert_eq!(
            decode_host_payload(&bytes).expect("every variant decodes"),
            variant,
            "the round trip of the {field} variant is exact"
        );
    }
}

// ---------------------------------------------------------------------------
// Decoder strictness
// ---------------------------------------------------------------------------

#[test]
fn a_host_payload_of_an_unknown_version_is_rejected() {
    let mut bytes = encode_host_payload(&reference_wire()).expect("the reference wire serializes");
    bytes[0] = 0x02;
    assert_eq!(
        decode_host_payload(&bytes),
        Err(HostPayloadError::UnknownVersion {
            version: Some(0x02)
        })
    );

    assert_eq!(
        decode_host_payload(&[]),
        Err(HostPayloadError::UnknownVersion { version: None })
    );
}

#[test]
fn a_truncated_host_payload_is_rejected() {
    let bytes = encode_host_payload(&reference_wire()).expect("the reference wire serializes");

    for cut in [1usize, bytes.len() / 2, bytes.len() - 1] {
        assert!(
            matches!(
                decode_host_payload(&bytes[..cut]),
                Err(HostPayloadError::MalformedBody { .. })
                    | Err(HostPayloadError::UnknownVersion { .. })
            ),
            "a payload cut to {cut} byte(s) must not decode"
        );
    }
}

#[test]
fn trailing_bytes_after_the_body_are_rejected() {
    let mut bytes = encode_host_payload(&reference_wire()).expect("the reference wire serializes");
    bytes.push(0);

    assert_eq!(
        decode_host_payload(&bytes),
        Err(HostPayloadError::TrailingBytes { trailing: 1 })
    );
}

/// A length prefix inside the body claiming more than the body holds must be a decode
/// error, never a read into whatever follows.
#[test]
fn a_length_lie_inside_the_body_is_rejected() {
    let wire = reference_wire();
    let bytes = encode_host_payload(&wire).expect("the wire request serializes");

    // The first field after the version byte is the user pubkey, a borsh Vec<u8> whose
    // 4-byte little-endian length prefix sits right behind the version. Inflate it.
    let mut lied = bytes.clone();
    lied[1..5].copy_from_slice(&(u32::MAX).to_le_bytes());

    assert!(
        matches!(
            decode_host_payload(&lied),
            Err(HostPayloadError::MalformedBody { .. })
        ),
        "a length lie must fail the decode, not read past the field"
    );
}

// ---------------------------------------------------------------------------
// Parity with the event's typed handles
// ---------------------------------------------------------------------------

#[test]
fn the_typed_handle_list_admits_a_matching_payload() {
    let wire = reference_wire();
    let ct_handles = typed_handles(&wire);

    check_handle_list_parity(&ct_handles, &wire)
        .expect("a payload whose handle list is the typed list is admitted");
}

#[test]
fn a_reordered_handle_list_is_rejected() {
    let wire = reference_wire();
    let mut ct_handles = typed_handles(&wire);
    ct_handles.swap(0, 1);

    assert!(
        check_handle_list_parity(&ct_handles, &wire).is_err(),
        "the same handles in another order are another request"
    );
}

#[test]
fn an_extra_typed_handle_is_rejected() {
    let wire = reference_wire();
    let mut ct_handles = typed_handles(&wire);
    ct_handles.push(handle(99, FHE_TYPE_UINT64));

    assert_eq!(
        check_handle_list_parity(&ct_handles, &wire),
        Err(HostPayloadError::HandleListMismatch {
            payload_handles: 2,
            event_handles: 3,
        })
    );
}

#[test]
fn an_omitted_typed_handle_is_rejected() {
    let wire = reference_wire();
    let mut ct_handles = typed_handles(&wire);
    ct_handles.pop();

    assert_eq!(
        check_handle_list_parity(&ct_handles, &wire),
        Err(HostPayloadError::HandleListMismatch {
            payload_handles: 2,
            event_handles: 1,
        })
    );
}

#[test]
fn a_substituted_typed_handle_is_rejected() {
    let wire = reference_wire();
    let mut ct_handles = typed_handles(&wire);
    ct_handles[1] = handle(99, FHE_TYPE_UINT64);

    assert!(
        check_handle_list_parity(&ct_handles, &wire).is_err(),
        "a typed list naming a handle the payload does not is another request"
    );
}

/// Duplicates are legal on both sides, and parity counts them: a payload with the handle
/// once does not match an event carrying it twice.
#[test]
fn a_duplicate_count_mismatch_is_rejected() {
    let wire = reference_wire();
    let mut ct_handles = typed_handles(&wire);
    ct_handles.push(ct_handles[0]);

    assert_eq!(
        check_handle_list_parity(&ct_handles, &wire),
        Err(HostPayloadError::HandleListMismatch {
            payload_handles: 2,
            event_handles: 3,
        })
    );
}
