//! End-to-end tests of the proxy against a stub prf-service endpoint.

mod common;

use alloy::{
    primitives::{Address, keccak256},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::{Response, StatusCode},
};
use axum::http::Version;
use common::{
    ACCEPTED, FakeRpc, KmsContext, StubEndpoint, TestProxy, auth_header, auth_header_expiring,
    auth_header_for, free_addr, now, prf_path, rate_limited_path, sender, stub_attestation,
    stub_output,
};
use coprocessor_proxy::core::UNKNOWN_SIGNER_WAIT;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

/// Checks an error the proxy wrote itself: its status, RFC-033 body, and that it is not cached.
async fn assert_proxy_error(response: Response, status: StatusCode, code: &str, retryable: bool) {
    assert_eq!(response.status(), status);
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["content-type"], "application/json");
    let error: Value = response.json().await.unwrap();
    assert_eq!(error["code"], code, "{error}");
    assert_eq!(error["retryable"], retryable, "{error}");
    assert!(error["message"].is_string(), "{error}");
}

#[tokio::test]
async fn test_forwards_authenticated_requests_untouched() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);

    let auth = auth_header(&sender(), &path, b"").await;
    let response = t
        .client
        .get(t.url(&path))
        .header("authorization", &auth)
        .header("x-forwarded-for", "9.9.9.9")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["content-type"],
        "application/octet-stream"
    );
    assert_eq!(
        response.headers()["x-prf-attestation"],
        stub_attestation(&path).as_str()
    );
    assert!(response.headers().get("cache-control").is_none());
    let body = response.bytes().await.unwrap();
    assert_eq!(keccak256(&body), keccak256(stub_output(&path)));

    let seen = t.endpoints[0].last_request();
    assert_eq!(seen.version, Some(Version::HTTP_2));
    // prf-service checks the `FhevmSig` header again.
    assert_eq!(seen.headers["authorization"], auth.as_str());
    assert_eq!(seen.headers["x-forwarded-for"], "127.0.0.1");
    assert_eq!(seen.headers["x-forwarded-proto"], "https");

    let response = t
        .client
        .head(t.url(&path))
        .header("authorization", &auth)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["x-prf-attestation"],
        stub_attestation(&path).as_str()
    );
    assert!(response.bytes().await.unwrap().is_empty());
}

#[tokio::test]
async fn test_rejects_unauthenticated_requests() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);
    let stranger = PrivateKeySigner::random();
    let other_coprocessor = Address::repeat_byte(0xa2);

    for (request, auth_code) in [
        (t.client.get(t.url(&path)), "auth_missing"),
        (
            t.client.get(t.url(&path)).bearer_auth("test"),
            "auth_malformed",
        ),
        (
            t.client
                .get(t.url(&path))
                .header("authorization", auth_header(&stranger, &path, b"").await),
            "auth_unknown_signer",
        ),
        (
            // Signed for another path.
            t.client
                .get(t.url(&prf_path(2)))
                .header("authorization", auth_header(&sender(), &path, b"").await),
            "auth_unknown_signer",
        ),
        (
            // Signed for another coprocessor.
            t.client.get(t.url(&path)).header(
                "authorization",
                auth_header_for(&sender(), other_coprocessor, &path, b"", now() + 60).await,
            ),
            "auth_unknown_signer",
        ),
        (
            // Signed for a body that is not sent.
            t.client
                .get(t.url(&path))
                .header("authorization", auth_header(&sender(), &path, b"{}").await),
            "auth_body_mismatch",
        ),
        (
            t.client
                .get(format!("{}?a=1", t.url(&path)))
                .header("authorization", auth_header(&sender(), &path, b"").await),
            "auth_query_string",
        ),
        (
            t.client.get(t.url(&path)).header(
                "authorization",
                auth_header_expiring(&sender(), &path, b"", now() + 3600).await,
            ),
            "auth_too_long_lived",
        ),
    ] {
        let response = request.send().await.unwrap();
        assert_eq!(response.headers()["www-authenticate"], "FhevmSig");
        assert_proxy_error(response, StatusCode::UNAUTHORIZED, auth_code, false).await;
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_rejects_targets_the_uri_does_not_reproduce() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);
    let addr = t.addr;
    let auth = auth_header(&sender(), &path, b"").await;
    let request = |target: &[u8]| {
        let mut request = b"GET ".to_vec();
        request.extend_from_slice(target);
        request.extend_from_slice(
            format!(
                " HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\nConnection: close\r\n\r\n"
            )
            .as_bytes(),
        );
        request
    };

    // The signed path, but absolute-form: the URI keeps only its path.
    let absolute = format!("https://{addr}{path}");
    assert_eq!(
        send_raw(&t, &request(absolute.as_bytes())).await,
        StatusCode::BAD_REQUEST
    );
    // Not UTF-8: the URI holds a lossy rendering.
    let lossy = [path.as_bytes(), b"\xff"].concat();
    assert_eq!(
        send_raw(&t, &request(&lossy)).await,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");

    // The same request in origin-form goes through.
    assert_eq!(
        send_raw(&t, &request(path.as_bytes())).await,
        StatusCode::OK
    );
}

