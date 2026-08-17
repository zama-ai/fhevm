use connector_utils::tests::{
    db::responses::{TestResponseType, insert_rand_response},
    setup::TestInstanceBuilder,
};
use rstest::rstest;
use sqlx::{Pool, Postgres};
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
async fn test_pick_response_with_pg_notif(
    #[case] response_type: TestResponseType,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut response_picker = init_response_picker(test_instance.db().clone()).await?;

    info!("Triggering Postgres notification with {response_type} insertion...");
    let inserted_response =
        insert_rand_response(test_instance.db(), response_type, None, None).await?;
    info!("Picking {response_type}...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking {response_type} data...");
    assert_eq!(responses[0].kind, inserted_response);
    info!("Data OK!");
    Ok(())
}

async fn init_response_picker(db_pool: Pool<Postgres>) -> anyhow::Result<DbKmsResponsePicker> {
    // Use high polling to ensure PG notifications are used in tests
    let config = Config {
        database_polling_timeout: Duration::from_secs(120),
        ..Default::default()
    };
    DbKmsResponsePicker::connect(db_pool, &config).await
}
