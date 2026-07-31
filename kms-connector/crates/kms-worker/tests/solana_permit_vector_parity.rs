//! Parity of the permit rules: the normative vector set, run through the Connector's own entry
//! point.
//!
//! The permit rules — typed form, domain ordering, signature over the reconstructed envelope —
//! must behave identically in every verifier that exists: the SDK, the relayer, this Connector,
//! the KMS side, and the fixture runner that generated the set. "Identically" is not a wish about
//! shared code; the Connector reuses the permit crate, but it is the Connector that maps the wire
//! form onto the crate's input, and that mapping is where softening happens in practice. A
//! Connector that accidentally accepted a domain list in the wrong order, or fed the crate a
//! truncated transport key, would be a second authority on what a permit is.
//!
//! So the set is run through [`SolanaUserDecryptRequest::decode`] and [`check_signature`] — the
//! same two functions the pipeline calls — and every outcome is compared against the outcome the
//! set declares, by rule name.
//!
//! The rule names are the cross-implementation contract, and this file writes the Connector's own
//! mapping onto them. It is deliberately not shared with the generator's mapping: two independent
//! mappings agreeing on the whole set is the evidence; one mapping used twice would be a tautology.
//!
//! The set contains no handle entries — it is a permit set — so each record is wrapped in a
//! minimal request with one synthetic entry. A control test runs the whole set a second time with
//! a different synthetic entry and requires identical outcomes, so the wrapper cannot be quietly
//! deciding anything.

mod solana_support;

// The schema lives with the fixtures, because five implementations include the same file. Each
// consumer uses the part of it that its own layer needs, so unused helpers are expected here
// rather than a sign of drift.
#[allow(dead_code)]
#[path = "../../../../solana/test-fixtures/permit/permit_vectors.rs"]
mod schema;

use kms_worker::core::solana::{
    failure::AuthorizationFailure,
    pipeline::check_signature,
    request::{
        RequestFormError, SolanaHandleEntryWire, SolanaUserDecryptRequest,
        SolanaUserDecryptRequestWire,
    },
};
use schema::{PERMIT_VECTOR_SCHEMA, PermitVector, PermitVectorFile, VectorResult, from_hex, rule};
use solana_support::*;
use std::collections::BTreeSet;
use zama_solana_permit::{PermitError, PermitWireFields};

/// The committed set, embedded at compile time: a renamed or deleted file is a build failure
/// rather than a suite that quietly stops checking anything.
const PERMIT_VECTORS: &str = include_str!("../../../../solana/test-fixtures/permit/permit_v1.json");

fn vector_file() -> PermitVectorFile {
    let file: PermitVectorFile =
        serde_json::from_str(PERMIT_VECTORS).expect("the committed vector file parses");
    assert_eq!(
        file.schema, PERMIT_VECTOR_SCHEMA,
        "a file of an unrecognized schema must be refused rather than interpreted"
    );
    file
}

/// The Connector's mapping from its own rejection onto the shared rule names.
///
/// Written from this side of the boundary on purpose. `None` means the Connector rejected for a
/// rule the permit dictionary does not name, which for a permit vector is itself a failure — the
/// caller reports it as one.
fn rule_name_of_form_error(error: &RequestFormError) -> Option<&'static str> {
    match error {
        RequestFormError::Permit(permit) => Some(match permit {
            PermitError::IdentityWidth { .. } => rule::IDENTITY_WIDTH,
            PermitError::TooManyAclDomainKeys { .. } => rule::TOO_MANY_ACL_DOMAIN_KEYS,
            PermitError::AclDomainKeysNotAscending { .. } => rule::ACL_DOMAIN_KEYS_NOT_ASCENDING,
            PermitError::DuplicateAclDomainKey { .. } => rule::DUPLICATE_ACL_DOMAIN_KEY,
            PermitError::DurationOutOfRange { .. } => rule::DURATION_OUT_OF_RANGE,
            PermitError::StartTimestampOutOfRange { .. } => rule::START_TIMESTAMP_OUT_OF_RANGE,
            PermitError::TransportKeyLength { .. } => rule::TRANSPORT_KEY_LENGTH,
            PermitError::UnknownKmsRoutingVersion { .. } => rule::UNKNOWN_KMS_ROUTING_VERSION,
            PermitError::KmsRoutingLength { .. } => rule::KMS_ROUTING_LENGTH,
            PermitError::SignatureMismatch => rule::SIGNATURE_MISMATCH,
            PermitError::UnusableUserPubkey => rule::UNUSABLE_USER_PUBKEY,
        }),
        // Rules of the request layer rather than the permit layer. A permit vector that lands here
        // is either malformed or the Connector is rejecting it for the wrong reason.
        RequestFormError::SignatureWidth { .. }
        | RequestFormError::EntryIdentityWidth { .. }
        | RequestFormError::AccessProofMalformed { .. }
        | RequestFormError::AccessProofTrailingBytes { .. }
        | RequestFormError::AccessProofTooManySiblings { .. }
        | RequestFormError::EmptyHandles => None,
    }
}

