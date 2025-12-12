//! KeyURL endpoint tests for the fhevm-relayer service.
//!
//! Tests both `/v1/keyurl` and `/v2/keyurl` endpoints using parameterized tests.
//! Focuses on successful scenarios that are reliably testable in the integration environment.
//!
//! Note: The 503 "Service Unavailable" scenario (when keyurl is not configured) is difficult
//! to reproduce in integration tests due to the service initialization process. This edge case
//! would require unit testing with mocked orchestrator components.

mod common;

use crate::common::utils::TestSetup;
use rstest::rstest;
use serde_json::Value;

mod helpers {
    use super::*;

    pub fn keyurl_v1_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/keyurl", setup.http_port)
    }

    pub fn keyurl_v2_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/keyurl", setup.http_port)
    }

    /// Validate the keyurl v1 response structure (snake_case)
    pub async fn validate_keyurl_v1_response(response: reqwest::Response) -> Value {
        validate_keyurl_response_internal(response, false).await
    }

    /// Validate the keyurl v2 response structure (camelCase)
    pub async fn validate_keyurl_v2_response(response: reqwest::Response) -> Value {
        validate_keyurl_response_internal(response, true).await
    }

    /// Internal validation function that handles both formats
    async fn validate_keyurl_response_internal(response: reqwest::Response, is_v2: bool) -> Value {
        // Check status code
        assert_eq!(response.status(), 200, "keyurl endpoint should return 200");

        // Check content type header
        let content_type = response.headers().get("content-type");
        assert!(
            content_type.is_some(),
            "keyurl should have content-type header"
        );

        let content_type_str = content_type.unwrap().to_str().unwrap();
        assert!(
            content_type_str.contains("application/json"),
            "keyurl should return JSON content-type, got: {}",
            content_type_str
        );

        // Parse JSON response
        let body: Value = response.json().await.unwrap();

        // Validate JSON structure
        // Check top-level structure
        assert!(body.get("response").is_some(), "Missing 'response' field");
        let response = &body["response"];

        // Handle v2-specific status field
        if is_v2 {
            assert!(body.get("status").is_some(), "Missing 'status' field in v2");
            assert!(
                body["status"].is_string(),
                "'status' should be a string in v2"
            );
        }

        // Choose field names based on version
        let (fhe_key_info_field, fhe_public_key_field, data_id_field) = if is_v2 {
            ("fheKeyInfo", "fhePublicKey", "dataId")
        } else {
            ("fhe_key_info", "fhe_public_key", "data_id")
        };

        // Check fhe_key_info array
        assert!(
            response.get(fhe_key_info_field).is_some(),
            "Missing '{}' field",
            fhe_key_info_field
        );
        assert!(
            response[fhe_key_info_field].is_array(),
            "'{}' should be an array",
            fhe_key_info_field
        );
        let fhe_key_info = response[fhe_key_info_field].as_array().unwrap();
        assert!(
            !fhe_key_info.is_empty(),
            "'{}' array should not be empty",
            fhe_key_info_field
        );

        // Check first fhe_key_info entry
        let first_key_info = &fhe_key_info[0];
        assert!(
            first_key_info.get(fhe_public_key_field).is_some(),
            "Missing '{}' field",
            fhe_public_key_field
        );

        let fhe_public_key = &first_key_info[fhe_public_key_field];
        assert!(
            fhe_public_key.get(data_id_field).is_some(),
            "Missing '{}' in {}",
            data_id_field,
            fhe_public_key_field
        );
        assert!(
            fhe_public_key.get("urls").is_some(),
            "Missing 'urls' in {}",
            fhe_public_key_field
        );
        assert!(
            fhe_public_key[data_id_field].is_string(),
            "'{}' should be a string",
            data_id_field
        );
        assert!(
            fhe_public_key["urls"].is_array(),
            "'urls' should be an array"
        );

        // Check crs object
        assert!(response.get("crs").is_some(), "Missing 'crs' field");
        assert!(response["crs"].is_object(), "'crs' should be an object");
        let crs = response["crs"].as_object().unwrap();
        assert!(!crs.is_empty(), "'crs' object should not be empty");

        // Check that crs contains "2048" key with proper structure
        assert!(crs.contains_key("2048"), "'crs' should contain '2048' key");
        let crs_2048 = &crs["2048"];
        assert!(
            crs_2048.get(data_id_field).is_some(),
            "Missing '{}' in crs.2048",
            data_id_field
        );
        assert!(crs_2048.get("urls").is_some(), "Missing 'urls' in crs.2048");
        assert!(
            crs_2048[data_id_field].is_string(),
            "'{}' should be a string",
            data_id_field
        );
        assert!(crs_2048["urls"].is_array(), "'urls' should be an array");

        body
    }
}

/// Test successful keyurl response for both v1 and v2 endpoints
#[rstest]
#[tokio::test]
async fn test_keyurl_endpoints_success() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let v1_url = helpers::keyurl_v1_url(&setup);
    let v2_url = helpers::keyurl_v2_url(&setup);

    // Get responses and validate them with their respective formats
    let v1_response = reqwest::get(&v1_url).await.unwrap();
    let v2_response = reqwest::get(&v2_url).await.unwrap();

    let _v1_body = helpers::validate_keyurl_v1_response(v1_response).await;
    let _v2_body = helpers::validate_keyurl_v2_response(v2_response).await;

    // Both endpoints validated successfully with their respective schemas
    // v1 uses snake_case, v2 uses camelCase + includes status field

    setup.shutdown().await;
}
