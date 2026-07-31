//! The envelope the wallet signs, and what verification accepts.
//!
//! Verification is reconstruction: the envelope is rebuilt from typed fields and the
//! signature is checked over those bytes. So the tests come in two shapes — the
//! envelope must be exactly the offchain-message form (byte goldens), and everything
//! that is *not* that form must fail, including forms a lax Ed25519 implementation
//! would accept.
//!
//! The last group matters more than it looks. Ed25519 implementations disagree about
//! non-canonical scalars and small-order keys, and this protocol is verified by five
//! of them. Any disagreement means the implementations accept different sets of
//! permits — a permit one party honors and another refuses. Strict semantics are
//! pinned here rather than left to whichever library each side happens to use.

mod common;

use common::*;
use zama_solana_permit::{
    build_envelope, render_canonical_text, verify_signature, PermitError, PermitFields,
    PermitWireFields, Signature,
};

// ---------------------------------------------------------------------------
// Envelope layout
// ---------------------------------------------------------------------------

/// The envelope is the preamble, the version, the signer count, the sole signer, and
/// the canonical text — in that order, with no length prefix and no application
/// domain.
#[test]
fn envelope_layout_is_the_offchain_message_form() {
    let fields = reference_fields();
    let envelope = build_envelope(&fields);

    // Header, as literal bytes rather than as the crate's own constants.
    assert_eq!(
        &envelope[..18],
        b"\xffsolana offchain\x01\x01",
        "envelope header"
    );
    assert_eq!(&envelope[18..50], fields.user_pubkey().as_bytes());
    assert_eq!(&envelope[50..], render_canonical_text(&fields).as_bytes());
    assert_eq!(envelope.len(), 584);
    assert_eq!(
        digest(&envelope),
        bytes32(REFERENCE_ENVELOPE_DIGEST_HEX),
        "reference envelope digest"
    );

    // And the whole thing equals a wallet-shaped envelope built independently.
    assert_eq!(
        envelope,
        envelope_over_text(fields.user_pubkey(), &render_canonical_text(&fields))
    );
}

#[test]
fn permissive_envelope_matches_its_independent_digest() {
    let fields = decoded(&permissive_wire());
    let envelope = build_envelope(&fields);

    assert_eq!(envelope.len(), 504);
    assert_eq!(digest(&envelope), bytes32(PERMISSIVE_ENVELOPE_DIGEST_HEX));
}

/// The leading byte is what keeps a permit signature from ever being a transaction
/// signature: `0xff` cannot begin a transaction, so the two byte spaces are
/// structurally disjoint.
#[test]
fn envelope_begins_with_the_byte_no_transaction_can_begin_with() {
    for wire in [reference_wire(), permissive_wire(), worst_case_wire(10)] {
        assert_eq!(build_envelope(&decoded(&wire))[0], 0xff);
    }
}

/// The signer count is always one, and the signer is the permit's own user — the text
/// names the same key on its `User:` line, so a wallet screen and the signed bytes
/// cannot disagree about who is signing.
#[test]
fn envelope_names_exactly_one_signer_which_is_the_permit_user() {
    for seed in 0..64u64 {
        let fields = decoded(&pseudo_valid_wire(seed));
        let envelope = build_envelope(&fields);

        assert_eq!(envelope[16], 1, "envelope version");
        assert_eq!(envelope[17], 1, "signer count");
        assert_eq!(&envelope[18..50], fields.user_pubkey().as_bytes());
    }
}

// ---------------------------------------------------------------------------
// Acceptance
// ---------------------------------------------------------------------------

/// A signature produced by a foreign implementation verifies. This is the test that
/// makes the goldens meaningful: had the crate and the test signer shared a bug, they
/// would agree with each other and disagree with this fixture.
#[test]
fn verify_accepts_a_signature_from_an_independent_implementation() {
    assert_eq!(
        verify_signature(
            &reference_fields(),
            &signature_from_hex(REFERENCE_SIGNATURE_HEX)
        ),
        Ok(())
    );
    assert_eq!(
        verify_signature(
            &decoded(&permissive_wire()),
            &signature_from_hex(PERMISSIVE_SIGNATURE_HEX)
        ),
        Ok(())
    );
}

/// The whole admitted space is signable and verifiable, not just the fixtures.
#[test]
fn verify_accepts_a_correctly_signed_permit_across_the_admitted_space() {
    for seed in 0..128u64 {
        let mut wire = pseudo_valid_wire(seed);
        // The generated user pubkey is arbitrary bytes; a signable permit needs the
        // fixture wallet's key in that field.
        wire.user_pubkey = pubkey_of_seed(USER_SEED).as_bytes().to_vec();
        let fields = decoded(&wire);

        let signature = sign_with_seed(USER_SEED, &build_envelope(&fields));
        assert_eq!(
            verify_signature(&fields, &signature),
            Ok(()),
            "seed {seed} should verify"
        );
    }
}

// ---------------------------------------------------------------------------
// Rejection
// ---------------------------------------------------------------------------

