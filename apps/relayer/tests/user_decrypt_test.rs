mod common;

use crate::common::utils::TestSetup;
use alloy::primitives::{Address, Bytes, B256};
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;

mod constants {
    pub const TIMEOUT_SECS: u64 = 10;
    pub const EXTRA_DATA: &str = "0x00";
    pub const REQUEST_VALIDITY_START: &str = "1742450894";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v1_user_decrypt_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/user-decrypt", setup.http_port)
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
        json!({
            "handleContractPairs": [{
                "handle": handle,
                "contractAddress": format!("{:?}", contract_address)
            }],
            "requestValidity": {
                "startTimestamp": constants::REQUEST_VALIDITY_START,
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
}

#[tokio::test]
async fn test_user_decrypt_success() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);
    let encrypted_bytes = helpers::random_encrypted_bytes();

    // Configure mock for successful response
    setup
        .fhevm_mock
        .on_user_decrypt_success(handles, user_address, encrypted_bytes);

    // Make HTTP request
    let client = reqwest::Client::new();
    let res = client
        .post(helpers::v1_user_decrypt_url(&setup))
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
async fn test_user_decrypt_sequential_requests() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);
    let encrypted_bytes = helpers::random_encrypted_bytes();

    let client = reqwest::Client::new();

    // Make multiple sequential requests
    for i in 0..3 {
        // Configure mock for each request
        setup.fhevm_mock.on_user_decrypt_success(
            handles.clone(),
            user_address,
            encrypted_bytes.clone(),
        );

        // Make HTTP request
        let start = std::time::Instant::now();
        let res = client
            .post(helpers::v1_user_decrypt_url(&setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
            .json(&payload)
            .send()
            .await
            .expect("Request should succeed");
        let duration = start.elapsed();

        // Verify success
        assert_eq!(
            res.status(),
            200,
            "Request {}: {}",
            i + 1,
            res.text().await.unwrap()
        );
        println!(
            "Sequential user decrypt request {} completed in {:?}",
            i + 1,
            duration
        );
    }
}
