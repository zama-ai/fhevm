//! The relayer's `hostPayload` encoder, pinned byte-for-byte against the normative
//! connector-auth vectors (spec §12).
//!
//! The relayer encodes the canonical `hostPayload` that the gateway carries opaquely; each KMS
//! party's connector decodes it. There is no shared codec crate — the two are independent
//! implementations of one byte layout, so they can only be kept in agreement by a common set of
//! frozen vectors. This test is the relayer's half of that agreement: for every vector,
//! `encode_host_payload(fields) == host_payload_hex`, the exact blob the connector's decoder is
//! pinned to consume. A drift in either implementation fails here with both blobs visible.

#[allow(dead_code)]
#[path = "../../solana/test-fixtures/connector-auth/connector_auth_vectors.rs"]
mod schema;

use fhevm_relayer::core::solana_host_payload::{encode_host_payload, SolanaHandleWire};
use schema::{from_hex, to_hex, ConnectorAuthVectorFile, CONNECTOR_AUTH_VECTOR_SCHEMA};
use std::path::{Path, PathBuf};
use zama_solana_permit::PermitWireFields;

fn vector_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../solana/test-fixtures/connector-auth/connector_auth_v1.json")
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    from_hex(hex).unwrap_or_else(|| panic!("fixture field is not valid hex: {hex}"))
}

fn decimal_u64(value: &str) -> u64 {
    value
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("fixture field is not a u64: {value}"))
}

#[test]
fn relayer_host_payload_encoder_matches_the_connector_auth_vectors() {
    let raw =
        std::fs::read_to_string(vector_path()).expect("the connector-auth vector file exists");
    let file: ConnectorAuthVectorFile =
        serde_json::from_str(&raw).expect("the connector-auth vector file parses");
    assert_eq!(
        file.schema, CONNECTOR_AUTH_VECTOR_SCHEMA,
        "unexpected vector-file schema"
    );
    assert!(
        !file.vectors.is_empty(),
        "the vector file must carry at least one record"
    );

    for vector in &file.vectors {
        let permit_wire = &vector.request.permit;
        let transport_key = vector
            .transport_key_bytes(&file)
            .unwrap_or_else(|| panic!("{}: transport key not found in the table", vector.name));

        let permit = PermitWireFields {
            user_pubkey: hex_bytes(&permit_wire.user_pubkey),
            transport_key,
            allowed_acl_domain_keys: permit_wire
                .allowed_acl_domain_keys
                .iter()
                .map(|k| hex_bytes(k))
                .collect(),
            start_timestamp: decimal_u64(&permit_wire.start_timestamp),
            duration_seconds: decimal_u64(&permit_wire.duration_seconds),
            verifying_program_id: hex_bytes(&permit_wire.verifying_program_id),
            chain_id: decimal_u64(&permit_wire.chain_id),
            extra_data: hex_bytes(&permit_wire.extra_data),
        };

        let signature = hex_bytes(&vector.request.signature);
        let handles: Vec<SolanaHandleWire> = vector
            .request
            .handles
            .iter()
            .map(|entry| SolanaHandleWire {
                handle: hex_bytes(&entry.handle),
                owner: hex_bytes(&entry.owner),
                encrypted_value_id: hex_bytes(&entry.encrypted_value_id),
                proof_leaf_count: decimal_u64(&entry.proof_leaf_count),
                access_proof: hex_bytes(&entry.access_proof),
            })
            .collect();

        let encoded = encode_host_payload(&permit, &signature, &handles).unwrap_or_else(|error| {
            panic!("{}: the vector fields must serialize: {error}", vector.name)
        });
        assert_eq!(
            to_hex(&encoded),
            vector.request.host_payload,
            "{}: relayer-encoded hostPayload does not match the frozen vector",
            vector.name
        );
    }
}
