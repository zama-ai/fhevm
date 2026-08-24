//! Runs the committed `extraData` byte-layout vectors against this crate's carrier codec.
//!
//! The vectors live in `solana/test-fixtures/user-decrypt/extra_data_v1.json` next to the other
//! cross-implementation fixture sets and are shared with the TypeScript mirror
//! (`sdk/js-sdk/src/core/coprocessor/SolanaUserDecrypt-p.test.ts`). They are hand-committed
//! literals — deliberately few, with no generator: the layouts are frozen behind their version
//! bytes, so a change that moves these bytes is a protocol change, not a fixture refresh.
//!
//! Only the `extraData` half of that fixture set runs here. Its sibling,
//! `signing_message_v1.json`, pins the retired ed25519 signing message and keeps the TypeScript
//! mirror as its only consumer until the SDK moves to the host-generic form; the Rust side of
//! user decrypt is pinned by the connector-auth vectors instead. The carrier itself is owned by
//! public decrypt now — see the `solana_extra_data` module docs for its status and removal
//! condition.

use connector_utils::types::solana_extra_data::{
    encode_solana_extra_data_context_only, encode_solana_extra_data_mmr_proof,
    parse_solana_mmr_proof_extra_data,
};
use serde::Deserialize;

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/../../../solana/test-fixtures/user-decrypt/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("can't read {path}: {err}"))
}

fn bytes(hex: &str) -> Vec<u8> {
    alloy::hex::decode(hex).expect("fixture hex decodes")
}

fn key32(hex: &str) -> [u8; 32] {
    bytes(hex).try_into().expect("fixture key is 32 bytes")
}

#[derive(Deserialize)]
struct ExtraDataFile {
    schema: String,
    records: Vec<ExtraDataRecord>,
    malformed: Vec<MalformedRecord>,
}

#[derive(Deserialize)]
struct ExtraDataRecord {
    name: String,
    input: ExtraDataInput,
    blob_hex: String,
}

#[derive(Deserialize)]
struct ExtraDataInput {
    context_id_hex: String,
    acl_value_key_hex: Option<String>,
    proof_slot: Option<String>,
    mmr_proof_hex: Option<String>,
}

#[derive(Deserialize)]
struct MalformedRecord {
    name: String,
    blob_hex: String,
}

#[test]
fn extra_data_vectors_encode_and_round_trip() {
    let file: ExtraDataFile =
        serde_json::from_str(&fixture("extra_data_v1.json")).expect("fixture parses");
    assert_eq!(file.schema, "zama-solana-user-decrypt-extra-data/v1");
    assert!(!file.records.is_empty());

    for record in &file.records {
        let context_id = key32(&record.input.context_id_hex);
        let expected_blob = bytes(&record.blob_hex);

        match &record.input.acl_value_key_hex {
            // The context-only version is encoder-only on this side: it has no parser to
            // round-trip through, and the strict proof-tail parser must reject it — the
            // malformed list below pins that rejection.
            None => {
                assert_eq!(
                    encode_solana_extra_data_context_only(context_id),
                    expected_blob,
                    "{}: encoder must produce the committed blob",
                    record.name
                );
            }
            Some(value_key_hex) => {
                let acl_value_key = key32(value_key_hex);
                let proof_slot: u64 = record
                    .input
                    .proof_slot
                    .as_deref()
                    .expect("a proof-tail record carries proof_slot")
                    .parse()
                    .expect("fixture proof_slot is a u64");
                let mmr_proof = bytes(
                    record
                        .input
                        .mmr_proof_hex
                        .as_deref()
                        .expect("a proof-tail record carries mmr_proof_hex"),
                );

                assert_eq!(
                    encode_solana_extra_data_mmr_proof(
                        context_id,
                        acl_value_key,
                        proof_slot,
                        &mmr_proof
                    ),
                    expected_blob,
                    "{}: encoder must produce the committed blob",
                    record.name
                );

                // Every committed proof-tail blob must survive the strict parser with its
                // fields intact.
                let parsed =
                    parse_solana_mmr_proof_extra_data(&expected_blob).unwrap_or_else(|| {
                        panic!("{}: the strict parser must accept this blob", record.name)
                    });
                assert_eq!(parsed.context_id, context_id, "{}", record.name);
                assert_eq!(parsed.acl_value_key, acl_value_key, "{}", record.name);
                assert_eq!(parsed.proof_slot, proof_slot, "{}", record.name);
                assert_eq!(parsed.mmr_proof_bytes, mmr_proof, "{}", record.name);
            }
        }
    }

    assert!(!file.malformed.is_empty());
    for record in &file.malformed {
        assert!(
            parse_solana_mmr_proof_extra_data(&bytes(&record.blob_hex)).is_none(),
            "{}: the strict parser must reject this blob",
            record.name
        );
    }
}
