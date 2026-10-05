//! End-to-end tests of the proxy against a stub Connector endpoint.

mod common;

use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::StatusCode;
use common::{
    MAX_BODY_BYTES, StubEndpoint, TestProxy, auth_header, auth_header_expiring, free_addr, now,
    sender,
};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
    VersionResponse,
};
use std::time::Duration;

#[tokio::test]
async fn test_forwards_authenticated_requests_untouched() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t.get(VERSION_ROUTE).await.send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await.unwrap(),
        VersionResponse::default()
    );

    // Deliberately non-canonical JSON: the proxy must forward it byte-for-byte.
    let body = r#"{ "ctHandles":["0x01"] ,  "extraData":"0x00"}"#;
    let response = t
        .post(PUBLIC_DECRYPTION_ROUTE, body)
        .await
        .header("content-type", "application/json")
        .header("x-forwarded-for", "9.9.9.9")
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
        .post(USER_DECRYPTION_ROUTE, "{}")
        .await
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rejects_unauthenticated_requests() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let route = PUBLIC_DECRYPTION_ROUTE;
    let stranger = PrivateKeySigner::random();

    for (request, auth_code) in [
        (t.client.post(t.url(route)).body("{}"), "auth_missing"),
        (
            t.client.post(t.url(route)).bearer_auth("test").body("{}"),
            "auth_malformed",
        ),
        (
            t.client
                .post(t.url(route))
                .header("authorization", auth_header(&stranger, route, b"{}").await)
                .body("{}"),
            "auth_unknown_signer",
        ),
        (
            // Signed for another body: the endpoint gets a truncated request, never handled.
            t.client
                .post(t.url(route))
                .header("authorization", auth_header(&sender(), route, b"{}").await)
                .body("{ }"),
            "auth_body_mismatch",
        ),
        (
            // Signed for another route.
            t.client
                .post(t.url(USER_DECRYPTION_ROUTE))
                .header("authorization", auth_header(&sender(), route, b"{}").await)
                .body("{}"),
            "auth_unknown_signer",
        ),
        (
            t.client
                .post(format!("{}?a=1", t.url(route)))
                .header("authorization", auth_header(&sender(), route, b"{}").await)
                .body("{}"),
            "auth_query_string",
        ),
        (t.client.get(t.url(VERSION_ROUTE)), "auth_missing"),
    ] {
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers()["www-authenticate"].to_str().unwrap(),
            "FhevmSig"
        );
        assert_eq!(
            response.headers()["content-type"].to_str().unwrap(),
            "application/json"
        );
        let error: ErrorResponse = response.json().await.unwrap();
        assert_eq!(error.code, ErrorCode::SenderAuthenticationFailed);
        assert!(error.message.starts_with(auth_code), "{}", error.message);
        assert!(!error.retryable);
        assert_eq!(error.decryption_id, None);
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_rejects_targets_the_uri_does_not_reproduce() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let route = PUBLIC_DECRYPTION_ROUTE;
    let addr = t.addr;
    let auth = auth_header(&sender(), route, b"{}").await;
    let request = |target: &[u8]| {
        let mut request = b"POST ".to_vec();
        request.extend_from_slice(target);
        request.extend_from_slice(
            format!(
                " HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\n\
                 Content-Length: 2\r\nConnection: close\r\n\r\n{{}}"
            )
            .as_bytes(),
        );
        request
    };

    // The signed route, but absolute-form: the URI keeps only its path.
    let absolute = format!("https://{addr}{route}");
    assert_eq!(
        send_raw(&t, &request(absolute.as_bytes())).await,
        StatusCode::BAD_REQUEST
    );
    // Not UTF-8: the URI holds a lossy rendering.
    let lossy = [route.as_bytes(), b"\xff"].concat();
    assert_eq!(
        send_raw(&t, &request(&lossy)).await,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");

    // The same request in origin-form goes through.
    assert_eq!(
        send_raw(&t, &request(route.as_bytes())).await,
        StatusCode::OK
    );
}

#[tokio::test]
async fn test_rejects_expired_headers_as_retryable() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let route = PUBLIC_DECRYPTION_ROUTE;
    let expired = auth_header_expiring(&sender(), route, b"{}", now() - 120).await;

    let response = t
        .client
        .post(t.url(route))
        .header("authorization", expired)
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::AuthExpired);
    assert!(error.retryable);
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
        // Even with credentials valid for the route.
        let auth = auth_header(&sender(), PUBLIC_DECRYPTION_ROUTE, b"").await;
        let response = request.header("authorization", auth).send().await.unwrap();
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
        .post(PUBLIC_DECRYPTION_ROUTE, vec![b' '; MAX_BODY_BYTES + 1])
        .await
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
    let hits = t.endpoints[0].hits();
    // A chunked body that does not match its signature is never handled.
    let status = send_chunked_signed_for(&t, &[2, 2], b"{}").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(t.endpoints[0].hits(), hits);

    // Exactly at the limit is fine.
    let response = t
        .post(PUBLIC_DECRYPTION_ROUTE, vec![b' '; MAX_BODY_BYTES])
        .await
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_passes_endpoint_errors_through() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .post(PUBLIC_DECRYPTION_ROUTE, "{}")
        .await
        .header("x-stub-overloaded", "1")
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
            .post(PUBLIC_DECRYPTION_ROUTE, "{}")
            .await
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
        |config| config.endpoint_response_timeout = Duration::from_millis(100),
    )
    .await;

    let response = t
        .post(PUBLIC_DECRYPTION_ROUTE, "{}")
        .await
        .header("x-stub-delay-ms", "500")
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
        .post(PUBLIC_DECRYPTION_ROUTE, "{}")
        .await
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
    let body = vec![b' '; chunk_sizes.iter().sum()];
    send_chunked_signed_for(t, chunk_sizes, &body).await
}

/// [`send_chunked`] with a header signed for `signed_body`.
async fn send_chunked_signed_for(
    t: &TestProxy,
    chunk_sizes: &[usize],
    signed_body: &[u8],
) -> StatusCode {
    let addr = t.addr;
    let auth = auth_header(&sender(), PUBLIC_DECRYPTION_ROUTE, signed_body).await;
    let mut request = format!(
        "POST {PUBLIC_DECRYPTION_ROUTE} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\n\
         Content-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
    )
    .into_bytes();
    for size in chunk_sizes {
        request.extend_from_slice(format!("{size:x}\r\n").as_bytes());
        request.extend(std::iter::repeat_n(b' ', *size));
        request.extend_from_slice(b"\r\n");
    }
    request.extend_from_slice(b"0\r\n\r\n");
    send_raw(t, &request).await
}

/// Sends raw request bytes over TLS and returns the response status.
async fn send_raw(t: &TestProxy, request: &[u8]) -> StatusCode {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = t.tls.connect(t.addr).await;
    stream.write_all(request).await.unwrap();

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
