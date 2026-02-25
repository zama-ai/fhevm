mod common;

use crate::common::redundancy::{common_redundancy_cases, expand_targets, RedundancyCase};
use crate::common::utils::TestSetup;
use crate::common::validation_helper::{
    expect_invalid_field, expect_malformed_json, expect_missing_field, expect_success,
    test_endpoint, test_endpoint_raw_body, with_invalid_field,
};
use alloy::primitives::{Address, Bytes};
use rand::{rng, RngExt};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;

mod constants {
    pub const EXTRA_DATA: &str = "0x00";

    // Validation error messages (directly from source code)
    pub use fhevm_relayer::http::validation_messages::*;
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v1_input_proof_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/input-proof", setup.http_port)
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
async fn test_error_gateway_rejection() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);
    setup
        .fhevm_mock
        .on_input_proof_error(user_address, ciphertext_data, 1);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
        payload,
        |_| {},
        |res| {
            Box::pin(async move {
                assert_eq!(res.status(), 400);
                let response_text = res.text().await.unwrap();
                println!("{}", response_text);
                assert!(
                    response_text.contains("Proof Rejected")
                        && response_text.contains("Proof Rejected")
                );
            })
        },
    )
    .await;

    setup.shutdown().await;
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

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
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

    // Send multiple concurrent requests using test_endpoint
    let mut tasks = tokio::task::JoinSet::new();
    let number_of_requests = 10;

    let (payload, user_address, ciphertext_data) = helpers::create_input_proof_payload(&setup);
    setup.fhevm_mock.on_input_proof_success(
        user_address,
        ciphertext_data,
        number_of_requests,
        ethereum_rpc_mock::SubscriptionTarget::All,
    );

    for i in 1..=number_of_requests {
        let payload_clone = payload.clone();
        let url = helpers::v1_input_proof_url(&setup);
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
        result.expect("Task should complete");
    }

    setup.shutdown().await;
}

/// Listener redundancy for input proof with clear cases.
#[tokio::test]
#[cfg_attr(
    not(feature = "long-running-tests"),
    ignore = "Long-running test - run with --features long-running-tests"
)]
async fn test_listener_redundancy_input_proof_matrix() {
    let cases: Vec<RedundancyCase> = common_redundancy_cases();
    let mut setups: HashMap<usize, TestSetup> = HashMap::new();

    for case in cases {
        if let std::collections::hash_map::Entry::Vacant(e) = setups.entry(case.listener_count) {
            let setup = TestSetup::new_with_listeners(case.listener_count)
                .await
                .expect("Failed to create test setup");
            e.insert(setup);
        }
        let setup = setups
            .get(&case.listener_count)
            .expect("Missing test setup for listener count");

        println!("input-proof redundancy case: {}", case.name);

        let request_targets = expand_targets(case.requests, &case.targets_per_event);

        for target in request_targets {
            let (payload, user_address, ciphertext_data) =
                helpers::create_input_proof_payload(setup);
            // Register one success pattern per request with its target
            setup.fhevm_mock.on_input_proof_success(
                user_address,
                ciphertext_data.clone(),
                1,
                target.clone(),
            );

            test_endpoint(
                &helpers::v1_input_proof_url(setup),
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
// Chain ID validation
#[case::empty_chain_id("contractChainId", json!(""), constants::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_decimal("contractChainId", json!("abc123"), constants::NUMBER_DECIMAL_OR_HEX)]
#[case::invalid_chain_id_hex("contractChainId", json!("0xzzz"), constants::NUMBER_DECIMAL_OR_HEX)]
// Contract address validation
#[case::empty_contract_address("contractAddress", json!(""), constants::HEX_MUST_START_WITH_0X)]
#[case::short_contract_address("contractAddress", json!("0xfds"), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::long_contract_address("contractAddress", json!("0x1234567890123456789012345678901234567890123"), constants::LENGTH_MUST_BE_42_CHARACTERS)]
#[case::missing_0x_contract_address("contractAddress", json!("1234567890123456789012345678901234567890"), constants::HEX_MUST_START_WITH_0X)]
#[case::invalid_hex_contract_address("contractAddress", json!("0x123zzz5678901234567890123456789012345678"), constants::HEX_INVALID_CHARACTERS)]
#[case::contract_address_with_invalid_hex_g("contractAddress", json!("0x123456789012345678901234567890123456789g"), constants::HEX_INVALID_CHARACTERS)]
#[tokio::test]
async fn test_error_invalid_fields_set_1(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
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
#[tokio::test]
async fn test_error_invalid_fields_set_2(
    #[case] field: &str,
    #[case] invalid_value: serde_json::Value,
    #[case] expected_issue: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
// Ciphertext validation
#[case::empty_ciphertext("ciphertextWithInputVerification", json!(""), constants::MUST_NOT_BE_EMPTY)]
#[case::invalid_hex_ciphertext("ciphertextWithInputVerification", json!("abcdefabcdefs"), constants::HEX_INVALID_STRING)]
#[case::odd_length_ciphertext("ciphertextWithInputVerification", json!("abcdef1"), constants::HEX_INVALID_STRING)]
#[case::ciphertext_with_invalid_hex_g("ciphertextWithInputVerification", json!("abcdefg"), constants::HEX_INVALID_STRING)]
#[case::ciphertext_with_0x_prefix("ciphertextWithInputVerification", json!("0xabcdef"), constants::HEX_MUST_NOT_START_WITH_0X)]
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
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
        base_payload,
        with_invalid_field(field, invalid_value),
        expect_invalid_field(field, expected_issue),
    )
    .await;

    setup.shutdown().await;
}

#[rstest]
#[case::missing_contract_chain_id("contractChainId")]
#[case::missing_contract_address("contractAddress")]
#[case::missing_user_address("userAddress")]
#[case::missing_ciphertext("ciphertextWithInputVerification")]
#[case::missing_extra_data("extraData")]
#[tokio::test]
async fn test_error_missing_fields(#[case] field: &str) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
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
#[case::contract_and_user_address(["contractAddress", "userAddress"], "contractAddress")]
#[case::chain_id_and_ciphertext(["contractChainId", "ciphertextWithInputVerification"], "contractChainId")]
#[tokio::test]
async fn test_error_missing_two_fields_reports_first_only(
    #[case] fields_to_remove: [&str; 2],
    #[case] expected_reported_field: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
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
#[case::chain_contract_user(["contractChainId", "contractAddress", "userAddress"], "contractChainId")]
#[case::contract_user_ciphertext(["contractAddress", "userAddress", "ciphertextWithInputVerification"], "contractAddress")]
#[case::chain_ciphertext_extra(["contractChainId", "ciphertextWithInputVerification", "extraData"], "contractChainId")]
#[tokio::test]
async fn test_error_missing_three_fields_reports_first_only(
    #[case] fields_to_remove: [&str; 3],
    #[case] expected_reported_field: &str,
) {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let (base_payload, _, _) = helpers::create_input_proof_payload(&setup);

    test_endpoint(
        &helpers::v1_input_proof_url(&setup),
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
        &helpers::v1_input_proof_url(&setup),
        malformed_json,
        expect_malformed_json(),
    )
    .await;

    setup.shutdown().await;
}
