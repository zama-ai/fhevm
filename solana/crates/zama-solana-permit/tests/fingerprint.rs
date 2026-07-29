//! The transport-key fingerprint.
//!
//! The signed text commits to a 32-byte digest of the transport key, because the full
//! key does not fit a hardware wallet's clear-signing budget. That makes the
//! fingerprint the one place where the signed bytes are a *summary* of a field rather
//! than the field, and therefore the one place where a verifier could be tricked into
//! trusting an input instead of recomputing it.
//!
//! The attack it stops is worth stating plainly: an attacker takes somebody else's
//! signed permit, attaches their own transport key, and the decryption result gets
//! sealed to the attacker. The defense is that no verifier accepts a fingerprint —
//! every one of them recomputes it from the key that actually traveled, so a
//! substituted key reconstructs a different text and the signature stops matching.

mod common;

use common::*;
use zama_solana_permit::{
    render_canonical_text, transport_key_fingerprint, verify_signature, PermitError,
    PermitWireFields, TransportKey,
};

/// Plain SHAKE-256, no domain-separation tag, 32 bytes out.
fn expected_fingerprint(key: &[u8]) -> [u8; 32] {
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };
    let mut hasher = Shake256::default();
    hasher.update(key);
    let mut out = [0u8; 32];
    hasher.finalize_xof().read(&mut out);
    out
}

// ---------------------------------------------------------------------------
// The digest itself
// ---------------------------------------------------------------------------

#[test]
fn fingerprint_matches_the_independently_computed_digest() {
    let key = transport_key(&reference_transport_key());

    assert_eq!(
        transport_key_fingerprint(&key),
        bytes32(REFERENCE_FINGERPRINT_HEX)
    );
}

/// The digest is plain SHAKE-256 over the key bytes: no domain-separation prefix, no
/// length prefix, no second hashing round. Any of those would be invisible in a single
/// golden but is visible here, because the expected value is computed from the
/// primitive directly.
#[test]
fn fingerprint_is_plain_shake256_of_the_key_bytes() {
    for label in [
        TRANSPORT_KEY_LABEL,
        b"another key".as_ref(),
        b"".as_ref(),
        b"\x00".as_ref(),
    ] {
        let bytes = transport_key_bytes(label);
        let key = transport_key(&bytes);

        assert_eq!(
            transport_key_fingerprint(&key),
            expected_fingerprint(&bytes),
            "fingerprint of the key expanded from {label:?}"
        );
    }
}

/// Every byte of the key is hashed. A fingerprint computed over a prefix of the key —
/// the shape of a truncation bug — would collide for keys differing only past the
/// truncation point.
#[test]
fn fingerprint_covers_every_byte_of_the_key() {
    let baseline_bytes = reference_transport_key();
    let baseline = transport_key_fingerprint(&transport_key(&baseline_bytes));

    for index in 0..baseline_bytes.len() {
        let mut mutated = baseline_bytes.clone();
        mutated[index] ^= 0x01;

        assert_ne!(
            transport_key_fingerprint(&transport_key(&mutated)),
            baseline,
            "flipping key byte {index} left the fingerprint unchanged"
        );
    }
}

/// Distinct keys have distinct fingerprints across a sample — the property the text's
/// commitment relies on.
#[test]
fn distinct_keys_have_distinct_fingerprints() {
    let mut seen = std::collections::HashMap::new();

    for seed in 0..256u64 {
        let bytes = transport_key_bytes(&seed.to_le_bytes());
        let fingerprint = transport_key_fingerprint(&transport_key(&bytes));

        if let Some(previous) = seen.insert(fingerprint, bytes.clone()) {
            assert_eq!(previous, bytes, "two distinct keys share a fingerprint");
        }
    }
}

// ---------------------------------------------------------------------------
// Substitution
// ---------------------------------------------------------------------------

/// Swapping the transport key under a signed permit breaks verification.
///
/// This is the attack in full: the signature and every other field are untouched and
/// genuine; only the key traveling alongside them is the attacker's. Because the
/// verifier recomputes the fingerprint, the reconstructed text differs from the signed
/// one and the signature no longer matches.
#[test]
fn substituting_the_transport_key_breaks_verification() {
    let honest_wire = reference_wire();
    let honest = decoded(&honest_wire);
    let signature = sign_with_seed(USER_SEED, &zama_solana_permit::build_envelope(&honest));
    assert_eq!(verify_signature(&honest, &signature), Ok(()));

    let attacker_wire = PermitWireFields {
        transport_key: transport_key_bytes(b"attacker transport key"),
        ..honest_wire
    };
    let attacked = decoded(&attacker_wire);

    // The reconstructed text differs precisely in the fingerprint line.
    let honest_text = render_canonical_text(&honest);
    let attacked_text = render_canonical_text(&attacked);
    assert_ne!(honest_text, attacked_text);
    let differing: Vec<(&str, &str)> = honest_text
        .lines()
        .zip(attacked_text.lines())
        .filter(|(a, b)| a != b)
        .collect();
    assert_eq!(differing.len(), 1, "only one line should differ");
    assert!(
        differing[0].0.starts_with("Transport key (SHAKE-256): "),
        "the differing line should be the fingerprint line, got {:?}",
        differing[0]
    );

    assert_eq!(
        verify_signature(&attacked, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// The same, across the admitted space rather than for one fixture: for any permit,
/// re-pointing it at another transport key invalidates its signature.
#[test]
fn transport_key_substitution_never_verifies() {
    for seed in 0..64u64 {
        let mut wire = pseudo_valid_wire(seed);
        wire.user_pubkey = pubkey_of_seed(USER_SEED).as_bytes().to_vec();
        let honest = decoded(&wire);
        let signature = sign_with_seed(USER_SEED, &zama_solana_permit::build_envelope(&honest));

        let attacked = decoded(&PermitWireFields {
            transport_key: transport_key_bytes(&(seed ^ 0xffff_ffff).to_le_bytes()),
            ..wire
        });

        assert_ne!(
            verify_signature(&attacked, &signature),
            Ok(()),
            "seed {seed}: a substituted transport key must not verify"
        );
    }
}

// ---------------------------------------------------------------------------
// API shape
// ---------------------------------------------------------------------------

/// The fingerprint function takes a key and returns a digest. It cannot be handed a
/// digest, and there is nowhere else in the public surface that accepts one — the
/// rendering and verification entry points take validated fields only, and the
/// fingerprint is not one of the fields.
///
/// The coercion is the assertion: a signature accepting a caller-supplied digest would
/// not compile against this line.
#[test]
fn fingerprint_takes_the_key_and_never_a_digest() {
    let fingerprinter: fn(&TransportKey) -> [u8; 32] = transport_key_fingerprint;

    let key = transport_key(&reference_transport_key());
    assert_eq!(fingerprinter(&key), bytes32(REFERENCE_FINGERPRINT_HEX));
}
