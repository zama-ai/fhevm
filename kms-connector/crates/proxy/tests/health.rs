mod common;

use alloy::transports::http::reqwest::{self, StatusCode, Url};
use common::{
    API_KEY, ENDPOINT_HEALTHCHECK_FREQUENCY, StubEndpoint, TestProxy, TestTls, free_addr,
    wait_for_listener,
};
use connector_utils::monitoring::{
    health::{Healthcheck, query_healthcheck_endpoint},
    server::{GIT_COMMIT_HASH, LivenessResponse, VersionResponse, start_monitoring_server},
};
use kms_connector_api::{ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE};
use proxy::{
    core::Proxy,
    monitoring::health::{HealthStatus, State},
};
use rstest::rstest;
use std::{str::FromStr, time::Duration};
use tokio_util::sync::CancellationToken;

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_healthcheck_endpoint() -> anyhow::Result<()> {
    // Start the `Proxy` with two endpoints, and its monitoring server
    let endpoints = vec![StubEndpoint::start().await, StubEndpoint::start().await];
    let t = TestProxy::start(endpoints).await;

    let monitoring_endpoint = free_addr();
    let monitoring_url = Url::from_str(&format!("http://{}/healthz", monitoring_endpoint))?;
    let cancel_token = CancellationToken::new();
    let monitoring_server_task =
        start_monitoring_server(monitoring_endpoint, t.state.clone(), cancel_token.clone())?;
    wait_for_listener(monitoring_endpoint).await;

    // Test `liveness` endpoint
    let url = format!("http://{}/liveness", monitoring_endpoint);
    let response = reqwest::get(&url).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<LivenessResponse>().await?,
        LivenessResponse {
            status_code: "200".to_string(),
            status: "alive".to_string(),
        }
    );

    // Test `version` endpoint
    let url = format!("http://{}/version", monitoring_endpoint);
    let response = reqwest::get(&url).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await?,
        VersionResponse {
            name: State::service_name().to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            build: GIT_COMMIT_HASH.to_string(),
        }
    );

    // Test the endpoint while everything is fine
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone())).await?;
    let status = fetch_status(&monitoring_url).await?;
    assert!(status.healthy);
    assert!(status.tls_listener_reachable);
    let mut expected: Vec<_> = t.endpoints.iter().map(|e| e.addr.to_string()).collect();
    expected.sort();
    let mut healthy_endpoints = status.healthy_endpoints;
    healthy_endpoints.sort();
    assert_eq!(healthy_endpoints, expected);
    assert!(status.unhealthy_endpoints.is_empty());
    assert!(status.details.is_empty());

    // Stop one endpoint: degraded but healthy, the other one is still able to serve
    let (stopped, live) = (&t.endpoints[0], &t.endpoints[1]);
    stopped.stop().await;
    let status = wait_for_unhealthy_endpoints(&monitoring_url, 1).await?;
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone())).await?;
    assert!(status.healthy);
    assert!(status.tls_listener_reachable);
    assert_eq!(status.healthy_endpoints, [live.addr.to_string()]);
    assert_eq!(status.unhealthy_endpoints, [stopped.addr.to_string()]);
    assert!(status.details.contains(&stopped.addr.to_string()));

    // The proxy stops selecting the unhealthy endpoint
    for _ in 0..10 {
        let response = t
            .client
            .post(t.url(PUBLIC_DECRYPTION_ROUTE))
            .bearer_auth(API_KEY)
            .body("{}")
            .send()
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
    }
    assert_eq!(live.hits(), 10);
    assert_eq!(stopped.hits(), 0);

    // Stop the last endpoint and verify healthcheck failure
    live.stop().await;
    let status = wait_for_unhealthy_endpoints(&monitoring_url, 2).await?;
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone()))
        .await
        .unwrap_err();
    assert!(!status.healthy);
    assert!(status.tls_listener_reachable);
    assert!(status.healthy_endpoints.is_empty());
    assert_eq!(status.unhealthy_endpoints.len(), 2);
    assert!(status.details.contains("No healthy endpoint available"));

    // The proxy has no endpoint left to select
    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body("{}")
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let error: ErrorResponse = response.json().await?;
    assert_eq!(error.code, ErrorCode::UpstreamTransient);
    assert!(error.retryable);

    cancel_token.cancel();
    monitoring_server_task.await?;
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_healthcheck_unreachable_tls_listener() -> anyhow::Result<()> {
    // Build the `Proxy` without starting it: its TLS listener is down. Pingora's server cannot
    // be stopped once started, so this case cannot be part of the test above.
    let endpoint = StubEndpoint::start().await;
    let addr = free_addr();
    let tls = TestTls::generate(addr);
    let config = TestProxy::config(addr, &tls, vec![endpoint.addr.to_string().parse()?]);
    let (_proxy, state) = Proxy::from_config(config)?;

    let monitoring_endpoint = free_addr();
    let monitoring_url = Url::from_str(&format!("http://{}/healthz", monitoring_endpoint))?;
    let cancel_token = CancellationToken::new();
    let monitoring_server_task =
        start_monitoring_server(monitoring_endpoint, state, cancel_token.clone())?;
    wait_for_listener(monitoring_endpoint).await;

    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone()))
        .await
        .unwrap_err();
    let status = fetch_status(&monitoring_url).await?;
    assert!(!status.healthy);
    assert!(!status.tls_listener_reachable);
    assert_eq!(status.healthy_endpoints, [endpoint.addr.to_string()]);
    assert!(status.details.contains("TLS listener"));

    cancel_token.cancel();
    monitoring_server_task.await?;
    Ok(())
}

async fn fetch_status(url: &Url) -> anyhow::Result<HealthStatus> {
    Ok(reqwest::get(url.clone()).await?.json().await?)
}

/// Polls `/healthz` until the background probes have evicted `count` endpoints.
async fn wait_for_unhealthy_endpoints(url: &Url, count: usize) -> anyhow::Result<HealthStatus> {
    let deadline = tokio::time::Instant::now() + 20 * ENDPOINT_HEALTHCHECK_FREQUENCY;
    loop {
        let status = fetch_status(url).await?;
        if status.unhealthy_endpoints.len() == count {
            return Ok(status);
        }
        anyhow::ensure!(
            tokio::time::Instant::now() < deadline,
            "only {} endpoint(s) evicted after the deadline, expected {count}",
            status.unhealthy_endpoints.len()
        );
        tokio::time::sleep(ENDPOINT_HEALTHCHECK_FREQUENCY / 2).await;
    }
}
