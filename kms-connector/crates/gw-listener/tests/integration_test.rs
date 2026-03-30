mod common;

use crate::common::{mock_event_on_gw, poll_db_for_event, start_test_listener};
use connector_utils::tests::setup::TestInstanceBuilder;
use connector_utils::types::db::EventType;
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[case::public_decryption(EventType::PublicDecryptionRequest)]
#[case::user_decryption(EventType::UserDecryptionRequest)]
#[case::prep_keygen(EventType::PrepKeygenRequest)]
#[case::keygen(EventType::KeygenRequest)]
#[case::crsgen(EventType::CrsgenRequest)]
#[case::prss_init(EventType::PrssInit)]
#[case::key_reshare_same_set(EventType::KeyReshareSameSet)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_event(#[case] event_type: EventType) -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    let (expected_event, _) = mock_event_on_gw(&test_instance, event_type).await?;
    poll_db_for_event(test_instance.db(), event_type, &expected_event).await?;
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}
