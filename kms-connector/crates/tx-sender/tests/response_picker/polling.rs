use connector_utils::tests::{
    db::responses::{TestResponseType, insert_rand_response},
    setup::TestInstanceBuilder,
};
use rstest::rstest;
use std::time::Duration;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, KmsResponsePicker};

#[rstest]
#[case::public_decryption(TestResponseType::PublicDecryption)]
#[case::user_decryption(TestResponseType::UserDecryption)]
#[case::prep_keygen(TestResponseType::PrepKeygen)]
#[case::keygen(TestResponseType::Keygen)]
#[case::crsgen(TestResponseType::Crsgen)]
#[case::new_kms_context(TestResponseType::NewKmsContext)]
#[case::epoch_result(TestResponseType::EpochResult)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_response_with_polling_backup(
    #[case] response_type: TestResponseType,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    info!("Inserting {response_type} before starting the picker...");
    let inserted_response =
        insert_rand_response(test_instance.db(), response_type, None, None).await?;

    let config = Config {
        database_polling_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let mut response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &config).await?;

    info!("Picking {response_type}...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking {response_type} data...");
    assert_eq!(responses[0].kind, inserted_response);
    info!("Data OK!");
    Ok(())
}