#[tokio::test]
async fn test_rejects_expired_headers_as_retryable() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);
    let expired = auth_header_expiring(&sender(), &path, b"", now() - 120).await;

    let response = t
        .client
        .get(t.url(&path))
        .header("authorization", expired)
        .send()
        .await
        .unwrap();
    assert_proxy_error(response, StatusCode::UNAUTHORIZED, "auth_expired", true).await;
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_default_deny_routing() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let label = "01".repeat(32);

    for (method, path, expected, code) in [
        ("GET", "/".to_string(), StatusCode::NOT_FOUND, "not_found"),
        (
            "GET",
            "/healthz".to_string(),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            "GET",
            format!("/v1/prf/1/labels/{label}"),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            "GET",
            format!("/v1/prf/1/labels/{label}/64/"),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            "GET",
            "/v1/prf/1/labels/../64".to_string(),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            "GET",
            format!("/v1/prf/./labels/{label}/64"),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            "POST",
            prf_path(1),
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
        ),
        (
            "PUT",
            prf_path(1),
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
        ),
    ] {
        // Even with credentials valid for the path.
        let auth = auth_header(&sender(), &path, b"").await;
        let response = t
            .client
            .request(method.parse().unwrap(), t.url(&path))
            .header("authorization", auth)
            .send()
            .await
            .unwrap();
        let allow = response
            .headers()
            .get("allow")
            .map(|v| v.to_str().unwrap().to_string());
        assert_eq!(
            allow.as_deref(),
            (expected == StatusCode::METHOD_NOT_ALLOWED).then_some("GET, HEAD"),
            "{method} {path}"
        );
        assert_proxy_error(response, expected, code, false).await;
    }

    // A `HEAD` answer has no body, an error included.
    let response = t.client.head(t.url("/")).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert!(response.bytes().await.unwrap().is_empty());
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");

    // The encoding of the parameters is prf-service's to check.
    let response = t
        .get("/v1/prf/01/labels/AB/064")
        .await
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(t.endpoints[0].hits(), 1);
}

#[tokio::test]
async fn test_rejects_request_bodies() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);

    let auth = auth_header(&sender(), &path, b"x").await;
    let response = t
        .client
        .get(t.url(&path))
        .header("authorization", auth)
        .body("x")
        .send()
        .await
        .unwrap();
    assert_proxy_error(response, StatusCode::BAD_REQUEST, "malformed", false).await;

    // Chunked bodies carry no `Content-Length`, the cap is enforced while streaming. The answer
    // comes at once: the HTTP/2 upstream path would otherwise wait for the endpoint until the
    // response timeout and answer `504`.
    let started = std::time::Instant::now();
    assert_eq!(
        send_chunked(&t, &path, &[1], b" ").await,
        StatusCode::BAD_REQUEST
    );
    assert!(started.elapsed() < Duration::from_secs(5));
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_checks_streamed_bodies_against_their_signature() {
    let t = TestProxy::start_with_config(vec![StubEndpoint::start().await], |config| {
        config.max_body_bytes = 4
    })
    .await;
    let path = prf_path(1);
    // What the endpoint received of the last request: it may receive the start of a body the
    // proxy rejects, never the whole of it.
    let received = || t.endpoints[0].last_request().body;

    assert_eq!(
        send_chunked(&t, &path, &[2, 3], b"     ").await,
        StatusCode::BAD_REQUEST
    );
    assert!(received().len() <= 4, "{:?}", received());
    assert_eq!(
        send_chunked(&t, &path, &[2, 2], b"{}").await,
        StatusCode::UNAUTHORIZED
    );
    assert!(received().len() < 4, "{:?}", received());
    assert_eq!(
        send_chunked(&t, &path, &[2, 2], b"    ").await,
        StatusCode::OK
    );
    assert_eq!(received(), b"    ");
}

