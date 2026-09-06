//! Strict decoding of the transport form.
//!
//! This is the layer that runs before any text exists and before any signature is
//! looked at. Every rule is tested on its own field, with everything else well
//! formed, so a passing test names exactly one reason for rejection — which is what
//! the normative vectors need in order to assert that a bad permit failed for the
//! reason it was built to fail for.
//!
//! Two rules here carry more weight than their size suggests. The transport-key
//! length is what fixes the key variant, because the permit carries no variant
//! field: accepting a 1568-byte key would leave two implementations free to disagree
//! about what a key means. And the timestamp bound is what makes the text rendering
//! total and keeps the authorization layer's `start + duration` away from the u64
//! edge.

mod common;

use common::*;
use zama_solana_permit::{
    IdentityField, PermitError, PermitFields, PermitWireFields, MAX_ALLOWED_SCOPES,
    MAX_DURATION_SECONDS, MAX_START_TIMESTAMP, MIN_DURATION_SECONDS, SCOPE_LEN, TRANSPORT_KEY_LEN,
};

/// Convenience: decode and expect a specific rejection.
fn expect_rejected(wire: &PermitWireFields, expected: PermitError) {
    match PermitFields::decode(wire) {
        Ok(_) => panic!("expected rejection {expected:?}, but the permit was accepted"),
        Err(actual) => assert_eq!(actual, expected),
    }
}

/// Convenience: decode and expect acceptance.
fn expect_accepted(wire: &PermitWireFields) -> PermitFields {
    PermitFields::decode(wire).expect("permit should have been accepted")
}

// ---------------------------------------------------------------------------
// Baseline
// ---------------------------------------------------------------------------

#[test]
fn decode_accepts_the_reference_permit() {
    let fields = expect_accepted(&reference_wire());

    assert_eq!(fields.user_pubkey().as_bytes(), &bytes32(USER_PUBKEY_HEX));
    assert_eq!(
        fields.verifying_program_id().as_bytes(),
        &bytes32(VERIFYING_PROGRAM_ID_HEX)
    );
    assert_eq!(fields.chain_id(), CHAIN_ID);
    assert_eq!(fields.start_timestamp(), START_TIMESTAMP);
    assert_eq!(fields.duration_seconds(), DURATION_SECONDS);
    assert_eq!(fields.transport_key().as_bytes().len(), TRANSPORT_KEY_LEN);
    assert_eq!(fields.allowed_scopes().as_slice().len(), 2);
    assert!(!fields.allowed_scopes().is_permissive());
    assert_eq!(
        fields.allowed_scopes().as_slice()[0].program().as_bytes(),
        &bytes32(APP_PROGRAM_ID_HEX)
    );
    assert_eq!(
        fields.allowed_scopes().as_slice()[0].scope().as_bytes(),
        &bytes32(SCOPE_43_HEX)
    );

    let (context, epoch) = kms_routing(&fields);
    assert_eq!(context.as_bytes(), &bytes32(KMS_CONTEXT_ID_HEX));
    assert_eq!(epoch.as_bytes(), &bytes32(KMS_EPOCH_ID_HEX));
}

/// An empty scope list is well formed, not an error: it is the permissive permit.
#[test]
fn decode_accepts_an_empty_scope_list_as_permissive() {
    let fields = expect_accepted(&permissive_wire());

    assert!(fields.allowed_scopes().is_permissive());
    assert!(fields.allowed_scopes().as_slice().is_empty());
}

// ---------------------------------------------------------------------------
// Identity widths
// ---------------------------------------------------------------------------

/// A 32-byte identity that arrives at any other width is rejected. Widths are
/// checked here because they are unrepresentable once the typed form is reached —
/// which is also why this test decodes rather than constructing typed values.
#[test]
fn decode_rejects_user_pubkey_of_wrong_width() {
    for len in [0usize, 1, 20, 31, 33, 64] {
        let wire = PermitWireFields {
            user_pubkey: vec![0x11; len],
            ..reference_wire()
        };
        expect_rejected(
            &wire,
            PermitError::IdentityWidth {
                field: IdentityField::UserPubkey,
                len,
            },
        );
    }
}

