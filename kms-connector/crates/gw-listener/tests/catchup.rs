mod common;

use crate::common::{mock_event_on_gw, poll_db_for_event, start_test_listener};
use connector_utils::{tests::setup::TestInstanceBuilder, types::db::EventType};
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_public_decryption_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::PublicDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_user_decryption_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::UserDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_prep_keygen_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::PrepKeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_keygen_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::KeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_crsgen_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::CrsgenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_prss_init_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::PrssInit).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_key_reshare_same_set_from_block() -> anyhow::Result<()> {
    test_catchup_from_block(EventType::KeyReshareSameSet).await
}

async fn test_catchup_from_block(event_type: EventType) -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();

    // Wait for two more anvil blocks so anvil is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let (event1, block1) = mock_event_on_gw(&test_instance, event_type).await?;
    assert!(block1.is_some());
    let event2 = if !matches!(event_type, EventType::PrssInit) {
        let (event2, block2) = mock_event_on_gw(&test_instance, event_type).await?;
        assert_ne!(block1, block2);
        Some(event2)
    } else {
        None
    };

    // Start the listener after the transactions are sent.
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), block1).await;

    poll_db_for_event(test_instance.db(), event_type, &event1).await?;
    if let Some(ref event2) = event2 {
        poll_db_for_event(test_instance.db(), event_type, event2).await?;
    }
    info!("Events successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}
