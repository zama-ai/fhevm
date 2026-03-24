mod common;

use crate::common::utils::{create_timeout_test_config, TestSetup};
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Address, Bytes};
use ethereum_rpc_mock::Response;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::http::endpoints::v2::types::input_proof::{
    InputProofPostResponseJson, InputProofStatusResponseJson,
};
use fhevm_relayer::http::validation_messages as constants_validation;
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use tempfile::TempDir;

mod constants {
    use alloy::sol_types::SolCall;

    pub const EXTRA_DATA: &str = "0x00";

    // Timeout test configuration
    pub const TIMEOUT_DURATION_SECS: u64 = 3;
    pub const CRON_INTERVAL_SECS: u64 = 1;
    pub const INITIAL_POLL_DELAY_MS: u64 = 500;

    pub const INPUT_PROOF_SELECTOR: [u8; 4] =
        fhevm_relayer::gateway::arbitrum::bindings::InputVerification::verifyProofRequestCall::SELECTOR;

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

    pub fn v2_input_proof_post_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/input-proof", setup.http_port)
    }

    pub fn v2_input_proof_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v2/input-proof/{}",
            setup.http_port, job_id
        )
    }

    pub fn random_address() -> Address {
        utils::random_address()
    }

    pub fn random_bytes() -> Bytes {
        let mut rng = rng();
        let len = rng.random_range(4..32);
        let bytes: Vec<u8> = (0..len).map(|_| rng.random()).collect();
        Bytes::from(bytes)
    }

    pub fn create_input_proof_payload(setup: &TestSetup) -> (serde_json::Value, Address, Bytes) {
        let contract_address = random_address();
        let user_address = random_address();
        let ciphertext_data = random_bytes();

        let payload = json!({
            "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
            "contractAddress": format!("{:?}", contract_address),
            "userAddress": format!("{:?}", user_address),
            "ciphertextWithInputVerification": hex::encode(&ciphertext_data),
            "extraData": constants::EXTRA_DATA
        });

        (payload, user_address, ciphertext_data)
    }

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_input_proof_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .json(payload)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
        let post_response: InputProofPostResponseJson = response
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
    ) -> (reqwest::StatusCode, InputProofStatusResponseJson) {
        let client = reqwest::Client::new();
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let response = client
                .get(v2_input_proof_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
                .expect("Failed to send GET request");

            let status = response.status();
            if status != reqwest::StatusCode::ACCEPTED {
                let body: InputProofStatusResponseJson =
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
    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data,
        1,
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

    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);

    // Configure mock to emit REQUEST event only (no response) - will timeout
    setup
        .fhevm_mock
        .on_input_proof_request_only(user_address, ciphertext_data);

    test_v2_timeout_flow(
        helpers::v2_input_proof_post_url(&setup),
        |job_id| helpers::v2_input_proof_get_url(&setup, job_id),
        payload,
        constants::TIMEOUT_DURATION_SECS,
        constants::CRON_INTERVAL_SECS,
        constants::INITIAL_POLL_DELAY_MS,
    )
    .await;

    // Cleanup
    setup.shutdown().await;
}

#[tokio::test]
async fn test_nonce_too_low_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);

    // First attempt fails with nonce-too-low, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.input_proof_contract,
        constants::INPUT_PROOF_SELECTOR,
        vec![Response::error("nonce too low".to_string())],
    );
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data,
        1,
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
async fn test_gateway_rejection_returns_200_with_accepted_false() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);
    setup
        .fhevm_mock
        .on_input_proof_error(user_address, ciphertext_data, 1);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    let result = body.result.expect("expected result with accepted=false");
    assert!(!result.accepted);
    assert!(result.handles.is_none());
    assert!(result.signatures.is_none());

    setup.shutdown().await;
}