#[test]
fn decode_rejects_verifying_program_id_of_wrong_width() {
    for len in [0usize, 20, 31, 33] {
        let wire = PermitWireFields {
            verifying_program_id: vec![0x22; len],
            ..reference_wire()
        };
        expect_rejected(
            &wire,
            PermitError::IdentityWidth {
                field: IdentityField::VerifyingProgramId,
                len,
            },
        );
    }
}

/// A wrong-width scope entry is reported with its index, so a caller can say which
/// entry was malformed. A bare 32-byte identity is the natural mistake and is a width
/// violation like any other: a scope is a program and a scope, never one alone.
#[test]
fn decode_rejects_scope_of_wrong_width() {
    for (index, len) in [(0usize, 63usize), (1, 65), (1, 0), (0, 32)] {
        let mut wire = reference_wire();
        wire.allowed_scopes[index] = vec![0x33; len];
        expect_rejected(&wire, PermitError::ScopeWidth { index, len });
    }
}

// ---------------------------------------------------------------------------
// Scope count
// ---------------------------------------------------------------------------

#[test]
fn decode_rejects_more_scopes_than_permitted() {
    for count in [MAX_ALLOWED_SCOPES + 1, MAX_ALLOWED_SCOPES + 2, 64] {
        let mut scopes: Vec<[u8; SCOPE_LEN]> = (0..count).map(distinct_scope).collect();
        scopes.sort_unstable();
        let wire = PermitWireFields {
            allowed_scopes: scopes.iter().map(|scope| scope.to_vec()).collect(),
            ..reference_wire()
        };
        expect_rejected(&wire, PermitError::TooManyScopes { count });
    }
}

/// Both ends of the list bound are accepted: the empty list and the maximum count.
#[test]
fn decode_accepts_scope_counts_up_to_the_maximum() {
    for count in 0..=MAX_ALLOWED_SCOPES {
        let fields = expect_accepted(&wire_with_scope_count(count));
        assert_eq!(fields.allowed_scopes().as_slice().len(), count);
    }
}

// ---------------------------------------------------------------------------
// Scope ordering
// ---------------------------------------------------------------------------

/// The required counterexample: the two fixture scopes are ascending in byte order and
/// descending as base58 strings. Presented in string order, the list must be
/// rejected.
///
/// This is the one ordering test that cannot be replaced by an easier pair. Sorting
/// scopes by their rendered *string* is a natural mistake — it is what you get by
/// sorting the values a user sees — and it agrees with byte order for almost every
/// pair. It diverges exactly when the encodings differ in length, which is why the
/// fixture pins a 43-character scope half against a 44-character one.
#[test]
fn decode_rejects_scopes_ordered_by_base58_string_instead_of_bytes() {
    let mut scopes = reference_scopes();
    scopes.reverse();
    let string_order = PermitWireFields {
        allowed_scopes: scopes,
        ..reference_wire()
    };

    // The pair really is the counterexample: string order is the reverse of byte order.
    assert!(SCOPE_44_BASE58 < SCOPE_43_BASE58);
    assert!(bytes32(SCOPE_44_HEX) > bytes32(SCOPE_43_HEX));

    expect_rejected(&string_order, PermitError::ScopesNotAscending { index: 1 });

    // The same two scopes in byte order are accepted — so the rejection above is about
    // ordering, not about these scopes.
    expect_accepted(&reference_wire());
}

#[test]
fn decode_rejects_descending_scopes() {
    let mut scopes: Vec<[u8; SCOPE_LEN]> = (0..4).map(distinct_scope).collect();
    scopes.sort_unstable();
    scopes.reverse();
    let wire = PermitWireFields {
        allowed_scopes: scopes.iter().map(|scope| scope.to_vec()).collect(),
        ..reference_wire()
    };

    expect_rejected(&wire, PermitError::ScopesNotAscending { index: 1 });
}

/// A repeated scope is rejected rather than collapsed. Silently deduplicating would
/// mean two different signed lists render the same text.
#[test]
fn decode_rejects_duplicate_scope() {
    let scope = scope_entry(bytes32(APP_PROGRAM_ID_HEX), bytes32(SCOPE_43_HEX));
    let wire = PermitWireFields {
        allowed_scopes: vec![scope.clone(), scope],
        ..reference_wire()
    };

    expect_rejected(&wire, PermitError::DuplicateScope { index: 1 });
}

