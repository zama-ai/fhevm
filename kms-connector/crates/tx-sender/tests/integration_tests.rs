use alloy::primitives::U256;
use anyhow::anyhow;
use connector_utils::{
    config::KmsWallet,
    conn::connect_to_gateway_with_wallet,
    tests::{
        db::responses::{
            insert_rand_crsgen_response, insert_rand_keygen_response,
            insert_rand_prep_keygen_response, insert_rand_public_decrypt_response,
            insert_rand_user_decrypt_response,
        },
        setup::{
            CHAIN_ID, DEPLOYER_PRIVATE_KEY, KMS_GENERATION_MOCK_ADDRESS,
            TestInstance, TestInstanceBuilder,
        },
    },
    types::db::OperationStatus,
};
use fhevm_gateway_bindings::kms_generation::KMSGeneration::KMSGenerationInstance;
use rstest::rstest;
use sqlx::Row;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, TransactionSender};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
#[ignore = "V2: Decryption responses now served via HTTP API, not on-chain transactions"]
async fn test_process_public_decryption_response() -> anyhow::Result<()> {
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
#[ignore = "V2: Decryption responses now served via HTTP API, not on-chain transactions"]
async fn test_process_user_decryption_response() -> anyhow::Result<()> {
    Ok(())
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

    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking PrepKeygenResponse in Postgres...");
    let inserted_response =
        insert_rand_prep_keygen_response(test_instance.db(), None, None).await?;
    info!("PrepKeygenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = keygen_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture PrepKeygenResponse"))??;
    assert_eq!(response.prepKeygenId, inserted_response.prep_keygen_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully updated response in DB!")
        .await;

    info!("Checking response was completed in DB...");
    let status: OperationStatus =
        sqlx::query("SELECT status FROM prep_keygen_responses WHERE prep_keygen_id = $1")
            .bind(response.prepKeygenId.as_le_slice())
            .fetch_one(test_instance.db())
            .await?
            .try_get("status")?;
    assert_eq!(status, OperationStatus::Completed);
    info!("Response successfully completed in DB! Stopping TransactionSender...");

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

    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking KeygenResponse in Postgres...");
    let inserted_response = insert_rand_keygen_response(test_instance.db(), None, None).await?;
    info!("KeygenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = activate_key_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture KeygenResponse"))??;
    assert_eq!(response.keyId, inserted_response.key_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully updated response in DB!")
        .await;

    info!("Checking response was completed in DB...");
    let status: OperationStatus =
        sqlx::query("SELECT status FROM keygen_responses WHERE key_id = $1")
            .bind(response.keyId.as_le_slice())
            .fetch_one(test_instance.db())
            .await?
            .try_get("status")?;
    assert_eq!(status, OperationStatus::Completed);
    info!("Response successfully completed in DB! Stopping TransactionSender...");

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

    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    let cancel_token = CancellationToken::new();
    let tx_sender_task = start_test_tx_sender(&test_instance, cancel_token.clone()).await?;

    info!("Mocking CrsgenResponse in Postgres...");
    let inserted_response = insert_rand_crsgen_response(test_instance.db(), None, None).await?;
    info!("CrsgenResponse successfully stored!");

    info!("Checking response has been sent to Anvil...");
    let (response, _) = activate_crs_stream
        .next()
        .await
        .ok_or_else(|| anyhow!("Failed to capture CrsgenResponse"))??;
    assert_eq!(response.crsId, inserted_response.crs_id);
    info!("Response successfully sent to Anvil!");

    test_instance
        .wait_for_log("Successfully updated response in DB!")
        .await;

    info!("Checking response was completed in DB...");
    let status: OperationStatus =
        sqlx::query("SELECT status FROM crsgen_responses WHERE crs_id = $1")
            .bind(response.crsId.as_le_slice())
            .fetch_one(test_instance.db())
            .await?
            .try_get("status")?;
    assert_eq!(status, OperationStatus::Completed);
    info!("Response successfully completed in DB! Stopping TransactionSender...");

    cancel_token.cancel();
    Ok(tx_sender_task.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
#[ignore = "V2: Decryption responses now served via HTTP API, not on-chain transactions"]
async fn stress_test() -> anyhow::Result<()> {
    Ok(())
}

async fn start_test_tx_sender(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
) -> anyhow::Result<JoinHandle<()>> {
    let response_picker =
        DbKmsResponsePicker::connect(test_instance.db().clone(), &Config::default()).await?;
    let provider = connect_to_gateway_with_wallet(
        test_instance.anvil_http_endpoint(),
        *CHAIN_ID as u64,
        KmsWallet::from_private_key_str(DEPLOYER_PRIVATE_KEY, Some(*CHAIN_ID as u64))?,
    )
    .await?;

    let tx_sender_inner = tx_sender::core::tx_sender::TransactionSenderInner::new(
        provider.clone(),
        KMSGenerationInstance::new(KMS_GENERATION_MOCK_ADDRESS, provider),
        tx_sender::core::tx_sender::TransactionSenderInnerConfig {
            tx_retries: 3,
            tx_retry_interval: Duration::from_millis(100),
            trace_reverted_tx: true,
            gas_multiplier_percent: 130,
        },
    );
    let tx_sender =
        TransactionSender::new(response_picker, tx_sender_inner, test_instance.db().clone());

    Ok(tokio::spawn(tx_sender.start(cancel_token)))
}
