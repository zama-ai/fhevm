//! Integration tests for the v3 `/v3/user-decrypt` endpoint (unified EIP-712
//! unified user-decryption). Uses the in-process `TestSetup` harness and
//! the `ethereum_rpc_mock` JSON-RPC mock — the same plumbing v2 tests use.
//!
//! Covers the full HTTP → calldata → mocked-gateway → shares → GET path
//! end-to-end, plus the validation / dispatch / schema-rejection surface.

mod common;

use crate::common::utils::{
    assert_retry_after_header_present, sign_v3_user_decrypt_envelope, user_decrypt_test_signer,
    TestSetup,
};
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Address, B256};
use ethereum_rpc_mock::fhevm::UserDecryptKind;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::http::endpoints::v2::types::user_decrypt::{
    UserDecryptPostResponseJson, UserDecryptStatusResponseJson,
};
use rand::{rng, RngExt};
use serde_json::json;
use std::str::FromStr;
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

    pub fn v3_user_decrypt_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v3/user-decrypt/{}",
            setup.http_port, job_id
        )
    }

    /// Extract the `ctHandle` strings from the first handle entry of a v3
    /// envelope as `B256` values — the format the `ethereum_rpc_mock`
    /// `on_user_decrypt_success(...)` registration expects.
    pub fn extract_handles_from_v3_envelope(payload: &serde_json::Value) -> Vec<B256> {
        payload["attestedPayload"]["handles"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|h| {
                h["ctHandle"].as_str().and_then(|s| {
                    let cleaned = s.strip_prefix("0x").unwrap_or(s);
                    B256::from_str(cleaned).ok()
                })
            })
            .collect()
    }

    pub fn extract_user_address_from_v3_envelope(payload: &serde_json::Value) -> Address {
        Address::from_str(
            payload["attestedPayload"]["userAddress"]
                .as_str()
                .expect("attestedPayload.userAddress must be a string"),
        )
        .expect("attestedPayload.userAddress must be a valid hex address")
    }

    /// Build a unified EIP-712 v3 envelope with a single random allowed contract, correctly
    /// signed by the fixed test signer so it passes the pre-check.
    pub fn create_v3_envelope() -> serde_json::Value {
        create_v3_envelope_with_allowed_contracts(vec![format!("{:?}", random_address())])
    }

    /// Like [`create_v3_envelope`] but with a caller-chosen `allowedContracts` list. The list is
    /// part of the signed payload, so signing happens after it is set.
    pub fn create_v3_envelope_with_allowed_contracts(
        allowed_contracts: Vec<String>,
    ) -> serde_json::Value {
        let signer = user_decrypt_test_signer();
        let user_address = signer.address();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut envelope = json!({
            "attestationType": "eip712-unified-user-decrypt-v1",
            "attestedPayload": {
                "version": "2.0",
                "type": "user_decryption",
                "handles": [{
                    "ctHandle": random_handle(),
                    "contractAddress": format!("{:?}", random_address()),
                    "ownerAddress": format!("{:?}", user_address),
                }],
                "userAddress": format!("{:?}", user_address),
                "allowedContracts": allowed_contracts,
                "requestValidity": {
                    "startTimestamp": (now - 1).to_string(),
                    "durationSeconds": "604800",
                },
                "publicKey": random_0x_hex(32),
                "extraData": "0x00",
            },
            "signature": "0x",
        });
        sign_v3_user_decrypt_envelope(&mut envelope, &signer);
        envelope
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
/// list (allowed by the unified EIP-712 spec).
#[tokio::test]
async fn v3_accepts_empty_allowed_contracts() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_v3_envelope_with_allowed_contracts(vec![]);

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
    let payload = helpers::create_v3_envelope_with_allowed_contracts(vec![
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

// ---------------------------------------------------------------------------
// Envelope-level rejection (attestationType, signature)
// ---------------------------------------------------------------------------

/// v3 rejects a signature that fails the pre-check. A random blob doesn't recover to
/// `userAddress`, and the host mock reports no code there, so the request is rejected with 400
/// and never forwarded.
#[tokio::test]
async fn v3_rejects_invalid_signature() {
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

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    setup.shutdown().await;
}

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

// ---------------------------------------------------------------------------
// Happy-path E2E: POST → calldata submitted → mocked-gateway emits unified
// `UserDecryptionRequest_1` request event + the shared
// `UserDecryptionResponse` shares → GET returns succeeded with the shares.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn v3_e2e_succeeds_against_mocked_gateway() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_v3_envelope();
    let handles = helpers::extract_handles_from_v3_envelope(&payload);
    let user_address = helpers::extract_user_address_from_v3_envelope(&payload);

    // Wire the mock to recognize the unified `userDecryptionRequest_0`
    // selector, emit a `UserDecryptionRequest_1` event with the decryption
    // id, and follow up with the standard 9 shares + 1 threshold-reached
    // event.
    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Unified,
        handles,
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // POST → 202 + jobId
    let response = reqwest::Client::new()
        .post(helpers::v3_user_decrypt_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("POST failed");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response);

    let post_response: UserDecryptPostResponseJson = response
        .json()
        .await
        .expect("Failed to parse POST response");
    assert_eq!(post_response.status, ApiResponseStatus::Queued);
    let job_id = post_response.result.job_id.clone();

    // Poll GET until terminal — the mock emits the threshold-reached event
    // at ~2s after the POST, so allow plenty of headroom.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let get_response = reqwest::Client::new()
            .get(helpers::v3_user_decrypt_get_url(&setup, &job_id))
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .expect("GET failed");

        let status = get_response.status();
        if status == reqwest::StatusCode::ACCEPTED {
            assert!(
                std::time::Instant::now() < deadline,
                "v3 job never completed"
            );
            continue;
        }

        assert_eq!(
            status,
            reqwest::StatusCode::OK,
            "v3 job ended in unexpected state {}: {:?}",
            status,
            get_response.text().await
        );
        let body: UserDecryptStatusResponseJson = get_response
            .json()
            .await
            .expect("Failed to parse GET response");
        assert_eq!(body.status, ApiResponseStatus::Succeeded);
        let result = body.result.expect("Succeeded GET must include result");
        assert!(
            !result.result.is_empty(),
            "Result items should not be empty"
        );
        for item in &result.result {
            assert!(!item.payload.is_empty(), "Share payload empty");
            assert!(!item.signature.is_empty(), "Share signature empty");
        }
        break;
    }

    setup.shutdown().await;
}
