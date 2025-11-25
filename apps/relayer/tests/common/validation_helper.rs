use fhevm_relayer::http::utils::{validation_messages, ErrorLabel, ErrorResponse};
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
