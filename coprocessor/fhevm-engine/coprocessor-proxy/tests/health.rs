mod common;

use alloy::transports::http::reqwest::{self, StatusCode};
use axum::http::Version;
use common::{
    ENDPOINT_HEALTHCHECK_FREQUENCY, FakeRpc, KmsContext, StubEndpoint, TestProxy, TestTls,
    free_addr, prf_path, sender, wait_for_listener,
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
    // Start the `Proxy` with two endpoints, and its health server
    let endpoints = vec![StubEndpoint::start().await, StubEndpoint::start().await];
    let t = TestProxy::start(endpoints).await;
    let cancel_token = CancellationToken::new();
    let base_url = serve_health(t.state.clone(), &cancel_token).await;

    // Test `liveness` endpoint
    let response = reqwest::get(format!("{base_url}/liveness")).await?;
    assert_eq!(response.status(), StatusCode::OK);

    // Test `version` endpoint
    let response = reqwest::get(format!("{base_url}/version")).await?;
    assert_eq!(response.status(), StatusCode::OK);
    let version: Value = response.json().await?;
    assert_eq!(version["name"], "coprocessor-proxy");

    // Test the endpoint while everything is fine
    let (status_code, status) = fetch_status(&base_url).await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    assert_eq!(status.status, "healthy");
    for check in ["tls_listener", "kms_tx_senders"] {
        assert_eq!(status.check(check), "ok", "{status:?}");
    }
    // The endpoints' health is reported, not a readiness condition.
    assert_eq!(status.dependencies.len(), 2, "{status:?}");
    assert!(status.details.is_empty(), "{status:?}");
    // The endpoints are probed over HTTP/2, the protocol of the proxied requests.
    for endpoint in &t.endpoints {
        let versions = endpoint.probe_versions();
        assert!(
            !versions.is_empty() && versions.iter().all(|v| *v == Version::HTTP_2),
            "{versions:?}"
        );
    }

    // Stop one endpoint: degraded but healthy, the other one is still able to serve
    let (stopped, live) = (&t.endpoints[0], &t.endpoints[1]);
    stopped.stop().await;
    let (status_code, status) = wait_for_details(&base_url, &stopped.addr.to_string()).await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    assert_eq!(status.status, "healthy");

    // The proxy stops selecting the unhealthy endpoint
    let (live_hits, stopped_hits) = (live.hits(), stopped.hits());
    for _ in 0..10 {
        let response = t.get(&prf_path(1)).await.send().await?;
        assert_eq!(response.status(), StatusCode::OK);
    }
    assert_eq!(live.hits(), live_hits + 10);
    assert_eq!(stopped.hits(), stopped_hits);

    // Stop the last endpoint: the proxy stays ready, it reports the endpoints in the details and
    // answers the requests itself.
    live.stop().await;
    let (status_code, status) =
        wait_for_details(&base_url, "No healthy endpoint available").await?;
    assert_eq!(status_code, StatusCode::OK, "{status:?}");
    assert_eq!(status.status, "healthy");
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
    let endpoint = StubEndpoint::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let addr = free_addr();
    let tls = TestTls::generate(addr);
    let config = TestProxy::config(addr, &tls, &rpc, vec![endpoint.addr.to_string().parse()?]);
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
    let endpoint = StubEndpoint::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    rpc.set_failing(true);
    let addresses = vec![endpoint.addr.to_string().parse()?];
    let t = TestProxy::launch(rpc, addresses, vec![endpoint], |_| {}).await;
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

/// Polls `/healthz` until its details mention `needle`.
async fn wait_for_details(
    base_url: &str,
    needle: &str,
) -> anyhow::Result<(StatusCode, HealthResponse)> {
    let deadline = tokio::time::Instant::now() + 20 * ENDPOINT_HEALTHCHECK_FREQUENCY;
    loop {
        let (status_code, status) = fetch_status(base_url).await?;
        if status.details.contains(needle) {
            return Ok((status_code, status));
        }
        anyhow::ensure!(
            tokio::time::Instant::now() < deadline,
            "`{needle}` not in the details after the deadline: {status:?}"
        );
        tokio::time::sleep(ENDPOINT_HEALTHCHECK_FREQUENCY / 2).await;
    }
}