#[tokio::test]
async fn test_errors_pingora_writes_are_not_cached() {
    let t = TestProxy::start_with_config(vec![StubEndpoint::start().await], |config| {
        config.max_body_bytes = 4;
        config.request_read_timeout = Duration::from_millis(500);
    })
    .await;
    let path = prf_path(1);
    let addr = t.addr;
    let auth = auth_header(&sender(), &path, b"    ").await;
    // Two of the four signed bytes, then nothing: reading the body times out, and Pingora's own
    // error answer is written.
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\n\
         Content-Length: 4\r\nConnection: close\r\n\r\n  "
    );

    let response = raw_response(&t, request.as_bytes())
        .await
        .to_ascii_lowercase();
    assert!(response.starts_with("http/1.1 400"), "{response}");
    assert!(
        response.contains("\r\ncache-control: no-store\r\n"),
        "{response}"
    );
}

#[tokio::test]
async fn test_passes_endpoint_answers_through() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t.get(&rate_limited_path()).await.send().await.unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(response.headers()["retry-after"], "2");
    assert!(response.headers().get("cache-control").is_none());
    let error: Value = response.json().await.unwrap();
    assert_eq!(
        error,
        json!({"code": "concurrency_limited", "message": "busy", "retryable": true})
    );
}

#[tokio::test]
async fn test_holds_requests_until_the_endpoint_answers() {
    let t = TestProxy::start_with_config(vec![StubEndpoint::start().await], |config| {
        config.endpoint_response_timeout = Duration::from_secs(4)
    })
    .await;
    let path = prf_path(1);

    let response = t
        .get(&path)
        .await
        .header("x-stub-delay-ms", "2000")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.bytes().await.unwrap(), stub_output(&path));
}

#[tokio::test]
async fn test_held_requests_share_one_endpoint_connection() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);
    // One request first, so its connection is pooled before the next ones look for one.
    let response = t.get(&path).await.send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let held = (0..5).map(|_| async {
        t.get(&path)
            .await
            .header("x-stub-delay-ms", "500")
            .send()
            .await
            .unwrap()
            .status()
    });
    for status in futures_util::future::join_all(held).await {
        assert_eq!(status, StatusCode::OK);
    }
    assert_eq!(t.endpoints[0].connections(), 1);
}

#[tokio::test]
async fn test_load_balances_across_endpoints() {
    let endpoints = vec![StubEndpoint::start().await, StubEndpoint::start().await];
    let t = TestProxy::start(endpoints).await;

    for _ in 0..10 {
        let response = t.get(&prf_path(1)).await.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
    assert_eq!(t.endpoints[0].hits() + t.endpoints[1].hits(), 10);
    assert!(t.endpoints[0].hits() > 0 && t.endpoints[1].hits() > 0);
}

#[tokio::test]
async fn test_upstream_timeout() {
    let t = TestProxy::start_with_config(vec![StubEndpoint::start().await], |config| {
        config.endpoint_response_timeout = Duration::from_millis(100)
    })
    .await;

    let response = t
        .get(&prf_path(1))
        .await
        .header("x-stub-delay-ms", "500")
        .send()
        .await
        .unwrap();
    // The proxy closes the connection after a failure: the client must not reuse it.
    assert_eq!(response.headers()["connection"], "close");
    assert_proxy_error(response, StatusCode::GATEWAY_TIMEOUT, "timeout", true).await;
}

#[tokio::test]
async fn test_unreachable_endpoint() {
    // Bind a port and close it right away: nothing listens there.
    let dead_addr = free_addr();
    let t = TestProxy::start_with_config(vec![], |config| {
        config.endpoint_addresses = vec![dead_addr.to_string().parse().unwrap()];
    })
    .await;

    let response = t.get(&prf_path(1)).await.send().await.unwrap();
    assert_eq!(response.headers()["connection"], "close");
    assert_proxy_error(
        response,
        StatusCode::BAD_GATEWAY,
        "upstream_transient",
        true,
    )
    .await;
}

#[tokio::test]
async fn test_endpoint_failure_mid_body_cuts_the_answer_short() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let path = prf_path(1);

    let mut response = t
        .get(&path)
        .await
        .header("x-stub-fail-mid-body", "1")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // The client gets what the endpoint sent, then the end of the connection: no error answer
    // inside the started body, and no answer that looks complete.
    let mut received = Vec::new();
    let end = loop {
        match response.chunk().await {
            Ok(Some(chunk)) => received.extend_from_slice(&chunk),
            end => break end,
        }
    };
    assert_eq!(received, stub_output(&path)[..32]);
    assert!(end.is_err(), "{end:?}");
}

