mod common;

use crate::common::utils::TestSetup;
use alloy::primitives::{Address, Bytes};
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;

mod constants {
    pub const TIMEOUT_SECS: u64 = 10;
    pub const EXTRA_DATA: &str = "0x00";
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v1_input_proof_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/input-proof", setup.http_port)
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

    pub fn create_input_proof_payload(
        chain_id: &str,
        contract_address: Address,
        user_address: Address,
        ciphertext_hex: &str,
    ) -> serde_json::Value {
        json!({
            "contractChainId": chain_id,
            "contractAddress": format!("{:?}", contract_address),
            "userAddress": format!("{:?}", user_address),
            "ciphertextWithInputVerification": ciphertext_hex,
            "extraData": constants::EXTRA_DATA
        })
    }
}

#[tokio::test]
async fn test_input_proof_reject_by_gateway_error() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let ciphertext_data = helpers::random_bytes();
    let ciphertext_hex = hex::encode(&ciphertext_data);

    // Configure mock to reject this request
    setup
        .fhevm_mock
        .on_input_proof_error(user_address, ciphertext_data);

    // Create payload
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
        &ciphertext_hex,
    );

    // Make HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should complete");

    // Verify rejection
    assert_eq!(res.status(), 400);
    let response_text = res.text().await.unwrap();
    assert!(response_text.contains("Transaction rejected") && response_text.contains("Rejected"));
}

#[tokio::test]
async fn test_input_proof_success() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let ciphertext_data = helpers::random_bytes();
    let ciphertext_hex = hex::encode(&ciphertext_data);

    // Configure mock for successful response
    setup
        .fhevm_mock
        .on_input_proof_success(user_address, ciphertext_data);

    // Create payload
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
        &ciphertext_hex,
    );

    // Make HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should succeed");

    // Verify success
    assert_eq!(res.status(), 200, "Response: {}", res.text().await.unwrap());
}

#[tokio::test]
async fn test_input_proof_concurrent_requests() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let ciphertext_data = helpers::random_bytes();
    let ciphertext_hex = hex::encode(&ciphertext_data);

    // Configure mock for successful responses
    setup
        .fhevm_mock
        .on_input_proof_success(user_address, ciphertext_data);

    // Create payload
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
        &ciphertext_hex,
    );

    // Send multiple concurrent requests
    let mut tasks = tokio::task::JoinSet::new();
    let number_of_requests = 10;
    let url = helpers::v1_input_proof_url(&setup);

    for i in 1..=number_of_requests {
        let url_clone = url.clone();
        let payload_clone = payload.clone();
        tasks.spawn(async move {
            let client = reqwest::Client::new();
            let res = client
                .post(url_clone)
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
                .json(&payload_clone)
                .send()
                .await;
            (res, i)
        });
    }

    // Verify all requests succeed
    while let Some(result) = tasks.join_next().await {
        let (res, index) = result.expect("Task should complete");
        let res = res.expect("HTTP request should succeed");
        assert_eq!(
            res.status(),
            200,
            "Request {}: {}",
            index,
            res.text().await.unwrap()
        );
    }
}

#[tokio::test]
async fn test_input_proof_empty_ciphertext_error() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Create payload with empty ciphertext
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        helpers::random_address(),
        helpers::random_address(),
        "", // Empty ciphertext
    );

    // Make request with empty ciphertext
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should complete");

    // Verify error response
    let status_code = res.status();
    let res_text = res.text().await;
    assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
    if let Ok(ok_text) = res_text {
        match serde_json::Value::from_str(&ok_text) {
            Ok(val) => {
                assert!(
                    val.get("errors").is_some(),
                    "Expected 'errors' field in response"
                );
                let errors = val.get("errors").unwrap();
                assert!(
                    errors.get("ciphertextWithInputVerification").is_some(),
                    "Expected 'ciphertextWithInputVerification' field in errors"
                );
            }
            Err(e) => println!("Returned error text could not be parsed: {}", e),
        }
    }
}

#[tokio::test]
async fn test_input_proof_invalid_contract_address_error() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Create payload with invalid contract address
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        helpers::random_address(),
        helpers::random_address(),
        &hex::encode(helpers::random_bytes()),
    );
    // Override with invalid contract address for this test
    let mut payload = payload;
    payload["contractAddress"] = json!("0xfds");

    // Make request with invalid contract address
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should complete");

    // Verify error response
    let status_code = res.status();
    let res_text = res.text().await;
    assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
    if let Ok(ok_text) = res_text {
        match serde_json::Value::from_str(&ok_text) {
            Ok(val) => {
                assert!(
                    val.get("errors").is_some(),
                    "Expected 'errors' field in response"
                );
                let errors = val.get("errors").unwrap();
                assert!(
                    errors.get("contractAddress").is_some(),
                    "Expected 'contractAddress' field in errors"
                );
                let errors = &errors.get("contractAddress").unwrap().as_array().unwrap()[0];
                assert_eq!(errors.get("code").unwrap(), "invalid_length",);
            }
            Err(e) => println!("Returned error text could not be parsed: {}", e),
        }
    }
}

#[tokio::test]
async fn test_input_proof_invalid_user_address_error() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Create payload with invalid user address
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        helpers::random_address(),
        helpers::random_address(),
        &hex::encode(helpers::random_bytes()),
    );
    // Override with invalid user address for this test
    let mut payload = payload;
    payload["userAddress"] = json!("0xfds");

    // Make request with invalid user address
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should complete");

    // Verify error response
    let status_code = res.status();
    let res_text = res.text().await;
    assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
    if let Ok(ok_text) = res_text {
        match serde_json::Value::from_str(&ok_text) {
            Ok(val) => {
                assert!(
                    val.get("errors").is_some(),
                    "Expected 'errors' field in response"
                );
                let errors = val.get("errors").unwrap();
                assert!(
                    errors.get("userAddress").is_some(),
                    "Expected 'userAddress' field in errors"
                );
                let errors = &errors.get("userAddress").unwrap().as_array().unwrap()[0];
                assert_eq!(errors.get("code").unwrap(), "invalid_length",);
            }
            Err(e) => println!("Returned error text could not be parsed: {}", e),
        }
    }
}

#[tokio::test]
async fn test_input_proof_invalid_hex_error() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Create payload with invalid hex data (odd length)
    let payload = helpers::create_input_proof_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        helpers::random_address(),
        helpers::random_address(),
        "abcdefabcdefs", // Invalid hex (odd length)
    );

    // Make request with invalid hex data
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_input_proof_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should complete");

    // Verify error response
    let status_code = res.status();
    let res_text = res.text().await;
    assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
    if let Ok(ok_text) = res_text {
        match serde_json::Value::from_str(&ok_text) {
            Ok(val) => {
                assert!(
                    val.get("errors").is_some(),
                    "Expected 'errors' field in response"
                );
                let errors = val.get("errors").unwrap();
                assert!(
                    errors.get("ciphertextWithInputVerification").is_some(),
                    "Expected 'ciphertextWithInputVerification' field in errors"
                );
                let errors = &errors
                    .get("ciphertextWithInputVerification")
                    .unwrap()
                    .as_array()
                    .unwrap()[0];
                assert_eq!(errors.get("code").unwrap(), "invalid_hex_characters",);
            }
            Err(e) => println!("Returned error text could not be parsed: {}", e),
        }
    }
}
