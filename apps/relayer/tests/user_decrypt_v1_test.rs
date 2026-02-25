mod common;

use crate::common::redundancy::{
    common_redundancy_cases, expand_targets, user_only_redundancy_cases, USER_DECRYPT_EVENT_COUNT,
};
use crate::common::utils::TestSetup;
use crate::common::validation_helper::{
    expect_invalid_field, expect_malformed_json, expect_missing_field, expect_success,
    test_endpoint, test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Address, Bytes, B256};
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

mod constants {
    pub const EXTRA_DATA: &str = "0x00";
    // Should failed since this date is in the future (2035)
    pub const FUTURE_DATE: &str = "2051218800";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";

    // Validation error messages (directly from source code)
    pub use fhevm_relayer::http::validation_messages::*;
}

mod helpers {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::common::utils;

    pub fn v1_user_decrypt_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/user-decrypt", setup.http_port)
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

    setup.fhevm_mock.on_user_decrypt_success(
        handles,
        user_address,
        encrypted_bytes,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        payload,
        |_| {},
        expect_success(),
    )
    .await;

    // V1 only: Consensus event arrives 1 block (~500ms) after shares. Sleep keeps relayer running to process it.
    // V2 tests already have sleep between POST and GET. In production, relayer runs continuously so no timing issues.
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    setup.shutdown().await;
}

/// Test consecutive duplicate requests succeed in V1
/// Documents that duplicate requests with identical content should both succeed
/// with consistent responses. Currently may expose race conditions in V1 handler.
#[tokio::test]
async fn test_consecutive_duplicate_requests_succeed() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();

    // Generate random payload once and use across two requests
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);
    let encrypted_bytes = helpers::random_encrypted_bytes();

    setup.fhevm_mock.on_user_decrypt_success(
        handles.clone(),
        user_address,
        encrypted_bytes,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    let url = helpers::v1_user_decrypt_url(&setup);
    let payload_clone = payload.clone();

    // Send both requests concurrently using tokio::spawn to expose the deduplication bug
    let request1 = tokio::spawn({
        let url = url.clone();
        let payload = payload.clone();
        async move {
            reqwest::Client::new()
                .post(&url)
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(10))
                .json(&payload)
                .send()
                .await
        }
    });

    let request2 = tokio::spawn({
        let url = url.clone();
        let payload = payload_clone;
        async move {
            reqwest::Client::new()
                .post(&url)
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(10))
                .json(&payload)
                .send()
                .await
        }
    });

    // Wait for both to complete
    let (result1, result2) = tokio::join!(request1, request2);

    let response1 = result1
        .expect("First request task failed")
        .expect("Failed to send first request");
    let response2 = result2
        .expect("Second request task failed")
        .expect("Failed to send second request");

    let response1_status = response1.status();
    let response2_status = response2.status();

    let response1_text = response1
        .text()
        .await
        .expect("Failed to get first response text");
    let response2_text = response2
        .text()
        .await
        .expect("Failed to get second response text");

    assert_eq!(
        response1_status,
        reqwest::StatusCode::OK,
        "First request should return 200 OK. Got status: {} with body: {}",
        response1_status,
        response1_text
    );

    assert_eq!(
        response2_status,
        reqwest::StatusCode::OK,
        "Second request should return 200 OK. Got status: {} with body: {}",
        response2_status,
        response2_text
    );

    // Step 3: CRITICAL TEST - Both responses should be identical
    // This documents the expected behavior where duplicate consecutive requests
    // should return consistent responses
    assert_eq!(
        response1_text, response2_text,
        "Duplicate requests should return identical responses.\n\
         First response: {}\nSecond response: {}",
        response1_text, response2_text
    );

    // V1 only: Consensus event arrives 1 block (~500ms) after shares. Sleep keeps relayer running to process it.
    // V2 tests already have sleep between POST and GET. In production, relayer runs continuously so no timing issues.
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    setup.shutdown().await;
}

