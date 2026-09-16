//! End-to-end tests of the proxy against a stub Connector endpoint.

mod common;

use alloy::transports::http::reqwest::StatusCode;
use common::{API_KEY, MAX_BODY_BYTES, StubEndpoint, TestProxy, free_addr};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
    VersionResponse,
};
use std::time::Duration;

#[tokio::test]
async fn test_forwards_authenticated_requests_untouched() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .get(t.url(VERSION_ROUTE))
        .bearer_auth(API_KEY)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await.unwrap(),
        VersionResponse::default()
    );

    // Deliberately non-canonical JSON: the proxy must forward it byte-for-byte.
    let body = r#"{ "ctHandles":["0x01"] ,  "extraData":"0x00"}"#;
    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("content-type", "application/json")
        .header("x-forwarded-for", "9.9.9.9")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let echo: serde_json::Value = response.json().await.unwrap();
    assert_eq!(echo["body"], body);
    assert_eq!(echo["authorization"], serde_json::Value::Null);
    assert_eq!(echo["x-forwarded-for"], "127.0.0.1");
    assert_eq!(echo["x-forwarded-proto"], "https");

    let response = t
        .client
        .post(t.url(USER_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rejects_unauthenticated_requests() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    for request in [
        t.client.post(t.url(PUBLIC_DECRYPTION_ROUTE)).body("{}"),
        t.client
            .post(t.url(PUBLIC_DECRYPTION_ROUTE))
            .bearer_auth("wrong")
            .body("{}"),
        t.client
            .post(t.url(USER_DECRYPTION_ROUTE))
            .header("authorization", format!("Basic {API_KEY}"))
            .body("{}"),
        t.client.get(t.url(VERSION_ROUTE)),
    ] {
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers()["www-authenticate"].to_str().unwrap(),
            "Bearer"
        );
        assert_eq!(
            response.headers()["content-type"].to_str().unwrap(),
            "application/json"
        );
        let error: ErrorResponse = response.json().await.unwrap();
        assert_eq!(error.code, ErrorCode::SenderAuthenticationFailed);
        assert!(!error.retryable);
        assert_eq!(error.decryption_id, None);
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_default_deny_routing() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    for (request, expected, expected_allow) in [
        (t.client.get(t.url("/")), StatusCode::NOT_FOUND, None),
        (t.client.get(t.url("/healthz")), StatusCode::NOT_FOUND, None),
        (
            t.client.post(t.url("/v2/public-decrypt")),
            StatusCode::NOT_FOUND,
            None,
        ),
        (
            t.client.post(t.url("/v1/public-decrypt/extra")),
            StatusCode::NOT_FOUND,
            None,
        ),
        (
            t.client.get(t.url(PUBLIC_DECRYPTION_ROUTE)),
            StatusCode::METHOD_NOT_ALLOWED,
            Some("POST"),
        ),
        (
            t.client.post(t.url(VERSION_ROUTE)),
            StatusCode::METHOD_NOT_ALLOWED,
            Some("GET"),
        ),
    ] {
        // Even with valid credentials.
        let response = request.bearer_auth(API_KEY).send().await.unwrap();
        assert_eq!(response.status(), expected);
        assert_eq!(
            response
                .headers()
                .get("allow")
                .and_then(|v| v.to_str().ok()),
            expected_allow
        );
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_rejects_oversized_bodies() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body(vec![b' '; MAX_BODY_BYTES + 1])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Malformed);
    assert!(!error.retryable);

    // Chunked bodies carry no `Content-Length`, the cap is enforced while streaming.
    let status = send_chunked(&t, &[MAX_BODY_BYTES, 1]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let status = send_chunked(&t, &[MAX_BODY_BYTES / 2, MAX_BODY_BYTES / 2]).await;
    assert_eq!(status, StatusCode::OK);

    // Exactly at the limit is fine.
    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body(vec![b' '; MAX_BODY_BYTES])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_passes_endpoint_errors_through() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("x-stub-overloaded", "1")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.headers()["retry-after"].to_str().unwrap(), "2");
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Overloaded);
    assert!(error.retryable);
}

#[tokio::test]
async fn test_load_balances_across_endpoints() {
    let endpoints = vec![StubEndpoint::start().await, StubEndpoint::start().await];
    let t = TestProxy::start(endpoints).await;

    for _ in 0..10 {
        let response = t
            .client
            .post(t.url(PUBLIC_DECRYPTION_ROUTE))
            .bearer_auth(API_KEY)
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
    assert_eq!(t.endpoints[0].hits() + t.endpoints[1].hits(), 10);
    assert!(t.endpoints[0].hits() > 0 && t.endpoints[1].hits() > 0);
}

#[tokio::test]
async fn test_upstream_timeout() {
    let endpoint = StubEndpoint::start().await;
    let t = TestProxy::start_with_config(
        vec![endpoint.addr.to_string().parse().unwrap()],
        vec![endpoint],
        Duration::from_millis(100),
    )
    .await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("x-stub-delay-ms", "500")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Timeout);
    assert!(error.retryable);
}

#[tokio::test]
async fn test_unreachable_endpoint() {
    // Bind a port and close it right away: nothing listens there.
    let dead_addr = free_addr();
    let t =
        TestProxy::start_with_addresses(vec![dead_addr.to_string().parse().unwrap()], vec![]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::UpstreamTransient);
    assert!(error.retryable);
}

// ---------------------------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------------------------

/// Sends a `Transfer-Encoding: chunked` request made of whitespace chunks of the given sizes to
/// the public decryption route, and returns the response status.
async fn send_chunked(t: &TestProxy, chunk_sizes: &[usize]) -> StatusCode {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let addr = t.addr;
    let mut stream = t.tls.connect(addr).await;
    let mut request = format!(
        "POST {PUBLIC_DECRYPTION_ROUTE} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: Bearer {API_KEY}\r\n\
         Content-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
    )
    .into_bytes();
    for size in chunk_sizes {
        request.extend_from_slice(format!("{size:x}\r\n").as_bytes());
        request.extend(std::iter::repeat_n(b' ', *size));
        request.extend_from_slice(b"\r\n");
    }
    request.extend_from_slice(b"0\r\n\r\n");
    stream.write_all(&request).await.unwrap();

    let mut response = Vec::new();
    // The server may close without a TLS `close_notify`, which surfaces as an EOF error.
    let _ = stream.read_to_end(&mut response).await;
    let response = String::from_utf8_lossy(&response);
    response
        .split_whitespace()
        .nth(1)
        .and_then(|status| StatusCode::from_bytes(status.as_bytes()).ok())
        .unwrap_or_else(|| panic!("unexpected response: {response}"))
}
