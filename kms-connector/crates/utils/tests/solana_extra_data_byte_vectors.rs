//! Runs the committed `extraData` byte-layout vectors against this crate's carrier codec.
//!
//! The vectors live in `solana/test-fixtures/user-decrypt/extra_data_v1.json` next to the other
//! cross-implementation fixture sets and are shared with the TypeScript mirror
//! (`sdk/js-sdk/src/solana/actions/publicDecryptCertificate.test.ts`). They are hand-committed
//! literals — deliberately few, with no generator: the layout is frozen behind its version byte,
//! so a change that moves these bytes is a protocol change, not a fixture refresh.
//!
//! The carrier names the encrypted value account a public handle lives in and nothing else; the
//! `PublicDecryptLeaf` proof is fetched by the connector, never carried. The `malformed` section
//! pins the strictness that makes the fixed width a real property: a longer blob, a shorter one,
//! and any other version byte are all refused.

use connector_utils::types::solana_extra_data::{
    SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN, encode_solana_public_decrypt_extra_data,
    parse_solana_public_decrypt_extra_data,
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
    encrypted_value_account_hex: String,
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
    assert_eq!(file.schema, "zama-solana-public-decrypt-extra-data/v1");
    assert!(!file.records.is_empty());

    for record in &file.records {
        let context_id = key32(&record.input.context_id_hex);
        let encrypted_value_account = key32(&record.input.encrypted_value_account_hex);
        let expected_blob = bytes(&record.blob_hex);

        assert_eq!(
            expected_blob.len(),
            SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN,
            "{}: the committed blob has the fixed width",
            record.name
        );
        assert_eq!(
            encode_solana_public_decrypt_extra_data(context_id, encrypted_value_account),
            expected_blob,
            "{}: encoder must produce the committed blob",
            record.name
        );
        let parsed = parse_solana_public_decrypt_extra_data(&expected_blob)
            .unwrap_or_else(|| panic!("{}: the strict parser must accept this blob", record.name));
        assert_eq!(parsed.context_id, context_id, "{}", record.name);
        assert_eq!(
            parsed.encrypted_value_account, encrypted_value_account,
            "{}",
            record.name
        );
    }

    assert!(!file.malformed.is_empty());
    for record in &file.malformed {
        assert!(
            parse_solana_public_decrypt_extra_data(&bytes(&record.blob_hex)).is_none(),
            "{}: the strict parser must reject this blob",
            record.name
        );
    }
}
