//! Admin endpoint tests for runtime configuration updates.
//!
//! Tests the /admin/config endpoint for dynamic rate limiting and other
//! configuration updates without service restart.

mod common;

use crate::common::utils::TestSetup;
use rstest::rstest;
use serde_json::json;

mod helpers {
    use super::*;

    pub fn admin_config_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/admin/config", setup.http_port)
    }
}

/// Test admin endpoint returns 403 when disabled (default config)
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_disabled_returns_403() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 50
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 403); // Forbidden
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["error"].as_str().unwrap().contains("not enabled"));

    setup.shutdown().await;
}

/// Test admin endpoint successfully updates TPS when enabled
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_enabled_valid_input_proof_tps() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 50
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200); // OK
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "input_proof_throttler_tps");
    assert_eq!(body["value"], 50);
    assert!(body["message"].as_str().unwrap().contains("successfully"));

    setup.shutdown().await;
}

#[rstest]
#[tokio::test]
async fn test_admin_endpoint_enabled_valid_user_decrypt_tps() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "user_decrypt_throttler_tps",
            "value": 50
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200); // OK
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "user_decrypt_throttler_tps");
    assert_eq!(body["value"], 50);
    assert!(body["message"].as_str().unwrap().contains("successfully"));

    setup.shutdown().await;
}

#[rstest]
#[tokio::test]
async fn test_admin_endpoint_enabled_valid_public_decrypt_tps() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "public_decrypt_throttler_tps",
            "value": 50
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200); // OK
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "public_decrypt_throttler_tps");
    assert_eq!(body["value"], 50);
    assert!(body["message"].as_str().unwrap().contains("successfully"));

    setup.shutdown().await;
}

/// Test admin endpoint rejects TPS value of 0
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_invalid_tps_zero() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 0
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400); // Bad Request
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["error"].as_str().unwrap().contains("greater than 0"));

    setup.shutdown().await;
}

/// Test admin endpoint rejects TPS value over 1000
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_invalid_tps_over_1000() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 1001
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400); // Bad Request
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("less than or equal to 1000"));

    setup.shutdown().await;
}

/// Test admin endpoint rejects unknown configuration parameter
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_unknown_parameter() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&json!({
            "name": "unknown_parameter",
            "value": 100
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 400); // Bad Request
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["error"]
        .as_str()
        .unwrap()
        .contains("Unknown configuration parameter"));

    setup.shutdown().await;
}

/// Test admin endpoint accepts valid TPS range boundaries
#[rstest]
#[tokio::test]
async fn test_admin_endpoint_valid_tps_boundaries() {
    let setup = TestSetup::new_with_admin_endpoint()
        .await
        .expect("Failed to create test setup with admin endpoint");
    let url = helpers::admin_config_url(&setup);

    let client = reqwest::Client::new();

    // Test minimum valid value (1)
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 1
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Test maximum valid value (1000)
    let response = client
        .post(&url)
        .json(&json!({
            "name": "input_proof_throttler_tps",
            "value": 1000
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    setup.shutdown().await;
}
