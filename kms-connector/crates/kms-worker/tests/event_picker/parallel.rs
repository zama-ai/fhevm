use alloy::primitives::U256;
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, check_no_uncompleted_request_in_db, insert_rand_request,
        },
        setup::TestInstanceBuilder,
    },
    types::db::EventType,
};
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_public_decryption_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::PublicDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_user_decryption_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::UserDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_prep_keygen_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::PrepKeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_keygen_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::KeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_crsgen_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::CrsgenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
#[ignore = "Not possible to have parallel PRSS Init the only ID currently allowed is 1"]
async fn test_parallel_prss_init_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::PrssInit).await
}

#[tokio::test]
async fn test_parallel_key_reshare_same_set_picking() -> anyhow::Result<()> {
    test_parallel_request_picking(EventType::KeyReshareSameSet).await
}

async fn test_parallel_request_picking(event_type: EventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut event_picker = init_event_picker(test_instance.db().clone()).await?;

    let insert_request0 = insert_rand_request(
        test_instance.db(),
        event_type,
        InsertRequestOptions::new().with_id(U256::ZERO),
    )
    .await?;
    let insert_request1 = insert_rand_request(
        test_instance.db(),
        event_type,
        InsertRequestOptions::new().with_id(U256::ONE),
    )
    .await?;

    info!("Picking two {event_type}...");
    let events0 = event_picker.pick_events().await?;
    let events1 = event_picker.pick_events().await?;

    info!("Checking {event_type} data...");
    assert_eq!(
        events0.iter().map(|e| e.kind.clone()).collect::<Vec<_>>(),
        vec![insert_request0.clone()]
    );
    assert_eq!(
        events1.iter().map(|e| e.kind.clone()).collect::<Vec<_>>(),
        vec![insert_request1]
    );

    info!("Data OK! Releasing first {event_type}...");
    for event in events0 {
        event.mark_as_pending(test_instance.db()).await;
    }

    info!("Done! Picking first {event_type} again...");
    let events0 = event_picker.pick_events().await?;
    info!("Done! Checking data again...");
    assert_eq!(
        events0.iter().map(|e| e.kind.clone()).collect::<Vec<_>>(),
        vec![insert_request0]
    );

    info!("Data OK! Marking all events as completed...");
    for event in events0 {
        event.mark_as_completed(test_instance.db()).await;
    }
    for event in events1 {
        event.mark_as_completed(test_instance.db()).await;
    }
    info!("Done! Checking there is no uncompleted request in DB...");
    check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;
    info!("Done!");
    Ok(())
}

async fn init_event_picker(db: Pool<Postgres>) -> anyhow::Result<DbEventPicker> {
    let config = Config {
        events_batch_size: 1,
        ..Default::default()
    };
    DbEventPicker::connect(db, &config).await
}
