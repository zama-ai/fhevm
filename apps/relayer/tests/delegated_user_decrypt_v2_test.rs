mod common;

use crate::common::utils::{
    assert_retry_after_header_present, create_timeout_test_config, TestSetup,
};
use crate::common::validation_helper::{
    expect_v2_malformed_json, expect_v2_missing_field, expect_v2_validation_error, test_endpoint,
    test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Address, Bytes, B256};
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

    pub fn v2_delegated_user_decrypt_post_url(setup: &TestSetup) -> String {
        format!(
            "http://localhost:{}/v2/delegated-user-decrypt",
            setup.http_port
        )
    }

    pub fn v2_delegated_user_decrypt_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v2/delegated-user-decrypt/{}",
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

    /// Create a delegated user decrypt payload
    pub fn create_delegated_user_decrypt_payload(
        chain_id: &str,
        contract_address: Address,
        delegator_address: Address,
        delegate_address: Address,
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
            "contractsChainId": chain_id,
            "contractAddresses": [format!("{:?}", contract_address)],
            "delegatorAddress": format!("{:?}", delegator_address),
            "delegateAddress": format!("{:?}", delegate_address),
            "startTimestamp": (now - 1).to_string(),
            "durationDays": constants::REQUEST_VALIDITY_DAYS,
            "signature": random_signature(),
            "publicKey": random_public_key(),
            "extraData": constants::EXTRA_DATA
        })
    }

    pub fn random_encrypted_bytes() -> Bytes {
        let mut rng = rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        Bytes::from(bytes)
    }

    pub fn extract_ciphertext_handles_from_delegated_payload(
        payload: &serde_json::Value,
    ) -> Vec<B256> {
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

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_delegated_user_decrypt_post_url(setup))
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
        for _ in 0..20 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let response = client
                .get(v2_delegated_user_decrypt_get_url(setup, job_id))
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

// Integration tests - verify the endpoint works end-to-end

/// Test basic success flow for delegated user decrypt.
#[tokio::test]
async fn test_delegated_user_decrypt_success() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    // Extract handle from payload for mock setup
    let handle_str = payload["handleContractPairs"][0]["handle"]
        .as_str()
        .unwrap();
    let handle = alloy::primitives::B256::from_str(handle_str).unwrap();

    setup.fhevm_mock.on_user_decrypt_success(
        vec![handle],
        delegate_address, // For delegated decrypt, the delegate is the user
        alloy::primitives::Bytes::from(vec![0x01, 0x02, 0x03]),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // Submit request
    let response = reqwest::Client::new()
        .post(helpers::v2_delegated_user_decrypt_post_url(&setup))
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

    // Wait for processing
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;

    let get_response = reqwest::Client::new()
        .get(helpers::v2_delegated_user_decrypt_get_url(&setup, job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send GET request");

    let status = get_response.status();
    let get_body: UserDecryptStatusResponseJson = get_response
        .json()
        .await
        .expect("Failed to parse GET response");

    // Should be either succeeded (200) or still queued (202)
    match status {
        reqwest::StatusCode::OK => {
            assert_eq!(get_body.status, ApiResponseStatus::Succeeded);
            assert!(get_body.result.is_some());
        }
        reqwest::StatusCode::ACCEPTED => {
            assert_eq!(get_body.status, ApiResponseStatus::Queued);
        }
        _ => panic!("Unexpected status code: {}", status),
    }

    setup.shutdown().await;
}

/// Test that delegated user decrypt request that times out properly updates database status.
#[tokio::test]
async fn test_delegated_user_decrypt_timeout_updates_db_status() {
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
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    // Don't configure mock response - this will cause a timeout
    let job_id = helpers::submit_request(&setup, &payload).await;

    // Wait for timeout to occur
    tokio::time::sleep(std::time::Duration::from_secs(
        constants::TIMEOUT_DURATION_SECS + constants::CRON_INTERVAL_SECS + 2,
    ))
    .await;

    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_ne!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Failed);

    setup.shutdown().await;
}

/// Test that delegated user decrypt request that fails during processing properly updates database status.
#[tokio::test]
async fn test_delegated_user_decrypt_max_retries_updates_db_status() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![
            ethereum_rpc_mock::Response::error("nonce too low".to_string()),
            ethereum_rpc_mock::Response::error("nonce too low".to_string()),
            ethereum_rpc_mock::Response::error("nonce too low".to_string()),
        ],
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_ne!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Failed);
    assert!(body.result.is_none());

    setup.shutdown().await;
}

// Contract error handling tests

/// Test that delegated user decrypt with contract paused error returns proper status.
#[tokio::test]
async fn test_delegated_user_decrypt_contract_paused_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(constants::REVERT_ENFORCED_PAUSE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("protocol_paused")
    );

    setup.shutdown().await;
}

