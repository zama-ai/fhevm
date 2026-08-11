//! What happens when a wallet signs something that is *not* the canonical text.
//!
//! The rendering rules — printable ASCII only, line feeds only, no trailing content,
//! a fixed line order, the explicit permissive line — cannot be expressed as
//! renderer rejections: the renderer takes validated fields and is incapable of
//! emitting any of these shapes. Their real enforcement is here. A verifier
//! reconstructs the text from typed fields and checks the signature over *that*, so
//! a permit signed over a text that breaks any pin simply fails verification, with
//! no code anywhere having to inspect the signed bytes.
//!
//! This is also the test that fails loudly if anyone ever adds a "verify against the
//! text I was given" path: every case below would start passing.

mod common;

use common::*;
use zama_solana_permit::{render_canonical_text, verify_signature, PermitError};

/// Baseline: a signature over the properly reconstructed text verifies. Without
/// this, every rejection below could be passing for the wrong reason.
#[test]
fn signature_over_canonical_text_is_accepted() {
    let fields = reference_fields();
    let canonical = render_canonical_text(&fields);
    let signature = sign_text_as_wallet(USER_SEED, &canonical);

    assert_eq!(verify_signature(&fields, &signature), Ok(()));
}

/// A single appended line feed is a different signed byte string. This is the pin
/// that says the final line carries no newline.
#[test]
fn signature_over_text_with_trailing_newline_is_rejected() {
    let fields = reference_fields();
    let mut tampered = render_canonical_text(&fields);
    tampered.push('\n');
    let signature = sign_text_as_wallet(USER_SEED, &tampered);

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// Arbitrary content appended after the final line — the classic "sign this, ignore
/// the rest" trick — cannot survive reconstruction.
#[test]
fn signature_over_text_with_trailing_bytes_is_rejected() {
    let fields = reference_fields();
    for suffix in [
        "\nACL domains: ALL (permissive)",
        "\n- 21111111111111111111111111111111111111111111",
        " ",
        "\n\n",
        "\u{0}",
    ] {
        let tampered = format!("{}{suffix}", render_canonical_text(&fields));
        let signature = sign_text_as_wallet(USER_SEED, &tampered);

        assert_eq!(
            verify_signature(&fields, &signature),
            Err(PermitError::SignatureMismatch),
            "trailing {suffix:?} must not verify"
        );
    }
}

/// A homoglyph attack: the wallet is shown a text where a Latin letter is swapped for
/// a Cyrillic one that looks identical, so a human reader cannot tell the difference.
/// Reconstruction is what defeats it — the verifier never sees the displayed text.
#[test]
fn signature_over_text_with_non_ascii_lookalike_is_rejected() {
    let fields = reference_fields();
    let canonical = render_canonical_text(&fields);

    // U+0430 CYRILLIC SMALL LETTER A in place of the Latin 'a' of the header line.
    let tampered = canonical.replacen("Zama", "Z\u{0430}ma", 1);
    assert_ne!(tampered, canonical, "the substitution must have applied");
    assert!(!tampered.is_ascii(), "the tampered text must be non-ASCII");

    let signature = sign_text_as_wallet(USER_SEED, &tampered);

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// Reordering the lines keeps every value a human would read but changes the bytes.
#[test]
fn signature_over_text_with_reordered_lines_is_rejected() {
    let fields = reference_fields();
    let canonical = render_canonical_text(&fields);
    let mut lines: Vec<&str> = canonical.lines().collect();
    lines.swap(1, 2); // User / Verifying program

    let signature = sign_text_as_wallet(USER_SEED, &lines.join("\n"));

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// A wallet that rendered the empty domain list as an empty enumeration block —
/// instead of the explicit permissive line — produces a signature no verifier
/// accepts. That is what keeps the breadth of a permissive grant visible: the
/// alternative rendering is not merely discouraged, it is unusable.
#[test]
fn signature_over_empty_enumeration_instead_of_permissive_line_is_rejected() {
    let fields = decoded(&permissive_wire());
    let canonical = render_canonical_text(&fields);
    let tampered = canonical.replace("ACL domains: ALL (permissive)", "ACL domains (0):");
    assert_ne!(tampered, canonical);

    let signature = sign_text_as_wallet(USER_SEED, &tampered);

    assert_eq!(
        verify_signature(&fields, &signature),
        Err(PermitError::SignatureMismatch)
    );
}

/// Dropping a line entirely — the shape an "optional lines" renderer would produce —
/// also fails. There are no optional lines.
#[test]
fn signature_over_text_missing_a_line_is_rejected() {
    let fields = reference_fields();
    let canonical = render_canonical_text(&fields);

    for dropped in 0..canonical.lines().count() {
        let tampered: Vec<&str> = canonical
            .lines()
            .enumerate()
            .filter_map(|(index, line)| (index != dropped).then_some(line))
            .collect();
        let signature = sign_text_as_wallet(USER_SEED, &tampered.join("\n"));

        assert_eq!(
            verify_signature(&fields, &signature),
            Err(PermitError::SignatureMismatch),
            "text missing line {dropped} must not verify"
        );
    }
}

/// Every single-line value substitution fails: the signed text and the typed fields
/// are one object, so changing either side breaks the pairing.
#[test]
fn signature_over_text_with_a_substituted_value_is_rejected() {
    let fields = reference_fields();
    let canonical = render_canonical_text(&fields);

    let substitutions = [
        ("Chain id: 10037641751006774702", "Chain id: 1"),
        (
            "Valid from: 2026-01-01T01:03:00Z for 604800 seconds",
            "Valid from: 2026-01-01T01:03:00Z for 31536000 seconds",
        ),
        ("ACL domains (2):", "ACL domains (1):"),
        (
            "Zama fhevm Solana user-decrypt permit v1",
            "Zama fhevm Solana user-decrypt permit v2",
        ),
    ];

    for (from, to) in substitutions {
        let tampered = canonical.replace(from, to);
        assert_ne!(tampered, canonical, "substitution {from:?} did not apply");
        let signature = sign_text_as_wallet(USER_SEED, &tampered);

        assert_eq!(
            verify_signature(&fields, &signature),
            Err(PermitError::SignatureMismatch),
            "substituting {from:?} -> {to:?} must not verify"
        );
    }
}
