mod common;

use alloy::primitives::U256;
use anyhow::anyhow;
use common::insert_rand_public_decrypt_response;
use connector_tests::setup::{TestInstance, test_instance_with_db_and_gw};
use connector_utils::types::KmsResponse;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tx_sender::core::{DbKmsResponsePicker, DbKmsResponseRemover, TransactionSender};

use crate::common::insert_rand_user_decrypt_response;

#[tokio::test]
async fn test_process_public_decryption_response() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_and_gw().await?;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    println!("Mocking PublicDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_public_decrypt_response(&test_instance.db).await?;
    println!("PublicDecryptionResponse successfully stored!");

    println!("Checking response has been sent to Anvil...");
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
    println!("Response successfully sent to Anvil!");

    println!("Checking response has been removed from DB...");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM public_decryption_responses")
            .fetch_one(&test_instance.db)
            .await?;
    assert_eq!(count, 0);
    println!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[tokio::test]
async fn test_process_user_decryption_response() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_and_gw().await?;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    println!("Mocking UserDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_user_decrypt_response(&test_instance.db).await?;
    println!("UserDecryptionResponse successfully stored!");

    println!("Checking response has been sent to Anvil...");
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
    println!("Response successfully sent to Anvil!");

    println!("Checking response has been removed from DB...");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(&test_instance.db)
            .await?;
    assert_eq!(count, 0);
    println!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[tokio::test]
#[ignore = "to enable when performance will be improved"]
async fn stress_test() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_and_gw().await?;
    // test_instance.disable_tracing();

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    let nb_response = 500;
    println!("Mocking {nb_response} UserDecryptionResponse in Postgres...");
    let mut responses_id = Vec::with_capacity(nb_response);
    for _ in 0..nb_response {
        match insert_rand_user_decrypt_response(&test_instance.db).await? {
            KmsResponse::UserDecryption { decryption_id, .. } => {
                responses_id.push(decryption_id);
            }
            _ => unreachable!(),
        }
    }
    println!("{nb_response} UserDecryptionResponse successfully stored!");

    println!("Checking responses has been sent to Anvil...");
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
    println!("Responses successfully sent to Anvil!");

    println!("Checking responses have been removed from DB...");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(&test_instance.db)
            .await?;
    assert_eq!(count, 0);
    println!("Responses successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

async fn start_test_tx_sender(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
) -> anyhow::Result<JoinHandle<()>> {
    let response_picker = DbKmsResponsePicker::connect(test_instance.db.clone()).await?;
    let response_remover = DbKmsResponseRemover::new(test_instance.db.clone());

    let tx_sender = TransactionSender::new(
        response_picker,
        test_instance.provider().clone(),
        test_instance.decryption_contract().clone(),
        response_remover,
    );

    Ok(tokio::spawn(tx_sender.start(cancel_token)))
}