/// Test that delegated user decrypt with invalid signature returns 400.
#[tokio::test]
async fn test_delegated_user_decrypt_invalid_signature_returns_400() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(constants::REVERT_INVALID_SIGNATURE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(body.status, ApiResponseStatus::Failed);

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

/// Test insufficient balance returns HTTP 503 with label "insufficient_balance"
#[tokio::test]
async fn test_insufficient_balance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(constants::REVERT_INSUFFICIENT_BALANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("insufficient_balance")
    );

    setup.shutdown().await;
}

/// Test insufficient allowance returns HTTP 503 with label "insufficient_allowance"
#[tokio::test]
async fn test_insufficient_allowance_returns_503() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(constants::REVERT_INSUFFICIENT_ALLOWANCE);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body.status, ApiResponseStatus::Failed);

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("insufficient_allowance")
    );

    setup.shutdown().await;
}

/// Test that delegated user decrypt with unknown error returns 500.
#[tokio::test]
async fn test_delegated_user_decrypt_unknown_error_returns_500() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
    setup
        .fhevm_mock
        .on_user_decrypt_revert(constants::REVERT_UNKNOWN_SELECTOR);

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body.status, ApiResponseStatus::Failed);

    let error = body.error.as_ref().expect("Error should be present");
    assert_eq!(
        error.get("label").and_then(|v| v.as_str()),
        Some("internal_server_error")
    );

    setup.shutdown().await;
}

// Deduplication and retry tests

/// Test consecutive duplicate requests return same job_id (deduplication).
#[tokio::test]
async fn test_consecutive_duplicate_requests_succeed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();

    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    let handles = helpers::extract_ciphertext_handles_from_delegated_payload(&payload);
    let encrypted_bytes = helpers::random_encrypted_bytes();

    setup.fhevm_mock.on_user_decrypt_success(
        handles,
        delegate_address,
        encrypted_bytes,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let client = reqwest::Client::new();
    let url = helpers::v2_delegated_user_decrypt_post_url(&setup);

    // First request
    let response1 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send first POST request");

    assert_eq!(response1.status(), reqwest::StatusCode::ACCEPTED);
    let post_response1: UserDecryptPostResponseJson =
        response1.json().await.expect("Failed to parse response");
    let job_id_1 = &post_response1.result.job_id;

    // Second request (duplicate)
    let response2 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send second POST request");

    assert_eq!(response2.status(), reqwest::StatusCode::ACCEPTED);
    let post_response2: UserDecryptPostResponseJson =
        response2.json().await.expect("Failed to parse response");
    let job_id_2 = &post_response2.result.job_id;

    // Deduplication: same content should return same job_id
    assert_eq!(job_id_1, job_id_2);

    setup.shutdown().await;
}

#[tokio::test]
async fn test_nonce_too_low_then_succeeds() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_delegated_payload(&payload);

    // First attempt fails, second succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![Response::error("nonce too low".to_string())],
    );
    setup.fhevm_mock.on_user_decrypt_success(
        handles.clone(),
        delegate_address,
        Bytes::from(vec![0x01]),
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
    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_delegated_payload(&payload);

    // First attempt fails with nonce-too-high, second attempt succeeds
    setup.fhevm_mock.queue_tx_responses_for_selector(
        setup.fhevm_mock.decryption_contract,
        constants::USER_DECRYPT_SELECTOR,
        vec![Response::error("nonce too high".to_string())],
    );
    setup.fhevm_mock.on_user_decrypt_success(
        handles.clone(),
        delegate_address,
        Bytes::from(vec![0x01]),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let job_id = helpers::submit_request(&setup, &payload).await;
    let (status, body) = helpers::poll_until_terminal(&setup, &job_id).await;

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body.status, ApiResponseStatus::Succeeded);
    assert!(body.result.is_some());

    setup.shutdown().await;
}

/// Test that retrying a failed request creates a new job_id
#[tokio::test]
async fn test_retry_after_failure_creates_new_job_id() {
    let setup = TestSetup::new_with_low_retries()
        .await
        .expect("Failed to create test setup with low retries");

    let contract_address = helpers::random_address();
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();

    let payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    setup.fhevm_mock.set_readiness_success();
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
    let url = helpers::v2_delegated_user_decrypt_post_url(&setup);

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
    let post_response1: UserDecryptPostResponseJson =
        response1.json().await.expect("Failed to parse response");
    let job_id_1 = post_response1.result.job_id.clone();

    // Wait for it to fail
    let (status1, body1) = helpers::poll_until_terminal(&setup, &job_id_1).await;
    assert_ne!(status1, reqwest::StatusCode::OK);
    assert_eq!(body1.status, ApiResponseStatus::Failed);

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
    let post_response2: UserDecryptPostResponseJson =
        response2.json().await.expect("Failed to parse response");
    let job_id_2 = post_response2.result.job_id.clone();

    // Retry after failure should create a NEW job_id
    assert_ne!(job_id_1, job_id_2);

    setup.shutdown().await;
}

// V2 error format tests

/// Test that malformed JSON returns V2 error format with status and request_id
#[tokio::test]
async fn test_v2_post_malformed_json_has_status_and_request_id() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
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

    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field("extraData", json!("invalid")),
        expect_v2_validation_error("extraData", constants_validation::EXACT_MUST_BE_0X00),
    )
    .await;

    setup.shutdown().await;
}

