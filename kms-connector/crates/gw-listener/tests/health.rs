use alloy::{
    providers::RootProvider,
    transports::http::reqwest::{self, StatusCode, Url},
};
use connector_utils::{
    monitoring::{
        health::{Healthcheck, query_healthcheck_endpoint},
        server::{GIT_COMMIT_HASH, LivenessResponse, VersionResponse, start_monitoring_server},
    },
    tests::setup::{TestInstanceBuilder, pick_free_port},
};
use gw_listener::monitoring::health::{HealthStatus, State};
use rstest::rstest;
use std::{net::SocketAddr, str::FromStr, time::Duration};
use tokio_util::sync::CancellationToken;

#[rstest]
#[timeout(Duration::from_secs(120))]
#[tokio::test]
async fn test_healthcheck_endpoints() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let state = State::new(
        test_instance.db().clone(),
        test_instance.provider().clone(),
        Duration::from_secs(5),
    );

    let monitoring_endpoint = SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port()))?;
    let monitoring_url = Some(Url::from_str(&format!(
        "http://{}/healthz",
        monitoring_endpoint
    ))?);
    let cancel_token = CancellationToken::new();
    let monitoring_server_task =
        start_monitoring_server(monitoring_endpoint, state, cancel_token.clone());

    // Wait for the start of the monitoring server
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
            name: State::<RootProvider>::service_name().to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            build: GIT_COMMIT_HASH.to_string(),
        }
    );

    // Test the `healthz` endpoint while everything is fine
    query_healthcheck_endpoint::<HealthStatus>(monitoring_url.clone()).await?;

    // Pause DB and verify healthcheck failure
    test_instance.db_container().pause().await?;
    query_healthcheck_endpoint::<HealthStatus>(monitoring_url.clone())
        .await
        .unwrap_err();
    test_instance.db_container().unpause().await?;

    // Test everything is fine
    query_healthcheck_endpoint::<HealthStatus>(monitoring_url.clone()).await?;

    // Pause Gateway and verify healthcheck failure
    test_instance.anvil_container().pause().await?;
    query_healthcheck_endpoint::<HealthStatus>(monitoring_url)
        .await
        .unwrap_err();

    cancel_token.cancel();
    monitoring_server_task.await?;
    Ok(())
}
