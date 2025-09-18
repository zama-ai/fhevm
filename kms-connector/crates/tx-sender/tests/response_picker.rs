mod common;

use common::{
    insert_rand_crsgen_response, insert_rand_keygen_response, insert_rand_prep_keygen_response,
    insert_rand_public_decrypt_response, insert_rand_user_decrypt_response,
};
use connector_utils::{tests::setup::TestInstanceBuilder, types::KmsResponse};
use rstest::rstest;
use std::time::Duration;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, KmsResponsePicker};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;

    info!("Triggering Postgres notification with PublicDecryptionResponse insertion...");
    let inserted_response = insert_rand_public_decrypt_response(test_instance.db()).await?;

    info!("Picking PublicDecryptionResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking PublicDecryptionResponse data...");
    assert_eq!(
        responses[0],
        KmsResponse::PublicDecryption(inserted_response)
    );
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;

    info!("Triggering Postgres notification with UserDecryptionResponse insertion...");
    let inserted_response = insert_rand_user_decrypt_response(test_instance.db()).await?;
    info!("Picking UserDecryptionResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking UserDecryptionResponse data...");
    assert_eq!(responses[0], KmsResponse::UserDecryption(inserted_response));
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_prep_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;

    info!("Triggering Postgres notification with PrepKeygenResponse insertion...");
    let inserted_response = insert_rand_prep_keygen_response(test_instance.db()).await?;
    info!("Picking PrepKeygenResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking PrepKeygenResponse data...");
    assert_eq!(responses[0], KmsResponse::PrepKeygen(inserted_response));
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;

    info!("Triggering Postgres notification with KeygenResponse insertion...");
    let inserted_response = insert_rand_keygen_response(test_instance.db()).await?;
    info!("Picking KeygenResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking KeygenResponse data...");
    assert_eq!(responses[0], KmsResponse::Keygen(inserted_response));
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_crsgen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;

    info!("Triggering Postgres notification with CrsgenResponse insertion...");
    let inserted_response = insert_rand_crsgen_response(test_instance.db()).await?;
    info!("Picking CrsgenResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking CrsgenResponse data...");
    assert_eq!(responses[0], KmsResponse::Crsgen(inserted_response));
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_polling_backup() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    info!("Inserting UserDecryptionResponse before starting the picker...");
    let inserted_response = insert_rand_user_decrypt_response(test_instance.db()).await?;

    let mut config = Config::default().await;
    config.database_polling_timeout = Duration::from_millis(500);
    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &config).await?;
    info!("Picking UserDecryptionResponse...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking UserDecryptionResponse data...");
    assert_eq!(responses[0], KmsResponse::UserDecryption(inserted_response));
    info!("Data OK!");
    Ok(())
}
