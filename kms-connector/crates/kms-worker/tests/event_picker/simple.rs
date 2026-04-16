use connector_utils::{
    tests::{
        db::requests::{InsertRequestOptions, insert_rand_request},
        setup::TestInstanceBuilder,
    },
    types::db::EventType,
};
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use rstest::rstest;
use std::time::Duration;
use tracing::info;

#[rstest]
#[case::public_decryption(EventType::PublicDecryptionRequest)]
#[case::user_decryption(EventType::UserDecryptionRequest)]
#[case::prep_keygen(EventType::PrepKeygenRequest)]
#[case::keygen(EventType::KeygenRequest)]
#[case::crsgen(EventType::CrsgenRequest)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_request(#[case] event_type: EventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    info!("Triggering Postgres notification with {event_type} insertion...");
    let inserted_request = insert_rand_request(
        test_instance.db(),
        event_type,
        InsertRequestOptions::default(),
    )
    .await?;

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
