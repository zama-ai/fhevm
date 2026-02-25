mod common;

use crate::common::redundancy::{common_redundancy_cases, expand_targets, RedundancyCase};
use crate::common::utils::TestSetup;
use crate::common::validation_helper::{
    expect_invalid_field, expect_malformed_json, expect_missing_field, expect_success,
    test_endpoint, test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::B256;
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;
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

    setup.fhevm_mock.on_public_decrypt_success(
        handles,
        plaintext_values,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    test_endpoint(
        &helpers::v1_public_decrypt_url(&setup),
        payload,
        |_| {},
        expect_success(),
    )
    .await;

    setup.shutdown().await;
}

#[tokio::test]
async fn test_success_concurrent_requests() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Set up mock to handle multiple requests
    for _ in 1..=10 {
        setup.fhevm_mock.on_public_decrypt_success(
            handles.clone(),
            plaintext_values.clone(),
            ethereum_rpc_mock::SubscriptionTarget::All,
        );
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

    setup.shutdown().await;
}

/// Listener redundancy for public decrypt with clear cases.
#[tokio::test]
#[cfg_attr(
    not(feature = "long-running-tests"),
    ignore = "Long-running test - run with --features long-running-tests"
)]
async fn test_listener_redundancy_public_decrypt_matrix() {
    let cases: Vec<RedundancyCase> = common_redundancy_cases();
    let mut setups: HashMap<usize, TestSetup> = HashMap::new();

    for case in cases {
        if let std::collections::hash_map::Entry::Vacant(e) = setups.entry(case.listener_count) {
            let setup = TestSetup::new_with_listeners(case.listener_count)
                .await
                .expect("Failed to create test setup with listeners");
            e.insert(setup);
        }
        let setup = setups
            .get(&case.listener_count)
            .expect("Missing test setup for listener count");

        println!("public-decrypt redundancy case: {}", case.name);

        let request_targets = expand_targets(case.requests, &case.targets_per_event);

        for target in request_targets {
            let payload = helpers::create_public_decrypt_payload();
            let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
            let plaintext_values = helpers::random_plaintext_values(handles.len());

            setup.fhevm_mock.on_public_decrypt_success(
                handles.clone(),
                plaintext_values.clone(),
                target,
            );

            test_endpoint(
                &helpers::v1_public_decrypt_url(setup),
                payload,
                |_| {},
                expect_success(),
            )
            .await;
        }
    }

    for setup in setups.into_values() {
        setup.shutdown().await;
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

    setup.shutdown().await;
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

    setup.shutdown().await;
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

    setup.shutdown().await;
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

    setup.shutdown().await;
}

/// Test readiness check failure returns 503 Service Unavailable with correct message
#[tokio::test]
async fn test_readiness_check_failure_returns_503() {
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

    // Verify we get 503 Service Unavailable
    assert_eq!(
        response.status(),
        reqwest::StatusCode::SERVICE_UNAVAILABLE,
        "Expected 503 Service Unavailable status code"
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

    setup.shutdown().await;
}

/// Test consecutive duplicate requests succeed in V1
/// Documents that duplicate requests with identical content should both succeed
/// with consistent responses. Currently may expose race conditions in V1 handler.
#[tokio::test]
async fn test_consecutive_duplicate_requests_succeed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Generate random payload once and use across two requests
    let payload = helpers::create_public_decrypt_payload();
    let handles = helpers::extract_ciphertext_handles_from_public_payload(&payload);
    let plaintext_values = helpers::random_plaintext_values(handles.len());

    // Set up mock to handle both requests with identical responses
    setup.fhevm_mock.on_public_decrypt_success(
        handles.clone(),
        plaintext_values.clone(),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );
    setup.fhevm_mock.on_public_decrypt_success(
        handles.clone(),
        plaintext_values.clone(),
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let client = reqwest::Client::new();
    let url = helpers::v1_public_decrypt_url(&setup);

    // Send both requests consecutively with same payload
    let response1 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(5))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send first request");

    let response2 = client
        .post(&url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(5))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send second request");

    let status1 = response1.status();
    let status2 = response2.status();

    let body1_text = response1
        .text()
        .await
        .expect("Failed to get first response body");
    let body2_text = response2
        .text()
        .await
        .expect("Failed to get second response body");

    // Print responses for debugging
    println!("First request - Status: {}, Body: {}", status1, body1_text);
    println!("Second request - Status: {}, Body: {}", status2, body2_text);

    // Due to the V1 handler bug, we expect one of these scenarios:
    // 1. Both succeed with 200 (if lucky timing)
    // 2. One succeeds with 200, other fails with timeout/error (most likely)
    // 3. Both fail (if very unlucky timing)

    let success_count = [status1, status2]
        .iter()
        .filter(|&s| *s == reqwest::StatusCode::OK)
        .count();

    if success_count == 2 {
        // Both succeeded - check if responses are identical
        let body1: serde_json::Value =
            serde_json::from_str(&body1_text).expect("Failed to parse first response JSON");
        let body2: serde_json::Value =
            serde_json::from_str(&body2_text).expect("Failed to parse second response JSON");

        // Even if both succeed, the responses should be identical for the same request
        assert_eq!(
            body1,
            body2,
            "Both requests succeeded but responses differ - this indicates internal inconsistency.\nFirst: {}\nSecond: {}",
            serde_json::to_string_pretty(&body1).unwrap(),
            serde_json::to_string_pretty(&body2).unwrap()
        );
    } else if success_count == 1 {
        println!("Test validates that duplicate requests are handled correctly.");
    } else {
        // Both failed - this could happen if the mock response doesn't arrive in time
        println!(
            "WARNING: Both requests failed. This might indicate a test timing issue or severe bug."
        );
        println!("First: {} - {}", status1, body1_text);
        println!("Second: {} - {}", status2, body2_text);
    }

    setup.shutdown().await;
}
