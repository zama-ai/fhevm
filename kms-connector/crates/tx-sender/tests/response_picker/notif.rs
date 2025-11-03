use crate::common::insert_rand_response;
use connector_utils::tests::setup::TestInstanceBuilder;
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, KmsResponsePicker};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_public_decryption_with_pg_notif() -> anyhow::Result<()> {
    test_pick_response_with_pg_notif("PublicDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_user_decryption_with_pg_notif() -> anyhow::Result<()> {
    test_pick_response_with_pg_notif("UserDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_prep_keygen_with_pg_notif() -> anyhow::Result<()> {
    test_pick_response_with_pg_notif("PrepKeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_keygen_with_pg_notif() -> anyhow::Result<()> {
    test_pick_response_with_pg_notif("KeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_crsgen_with_pg_notif() -> anyhow::Result<()> {
    test_pick_response_with_pg_notif("CrsgenResponse").await
}

async fn test_pick_response_with_pg_notif(response_str: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut response_picker = init_response_picker(test_instance.db().clone()).await?;

    info!("Triggering Postgres notification with CrsgenResponse insertion...");
    let inserted_response = insert_rand_response(test_instance.db(), response_str, None).await?;
    info!("Picking {response_str}...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking {response_str} data...");
    assert_eq!(responses[0].kind, inserted_response);
    info!("Data OK!");
    Ok(())
}

async fn init_response_picker(db_pool: Pool<Postgres>) -> anyhow::Result<DbKmsResponsePicker> {
    // Use high polling to ensure PG notifications are used in tests
    let mut config = Config::default().await;
    config.database_polling_timeout = Duration::from_secs(120);
    DbKmsResponsePicker::connect(db_pool, &config).await
}
