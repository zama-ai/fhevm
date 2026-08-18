//! Runs the committed v3 Solana envelope records against the real endpoint types.
//!
//! `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json` is the HTTP seam between the SDK and
//! this relayer, and it is consumed from both sides: the SDK builds its request and compares it to
//! the same records this file feeds to `SolanaUserDecryptRequestJson`. That is the point of a shared
//! file rather than two independent test suites — a key renamed on one side and mirrored in that
//! side's own test would look green twice and fail in production.
//!
//! Nothing here is generated, and nothing is asserted against a hand-copied permit: the permit half
//! of every record is derived from the permit canon
//! (`solana/test-fixtures/permit/permit_v1.json`, record `reference-permit-two-domains`) through the
//! derivation the fixture states, so these records cannot disagree with the canon about a signature.
//!
//! Each rejecting record names the layer that must refuse it. The layers are not interchangeable: a
//! record that should die in deserialization but survives into the conversion has found a hole in
//! the strictness of the wire type, which is exactly what `deny_unknown_fields` is there to close.

// The schema lives with the fixtures, because five implementations include the same file. Each
// consumer uses the part of it that its own layer needs, so unused helpers are expected here rather
// than a sign of drift.
#[allow(dead_code)]
#[path = "../../solana/test-fixtures/permit/permit_vectors.rs"]
mod permit_vectors;

use alloy::primitives::U256;
use fhevm_relayer::core::event::UserDecryptRequest;
use fhevm_relayer::host::handle_chain_id::extract_chain_id_from_u256;
use fhevm_relayer::http::endpoints::v3::types::SolanaUserDecryptRequestJson;
use permit_vectors::{PermitVectorFile, PERMIT_VECTOR_SCHEMA};
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;
use validator::Validate;

/// Schema identifier of the envelope fixture; an unrecognized one is refused rather than guessed at.
const ENVELOPE_SCHEMA: &str = "zama-solana-user-decrypt-envelope/v1";

/// The permit record every envelope record is built on.
const PERMIT_RECORD: &str = "reference-permit-two-domains";

fn fixture(relative: &str) -> String {
    let path = format!(
        "{}/../solana/test-fixtures/{relative}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("can't read {path}: {err}"))
}

/// The layer that must refuse a record. Closed vocabulary: an unknown value is a fixture the test
/// does not understand, and guessing which layer was meant would let a record pass for the wrong
/// reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RejectedBy {
    /// The typed envelope refuses to deserialize.
    JsonShape,
    /// It deserializes, and the payload validator refuses it.
    PayloadRules,
    /// It validates, and the conversion into the internal request refuses it.
    RequestDecode,
    /// It converts, and a handle's embedded chain id is not one this relayer serves.
    HostChainSupport,
}

impl RejectedBy {
    fn parse(name: &str) -> Self {
        match name {
            "json-shape" => Self::JsonShape,
            "payload-rules" => Self::PayloadRules,
            "request-decode" => Self::RequestDecode,
            "host-chain-support" => Self::HostChainSupport,
            other => panic!("the fixture names an unknown rejection layer: {other}"),
        }
    }
}

/// The permit half of every request, derived from the canon exactly as the fixture states.
struct PermitHalf {
    payload: Map<String, Value>,
    signature: String,
    chain_id: u64,
    transport_key_hex: String,
    extra_data_hex: String,
}

fn permit_half() -> PermitHalf {
    let file: PermitVectorFile =
        serde_json::from_str(&fixture("permit/permit_v1.json")).expect("permit canon parses");
    assert_eq!(file.schema, PERMIT_VECTOR_SCHEMA);

    let record = file
        .vectors
        .iter()
        .find(|vector| vector.name == PERMIT_RECORD)
        .unwrap_or_else(|| panic!("the permit canon carries no record named {PERMIT_RECORD}"));
    let transport_key_hex = file
        .transport_keys
        .get(&record.permit.transport_key)
        .expect("the record's transport key is in the table")
        .clone();

    let mut payload = Map::new();
    payload.insert(
        "userPubkey".to_string(),
        json!(format!("0x{}", record.permit.user_pubkey)),
    );
    payload.insert(
        "transportKey".to_string(),
        json!(format!("0x{transport_key_hex}")),
    );
    payload.insert(
        "allowedAclDomainKeys".to_string(),
        json!(record
            .permit
            .allowed_acl_domain_keys
            .iter()
            .map(|key| format!("0x{key}"))
            .collect::<Vec<_>>()),
    );
    payload.insert(
        "requestValidity".to_string(),
        json!({
            "startTimestamp": record.permit.start_timestamp,
            "durationSeconds": record.permit.duration_seconds,
        }),
    );
    payload.insert(
        "verifyingProgramId".to_string(),
        json!(format!("0x{}", record.permit.verifying_program_id)),
    );
    payload.insert("chainId".to_string(), json!(record.permit.chain_id));
    payload.insert(
        "extraData".to_string(),
        json!(format!("0x{}", record.permit.extra_data)),
    );

    PermitHalf {
        payload,
        signature: format!("0x{}", record.signature),
        chain_id: record.permit.chain_id.parse().expect("decimal u64"),
        transport_key_hex,
        extra_data_hex: record.permit.extra_data.clone(),
    }
}

