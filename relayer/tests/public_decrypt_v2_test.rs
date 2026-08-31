mod common;

use crate::common::utils::{
    assert_retry_after_header_present, create_timeout_test_config,
    register_host_acl_allow_all_dynamic, register_host_acl_deny_all,
    register_host_acl_partial_deny, register_host_acl_rpc_error, TestSetup, TEST_HOST_CHAIN_ID,
    TEST_HOST_CHAIN_ID_2,
};
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Bytes, B256};
use ethereum_rpc_mock::Response;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::http::endpoints::v2::types::public_decrypt::PublicDecryptPostResponseJson;
use fhevm_relayer::http::validation_messages as constants_validation;
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
    // The v2 POST → poll lifecycle and payload builders live in
    // `common::flows` so the listener-redundancy suite can drive this flow too.
    pub use crate::common::flows::public_decrypt::*;
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
    setup.ct_attestation().serve_attestations().await;

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
        error.label(),
        "internal_server_error",
        "Expected label 'internal_server_error' for max retries exceeded"
    );

    setup.shutdown().await;
}

/// Test contract paused (EnforcedPause 0xd93c0665) returns HTTP 503 with label "protocol_paused"
#[tokio::test]
async fn test_contract_paused_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.ct_attestation().serve_attestations().await;
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
        error.label(),
        "protocol_paused",
        "Expected label 'protocol_paused' for EnforcedPause error"
    );

    setup.shutdown().await;
}

/// Test invalid signature (0x2a873d27) returns HTTP 400 with label "validation_failed" and signature details
#[tokio::test]
async fn test_invalid_signature_returns_400() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.ct_attestation().serve_attestations().await;
    setup
        .fhevm_mock
        .on_public_decrypt_revert(constants::REVERT_INVALID_SIGNATURE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(error.label(), "validation_failed");

    setup.shutdown().await;
}

/// Test insufficient balance (ERC20InsufficientBalance 0xe450d38c) returns HTTP 503 with label "insufficient_balance"
#[tokio::test]
async fn test_insufficient_balance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.ct_attestation().serve_attestations().await;
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
        error.label(),
        "insufficient_balance",
        "Expected label 'insufficient_balance' for ERC20InsufficientBalance error"
    );

    setup.shutdown().await;
}

/// Test insufficient allowance (ERC20InsufficientAllowance 0xfb8f41b2) returns HTTP 503 with label "insufficient_allowance"
#[tokio::test]
async fn test_insufficient_allowance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.ct_attestation().serve_attestations().await;
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
        error.label(),
        "insufficient_allowance",
        "Expected label 'insufficient_allowance' for ERC20InsufficientAllowance error"
    );

    setup.shutdown().await;
}

/// Test unknown selector returns HTTP 500 with label "internal_server_error"
#[tokio::test]
async fn test_unknown_selector_returns_500() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();

    setup.ct_attestation().serve_attestations().await;
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
        error.label(),
        "internal_server_error",
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
    setup.ct_attestation().serve_attestations().await;

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

/// Test that a terminal readiness failure — Coprocessors serving attestations over divergent
/// ciphertext material, so no group ever reaches the majority threshold — transitions the request
/// from 'queued' straight to 'failure', without burning the retry budget that an
/// as-yet-unattested ciphertext is entitled to.
#[tokio::test]
async fn test_readiness_no_consensus_returns_failure_v2() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    // Every Coprocessor signs valid attestations, but over different material
    setup.ct_attestation().serve_divergent_attestations().await;

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;

    // Poll until terminal state — before fix this would panic with
    // "Request did not reach terminal state in time" because DB stays 'queued'
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "Expected 500 when the Coprocessors cannot agree on the ciphertext material"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.label(),
        "no_attestation_consensus",
        "Expected label 'no_attestation_consensus', distinct from the retryable timeout label"
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

    // No Coprocessor has published an attestation for the handle
    setup.ct_attestation().serve_nothing().await;

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
        error.label(),
        "readiness_check_timed_out",
        "Expected label 'readiness_check_timed_out' for readiness timeout"
    );

    setup.shutdown().await;
}

