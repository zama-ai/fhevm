//! The canonical text, byte for byte.
//!
//! These are the goldens the whole protocol rests on: five implementations must
//! render the same bytes for the same fields, or a signature made against one is
//! worthless to the others. The expected literals below were computed *outside*
//! this crate (independent base58, SHAKE-256, Ed25519 and calendar
//! implementations), so a bug shared between the renderer and a helper it uses
//! cannot make these pass.
//!
//! Every golden here is also destined for the normative vector set, which is why
//! the fixture values are pinned rather than generated per run.

mod common;

use common::*;
use zama_solana_permit::{render_canonical_text, PermitFields};

// ---------------------------------------------------------------------------
// Fixture integrity
// ---------------------------------------------------------------------------

/// The fixture wallet's public key must belong to the fixture seed. If someone
/// edits one without the other, every golden below would need rewriting — this
/// fails first and says so.
#[test]
fn fixture_user_pubkey_belongs_to_fixture_seed() {
    assert_eq!(
        pubkey_of_seed(USER_SEED).as_bytes(),
        &bytes32(USER_PUBKEY_HEX),
        "fixture wallet pubkey drifted from its seed"
    );
}

/// The reference permit's two ACL-domain keys are the counterexample pair: byte
/// order and base58-string order disagree. The happy-path golden therefore only
/// stays green while the list is ordered by bytes.
#[test]
fn fixture_acl_domain_pair_has_opposite_byte_and_string_order() {
    let key_43 = bytes32(ACL_DOMAIN_KEY_43_HEX);
    let key_44 = bytes32(ACL_DOMAIN_KEY_44_HEX);

    assert_eq!(ACL_DOMAIN_KEY_43_BASE58.len(), 43);
    assert_eq!(ACL_DOMAIN_KEY_44_BASE58.len(), 44);
    assert!(key_43 < key_44, "the 43-char key must be smaller in bytes");
    assert!(
        ACL_DOMAIN_KEY_43_BASE58 > ACL_DOMAIN_KEY_44_BASE58,
        "the 43-char key must be larger as a string — otherwise this pair proves nothing"
    );
}

// ---------------------------------------------------------------------------
// Goldens
// ---------------------------------------------------------------------------

/// The reference permit's canonical text, byte for byte.
const REFERENCE_TEXT: &str = "Zama fhevm Solana user-decrypt permit v1\n\
User: Dzo7VaLffWBjA59P59wUCbRupUFKLts9BjFeTpM8G2EA\n\
Verifying program: 6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu\n\
Chain id: 10037641751006774702\n\
Transport key (SHAKE-256): CvgsmpoXufMbtBHn3zSjKjn6V2b2tNvrL6HJnZdwWnu4\n\
KMS context: DcvW9UCt85BDoYYoLtJkXamSPU11M6kF6auXdnu5H3BD\n\
KMS epoch: 93H5dJNEzmALYsPvnAD4zjKgaHjyqKNNovckD9AoPjK7\n\
Valid from: 2026-01-01T01:03:00Z for 604800 seconds\n\
ACL domains (2):\n\
- zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\n\
- 21111111111111111111111111111111111111111111";

/// The same permit with an empty domain list.
const PERMISSIVE_TEXT: &str = "Zama fhevm Solana user-decrypt permit v1\n\
User: Dzo7VaLffWBjA59P59wUCbRupUFKLts9BjFeTpM8G2EA\n\
Verifying program: 6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu\n\
Chain id: 10037641751006774702\n\
Transport key (SHAKE-256): CvgsmpoXufMbtBHn3zSjKjn6V2b2tNvrL6HJnZdwWnu4\n\
KMS context: DcvW9UCt85BDoYYoLtJkXamSPU11M6kF6auXdnu5H3BD\n\
KMS epoch: 93H5dJNEzmALYsPvnAD4zjKgaHjyqKNNovckD9AoPjK7\n\
Valid from: 2026-01-01T01:03:00Z for 604800 seconds\n\
ACL domains: ALL (permissive)";

#[test]
fn render_reference_permit_matches_golden_bytes() {
    let rendered = render_canonical_text(&reference_fields());

    assert_eq!(rendered.as_bytes(), REFERENCE_TEXT.as_bytes());
    assert_eq!(rendered.len(), 534, "reference text length");
}

