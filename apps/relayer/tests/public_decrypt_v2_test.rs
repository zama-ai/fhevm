mod common;

use crate::common::utils::{assert_retry_after_header_present, TestSetup};
use alloy::primitives::B256;
use fhevm_relayer::http::endpoints::v2::types::public_decrypt::{
    PublicDecryptPostResponseJson, PublicDecryptStatusResponseJson,
};
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;

mod constants {
    pub const EXTRA_DATA: &str = "0x00";
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

    // Step 1: POST request should return reference ID
    let response = reqwest::Client::new()
        .post(helpers::v2_public_decrypt_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send POST request");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response);

    let post_response: PublicDecryptPostResponseJson = response
        .json()
        .await
        .expect("Failed to parse POST response");

    assert_eq!(post_response.status, "queued");
    let job_id = &post_response.result.job_id;

    // Step 2: GET request should eventually return completed result
    // Give some time for processing
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    let get_response = reqwest::Client::new()
        .get(helpers::v2_public_decrypt_get_url(&setup, job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send GET request");

    let status = get_response.status();

    // Check Retry-After header before consuming response
    if status == reqwest::StatusCode::ACCEPTED {
        assert_retry_after_header_present(&get_response);
    }

    let get_body: PublicDecryptStatusResponseJson = get_response
        .json()
        .await
        .expect("Failed to parse GET response");

    // Should be either succeeded (200) or still queued (202)
    match status {
        reqwest::StatusCode::OK => {
            assert_eq!(get_body.status, "succeeded");
            assert!(get_body.result.is_some());
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

    let post_response2: PublicDecryptPostResponseJson = response2
        .json()
        .await
        .expect("Failed to parse second POST response");

    assert_eq!(post_response2.status, "queued");
    let job_id_2 = &post_response2.result.job_id;

    // Print job_ids for debugging
    println!("First request job_id: {}", job_id_1);
    println!("Second request job_id: {}", job_id_2);

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