/// The overall request budget, not the retry counter, is what ends a check against unresponsive
/// buckets.
///
/// Every bucket accepts the connection and then stalls, so each attempt costs a full
/// `head_timeout` instead of returning fast the way a 404 does. The retry counter is set high
/// enough (100 × 50ms) that exhausting it would take far longer than the 200ms budget, so a 503
/// here can only have come from the budget expiring. Without that budget this shape of failure is
/// what stretches a nominal four-minute retry policy into tens of minutes while holding a
/// readiness throttler permit throughout.
#[tokio::test]
async fn test_stalled_buckets_are_bounded_by_the_request_budget() {
    let setup = TestSetup::new_with_short_request_budget()
        .await
        .expect("Failed to create test setup");

    setup
        .ct_attestation()
        .serve_stalled(std::time::Duration::from_secs(30))
        .await;

    let payload = helpers::create_public_decrypt_payload();
    let started = std::time::Instant::now();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;
    let elapsed = started.elapsed();

    assert_eq!(
        status,
        reqwest::StatusCode::SERVICE_UNAVAILABLE,
        "A stalled fan-out must fail closed and stay retriable"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.label(),
        "readiness_check_timed_out",
        "The budget expiring reuses the existing timeout label, not a new one"
    );

    // 100 attempts × 50ms of sleeping alone is 5s before any probing is counted, so finishing well
    // inside that is what distinguishes the budget from the retry counter.
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "Expected the request budget to end the check early, took {elapsed:?}"
    );

    setup.shutdown().await;
}

/// The majority threshold, not the registry size, is what readiness requires.
///
/// Two of the three registered Coprocessors have published; the third 404s. The mocked
/// `getCoprocessorMajorityThreshold()` reports 2, so the quorum is satisfied without the silent
/// bucket and the request goes through. Substituting the registry size for the threshold — the
/// off-by-one every all-buckets-agree test misses, since there the two numbers coincide — starves
/// this round instead and turns the 200 into a 503.
#[tokio::test]
async fn test_partial_quorum_passes_readiness() {
    let setup = TestSetup::new_with_fast_readiness()
        .await
        .expect("Failed to create test setup");

    setup
        .ct_attestation()
        .serve_attestations_from_first(2)
        .await;

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

    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "A threshold-sized quorum is consensus; the third Coprocessor is not required"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

/// The other side of the same boundary: one attestation short of the threshold is retriable, not
/// terminal.
///
/// Only one of three buckets has published, so the leading group cannot reach the majority of 2.
/// Nothing disagreed, though — a lone unanimous vote is a gap, not a split — so this must spend the
/// retry budget and end on the retryable timeout label rather than `no_attestation_consensus`.
/// Loosening the quorum check to "any valid attestation" would let this request through instead.
///
/// The vote is genuinely counted before the verdict: the mock holds the empty buckets back so the
/// attestation is not overtaken by the two 404s, which would otherwise decide the round at zero
/// votes and leave the quorum arithmetic untested.
#[tokio::test]
async fn test_below_quorum_is_retriable_not_terminal() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    setup
        .ct_attestation()
        .serve_attestations_from_first(1)
        .await;

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::SERVICE_UNAVAILABLE,
        "One attestation under a majority of 2 must fail closed"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.label(),
        "readiness_check_timed_out",
        "Agreement that merely lacks numbers stays retryable, so not 'no_attestation_consensus'"
    );

    setup.shutdown().await;
}

/// The evidence that explains a readiness give-up must reach the caller, not just the label.
///
/// Same shape as `test_below_quorum_is_retriable_not_terminal` (one attestation short of the
/// majority of 2), but this test pins the response body's `message` rather than just its `label`:
/// it must still start with the standard phrase — that prefix is what the status handler matches
/// with `starts_with` to pick the `readiness_check_timed_out` label — and then carry detail past
/// it explaining which handle, and how many attested.
#[tokio::test]
async fn test_readiness_timeout_message_carries_round_detail() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    setup
        .ct_attestation()
        .serve_attestations_from_first(1)
        .await;

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.label(),
        "readiness_check_timed_out",
        "the appended detail must not relabel the response"
    );

    let message = error.message();
    assert!(
        message.starts_with(fhevm_relayer::core::errors::READINESS_CHECK_TIMEOUT_MSG),
        "message must still start with the standard phrase: {message}"
    );
    assert!(
        message.len() > fhevm_relayer::core::errors::READINESS_CHECK_TIMEOUT_MSG.len(),
        "message must carry detail past the standard phrase: {message}"
    );
    assert!(
        message.contains("required attested"),
        "message must carry the round's redacted summary: {message}"
    );

    setup.shutdown().await;
}

/// No digest value may reach the caller. This pins the HTTP response against a future change
/// routing the operator-only board into a stored reason.
#[tokio::test]
async fn test_readiness_timeout_message_has_no_digest_values() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    setup
        .ct_attestation()
        .serve_attestations_from_first(1)
        .await;

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (_, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    let error = body.error.as_ref().expect("Error should be present");
    let message = error.message();

    // The digests the mock attests.
    for leaked in [B256::repeat_byte(0xBB), B256::repeat_byte(0xCC)] {
        assert!(
            !message.contains(&format!("{leaked:x}")),
            "leaked digest {leaked}: {message}"
        );
    }
    assert!(
        message.contains("required attested"),
        "message must still carry the round's redacted summary: {message}"
    );

    setup.shutdown().await;
}

