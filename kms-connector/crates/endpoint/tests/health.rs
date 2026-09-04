use alloy::transports::http::reqwest::{self, StatusCode, Url};
use connector_utils::{
    monitoring::{
        health::{Healthcheck, query_healthcheck_endpoint},
        server::{GIT_COMMIT_HASH, LivenessResponse, VersionResponse, start_monitoring_server},
    },
    tests::setup::{DbInstance, TestInstanceBuilder, pick_free_port},
};
use endpoint::{
    core::{Config, Endpoint},
    monitoring::health::{HealthStatus, State},
};
use rstest::rstest;
use std::{net::SocketAddr, str::FromStr, time::Duration};
use tokio_util::sync::CancellationToken;

#[rstest]
#[timeout(Duration::from_secs(300))]
#[tokio::test]
async fn test_healthcheck_endpoint() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup_container().await?)
        .build();
    let config = Config {
        database_url: test_instance.db_url().to_string(),
        database_pool_size: 3,
        http_endpoint: SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port()))?,
        supported_chain_ids: vec![31337],
        healthcheck_timeout: Duration::from_secs(5),
        ..Config::default()
    };

    // Start the `Endpoint` and its monitoring server
    let (endpoint, state) = Endpoint::from_config(config).await?;
    let endpoint_cancel_token = CancellationToken::new();
    let endpoint_task = tokio::spawn(endpoint.start(endpoint_cancel_token.clone()));
    test_instance.wait_for_log("HTTP server listening at").await;

    let monitoring_endpoint = SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port()))?;
    let monitoring_url = Url::from_str(&format!("http://{}/healthz", monitoring_endpoint))?;
    let cancel_token = CancellationToken::new();
    let monitoring_server_task =
        start_monitoring_server(monitoring_endpoint, state, cancel_token.clone());
    test_instance
        .wait_for_log("Monitoring server listening at")
        .await;

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
    assert!(status.database_connected);
    assert!(status.http_server_reachable);
    assert_eq!(status.in_flight_decryptions, 0);

    // Pause DB and verify healthcheck failure
    test_instance.db_container().pause().await?;
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone()))
        .await
        .unwrap_err();
    let status = fetch_status(&monitoring_url).await?;
    assert!(!status.healthy);
    assert!(!status.database_connected);
    assert!(status.http_server_reachable);
    test_instance.db_container().unpause().await?;

    // Test everything is fine
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone())).await?;

    // Stop the public HTTP server and verify healthcheck failure
    endpoint_cancel_token.cancel();
    endpoint_task.await??;
    query_healthcheck_endpoint::<HealthStatus>(Some(monitoring_url.clone()))
        .await
        .unwrap_err();
    let status = fetch_status(&monitoring_url).await?;
    assert!(!status.healthy);
    assert!(status.database_connected);
    assert!(!status.http_server_reachable);

    cancel_token.cancel();
    monitoring_server_task.await?;
    Ok(())
}

async fn fetch_status(url: &Url) -> anyhow::Result<HealthStatus> {
    Ok(reqwest::get(url.clone()).await?.json().await?)
}
