mod common;

use crate::common::utils::{
    assert_retry_after_header_present, create_timeout_test_config, TestSetup,
};
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::B256;
use ethereum_rpc_mock::Response;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::http::endpoints::v2::types::public_decrypt::{
    PublicDecryptPostResponseJson, PublicDecryptStatusResponseJson,
};
use fhevm_relayer::http::validation_messages as constants_validation;
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use std::str::FromStr;
use tempfile::TempDir;

mod constants {
    use alloy::sol_types::SolCall;

    pub const EXTRA_DATA: &str = "0x00";

    // Timeout test configuration
    pub const TIMEOUT_DURATION_SECS: u64 = 3;
    pub const CRON_INTERVAL_SECS: u64 = 1;
    pub const INITIAL_POLL_DELAY_MS: u64 = 500;

    pub const PUBLIC_DECRYPT_SELECTOR: [u8; 4] =
        fhevm_relayer::gateway::arbitrum::bindings::Decryption::publicDecryptionRequestCall::SELECTOR;

    // Contract error selectors for testing error classification
    // These match the selectors in src/gateway/arbitrum/transaction/contract_error_parser.rs
    pub const REVERT_ENFORCED_PAUSE: &str = "execution reverted: 0xd93c0665";
    pub const REVERT_INVALID_SIGNATURE: &str = "execution reverted: 0x2a873d27";
    pub const REVERT_INSUFFICIENT_BALANCE: &str = "execution reverted: 0xe450d38c";
    pub const REVERT_INSUFFICIENT_ALLOWANCE: &str = "execution reverted: 0xfb8f41b2";
    pub const REVERT_UNKNOWN_SELECTOR: &str = "execution reverted: 0x12345678";
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v2_public_decrypt_post_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/public-decrypt", setup.http_port)
    }

    pub fn v2_public_decrypt_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v2/public-decrypt/{}",
            setup.http_port, job_id
        )
    }

    pub fn random_handle() -> String {
        utils::random_handle()
    }

    pub fn create_public_decrypt_payload() -> serde_json::Value {
        let handle = random_handle();
        json!({
            "ciphertextHandles": [handle],
            "extraData": constants::EXTRA_DATA
        })
    }

    pub fn random_plaintext_values(count: usize) -> Vec<u64> {
        let mut rng = rng();
        (0..count).map(|_| rng.random()).collect()
    }

    pub fn extract_ciphertext_handles_from_public_payload(
        payload: &serde_json::Value,
    ) -> Vec<B256> {
        payload["ciphertextHandles"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|handle| {
                handle.as_str().and_then(|s| {
                    let cleaned = s.strip_prefix("0x").unwrap_or(s);
                    B256::from_str(cleaned).ok()
                })
            })
            .collect()
    }

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_public_decrypt_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .json(payload)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
        let post_response: PublicDecryptPostResponseJson = response
            .json()
            .await
            .expect("Failed to parse POST response");
        assert_eq!(post_response.status, ApiResponseStatus::Queued);
        post_response.result.job_id
    }

    /// Poll GET endpoint until terminal state, return (status, body)
    pub async fn poll_until_terminal(
        setup: &TestSetup,
        job_id: &str,
    ) -> (reqwest::StatusCode, PublicDecryptStatusResponseJson) {
        let client = reqwest::Client::new();
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let response = client
                .get(v2_public_decrypt_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
                .expect("Failed to send GET request");

            let status = response.status();
            if status != reqwest::StatusCode::ACCEPTED {
                let body: PublicDecryptStatusResponseJson =
                    response.json().await.expect("Failed to parse GET response");
                return (status, body);
            }
        }
        panic!("Request did not reach terminal state in time");
    }
}

#[tokio::test]
async fn test_success_single_request() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    setup.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

