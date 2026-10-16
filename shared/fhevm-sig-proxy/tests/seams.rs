//! The per-deployment settings, observed from outside: route table, upstreams, per-route body cap
//! and timeout, `Authorization` forwarding.

mod common;

use common::TestProxy;
use fhevm_sig_proxy::{
    ProxyConfig,
    conformance::{ENDPOINT_HEALTHCHECK_FREQUENCY, authorization, free_port},
};
use http::{Method, StatusCode};
use std::time::Duration;

fn defaults(_: &mut ProxyConfig) {}

#[tokio::test]
async fn each_route_reaches_its_own_upstream() {
    let t = TestProxy::start(defaults).await;

    assert_eq!(
        t.signed(Method::POST, "/small", b"").await.status(),
        StatusCode::OK
    );
    assert_eq!(t.a.requests().len(), 1);
    assert_eq!(t.b.requests().len(), 0);

    assert_eq!(
        t.signed(Method::POST, "/large", b"{}").await.status(),
        StatusCode::OK
    );
    assert_eq!(
        t.signed(Method::GET, "/items/7", b"").await.status(),
        StatusCode::OK
    );
    assert_eq!(t.a.requests().len(), 1);
    let targets: Vec<_> = t.b.requests().into_iter().map(|r| r.target).collect();
    assert_eq!(targets, ["/large", "/items/7"]);
}

#[tokio::test]
async fn each_route_has_its_own_body_cap() {
    let t = TestProxy::start(defaults).await;

    let refused = t.signed(Method::POST, "/small", b"x").await;
    assert_eq!(refused.status(), StatusCode::BAD_REQUEST);
    assert_eq!(refused.text().await.unwrap(), "BodyTooLarge");
    // A declared length over the cap is refused before any endpoint connection.
    assert!(t.a.heads().is_empty());

    let body = vec![b'x'; 1024];
    assert_eq!(
        t.signed(Method::POST, "/large", &body).await.status(),
        StatusCode::OK
    );
    assert_eq!(t.b.requests()[0].body, body);
}

#[tokio::test]
async fn a_streamed_body_is_capped_by_its_route() {
    let t = TestProxy::start(defaults).await;
    let chunked = |body: &'static [u8]| {
        let chunks = futures_util::stream::iter([Ok::<_, std::io::Error>(body)]);
        reqwest::Body::wrap_stream(chunks)
    };

    let refused = t
        .client
        .post(t.url("/small"))
        .header("authorization", authorization("/small", b"x").await)
        .body(chunked(b"x"))
        .send()
        .await
        .unwrap();
    assert_eq!(refused.status(), StatusCode::BAD_REQUEST);
    assert!(t.a.requests().is_empty());

    let accepted = t
        .client
        .post(t.url("/large"))
        .header("authorization", authorization("/large", b"x").await)
        .body(chunked(b"x"))
        .send()
        .await
        .unwrap();
    assert_eq!(accepted.status(), StatusCode::OK);
    assert_eq!(t.b.requests()[0].body, "x");
}

#[tokio::test]
async fn each_route_has_its_own_response_timeout() {
    let t = TestProxy::start(defaults).await;
    t.a.hold(Duration::from_secs(1));
    t.b.hold(Duration::from_secs(1));

    let timed_out = t.signed(Method::POST, "/small", b"").await;
    assert_eq!(timed_out.status(), StatusCode::GATEWAY_TIMEOUT);
    assert_eq!(timed_out.text().await.unwrap(), "EndpointTimeout");
    assert_eq!(
        t.signed(Method::POST, "/large", b"").await.status(),
        StatusCode::OK
    );
}

