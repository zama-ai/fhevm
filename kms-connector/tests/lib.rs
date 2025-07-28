use alloy::{
    hex::FromHex,
    primitives::{FixedBytes, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::{
    config::KmsWallet,
    conn::{GatewayProvider, WalletGatewayProvider},
    tests::setup::{
        CHAIN_ID, DECRYPTION_MOCK_ADDRESS, DEPLOYER_PRIVATE_KEY, DbInstance,
        GATEWAY_CONFIG_MOCK_ADDRESS, GatewayInstance, KMS_MANAGEMENT_MOCK_ADDRESS, KmsInstance,
        S3_CT, S3Instance, TestInstance,
    },
};
use gw_listener::core::{DbEventPublisher, GatewayListener};
use kms_worker::core::{
    DbEventPicker, DbKmsResponsePublisher, KmsWorker, event_processor::DbEventProcessor,
};
use rstest::rstest;
use std::time::Duration;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tx_sender::core::{DbKmsResponsePicker, DbKmsResponseRemover, TransactionSender};

#[rstest]
#[timeout(Duration::from_secs(120))]
#[tokio::test]
#[ignore = "mock contract not adapted to real e2e tests for now"]
async fn test_e2e_public_decrypt() -> anyhow::Result<()> {
    let test_instance_builder = TestInstance::builder()
        .with_db(DbInstance::setup().await?)
        .with_gateway(GatewayInstance::setup().await?);
    let s3_instance = S3Instance::setup().await?;
    let kms_instance = KmsInstance::setup(&s3_instance.url).await?;
    let test_instance = test_instance_builder
        .with_s3(s3_instance)
        .with_kms(kms_instance)
        .build();

    let cancel_token = CancellationToken::new();

    let kms_connector = KmsConnector::setup(&test_instance).await?;
    let connector_task = tokio::spawn(kms_connector.start(cancel_token.clone()));

    info!("Mocking PublicDecryptionRequest on Anvil...");
    let pending_tx = test_instance
        .decryption_contract()
        .publicDecryptionRequest(vec![FixedBytes::from_hex(S3_CT).unwrap()], vec![].into())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

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
    assert_eq!(response.decryptionId, U256::ONE);
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
    tokio::time::timeout(Duration::from_secs(1), connector_task).await??;
    Ok(())
}

struct KmsConnector {
    gw_listener: GatewayListener<GatewayProvider, DbEventPublisher>,
    kms_worker: KmsWorker<DbEventPicker, DbEventProcessor<GatewayProvider>, DbKmsResponsePublisher>,
    tx_sender: TransactionSender<DbKmsResponsePicker, WalletGatewayProvider, DbKmsResponseRemover>,
}

impl KmsConnector {
    pub async fn setup(test_instance: &TestInstance) -> anyhow::Result<Self> {
        info!("Setting up KMS Connector sub-components...");
        let mut gw_listener_conf = gw_listener::core::Config {
            database_url: test_instance.db_url().to_string(),
            gateway_url: test_instance.anvil_ws_endpoint(),
            chain_id: *CHAIN_ID as u64,
            ..Default::default()
        };
        gw_listener_conf.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
        gw_listener_conf.kms_management_contract.address = KMS_MANAGEMENT_MOCK_ADDRESS;
        let (gw_listener, _) = GatewayListener::from_config(gw_listener_conf).await?;

        let mut kms_worker_conf = kms_worker::core::Config {
            database_url: test_instance.db_url().to_string(),
            kms_core_endpoint: test_instance.kms_url().to_string(),
            gateway_url: test_instance.anvil_ws_endpoint(),
            chain_id: *CHAIN_ID as u64,
            ..Default::default()
        };
        kms_worker_conf.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
        kms_worker_conf.gateway_config_contract.address = GATEWAY_CONFIG_MOCK_ADDRESS;
        let (kms_worker, _) = KmsWorker::from_config(kms_worker_conf).await?;

        let mut tx_sender_conf = tx_sender::core::Config::default().await;
        tx_sender_conf.database_url = test_instance.db_url().to_string();
        tx_sender_conf.gateway_url = test_instance.anvil_ws_endpoint();
        tx_sender_conf.chain_id = *CHAIN_ID as u64;
        tx_sender_conf.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
        tx_sender_conf.kms_management_contract.address = KMS_MANAGEMENT_MOCK_ADDRESS;
        tx_sender_conf.wallet =
            KmsWallet::from_private_key_str(DEPLOYER_PRIVATE_KEY, Some(*CHAIN_ID as u64))?;
        let (tx_sender, _) = TransactionSender::from_config(tx_sender_conf).await?;
        info!("KMS Connector sub-components successfully setup!");

        Ok(Self {
            gw_listener,
            kms_worker,
            tx_sender,
        })
    }

    pub async fn start(self, cancel_token: CancellationToken) -> Vec<()> {
        let mut tasks = tokio::task::JoinSet::new();
        tasks.spawn(self.gw_listener.start(cancel_token.clone()));
        tasks.spawn(self.kms_worker.start(cancel_token.clone()));
        tasks.spawn(self.tx_sender.start(cancel_token.clone()));
        tasks.join_all().await
    }
}