// Field validation tests - Chain ID and Contract Addresses

#[rstest]
#[case::empty_chain_id("contractsChainId", json!(""), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_decimal("contractsChainId", json!("abc123"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_hex("contractsChainId", json!("0xzzz"), constants_validation::NUMBER_DECIMAL_OR_HEX)]
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
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

// Field validation tests - Delegator/Delegate Address and Handle Pairs

#[rstest]
#[case::empty_delegator_address("delegatorAddress", json!(""), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::short_delegator_address("delegatorAddress", json!("0xfds"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_delegator_address("delegatorAddress", json!("0x1234567890123456789012345678901234567890123"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_delegator_address("delegatorAddress", json!("1234567890123456789012345678901234567890"), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_delegator_address("delegatorAddress", json!("0x123zzz5678901234567890123456789012345678"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::delegator_address_with_invalid_hex_g("delegatorAddress", json!("0x123456789012345678901234567890123456789g"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::empty_delegate_address("delegateAddress", json!(""), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::short_delegate_address("delegateAddress", json!("0xfds"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_delegate_address("delegateAddress", json!("0x1234567890123456789012345678901234567890123"), constants_validation::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_delegate_address("delegateAddress", json!("1234567890123456789012345678901234567890"), constants_validation::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_delegate_address("delegateAddress", json!("0x123zzz5678901234567890123456789012345678"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::delegate_address_with_invalid_hex_g("delegateAddress", json!("0x123456789012345678901234567890123456789g"), constants_validation::HEX_INVALID_CHARACTERS)]
#[case::empty_handle_contract_pairs("handleContractPairs", json!([]), constants_validation::MUST_NOT_BE_EMPTY)]
#[tokio::test]
async fn test_error_invalid_fields_set_2(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

// Field validation tests - Signature, Public Key, Extra Data

#[rstest]
#[case::short_signature("signature", json!("abcdef12"), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::long_signature("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::signature_with_0x_prefix("signature", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::HEX_MUST_NOT_START_WITH_0X)]
#[case::signature_with_invalid_hex_g("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants_validation::HEX_INVALID_STRING)]
#[case::empty_signature("signature", json!(""), constants_validation::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::public_key_with_0x_prefix("publicKey", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants_validation::HEX_MUST_NOT_START_WITH_0X)]
#[case::public_key_with_invalid_hex_g("publicKey", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants_validation::HEX_INVALID_STRING)]
#[case::empty_public_key("publicKey", json!(""), constants_validation::MUST_NOT_BE_EMPTY)]
#[case::empty_extra_data("extraData", json!(""), constants_validation::EXACT_MUST_BE_0X00)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants_validation::EXACT_MUST_BE_0X00)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants_validation::EXACT_MUST_BE_0X00)]
#[tokio::test]
async fn test_error_invalid_fields_set_3(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_v2_validation_error(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

// Field validation tests - Nested handle fields

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
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    base_payload["handleContractPairs"][0]["handle"] = json!(invalid_handle);

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        |_| {},
        expect_v2_validation_error("handleContractPairs", expected_issue),
    )
    .await;

    setup.shutdown().await;
}

// Field validation tests - Timestamp

#[tokio::test]
async fn test_error_future_start_timestamp() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    base_payload["startTimestamp"] = json!(constants::FUTURE_DATE);

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        |_| {},
        expect_v2_validation_error(
            "startTimestamp",
            constants_validation::TIMESTAMP_MUST_NOT_BE_IN_FUTURE,
        ),
    )
    .await;

    setup.shutdown().await;
}

// Missing field tests

#[rstest]
#[case::missing_contracts_chain_id("contractsChainId")]
#[case::missing_contract_addresses("contractAddresses")]
#[case::missing_delegator_address("delegatorAddress")]
#[case::missing_delegate_address("delegateAddress")]
#[case::missing_handle_contract_pairs("handleContractPairs")]
#[case::missing_start_timestamp("startTimestamp")]
#[case::missing_duration_days("durationDays")]
#[case::missing_signature("signature")]
#[case::missing_public_key("publicKey")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let delegator_address = helpers::random_address();
    let delegate_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_delegated_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        delegator_address,
        delegate_address,
    );

    test_endpoint(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        base_payload,
        |p| {
            p.as_object_mut().unwrap().remove(field);
        },
        expect_v2_missing_field(field),
    )
    .await;

    setup.shutdown().await;
}

// Malformed JSON tests

#[rstest]
#[case::missing_closing_brace(r#"{"field": "value""#)]
#[case::missing_comma(r#"{"field1": "value1" "field2": "value2"}"#)]
#[tokio::test]
async fn test_error_malformed_json(#[case] malformed_json: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v2_delegated_user_decrypt_post_url(&setup),
        malformed_json,
        expect_v2_malformed_json(),
    )
    .await;

    setup.shutdown().await;
}
