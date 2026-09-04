//! Per-flow drivers for the v2 request lifecycle (POST → jobId → poll GET
//! until terminal). One submodule per flow, each exposing the request-payload
//! builders and the two lifecycle calls its endpoint needs.
//!
//! Shared so that suites which drive several flows at once — notably
//! `listener_redundancy_test` — do not have to restate each flow's plumbing.

#![allow(dead_code)]

use crate::common::utils::{self, TestSetup};
use alloy::primitives::{Address, Bytes, B256};
use fhevm_relayer::http::endpoints::v2::types::error::ApiResponseStatus;
use rand::{rng, RngExt};
use serde_json::json;
use std::str::FromStr;

/// `extraData` accepted by every v2 endpoint.
pub const EXTRA_DATA: &str = "0x00";

/// Delay between two GET polls, and the number of polls before giving up.
const POLL_INTERVAL_MS: u64 = 500;
const POLL_ATTEMPTS: usize = 10;
const REQUEST_TIMEOUT_SECS: u64 = 10;

pub mod public_decrypt {
    use super::*;
    use fhevm_relayer::http::endpoints::v2::types::public_decrypt::{
        PublicDecryptPostResponseJson, PublicDecryptStatusResponseJson,
    };

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
            "extraData": EXTRA_DATA
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

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_public_decrypt_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(payload)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
        let post_response: PublicDecryptPostResponseJson = response
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
    ) -> (reqwest::StatusCode, PublicDecryptStatusResponseJson) {
        let client = reqwest::Client::new();
        for _ in 0..POLL_ATTEMPTS {
            tokio::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
            let response = client
                .get(v2_public_decrypt_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
                .send()
                .await
                .expect("Failed to send GET request");

            let status = response.status();
            if status != reqwest::StatusCode::ACCEPTED {
                let body: PublicDecryptStatusResponseJson =
                    response.json().await.expect("Failed to parse GET response");
                return (status, body);
            }
        }
        panic!("Request did not reach terminal state in time");
    }
}

pub mod input_proof {
    use super::*;
    use fhevm_relayer::http::endpoints::v2::types::input_proof::{
        InputProofPostResponseJson, InputProofStatusResponseJson,
    };

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
            "extraData": EXTRA_DATA
        });

        (payload, user_address, ciphertext_data)
    }

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_input_proof_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .json(payload)
            .send()
            .await
            .expect("Failed to send POST request");

        assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);
        let post_response: InputProofPostResponseJson = response
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
    ) -> (reqwest::StatusCode, InputProofStatusResponseJson) {
        let client = reqwest::Client::new();
        for _ in 0..POLL_ATTEMPTS {
            tokio::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
            let response = client
                .get(v2_input_proof_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
                .send()
                .await
                .expect("Failed to send GET request");

            let status = response.status();
            if status != reqwest::StatusCode::ACCEPTED {
                let body: InputProofStatusResponseJson =
                    response.json().await.expect("Failed to parse GET response");
                return (status, body);
            }
        }
        panic!("Request did not reach terminal state in time");
    }
}

pub mod user_decrypt {
    use super::*;
    use fhevm_relayer::http::endpoints::v2::types::user_decrypt::{
        UserDecryptPostResponseJson, UserDecryptStatusResponseJson,
    };
    use std::time::{SystemTime, UNIX_EPOCH};

    pub const REQUEST_VALIDITY_DAYS: &str = "10";

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
                "durationDays": REQUEST_VALIDITY_DAYS
            },
            "contractsChainId": chain_id,
            "contractAddresses": [format!("{:?}", contract_address)],
            "userAddress": format!("{:?}", user_address),
            "signature": random_signature(),
            "publicKey": random_public_key(),
            "extraData": EXTRA_DATA
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

    /// Submit POST request and return job_id
    pub async fn submit_request(setup: &TestSetup, payload: &serde_json::Value) -> String {
        let response = reqwest::Client::new()
            .post(v2_user_decrypt_post_url(setup))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
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
        for _ in 0..POLL_ATTEMPTS {
            tokio::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
            let response = client
                .get(v2_user_decrypt_get_url(setup, job_id))
                .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
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