/// An empty domain list renders as one line naming the breadth, not as an empty
/// enumeration block: the human signer has to be able to see that the grant covers
/// everything.
#[test]
fn render_permissive_emits_single_all_domains_line() {
    let rendered = render_canonical_text(&decoded(&permissive_wire()));

    assert_eq!(rendered.as_bytes(), PERMISSIVE_TEXT.as_bytes());
    assert!(
        rendered.ends_with("\nACL domains: ALL (permissive)"),
        "permissive form must be the final line: {rendered}"
    );
    assert!(
        !rendered.contains("ACL domains (0)"),
        "an empty enumeration block hides the breadth of the grant: {rendered}"
    );
    assert!(
        !rendered.contains("\n- "),
        "the permissive form has no domain lines: {rendered}"
    );
}

/// The enumeration header states the count, and one line per key follows in signed
/// order.
#[test]
fn render_domain_enumeration_states_count_and_lists_every_key() {
    for count in 1..=zama_solana_permit::MAX_ACL_DOMAIN_KEYS {
        let wire = wire_with_domain_count(count);
        let rendered = render_canonical_text(&decoded(&wire));

        assert!(
            rendered.contains(&format!("\nACL domains ({count}):\n")),
            "missing enumeration header for {count} domains: {rendered}"
        );
        assert_eq!(
            rendered.matches("\n- ").count(),
            count,
            "one line per domain key at count {count}"
        );

        // Rendering follows the typed list order, which the decode step has already
        // fixed to ascending byte order.
        let keys = decoded(&wire).allowed_acl_domain_keys().as_slice().to_vec();
        let rendered_keys: Vec<&str> = rendered
            .lines()
            .filter_map(|line| line.strip_prefix("- "))
            .collect();
        assert_eq!(rendered_keys.len(), keys.len());
        for (key, line) in keys.iter().zip(rendered_keys) {
            assert_eq!(
                bs58_reference(key.as_bytes()),
                line,
                "domain line does not match its typed key"
            );
        }
    }
}

/// The ten-domain permit is the widest list the protocol admits.
#[test]
fn render_ten_domains_enumeration() {
    let rendered = render_canonical_text(&decoded(&wire_with_domain_count(10)));

    assert!(rendered.contains("\nACL domains (10):\n"));
    assert_eq!(rendered.matches("\n- ").count(), 10);
    assert!(
        !rendered.ends_with('\n'),
        "the last domain line carries no line feed"
    );
}

// ---------------------------------------------------------------------------
// Integer and timestamp rendering
// ---------------------------------------------------------------------------

/// Decimal, no leading zeros, no separators — and zero renders as a bare `0`
/// rather than being omitted or padded.
#[test]
fn render_zero_valued_integers_as_bare_zero() {
    let wire = zama_solana_permit::PermitWireFields {
        chain_id: 0,
        start_timestamp: 0,
        duration_seconds: zama_solana_permit::MIN_DURATION_SECONDS,
        ..permissive_wire()
    };

    let rendered = render_canonical_text(&decoded(&wire));

    assert!(rendered.contains("\nChain id: 0\n"), "{rendered}");
    assert!(
        rendered.contains("\nValid from: 1970-01-01T00:00:00Z for 1 seconds\n"),
        "{rendered}"
    );
}

/// No thousands separators anywhere, at the largest values each integer can take.
#[test]
fn render_large_integers_without_separators() {
    let wire = zama_solana_permit::PermitWireFields {
        chain_id: u64::MAX,
        start_timestamp: zama_solana_permit::MAX_START_TIMESTAMP,
        duration_seconds: zama_solana_permit::MAX_DURATION_SECONDS,
        ..permissive_wire()
    };

    let rendered = render_canonical_text(&decoded(&wire));

    assert!(
        rendered.contains("\nChain id: 18446744073709551615\n"),
        "{rendered}"
    );
    assert!(
        rendered.contains("\nValid from: 9999-12-31T23:59:59Z for 31536000 seconds\n"),
        "{rendered}"
    );
    assert!(!rendered.contains(','), "{rendered}");
    assert!(!rendered.contains('_'), "{rendered}");
}

