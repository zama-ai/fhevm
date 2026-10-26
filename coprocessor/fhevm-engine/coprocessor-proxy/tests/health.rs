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
use std::{collections::HashMap, sync::Arc, time::Duration};
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
    let t = TestProxy::start(StubPrfService::start().await).await;
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
    let response = t.get(&prf_path(1)).await.send().await?;
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
