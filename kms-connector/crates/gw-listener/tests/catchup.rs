mod common;

use crate::common::{mock_event_on_gw, poll_db_for_event, start_test_listener};
use connector_utils::tests::{db::requests::TestEventType, setup::TestInstanceBuilder};
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption)]
#[case::user_decryption(TestEventType::UserDecryption)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2)]
#[case::prep_keygen(TestEventType::PrepKeygen)]
#[case::keygen(TestEventType::Keygen)]
#[case::crsgen(TestEventType::Crsgen)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup_from_block(#[case] event_type: TestEventType) -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();

    // Wait for two more anvil blocks so anvil is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let (event1, block1) = mock_event_on_gw(&test_instance, event_type).await?;
    assert!(block1.is_some());
    let (event2, block2) = mock_event_on_gw(&test_instance, event_type).await?;
    assert_ne!(block1, block2);

    // Start the listener after the transactions are sent.
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), block1).await;

    poll_db_for_event(test_instance.db(), event_type, &event1).await?;
    poll_db_for_event(test_instance.db(), event_type, &event2).await?;
    info!("Events successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}