/// Every timestamp component is zero-padded to its fixed width, to the second, with
/// no fractional part — including the calendar corners: a leap day, the day after a
/// non-leap century boundary, and the last second before a year rolls over.
#[test]
fn render_timestamp_is_fixed_width_and_calendar_correct() {
    let cases = [
        (0u64, "1970-01-01T00:00:00Z"),
        (1, "1970-01-01T00:00:01Z"),
        (1_041_418_205, "2003-01-01T10:50:05Z"),
        (946_684_799, "1999-12-31T23:59:59Z"),
        // Leap day of a leap year divisible by 400.
        (951_782_400, "2000-02-29T00:00:00Z"),
        // Leap day of an ordinary leap year.
        (1_709_210_096, "2024-02-29T12:34:56Z"),
        // 2100 is not a leap year: the day after 28 February is 1 March.
        (4_107_542_399, "2100-02-28T23:59:59Z"),
        (4_107_542_400, "2100-03-01T00:00:00Z"),
        // The latest start the typed form admits.
        (253_402_300_799, "9999-12-31T23:59:59Z"),
    ];

    for (start_timestamp, expected) in cases {
        let wire = zama_solana_permit::PermitWireFields {
            start_timestamp,
            ..permissive_wire()
        };
        let rendered = render_canonical_text(&decoded(&wire));
        let line = rendered
            .lines()
            .find(|line| line.starts_with("Valid from: "))
            .expect("the validity line is always present");

        assert_eq!(
            line,
            format!("Valid from: {expected} for {DURATION_SECONDS} seconds"),
            "timestamp {start_timestamp} rendered wrong"
        );
        assert_eq!(expected.len(), 20, "the timestamp form is fixed width");
    }
}

// ---------------------------------------------------------------------------
// Line structure
// ---------------------------------------------------------------------------

/// Both terminal forms — a last domain line and the permissive line — end without a
/// line feed. A stray trailing newline is a different signed byte string.
#[test]
fn render_final_line_has_no_trailing_newline() {
    for wire in [
        reference_wire(),
        permissive_wire(),
        wire_with_domain_count(1),
        wire_with_domain_count(10),
    ] {
        let rendered = render_canonical_text(&decoded(&wire));
        assert!(
            !rendered.ends_with('\n'),
            "text must not end with a line feed: {rendered:?}"
        );
        assert!(
            !rendered.ends_with(char::is_whitespace),
            "text must not end with whitespace: {rendered:?}"
        );
    }
}

/// The line order is fixed and every line is always present: there are no optional
/// lines, so a verifier reconstructing the text never has to guess a layout.
#[test]
fn render_line_sequence_is_fixed() {
    let expected_prefixes = [
        "Zama fhevm Solana user-decrypt permit v1",
        "User: ",
        "Verifying program: ",
        "Chain id: ",
        "Transport key (SHAKE-256): ",
        "KMS context: ",
        "KMS epoch: ",
        "Valid from: ",
        "ACL domains",
    ];

    for wire in [
        reference_wire(),
        permissive_wire(),
        wire_with_domain_count(10),
    ] {
        let rendered = render_canonical_text(&decoded(&wire));
        let lines: Vec<&str> = rendered.lines().collect();

        for (index, prefix) in expected_prefixes.iter().enumerate() {
            assert!(
                lines[index].starts_with(prefix),
                "line {index} should start with {prefix:?}, got {:?}",
                lines[index]
            );
        }

        let domain_count = decoded(&wire).allowed_acl_domain_keys().as_slice().len();
        assert_eq!(
            lines.len(),
            expected_prefixes.len() + domain_count,
            "no lines beyond the template plus one per domain: {rendered}"
        );
    }
}

/// Line feeds only: no carriage returns anywhere in the canonical bytes.
#[test]
fn render_uses_line_feeds_only() {
    for wire in [reference_wire(), permissive_wire(), worst_case_wire(10)] {
        let rendered = render_canonical_text(&decoded(&wire));
        assert!(!rendered.contains('\r'), "{rendered:?}");
    }
}

