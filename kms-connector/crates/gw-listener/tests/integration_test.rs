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
#[case::crsgen(TestEventType::Crsgen)]
#[case::abort_keygen(TestEventType::AbortKeygen)]
#[case::abort_crsgen(TestEventType::AbortCrsgen)]
#[case::new_kms_context(TestEventType::NewKmsContext)]
#[case::new_kms_epoch(TestEventType::NewKmsEpoch)]
#[case::kms_context_destroyed(TestEventType::KmsContextDestroyed)]
#[case::kms_epoch_destroyed(TestEventType::KmsEpochDestroyed)]
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

/// Besides storing the event for the kms-worker (covered by `test_publish_event`),
/// `KmsContextDestroyed` must invalidate the destroyed context in the `kms_context` validation
/// cache — even when the context was never cached — leaving other contexts untouched.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_kms_context_destroyed_invalidates_cache() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_bc_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    // One cached context to destroy, one cached context to keep, and one context to destroy
    // without caching it first.
    let destroyed_context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(7);
    let other_context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(8);
    let uncached_context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(9);
    publish_context_and_epoch(test_instance.db(), destroyed_context_id, U256::from(1)).await?;
    publish_context_and_epoch(test_instance.db(), other_context_id, U256::from(2)).await?;

    for context_id in [destroyed_context_id, uncached_context_id] {
        info!("Destroying KMS context #{context_id} on Anvil...");
        test_instance
            .protocol_config_contract()
            .destroyKmsContext(context_id)
            .send()
            .await?
            .get_receipt()
            .await?;
    }

    poll_db_until_invalid(test_instance.db(), "kms_context", destroyed_context_id).await?;
    poll_db_until_invalid(test_instance.db(), "kms_context", uncached_context_id).await?;
    assert_eq!(
        fetch_valid_flag(test_instance.db(), "kms_context", other_context_id).await?,
        Some(true),
        "the untouched context should remain valid"
    );
    info!("Contexts successfully invalidated! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

/// Besides storing the event for the kms-worker (covered by `test_publish_event`),
/// `KmsEpochDestroyed` must invalidate the destroyed epoch in the `kms_epoch` validation cache —
/// even when the epoch was never cached — without touching other epochs of the same context.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_kms_epoch_destroyed_invalidates_cache() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_bc_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task =
        start_test_listener(&mut test_instance, cancel_token.clone(), None).await;

    // Two cached epochs of the same context (one to destroy, one to keep), plus an epoch to
    // destroy without caching it first.
    let context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(10);
    let destroyed_epoch_id = U256::from(4);
    let kept_epoch_id = U256::from(5);
    let uncached_epoch_id = U256::from(6);
    for epoch_id in [destroyed_epoch_id, kept_epoch_id] {
        publish_context_and_epoch(test_instance.db(), context_id, epoch_id).await?;
    }

    for epoch_id in [destroyed_epoch_id, uncached_epoch_id] {
        info!("Destroying KMS epoch #{epoch_id} on Anvil...");
        test_instance
            .protocol_config_contract()
            .destroyKmsEpoch(epoch_id)
            .send()
            .await?
            .get_receipt()
            .await?;
    }

    poll_db_until_invalid(test_instance.db(), "kms_epoch", destroyed_epoch_id).await?;
    poll_db_until_invalid(test_instance.db(), "kms_epoch", uncached_epoch_id).await?;
    assert_eq!(
        fetch_valid_flag(test_instance.db(), "kms_epoch", kept_epoch_id).await?,
        Some(true),
        "the other epoch of the context should remain cached as valid"
    );
    assert_eq!(
        fetch_valid_flag(test_instance.db(), "kms_context", context_id).await?,
        Some(true),
        "the context of the destroyed epochs should remain valid"
    );
    info!("Epochs successfully invalidated! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

/// Fetches the `is_valid` flag of the given `kms_context`/`kms_epoch` row, if cached.
async fn fetch_valid_flag(
    db: &Pool<Postgres>,
    table: &str,
    id: U256,
) -> anyhow::Result<Option<bool>> {
    sqlx::query_scalar(&format!("SELECT is_valid FROM {table} WHERE id = $1"))
        .bind(id.as_le_slice())
        .fetch_optional(db)
        .await
        .map_err(anyhow::Error::from)
}

/// Polls the DB until the given `kms_context`/`kms_epoch` row has been marked as invalid.
async fn poll_db_until_invalid(db: &Pool<Postgres>, table: &str, id: U256) -> anyhow::Result<()> {
    let timeout = Duration::from_secs(30);
    let poll_interval = Duration::from_millis(200);
    let start = std::time::Instant::now();
    loop {
        if fetch_valid_flag(db, table, id).await? == Some(false) {
            info!("#{id} invalidated in the {table} cache!");
            return Ok(());
        }
        if start.elapsed() > timeout {
            anyhow::bail!("Timed out waiting for #{id} invalidation in the {table} cache");
        }
        tokio::time::sleep(poll_interval).await;
    }
}