/// A round that missed is retried, and a later round that finds the attestations succeeds.
///
/// Every bucket 404s its first probe and serves from the second on, so the first readiness attempt
/// misses and the second reaches consensus. This is the only test that proves the retry loop
/// converts a late attestation into a success: treating `MissedThisRound` as terminal would fail
/// the request on the first miss with `no_attestation_consensus`.
#[tokio::test]
async fn test_missed_round_retries_then_passes() {
    let setup = TestSetup::new_with_fast_readiness()
        .await
        .expect("Failed to create test setup");

    setup.ct_attestation().serve_after_n_misses(1).await;

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

    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "The second attempt finds the attestations, so the request must succeed"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

// The same three readiness outcomes under `source: gateway_chain`. Kept as their own tests rather
// than cases of the ones above: the two sources fail for unrelated reasons and only overlap on the
// timeout label, so sharing a body would hide more than it saves.

/// `isPublicDecryptionReady` answering true is enough on its own — this setup starts no attestation
/// buckets at all, so nothing else could have let the request through.
///
/// The mock answers true only for a call carrying both handles and the request's `extra_data`, so
/// this also pins what the checker forwards. Two handles, because a single one cannot catch a
/// truncated or reordered vec.
#[tokio::test]
async fn test_gateway_chain_readiness_success() {
    let setup = TestSetup::new_with_gateway_chain_readiness()
        .await
        .expect("Failed to create test setup");
    let payload = json!({
        "ciphertextHandles": [helpers::random_handle(), helpers::random_handle()],
        "extraData": constants::EXTRA_DATA
    });
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    setup
        .fhevm_mock
        .set_readiness_success_for_handles(handles.clone(), Bytes::from(vec![0x00]));
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

/// Readiness retries rather than giving up on the first "not ready": the Gateway answers false
/// once, then true, and the request goes through.
///
/// The success and timeout tests either side of this one both stay green if the retry loop
/// collapses to a single attempt, which in production would cut the wait-for-ciphertext budget to
/// one poll and 503 every request that is merely early.
#[tokio::test]
async fn test_gateway_chain_readiness_retries_then_passes() {
    let setup = TestSetup::new_with_gateway_chain_readiness()
        .await
        .expect("Failed to create test setup");
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    setup.fhevm_mock.set_readiness_success_after_n_failures(1);
    setup.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::OK,
        "The second readiness attempt answers true, so the request must succeed"
    );
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

/// A ciphertext the Gateway never reports ready exhausts the retry budget and stays retryable —
/// the same label an unattested handle earns off-chain, since both mean "not yet".
#[tokio::test]
async fn test_gateway_chain_readiness_timeout_returns_503() {
    let setup = TestSetup::new_with_gateway_chain_readiness()
        .await
        .expect("Failed to create test setup");

    setup.fhevm_mock.set_readiness_failure();

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(error.label(), "readiness_check_timed_out");

    setup.shutdown().await;
}

/// An unreachable gateway read node is a fault of the relayer's own dependency, not of the
/// ciphertext, so it surfaces as a 500 rather than as the retryable readiness label. This is the
/// only path that reaches `ReadinessCheckError::GwContractError`; the off-chain source has no
/// counterpart to it.
#[tokio::test]
async fn test_gateway_chain_readiness_contract_error_returns_500() {
    let setup = TestSetup::new_with_gateway_chain_readiness()
        .await
        .expect("Failed to create test setup");

    setup.fhevm_mock.set_readiness_contract_error();

    let payload = helpers::create_public_decrypt_payload();
    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(error.label(), "internal_server_error");

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
        expect_v2_validation_error("extraData", constants_validation::INVALID_EXTRA_DATA_FORMAT),
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
#[case::empty_extra_data("extraData", json!(""), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[case::untagged_context_id_extra_data(
    "extraData",
    json!("0x010000000000000000000000000000000000000000000000000000000000000001"),
    constants_validation::INVALID_EXTRA_DATA_FORMAT
)]
#[case::untagged_epoch_id_extra_data(
    "extraData",
    json!("0x0207000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002"),
    constants_validation::INVALID_EXTRA_DATA_FORMAT
)]
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

// ---------------------------------------------------------------------------
// Host ACL check tests
// ---------------------------------------------------------------------------

