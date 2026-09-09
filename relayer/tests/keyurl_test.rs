//! KeyURL endpoint tests for the fhevm-relayer service.
//!
//! Tests the `/v2/keyurl` endpoint.
//! Focuses on successful scenarios that are reliably testable in the integration environment.

mod common;

use crate::common::utils::{
    test_keyurl_expected_data_id, test_keyurl_expected_url, TestSetup, TEST_CONFIG_PATH,
    TEST_KEYURL_CRS_ID, TEST_KEYURL_KEY_ID,
};
use alloy::primitives::U256;
use rstest::rstest;
use serde_json::Value;

/// Static `/v2/keyurl` values wired into the `keyurl.source: config` test config.
const CONFIG_MODE_KEY_DATA_ID: &str =
    "0x0400000000000000000000000000000000000000000000000000000000000003";
const CONFIG_MODE_KEY_URL: &str = "http://minio:9000/kms-public/PUB-p1/PublicKey/0400000000000000000000000000000000000000000000000000000000000003";
const CONFIG_MODE_CRS_DATA_ID: &str =
    "0x0400000000000000000000000000000000000000000000000000000000000004";
const CONFIG_MODE_CRS_URL: &str = "http://minio:9000/kms-public/PUB-p1/CRS/0400000000000000000000000000000000000000000000000000000000000004";

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

        // --- Chain-sourced values (served from the host-chain poller) ---

        // dataId carries the real on-chain getActiveKeyId / getActiveCrsId, as 0x-prefixed hex.
        assert_eq!(
            fhe_public_key["dataId"].as_str().unwrap(),
            test_keyurl_expected_data_id(U256::from(TEST_KEYURL_KEY_ID)),
            "fhePublicKey.dataId should equal on-chain getActiveKeyId"
        );
        assert_eq!(
            crs_2048["dataId"].as_str().unwrap(),
            test_keyurl_expected_data_id(U256::from(TEST_KEYURL_CRS_ID)),
            "crs.2048.dataId should equal on-chain getActiveCrsId"
        );

        // urls are reconstructed from the KMS context node's storageUrl/storagePrefix and the
        // hex-encoded id: {storageUrl}/{storagePrefix}/{PublicKey|CRS}/{id_hex}.
        assert_eq!(
            fhe_public_key["urls"][0].as_str().unwrap(),
            test_keyurl_expected_url("PublicKey", U256::from(TEST_KEYURL_KEY_ID)),
            "fhePublicKey.urls[0] should be the reconstructed object URL"
        );
        assert_eq!(
            crs_2048["urls"][0].as_str().unwrap(),
            test_keyurl_expected_url("CRS", U256::from(TEST_KEYURL_CRS_ID)),
            "crs.2048.urls[0] should be the reconstructed object URL"
        );

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

/// `keyurl.source: config` against a v0.13-style deployment: the host mock reverts the
/// KMS-context getters (`register_missing_kms_context_getters`), so a relayer that still polled
/// could not pass its startup gate — a served response proves no poller runs.
#[rstest]
#[tokio::test]
async fn test_keyurl_v2_served_from_static_config() {
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
    let config_path = write_config_source_config(&temp_dir);

    let setup = TestSetup::new_with_config_path(Some(config_path))
        .await
        .expect("Relayer should start in keyurl.source: config mode without any chain KMS context");

    let response = reqwest::get(&helpers::keyurl_v2_url(&setup)).await.unwrap();
    assert_eq!(response.status(), 200, "keyurl endpoint should return 200");
    let body: Value = response.json().await.unwrap();

    let fhe_public_key = &body["response"]["fheKeyInfo"][0]["fhePublicKey"];
    assert_eq!(
        fhe_public_key["dataId"].as_str().unwrap(),
        CONFIG_MODE_KEY_DATA_ID,
        "fhePublicKey.dataId should be the configured value"
    );
    assert_eq!(
        fhe_public_key["urls"].as_array().unwrap(),
        &vec![Value::String(CONFIG_MODE_KEY_URL.to_string())],
        "fhePublicKey.urls should be the configured values"
    );

    let crs_2048 = &body["response"]["crs"]["2048"];
    assert_eq!(
        crs_2048["dataId"].as_str().unwrap(),
        CONFIG_MODE_CRS_DATA_ID,
        "crs.2048.dataId should be the configured value"
    );
    assert_eq!(
        crs_2048["urls"].as_array().unwrap(),
        &vec![Value::String(CONFIG_MODE_CRS_URL.to_string())],
        "crs.2048.urls should be the configured values"
    );

    setup.shutdown().await;
}

/// Copy the integration test config, replacing its `keyurl` block with a `source: config` one.
fn write_config_source_config(temp_dir: &tempfile::TempDir) -> std::path::PathBuf {
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(TEST_CONFIG_PATH).expect("read test config"))
            .expect("parse test config");

    let keyurl = serde_yaml::from_str::<serde_yaml::Value>(&format!(
        "source: config
fhe_public_key:
  data_id: \"{CONFIG_MODE_KEY_DATA_ID}\"
  urls:
    - \"{CONFIG_MODE_KEY_URL}\"
crs:
  data_id: \"{CONFIG_MODE_CRS_DATA_ID}\"
  urls:
    - \"{CONFIG_MODE_CRS_URL}\"
"
    ))
    .expect("parse static keyurl block");
    config["keyurl"] = keyurl;

    let config_path = temp_dir.path().join("keyurl_source_config.yaml");
    std::fs::write(
        &config_path,
        serde_yaml::to_string(&config).expect("serialize config"),
    )
    .expect("write config");
    config_path
}