#[tokio::test]
async fn test_nonce_too_low_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // First attempt fails with nonce-too-low, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::PUBLIC_DECRYPT_SELECTOR,
        vec![Response::error("nonce too low".to_string())],
    );
    setup.fhevm_mock.on_public_decrypt_success(
        handles.clone(),
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

#[tokio::test]
async fn test_nonce_too_high_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // First attempt fails with nonce-too-high, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::PUBLIC_DECRYPT_SELECTOR,
        vec![Response::error("nonce too high".to_string())],
    );
    setup.fhevm_mock.on_public_decrypt_success(
        handles.clone(),
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

#[tokio::test]
async fn test_max_retries_exceeded_fails() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");
    let payload = helpers::create_public_decrypt_payload();

    // Set up readiness check to pass
    setup.fhevm_mock.set_readiness_success();

    // Queue more errors than max_attempts (3 errors > 2 max_attempts)
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::PUBLIC_DECRYPT_SELECTOR,
        vec![
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
        ],
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_ne!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("internal_server_error"),
        "Expected label 'internal_server_error' for max retries exceeded"
    );

    setup.shutdown().await;
}

/// Test contract paused (EnforcedPause 0xd93c0665) returns HTTP 503 with label "protocol_paused"
#[tokio::test]
async fn test_contract_paused_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_ENFORCED_PAUSE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("protocol_paused"),
        "Expected label 'protocol_paused' for EnforcedPause error"
    );

    setup.shutdown().await;
}

/// Test invalid signature (0x2a873d27) returns HTTP 400 with label "validation_failed" and signature details
#[tokio::test]
async fn test_invalid_signature_returns_400() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_INVALID_SIGNATURE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    assert_eq!(
        body.error,
        Some(serde_json::json!({
            "label": "validation_failed",
            "message": "Validation failed for 1 field(s)",
            "details": [{ "field": "signature", "issue": "Signature is invalid" }]
        }))
    );

    setup.shutdown().await;
}

/// Test insufficient balance (ERC20InsufficientBalance 0xe450d38c) returns HTTP 503 with label "insufficient_balance"
#[tokio::test]
async fn test_insufficient_balance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_INSUFFICIENT_BALANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("insufficient_balance"),
        "Expected label 'insufficient_balance' for ERC20InsufficientBalance error"
    );

    setup.shutdown().await;
}

/// Test insufficient allowance (ERC20InsufficientAllowance 0xfb8f41b2) returns HTTP 503 with label "insufficient_allowance"
#[tokio::test]
async fn test_insufficient_allowance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_INSUFFICIENT_ALLOWANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("insufficient_allowance"),
        "Expected label 'insufficient_allowance' for ERC20InsufficientAllowance error"
    );

    setup.shutdown().await;
}

/// Test unknown selector returns HTTP 500 with label "internal_server_error"
#[tokio::test]
async fn test_unknown_selector_returns_500() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_UNKNOWN_SELECTOR);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("internal_server_error"),
        "Expected label 'internal_server_error' for unknown selector error"
    );

    setup.shutdown().await;
}

