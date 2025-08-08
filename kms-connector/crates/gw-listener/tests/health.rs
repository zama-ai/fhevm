use connector_utils::{
    monitoring::{health::query_healthcheck_endpoint, server::start_monitoring_server},
    tests::setup::{TestInstanceBuilder, pick_free_port},
};
use gw_listener::monitoring::health::{HealthStatus, State};
use rstest::rstest;
use std::{net::SocketAddr, str::FromStr, time::Duration};
use tokio_util::sync::CancellationToken;

#[rstest]
#[timeout(Duration::from_secs(120))]
#[tokio::test]
async fn test_healthcheck_endpoint() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let state = State::new(
        test_instance.db().clone(),
        test_instance.provider().clone(),
        Duration::from_secs(5),
    );

    let monitoring_endpoint = SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port()))?;
    let cancel_token = CancellationToken::new();
    let monitoring_server_task =
        start_monitoring_server(monitoring_endpoint, state, cancel_token.clone());

    // Wait for the start of the monitoring server
    test_instance
        .wait_for_log("Monitoring server listening at")
        .await;

    // Test the endpoint while everything is fine
    query_healthcheck_endpoint::<HealthStatus>(monitoring_endpoint).await?;

    // Pause DB and verify healthcheck failure
    test_instance.db_container().pause().await?;
    query_healthcheck_endpoint::<HealthStatus>(monitoring_endpoint)
        .await
        .unwrap_err();
    test_instance.db_container().unpause().await?;

    // Test everything is fine
    query_healthcheck_endpoint::<HealthStatus>(monitoring_endpoint).await?;

    // Pause Gateway and verify healthcheck failure
    test_instance.anvil_container().pause().await?;
    query_healthcheck_endpoint::<HealthStatus>(monitoring_endpoint)
        .await
        .unwrap_err();

    cancel_token.cancel();
    monitoring_server_task.await?;
    Ok(())
}