/// The envelope fixture, and the permit half its records are composed with.
struct Fixture {
    file: Value,
    permit: PermitHalf,
}

impl Fixture {
    fn load() -> Self {
        let file: Value = serde_json::from_str(&fixture("user-decrypt/relayer_envelope_v1.json"))
            .expect("envelope fixture parses");
        assert_eq!(file["schema"], ENVELOPE_SCHEMA);
        Self {
            file,
            permit: permit_half(),
        }
    }

    fn records(&self, key: &str) -> &Vec<Value> {
        self.file[key]
            .as_array()
            .unwrap_or_else(|| panic!("the fixture carries a `{key}` list"))
    }

    /// Composes one record into the request body exactly as the fixture's `composition` states.
    fn compose(&self, record: &Value) -> Value {
        let mut payload = self.permit.payload.clone();
        payload.insert("handles".to_string(), record["handles"].clone());
        if let Some(extra) = record.get("payload_extra").and_then(Value::as_object) {
            for (key, value) in extra {
                payload.insert(key.clone(), value.clone());
            }
        }
        json!({
            "attestationType": self.file["attestation_type"].clone(),
            "attestedPayload": Value::Object(payload),
            "signature": self.permit.signature.clone(),
        })
    }
}

fn name_of(record: &Value) -> &str {
    record["name"].as_str().expect("a record names itself")
}

////////////////////////////////////////////////////////////////////////////////

/// Every accepted record travels the whole seam: it deserializes strictly, validates, and converts
/// into the Solana request the gateway call is built from — including the permit signature check the
/// conversion performs, which is what makes these records more than a JSON-shape test.
#[test]
fn every_accepted_record_becomes_a_solana_request() {
    let fixture = Fixture::load();
    let accepted = fixture.records("accepted");
    assert!(!accepted.is_empty());

    for record in accepted {
        let name = name_of(record);
        let body = fixture.compose(record);

        let parsed: SolanaUserDecryptRequestJson =
            serde_json::from_value(body).unwrap_or_else(|err| panic!("{name}: should parse: {err}"));
        parsed
            .validate()
            .unwrap_or_else(|err| panic!("{name}: should validate: {err}"));

        let handle_count = parsed.attested_payload.handles.len();
        let request = UserDecryptRequest::try_from(parsed)
            .unwrap_or_else(|err| panic!("{name}: should convert: {err}"));

        match &request {
            UserDecryptRequest::SolanaSrfc38V1 {
                ct_handles,
                public_key,
                extra_data,
                solana_request,
                ..
            } => {
                assert_eq!(ct_handles.len(), handle_count, "{name}: handle count");
                assert_eq!(
                    hex::encode(public_key.as_ref()),
                    fixture.permit.transport_key_hex,
                    "{name}: the transport key travels verbatim"
                );
                assert_eq!(
                    hex::encode(extra_data.as_ref()),
                    fixture.permit.extra_data_hex,
                    "{name}: the signed routing field travels verbatim"
                );
                assert!(
                    !solana_request.is_empty(),
                    "{name}: the encoded request carries the permit and the evidence"
                );
            }
            other => panic!("{name}: converted into the wrong variant: {other:?}"),
        }
    }
}

