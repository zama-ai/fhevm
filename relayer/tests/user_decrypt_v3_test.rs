//! Integration tests for the v3 `/v3/user-decrypt` endpoint (RFC016
//! unified user-decryption). Uses the in-process `TestSetup` harness and
//! the `ethereum_rpc_mock` JSON-RPC mock — the same plumbing v2 tests use.
//!
//! Happy-path E2E (with mocked gateway events emitting the unified
//! `UserDecryptionRequest_1` event and the shared `UserDecryptionResponse`
//! shares) lives in `tests/user_decrypt_v3_e2e_test.rs` once the mock is
//! extended for the unified selector + event. This file covers everything
//! that doesn't require mock changes: validation, dispatch by
//! `attestationType`, schema rejection.

mod common;

use crate::common::utils::TestSetup;
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::Address;
use rand::{rng, RngExt};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v3_user_decrypt_post_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v3/user-decrypt", setup.http_port)
    }

    pub fn random_address() -> Address {
        utils::random_address()
    }

    pub fn random_handle() -> String {
        utils::random_handle()
    }

    pub fn random_0x_hex(byte_len: usize) -> String {
        let mut rng = rng();
        let mut s = String::from("0x");
        for _ in 0..(byte_len * 2) {
            s.push_str(&format!("{:x}", rng.random_range(0..16)));
        }
        s
    }

    /// Build a syntactically valid RFC016 v3 envelope. Callers can mutate
    /// the returned `serde_json::Value` to test rejection paths.
    pub fn create_v3_envelope() -> serde_json::Value {
        let user_address = random_address();
        let contract_address = random_address();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        json!({
            "attestationType": "eip712-unified-user-decrypt-v1",
            "attestedPayload": {
                "version": "2.0",
                "type": "user_decryption",
                "handles": [{
                    "ctHandle": random_handle(),
                    "contractAddress": format!("{:?}", contract_address),
                    "ownerAddress": format!("{:?}", user_address),
                }],
                "userAddress": format!("{:?}", user_address),
                "allowedContracts": [format!("{:?}", contract_address)],
                "requestValidity": {
                    "startTimestamp": (now - 1).to_string(),
                    "durationSeconds": "604800",
                },
                "publicKey": random_0x_hex(32),
                "extraData": "0x00",
            },
            "signature": random_0x_hex(65),
        })
    }
}

// ---------------------------------------------------------------------------
// Happy-path accept tests (POST → 202)
// ---------------------------------------------------------------------------

/// v3 accepts a single direct-access handle (`ownerAddress == userAddress`).
#[tokio::test]
async fn v3_accepts_direct_handle() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_v3_envelope();

    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(
        response.status(),
        reqwest::StatusCode::ACCEPTED,
        "expected 202, got {}: {:?}",
        response.status(),
        response.text().await
    );

    setup.shutdown().await;
}

/// v3 accepts a mixed batch: one direct (`ownerAddress == userAddress`)
/// and one delegated (`ownerAddress != userAddress`) handle entry in one
/// request.
#[tokio::test]
async fn v3_accepts_mixed_direct_and_delegated_handles() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let mut payload = helpers::create_v3_envelope();
    let direct_owner = payload["attestedPayload"]["userAddress"]
        .as_str()
        .unwrap()
        .to_string();
    let delegated_owner = format!("{:?}", helpers::random_address());
    let contract = format!("{:?}", helpers::random_address());

    payload["attestedPayload"]["handles"] = json!([
        {
            "ctHandle": helpers::random_handle(),
            "contractAddress": contract,
            "ownerAddress": direct_owner,
        },
        {
            "ctHandle": helpers::random_handle(),
            "contractAddress": contract,
            "ownerAddress": delegated_owner,
        }
    ]);

    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    setup.shutdown().await;
}

