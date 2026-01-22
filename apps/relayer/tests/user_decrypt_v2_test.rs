mod common;

use crate::common::utils::{
    assert_retry_after_header_present, create_timeout_test_config, TestSetup,
};
use alloy::primitives::{Address, Bytes, B256};
use ethereum_rpc_mock::Response;
use fhevm_relayer::http::endpoints::v2::types::user_decrypt::{
    UserDecryptPostResponseJson, UserDecryptStatusResponseJson,
};
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::TempDir;

mod constants {
    use alloy::sol_types::SolCall;

    pub const EXTRA_DATA: &str = "0x00";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";

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

    pub fn random_encrypted_bytes() -> Bytes {
        let mut rng = rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        Bytes::from(bytes)
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

    /// Validates UserDecrypt response format compatibility with TKMS library
    /// for client-side plaintext reconstruction
    pub fn verify_tkms_compatibility() {
        use alloy::primitives::Bytes;
        use fhevm_relayer::http::endpoints::v1::types::user_decrypt as v1_types;
        use fhevm_relayer::http::endpoints::v2::types::user_decrypt as v2_types;
        use serde_json;

        // Create identical test data
        let test_payload = Bytes::from(vec![0x01, 0x02, 0x03]);
        let test_signature = Bytes::from(vec![0x04, 0x05, 0x06]);
        let test_extra_data = "0x00".to_string();

        // Create v1 response
        let v1_item = v1_types::UserDecryptResponsePayloadJson {
            payload: test_payload.clone(),
            signature: test_signature.clone(),
            extra_data: test_extra_data.clone(),
        };
        let v1_response = v1_types::UserDecryptResponseJson {
            response: vec![v1_item],
        };

        // Create v2 response
        let v2_item = v2_types::UserDecryptResponsePayloadJson {
            payload: test_payload.clone(),
            signature: test_signature.clone(),
            extra_data: test_extra_data.clone(),
        };
        let v2_response = v2_types::UserDecryptResponseJson {
            result: vec![v2_item],
        };

        // Serialize both to JSON
        let v1_json = serde_json::to_string(&v1_response).expect("Failed to serialize v1 response");
        let v2_json = serde_json::to_string(&v2_response).expect("Failed to serialize v2 response");

        // Parse back to compare structure
        let v1_parsed: serde_json::Value =
            serde_json::from_str(&v1_json).expect("Failed to parse v1 JSON");
        let v2_parsed: serde_json::Value =
            serde_json::from_str(&v2_json).expect("Failed to parse v2 JSON");

        // Check that both have the expected structure
        assert_eq!(
            v1_parsed["response"].as_array().unwrap().len(),
            1,
            "v1 should have one response item"
        );
        assert_eq!(
            v2_parsed["result"].as_array().unwrap().len(),
            1,
            "v2 should have one result item"
        );

        let v1_item = &v1_parsed["response"][0];
        let v2_item = &v2_parsed["result"][0];

        // Verify field presence and types
        assert!(
            v1_item["payload"].is_string(),
            "v1 payload should be string"
        );
        assert!(
            v1_item["signature"].is_string(),
            "v1 signature should be string"
        );

        assert!(
            v2_item["payload"].is_string(),
            "v2 payload should be string"
        );
        assert!(
            v2_item["signature"].is_string(),
            "v2 signature should be string"
        );
        // Verify payload and signature values match
        assert_eq!(
            v1_item["payload"], v2_item["payload"],
            "Payload values must match between v1 and v2"
        );
        assert_eq!(
            v1_item["signature"], v2_item["signature"],
            "Signature values must match between v1 and v2"
        );

        // Note: Both v1 and v2 now have aligned behavior - neither serializes extra_data
        assert!(
            v2_item.get("extra_data").is_none(),
            "v2 should not serialize extra_data field (aligned with v1)"
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
        assert_eq!(post_response.status, "queued");
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
    let encrypted_bytes = helpers::random_encrypted_bytes();

    setup.fhevm_mock.on_user_decrypt_success(
        handles,
        user_address,
        encrypted_bytes,
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

    assert_eq!(post_response.status, "queued");
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
            assert_eq!(get_body.status, "succeeded");
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
                // Note: extra_data is no longer serialized (aligned with V1 behavior)
            }
        }
        reqwest::StatusCode::ACCEPTED => {
            assert_eq!(get_body.status, "queued");
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
    let encrypted_bytes = helpers::random_encrypted_bytes();

    setup.fhevm_mock.on_user_decrypt_success(
        handles,
        user_address,
        encrypted_bytes,
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

    assert_eq!(post_response1.status, "queued");
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

    assert_eq!(post_response2.status, "queued");
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
        handles.clone(),
        user_address,
        Bytes::from(vec![0x01]),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, "succeeded");
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
        .on_user_decrypt_request_only(handles, user_address);

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
        handles.clone(),
        user_address,
        Bytes::from(vec![0x01]),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, "succeeded");
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
    assert_eq!(body.status, "failed");
    assert!(body.result.is_none());

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
        .on_user_decrypt_revert(constants::REVERT_ENFORCED_PAUSE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, "failed");
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
        .on_user_decrypt_revert(constants::REVERT_INVALID_SIGNATURE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(body.status, "failed");
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
        .on_user_decrypt_revert(constants::REVERT_INSUFFICIENT_BALANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, "failed");
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
        .on_user_decrypt_revert(constants::REVERT_INSUFFICIENT_ALLOWANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, "failed");
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
        .on_user_decrypt_revert(constants::REVERT_UNKNOWN_SELECTOR);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.status, "failed");
    assert!(body.result.is_none());

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("internal_server_error"),
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
    assert_eq!(body1.status, "failed");
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
