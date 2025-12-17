mod common;

use crate::common::utils::{assert_retry_after_header_present, TestSetup};
use alloy::primitives::{Address, Bytes};
use fhevm_relayer::http::endpoints::v2::types::input_proof::{
    InputProofPostResponseJson, InputProofStatusResponseJson,
};
use rand::{rng, Rng};
use serde_json::json;

mod constants {
    pub const EXTRA_DATA: &str = "0x00";
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v2_input_proof_post_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/input-proof", setup.http_port)
    }

    pub fn v2_input_proof_get_url(setup: &TestSetup, job_id: &str) -> String {
        format!(
            "http://localhost:{}/v2/input-proof/{}",
            setup.http_port, job_id
        )
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

    pub fn create_input_proof_payload(setup: &TestSetup) -> (serde_json::Value, Address, Bytes) {
        let contract_address = random_address();
        let user_address = random_address();
        let ciphertext_data = random_bytes();

        let payload = json!({
            "contractChainId": setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
            "contractAddress": format!("{:?}", contract_address),
            "userAddress": format!("{:?}", user_address),
            "ciphertextWithInputVerification": hex::encode(&ciphertext_data),
            "extraData": constants::EXTRA_DATA
        });

        (payload, user_address, ciphertext_data)
    }
}

#[tokio::test]
async fn test_success_single_request() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data,
        1,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // Step 1: POST request should return reference ID
    let response = reqwest::Client::new()
        .post(helpers::v2_input_proof_post_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send POST request");

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
    assert_retry_after_header_present(&response);

    let post_response: InputProofPostResponseJson = response
        .json()
        .await
        .expect("Failed to parse POST response");

    assert_eq!(post_response.status, "queued");
    let job_id = &post_response.result.job_id;

    // Step 2: GET request should eventually return completed result
    // Give some time for processing
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    let get_response = reqwest::Client::new()
        .get(helpers::v2_input_proof_get_url(&setup, job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send GET request");

    let status = get_response.status();

    // Check Retry-After header before consuming response
    if status == reqwest::StatusCode::ACCEPTED {
        assert_retry_after_header_present(&get_response);
    }

    let get_body: InputProofStatusResponseJson = get_response
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