#[tokio::test]
async fn test_accepted_signers_follow_protocol_config() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let next = PrivateKeySigner::random();

    // A context switch: both node sets are accepted while both contexts are Active.
    t.rpc.set_contexts(vec![
        KmsContext::active(&[sender().address()]),
        KmsContext::active(&[next.address()]),
    ]);
    t.wait_until_accepted(&next).await;
    assert_eq!(t.auth_code(&sender()).await, ACCEPTED);

    // The old context is destroyed.
    t.rpc.set_contexts(vec![
        KmsContext::destroyed(&[sender().address()]),
        KmsContext::active(&[next.address()]),
    ]);
    t.wait_until_refused(&sender()).await;

    // A failed read keeps the last set.
    t.rpc.set_failing(true);
    tokio::time::sleep(5 * common::POLL_INTERVAL).await;
    assert_eq!(t.auth_code(&next).await, ACCEPTED);
    assert_eq!(t.auth_code(&sender()).await, "auth_unknown_signer");
}

#[tokio::test]
async fn test_refuses_everything_before_the_first_read() {
    let endpoint = StubEndpoint::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    rpc.set_failing(true);
    let t = TestProxy::launch(rpc, vec![endpoint], |_| {}).await;

    tokio::time::sleep(3 * common::POLL_INTERVAL).await;
    assert_eq!(t.auth_code(&sender()).await, "auth_unknown_signer");
    assert_eq!(t.endpoints[0].hits(), 0);

    t.rpc.set_failing(false);
    t.wait_until_accepted(&sender()).await;
}

/// Starts the proxy with a poll interval no test waits for: only an unknown signer makes it read.
async fn start_without_polls() -> TestProxy {
    TestProxy::start_with_config(vec![StubEndpoint::start().await], |config| {
        config.kms_registry_poll_interval = Duration::from_secs(600);
    })
    .await
}

#[tokio::test]
async fn test_reads_again_for_an_unknown_signer() {
    let t = start_without_polls().await;
    let next = PrivateKeySigner::random();
    t.rpc.set_contexts(vec![
        KmsContext::active(&[sender().address()]),
        KmsContext::active(&[next.address()]),
    ]);

    let path = prf_path(1);
    let response = t
        .client
        .get(t.url(&path))
        .header("authorization", auth_header(&next, &path, b"").await)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(t.endpoints[0].hits(), 1);
}