#[tokio::test]
async fn test_nonce_too_high_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);

    // First attempt fails with nonce-too-high, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.input_proof_contract,
        constants::INPUT_PROOF_SELECTOR,
        vec![Response::error("nonce too high".to_string())],
    );
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data,
        1,
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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    // Queue more errors than max_attempts (3 errors > 2 max_attempts)
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.input_proof_contract,
        constants::INPUT_PROOF_SELECTOR,
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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    setup
        .fhevm_mock
        .on_input_proof_revert(constants::REVERT_ENFORCED_PAUSE);

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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    setup
        .fhevm_mock
        .on_input_proof_revert(constants::REVERT_INVALID_SIGNATURE);

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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    setup
        .fhevm_mock
        .on_input_proof_revert(constants::REVERT_INSUFFICIENT_BALANCE);

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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    setup
        .fhevm_mock
        .on_input_proof_revert(constants::REVERT_INSUFFICIENT_ALLOWANCE);

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
    let (payload, _user_address, _ciphertext_data) = helpers::create_input_proof_payload(&setup);

    setup
        .fhevm_mock
        .on_input_proof_revert(constants::REVERT_UNKNOWN_SELECTOR);

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

/// Test consecutive duplicate requests return same job_id (deduplication)
/// Validates that identical input-proof requests submitted while the first is still active
/// return the same ext_job_id, enabling proper deduplication behavior.
#[tokio::test]
async fn test_consecutive_duplicate_requests_return_same_job_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Create a fixed payload to use for both requests
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let ciphertext_data = helpers::random_bytes();

    let payload = json!({
        "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        "contractAddress": format!("{:?}", contract_address),
        "userAddress": format!("{:?}", user_address),
        "ciphertextWithInputVerification": hex::encode(&ciphertext_data),
        "extraData": constants::EXTRA_DATA
    });

    // Setup mock for success
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data.clone(),
        1,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_input_proof_post_url(&setup);

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
    let post_response1: InputProofPostResponseJson = response1
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
    let post_response2: InputProofPostResponseJson = response2
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
        "Duplicate input-proof requests with identical content should return the same job_id when \
         the first request is still active. Got different job_ids: '{}' vs '{}'",
        job_id_1, job_id_2
    );

    // Wait for processing
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    // GET with first job_id should work
    let get_response1 = client
        .get(helpers::v2_input_proof_get_url(&setup, job_id_1))
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
        .get(helpers::v2_input_proof_get_url(&setup, job_id_2))
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

/// Test that modifying any field in the payload produces a different job_id
/// Validates that content hashing correctly distinguishes between different requests
#[tokio::test]
async fn test_different_payloads_produce_different_job_ids() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let ciphertext_data1 = helpers::random_bytes();
    let ciphertext_data2 = helpers::random_bytes();

    let payload1 = json!({
        "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        "contractAddress": format!("{:?}", contract_address),
        "userAddress": format!("{:?}", user_address),
        "ciphertextWithInputVerification": hex::encode(&ciphertext_data1),
        "extraData": constants::EXTRA_DATA
    });

    let payload2 = json!({
        "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        "contractAddress": format!("{:?}", contract_address),
        "userAddress": format!("{:?}", user_address),
        "ciphertextWithInputVerification": hex::encode(&ciphertext_data2), // Different ciphertext
        "extraData": constants::EXTRA_DATA
    });

    // Setup mocks for both requests
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data1,
        1,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data2,
        2,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id_1 = helpers::submit_request(&setup, &payload1).await;
    let job_id_2 = helpers::submit_request(&setup, &payload2).await;

    println!("First request job_id: {}", job_id_1);
    println!("Second request job_id: {}", job_id_2);

    // Different payloads should produce different job_ids
    assert_ne!(
        job_id_1, job_id_2,
        "Different input-proof requests should produce different job_ids. \
         Got same job_id '{}' for both requests.",
        job_id_1
    );

    setup.shutdown().await;
}