/// Test consecutive duplicate requests succeed in V2
/// Documents that duplicate requests with identical content should both succeed
/// and validates duplicate requests return valid job_ids.
#[tokio::test]
async fn test_consecutive_duplicate_requests_succeed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Generate random payload once and use across two requests
    let payload = helpers::create_public_decrypt_payload();
    let handles = payload["ciphertextHandles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| B256::from_str(h.as_str().unwrap().strip_prefix("0x").unwrap()).unwrap())
        .collect::<Vec<_>>();
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    setup.fhevm_mock.on_public_decrypt_success(
        handles.clone(),
        plaintext_values.clone(),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_public_decrypt_post_url(&setup);

    // Send first POST request
    let response1 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send first POST request");

    assert_eq!(response1.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response1);

    let post_response1: PublicDecryptPostResponseJson = response1
        .json()
        .await
        .expect("Failed to parse first POST response");

    assert_eq!(post_response1.status, ApiResponseStatus::Queued);
    let job_id_1 = &post_response1.result.job_id;

    // Send consecutive duplicate request (same payload)
    let response2 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send second POST request");

    assert_eq!(response2.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response2);

    let post_response2: PublicDecryptPostResponseJson = response2
        .json()
        .await
        .expect("Failed to parse second POST response");

    assert_eq!(post_response2.status, ApiResponseStatus::Queued);
    let job_id_2 = &post_response2.result.job_id;

    // Print job_ids for debugging
    println!("First request job_id: {}", job_id_1);
    println!("Second request job_id: {}", job_id_2);

    // CRITICAL ASSERTION: For duplicate requests sent while first is still active,
    // the system should return the SAME ext_job_id (deduplication behavior)
    assert_eq!(
        job_id_1, job_id_2,
        "Duplicate requests with identical content should return the same job_id when \
         the first request is still active. Got different job_ids: '{}' vs '{}'",
        job_id_1, job_id_2
    );

    // Wait for processing
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    // GET with first job_id should work
    let get_response1 = client
        .get(helpers::v2_public_decrypt_get_url(&setup, job_id_1))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send first GET request");

    let status1 = get_response1.status();
    println!("First GET job_id '{}' - Status: {}", job_id_1, status1);

    // Should NOT be 404
    assert_ne!(
        status1,
        reqwest::StatusCode::NOT_FOUND,
        "GET request for first job_id '{}' returned 404. This indicates the job_id \
         returned by POST doesn't exist in the database.",
        job_id_1
    );

    // GET with second job_id should also work (since they should be identical)
    let get_response2 = client
        .get(helpers::v2_public_decrypt_get_url(&setup, job_id_2))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send second GET request");

    let status2 = get_response2.status();
    println!("Second GET job_id '{}' - Status: {}", job_id_2, status2);

    // Should NOT be 404 - documents expected behavior
    assert_ne!(
        status2,
        reqwest::StatusCode::NOT_FOUND,
        "GET request for second job_id '{}' returned 404. This indicates the job_id \
         returned by POST doesn't exist in the database. Both job_ids should be retrievable \
         for duplicate requests with identical content.",
        job_id_2
    );

    setup.shutdown().await;
}

/// Test that retrying a failed request creates a new job_id
/// This validates that the migration to allow multiple rows with same int_job_id works correctly
#[tokio::test]
async fn test_retry_after_failure_creates_new_job_id() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");

    // Generate payload once - will be used for both attempts
    let payload = helpers::create_public_decrypt_payload();

    // Set up readiness check to pass
    setup.fhevm_mock.set_readiness_success();

    // Configure mock to fail with max retries exceeded
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::PUBLIC_DECRYPT_SELECTOR,
        vec![
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
        ],
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_public_decrypt_post_url(&setup);

    // First attempt - will fail
    let response1 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send first POST request");

    assert_eq!(response1.status(), reqwest::StatusCode::ACCEPTED);
    let post_response1: PublicDecryptPostResponseJson = response1
        .json()
        .await
        .expect("Failed to parse first POST response");

    let job_id_1 = post_response1.result.job_id.clone();
    println!("First attempt job_id: {}", job_id_1);

    // Wait for it to fail
    let (status1, body1) = helpers::poll_until_terminal(&setup, &job_id_1).await;
    assert_ne!(status1, reqwest::StatusCode::OK);
    assert_eq!(body1.status, ApiResponseStatus::Failed);
    println!("First attempt failed as expected");

    // Retry with same payload after failure
    let response2 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send retry POST request");

    assert_eq!(response2.status(), reqwest::StatusCode::ACCEPTED);
    let post_response2: PublicDecryptPostResponseJson = response2
        .json()
        .await
        .expect("Failed to parse retry POST response");

    let job_id_2 = post_response2.result.job_id.clone();
    println!("Retry attempt job_id: {}", job_id_2);

    // CRITICAL: After migration, retrying a failed request should create a NEW job_id
    assert_ne!(
        job_id_1, job_id_2,
        "Retry after failure should create a new job_id. \
         Before migration fix, this would return the same job_id or fail with duplicate key error. \
         Got same job_id '{}' for both attempts.",
        job_id_1
    );

    println!("✅ Retry created new job_id as expected");

    setup.shutdown().await;
}

#[tokio::test]
async fn test_timeout() {
    use crate::common::utils::test_v2_timeout_flow;

    // Create setup with fast timeout config
    let temp_config_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_config_path = create_timeout_test_config(
        &temp_config_dir,
        constants::TIMEOUT_DURATION_SECS,
        constants::CRON_INTERVAL_SECS,
    )
    .expect("Failed to create timeout config");

    let setup = TestSetup::new_with_config_path(Some(temp_config_path))
        .await
        .expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);

    // Configure mock to emit REQUEST event only (no response) - will timeout
    setup.fhevm_mock.on_public_decrypt_request_only(handles);

    test_v2_timeout_flow(
        helpers::v2_public_decrypt_post_url(&setup),
        |job_id| helpers::v2_public_decrypt_get_url(&setup, job_id),
        payload,
        constants::TIMEOUT_DURATION_SECS,
        constants::CRON_INTERVAL_SECS,
        constants::INITIAL_POLL_DELAY_MS,
    )
    .await;

    // Cleanup
    setup.shutdown().await;
}

