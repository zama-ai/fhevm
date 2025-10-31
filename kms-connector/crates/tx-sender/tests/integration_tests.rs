mod common;

use alloy::primitives::U256;
use anyhow::anyhow;
use common::{
    insert_rand_keygen_response, insert_rand_prep_keygen_response,
    insert_rand_public_decrypt_response, insert_rand_user_decrypt_response,
};
use connector_utils::{
    config::KmsWallet,
    conn::connect_to_gateway_with_wallet,
    tests::setup::{
        CHAIN_ID, DECRYPTION_MOCK_ADDRESS, DEPLOYER_PRIVATE_KEY, KMS_GENERATION_MOCK_ADDRESS,
        TestInstance, TestInstanceBuilder,
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::DecryptionInstance,
    kms_generation::KMSGeneration::KMSGenerationInstance,
};
use rstest::rstest;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tx_sender::core::{
    Config, DbKmsResponsePicker, DbKmsResponseRemover, TransactionSender,
    tx_sender::{TransactionSenderInner, TransactionSenderInnerConfig},
};

use crate::common::insert_rand_crsgen_response;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_process_public_decryption_response() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut response_filter = test_instance
        .decryption_contract()
        .PublicDecryptionResponse_filter()
        .watch()
        .await?;
    response_filter.poller = response_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let mut response_stream = response_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking PublicDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_public_decrypt_response(test_instance.db(), None).await?;
    info!("PublicDecryptionResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = response_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture PublicDecryptionResponse"))??;
    assert_eq!(response.decryptionId, inserted_response.decryption_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully removed response from DB!")
        .await;

    info!("Checking response has been removed from DB...");
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
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_process_user_decryption_response() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut response_filter = test_instance
        .decryption_contract()
        .UserDecryptionResponse_filter()
        .watch()
        .await?;
    response_filter.poller = response_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let mut response_stream = response_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking UserDecryptionResponse in Postgres...");
    let inserted_response = insert_rand_user_decrypt_response(test_instance.db(), None).await?;
    info!("UserDecryptionResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = response_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture UserDecryptionResponse"))??;
    assert_eq!(response.decryptionId, inserted_response.decryption_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully removed response from DB!")
        .await;

    info!("Checking response has been removed from DB...");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_process_prep_keygen_response() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut keygen_filter = test_instance
        .kms_generation_contract()
        .KeygenRequest_filter()
        .watch()
        .await?;
    keygen_filter.poller = keygen_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let mut keygen_stream = keygen_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking PrepKeygenResponse in Postgres...");
    let inserted_response = insert_rand_prep_keygen_response(test_instance.db(), None).await?;
    info!("PrepKeygenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = keygen_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture PrepKeygenResponse"))??;
    assert_eq!(response.prepKeygenId, inserted_response.prep_keygen_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully removed response from DB!")
        .await;

    info!("Checking response has been removed from DB...");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(prep_keygen_id) FROM prep_keygen_responses")
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_process_keygen_response() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut activate_key_filter = test_instance
        .kms_generation_contract()
        .ActivateKey_filter()
        .watch()
        .await?;
    activate_key_filter.poller = activate_key_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let mut activate_key_stream = activate_key_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking KeygenResponse in Postgres...");
    let inserted_response = insert_rand_keygen_response(test_instance.db(), None).await?;
    info!("KeygenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = activate_key_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture KeygenResponse"))??;
    assert_eq!(response.keyId, inserted_response.key_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully removed response from DB!")
        .await;

    info!("Checking response has been removed from DB...");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(key_id) FROM keygen_responses")
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_process_crsgen_response() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut activate_crs_filter = test_instance
        .kms_generation_contract()
        .ActivateCrs_filter()
        .watch()
        .await?;
    activate_crs_filter.poller = activate_crs_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let mut activate_crs_stream = activate_crs_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking CrsgenResponse in Postgres...");
    let inserted_response = insert_rand_crsgen_response(test_instance.db(), None).await?;
    info!("CrsgenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = activate_crs_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture CrsgenResponse"))??;
    assert_eq!(response.crsId, inserted_response.crs_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully removed response from DB!")
        .await;

    info!("Checking response has been removed from DB...");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(crs_id) FROM crsgen_responses")
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(count, 0);
    info!("Response successfully removed from DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn stress_test() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let mut response_filter = test_instance
        .decryption_contract()
        .UserDecryptionResponse_filter()
        .watch()
        .await?;
    response_filter.poller = response_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));
    let response_stream = response_filter.into_stream();

    // Wait for 2 anvil blocks before starting the tx-sender, so event listening is fully ready
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    let nb_response = 500;
    info!("Mocking {nb_response} UserDecryptionResponse in Postgres...");
    let mut responses_id = Vec::with_capacity(nb_response);
    for _ in 0..nb_response {
        let response = insert_rand_user_decrypt_response(test_instance.db(), None).await?;
        responses_id.push(response.decryption_id);
    }
    info!("{nb_response} UserDecryptionResponse successfully stored!");

    info!("Checking responses has been sent to Anvil...");
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

    for _ in 0..nb_response {
        test_instance
            .wait_for_log("Successfully removed response from DB!")
            .await;
    }
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
    let response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default().await).await?;
    let response_remover = DbKmsResponseRemover::new(test_instance.db().clone());
    let provider = connect_to_gateway_with_wallet(
        test_instance.anvil_http_endpoint().as_str(),
        *CHAIN_ID as u64,
        KmsWallet::from_private_key_str(DEPLOYER_PRIVATE_KEY, Some(*CHAIN_ID as u64))?,
    )
    .await?;

    let tx_sender_inner = TransactionSenderInner::new(
        provider.clone(),
        DecryptionInstance::new(DECRYPTION_MOCK_ADDRESS, provider.clone()),
        KMSGenerationInstance::new(KMS_GENERATION_MOCK_ADDRESS, provider),
        TransactionSenderInnerConfig {
            tx_retries: 3,
            tx_retry_interval: Duration::from_millis(100),
            trace_reverted_tx: true,
            gas_multiplier_percent: 130,
        },
    );
    let tx_sender = TransactionSender::new(response_picker, tx_sender_inner, response_remover);

    Ok(tokio::spawn(tx_sender.start(cancel_token)))
}