#[tokio::test]
async fn test_success_concurrent_requests() {
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

    setup.fhevm_mock.on_user_decrypt_success(
        handles,
        user_address,
        encrypted_bytes,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    // Send multiple concurrent requests using test_endpoint
    let mut tasks = tokio::task::JoinSet::new();
    let number_of_requests = 5;

    for i in 1..=number_of_requests {
        let payload_clone = payload.clone();
        let url = helpers::v1_user_decrypt_url(&setup);
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

    // V1 only: Consensus event arrives 1 block (~500ms) after shares. Sleep keeps relayer running to process it.
    // V2 tests already have sleep between POST and GET. In production, relayer runs continuously so no timing issues.
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    setup.shutdown().await;
}

/// Listener redundancy scenarios for user decrypt with clearer case descriptions.
#[tokio::test]
#[cfg_attr(
    not(feature = "long-running-tests"),
    ignore = "Long-running test - run with --features long-running-tests"
)]
async fn test_listener_redundancy_user_decrypt_matrix() {
    let mut cases = common_redundancy_cases();
    cases.extend(user_only_redundancy_cases());
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

        let user_address = helpers::random_address();
        let contract_address = helpers::random_address();
        println!("user-decrypt redundancy case: {}", case.name);

        for _ in 0..case.requests {
            let payload = helpers::create_user_decrypt_payload(
                &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
                contract_address,
                user_address,
            );
            let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);
            let encrypted_bytes = helpers::random_encrypted_bytes();

            let per_event_targets =
                expand_targets(USER_DECRYPT_EVENT_COUNT, &case.targets_per_event);

            setup.fhevm_mock.on_user_decrypt_success_with_targets(
                handles.clone(),
                user_address,
                encrypted_bytes,
                per_event_targets,
            );

            test_endpoint(
                &helpers::v1_user_decrypt_url(setup),
                payload.clone(),
                |_| {},
                expect_success(),
            )
            .await;
        }

        // Allow consensus event to be processed
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    for setup in setups.into_values() {
        setup.shutdown().await;
    }
}