/// A different wallet's signature over the very same envelope is rejected: the sole
/// accepted signer is the key the permit names.
#[test]
fn verify_rejects_a_signature_from_another_signer() {
    let fields = reference_fields();
    let other_seed = b"zama-solana-permit-user-seed-002";

    let signature = sign_with_seed(other_seed, &build_envelope(&fields));

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// The bare text, signed without the envelope — the shape a raw-signing fallback
/// would produce. Accepting it would drop the structural separation from
/// transactions.
#[test]
fn verify_rejects_a_signature_over_the_bare_text() {
    let fields = reference_fields();
    let signature = sign_with_seed(USER_SEED, render_canonical_text(&fields).as_bytes());

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// Every header mutation is rejected: a different preamble, a different version, a
/// different signer count, a repeated signer, a length prefix.
#[test]
fn verify_rejects_signatures_over_mutated_envelope_headers() {
    let fields = reference_fields();
    let text = render_canonical_text(&fields);
    let canonical = build_envelope(&fields);

    let mut variants: Vec<(&str, Vec<u8>)> = Vec::new();

    // Preamble without the leading 0xff — i.e. bytes a transaction could also carry.
    let mut without_marker = canonical.clone();
    without_marker[0] = b'\x00';
    variants.push(("preamble without its marker byte", without_marker));

    // A different preamble string.
    let mut other_preamble = canonical.clone();
    other_preamble[1..16].copy_from_slice(b"solana offchaiN"[..15].as_ref());
    variants.push(("altered preamble text", other_preamble));

    for version in [0u8, 2, 0xff] {
        let mut mutated = canonical.clone();
        mutated[16] = version;
        variants.push(("envelope version", mutated));
    }

    for count in [0u8, 2, 0xff] {
        let mut mutated = canonical.clone();
        mutated[17] = count;
        variants.push(("signer count", mutated));
    }

    // Two signers, the user twice — the shape a multi-signer envelope would take.
    let mut two_signers = Vec::new();
    two_signers.extend_from_slice(b"\xffsolana offchain");
    two_signers.push(1);
    two_signers.push(2);
    two_signers.extend_from_slice(fields.user_pubkey().as_bytes());
    two_signers.extend_from_slice(fields.user_pubkey().as_bytes());
    two_signers.extend_from_slice(text.as_bytes());
    variants.push(("two signers", two_signers));

    // A length prefix before the text, which this envelope version does not have.
    let mut length_prefixed = Vec::new();
    length_prefixed.extend_from_slice(&canonical[..50]);
    length_prefixed.extend_from_slice(&(text.len() as u16).to_le_bytes());
    length_prefixed.extend_from_slice(text.as_bytes());
    variants.push(("length-prefixed text", length_prefixed));

    for (what, envelope) in variants {
        assert_ne!(
            envelope, canonical,
            "{what}: variant equals the canonical form"
        );
        let signature = sign_with_seed(USER_SEED, &envelope);
        assert_eq!(
            verify_signature(&fields, &signature),
            Err(PermitError::SignatureMismatch),
            "a signature over an envelope with a mutated {what} must not verify"
        );
    }
}

/// A signature whose scalar is not reduced — `S + L` instead of `S` — is rejected.
///
/// Both encodings satisfy the naive verification equation, so a lax implementation
/// accepts both and a strict one accepts only the reduced form. With five
/// implementations verifying the same permits, that difference is not academic: it is
/// two implementations disagreeing about whether a permit is authorized. This test
/// pins the strict side.
#[test]
fn verify_rejects_a_signature_with_a_non_canonical_scalar() {
    let fields = reference_fields();
    let canonical = signature_from_hex(REFERENCE_SIGNATURE_HEX);
    let non_canonical = signature_from_hex(NON_CANONICAL_SIGNATURE_HEX);

    // The two differ only in the scalar half, and the canonical one does verify.
    assert_eq!(canonical.as_bytes()[..32], non_canonical.as_bytes()[..32]);
    assert_ne!(canonical.as_bytes()[32..], non_canonical.as_bytes()[32..]);
    assert_eq!(verify_signature(&fields, &canonical), Ok(()));

    assert_eq!(
        verify_signature(&fields, &non_canonical),
        Err(PermitError::SignatureMismatch)
    );
}

/// A user pubkey that is not a point on the curve cannot verify anything, and says so
/// distinctly — the permit is unusable, rather than carrying a bad signature.
///
/// The encodings below are y-coordinates for which no curve point exists. Note that
/// most 32-byte patterns one reaches for *are* valid points — an all-`0xff` key, for
/// instance, decompresses fine — so the inputs here are chosen, not guessed.
#[test]
fn verify_rejects_a_user_pubkey_that_is_not_a_curve_point() {
    for encoded in [
        "0200000000000000000000000000000000000000000000000000000000000000",
        "0700000000000000000000000000000000000000000000000000000000000000",
        "0800000000000000000000000000000000000000000000000000000000000000",
        "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
    ] {
        let wire = PermitWireFields {
            user_pubkey: bytes32(encoded).to_vec(),
            ..reference_wire()
        };
        // Strict decoding accepts it: the width is right, and whether the bytes are a
        // curve point is not a typed-form question. It is caught at verification, in
        // one place, together with every other reason a key is unusable.
        let fields = PermitFields::decode(&wire).expect("32 bytes is a well-formed identity");

        let signature = sign_with_seed(USER_SEED, &build_envelope(&fields));

        assert_eq!(
            verify_signature(&fields, &signature),
            Err(PermitError::UnusableUserPubkey),
            "key {encoded} is not a curve point and must be unusable"
        );
    }
}

/// A user pubkey whose y-coordinate is encoded above the field modulus is rejected,
/// even though it decompresses to a perfectly good point.
///
/// There is no attack behind this one: the encoding is part of the signed envelope, so
/// a re-encoded key changes the message and the victim's signature does not carry over.
/// The reason is uniformity. Ed25519 libraries disagree about non-canonical
/// coordinates — the one this crate uses accepts them — so leaving the question open
/// means the five implementations that verify these permits accept different key sets.
/// Every real wallet key is canonical, so rejecting non-canonical encodings costs
/// nothing and removes the disagreement.
#[test]
fn verify_rejects_a_non_canonically_encoded_user_pubkey() {
    // y = 2^255 - 1, which reduces to the same point as the canonical encoding of 18.
    let non_canonical = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let wire = PermitWireFields {
        user_pubkey: bytes32(non_canonical).to_vec(),
        ..reference_wire()
    };
    let fields = PermitFields::decode(&wire).expect("32 bytes is a well-formed identity");

    let signature = sign_with_seed(USER_SEED, &build_envelope(&fields));

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::UnusableUserPubkey),
        "a non-canonical coordinate encoding must not be usable as a permit user"
    );
}

