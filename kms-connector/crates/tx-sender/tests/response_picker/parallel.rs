use crate::common::insert_rand_response;
use alloy::primitives::U256;
use connector_utils::tests::setup::TestInstanceBuilder;
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tracing::info;
use tx_sender::core::{
    Config, DbKmsResponsePicker, DbKmsResponseRemover, KmsResponsePicker, KmsResponseRemover,
};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_public_decryption_picking() -> anyhow::Result<()> {
    test_parallel_response_picking("PublicDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_user_decryption_picking() -> anyhow::Result<()> {
    test_parallel_response_picking("UserDecryptionResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_prep_keygen_picking() -> anyhow::Result<()> {
    test_parallel_response_picking("PrepKeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_keygen_picking() -> anyhow::Result<()> {
    test_parallel_response_picking("KeygenResponse").await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_parallel_crsgen_picking() -> anyhow::Result<()> {
    test_parallel_response_picking("CrsgenResponse").await
}

async fn test_parallel_response_picking(request_str: &str) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut response_picker = init_response_picker(test_instance.db().clone()).await?;
    let response_remover = DbKmsResponseRemover::new(test_instance.db().clone());

    let insert_response0 =
        insert_rand_response(test_instance.db(), request_str, Some(U256::ZERO)).await?;
    let insert_response1 =
        insert_rand_response(test_instance.db(), request_str, Some(U256::ONE)).await?;

    info!("Picking two {request_str}...");
    let responses0 = response_picker.pick_responses().await?;
    let responses1 = response_picker.pick_responses().await?;

    info!("Checking {request_str} data...");
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

    info!("Data OK! Releasing first {request_str}...");
    for response in responses0 {
        response_remover
            .mark_response_as_pending(response.kind)
            .await;
    }

    info!("Done! Picking first {request_str} again...");
    let responses0 = response_picker.pick_responses().await?;
    info!("Done! Checking data again...");
    assert_eq!(
        responses0
            .iter()
            .map(|e| e.kind.clone())
            .collect::<Vec<_>>(),
        vec![insert_response0]
    );

    info!("Data OK! Releasing and deleting all responses...");
    for response in responses0 {
        response_remover
            .mark_response_as_pending(response.kind.clone())
            .await;
        response_remover.remove_response(&response.kind).await?;
    }
    for response in responses1 {
        response_remover
            .mark_response_as_pending(response.kind.clone())
            .await;
        response_remover.remove_response(&response.kind).await?;
    }
    info!("Done! Checking DB is empty...");
    check_db_empty(test_instance.db(), request_str).await?;
    info!("Done!");
    Ok(())
}

async fn init_response_picker(db: Pool<Postgres>) -> anyhow::Result<DbKmsResponsePicker> {
    let mut config = Config::default().await;
    config.responses_batch_size = 1;
    DbKmsResponsePicker::connect(db, &config).await
}

async fn check_db_empty(db: &Pool<Postgres>, response_str: &str) -> sqlx::Result<()> {
    info!("Checking {response_str} has been removed from DB...");
    let query = match response_str {
        "PublicDecryptionResponse" => {
            "SELECT COUNT(decryption_id) FROM public_decryption_responses"
        }
        "UserDecryptionResponse" => "SELECT COUNT(decryption_id) FROM user_decryption_responses",
        "PrepKeygenResponse" => "SELECT COUNT(prep_keygen_id) FROM prep_keygen_responses",
        "KeygenResponse" => "SELECT COUNT(key_id) FROM keygen_responses",
        "CrsgenResponse" => "SELECT COUNT(crs_id) FROM crsgen_responses",
        s => panic!("Unexpected response kind: {s}"),
    };
    let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
    assert_eq!(count, 0);
    info!("{response_str} successfully removed from DB! Stopping TransactionSender...");
    Ok(())
}