/// Test that a readiness check contract error (RPC node unavailable) correctly
/// transitions the request from 'queued' to 'failure' so V2 clients see failure.
/// Before the fix, update_status_to_failure_on_tx_failed silently no-oped because
/// the request was still in 'queued' state (not 'processing' or 'tx_in_flight').
#[tokio::test]
async fn test_readiness_contract_error_returns_failure_v2() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    // Configure readiness checks to return RPC error (node unavailable)
    setup.fhevm_mock.set_readiness_contract_error();

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;

    // Poll until terminal state — before fix this would panic with
    // "Request did not reach terminal state in time" because DB stays 'queued'
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "Expected 500 for readiness check contract error (RPC error is not a known revert)"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("internal_server_error"),
        "Expected label 'internal_server_error' for readiness check contract error"
    );

    setup.shutdown().await;
}

/// Test that a readiness check timeout (ciphertext never ready) correctly returns
/// HTTP 503 with label "readiness_check_timed_out" so V2 clients can distinguish
/// readiness timeouts from gateway response timeouts.
#[tokio::test]
async fn test_readiness_timeout_returns_503_with_correct_label() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    // Configure readiness checks to always return false (ciphertext never ready)
    setup.fhevm_mock.set_readiness_failure();

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;

    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::SERVICE_UNAVAILABLE,
        "Expected 503 for readiness check timeout"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("readiness_check_timed_out"),
        "Expected label 'readiness_check_timed_out' for readiness timeout"
    );

    setup.shutdown().await;
}

/// Test that malformed JSON returns V2 error format with status and request_id
#[tokio::test]
async fn test_v2_post_malformed_json_has_status_and_request_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_public_decrypt_post_url(&setup),
        "{ invalid json }",
        expect_v2_malformed_json(),
    )
    .await;

    setup.shutdown().await;
}

/// Test that validation errors return V2 error format with status and request_id
#[tokio::test]
async fn test_v2_post_validation_error_has_status_and_request_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v2_public_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field("extraData", json!("invalid")),
        expect_v2_validation_error("extraData", constants_validation::EXACT_MUST_BE_0X00),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Ciphertext handles validation
#[case::empty_ciphertext_handles("ciphertextHandles", json!([]), constants_validation::MUST_NOT_BE_EMPTY)]
#[case::invalid_hex_ciphertext_handle("ciphertextHandles", json!(["0xabcdefabcdefs"]), constants_validation::HEX_INVALID_STRING)]
#[case::odd_length_ciphertext_handle("ciphertextHandles", json!(["0xabcdef1"]), constants_validation::HEX_INVALID_STRING)]
#[case::ciphertext_handle_with_invalid_hex_g("ciphertextHandles", json!(["0xabcdefg"]), constants_validation::HEX_INVALID_STRING)]
#[case::ciphertext_handle_without_0x_prefix("ciphertextHandles", json!(["abcdef123456789012345678901234567890123456789012345678901234567890"]), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::empty_string_ciphertext_handle("ciphertextHandles", json!([""]), constants_validation::HEX_MUST_START_WITH_0X)]
// Extra data validation
#[case::empty_extra_data("extraData", json!(""), constants_validation::EXACT_MUST_BE_0X00)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants_validation::EXACT_MUST_BE_0X00)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants_validation::EXACT_MUST_BE_0X00)]
#[tokio::test]
async fn test_error_invalid_fields(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v2_public_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_ciphertext_handles("ciphertextHandles")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v2_public_decrypt_post_url(&setup),
        base_payload,
        |p| {
            p.as_object_mut().unwrap().remove(field);
        },
        expect_v2_missing_field(field),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_closing_brace(r#"{"field": "value""#)]
#[case::missing_comma(r#"{"field1": "value1" "field2": "value2"}"#)]
#[tokio::test]
async fn test_error_malformed_json(#[case] malformed_json: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_public_decrypt_post_url(&setup),
        malformed_json,
        expect_v2_malformed_json(),
    )
    .await;

    setup.shutdown().await;
}
