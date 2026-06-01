use alloy::primitives::U256;
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
async fn test_parallel_response_picking(
    #[case] response_type: TestResponseType,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut response_picker = init_response_picker(test_instance.db().clone()).await?;

    let insert_response0 =
        insert_rand_response(test_instance.db(), response_type, Some(U256::ZERO), None).await?;
    let insert_response1 =
        insert_rand_response(test_instance.db(), response_type, Some(U256::ONE), None).await?;

    info!("Picking two {response_type}...");
    let responses0 = response_picker.pick_responses().await?;
    let responses1 = response_picker.pick_responses().await?;

    info!("Checking {response_type} data...");
    assert_eq!(
        responses0
            .iter()
            .map(|e| e.kind.clone())
            .collect::<Vec<_>>(),
        vec![insert_response0.clone()]
    );
    assert_eq!(
        responses1
            .iter()
            .map(|e| e.kind.clone())
            .collect::<Vec<_>>(),
        vec![insert_response1]
    );

    info!("Data OK! Releasing first {response_type}...");
    for response in responses0 {
        response.mark_as_pending(test_instance.db()).await;
    }

    info!("Done! Picking first {response_type} again...");
    let responses0 = response_picker.pick_responses().await?;
    info!("Done! Checking data again...");
    assert_eq!(
        responses0
            .iter()
            .map(|e| e.kind.clone())
            .collect::<Vec<_>>(),
        vec![insert_response0]
    );

    info!("Data OK! Marking all responses as completed...");
    for response in responses0 {
        response.mark_as_completed(test_instance.db()).await;
    }
    for response in responses1 {
        response.mark_as_completed(test_instance.db()).await;
    }

    info!("Done! Checking there is no uncompleted response in DB...");
    check_no_uncompleted_response_in_db(test_instance.db(), response_type).await?;
    info!("Done!");
    Ok(())
}

async fn init_response_picker(db: Pool<Postgres>) -> anyhow::Result<DbKmsResponsePicker> {
    let config = Config {
        responses_batch_size: 1,
        ..Default::default()
    };
    DbKmsResponsePicker::connect(db, &config).await
}

async fn check_no_uncompleted_response_in_db(
    db: &Pool<Postgres>,
    response_type: TestResponseType,
) -> sqlx::Result<()> {
    info!("Checking {response_type} has been removed from DB...");
    let query = match response_type {
        TestResponseType::PublicDecryption => {
            "SELECT COUNT(decryption_id) FROM public_decryption_responses WHERE status = 'pending'"
        }
        TestResponseType::UserDecryption => {
            "SELECT COUNT(decryption_id) FROM user_decryption_responses WHERE status = 'pending'"
        }
        TestResponseType::PrepKeygen => {
            "SELECT COUNT(prep_keygen_id) FROM prep_keygen_responses WHERE status = 'pending'"
        }
        TestResponseType::Keygen => {
            "SELECT COUNT(key_id) FROM keygen_responses WHERE status = 'pending'"
        }
        TestResponseType::Crsgen => {
            "SELECT COUNT(crs_id) FROM crsgen_responses WHERE status = 'pending'"
        }
        TestResponseType::NewKmsContext => {
            "SELECT COUNT(context_id) FROM new_kms_context_responses WHERE status = 'pending'"
        }
        TestResponseType::EpochResult => {
            "SELECT COUNT(epoch_id) FROM epoch_result_responses WHERE status = 'pending'"
        }
    };
    let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
    assert_eq!(count, 0);
    info!("{response_type} successfully removed from DB! Stopping TransactionSender...");
    Ok(())
}
