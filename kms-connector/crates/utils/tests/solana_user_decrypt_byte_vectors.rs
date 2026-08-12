//! Runs the committed user-decrypt byte-layout vectors against this crate's codecs.
//!
//! The vectors live in `solana/test-fixtures/user-decrypt/` next to the other cross-implementation
//! fixture sets and are shared with the TypeScript mirror
//! (`sdk/js-sdk/src/core/coprocessor/SolanaUserDecrypt-p.test.ts`). They are hand-committed
//! literals — deliberately few, with no generator: the signing-message layout is frozen behind its
//! domain tag and the extraData layouts behind their version bytes, so a change that moves these
//! bytes is a protocol change, not a fixture refresh.

use connector_utils::types::solana_extra_data::{
    SolanaUserDecryptSigningInput, encode_solana_extra_data_context_only,
    encode_solana_extra_data_mmr_proof, parse_solana_mmr_proof_extra_data,
    parse_solana_user_decrypt_extra_data, solana_user_decrypt_signing_message,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};

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
struct SigningMessageFile {
    schema: String,
    records: Vec<SigningMessageRecord>,
}

#[derive(Deserialize)]
struct SigningMessageRecord {
    name: String,
    input: SigningMessageInput,
    expected: SigningMessageExpected,
}

#[derive(Deserialize)]
struct SigningMessageInput {
    contracts_chain_id: String,
    public_key_hex: String,
    handles_hex: Vec<String>,
    identity_hex: String,
    context_id_hex: String,
    nonce_hex: String,
    allowed_acl_domain_keys_hex: Vec<String>,
    start_timestamp: String,
    duration_seconds: String,
    acl_value_key_hex: String,
    proof_slot: String,
    mmr_proof_hex: String,
}

#[derive(Deserialize)]
struct SigningMessageExpected {
    commitment_hex: String,
    message_utf8: String,
}

#[test]
fn signing_message_vectors_match() {
    let file: SigningMessageFile =
        serde_json::from_str(&fixture("signing_message_v1.json")).expect("fixture parses");
    assert_eq!(file.schema, "zama-solana-user-decrypt-signing-message/v1");
    assert!(!file.records.is_empty());

    for record in &file.records {
        let input = &record.input;
        let public_key = bytes(&input.public_key_hex);
        let handles: Vec<[u8; 32]> = input.handles_hex.iter().map(|h| key32(h)).collect();
        let identity = key32(&input.identity_hex);
        let context_id = key32(&input.context_id_hex);
        let nonce = key32(&input.nonce_hex);
        let domain_keys: Vec<[u8; 32]> = input
            .allowed_acl_domain_keys_hex
            .iter()
            .map(|k| key32(k))
            .collect();
        let acl_value_key = key32(&input.acl_value_key_hex);
        let mmr_proof = bytes(&input.mmr_proof_hex);

        let message = solana_user_decrypt_signing_message(&SolanaUserDecryptSigningInput {
            contracts_chain_id: input.contracts_chain_id.parse().unwrap(),
            public_key: &public_key,
            handles: &handles,
            identity: &identity,
            context_id: &context_id,
            nonce: &nonce,
            allowed_acl_domain_keys: &domain_keys,
            start_timestamp: input.start_timestamp.parse().unwrap(),
            duration_seconds: input.duration_seconds.parse().unwrap(),
            acl_value_key: &acl_value_key,
            mmr_proof_bytes: &mmr_proof,
            proof_slot: input.proof_slot.parse().unwrap(),
        });

        assert_eq!(
            message,
            record.expected.message_utf8.as_bytes(),
            "{}: signing message must match the committed vector",
            record.name
        );

        // The commitment is diagnostic (neither implementation exposes it), so keep it honest:
        // its sha256 must be the digest the message carries.
        let digest = Sha256::digest(bytes(&record.expected.commitment_hex));
        let digest_hex = record
            .expected
            .message_utf8
            .rsplit(' ')
            .next()
            .expect("message ends with the digest");
        assert_eq!(
            alloy::hex::encode(digest),
            digest_hex,
            "{}: commitment_hex must hash to the digest inside the message",
            record.name
        );
    }
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

        let blob = match &record.input.acl_value_key_hex {
            None => encode_solana_extra_data_context_only(context_id),
            Some(value_key_hex) => encode_solana_extra_data_mmr_proof(
                context_id,
                key32(value_key_hex),
                record.input.proof_slot.as_deref().unwrap().parse().unwrap(),
                &bytes(record.input.mmr_proof_hex.as_deref().unwrap()),
            ),
        };
        assert_eq!(
            blob, expected_blob,
            "{}: encoder must produce the committed blob",
            record.name
        );

        // Every committed blob must survive the lenient parser with its fields intact.
        let parsed = parse_solana_user_decrypt_extra_data(&expected_blob);
        assert_eq!(parsed.context_id, context_id, "{}", record.name);
        match &record.input.acl_value_key_hex {
            None => {
                assert_eq!(parsed.acl_value_key, [0u8; 32], "{}", record.name);
                assert_eq!(parsed.proof_slot, 0, "{}", record.name);
                assert!(parsed.mmr_proof_bytes.is_empty(), "{}", record.name);
            }
            Some(value_key_hex) => {
                assert_eq!(
                    parsed.acl_value_key,
                    key32(value_key_hex),
                    "{}",
                    record.name
                );
                assert_eq!(
                    parsed.proof_slot,
                    record
                        .input
                        .proof_slot
                        .as_deref()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    "{}",
                    record.name
                );
                assert_eq!(
                    parsed.mmr_proof_bytes,
                    bytes(record.input.mmr_proof_hex.as_deref().unwrap()),
                    "{}",
                    record.name
                );
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
