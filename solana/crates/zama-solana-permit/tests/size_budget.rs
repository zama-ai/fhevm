//! Size of the signed bytes.
//!
//! A hardware wallet will only clear-sign a message up to a fixed length; beyond it,
//! the user is asked to blind-sign, which defeats the whole point of a human-readable
//! permit. So the widest permit the protocol admits has to fit, with margin, and the
//! margin has to be asserted rather than assumed — every future line added to the
//! template spends it.
//!
//! The ceiling is the wire packet size, 1232 bytes. It is written as a literal here
//! rather than imported: this crate stays free of Solana dependencies so that
//! consumers on different Solana versions can all use it.

mod common;

use common::*;
use zama_solana_permit::{build_envelope, render_canonical_text, MAX_ACL_DOMAIN_KEYS};

/// The clear-signing ceiling: a wallet refuses to display more than this.
const CLEAR_SIGNING_LIMIT: usize = 1232;
/// Documented worst case at the maximum domain count.
const WORST_CASE_TEXT_AT_MAX_DOMAINS: usize = 914;
/// The same, wrapped in the envelope.
const WORST_CASE_ENVELOPE_AT_MAX_DOMAINS: usize = 964;

fn sizes(domain_count: usize) -> (usize, usize) {
    let fields = decoded(&worst_case_wire(domain_count));
    (
        render_canonical_text(&fields).len(),
        build_envelope(&fields).len(),
    )
}

/// The widest admissible permit fits the budget, and the documented numbers are the
/// real ones.
#[test]
fn the_widest_permit_matches_its_documented_size_and_fits() {
    let (text, envelope) = sizes(MAX_ACL_DOMAIN_KEYS);

    assert_eq!(text, WORST_CASE_TEXT_AT_MAX_DOMAINS, "worst-case text size");
    assert_eq!(
        envelope, WORST_CASE_ENVELOPE_AT_MAX_DOMAINS,
        "worst-case envelope size"
    );
    assert!(
        envelope < CLEAR_SIGNING_LIMIT,
        "the worst case must clear-sign: {envelope} against {CLEAR_SIGNING_LIMIT}"
    );
}

/// The size a typical scoped permit approaches.
#[test]
fn the_two_domain_worst_case_matches_its_documented_size() {
    assert_eq!(sizes(2), (537, 587));
}

/// The remaining margin, stated as a number so that spending it is a visible change.
/// Adding a line to the template shrinks this; the diff will say by how much.
#[test]
fn the_clear_signing_margin_is_pinned() {
    let (_, envelope) = sizes(MAX_ACL_DOMAIN_KEYS);
    let margin = CLEAR_SIGNING_LIMIT - envelope;

    assert_eq!(
        margin, 268,
        "clear-signing margin changed — a template change spent or freed budget"
    );
}

/// Every permit the typed form admits fits, not only the constructed worst case.
#[test]
fn every_admissible_permit_fits_the_budget() {
    for seed in 0..1024u64 {
        let fields = decoded(&pseudo_valid_wire(seed));
        let envelope = build_envelope(&fields).len();

        assert!(
            envelope < CLEAR_SIGNING_LIMIT,
            "seed {seed}: envelope of {envelope} bytes does not clear-sign"
        );
        assert!(
            envelope <= WORST_CASE_ENVELOPE_AT_MAX_DOMAINS,
            "seed {seed}: envelope of {envelope} bytes exceeds the worst case, \
             so the worst case is not the worst case"
        );
    }
}

/// Size grows with the domain count and with nothing else surprising: each additional
/// domain costs one line of fixed width, so the worst case is the maximum count and not
/// some interior point.
#[test]
fn each_additional_domain_costs_a_fixed_number_of_bytes() {
    // A 44-character key line is "- " plus the encoding plus the line feed. The final
    // line has no feed, which is why the step is measured between consecutive counts.
    const PER_DOMAIN: usize = 2 + 44 + 1;

    let mut previous = sizes(1).0;
    for count in 2..=MAX_ACL_DOMAIN_KEYS {
        let current = sizes(count).0;
        // The enumeration header states the count as a decimal integer, so a count that
        // gains a digit costs one byte beyond its own line. Pinned rather than smoothed
        // over: it is the only place the text does not grow linearly in the domain count,
        // and the documented worst case is measured where that byte is already spent.
        let header_growth = count.to_string().len() - (count - 1).to_string().len();
        assert_eq!(
            current - previous,
            PER_DOMAIN + header_growth,
            "the step from {} to {count} domains",
            count - 1
        );
        previous = current;
    }
}

/// The envelope is a fixed 50 bytes of framing over the text, at every size.
#[test]
fn the_envelope_adds_a_constant_overhead() {
    for count in 0..=MAX_ACL_DOMAIN_KEYS {
        let (text, envelope) = sizes(count);
        assert_eq!(
            envelope - text,
            16 + 1 + 1 + 32,
            "framing at {count} domains"
        );
    }
}

/// Going from the permissive line to an enumerated list costs what the enumeration
/// costs, and the permissive form is never the larger one — a permissive permit is the
/// widest grant but the shortest text, which is worth knowing when reading the budget.
#[test]
fn the_permissive_form_is_the_shortest() {
    let permissive = render_canonical_text(&decoded(&worst_case_wire(0))).len();

    for count in 1..=MAX_ACL_DOMAIN_KEYS {
        assert!(
            permissive < sizes(count).0,
            "permissive should be shorter than {count} enumerated domains"
        );
    }
}