// ---------------------------------------------------------------------------
// Charset
// ---------------------------------------------------------------------------

/// The canonical bytes are printable ASCII plus line feeds, for every permit the
/// typed form admits.
///
/// This is stated as a property rather than as a rejection test on purpose: the
/// renderer takes validated fields, and none of them can carry an arbitrary byte
/// into the output — identities become base58, integers become decimal digits, the
/// timestamp becomes a fixed pattern. So a non-ASCII byte is not something the
/// renderer must reject; it is something the renderer must be incapable of
/// producing. A signed text that *does* contain one is a different question, and it
/// is answered where signatures are verified: reconstruction never yields it.
#[test]
fn rendered_text_is_printable_ascii_and_line_feeds_only() {
    let mut checked = 0;
    for seed in 0..512u64 {
        let wire = pseudo_valid_wire(seed);
        let rendered = render_canonical_text(&decoded(&wire));

        for (offset, byte) in rendered.bytes().enumerate() {
            assert!(
                (0x20..=0x7e).contains(&byte) || byte == 0x0a,
                "seed {seed}: byte {byte:#04x} at offset {offset} is outside the canonical charset"
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 512, "the whole sample must have been rendered");
}

/// No empty lines and no leading or trailing spaces on any line: the enumeration
/// block cannot degenerate into blank structure that a wallet would render
/// ambiguously.
#[test]
fn rendered_lines_are_never_blank_or_space_padded() {
    for seed in 0..128u64 {
        let rendered = render_canonical_text(&decoded(&pseudo_valid_wire(seed)));
        for (index, line) in rendered.lines().enumerate() {
            assert!(!line.is_empty(), "seed {seed}: line {index} is empty");
            assert_eq!(line.trim(), line, "seed {seed}: line {index} is padded");
        }
    }
}

// ---------------------------------------------------------------------------
// A base58 encoder that is not the crate's
// ---------------------------------------------------------------------------

/// Independent base58 encoder, used to check rendered identity lines without
/// borrowing the implementation under test. Bitcoin alphabet, standard
/// leading-zero handling, schoolbook long division — slow and obviously correct.
fn bs58_reference(bytes: &[u8; 32]) -> String {
    const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut digits: Vec<u8> = Vec::new();
    for &byte in bytes.iter() {
        let mut carry = u32::from(byte);
        for digit in digits.iter_mut() {
            carry += u32::from(*digit) << 8;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }
    let leading_zeros = bytes.iter().take_while(|byte| **byte == 0).count();
    let mut out = String::with_capacity(leading_zeros + digits.len());
    out.extend(std::iter::repeat_n('1', leading_zeros));
    out.extend(
        digits
            .iter()
            .rev()
            .map(|digit| ALPHABET[*digit as usize] as char),
    );
    out
}

#[test]
fn reference_base58_encoder_agrees_with_known_encodings() {
    // Pins the checker itself against encodings computed elsewhere, including the
    // 43/44-character pair and an all-zero key (leading-zero handling).
    assert_eq!(
        bs58_reference(&bytes32(ACL_DOMAIN_KEY_43_HEX)),
        ACL_DOMAIN_KEY_43_BASE58
    );
    assert_eq!(
        bs58_reference(&bytes32(ACL_DOMAIN_KEY_44_HEX)),
        ACL_DOMAIN_KEY_44_BASE58
    );
    assert_eq!(bs58_reference(&[0u8; 32]), "1".repeat(32));
    assert_eq!(
        bs58_reference(&bytes32(VERIFYING_PROGRAM_ID_HEX)),
        "6AtbvED1rfX68aCT1tYgU1aeu4kFksPDxZG9gtB1Fgtu"
    );
}

/// The text is a deterministic function of the fields: rendering twice yields the
/// same bytes, and rendering a re-decoded copy yields the same bytes.
#[test]
fn render_is_deterministic() {
    let wire = reference_wire();
    let first = render_canonical_text(&decoded(&wire));
    let second = render_canonical_text(&decoded(&wire));
    let third = render_canonical_text(&PermitFields::decode(&wire).expect("well formed"));

    assert_eq!(first, second);
    assert_eq!(first, third);
}
