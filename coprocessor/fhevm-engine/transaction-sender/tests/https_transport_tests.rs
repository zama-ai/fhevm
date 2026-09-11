//! Transport-only change: the Gateway endpoint moved from WebSocket to HTTP.
//!
//! These cover the parts of that move that are not visible in the submission
//! path: URL validation, the client policy, and the fact that inclusion is
//! still awaited by the node rather than polled by the client.

use transaction_sender::gateway_http_client;

/// A leftover `wss://` value must fail immediately, not be silently rewritten
/// and not be retried forever.
#[test]
fn websocket_urls_are_rejected_rather_than_rewritten() {
    for url in ["wss://gateway.invalid/rpc", "ws://gateway.invalid/rpc"] {
        let parsed: alloy::transports::http::reqwest::Url = url.parse().expect("url");
        let err = gateway_http_client(&parsed).expect_err("must reject");
        let msg = err.to_string();
        assert!(
            msg.contains("http:// or https://"),
            "error should name the requirement, got: {msg}"
        );
        assert!(
            !msg.contains("https://gateway.invalid"),
            "the URL must not be rewritten for the caller: {msg}"
        );
    }
}

#[test]
fn http_and_https_urls_are_accepted() {
    for url in ["http://gateway.invalid/rpc", "https://gateway.invalid/rpc"] {
        let parsed: alloy::transports::http::reqwest::Url = url.parse().expect("url");
        gateway_http_client(&parsed).expect("must accept");
    }
}

/// The submission path must not inherit a transport-level retry: a retried
/// `eth_sendRawTransactionSync` is indistinguishable from a duplicate send, and
/// the operation retry loop already owns recovery.
#[tokio::test]
async fn submission_is_not_retried_by_the_transport() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let hits = Arc::new(AtomicUsize::new(0));
    let seen = hits.clone();
    let app = axum::Router::new().route(
        "/",
        axum::routing::post(move || {
            let seen = seen.clone();
            async move {
                seen.fetch_add(1, Ordering::SeqCst);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "upstream exploded",
                )
            }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let url: alloy::transports::http::reqwest::Url =
        format!("http://{addr}/").parse().expect("url");
    let client = gateway_http_client(&url).expect("client");
    let res = client.post(url).body("{}").send().await.expect("response");

    assert_eq!(res.status().as_u16(), 500);
    assert_eq!(
        hits.load(Ordering::SeqCst),
        1,
        "the failed request must reach the server exactly once"
    );
}

/// A redirect must not be followed: it would resend the signed transaction to
/// a host the operator did not configure.
#[tokio::test]
async fn redirects_are_not_followed() {
    let app = axum::Router::new()
        .route(
            "/",
            axum::routing::post(|| async {
                (
                    axum::http::StatusCode::TEMPORARY_REDIRECT,
                    [(axum::http::header::LOCATION, "/elsewhere")],
                    "",
                )
            }),
        )
        .route("/elsewhere", axum::routing::post(|| async { "reached" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    let url: alloy::transports::http::reqwest::Url =
        format!("http://{addr}/").parse().expect("url");
    let client = gateway_http_client(&url).expect("client");
    let res = client.post(url).body("{}").send().await.expect("response");
    assert_eq!(
        res.status().as_u16(),
        307,
        "the redirect must not be followed"
    );
}

/// Every non-HTTP scheme is refused, and the message names what is required
/// rather than what was given — the Gateway URL carries an auth token on this
/// endpoint, so it must not be echoed.
#[test]
fn non_http_schemes_are_refused_without_echoing_the_url() {
    for url in [
        "ws://gateway.invalid/rpc/SECRET",
        "wss://gateway.invalid/rpc/SECRET",
        "file:///tmp/socket",
        "ipc:///tmp/geth.ipc",
    ] {
        let parsed = url
            .parse::<alloy::transports::http::reqwest::Url>()
            .expect("valid fixture URL");
        let err = gateway_http_client(&parsed)
            .expect_err("only http and https may be accepted")
            .to_string();
        assert!(
            err.contains("http:// or https://"),
            "the error must name the requirement, got: {err}"
        );
        assert!(
            !err.contains("SECRET"),
            "the error must not echo the URL, got: {err}"
        );
    }
}

#[tokio::test]
async fn public_chain_id_probe_returns_configuration_errors_without_panicking() {
    let error = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        transaction_sender::get_chain_id(
            "wss://gateway.invalid/SECRET".parse().unwrap(),
            std::time::Duration::from_secs(4),
        ),
    )
    .await
    .expect("invalid scheme must not be retried")
    .expect_err("invalid scheme must return an error");
    assert!(!format!("{error:?}").contains("SECRET"));
}
