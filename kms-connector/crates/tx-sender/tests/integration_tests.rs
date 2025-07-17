mod common;

use crate::common::insert_rand_user_decrypt_response;
use alloy::primitives::U256;
use anyhow::anyhow;
use common::insert_rand_public_decrypt_response;
use connector_utils::{
    tests::setup::{TestInstance, TestInstanceBuilder},
    types::KmsResponse,
};
use rstest::rstest;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tx_sender::core::{DbKmsResponsePicker, DbKmsResponseRemover, TransactionSender};

#[rstest]
#[timeout(Duration::from_secs(10))]
#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_process_public_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking PublicDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_public_decrypt_response(test_instance.db()).await?;
    info!("PublicDecryptionResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let mut response_stream = test_instance
        .decryption_contract()
        .PublicDecryptionResponse_filter()
        .watch()
        .await?
        .into_stream();
    let (response, _) = response_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture PublicDecryptionResponse"))??;
    match inserted_response {
        KmsResponse::PublicDecryption { decryption_id, .. } => {
            assert_eq!(response.decryptionId, decryption_id)
        }
        _ => unreachable!(),
    }
    info!("Response successfully sent to Anvil!");

    info!("Checking response has been removed from DB...");
    tokio::time::sleep(Duration::from_millis(300)).await; // give some time for the removal
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM public_decryption_responses")
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(10))]
#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_process_user_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking UserDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_user_decrypt_response(test_instance.db()).await?;
    info!("UserDecryptionResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let mut response_stream = test_instance
        .decryption_contract()
        .UserDecryptionResponse_filter()
        .watch()
        .await?
        .into_stream();
    let (response, _) = response_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture UserDecryptionResponse"))??;
    match inserted_response {
        KmsResponse::UserDecryption { decryption_id, .. } => {
            assert_eq!(response.decryptionId, decryption_id)
        }
        _ => unreachable!(),
    }
    info!("Response successfully sent to Anvil!");

    info!("Checking response has been removed from DB...");
    tokio::time::sleep(Duration::from_millis(300)).await; // give some time for the removal
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[tokio::test]
#[ignore = "to enable when performance will be improved"]
async fn stress_test() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    let nb_response = 500;
    info!("Mocking {nb_response} UserDecryptionResponse in Postgres...");
    let mut responses_id = Vec::with_capacity(nb_response);
    for _ in 0..nb_response {
        match insert_rand_user_decrypt_response(test_instance.db()).await? {
            KmsResponse::UserDecryption { decryption_id, .. } => {
                responses_id.push(decryption_id);
            }
            _ => unreachable!(),
        }
    }
    info!("{nb_response} UserDecryptionResponse successfully stored!");

    info!("Checking responses has been sent to Anvil...");
    let response_stream = test_instance
        .decryption_contract()
        .UserDecryptionResponse_filter()
        .watch()
        .await?
        .into_stream();

    let mut anvil_responses_id = response_stream
        .map(|res| res.unwrap())
        .map(|(r, _)| r.decryptionId)
        .take(nb_response)
        .collect::<Vec<U256>>()
        .await;
    responses_id.sort();
    anvil_responses_id.sort();
    assert_eq!(responses_id, anvil_responses_id);
    info!("Responses successfully sent to Anvil!");

    info!("Checking responses have been removed from DB...");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(count, 0);
    info!("Responses successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

async fn start_test_tx_sender(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
) -> anyhow::Result<JoinHandle<()>> {
    let response_picker = DbKmsResponsePicker::connect(test_instance.db().clone()).await?;
    let response_remover = DbKmsResponseRemover::new(test_instance.db().clone());

    let tx_sender = TransactionSender::new(
        response_picker,
        test_instance.provider().clone(),
        test_instance.decryption_contract().clone(),
        response_remover,
    );

    Ok(tokio::spawn(tx_sender.start(cancel_token)))
}