/// v3 accepts the permissive mode where `allowedContracts` is an empty
/// list. RFC016 explicitly allows this.
#[tokio::test]
async fn v3_accepts_empty_allowed_contracts() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let mut payload = helpers::create_v3_envelope();
    payload["attestedPayload"]["allowedContracts"] = json!([]);

    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    setup.shutdown().await;
}

/// v3 accepts a multi-element `allowedContracts` list.
#[tokio::test]
async fn v3_accepts_multiple_allowed_contracts() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let mut payload = helpers::create_v3_envelope();
    payload["attestedPayload"]["allowedContracts"] = json!([
        format!("{:?}", helpers::random_address()),
        format!("{:?}", helpers::random_address()),
        format!("{:?}", helpers::random_address()),
    ]);

    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    setup.shutdown().await;
}

/// v3 accepts an arbitrary `signature` blob (the relayer forwards it
/// verbatim; it never verifies). 65-byte random hex is the realistic
/// length but the relayer doesn't enforce that.
#[tokio::test]
async fn v3_accepts_arbitrary_signature_blob() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let mut payload = helpers::create_v3_envelope();
    // 100-byte arbitrary signature — not a valid EIP-712 signature.
    payload["signature"] = json!(helpers::random_0x_hex(100));

    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    setup.shutdown().await;
}

// ---------------------------------------------------------------------------
// Envelope-level rejection (attestationType, signature)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn v3_rejects_unknown_attestation_type() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        with_invalid_field("attestationType", json!("ed25519-solana-user-decrypt-v1")),
        expect_v2_validation_error("attestationType", "Unsupported attestationType"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_missing_attestation_type() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            p.as_object_mut().unwrap().remove("attestationType");
        },
        expect_v2_validation_error("attestationType", ""),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_empty_signature() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        with_invalid_field("signature", json!("0x")),
        expect_v2_validation_error("signature", ""),
    )
    .await;

    setup.shutdown().await;
}

// ---------------------------------------------------------------------------
// Inner-payload rejection (version, type, handles, requestValidity)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn v3_rejects_wrong_version() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            p["attestedPayload"]["version"] = json!("1.0");
        },
        expect_v2_validation_error("attestedPayload.version", "Unsupported version"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_wrong_payload_type() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            p["attestedPayload"]["type"] = json!("public_decryption");
        },
        expect_v2_validation_error("attestedPayload.type", "Unsupported payload type"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_empty_handles() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            p["attestedPayload"]["handles"] = json!([]);
        },
        expect_v2_validation_error("attestedPayload.handles", "Must not be empty"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_handle_entry_missing_owner_address() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            // Strip ownerAddress from the first handle entry.
            let entry = p["attestedPayload"]["handles"][0].as_object_mut().unwrap();
            entry.remove("ownerAddress");
        },
        // Missing required field surfaces as a serde deserialization error,
        // which the parsing layer maps to the `missing_fields` label with
        // the missing field name (`ownerAddress`).
        expect_v2_missing_field("ownerAddress"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_expired_request_validity_window() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            // start a year ago, duration of 1 second — window already
            // expired, should be rejected.
            p["attestedPayload"]["requestValidity"] = json!({
                "startTimestamp": "1000000",
                "durationSeconds": "1",
            });
        },
        expect_v2_validation_error(
            "attestedPayload.requestValidity",
            "requestValidity window has already expired",
        ),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_unknown_extra_field_on_envelope() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint(
        &url,
        helpers::create_v3_envelope(),
        |p: &mut serde_json::Value| {
            p["extraneousField"] = json!("nope");
        },
        // `deny_unknown_fields` surfaces as a validation error in this
        // codebase (parser flags the unknown field) rather than a generic
        // malformed-JSON.
        expect_v2_validation_error("extraneousField", "Unknown field"),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn v3_rejects_malformed_json() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::v3_user_decrypt_post_url(&setup);

    test_endpoint_raw_body(&url, "{ this is not json", expect_v2_malformed_json()).await;

    setup.shutdown().await;
}
