use fhevm_relayer::http::{validation_messages, ErrorLabel, ErrorResponse};
use serde_json::Value;

const TESTS_MAX_RETRIES: i16 = 20;

#[allow(dead_code)]
pub async fn test_endpoint(
    url: &str,
    base_payload: Value,
    modify: impl FnOnce(&mut Value),
    verify: impl FnOnce(
        reqwest::Response,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>,
) {
    let mut payload = base_payload.clone();
    modify(&mut payload);

    let client = reqwest::Client::new();

    // Retry configuration
    let retry_delay = std::time::Duration::from_millis(250);
    let mut last_error = None;

    for attempts in 0..TESTS_MAX_RETRIES {
        let result = client
            .post(url)
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .json(&payload)
            .send()
            .await;

        match result {
            Ok(res) => {
                verify(res).await;
                return;
            }
            Err(e) => {
                if e.is_connect() || e.is_request() {
                    // Log to CI so you know it's waiting
                    println!(
                        "Attempt {}/{} failed to connect to {}. Retrying...",
                        attempts, TESTS_MAX_RETRIES, url
                    );
                    last_error = Some(e);
                    tokio::time::sleep(retry_delay).await;
                    continue;
                }

                panic!("Request failed with non-recoverable error: {:?}", e);
            }
        }
    }
    panic!(
        "Test failed: Could not connect to {} after {} attempts. Is the server running? Last error: {:?}",
        url, TESTS_MAX_RETRIES, last_error
        );
}

// Verify functions
#[allow(dead_code)]
pub fn expect_success(
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    |res| {
        Box::pin(async move {
            let status = res.status();
            let text = res.text().await.unwrap();
            assert_eq!(status, 200, "Response: {}", text);
        })
    }
}

#[allow(dead_code)]
pub fn expect_missing_field(
    field: &str,
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    let field = field.to_string();
    move |res| {
        Box::pin(async move {
            assert_eq!(res.status(), 400);
            let error: ErrorResponse = res.json().await.unwrap();
            assert_eq!(error.error.label, ErrorLabel::MissingFields);
            let details = error.error.clone().details.unwrap();
            assert!(
                details.iter().any(|d| d.field == field),
                "Expected missing field '{}', got: {:?}",
                field,
                details
            );
            // Also assert that the issue message is correct for missing fields
            let field_detail = details.iter().find(|d| d.field == field).unwrap();
            assert_eq!(
                field_detail.issue,
                validation_messages::GENERIC_REQUIRED_BUT_MISSING,
                "Expected missing field '{}' to have issue '{}', got: '{}'",
                field,
                validation_messages::GENERIC_REQUIRED_BUT_MISSING,
                field_detail.issue
            );
        })
    }
}

#[allow(dead_code)]
pub fn expect_validation_issues(
    issues: &[(&str, &str)],
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    let issues: Vec<(String, String)> = issues
        .iter()
        .map(|(f, i)| (f.to_string(), i.to_string()))
        .collect();
    move |res| {
        Box::pin(async move {
            assert_eq!(res.status(), 400);
            let error: ErrorResponse = res.json().await.unwrap();
            assert_eq!(error.error.label, ErrorLabel::ValidationFailed);
            let details = error.error.details.unwrap();

            for (field, issue_contains) in &issues {
                assert!(
                    details
                        .iter()
                        .any(|d| d.field == *field && d.issue.contains(issue_contains)),
                    "Expected field '{}' with issue containing '{}', got: {:?}",
                    field,
                    issue_contains,
                    details
                );
            }
        })
    }
}

// Helper to create invalid field modifier
#[allow(dead_code)]
pub fn with_invalid_field(
    field: &str,
    invalid_value: serde_json::Value,
) -> impl Fn(&mut serde_json::Value) {
    let field = field.to_string();
    move |p: &mut serde_json::Value| {
        p[&field] = invalid_value.clone();
    }
}

// Simplified expectation for single field validation error
#[allow(dead_code)]
pub fn expect_invalid_field(
    field: &str,
    issue_contains: &str,
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    expect_validation_issues(&[(field, issue_contains)])
}

// Test endpoint with raw text instead of JSON for malformed JSON testing
#[allow(dead_code)]
pub async fn test_endpoint_raw_body(
    url: &str,
    raw_body: &str,
    verify: impl FnOnce(
        reqwest::Response,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>,
) {
    let res = reqwest::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .body(raw_body.to_string())
        .send()
        .await
        .unwrap();

    verify(res).await;
}

// Verify function for malformed JSON errors
#[allow(dead_code)]
pub fn expect_malformed_json(
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    |res| {
        Box::pin(async move {
            assert_eq!(res.status(), 400);
            let error: ErrorResponse = res.json().await.unwrap();
            assert_eq!(error.error.label, ErrorLabel::MalformedJson);
        })
    }
}

/// Verify that a V2 error response has the correct structure with status and request_id fields
#[allow(dead_code)]
pub fn expect_v2_error_response(
    expected_status_code: u16,
    expected_error_label: &str,
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    let expected_error_label = expected_error_label.to_string();
    move |res| {
        Box::pin(async move {
            let status_code = res.status().as_u16();
            assert_eq!(
                status_code, expected_status_code,
                "Expected status code {}, got {}",
                expected_status_code, status_code
            );

            let body: serde_json::Value = res.json().await.expect("Failed to parse response body");

            // Verify status field exists and equals "failed"
            assert_eq!(
                body.get("status").and_then(|v| v.as_str()),
                Some("failed"),
                "V2 error response must have status='failed', got: {:?}",
                body
            );

            // Verify request_id field exists and is a non-empty string
            let request_id = body
                .get("request_id")
                .expect("V2 error response must have request_id field");
            assert!(
                request_id.is_string(),
                "request_id must be a string, got: {:?}",
                request_id
            );
            let request_id_str = request_id.as_str().unwrap();
            assert!(!request_id_str.is_empty(), "request_id must not be empty");

            // Verify error field exists and has the expected label
            let error = body
                .get("error")
                .expect("V2 error response must have error field");
            assert_eq!(
                error.get("label").and_then(|v| v.as_str()),
                Some(expected_error_label.as_str()),
                "Expected error label '{}', got: {:?}",
                expected_error_label,
                error
            );
        })
    }
}

/// Verify V2 validation error with specific field details
#[allow(dead_code)]
pub fn expect_v2_validation_error(
    field: &str,
    issue_contains: &str,
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    let field = field.to_string();
    let issue_contains = issue_contains.to_string();
    move |res| {
        Box::pin(async move {
            assert_eq!(res.status(), 400);

            let body: serde_json::Value = res.json().await.expect("Failed to parse response body");

            // Verify V2 structure
            assert_eq!(
                body.get("status").and_then(|v| v.as_str()),
                Some("failed"),
                "V2 error response must have status='failed'"
            );
            assert!(
                body.get("request_id")
                    .and_then(|v| v.as_str())
                    .map(|s| !s.is_empty())
                    .unwrap_or(false),
                "V2 error response must have non-empty request_id"
            );

            // Verify error details
            let error = body.get("error").expect("Must have error field");
            let label = error.get("label").and_then(|v| v.as_str()).unwrap_or("");
            assert!(
                label == "validation_failed" || label == "missing_fields",
                "Expected validation_failed or missing_fields label, got: {}",
                label
            );

            let details = error.get("details").and_then(|v| v.as_array());
            assert!(details.is_some(), "Error must have details array");
            let details = details.unwrap();
            assert!(
                details.iter().any(|d| {
                    let f = d.get("field").and_then(|v| v.as_str()).unwrap_or("");
                    let i = d.get("issue").and_then(|v| v.as_str()).unwrap_or("");
                    f == field && i.contains(&issue_contains)
                }),
                "Expected field '{}' with issue containing '{}', got: {:?}",
                field,
                issue_contains,
                details
            );
        })
    }
}

/// Verify V2 malformed JSON error response
#[allow(dead_code)]
pub fn expect_v2_malformed_json(
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    expect_v2_error_response(400, "malformed_json")
}

/// Verify V2 missing field error response
#[allow(dead_code)]
pub fn expect_v2_missing_field(
    field: &str,
) -> impl FnOnce(reqwest::Response) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
{
    let field = field.to_string();
    move |res| {
        Box::pin(async move {
            assert_eq!(res.status(), 400);

            let body: serde_json::Value = res.json().await.expect("Failed to parse response body");

            // Verify V2 structure
            assert_eq!(
                body.get("status").and_then(|v| v.as_str()),
                Some("failed"),
                "V2 error response must have status='failed'"
            );
            assert!(
                body.get("request_id")
                    .and_then(|v| v.as_str())
                    .map(|s| !s.is_empty())
                    .unwrap_or(false),
                "V2 error response must have non-empty request_id"
            );

            // Verify error details
            let error = body.get("error").expect("Must have error field");
            let label = error.get("label").and_then(|v| v.as_str()).unwrap_or("");
            assert_eq!(
                label, "missing_fields",
                "Expected missing_fields label, got: {}",
                label
            );

            let details = error.get("details").and_then(|v| v.as_array());
            assert!(details.is_some(), "Error must have details array");
            let details = details.unwrap();
            assert!(
                details.iter().any(|d| {
                    let f = d.get("field").and_then(|v| v.as_str()).unwrap_or("");
                    let i = d.get("issue").and_then(|v| v.as_str()).unwrap_or("");
                    f == field && i.contains(validation_messages::GENERIC_REQUIRED_BUT_MISSING)
                }),
                "Expected field '{}' with issue containing '{}', got: {:?}",
                field,
                validation_messages::GENERIC_REQUIRED_BUT_MISSING,
                details
            );
        })
    }
}