/// A duplicate that is not adjacent is still a duplicate — the rule is about the set,
/// not about neighbors. Such a list is necessarily also non-ascending, and either
/// rejection is correct; what matters is that it cannot be accepted.
#[test]
fn decode_rejects_a_repeated_scope_at_a_distance() {
    let mut scopes: Vec<[u8; SCOPE_LEN]> = (0..3).map(distinct_scope).collect();
    scopes.sort_unstable();
    let wire = PermitWireFields {
        allowed_scopes: vec![
            scopes[0].to_vec(),
            scopes[1].to_vec(),
            scopes[0].to_vec(),
            scopes[2].to_vec(),
        ],
        ..reference_wire()
    };

    assert!(
        PermitFields::decode(&wire).is_err(),
        "a list repeating an earlier scope must not be accepted"
    );
}

/// Ordering is decided by the first differing byte, at either end of the entry and in
/// either half: entries that differ only in their last scope byte are as ordered as
/// entries that differ in their first program byte.
#[test]
fn decode_accepts_minimally_separated_ascending_scopes() {
    for differing_index in [0usize, 31, 32, 63] {
        let mut lower = [0x40u8; SCOPE_LEN];
        let mut higher = lower;
        higher[differing_index] = 0x41;
        lower[differing_index] = 0x40;

        let wire = PermitWireFields {
            allowed_scopes: vec![lower.to_vec(), higher.to_vec()],
            ..reference_wire()
        };
        let fields = expect_accepted(&wire);
        assert_eq!(fields.allowed_scopes().as_slice()[0].as_bytes(), &lower);

        // The same pair swapped is rejected, so acceptance above was about order.
        let swapped = PermitWireFields {
            allowed_scopes: vec![higher.to_vec(), lower.to_vec()],
            ..reference_wire()
        };
        expect_rejected(&swapped, PermitError::ScopesNotAscending { index: 1 });
    }
}

/// There is one ordering rule, in one place: a consumer that builds the list directly
/// cannot end up with a laxer one.
#[test]
fn validated_scope_list_applies_the_same_rules_as_decoding() {
    use zama_solana_permit::{AllowedScopes, ApplicationScope, Identity};

    let program = Identity::new(bytes32(APP_PROGRAM_ID_HEX));
    let scope_43 = ApplicationScope::new(program, Identity::new(bytes32(SCOPE_43_HEX)));
    let scope_44 = ApplicationScope::new(program, Identity::new(bytes32(SCOPE_44_HEX)));

    assert!(AllowedScopes::new(vec![]).is_ok(), "permissive is valid");
    assert!(AllowedScopes::new(vec![scope_43, scope_44]).is_ok());
    assert_eq!(
        AllowedScopes::new(vec![scope_44, scope_43]),
        Err(PermitError::ScopesNotAscending { index: 1 })
    );
    assert_eq!(
        AllowedScopes::new(vec![scope_43, scope_43]),
        Err(PermitError::DuplicateScope { index: 1 })
    );

    let too_many: Vec<ApplicationScope> = {
        let mut scopes: Vec<[u8; SCOPE_LEN]> =
            (0..MAX_ALLOWED_SCOPES + 1).map(distinct_scope).collect();
        scopes.sort_unstable();
        scopes
            .into_iter()
            .map(ApplicationScope::from_bytes)
            .collect()
    };
    assert_eq!(
        AllowedScopes::new(too_many),
        Err(PermitError::TooManyScopes {
            count: MAX_ALLOWED_SCOPES + 1
        })
    );
}

/// The typed list answers the one question the authorization layer asks of it: is this
/// `(program, scope)` admitted. Permissive admits everything; a signed list admits the
/// pairs it names and nothing else — not the program alone, not the scope alone.
#[test]
fn validated_scope_list_admits_exactly_its_pairs() {
    use zama_solana_permit::Identity;

    let program = Identity::new(bytes32(APP_PROGRAM_ID_HEX));
    let other_program = Identity::new(bytes32(VERIFYING_PROGRAM_ID_HEX));
    let scope_43 = Identity::new(bytes32(SCOPE_43_HEX));
    let scope_44 = Identity::new(bytes32(SCOPE_44_HEX));
    let other_scope = Identity::new([0x55; 32]);

    let scoped = expect_accepted(&reference_wire());
    let scopes = scoped.allowed_scopes();
    assert!(scopes.admits(&program, &scope_43));
    assert!(scopes.admits(&program, &scope_44));
    assert!(!scopes.admits(&program, &other_scope));
    assert!(!scopes.admits(&other_program, &scope_43));

    let permissive = expect_accepted(&permissive_wire());
    assert!(permissive
        .allowed_scopes()
        .admits(&other_program, &other_scope));
}

