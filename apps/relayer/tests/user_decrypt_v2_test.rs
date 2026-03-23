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
use alloy::primitives::{Address, B256};
use ethereum_rpc_mock::fhevm::UserDecryptKind;
use ethereum_rpc_mock::Response;
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use fhevm_relayer::http::endpoints::v2::types::user_decrypt::{
    UserDecryptPostResponseJson, UserDecryptStatusResponseJson,
};
use fhevm_relayer::http::validation_messages as constants_validation;
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

mod constants {
    use alloy::sol_types::SolCall;

    pub const EXTRA_DATA: &str = "0x00";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";
    // Should fail since this date is in the future (2035)
    pub const FUTURE_DATE: &str = "2051218800";

    // Timeout test configuration
    pub const TIMEOUT_DURATION_SECS: u64 = 3;
    pub const CRON_INTERVAL_SECS: u64 = 1;
    pub const INITIAL_POLL_DELAY_MS: u64 = 500;

    pub const USER_DECRYPT_SELECTOR: [u8; 4] =
        fhevm_relayer::gateway::arbitrum::bindings::Decryption::userDecryptionRequestCall::SELECTOR;

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

    pub fn v2_user_decrypt_post_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/user-decrypt", setup.http_port)
    }

    pub fn v2_user_decrypt_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v2/user-decrypt/{}",
            setup.http_port, job_id
        )
    }

    pub fn random_address() -> Address {
        utils::random_address()
    }

    pub fn random_handle() -> String {
        utils::random_handle()
    }

    pub fn random_signature() -> String {
        let mut rng = rng();
        (0..130)
            .map(|_| rng.random_range(0..16))
            .map(|digit| format!("{:x}", digit))
            .collect()
    }

    pub fn random_public_key() -> String {
        let mut rng = rng();
        (0..64)
            .map(|_| rng.random_range(0..16))
            .map(|digit| format!("{:x}", digit))
            .collect()
    }

    pub fn create_user_decrypt_payload(
        chain_id: &str,
        contract_address: Address,
        user_address: Address,
    ) -> serde_json::Value {
        let handle = random_handle();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        json!({
            "handleContractPairs": [{
                "handle": handle,
                "contractAddress": format!("{:?}", contract_address)
            }],
            "requestValidity": {
                "startTimestamp": (now - 1).to_string(),
                "durationDays": constants::REQUEST_VALIDITY_DAYS
            },
            "contractsChainId": chain_id,
            "contractAddresses": [format!("{:?}", contract_address)],
            "userAddress": format!("{:?}", user_address),
            "signature": random_signature(),
            "publicKey": random_public_key(),
            "extraData": constants::EXTRA_DATA
        })
    }

    pub fn extract_ciphertext_handles_from_user_payload(payload: &serde_json::Value) -> Vec<B256> {
        payload["handleContractPairs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|pair| {
                pair["handle"].as_str().and_then(|s| {
                    let cleaned = s.strip_prefix("0x").unwrap_or(s);
                    B256::from_str(cleaned).ok()
                })
            })
            .collect()
    }

    /// Validates UserDecrypt v2 response format compatibility with TKMS library
    /// for client-side plaintext reconstruction
    pub fn verify_tkms_compatibility() {
        use alloy::primitives::Bytes;
        use fhevm_relayer::http::endpoints::v2::types::user_decrypt as v2_types;

        // Create test data
        let test_payload = Bytes::from(vec![0x01, 0x02, 0x03]);
        let test_signature = Bytes::from(vec![0x04, 0x05, 0x06]);
        let test_extra_data = "0x00".to_string();

        // Create v2 response
        let v2_item = v2_types::UserDecryptResponsePayloadJson {
            payload: test_payload.clone(),
            signature: test_signature.clone(),
            extra_data: test_extra_data.clone(),
        };
        let v2_response = v2_types::UserDecryptResponseJson {
            result: vec![v2_item],
        };

        // Serialize to JSON
        let v2_json = serde_json::to_string(&v2_response).expect("Failed to serialize v2 response");

        // Parse back to verify structure
        let v2_parsed: serde_json::Value =
            serde_json::from_str(&v2_json).expect("Failed to parse v2 JSON");

        assert_eq!(
            v2_parsed["result"].as_array().unwrap().len(),
            1,
            "v2 should have one result item"
        );

        let v2_item = &v2_parsed["result"][0];

        assert!(
            v2_item["payload"].is_string(),
            "v2 payload should be string"
        );
        assert!(
            v2_item["signature"].is_string(),
            "v2 signature should be string"
        );
        assert!(
            v2_item.get("extra_data").is_none(),
            "v2 should not serialize extra_data field"
        );
    }

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_user_decrypt_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .json(payload)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
        let post_response: UserDecryptPostResponseJson = response
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
    ) -> (reqwest::StatusCode, UserDecryptStatusResponseJson) {
        let client = reqwest::Client::new();
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let response = client
                .get(v2_user_decrypt_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
                .expect("Failed to send GET request");

            let status = response.status();
            if status != reqwest::StatusCode::ACCEPTED {
                let body: UserDecryptStatusResponseJson =
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles,
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // Step 1: POST request should return reference ID
    let response = reqwest::Client::new()
        .post(helpers::v2_user_decrypt_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send POST request");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response);

    let post_response: UserDecryptPostResponseJson = response
        .json()
        .await
        .expect("Failed to parse POST response");

    assert_eq!(post_response.status, ApiResponseStatus::Queued);
    let job_id = &post_response.result.job_id;

    // Step 2: GET request should eventually return completed result
    // Give some time for processing
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;

    let get_response = reqwest::Client::new()
        .get(helpers::v2_user_decrypt_get_url(&setup, job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send GET request");

    let status = get_response.status();

    // Check Retry-After header before consuming response
    if status == reqwest::StatusCode::ACCEPTED {
        assert_retry_after_header_present(&get_response);
    }

    let get_body: UserDecryptStatusResponseJson = get_response
        .json()
        .await
        .expect("Failed to parse GET response");

    // Should be either succeeded (200) or still queued (202)
    match status {
        reqwest::StatusCode::OK => {
            assert_eq!(get_body.status, ApiResponseStatus::Succeeded);
            assert!(get_body.result.is_some());

            // Validate the response structure for TKMS library compatibility
            let result = get_body.result.unwrap();
            assert!(
                !result.result.is_empty(),
                "Result items should not be empty"
            );

            // Verify each result item has the required fields
            for result_item in &result.result {
                assert!(
                    !result_item.payload.is_empty(),
                    "Payload should not be empty"
                );
                assert!(
                    !result_item.signature.is_empty(),
                    "Signature should not be empty"
                );
                // Note: extra_data is no longer serialized
            }
        }
        reqwest::StatusCode::ACCEPTED => {
            assert_eq!(get_body.status, ApiResponseStatus::Queued);
        }
        _ => panic!("Unexpected status code: {}", status),
    }

    setup.shutdown().await;
}

/// Test consecutive duplicate requests succeed in V2
/// Documents that duplicate requests with identical content should both succeed
/// and validates duplicate requests return valid job_ids.
#[tokio::test]
async fn test_consecutive_duplicate_requests_succeed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();

    // Generate random payload once and use across two requests
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles,
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_user_decrypt_post_url(&setup);

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

    let post_response1: UserDecryptPostResponseJson = response1
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

    let post_response2: UserDecryptPostResponseJson = response2
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
        .get(helpers::v2_user_decrypt_get_url(&setup, job_id_1))
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
        .get(helpers::v2_user_decrypt_get_url(&setup, job_id_2))
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

#[tokio::test]
async fn test_nonce_too_low_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    // First attempt fails with nonce-too-low, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![Response::error("nonce too low".to_string())],
    );
    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles.clone(),
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

#[test]
fn test_tkms_compatibility() {
    // Validates response format compatibility with TKMS library for plaintext reconstruction
    helpers::verify_tkms_compatibility();
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

    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    // Configure mock to emit REQUEST event only (no response) - will timeout
    setup
        .fhevm_mock
        .on_user_decrypt_request_only(UserDecryptKind::Direct, handles, user_address);

    test_v2_timeout_flow(
        helpers::v2_user_decrypt_post_url(&setup),
        |job_id| helpers::v2_user_decrypt_get_url(&setup, job_id),
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
async fn test_nonce_too_high_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    // First attempt fails with nonce-too-high, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![Response::error("nonce too high".to_string())],
    );
    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles.clone(),
        user_address,
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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Set up readiness check to pass
    setup.fhevm_mock.set_readiness_success();

    // Queue more errors than max_attempts (3 errors > 2 max_attempts)
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(UserDecryptKind::Direct, constants::REVERT_ENFORCED_PAUSE);

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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(UserDecryptKind::Direct, constants::REVERT_INVALID_SIGNATURE);

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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup.fhevm_mock.on_user_decrypt_revert(
        UserDecryptKind::Direct,
        constants::REVERT_INSUFFICIENT_BALANCE,
    );

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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup.fhevm_mock.on_user_decrypt_revert(
        UserDecryptKind::Direct,
        constants::REVERT_INSUFFICIENT_ALLOWANCE,
    );

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
    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(UserDecryptKind::Direct, constants::REVERT_UNKNOWN_SELECTOR);

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

/// Test that retrying a failed request creates a new job_id
/// This validates that the migration to allow multiple rows with same int_job_id works correctly
#[tokio::test]
async fn test_retry_after_failure_creates_new_job_id() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");

    let contract_address = helpers::random_address();
    let user_address = helpers::random_address();

    // Generate payload once - will be used for both attempts
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Set up readiness check to pass
    setup.fhevm_mock.set_readiness_success();

    // Configure mock to fail with max retries exceeded
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
            Response::error("nonce too low".to_string()),
        ],
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_user_decrypt_post_url(&setup);

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
    let post_response1: UserDecryptPostResponseJson = response1
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
    let post_response2: UserDecryptPostResponseJson = response2
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

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

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.label(),
        "internal_server_error",
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

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

/// Test that malformed JSON returns V2 error format with status and request_id
#[tokio::test]
async fn test_v2_post_malformed_json_has_status_and_request_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_user_decrypt_post_url(&setup),
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field("extraData", json!("invalid")),
        expect_v2_validation_error("extraData", constants_validation::INVALID_EXTRA_DATA_FORMAT),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Chain ID validation
#[case::empty_chain_id("contractsChainId", json!(""), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_decimal("contractsChainId", json!("abc123"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_hex("contractsChainId", json!("0xzzz"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
// Contract addresses validation
#[case::empty_contract_addresses("contractAddresses", json!([]), constants_validation::MUST_NOT_BE_EMPTY)]
#[case::short_contract_address("contractAddresses", json!(["0xfds"]), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_contract_address("contractAddresses", json!(["0x1234567890123456789012345678901234567890123"]), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_contract_address("contractAddresses", json!(["1234567890123456789012345678901234567890"]), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_contract_address("contractAddresses", json!(["0x123zzz5678901234567890123456789012345678"]), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::contract_address_with_invalid_hex_g("contractAddresses", json!(["0x123456789012345678901234567890123456789g"]), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::empty_string_contract_address("contractAddresses", json!([""]), constants_validation::HEX_MUST_START_WITH_0X)]
#[tokio::test]
async fn test_error_invalid_fields_set_1(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
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
#[case::empty_string_user_address("userAddress", json!(""), constants_validation::HEX_MUST_START_WITH_0X)]
// Handle contract pairs validation
#[case::empty_handle_contract_pairs("handleContractPairs", json!([]), constants_validation::MUST_NOT_BE_EMPTY)]
#[tokio::test]
async fn test_error_invalid_fields_set_2(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Signature validation
#[case::short_signature("signature", json!("abcdef12"), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::long_signature("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::signature_with_0x_prefix("signature", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::HEX_MUST_NOT_START_WITH_0X)]
#[case::signature_with_invalid_hex_g("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants_validation::HEX_INVALID_STRING)]
#[case::empty_signature("signature", json!(""), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
// Public key validation
#[case::public_key_with_0x_prefix("publicKey", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::HEX_MUST_NOT_START_WITH_0X)]
#[case::public_key_with_invalid_hex_g("publicKey", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants_validation::HEX_INVALID_STRING)]
#[case::empty_public_key("publicKey", json!(""), constants_validation::MUST_NOT_BE_EMPTY)]
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
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::short_handle("0xabcdef", constants_validation::LENGTH_MUST_BE_64_CHARACTERS)]
#[case::long_handle(
    "0xabcdef1234567890123456789012345678901234567890123456789012345678901234567890",
    constants_validation::LENGTH_MUST_BE_64_CHARACTERS
)]
#[case::handle_with_invalid_hex_g(
    "0xabcdefg123456789012345678901234567890123456789012345678901234567890",
    constants_validation::HEX_INVALID_STRING
)]
#[case::handle_without_0x_prefix(
    "abcdef123456789012345678901234567890123456789012345678901234567890",
    constants_validation::HEX_MUST_START_WITH_0X
)]
#[case::empty_handle("", constants_validation::HEX_MUST_START_WITH_0X)]
#[tokio::test]
async fn test_error_invalid_nested_handle_fields(
    #[case] invalid_handle: &str,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Modify the nested handle field with the test case value
    base_payload["handleContractPairs"][0]["handle"] = json!(invalid_handle);

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
        base_payload,
        |_| {}, // No additional modifications needed
        expect_v2_validation_error("handleContractPairs", expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::future_timestamp(
    constants::FUTURE_DATE,
    constants_validation::TIMESTAMP_MUST_NOT_BE_IN_FUTURE
)]
#[tokio::test]
async fn test_error_invalid_nested_handle_fields_2(
    #[case] future_date: &str,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Modify the nested handle field with the test case value
    base_payload["requestValidity"]["startTimestamp"] = json!(future_date);

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
        base_payload,
        |_| {}, // No additional modifications needed
        expect_v2_validation_error("requestValidity", expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_contracts_chain_id("contractsChainId")]
#[case::missing_contract_addresses("contractAddresses")]
#[case::missing_user_address("userAddress")]
#[case::missing_handle_contract_pairs("handleContractPairs")]
#[case::missing_request_validity("requestValidity")]
#[case::missing_signature("signature")]
#[case::missing_public_key("publicKey")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v2_user_decrypt_post_url(&setup),
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
        &helpers::v2_user_decrypt_post_url(&setup),
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Override default allow-all ACL with deny-all
    let acl_address =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    // User decrypt: 1 handle → 2 calls (isAllowed for user + contract) in multicall
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Override default allow-all ACL with RPC error
    let acl_address =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
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

/// 2 handle-contract pairs (4 multicall calls: 2 per pair) → 200 success.
#[tokio::test]
async fn test_multi_pair_acl_all_allowed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle1 = helpers::random_handle();
    let handle2 = helpers::random_handle();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle1, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle2, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    // Replace default ACL mock with dynamic allow-all that handles count 4
    let acl_address =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address);

    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles,
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

/// 2 handle-contract pairs, deny user-check on pair 1 (index 2 in multicall) → 400.
#[tokio::test]
async fn test_multi_pair_acl_partial_deny() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle1 = helpers::random_handle();
    let handle2 = helpers::random_handle();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle1, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle2, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    // Override default ACL with partial deny (index 2 = user check on pair 1)
    let acl_address =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_partial_deny(&setup.host_server, acl_address, vec![2]);

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

/// Handle-contract pair with unsupported chain_id 99999 → immediate 400 from POST.
#[tokio::test]
async fn test_unsupported_chain_id_returns_400() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let unsupported_handle = crate::common::utils::random_handle_with_chain_id(99999);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [{
            "handle": unsupported_handle,
            "contractAddress": format!("{:?}", contract_address)
        }],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    // POST should return 400 synchronously (no job created)
    let response = reqwest::Client::new()
        .post(helpers::v2_user_decrypt_post_url(&setup))
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

/// 2 handle-contract pairs (4 multicall calls), all denied → 400.
#[tokio::test]
async fn test_multi_pair_acl_all_denied() {
    let setup = TestSetup::new_with_minimal_readiness()
        .await
        .expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle1 = helpers::random_handle();
    let handle2 = helpers::random_handle();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle1, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle2, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    let acl_address =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_deny_all(&setup.host_server, acl_address);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for all pairs denied"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

// ---------------------------------------------------------------------------
// Cross-chain ACL tests (handle-contract pairs spanning chain 8009 and 9001)
// ---------------------------------------------------------------------------

/// Cross-chain: pairs on both chains, all allowed → 200 success.
#[tokio::test]
async fn test_cross_chain_acl_all_allowed() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle_a = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle_a, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle_b, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);

    // Both chains allow all
    let acl_address_a =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    let acl_address_b =
        Address::from_str(&setup.settings.host_chains[1].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address_a);
    register_host_acl_allow_all_dynamic(&setup.host_server, acl_address_b);

    setup.fhevm_mock.on_user_decrypt_success(
        UserDecryptKind::Direct,
        handles,
        user_address,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

/// Cross-chain: pairs on both chains, all denied → 400.
#[tokio::test]
async fn test_cross_chain_acl_all_denied() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle_a = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle_a, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle_b, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    let acl_address_a =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    let acl_address_b =
        Address::from_str(&setup.settings.host_chains[1].acl_address).expect("Invalid ACL address");
    setup.host_server.reset_state();
    register_host_acl_deny_all(&setup.host_server, acl_address_a);
    register_host_acl_deny_all(&setup.host_server, acl_address_b);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(
        status,
        reqwest::StatusCode::BAD_REQUEST,
        "Expected 400 for all pairs denied across chains"
    );
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Expected error in response");
    assert_eq!(error.label(), "not_allowed_on_host_acl");

    setup.shutdown().await;
}

/// Cross-chain: chain A allows all, chain B denies → 400 partial deny.
#[tokio::test]
async fn test_cross_chain_acl_partial_deny() {
    let setup = TestSetup::new_with_multi_chain()
        .await
        .expect("Failed to create multi-chain test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let chain_id = setup.settings.gateway.blockchain_rpc.chain_id.to_string();
    let handle_a = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID);
    let handle_b = crate::common::utils::random_handle_with_chain_id(TEST_HOST_CHAIN_ID_2);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = json!({
        "handleContractPairs": [
            { "handle": handle_a, "contractAddress": format!("{:?}", contract_address) },
            { "handle": handle_b, "contractAddress": format!("{:?}", contract_address) }
        ],
        "requestValidity": {
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS
        },
        "contractsChainId": chain_id,
        "contractAddresses": [format!("{:?}", contract_address)],
        "userAddress": format!("{:?}", user_address),
        "signature": helpers::random_signature(),
        "publicKey": helpers::random_public_key(),
        "extraData": constants::EXTRA_DATA
    });

    // Chain A: allow all, Chain B: deny all
    let acl_address_a =
        Address::from_str(&setup.settings.host_chains[0].acl_address).expect("Invalid ACL address");
    let acl_address_b =
        Address::from_str(&setup.settings.host_chains[1].acl_address).expect("Invalid ACL address");
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