/// The Connector's mapping for the signature rule.
fn rule_name_of_signature_failure(failure: &AuthorizationFailure) -> Option<&'static str> {
    match failure {
        AuthorizationFailure::SignatureMismatch => Some(rule::SIGNATURE_MISMATCH),
        AuthorizationFailure::UnusableUserPubkey => Some(rule::UNUSABLE_USER_PUBKEY),
        _ => None,
    }
}

/// The synthetic handle entry a permit record is wrapped in. Two shapes exist so the control test
/// can show the choice does not matter.
#[derive(Clone, Copy)]
enum Wrapper {
    First,
    Second,
}

impl Wrapper {
    fn entry(self) -> SolanaHandleEntryWire {
        let (tag, owner_tag) = match self {
            Self::First => (0x10, 0x71),
            Self::Second => (0x20, 0x72),
        };
        SolanaHandleEntryWire {
            handle: handle(tag, FHE_TYPE_UINT64).to_vec(),
            owner: [owner_tag; 32].to_vec(),
            value_key: [owner_tag ^ 0xff; 32].to_vec(),
            proof_leaf_count: 0,
            access_proof: Vec::new(),
        }
    }
}

/// A record in the transport form the Connector receives.
fn wire_of(
    record: &PermitVector,
    file: &PermitVectorFile,
    wrapper: Wrapper,
) -> SolanaUserDecryptRequestWire {
    let permit = &record.permit;
    SolanaUserDecryptRequestWire {
        permit: PermitWireFields {
            user_pubkey: from_hex(&permit.user_pubkey).expect("hex"),
            transport_key: record
                .transport_key_bytes(file)
                .expect("the record names a key in the file's table"),
            allowed_acl_domain_keys: permit
                .allowed_acl_domain_keys
                .iter()
                .map(|key| from_hex(key).expect("hex"))
                .collect(),
            start_timestamp: permit.start_timestamp.parse().expect("a decimal string"),
            duration_seconds: permit.duration_seconds.parse().expect("a decimal string"),
            verifying_program_id: from_hex(&permit.verifying_program_id).expect("hex"),
            chain_id: permit.chain_id.parse().expect("a decimal string"),
            extra_data: from_hex(&permit.extra_data).expect("hex"),
        },
        signature: from_hex(&record.signature).expect("hex"),
        handles: vec![wrapper.entry()],
    }
}

/// What the Connector made of one record: accepted, or rejected by a named rule.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Accepted,
    Rejected(&'static str),
}

/// Runs one record through the Connector's permit path.
fn outcome_of(record: &PermitVector, file: &PermitVectorFile, wrapper: Wrapper) -> Outcome {
    match SolanaUserDecryptRequest::decode(&wire_of(record, file, wrapper)) {
        Err(error) => Outcome::Rejected(rule_name_of_form_error(&error).unwrap_or_else(|| {
            panic!(
                "record '{}' was rejected by a rule outside the permit dictionary: {error}",
                record.name
            )
        })),
        Ok(request) => match check_signature(&request) {
            Ok(()) => Outcome::Accepted,
            Err(failure) => {
                Outcome::Rejected(rule_name_of_signature_failure(&failure).unwrap_or_else(|| {
                    panic!(
                        "record '{}' failed the signature rule with an unrelated failure: {failure}",
                        record.name
                    )
                }))
            }
        },
    }
}

/// The set is non-empty and carries the schema it claims. Without this, every assertion below
/// would pass vacuously against an empty or renamed file.
#[test]
fn the_committed_permit_set_is_the_one_this_suite_checks() {
    let file = vector_file();

    assert!(
        file.vectors.len() >= 20,
        "the permit set has shrunk to {} records, which is not the normative set",
        file.vectors.len()
    );
    assert!(
        file.vectors
            .iter()
            .any(|record| record.result == VectorResult::Valid),
        "a set with no accepted record cannot show that the Connector accepts anything"
    );
}

