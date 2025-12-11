mod common;

use crate::common::utils::TestSetup;
use crate::common::validation_helper::{
    expect_invalid_field, expect_malformed_json, expect_missing_field, expect_success,
    test_endpoint, test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::B256;
use rand::{rng, Rng};
use rstest::rstest;
use serde_json::json;
use std::str::FromStr;

mod constants {
    pub const EXTRA_DATA: &str = "0x00";

    // Validation error messages (directly from source code)
    pub use fhevm_relayer::http::validation_messages::*;
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
async fn test_success_single_request() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    setup
        .fhevm_mock
        .on_public_decrypt_success(handles, plaintext_values);

    test_endpoint(
        &helpers::v1_public_decrypt_url(&setup),
        payload,
        |_| {},
        expect_success(),
    )
    .await;
}

#[tokio::test]
async fn test_success_concurrent_requests() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Set up mock to handle multiple requests
    for _ in 1..=10 {
        setup
            .fhevm_mock
            .on_public_decrypt_success(handles.clone(), plaintext_values.clone());
    }

    // Send multiple concurrent requests using test_endpoint
    let mut tasks = tokio::task::JoinSet::new();
    let number_of_requests = 10;

    for i in 1..=number_of_requests {
        let payload_clone = payload.clone();
        let url = helpers::v1_public_decrypt_url(&setup);
        tasks.spawn(async move {
            test_endpoint(
                &url,
                payload_clone,
                |_| {}, // No modifications needed
                expect_success(),
            )
            .await;
            i // Return request index for tracking
        });
    }

    // Wait for all requests to complete
    while let Some(result) = tasks.join_next().await {
        let index = result.expect("Task should complete");
        println!("Concurrent request {} completed successfully", index);
    }
}

#[rstest]
// Ciphertext handles validation
#[case::empty_ciphertext_handles("ciphertextHandles", json!([]), constants::MUST_NOT_BE_EMPTY)]
#[case::invalid_hex_ciphertext_handle("ciphertextHandles", json!(["0xabcdefabcdefs"]), constants::HEX_INVALID_STRING)]
#[case::odd_length_ciphertext_handle("ciphertextHandles", json!(["0xabcdef1"]), constants::HEX_INVALID_STRING)]
#[case::ciphertext_handle_with_invalid_hex_g("ciphertextHandles", json!(["0xabcdefg"]), constants::HEX_INVALID_STRING)]
#[case::ciphertext_handle_without_0x_prefix("ciphertextHandles", json!(["abcdef123456789012345678901234567890123456789012345678901234567890"]), constants::HEX_MUST_START_WITH_0X)]
#[case::empty_string_ciphertext_handle("ciphertextHandles", json!([""]), constants::HEX_MUST_START_WITH_0X)]
// Extra data validation
#[case::empty_extra_data("extraData", json!(""), constants::EXACT_MUST_BE_0X00)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants::EXACT_MUST_BE_0X00)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants::EXACT_MUST_BE_0X00)]
#[tokio::test]
async fn test_error_invalid_fields(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v1_public_decrypt_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;
}

#[rstest]
#[case::missing_ciphertext_handles("ciphertextHandles")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v1_public_decrypt_url(&setup),
        base_payload,
        |p| {
            p.as_object_mut().unwrap().remove(field);
        },
        expect_missing_field(field),
    )
    .await;
}

#[rstest]
#[case::both_fields(["ciphertextHandles", "extraData"], "ciphertextHandles")]
#[tokio::test]
async fn test_error_missing_two_fields_reports_first_only(
    #[case] fields_to_remove: [&str; 2],
    #[case] expected_reported_field: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let base_payload = helpers::create_public_decrypt_payload();

    test_endpoint(
        &helpers::v1_public_decrypt_url(&setup),
        base_payload,
        |p| {
            for field in &fields_to_remove {
                p.as_object_mut().unwrap().remove(*field);
            }
        },
        expect_missing_field(expected_reported_field), // Only expect the first field to be reported
    )
    .await;
}

#[rstest]
#[case::missing_closing_brace(r#"{"field": "value""#)]
#[case::missing_comma(r#"{"field1": "value1" "field2": "value2"}"#)]
#[tokio::test]
async fn test_error_malformed_json(#[case] malformed_json: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    test_endpoint_raw_body(
        &helpers::v1_public_decrypt_url(&setup),
        malformed_json,
        expect_malformed_json(),
    )
    .await;
}

/// Test readiness check failure returns 504 Gateway Timeout with correct message
#[tokio::test]
async fn test_readiness_check_failure_returns_504() {
    // Use fast readiness config (4 attempts × 250ms = ~1s total)
    let setup = TestSetup::new_with_fast_readiness()
        .await
        .expect("Failed to create test setup");

    // Configure mock to always return false for readiness checks
    setup.fhevm_mock.set_readiness_failure();

    let payload = helpers::create_public_decrypt_payload();

    // Make direct HTTP request to test the endpoint
    let response = reqwest::Client::new()
        .post(helpers::v1_public_decrypt_url(&setup))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send HTTP request");

    // Verify we get 504 Gateway Timeout
    assert_eq!(
        response.status(),
        reqwest::StatusCode::GATEWAY_TIMEOUT,
        "Expected 504 Gateway Timeout status code"
    );

    // Verify error message
    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response JSON");

    assert_eq!(
        body["message"].as_str(),
        Some("Ciphertext not ready for decryption on the gateway chain"),
        "Expected readiness failure error message"
    );
}
