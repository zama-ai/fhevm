//! Clean FHEVM helper functions for tests
//!
//! These helpers provide a clean API for setting up FHEVM mock patterns
//! for isolated test setups.

use alloy::primitives::{Address, Bytes, B256};
use ethereum_rpc_mock::fhevm::FhevmMockWrapper;
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;

/// FHEVM mock setup extensions for isolated testing
pub trait FhevmMockSetup {
    /// Setup FHEVM mock for successful public decryption
    fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>);

    /// Setup FHEVM mock for successful user decryption
    fn setup_for_user_decrypt_success_response(
        &self,
        user_address: Address,
        ciphertext_handles: Vec<B256>,
    );

    /// Setup FHEVM mock for successful input proof verification
    fn setup_for_input_proof_success_response(&self, user_address: Address, ciphertext_data: Bytes);

    /// Setup FHEVM mock for input proof rejection
    fn setup_for_input_proof_reject_response(&self, user_address: Address, ciphertext_data: Bytes);
}

impl FhevmMockSetup for FhevmMockWrapper {
    fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>) {
        let plaintext_values = vec![42u64; ciphertext_handles.len()];
        self.on_public_decrypt_success(ciphertext_handles, plaintext_values);
    }

    fn setup_for_user_decrypt_success_response(
        &self,
        user_address: Address,
        ciphertext_handles: Vec<B256>,
    ) {
        let encrypted_bytes = Bytes::from(vec![42u8; 32]);
        self.on_user_decrypt_success(ciphertext_handles, user_address, encrypted_bytes);
    }

    fn setup_for_input_proof_success_response(
        &self,
        user_address: Address,
        ciphertext_data: Bytes,
    ) {
        self.on_input_proof_success(user_address, ciphertext_data);
    }

    fn setup_for_input_proof_reject_response(&self, user_address: Address, ciphertext_data: Bytes) {
        self.on_input_proof_error(user_address, ciphertext_data);
    }
}

/// Extract ciphertext handles from public decrypt payload
pub fn extract_ciphertext_handles_from_public_payload(payload: &serde_json::Value) -> Vec<B256> {
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

/// Extract ciphertext handles from user decrypt payload
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

/// Generate a random ciphertext handle
pub fn random_handle() -> String {
    let mut rng = rng();
    (0..64)
        .map(|_| rng.random_range(0..16))
        .map(|digit| format!("{:x}", digit))
        .collect()
}

/// Generate a random payload for public decryption
pub fn random_payload_for_public_decrypt() -> serde_json::Value {
    let random_handle = random_handle();
    json!({"ciphertextHandles": [random_handle], "extraData": "0x00"})
}

/// Generate a random payload for user decryption
pub fn random_payload_for_user_decrypt() -> serde_json::Value {
    let random_handle = random_handle();
    json!({"handleContractPairs":[{"handle":random_handle,"contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a","extraData":"0x00"})
}

/// Make HTTP POST request to public decrypt endpoint
pub async fn post_public_decrypt(
    client: &reqwest::Client,
    base_url: &str,
    payload: &serde_json::Value,
    timeout_secs: u64,
) -> (reqwest::Response, std::time::Duration) {
    let start = tokio::time::Instant::now();
    let res = client
        .post(format!("{}/v1/public-decrypt", base_url))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .json(payload)
        .send()
        .await
        .unwrap();
    let elapsed = start.elapsed();

    // Print error message if status is not 200 OK
    let status = res.status();
    if status != 200 {
        let error_text = res
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read error response".to_string());
        tracing::info!(
            "POST /v1/public-decrypt failed with status {}: {}",
            status,
            error_text
        );
        panic!(
            "POST /v1/public-decrypt failed with status {}: {}",
            status, error_text
        );
    }

    (res, elapsed)
}

/// Make HTTP POST request to user decrypt endpoint
pub async fn post_user_decrypt(
    client: &reqwest::Client,
    base_url: &str,
    payload: &serde_json::Value,
    timeout_secs: u64,
) -> (reqwest::Response, std::time::Duration) {
    let start = tokio::time::Instant::now();
    let res = client
        .post(format!("{}/v1/user-decrypt", base_url))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .json(payload)
        .send()
        .await
        .unwrap();
    let elapsed = start.elapsed();

    // Print error message if status is not 200 OK
    let status = res.status();
    if status != 200 {
        let error_text = res
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read error response".to_string());
        tracing::info!(
            "POST /v1/user-decrypt failed with status {}: {}",
            status,
            error_text
        );
        panic!(
            "POST /v1/user-decrypt failed with status {}: {}",
            status, error_text
        );
    }

    (res, elapsed)
}