/// When the host chain ACL contract returns false for all handles,
/// the request should fail with 400 and label "not_allowed_on_host_acl".
#[tokio::test]
async fn test_not_allowed_on_host_acl_returns_400() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();

    // Override default allow-all ACL with deny-all
    let acl_address =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    // Public decrypt: 1 handle → 1 isAllowedForDecryption call in multicall
    register_host_acl_deny_all(&setup.host_server, acl_address);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for ACL not allowed"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    // Verify the error label
    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

/// When the host chain RPC is unavailable, the request should fail with 500
/// after exhausting retries.
#[tokio::test]
async fn test_host_acl_rpc_error_returns_500() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();

    // Override default allow-all ACL with RPC error
    let acl_address =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_rpc_error(&setup.host_server, acl_address);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        "Expected 500 for ACL RPC error"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    setup.shutdown().await;
}

/// 3 handles, all allowed on host ACL → 200 success.
#[tokio::test]
async fn test_multi_handle_acl_all_allowed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let handles_hex: Vec<String> = (0..3).map(|_| helpers::random_handle()).collect();
    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });
    let handles: Vec<B256> = handles_hex
        .iter()
        .map(|h| B256::from_str(h).unwrap())
        .collect();
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Replace default ACL mock (count 1-2) with dynamic allow-all that handles count 3
    let acl_address =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address);

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

/// 3 handles, indices 0 and 2 denied → 400 with label "not_allowed_on_host_acl".
#[tokio::test]
async fn test_multi_handle_acl_partial_deny() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let handles_hex: Vec<String> = (0..3).map(|_| helpers::random_handle()).collect();
    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });

    // Override default ACL with partial deny (indices 0 and 2)
    let acl_address =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_partial_deny(&setup.host_server, acl_address, vec![0, 2]);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for partial ACL denial"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

/// Handle with unsupported chain_id 99999 → immediate 400 from POST.
#[tokio::test]
async fn test_unsupported_chain_id_returns_400() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let unsupported_handle = crate::common::utils::random_handle_with_chain_id(99999);
    let payload = json!({
        "ciphertextHandles": [unsupported_handle],
        "extraData": constants::EXTRA_DATA
    });

    // POST should return 400 synchronously (no job created)
    let response = reqwest::Client::new()
        .post(helpers::v2_public_decrypt_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send POST request");

    assert_eq!(
        response.status(),
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for unsupported chain ID"
    );

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["status"].as_str(), Some("failed"));
    assert_eq!(
        body["error"]["label"].as_str(),
        Some("host_chain_id_not_supported")
    );

    setup.shutdown().await;
}

/// 3 handles on same chain, all denied → 400 with label "not_allowed_on_host_acl".
#[tokio::test]
async fn test_multi_handle_acl_all_denied() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let handles_hex: Vec<String> = (0..3).map(|_| helpers::random_handle()).collect();
    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });

    let acl_address =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_deny_all(&setup.host_server, acl_address);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for all handles denied"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

// ---------------------------------------------------------------------------
// Cross-chain ACL tests (handles spanning chain 8009 and 9001)
// ---------------------------------------------------------------------------

/// Cross-chain: 2 handles on chain A + 1 on chain B, all allowed → 200 success.
#[tokio::test]
async fn test_cross_chain_acl_all_allowed() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let handle_a1 = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_a2 = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b1 = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);
    let handles_hex = vec![handle_a1, handle_a2, handle_b1];

    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });
    let handles: Vec<B256> = handles_hex
        .iter()
        .map(|h| B256::from_str(h).unwrap())
        .collect();
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Both chains allow all
    let acl_address_a =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    let acl_address_b =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[1].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address_a);
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address_b);

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

/// Cross-chain: handles on both chains, all denied → 400.
#[tokio::test]
async fn test_cross_chain_acl_all_denied() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let handle_a = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);
    let handles_hex = vec![handle_a, handle_b];

    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });

    let acl_address_a =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    let acl_address_b =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[1].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_deny_all(&setup.host_server, acl_address_a);
    register_host_acl_deny_all(&setup.host_server, acl_address_b);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for all handles denied across chains"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

/// Cross-chain: chain A allows, chain B denies → 400 partial deny.
#[tokio::test]
async fn test_cross_chain_acl_partial_deny() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let handle_a = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);
    let handles_hex = vec![handle_a, handle_b];

    let payload = json!({
        "ciphertextHandles": handles_hex,
        "extraData": constants::EXTRA_DATA
    });

    // Chain A: allow all, Chain B: deny all
    let acl_address_a =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[0].acl_address)
            .expect("Invalid ACL address");
    let acl_address_b =
        alloy::primitives::Address::from_str(&setup.settings.host_chains[1].acl_address)
            .expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address_a);
    register_host_acl_deny_all(&setup.host_server, acl_address_b);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for cross-chain partial deny"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}
