mod common;

use crate::common::{mock_event_on_gw, poll_db_for_event, start_test_listener};
use alloy::primitives::U256;
use connector_utils::{
    tests::{db::requests::TestEventType, setup::TestInstanceBuilder},
    types::KMS_CONTEXT_COUNTER_BASE,
};
use gw_listener::core::publish_context_and_epoch;
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption)]
#[case::user_decryption(TestEventType::UserDecryption)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2)]
#[case::prep_keygen(TestEventType::PrepKeygen)]
#[case::keygen(TestEventType::Keygen)]
#[case::compressed_key_migration(TestEventType::CompressedKeyMigrationKeygen)]
#[case::crsgen(TestEventType::Crsgen)]
#[case::abort_keygen(TestEventType::AbortKeygen)]
#[case::abort_crsgen(TestEventType::AbortCrsgen)]
#[case::new_kms_context(TestEventType::NewKmsContext)]
#[case::new_kms_epoch(TestEventType::NewKmsEpoch)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_event(#[case] event_type: TestEventType) -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_bc_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    let (expected_event, _) = mock_event_on_gw(&test_instance, event_type).await?;
    poll_db_for_event(test_instance.db(), event_type, &expected_event).await?;
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

/// `KmsContextDestroyed` stores no event for the kms-worker: the listener must instead mark all
/// the epochs of the destroyed context as invalid in the `kms_context` table, leaving other
/// contexts untouched.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_kms_context_destroyed_invalidates_context_epochs() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_bc_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    // Two epochs of the context to destroy, plus one epoch of an untouched context
    let destroyed_context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(7);
    let other_context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(8);
    for (context_id, epoch_id) in [
        (destroyed_context_id, U256::from(1)),
        (destroyed_context_id, U256::from(2)),
        (other_context_id, U256::from(3)),
    ] {
        publish_context_and_epoch(test_instance.db(), context_id, epoch_id).await?;
    }

    info!("Destroying KMS context #{destroyed_context_id} on Anvil...");
    test_instance
        .protocol_config_contract()
        .destroyKmsContext(destroyed_context_id)
        .send()
        .await?
        .get_receipt()
        .await?;

    poll_db_for_invalid_context(test_instance.db(), destroyed_context_id).await?;
    assert_eq!(
        fetch_valid_flags(test_instance.db(), other_context_id).await?,
        vec![true],
        "the untouched context should remain valid"
    );
    assert_eq!(
        fetch_valid_flags(test_instance.db(), destroyed_context_id).await?,
        vec![false, false]
    );
    info!("Context successfully invalidated! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

async fn fetch_valid_flags(db: &Pool<Postgres>, context_id: U256) -> anyhow::Result<Vec<bool>> {
    sqlx::query_scalar("SELECT is_valid FROM kms_context WHERE id = $1")
        .bind(context_id.as_le_slice())
        .fetch_all(db)
        .await
        .map_err(anyhow::Error::from)
}

/// Polls the DB until all the epochs of the given context have been marked as invalid.
async fn poll_db_for_invalid_context(db: &Pool<Postgres>, context_id: U256) -> anyhow::Result<()> {
    let timeout = Duration::from_secs(30);
    let poll_interval = Duration::from_millis(200);
    let start = std::time::Instant::now();
    loop {
        let valid_flags = fetch_valid_flags(db, context_id).await?;
        if !valid_flags.is_empty() && valid_flags.iter().all(|valid| !valid) {
            info!(
                "All {} epoch(s) of context #{context_id} invalidated in DB!",
                valid_flags.len()
            );
            return Ok(());
        }
        if start.elapsed() > timeout {
            anyhow::bail!("Timed out waiting for context #{context_id} invalidation in DB");
        }
        tokio::time::sleep(poll_interval).await;
    }
}