/// Small-order public keys never verify, whatever signature accompanies them.
///
/// These encodings belong to the torsion subgroup, where a signature can be forged
/// for any message under some verification routines. Nothing may verify under them.
#[test]
fn verify_rejects_small_order_user_pubkeys() {
    let small_order = [
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0100000000000000000000000000000000000000000000000000000000000000",
        "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
        "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
    ];

    for encoded in small_order {
        let wire = PermitWireFields {
            user_pubkey: bytes32(encoded).to_vec(),
            ..reference_wire()
        };
        let fields = PermitFields::decode(&wire).expect("well-formed width");

        for signature in [
            signature_from_hex(REFERENCE_SIGNATURE_HEX),
            Signature::new([0u8; 64]),
            sign_with_seed(USER_SEED, &build_envelope(&fields)),
        ] {
            assert_ne!(
                verify_signature(&fields, &signature),
                Ok(()),
                "small-order key {encoded} must never verify"
            );
        }
    }
}

/// An all-zero signature is rejected. Trivial, and worth pinning: it is the value a
/// caller ends up passing when a signature field is missing or defaulted.
#[test]
fn verify_rejects_an_empty_signature() {
    assert_ne!(
        verify_signature(&reference_fields(), &Signature::new([0u8; 64])),
        Ok(())
    );
}

/// A signature valid for one permit is invalid for any other. Reuse across permits is
/// what the reconstruction discipline exists to prevent.
#[test]
fn a_signature_does_not_carry_over_to_another_permit() {
    let fields = reference_fields();
    let signature = signature_from_hex(REFERENCE_SIGNATURE_HEX);

    for seed in 0..32u64 {
        let mut wire = pseudo_valid_wire(seed);
        wire.user_pubkey = pubkey_of_seed(USER_SEED).as_bytes().to_vec();
        let other = decoded(&wire);
        if other == fields {
            continue;
        }

        assert_ne!(
            verify_signature(&other, &signature),
            Ok(()),
            "seed {seed}: the reference signature must not authorize a different permit"
        );
    }
}

// ---------------------------------------------------------------------------
// API shape
// ---------------------------------------------------------------------------

/// Verification takes validated fields and a signature. Nothing else.
///
/// The coercion below is the assertion: it only compiles while the function has
/// exactly this signature. A text parameter, an envelope parameter, or a fingerprint
/// parameter would break this line — which is the point, since each of those is a way
/// to verify against something the caller supplied instead of against a locally
/// reconstructed permit. The mirror-image compile-failure checks live in the crate's
/// documentation tests.
#[test]
fn verification_takes_only_typed_fields_and_a_signature() {
    let verifier: fn(&PermitFields, &Signature) -> Result<(), PermitError> = verify_signature;
    let builder: fn(&PermitFields) -> Vec<u8> = build_envelope;
    let renderer: fn(&PermitFields) -> String = render_canonical_text;

    // Exercised through the pinned types, so the coercions are not dead code.
    let fields = reference_fields();
    assert_eq!(renderer(&fields), render_canonical_text(&fields));
    assert_eq!(builder(&fields), build_envelope(&fields));
    assert_eq!(
        verifier(&fields, &signature_from_hex(REFERENCE_SIGNATURE_HEX)),
        Ok(())
    );
}
