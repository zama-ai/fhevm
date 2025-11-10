mod common;

use crate::common::utils::TestSetup;
use alloy::primitives::B256;
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

    pub fn v1_public_decrypt_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/public-decrypt", setup.http_port)
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
}

#[tokio::test]
async fn test_public_decrypt_success() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Configure mock for successful response
    setup
        .fhevm_mock
        .on_public_decrypt_success(handles, plaintext_values);

    // Make HTTP request
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();
    let res = client
        .post(helpers::v1_public_decrypt_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should succeed");
    let duration = start.elapsed();

    // Verify success
    assert_eq!(res.status(), 200, "Response: {}", res.text().await.unwrap());
    println!("Public decrypt request completed in {:?}", duration);
}

#[tokio::test]
async fn test_public_decrypt_sequential_requests() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    let client = reqwest::Client::new();

    // Make first request
    setup
        .fhevm_mock
        .on_public_decrypt_success(handles.clone(), plaintext_values.clone());
    let start = std::time::Instant::now();
    let res = client
        .post(helpers::v1_public_decrypt_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
        .json(&payload)
        .send()
        .await
        .expect("Request should succeed");
    let duration = start.elapsed();
    assert_eq!(
        res.status(),
        200,
        "First request: {}",
        res.text().await.unwrap()
    );
    println!("First public decrypt request took: {:?}", duration);

    // Make sequential requests
    for i in 0..3 {
        setup
            .fhevm_mock
            .on_public_decrypt_success(handles.clone(), plaintext_values.clone());
        let start = std::time::Instant::now();
        let res = client
            .post(helpers::v1_public_decrypt_url(&setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
            .json(&payload)
            .send()
            .await
            .expect("Request should succeed");
        let duration = start.elapsed();
        assert_eq!(
            res.status(),
            200,
            "Sequential request {}: {}",
            i + 1,
            res.text().await.unwrap()
        );
        println!(
            "Sequential public decrypt request {} took: {:?}",
            i + 1,
            duration
        );
    }
}

#[tokio::test]
async fn test_public_decrypt_concurrent_requests() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    let number_of_requests = 5;

    // Send concurrent requests
    let mut tasks = tokio::task::JoinSet::new();
    let url = helpers::v1_public_decrypt_url(&setup);

    for i in 0..number_of_requests {
        let payload_clone = payload.clone();
        let url_clone = url.clone();
        let handles_clone = handles.clone();
        let plaintext_values_clone = plaintext_values.clone();
        let fhevm_mock_clone = setup.fhevm_mock.clone();

        tasks.spawn(async move {
            // Configure mock for this request
            fhevm_mock_clone.on_public_decrypt_success(handles_clone, plaintext_values_clone);

            // Make HTTP request
            let client = reqwest::Client::new();
            let start = std::time::Instant::now();
            let res = client
                .post(url_clone)
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(constants::TIMEOUT_SECS))
                .json(&payload_clone)
                .send()
                .await
                .expect("Request should succeed");
            let duration = start.elapsed();

            assert_eq!(
                res.status(),
                200,
                "Concurrent request {}: {}",
                i + 1,
                res.text().await.unwrap()
            );
            println!(
                "Concurrent public decrypt request {} took: {:?}",
                i + 1,
                duration
            );
            duration
        });
    }

    // Wait for all requests to complete
    let mut durations = Vec::new();
    while let Some(result) = tasks.join_next().await {
        let duration = result.expect("Task should complete successfully");
        durations.push(duration.as_micros());
    }

    durations.sort();
    println!("All concurrent request timings: {:?}μs", durations);
}