/// Test that retrying a failed request creates a new job_id
/// Validates that the migration to allow multiple rows with same int_job_id works correctly
#[tokio::test]
async fn test_retry_after_failure_creates_new_job_id() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");

    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let ciphertext_data = helpers::random_bytes();

    // Generate payload once - will be used for both attempts
    let payload = json!({
        "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        "contractAddress": format!("{:?}", contract_address),
        "userAddress": format!("{:?}", user_address),
        "ciphertextWithInputVerification": hex::encode(&ciphertext_data),
        "extraData": constants::EXTRA_DATA
    });

    // Configure mock to fail with max retries exceeded
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.input_proof_contract,
        constants::INPUT_PROOF_SELECTOR,
        vec![
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
        ],
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_input_proof_post_url(&setup);

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
    let post_response1: InputProofPostResponseJson = response1
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
    let post_response2: InputProofPostResponseJson = response2
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

/// Test that malformed JSON returns V2 error format with status and request_id
#[tokio::test]
async fn test_v2_post_malformed_json_has_status_and_request_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_input_proof_post_url(&setup),
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
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v2_input_proof_post_url(&setup),
        base_payload,
        with_invalid_field("contractAddress", json!("")),
        expect_v2_validation_error(
            "contractAddress",
            constants_validation::HEX_MUST_START_WITH_0X,
        ),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Chain ID validation
#[case::empty_chain_id("contractChainId", json!(""), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_decimal("contractChainId", json!("abc123"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_hex("contractChainId", json!("0xzzz"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
// Contract address validation
#[case::empty_contract_address("contractAddress", json!(""), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::short_contract_address("contractAddress", json!("0xfds"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_contract_address("contractAddress", json!("0x1234567890123456789012345678901234567890123"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_contract_address("contractAddress", json!("1234567890123456789012345678901234567890"), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_contract_address("contractAddress", json!("0x123zzz5678901234567890123456789012345678"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::contract_address_with_invalid_hex_g("contractAddress", json!("0x123456789012345678901234567890123456789g"), constants_validation::HEX_INVALID_CHARACTERS)]
#[tokio::test]
async fn test_error_invalid_fields_set_1(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v2_input_proof_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// User address validation
#[case::empty_user_address("userAddress", json!(""), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::short_user_address("userAddress", json!("0xfds"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_user_address("userAddress", json!("0x1234567890123456789012345678901234567890123"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_user_address("userAddress", json!("1234567890123456789012345678901234567890"), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_user_address("userAddress", json!("0x123zzz5678901234567890123456789012345678"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::user_address_with_invalid_hex_g("userAddress", json!("0x123456789012345678901234567890123456789g"), constants_validation::HEX_INVALID_CHARACTERS)]
#[tokio::test]
async fn test_error_invalid_fields_set_2(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v2_input_proof_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Ciphertext validation
#[case::empty_ciphertext("ciphertextWithInputVerification", json!(""), constants_validation::MUST_NOT_BE_EMPTY)]
#[case::invalid_hex_ciphertext("ciphertextWithInputVerification", json!("abcdefabcdefs"), constants_validation::HEX_INVALID_STRING)]
#[case::odd_length_ciphertext("ciphertextWithInputVerification", json!("abcdef1"), constants_validation::HEX_INVALID_STRING)]
#[case::ciphertext_with_invalid_hex_g("ciphertextWithInputVerification", json!("abcdefg"), constants_validation::HEX_INVALID_STRING)]
#[case::ciphertext_with_0x_prefix("ciphertextWithInputVerification", json!("0xabcdef"), constants_validation::HEX_MUST_NOT_START_WITH_0X)]
// Extra data validation
#[case::empty_extra_data("extraData", json!(""), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants_validation::INVALID_EXTRA_DATA_FORMAT)]
#[tokio::test]
async fn test_error_invalid_fields_set_3(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v2_input_proof_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_contract_chain_id("contractChainId")]
#[case::missing_contract_address("contractAddress")]
#[case::missing_user_address("userAddress")]
#[case::missing_ciphertext("ciphertextWithInputVerification")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v2_input_proof_post_url(&setup),
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
        &helpers::v2_input_proof_post_url(&setup),
        malformed_json,
        expect_v2_malformed_json(),
    )
    .await;

    setup.shutdown().await;
}