// ---------------------------------------------------------------------------
// Validity window
// ---------------------------------------------------------------------------

/// A zero-length window is rejected rather than treated as "already expired": a
/// permit that authorizes nothing has no legitimate producer, and accepting it would
/// leave every consumer to decide independently what it means.
#[test]
fn decode_rejects_zero_duration() {
    let wire = PermitWireFields {
        duration_seconds: 0,
        ..reference_wire()
    };
    expect_rejected(
        &wire,
        PermitError::DurationOutOfRange {
            duration_seconds: 0,
        },
    );
}

#[test]
fn decode_rejects_duration_above_one_year() {
    for duration_seconds in [MAX_DURATION_SECONDS + 1, MAX_DURATION_SECONDS * 2, u64::MAX] {
        let wire = PermitWireFields {
            duration_seconds,
            ..reference_wire()
        };
        expect_rejected(&wire, PermitError::DurationOutOfRange { duration_seconds });
    }
}

#[test]
fn decode_accepts_duration_at_both_bounds() {
    for duration_seconds in [MIN_DURATION_SECONDS, MAX_DURATION_SECONDS] {
        let wire = PermitWireFields {
            duration_seconds,
            ..reference_wire()
        };
        assert_eq!(expect_accepted(&wire).duration_seconds(), duration_seconds);
    }
}

#[test]
fn decode_rejects_start_timestamp_beyond_the_representable_range() {
    for start_timestamp in [MAX_START_TIMESTAMP + 1, u64::MAX / 2, u64::MAX] {
        let wire = PermitWireFields {
            start_timestamp,
            ..reference_wire()
        };
        expect_rejected(
            &wire,
            PermitError::StartTimestampOutOfRange { start_timestamp },
        );
    }
}

#[test]
fn decode_accepts_start_timestamp_at_both_bounds() {
    for start_timestamp in [0, MAX_START_TIMESTAMP] {
        let wire = PermitWireFields {
            start_timestamp,
            ..reference_wire()
        };
        assert_eq!(expect_accepted(&wire).start_timestamp(), start_timestamp);
    }
}

/// The timestamp bound exists so that the authorization layer can add the duration
/// without overflow checks of its own. Nothing the typed form accepts can come near
/// the u64 edge — asserted over the whole admitted space, including both corners
/// simultaneously.
#[test]
fn accepted_window_can_never_overflow_when_summed() {
    let corners = expect_accepted(&PermitWireFields {
        start_timestamp: MAX_START_TIMESTAMP,
        duration_seconds: MAX_DURATION_SECONDS,
        ..reference_wire()
    });
    assert!(corners
        .start_timestamp()
        .checked_add(corners.duration_seconds())
        .is_some());

    for seed in 0..256u64 {
        let fields = expect_accepted(&pseudo_valid_wire(seed));
        assert!(
            fields
                .start_timestamp()
                .checked_add(fields.duration_seconds())
                .is_some(),
            "seed {seed}: accepted window sums past the u64 edge"
        );
    }
}

// ---------------------------------------------------------------------------
// Transport key
// ---------------------------------------------------------------------------

/// The single accepted length is what fixes both the key variant and its
/// representation, because the permit carries no field for either. The accepted
/// length is the tfhe safe-serialized `UnifiedPublicEncKey::MlKem512` container —
/// what a KMS user-decryption request actually carries — so the most dangerous
/// near-miss is the bare 800-byte encapsulation key: the representation this permit
/// once accepted, and byte-for-byte the payload of the accepted container.
#[test]
fn decode_rejects_the_bare_key_of_the_accepted_variant() {
    let wire = PermitWireFields {
        transport_key: transport_key_bytes_of_len(800),
        ..reference_wire()
    };
    expect_rejected(&wire, PermitError::TransportKeyLength { len: 800 });
}

