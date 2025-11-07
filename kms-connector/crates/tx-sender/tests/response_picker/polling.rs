use crate::common::insert_rand_response;
use connector_utils::tests::setup::TestInstanceBuilder;
use rstest::rstest;
use std::time::Duration;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, KmsResponsePicker};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_public_decryption_with_polling_backup() -> anyhow::Result<()> {
    test_pick_response_with_polling_backup("PublicDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_user_decryption_with_polling_backup() -> anyhow::Result<()> {
    test_pick_response_with_polling_backup("UserDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_prep_keygen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_response_with_polling_backup("PrepKeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_keygen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_response_with_polling_backup("KeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_crsgen_with_polling_backup() -> anyhow::Result<()> {
    test_pick_response_with_polling_backup("CrsgenResponse").await
}

async fn test_pick_response_with_polling_backup(response_str: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    info!("Inserting {response_str} before starting the picker...");
    let inserted_response = insert_rand_response(test_instance.db(), response_str, None).await?;

    let mut config = Config::default().await;
    config.database_polling_timeout = Duration::from_millis(500);
    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &config).await?;

    info!("Picking {response_str}...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking {response_str} data...");
    assert_eq!(responses[0].kind, inserted_response);
    info!("Data OK!");
    Ok(())
}
