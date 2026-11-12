mod common;

use alloy::transports::http::reqwest::{self, StatusCode};
use common::{
    FakeRpc, KmsContext, StubPrfService, TestProxy, TestTls, free_addr, prf_path, sender,
    wait_for_listener,
};
use coprocessor_proxy::{core::Proxy, monitoring::health::State};
use fhevm_engine_common::healthz_server::HttpServer;
use rstest::rstest;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    num::{NonZeroU32, NonZeroUsize},
    sync::Arc,
    time::Duration,
};
use tokio_util::sync::CancellationToken;

/// The `/healthz` answer of the coprocessor health server.
#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    dependencies: HashMap<String, String>,
    details: String,
}

impl HealthResponse {
    fn check(&self, name: &str) -> &str {
        &self.dependencies[name]
    }
}

/// Serves `/healthz`, `/liveness` and `/version` of `state` on a free port.
async fn serve_health(state: State, cancel_token: &CancellationToken) -> String {
    let port = free_addr().port();
    let server = HttpServer::new(Arc::new(state), port, cancel_token.child_token());
    tokio::spawn(async move { server.start().await });
    let addr = format!("127.0.0.1:{port}").parse().unwrap();
    wait_for_listener(addr).await;
    format!("http://{addr}")
}

async fn fetch_status(base_url: &str) -> anyhow::Result<(StatusCode, HealthResponse)> {
    let response = reqwest::get(format!("{base_url}/healthz")).await?;
    Ok((response.status(), response.json().await?))
}

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_healthcheck_endpoint() -> anyhow::Result<()> {
    // The self-check carries no `X-Forwarded-For`, and must pass behind a front layer too.
    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.forwarded_hops = 1;
    })
    .await;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(t.state.clone(), &cancel_token).await;

    let response = reqwest::get(format!("{base_url}/liveness")).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let response = reqwest::get(format!("{base_url}/version")).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let version: Value = response.json().await?;
    assert_eq!(version["name"], "coprocessor-proxy");

    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    assert_eq!(status.status, "healthy");
    for check in ["tls_listener", "kms_tx_senders"] {
        assert_eq!(status.check(check), "ok", "{status:?}");
    }
    assert_eq!(status.dependencies.len(), 2, "{status:?}");
    assert!(status.details.is_empty(), "{status:?}");

    // Without prf-service the proxy stays ready: it answers the requests itself, retryable.
    t.prf_service.stop().await;
    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    let response = t
        .get(&prf_path(1))
        .await
        .header("x-forwarded-for", "127.0.0.1")
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let error: Value = response.json().await?;
    assert_eq!(error["code"], "upstream_transient");
    assert_eq!(error["retryable"], true);

    cancel_token.cancel();
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_healthcheck_unreachable_tls_listener() -> anyhow::Result<()> {
    // Build the `Proxy` without starting it: its TLS listener is down. Pingora's server cannot
    // be stopped once started, so this case cannot be part of the test above.
    let prf_service = StubPrfService::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let addr = free_addr();
    let tls = TestTls::generate(addr);
    let config = TestProxy::config(addr, &tls, &rpc, prf_service.addr.to_string().parse()?);
    let (_proxy, state) = Proxy::from_config(config)?;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(state, &cancel_token).await;

    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(status.check("tls_listener"), "fail");
    // Nothing reads the KMS tx-senders while the proxy does not run.
    assert_eq!(status.check("kms_tx_senders"), "fail");
    assert!(status.details.contains("TLS listener"), "{status:?}");
    assert!(status.details.contains("KMS tx-senders"), "{status:?}");

    cancel_token.cancel();
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_not_ready_before_the_first_read() -> anyhow::Result<()> {
    let prf_service = StubPrfService::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    rpc.set_failing(true);
    let t = TestProxy::launch(rpc, prf_service, |_| {}).await;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(t.state.clone(), &cancel_token).await;

    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(status.check("kms_tx_senders"), "fail");
    assert_eq!(status.check("tls_listener"), "ok");

    t.rpc.set_failing(false);
    t.wait_until_accepted(&sender()).await;
    let (_, status) = fetch_status(&base_url).await?;
    assert_eq!(status.check("kms_tx_senders"), "ok");

    cancel_token.cancel();
    Ok(())
}

#[rstest]
#[case::without_a_front_layer(0)]
#[case::behind_a_front_layer(1)]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_ready_while_the_replica_holds_its_connection_limit(
    #[case] forwarded_hops: usize,
) -> anyhow::Result<()> {
    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.max_connections = NonZeroUsize::new(1).unwrap();
        config.forwarded_hops = forwarded_hops;
    })
    .await;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(t.state.clone(), &cancel_token).await;

    // The client keeps the connection of an authenticated request open: the replica is at its
    // limit. The proxy counts the startup connections out only after it noticed their close.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        let request = t.get(&prf_path(1)).await;
        match request.header("x-forwarded-for", "127.0.0.1").send().await {
            Ok(response) => {
                assert_eq!(response.status(), StatusCode::OK);
                response.bytes().await?;
                break;
            }
            Err(e) => assert!(tokio::time::Instant::now() < deadline, "{e}"),
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(t.tls.try_connect(t.addr).await.is_err());

    // Each self-check opens a connection and uses its pass: none is left for the next
    // connection, which the limit still refuses.
    for _ in 0..2 {
        let (status_code, status) = fetch_status(&base_url).await?;
        assert_eq!(status_code, StatusCode::OK, "{status:?}");
        assert_eq!(status.check("tls_listener"), "ok", "{status:?}");
    }
    assert!(t.tls.try_connect(t.addr).await.is_err());

    cancel_token.cancel();
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_ready_while_the_connections_waiting_for_their_first_head_are_at_their_cap()
-> anyhow::Result<()> {
    use tokio::io::AsyncReadExt;

    // 5 new connections per second and 2 s for the first head: at most 10 wait at once.
    let t = TestProxy::start_with_config(StubPrfService::start().await, |config| {
        config.max_new_connections_per_second = NonZeroU32::new(5).unwrap();
        config.request_head_timeout = Duration::from_secs(2);
    })
    .await;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(t.state.clone(), &cancel_token).await;
    // The startup connections leave the rate's 1 s window.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let refused = || {
        prometheus::gather()
            .iter()
            .filter(|family| family.name() == "coprocessor_proxy_connections_refused_counter")
            .flat_map(|family| family.get_metric())
            .filter(|metric| metric.get_label()[0].value() == "first_heads")
            .map(|metric| metric.get_counter().value())
            .sum::<f64>()
    };
    let refused_before = refused();

    // Ten connections accepted within the rate, whose handshakes then complete together: they
    // take every place for 2 s.
    let mut tcp = Vec::new();
    for _ in 0..10 {
        tcp.push(tokio::net::TcpStream::connect(t.addr).await?);
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    let handshakes = tcp.into_iter().map(|tcp| t.tls.handshake(tcp));
    let _waiting = futures_util::future::try_join_all(handshakes).await?;

    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    assert_eq!(status.check("tls_listener"), "ok", "{status:?}");

    // The self-check used its pass: the next connection finds no place and is closed.
    let mut stream = t.tls.try_connect(t.addr).await?;
    let mut byte = [0u8; 1];
    let read = tokio::time::timeout(Duration::from_millis(500), stream.read(&mut byte)).await;
    assert!(matches!(read, Ok(Ok(0) | Err(_))), "{read:?}");
    assert_eq!(refused() - refused_before, 1.0);

    cancel_token.cancel();
    Ok(())
}