/// A well-formed key of the deprecated larger ML-KEM variant is rejected — not
/// because its bytes are wrong, but because its length is the variant declaration.
/// Both of its shapes are refused: the bare 1568-byte encapsulation key, and the
/// roughly 1637-byte safe-serialized container (only "not 869" is normative here, so
/// filler of those lengths is the exact test input).
#[test]
fn decode_rejects_transport_key_of_the_larger_variant_length() {
    for len in [1568usize, 1637] {
        let wire = PermitWireFields {
            transport_key: transport_key_bytes_of_len(len),
            ..reference_wire()
        };
        expect_rejected(&wire, PermitError::TransportKeyLength { len });
    }
}

#[test]
fn decode_rejects_transport_key_of_any_other_length() {
    for len in [0usize, 1, 32, 868, 870, 1600] {
        let wire = PermitWireFields {
            transport_key: transport_key_bytes_of_len(len),
            ..reference_wire()
        };
        expect_rejected(&wire, PermitError::TransportKeyLength { len });
    }
}

#[test]
fn decode_accepts_transport_key_of_the_accepted_length() {
    let fields = expect_accepted(&reference_wire());
    assert_eq!(
        fields.transport_key().as_bytes().as_slice(),
        reference_transport_key().as_slice()
    );
}

// ---------------------------------------------------------------------------
// KMS routing field
// ---------------------------------------------------------------------------

/// Unknown versions are rejected before rendering, which is what keeps rendering a
/// total function: the renderer only ever sees versions it knows how to render.
#[test]
fn decode_rejects_unknown_kms_routing_version() {
    for version in [0x00u8, 0x01, 0x03, 0x04, 0xff] {
        let mut bytes = extra_data(bytes32(KMS_CONTEXT_ID_HEX), bytes32(KMS_EPOCH_ID_HEX));
        bytes[0] = version;
        let wire = PermitWireFields {
            extra_data: bytes,
            ..reference_wire()
        };
        expect_rejected(
            &wire,
            PermitError::UnknownKmsRoutingVersion {
                version: Some(version),
            },
        );
    }
}

/// An empty routing field carries no version byte at all — a distinct rejection from
/// "unknown version", so the diagnostic does not have to invent a version.
#[test]
fn decode_rejects_empty_kms_routing_field() {
    let wire = PermitWireFields {
        extra_data: Vec::new(),
        ..reference_wire()
    };
    expect_rejected(
        &wire,
        PermitError::UnknownKmsRoutingVersion { version: None },
    );
}

/// The known version has exactly one length. Both a byte short and a byte long are
/// rejected: a length that "contains" the fields with room to spare is a second
/// encoding of the same routing material.
#[test]
fn decode_rejects_kms_routing_field_of_wrong_length_for_its_version() {
    for len in [1usize, 33, 64, 66, 97] {
        let mut bytes = vec![0u8; len];
        bytes[0] = zama_solana_permit::KMS_ROUTING_VERSION_BYTE;
        let wire = PermitWireFields {
            extra_data: bytes,
            ..reference_wire()
        };
        expect_rejected(
            &wire,
            PermitError::KmsRoutingLength {
                version: zama_solana_permit::KMS_ROUTING_VERSION_BYTE,
                len,
            },
        );
    }
}

/// The typed routing material round-trips back to the exact signed bytes: the
/// parsed form loses nothing, so re-deriving the signed field from it is lossless.
#[test]
fn kms_routing_round_trips_to_its_signed_bytes() {
    let wire = reference_wire();
    let fields = expect_accepted(&wire);

    assert_eq!(fields.extra_data().to_extra_data(), wire.extra_data);
}

// ---------------------------------------------------------------------------
// Determinism of rejection
// ---------------------------------------------------------------------------

/// A permit violating several rules at once is rejected the same way every time.
/// Which violation wins is the implementation's choice — vectors carry exactly one
/// violation each, so nothing normative rides on the order — but it must not vary
/// between runs, or diagnostics become unreproducible.
#[test]
fn multiple_violations_produce_a_deterministic_rejection() {
    let wire = PermitWireFields {
        user_pubkey: vec![0u8; 31],
        duration_seconds: 0,
        start_timestamp: u64::MAX,
        transport_key: transport_key_bytes_of_len(7),
        extra_data: vec![0x09],
        ..reference_wire()
    };

    let first = PermitFields::decode(&wire).expect_err("must be rejected");
    for _ in 0..8 {
        assert_eq!(
            PermitFields::decode(&wire).expect_err("must be rejected"),
            first
        );
    }
}
