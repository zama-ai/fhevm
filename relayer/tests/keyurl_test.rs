//! KeyURL endpoint tests for the fhevm-relayer service.
//!
//! Tests the `/v2/keyurl` endpoint.
//! Focuses on successful scenarios that are reliably testable in the integration environment.

mod common;

use crate::common::utils::TestSetup;
use rstest::rstest;
use serde_json::Value;

mod helpers {
    use super::*;

    pub fn keyurl_v2_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v2/keyurl", setup.http_port)
    }

    /// Validate the keyurl v2 response structure (camelCase)
    pub async fn validate_keyurl_v2_response(response: reqwest::Response) -> Value {
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
        assert!(body.get("response").is_some(), "Missing 'response' field");
        let response = &body["response"];

        assert!(body.get("status").is_some(), "Missing 'status' field");
        assert!(body["status"].is_string(), "'status' should be a string");

        // Check fheKeyInfo array
        assert!(
            response.get("fheKeyInfo").is_some(),
            "Missing 'fheKeyInfo' field"
        );
        assert!(
            response["fheKeyInfo"].is_array(),
            "'fheKeyInfo' should be an array"
        );
        let fhe_key_info = response["fheKeyInfo"].as_array().unwrap();
        assert!(
            !fhe_key_info.is_empty(),
            "'fheKeyInfo' array should not be empty"
        );

        // Check first fheKeyInfo entry
        let first_key_info = &fhe_key_info[0];
        assert!(
            first_key_info.get("fhePublicKey").is_some(),
            "Missing 'fhePublicKey' field"
        );

        let fhe_public_key = &first_key_info["fhePublicKey"];
        assert!(
            fhe_public_key.get("dataId").is_some(),
            "Missing 'dataId' in fhePublicKey"
        );
        assert!(
            fhe_public_key.get("urls").is_some(),
            "Missing 'urls' in fhePublicKey"
        );
        assert!(
            fhe_public_key["dataId"].is_string(),
            "'dataId' should be a string"
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
            crs_2048.get("dataId").is_some(),
            "Missing 'dataId' in crs.2048"
        );
        assert!(crs_2048.get("urls").is_some(), "Missing 'urls' in crs.2048");
        assert!(
            crs_2048["dataId"].is_string(),
            "'dataId' should be a string"
        );
        assert!(crs_2048["urls"].is_array(), "'urls' should be an array");

        body
    }
}

/// Test successful keyurl response for v2 endpoint
#[rstest]
#[tokio::test]
async fn test_keyurl_endpoints_success() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    let v2_url = helpers::keyurl_v2_url(&setup);
    let v2_response = reqwest::get(&v2_url).await.unwrap();
    let _v2_body = helpers::validate_keyurl_v2_response(v2_response).await;

    setup.shutdown().await;
}
