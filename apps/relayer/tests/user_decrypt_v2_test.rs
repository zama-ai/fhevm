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
        let test_extra_data = Bytes::from(vec![0x00]);

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
        assert!(
            v2_item["extra_data"].is_string(),
            "v2 extra_data should be string"
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

        // Note: v1 doesn't serialize extra_data, so we only verify v2 has it
        assert_eq!(
            v2_item["extra_data"], "00",
            "v2 extra_data should be hex encoded"
        );
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
                assert!(
                    !result_item.extra_data.is_empty(),
                    "Extra data should not be empty"
                );
            }
        }
        reqwest::StatusCode::ACCEPTED => {
            assert_eq!(get_body.status, "queued");
        }
        _ => panic!("Unexpected status code: {}", status),
    }
}

#[test]
fn test_tkms_compatibility() {
    // Validates response format compatibility with TKMS library for plaintext reconstruction
    helpers::verify_tkms_compatibility();
}
