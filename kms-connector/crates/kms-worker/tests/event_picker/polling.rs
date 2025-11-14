use crate::common::insert_rand_request;
use connector_utils::{tests::setup::TestInstanceBuilder, types::db::EventType};
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_public_decryption_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::PublicDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_user_decryption_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::UserDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_prep_keygen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::PrepKeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_keygen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::KeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_crsgen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::CrsgenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_prss_init_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::PrssInit).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_key_reshare_same_set_with_polling_backup() -> anyhow::Result<()> {
    test_pick_request_with_polling_backup(EventType::KeyReshareSameSet).await
}

async fn test_pick_request_with_polling_backup(event_type: EventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    info!("Inserting {event_type} before starting the event picker...");
    let inserted_request = insert_rand_request(test_instance.db(), event_type, None, false).await?;

    let mut event_picker = init_event_picker(test_instance.db().clone()).await?;
    info!("Picking {event_type}...");
    let events = event_picker.pick_events().await?;

    info!("Checking {event_type} data...");
    assert_eq!(
        events.into_iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![inserted_request],
    );
    info!("Data OK!");
    Ok(())
}

async fn init_event_picker(db_pool: Pool<Postgres>) -> anyhow::Result<DbEventPicker> {
    let config = Config {
        db_fast_event_polling: Duration::from_millis(500),
        db_long_event_polling: Duration::from_millis(500),
        ..Default::default()
    };
    DbEventPicker::connect(db_pool, &config).await
}
