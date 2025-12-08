mod common;

use crate::common::utils::TestSetup;
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

    setup
        .fhevm_mock
        .on_public_decrypt_success(handles, plaintext_values);

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
}