#[rstest]
// Chain ID validation
#[case::empty_chain_id("contractsChainId", json!(""), constants::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_decimal("contractsChainId", json!("abc123"), constants::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_hex("contractsChainId", json!("0xzzz"), constants::NUMBER_DECIMAL_OR_HEX)]
// Contract addresses validation
#[case::empty_contract_addresses("contractAddresses", json!([]), constants::MUST_NOT_BE_EMPTY)]
#[case::short_contract_address("contractAddresses", json!(["0xfds"]), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_contract_address("contractAddresses", json!(["0x1234567890123456789012345678901234567890123"]), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_contract_address("contractAddresses", json!(["1234567890123456789012345678901234567890"]), constants::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_contract_address("contractAddresses", json!(["0x123zzz5678901234567890123456789012345678"]), constants::HEX_INVALID_CHARACTERS)]
#[case::contract_address_with_invalid_hex_g("contractAddresses", json!(["0x123456789012345678901234567890123456789g"]), constants::HEX_INVALID_CHARACTERS)]
#[case::empty_string_contract_address("contractAddresses", json!([""]), constants::HEX_MUST_START_WITH_0X)]
#[tokio::test]
async fn test_error_invalid_fields_set_1(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// User address validation
#[case::empty_user_address("userAddress", json!(""), constants::HEX_MUST_START_WITH_0X)]
#[case::short_user_address("userAddress", json!("0xfds"), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_user_address("userAddress", json!("0x1234567890123456789012345678901234567890123"), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_user_address("userAddress", json!("1234567890123456789012345678901234567890"), constants::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_user_address("userAddress", json!("0x123zzz5678901234567890123456789012345678"), constants::HEX_INVALID_CHARACTERS)]
#[case::user_address_with_invalid_hex_g("userAddress", json!("0x123456789012345678901234567890123456789g"), constants::HEX_INVALID_CHARACTERS)]
#[case::empty_string_user_address("userAddress", json!(""), constants::HEX_MUST_START_WITH_0X)]
// Handle contract pairs validation
#[case::empty_handle_contract_pairs("handleContractPairs", json!([]), constants::MUST_NOT_BE_EMPTY)]
#[tokio::test]
async fn test_error_invalid_fields_set_2(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Signature validation
#[case::short_signature("signature", json!("abcdef12"), constants::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::long_signature("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants::LENGTH_MUST_BE_130_CHARACTERS)]
#[case::signature_with_0x_prefix("signature", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants::HEX_MUST_NOT_START_WITH_0X)]
#[case::signature_with_invalid_hex_g("signature", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants::HEX_INVALID_STRING)]
#[case::empty_signature("signature", json!(""), constants::LENGTH_MUST_BE_130_CHARACTERS)]
// Public key validation
#[case::public_key_with_0x_prefix("publicKey", json!("0xabcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"), constants::HEX_MUST_NOT_START_WITH_0X)]
#[case::public_key_with_invalid_hex_g("publicKey", json!("abcdef123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890g"), constants::HEX_INVALID_STRING)]
#[case::empty_public_key("publicKey", json!(""), constants::MUST_NOT_BE_EMPTY)]
// Extra data validation
#[case::empty_extra_data("extraData", json!(""), constants::EXACT_MUST_BE_0X00)]
#[case::wrong_extra_data("extraData", json!("0x01"), constants::EXACT_MUST_BE_0X00)]
#[case::invalid_extra_data("extraData", json!("invalid"), constants::EXACT_MUST_BE_0X00)]
#[tokio::test]
async fn test_error_invalid_fields_set_3(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::short_handle("0xabcdef", constants::LENGTH_MUST_BE_64_CHARACTERS)]
#[case::long_handle(
    "0xabcdef1234567890123456789012345678901234567890123456789012345678901234567890",
    constants::LENGTH_MUST_BE_64_CHARACTERS
)]
#[case::handle_with_invalid_hex_g(
    "0xabcdefg123456789012345678901234567890123456789012345678901234567890",
    constants::HEX_INVALID_STRING
)]
#[case::handle_without_0x_prefix(
    "abcdef123456789012345678901234567890123456789012345678901234567890",
    constants::HEX_MUST_START_WITH_0X
)]
#[case::empty_handle("", constants::HEX_MUST_START_WITH_0X)]
#[tokio::test]
async fn test_error_invalid_nested_handle_fields(
    #[case] invalid_handle: &str,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Modify the nested handle field with the test case value
    base_payload["handleContractPairs"][0]["handle"] = json!(invalid_handle);

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        base_payload,
        |_| {}, // No additional modifications needed
        expect_invalid_field("handleContractPairs", expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::future_timestamp(constants::FUTURE_DATE, constants::TIMESTAMP_MUST_NOT_BE_IN_FUTURE)]
#[tokio::test]
async fn test_error_invalid_nested_handle_fields_2(
    #[case] future_date: &str,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let mut base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Modify the nested handle field with the test case value
    base_payload["requestValidity"]["startTimestamp"] = json!(future_date);

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
        base_payload,
        |_| {}, // No additional modifications needed
        expect_invalid_field("requestValidity", expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_contracts_chain_id("contractsChainId")]
#[case::missing_contract_addresses("contractAddresses")]
#[case::missing_user_address("userAddress")]
#[case::missing_handle_contract_pairs("handleContractPairs")]
#[case::missing_request_validity("requestValidity")]
#[case::missing_signature("signature")]
#[case::missing_public_key("publicKey")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
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
#[case::contract_and_user_addresses(["contractAddresses", "userAddress"], "contractAddresses")]
#[case::chain_id_and_handle_pairs(["contractsChainId", "handleContractPairs"], "handleContractPairs")]
#[tokio::test]
async fn test_error_missing_two_fields_reports_first_only(
    #[case] fields_to_remove: [&str; 2],
    #[case] expected_reported_field: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let base_payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    test_endpoint(
        &helpers::v1_user_decrypt_url(&setup),
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
        &helpers::v1_user_decrypt_url(&setup),
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

    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );

    // Make direct HTTP request to test the endpoint
    let response = reqwest::Client::new()
        .post(helpers::v1_user_decrypt_url(&setup))
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