/// Every record behaves as declared. This is the parity claim itself: the Connector's outcome, and
/// its reason, agree with the set for all of it.
#[test]
fn every_permit_vector_behaves_as_declared_through_the_connector() {
    let file = vector_file();

    for record in &file.vectors {
        let outcome = outcome_of(record, &file, Wrapper::First);
        match record.result {
            VectorResult::Valid | VectorResult::Acceptable => assert_eq!(
                outcome,
                Outcome::Accepted,
                "record '{}' is declared {:?} at the permit layer but the Connector rejected it",
                record.name,
                record.result
            ),
            VectorResult::Invalid => {
                let declared = record.rule.as_deref().unwrap_or_else(|| {
                    panic!("record '{}' rejects without naming a rule", record.name)
                });
                assert_eq!(
                    outcome,
                    Outcome::Rejected(
                        rule::ALL
                            .iter()
                            .copied()
                            .find(|name| *name == declared)
                            .unwrap_or_else(|| panic!(
                                "record '{}' names rule '{declared}', which is not in the shared \
                                 dictionary",
                                record.name
                            ))
                    ),
                    "record '{}' must be rejected by '{declared}'",
                    record.name
                );
            }
        }
    }
}

/// A rejecting record is derived from an accepted one by a single documented mutation, and the base
/// must itself be accepted here. Without this, "the Connector rejects it" could be true for a
/// reason that has nothing to do with the mutation — including the Connector rejecting the base too.
#[test]
fn the_base_of_every_rejecting_vector_is_accepted_by_the_connector() {
    let file = vector_file();

    for record in &file.vectors {
        let Some(base_name) = record.derived_from.as_deref() else {
            continue;
        };
        let base = file
            .vectors
            .iter()
            .find(|candidate| candidate.name == base_name)
            .unwrap_or_else(|| {
                panic!(
                    "record '{}' is derived from '{base_name}', which is not in the set",
                    record.name
                )
            });
        assert_eq!(
            outcome_of(base, &file, Wrapper::First),
            Outcome::Accepted,
            "record '{}' derives from '{base_name}', which the Connector does not accept — so its \
             rejection says nothing about the mutation",
            record.name
        );
    }
}

/// Every rule the set exercises is a rule the Connector actually produces. A Connector that mapped
/// two distinct violations onto one name, or never produced a name at all, would pass the
/// record-by-record check above while quietly having no opinion about part of the dictionary.
#[test]
fn every_rule_the_set_exercises_is_one_the_connector_produces() {
    let file = vector_file();
    let mut declared = BTreeSet::new();
    let mut produced = BTreeSet::new();

    for record in &file.vectors {
        if let Some(name) = record.rule.as_deref() {
            declared.insert(name.to_owned());
        }
        if let Outcome::Rejected(name) = outcome_of(record, &file, Wrapper::First) {
            produced.insert(name.to_owned());
        }
    }

    assert_eq!(
        declared, produced,
        "the rules the set declares and the rules the Connector produces have diverged"
    );
}

/// The set contains no handle entries, so each record is wrapped in a synthetic one. Running the
/// whole set again with a different wrapper must change nothing: if it did, the wrapper — and not
/// the permit — would be deciding outcomes, and the parity claim would be about the wrong thing.
#[test]
fn the_synthetic_handle_entry_decides_nothing() {
    let file = vector_file();

    for record in &file.vectors {
        assert_eq!(
            outcome_of(record, &file, Wrapper::First),
            outcome_of(record, &file, Wrapper::Second),
            "record '{}' depends on the synthetic handle entry it was wrapped in",
            record.name
        );
    }
}

/// The signature rule is reached only for records whose typed form is well formed. A record
/// declaring a typed-form rule must be refused before any signature work, which is what makes the
/// set's "the signature is copied from the base record" note true rather than aspirational.
#[test]
fn typed_form_rules_are_decided_before_the_signature() {
    let file = vector_file();
    let typed_form_rules = [
        rule::IDENTITY_WIDTH,
        rule::TOO_MANY_ACL_DOMAIN_KEYS,
        rule::ACL_DOMAIN_KEYS_NOT_ASCENDING,
        rule::DUPLICATE_ACL_DOMAIN_KEY,
        rule::DURATION_OUT_OF_RANGE,
        rule::START_TIMESTAMP_OUT_OF_RANGE,
        rule::TRANSPORT_KEY_LENGTH,
        rule::UNKNOWN_KMS_ROUTING_VERSION,
        rule::KMS_ROUTING_LENGTH,
    ];

    for record in &file.vectors {
        let Some(declared) = record.rule.as_deref() else {
            continue;
        };
        if !typed_form_rules.contains(&declared) {
            continue;
        }
        let error = SolanaUserDecryptRequest::decode(&wire_of(record, &file, Wrapper::First))
            .expect_err("a typed-form violation is refused by decoding alone");
        assert_eq!(
            rule_name_of_form_error(&error),
            Some(declared),
            "record '{}' must be refused by decoding, before the signature is looked at",
            record.name
        );
    }
}