#[tokio::test]
async fn test_a_miss_during_a_poll_waits_for_the_next_poll() {
    let t = start_without_polls().await;
    let next = PrivateKeySigner::random();
    let polls = t.rpc.finalized_polls();
    t.rpc.set_reply_delay(Duration::from_secs(1));

    let during_the_poll = async {
        // The poll woken below has the finalized block from before `next`'s context.
        let deadline = Instant::now() + Duration::from_secs(5);
        while t.rpc.finalized_polls() == polls {
            assert!(Instant::now() < deadline, "the miss woke no poll");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        // The next poll's five calls end about 1 s after this poll: a request checked when this
        // poll ends is refused.
        t.rpc.set_reply_delay(Duration::from_millis(200));
        t.rpc.set_contexts(vec![
            KmsContext::active(&[sender().address()]),
            KmsContext::active(&[next.address()]),
        ]);
        t.auth_code(&next).await
    };
    let unknown = PrivateKeySigner::random();
    let (_, code) = tokio::join!(t.auth_code(&unknown), during_the_poll);
    assert_eq!(code, ACCEPTED);
}

#[tokio::test]
async fn test_refuses_an_unknown_signer_within_the_wait() {
    let t = start_without_polls().await;
    t.rpc.set_stalled(true);

    let started = Instant::now();
    assert_eq!(
        t.auth_code(&PrivateKeySigner::random()).await,
        "auth_unknown_signer"
    );
    let waited = started.elapsed();
    assert!(
        waited >= UNKNOWN_SIGNER_WAIT && waited < UNKNOWN_SIGNER_WAIT + Duration::from_secs(1),
        "{waited:?}"
    );
}

#[tokio::test]
async fn test_concurrent_unknown_signers_share_one_read() {
    let t = start_without_polls().await;
    let unknown = PrivateKeySigner::random();
    // A woken read starts a second after the previous one: the requests below all arrive before.
    assert_eq!(t.auth_code(&unknown).await, "auth_unknown_signer");
    let polls = t.rpc.finalized_polls();

    let codes = futures_util::future::join_all((0..10).map(|_| t.auth_code(&unknown))).await;
    assert!(
        codes.iter().all(|code| code == "auth_unknown_signer"),
        "{codes:?}"
    );
    // Long enough for the woken reads that would follow the first one.
    tokio::time::sleep(Duration::from_secs(3)).await;
    assert_eq!(t.rpc.finalized_polls(), polls + 1);
}

#[tokio::test]
async fn test_counts_requests_by_route_and_status() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;
    let count = |route: &str, status: &str| {
        prometheus::gather()
            .iter()
            .filter(|family| family.name() == "coprocessor_proxy_requests_total")
            .flat_map(|family| family.get_metric())
            .filter(|metric| {
                let label = |name: &str| {
                    metric
                        .get_label()
                        .iter()
                        .find(|l| l.name() == name)
                        .map(|l| l.value().to_string())
                };
                label("route").as_deref() == Some(route)
                    && label("status").as_deref() == Some(status)
            })
            .map(|metric| metric.get_counter().value())
            .sum::<f64>()
    };
    let (ok, not_found) = (count("prf", "200"), count("none", "404"));

    assert_eq!(
        t.get(&prf_path(1)).await.send().await.unwrap().status(),
        StatusCode::OK
    );
    assert_eq!(
        t.client.get(t.url("/")).send().await.unwrap().status(),
        StatusCode::NOT_FOUND
    );
    assert!(count("prf", "200") > ok);
    assert!(count("none", "404") > not_found);
}

// ---------------------------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------------------------

/// Sends a `Transfer-Encoding: chunked` `GET` of `path` made of whitespace chunks of the given
/// sizes, with a header signed for `signed_body`, and returns the response status.
async fn send_chunked(
    t: &TestProxy,
    path: &str,
    chunk_sizes: &[usize],
    signed_body: &[u8],
) -> StatusCode {
    let addr = t.addr;
    let auth = auth_header(&sender(), path, signed_body).await;
    let mut request = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: {auth}\r\n\
         Transfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
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
    let response = raw_response(t, request).await;
    response
        .split_whitespace()
        .nth(1)
        .and_then(|status| StatusCode::from_bytes(status.as_bytes()).ok())
        .unwrap_or_else(|| panic!("unexpected response: {response}"))
}

/// Sends raw request bytes over TLS and returns the whole response, read until the server closes.
async fn raw_response(t: &TestProxy, request: &[u8]) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = t.tls.connect(t.addr).await;
    stream.write_all(request).await.unwrap();

    let mut response = Vec::new();
    // The server may close without a TLS `close_notify`, which surfaces as an EOF error.
    let _ = stream.read_to_end(&mut response).await;
    String::from_utf8_lossy(&response).into_owned()
}