/// Every accepted record's handles belong to the host chain the permit is signed for. The fixture's
/// handles are hand-written, so this also keeps a typo in one of them from being read as a chain-id
/// bug in the code under test.
#[test]
fn accepted_handles_belong_to_the_signed_host_chain() {
    let fixture = Fixture::load();

    for record in fixture.records("accepted") {
        let name = name_of(record);
        let parsed: SolanaUserDecryptRequestJson =
            serde_json::from_value(fixture.compose(record)).expect("accepted record parses");
        let request = UserDecryptRequest::try_from(parsed).expect("accepted record converts");

        for handle in request.ct_handles() {
            assert_eq!(
                extract_chain_id_from_u256(handle),
                fixture.permit.chain_id,
                "{name}: a handle names another host chain"
            );
        }
    }
}

/// Every rejecting record is refused, and refused by the layer it names — not merely somewhere.
#[test]
fn every_rejecting_record_is_refused_by_the_layer_it_names() {
    let fixture = Fixture::load();
    let rejected = fixture.records("rejected");
    assert!(!rejected.is_empty());

    for record in rejected {
        let name = name_of(record);
        let declared = RejectedBy::parse(
            record["rejected_by"]
                .as_str()
                .unwrap_or_else(|| panic!("{name}: must name its rejection layer")),
        );
        let body = fixture.compose(record);

        let parsed = match serde_json::from_value::<SolanaUserDecryptRequestJson>(body) {
            Err(_) => {
                assert_eq!(
                    declared,
                    RejectedBy::JsonShape,
                    "{name}: refused while deserializing, but names another layer"
                );
                continue;
            }
            Ok(parsed) => {
                assert_ne!(
                    declared,
                    RejectedBy::JsonShape,
                    "{name}: names the wire shape, and the wire type accepted it"
                );
                parsed
            }
        };

        if parsed.validate().is_err() {
            assert_eq!(
                declared,
                RejectedBy::PayloadRules,
                "{name}: refused by the validator, but names another layer"
            );
            continue;
        }
        assert_ne!(
            declared,
            RejectedBy::PayloadRules,
            "{name}: names the validator, and the validator accepted it"
        );

        let request = match UserDecryptRequest::try_from(parsed) {
            Err(_) => {
                assert_eq!(
                    declared,
                    RejectedBy::RequestDecode,
                    "{name}: refused while converting, but names another layer"
                );
                continue;
            }
            Ok(request) => {
                assert_ne!(
                    declared,
                    RejectedBy::RequestDecode,
                    "{name}: names the conversion, and the conversion accepted it"
                );
                request
            }
        };

        // What is left is refused above the conversion, on the handle's host chain. The rejection
        // itself belongs to the configured chain-id check in the handler; what this asserts is the
        // condition that check fires on, and that nothing below it quietly accepted the mismatch.
        assert_eq!(declared, RejectedBy::HostChainSupport, "{name}: was accepted");
        let foreign: Vec<&U256> = request
            .ct_handles()
            .into_iter()
            .filter(|handle| extract_chain_id_from_u256(handle) != fixture.permit.chain_id)
            .collect();
        assert!(
            !foreign.is_empty(),
            "{name}: names the host-chain layer, and every handle belongs to the signed chain"
        );
    }
}

/// The fixture's rejection vocabulary is closed and fully exercised: every layer it documents is
/// reached by some record, and no record names a layer the file does not document.
#[test]
fn the_fixture_exercises_every_layer_it_documents() {
    let fixture = Fixture::load();

    let documented: BTreeSet<RejectedBy> = fixture.file["rejected_by_values"]
        .as_object()
        .expect("the fixture documents its rejection layers")
        .keys()
        .map(|name| RejectedBy::parse(name))
        .collect();
    let exercised: BTreeSet<RejectedBy> = fixture
        .records("rejected")
        .iter()
        .map(|record| {
            RejectedBy::parse(record["rejected_by"].as_str().expect("names its layer"))
        })
        .collect();

    assert_eq!(
        documented, exercised,
        "the documented and exercised rejection layers differ"
    );
}

/// Both access modes are represented among the accepted records: an empty proof for current access
/// and a borsh MMR proof for superseded access. A fixture that lost one of them would still pass
/// every assertion above while covering half the seam.
#[test]
fn the_accepted_records_cover_both_access_modes() {
    let fixture = Fixture::load();

    let mut modes = BTreeSet::new();
    for record in fixture.records("accepted") {
        for entry in record["handles"].as_array().expect("handles is a list") {
            let proof = entry["accessProof"].as_str().expect("accessProof is a string");
            modes.insert(proof == "0x");
        }
    }

    assert_eq!(
        modes,
        BTreeSet::from([true, false]),
        "the accepted records must carry both an empty and a non-empty access proof"
    );
}
