mod common;

use crate::common::utils::TestSetup;
use alloy::primitives::{Address, Bytes, B256};
use fhevm_relayer::http::endpoints::v2::types::user_decrypt::{
    UserDecryptPostResponseJson, UserDecryptStatusResponseJson,
};
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

mod constants {
    pub const EXTRA_DATA: &str = "0x00";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";
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

    setup
        .fhevm_mock
        .on_user_decrypt_success(handles, user_address, encrypted_bytes);

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
    let get_body: UserDecryptStatusResponseJson = get_response
        .json()
        .await
        .expect("Failed to parse GET response");

    // Should be either succeeded (200) or still queued (202)
    match status {
        reqwest::StatusCode::OK => {
            assert_eq!(get_body.status, "succeeded");
            assert!(get_body.result.is_some());

            // Validate the response structure includes extra_data
            let result = get_body.result.unwrap();
            assert!(!result.payloads.is_empty(), "Payloads should not be empty");
            assert!(
                !result.signatures.is_empty(),
                "Signatures should not be empty"
            );
            assert!(
                !result.extra_data.is_empty(),
                "Extra data should not be empty"
            );

            // Ensure extra_data has the same length as payloads/signatures
            assert_eq!(
                result.payloads.len(),
                result.signatures.len(),
                "Payloads and signatures should have same length"
            );
            assert_eq!(
                result.payloads.len(),
                result.extra_data.len(),
                "Payloads and extra_data should have same length"
            );

            // Verify extra_data format (should be hex with 0x prefix)
            for extra_data_item in &result.extra_data {
                assert!(
                    extra_data_item.starts_with("0x"),
                    "Extra data should start with 0x prefix: {}",
                    extra_data_item
                );
            }

            println!(
                "✅ V2 response validation passed: {} payloads, {} signatures, {} extra_data items",
                result.payloads.len(),
                result.signatures.len(),
                result.extra_data.len()
            );
        }
        reqwest::StatusCode::ACCEPTED => {
            assert_eq!(get_body.status, "queued");
        }
        _ => panic!("Unexpected status code: {}", status),
    }
}
