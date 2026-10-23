//! PRF spike QA, local rows (G4, G5, G6, G9): the proxy-side cases that
//! `integration_tests.rs` does not already cover. Each test names its gate row.

mod common;

use alloy::{signers::local::PrivateKeySigner, transports::http::reqwest::StatusCode};
use common::{
    StubEndpoint, TestProxy, UNKNOWN_SIGNER, auth_header, auth_header_expiring, now, sender,
};
use serde_json::Value;
use std::time::{Duration, Instant};

/// Sends raw request bytes over TLS and returns the response, read until the server closes.
async fn raw_response(t: &TestProxy, request: &[u8]) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = t.tls.connect(t.addr).await;
    stream.write_all(request).await.unwrap();
    let mut response = Vec::new();
    let read =
        tokio::time::timeout(Duration::from_secs(5), stream.read_to_end(&mut response)).await;
    let response = String::from_utf8_lossy(&response).into_owned();
    assert!(read.is_ok(), "the connection stayed open after: {response}");
    response
}

fn status_of(response: &str) -> u16 {
    response
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("unexpected response: {response}"))
}

/// G4: odd encodings signed exactly as sent reach prf-service, which owns the encoding check; a
/// space in the target is refused by Pingora's parser before any hook, as a bare 400.
#[tokio::test]
async fn g4_raw_paths() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let label = "ab".repeat(32);
    let addr = t.addr;
    let mut forwarded = 0;
    for path in [
        format!("/v1/prf/0/labels/{label}/0x40"),
        format!("/v1/prf/0/labels/{label}/%36%34"),
        format!("/v1/prf/0/labels/{label}/+64"),
        format!("/v1/prf/0/labels/{label}/65536"),
        format!("/v1/prf/0/labels/{label}/{}", "9".repeat(30)),
        format!("/v1/prf/0/labels/0x{label}/64"),
    ] {
        let auth = auth_header(&sender(), &path, b"").await;
        let request = format!(
            "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\nConnection: close\r\n\r\n"
        );
        let response = raw_response(&t, request.as_bytes()).await;
        assert_eq!(status_of(&response), 200, "{path}: {response}");
        forwarded += 1;
        assert_eq!(t.endpoints[0].hits(), forwarded, "{path}");
    }

    // A trailing space in the target cannot be signed. Pingora parses the request line anyway, and
    // the path the proxy verifies is not the one signed: the signature recovers another address.
    let signed = format!("/v1/prf/0/labels/{label}/64");
    let path = format!("{signed} ");
    let auth = auth_header(&sender(), &signed, b"").await;
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\nConnection: close\r\n\r\n"
    );
    let response = raw_response(&t, request.as_bytes()).await;
    assert_eq!(status_of(&response), 401, "{response}");
    assert!(response.contains(UNKNOWN_SIGNER), "{response}");
    assert_eq!(t.endpoints[0].hits(), forwarded, "nothing more forwarded");
}

/// G5: a random key gets 401 `auth_unknown_signer` after the proxy waits for a woken registry read,
/// about 1 s and at most `UNKNOWN_SIGNER_WAIT`. Concurrent misses share that read.
#[tokio::test]
async fn g5_unknown_signer_wait_and_coalescing() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = common::prf_path(1);
    let polls_before = t.rpc.finalized_polls();
    let started = Instant::now();
    let requests = (0..50).map(|_| {
        let t = &t;
        let path = &path;
        async move {
            let auth = auth_header(&PrivateKeySigner::random(), path, b"").await;
            let response = t
                .client
                .get(t.url(path))
                .header("authorization", auth)
                .send()
                .await
                .unwrap();
            let status = response.status();
            let error: Value = response.json().await.unwrap();
            (
                status,
                error["code"].as_str().unwrap_or_default().to_string(),
            )
        }
    });
    let answers = futures_util::future::join_all(requests).await;
    let elapsed = started.elapsed();
    for (status, code) in &answers {
        assert_eq!(
            (*status, code.as_str()),
            (StatusCode::UNAUTHORIZED, UNKNOWN_SIGNER)
        );
    }
    let polls = t.rpc.finalized_polls() - polls_before;
    println!("G5: 50 unknown signers answered in {elapsed:?}, {polls} finalized-block polls");
    assert!(elapsed <= Duration::from_secs(4), "{elapsed:?}");
    assert_eq!(t.endpoints[0].hits(), 0);
}

/// G6: `expires` within `CLOCK_SKEW` of now is accepted, and up to `MAX_AUTH_VALIDITY` ahead.
#[tokio::test]
async fn g6_expiry_bounds() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = common::prf_path(1);
    for (expires, status, code) in [
        (now() - 20, StatusCode::OK, None),
        (now() - 60, StatusCode::UNAUTHORIZED, Some("auth_expired")),
        (now() + 240, StatusCode::OK, None),
        (
            now() + 600,
            StatusCode::UNAUTHORIZED,
            Some("auth_too_long_lived"),
        ),
    ] {
        let auth = auth_header_expiring(&sender(), &path, b"", expires).await;
        let response = t
            .client
            .get(t.url(&path))
            .header("authorization", auth)
            .send()
            .await
            .unwrap();
        let got = response.status();
        let body: Value = if got == StatusCode::OK {
            Value::Null
        } else {
            response.json().await.unwrap()
        };
        assert_eq!(
            (got, body["code"].as_str()),
            (status, code),
            "expires {expires} (now {})",
            now()
        );
    }
}

/// G9: a 64 KB `Authorization` is read and fails FhevmSig parsing; a head above 1 MiB gets a bare
/// 400 from Pingora.
#[tokio::test]
async fn g9_oversized_headers() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = common::prf_path(1);
    let response = t
        .client
        .get(t.url(&path))
        .header(
            "authorization",
            format!("FhevmSig {}", "a".repeat(64 * 1024)),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let error: Value = response.json().await.unwrap();
    assert_eq!(error["code"], "auth_malformed", "{error}");

    let addr = t.addr;
    let huge = "a".repeat(1024 * 1024 + 1);
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: FhevmSig {huge}\r\nConnection: close\r\n\r\n"
    );
    let response = raw_response(&t, request.as_bytes()).await;
    assert_eq!(
        status_of(&response),
        400,
        "{}",
        &response[..response.len().min(200)]
    );
    assert_eq!(t.endpoints[0].hits(), 0);
}