#[tokio::test]
async fn a_typed_route_accepts_only_its_methods_and_canonical_parameters() {
    let t = TestProxy::start(defaults).await;

    assert_eq!(
        t.signed(Method::HEAD, "/items/65535", b"").await.status(),
        StatusCode::OK
    );

    let not_allowed = t.signed(Method::POST, "/items/3", b"").await;
    assert_eq!(not_allowed.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(not_allowed.headers()["allow"], "GET, HEAD");

    for path in ["/items/03", "/items/65536"] {
        let refused = t.signed(Method::GET, path, b"").await;
        assert_eq!(refused.status(), StatusCode::BAD_REQUEST, "{path}");
        assert_eq!(refused.text().await.unwrap(), "NonCanonicalPath");
    }
    assert_eq!(
        t.signed(Method::GET, "/items", b"").await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(t.b.requests().len(), 1);
}

#[tokio::test]
async fn authorization_is_removed_unless_the_endpoints_read_it() {
    let removed = TestProxy::start(defaults).await;
    removed.signed(Method::GET, "/items/1", b"").await;
    assert!(
        !removed.b.requests()[0]
            .headers
            .contains_key("authorization")
    );

    let forwarded = TestProxy::start(|config| config.forward_authorization = true).await;
    let sent = authorization("/items/1", b"").await;
    let response = forwarded
        .client
        .get(forwarded.url("/items/1"))
        .header("authorization", &sent)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(forwarded.b.requests()[0].headers["authorization"], sent);
}

#[tokio::test]
async fn hop_by_hop_and_connection_nominated_headers_never_reach_an_endpoint() {
    let t = TestProxy::start(defaults).await;
    let response = t
        .client
        .get(t.url("/items/1"))
        .header("authorization", authorization("/items/1", b"").await)
        .header("connection", "keep-alive, x-private-note")
        .header("x-private-note", "secret")
        .header("keep-alive", "timeout=5")
        .header("proxy-authorization", "Basic eDp5")
        .header("te", "trailers")
        .header("trailer", "x-checksum")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let headers = &t.b.requests()[0].headers;
    for name in [
        "x-private-note",
        "keep-alive",
        "proxy-authorization",
        "te",
        "trailer",
    ] {
        assert!(!headers.contains_key(name), "{name} reached the endpoint");
    }
}

#[tokio::test]
async fn a_websocket_upgrade_reaches_an_endpoint_as_a_plain_request() {
    let t = TestProxy::start(defaults).await;
    let response = t
        .client
        .get(t.url("/items/1"))
        .header("authorization", authorization("/items/1", b"").await)
        .header("connection", "Upgrade")
        .header("upgrade", "websocket")
        .header("sec-websocket-version", "13")
        .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let headers = &t.b.requests()[0].headers;
    for name in ["connection", "upgrade"] {
        assert!(!headers.contains_key(name), "{name} reached the endpoint");
    }
}

#[tokio::test]
async fn a_request_without_the_signed_body_never_reaches_an_endpoint() {
    let t = TestProxy::start(defaults).await;
    let signed_for_a_body = authorization("/items/1", b"x").await;

    let response = t
        .client
        .get(t.url("/items/1"))
        .header("authorization", signed_for_a_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.text().await.unwrap(), "Auth(BodyMismatch)");
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(t.b.heads().is_empty());
}

#[tokio::test]
async fn an_upstream_down_fails_only_its_own_routes() {
    let down = format!("127.0.0.1:{}", free_port());
    let t = TestProxy::start(|config| {
        config.upstreams[1].endpoint_addresses = vec![down.parse().unwrap()];
    })
    .await;
    tokio::time::sleep(ENDPOINT_HEALTHCHECK_FREQUENCY * 3).await;

    let health = t.state.endpoints_health();
    assert_eq!(health.0[0].name, "a");
    assert_eq!(health.0[0].healthy, [t.a.addr.to_string()]);
    assert_eq!(health.0[1].name, "b");
    assert_eq!(health.0[1].unhealthy, [down]);
    assert!(health.any_upstream_healthy());
    let unavailable = t.signed(Method::GET, "/items/1", b"").await;
    assert_eq!(unavailable.status(), StatusCode::BAD_GATEWAY);
    assert_eq!(unavailable.text().await.unwrap(), "EndpointUnavailable");
    assert_eq!(
        t.signed(Method::POST, "/small", b"").await.status(),
        StatusCode::OK
    );
    // The TLS listener answers its self-check on a typed route.
    assert_eq!(t.state.tls_listener_healthcheck().await, Ok(()));
}
