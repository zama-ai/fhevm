//! Health endpoint tests for the fhevm-relayer service.
//!
//! Tests three endpoints: `/liveness`, `/healthz`, and `/version`.
//! Focuses on happy path scenarios with all dependencies healthy.

mod common;

use crate::common::utils::TestSetup;
use rstest::rstest;
use serde_json::json;

mod helpers {
    use super::*;

    pub fn liveness_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/liveness", setup.http_port)
    }

    pub fn health_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/healthz", setup.http_port)
    }

    pub fn version_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/version", setup.http_port)
    }
}

/// Test liveness endpoint returns HTTP 200 with status "alive".
#[rstest]
#[tokio::test]
async fn test_liveness_endpoint_success() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::liveness_url(&setup);

    let response = reqwest::get(&url).await.unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let expected = json!({
        "status": "alive"
    });

    assert_eq!(body, expected);

    setup.shutdown().await;
}

/// Test version endpoint returns build metadata with correct structure.
#[rstest]
#[tokio::test]
async fn test_version_endpoint_success() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::version_url(&setup);

    let response = reqwest::get(&url).await.unwrap();

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.unwrap();

    // Verify required fields and types
    assert!(body.get("name").is_some());
    assert!(body.get("version").is_some());
    assert!(body.get("build").is_some());
    assert!(body["name"].is_string());
    assert!(body["version"].is_string());
    assert!(body["build"].is_string());

    // Verify build format: "hash-clean" or "hash-dirty"
    let build = body["build"].as_str().unwrap();
    assert!(build.contains('-'));
    assert!(build.ends_with("clean") || build.ends_with("dirty"));

    setup.shutdown().await;
}

/// Test health endpoint returns HTTP 200 when all dependencies are healthy.
/// Checks gateway_http, gateway_ws_{0,1,2}, and database dependencies.
#[rstest]
#[tokio::test]
async fn test_health_endpoint_all_healthy() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");
    let url = helpers::health_url(&setup);

    let response = reqwest::get(&url).await.unwrap();

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "healthy");

    let dependencies = body["dependencies"].as_object().unwrap();

    // All dependencies should report "ok"
    for (_name, status) in dependencies {
        assert_eq!(status.as_str().unwrap(), "ok");
    }

    // Verify expected dependencies are present
    assert!(dependencies.contains_key("gateway_http"));
    // Check for multiple listener instances (default is 3 from config)
    assert!(dependencies.contains_key("gateway_listener_0"));
    assert!(dependencies.contains_key("gateway_listener_1"));
    assert!(dependencies.contains_key("gateway_listener_2"));
    assert!(dependencies.contains_key("database"));

    setup.shutdown().await;
}

// Note: RPC failure tests would require mock health checker infrastructure
// to inject failures. Current TestSetup uses real health checkers with
// mock RPC servers that always succeed, limiting failure scenario testing.
