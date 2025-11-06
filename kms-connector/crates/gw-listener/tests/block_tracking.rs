mod common;

use crate::common::{check_event_in_db, fetch_from_db, mock_event_on_gw, start_test_listener};
use connector_utils::{tests::setup::TestInstanceBuilder, types::db::EventType};
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_public_decryption() -> anyhow::Result<()> {
    test_block_tracking(EventType::PublicDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_user_decryption() -> anyhow::Result<()> {
    test_block_tracking(EventType::UserDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_prep_keygen() -> anyhow::Result<()> {
    test_block_tracking(EventType::PrepKeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_keygen() -> anyhow::Result<()> {
    test_block_tracking(EventType::KeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_crsgen() -> anyhow::Result<()> {
    test_block_tracking(EventType::CrsgenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
#[ignore = "
    As there is currently only one PRSS init ID allowed,
    the test won't pass as there will be only one row in the DB instead of two
"]
async fn test_block_tracking_prss_init() -> anyhow::Result<()> {
    test_block_tracking(EventType::PrssInit).await
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[tokio::test]
async fn test_block_tracking_key_reshare_same_set() -> anyhow::Result<()> {
    test_block_tracking(EventType::KeyReshareSameSet).await
}

async fn test_block_tracking(event_type: EventType) -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    let (expected_event, _) = mock_event_on_gw(&test_instance, event_type).await?;
    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

    let rows = fetch_from_db(test_instance.db(), event_type).await?;
    check_event_in_db(&rows, expected_event)?;
    info!("Event successfully stored! Stopping GatewayListener...");
    cancel_token.cancel();
    gw_listener_task?.await?;

    info!(
        "GatewayListener is stopped! Mocking another {event_type} on Anvil while the listener is down..."
    );
    let (expected_event, _) = mock_event_on_gw(&test_instance, event_type).await?;

    info!("Restarting the listener to catch up missed {event_type}...");
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

    let rows = fetch_from_db(test_instance.db(), event_type).await?;
    check_event_in_db(&rows, expected_event)?;
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}
