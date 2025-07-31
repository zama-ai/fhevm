use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use connector_utils::tests::{
    rand::{rand_address, rand_public_key, rand_u256},
    setup::{
        DECRYPTION_MOCK_ADDRESS, KMS_MANAGEMENT_MOCK_ADDRESS, TestInstance, TestInstanceBuilder,
    },
};
use connector_utils::types::db::SnsCiphertextMaterialDbItem;
use fhevm_gateway_rust_bindings::decryption::IDecryption::{ContractsInfo, RequestValidity};
use gw_listener::core::{Config, DbEventPublisher, GatewayListener};
use sqlx::Row;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_public_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking PublicDecryptionRequest on Anvil...");
    let pending_tx = test_instance
        .decryption_contract()
        .publicDecryptionRequest(vec![], Bytes::new())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT decryption_id, sns_ct_materials FROM public_decryption_requests")
        .fetch_one(test_instance.db())
        .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let sns_ct_materials =
        row.try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;
    assert_eq!(decryption_id, U256::ONE);
    assert_eq!(
        sns_ct_materials,
        vec![SnsCiphertextMaterialDbItem::default()]
    );
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_user_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking UserDecryptionRequest on Anvil...");
    let rand_user_addr = rand_address();
    let rand_pub_key = rand_public_key();
    let pending_tx = test_instance
        .decryption_contract()
        .userDecryptionRequest(
            vec![],
            RequestValidity::default(),
            ContractsInfo::default(),
            rand_user_addr,
            rand_pub_key.clone().into(),
            vec![].into(),
            vec![].into(),
        )
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT decryption_id, sns_ct_materials, user_address, public_key FROM user_decryption_requests")
        .fetch_one(test_instance.db())
        .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let user_address = Address::from(row.try_get::<[u8; 20], _>("user_address")?);
    let pub_key = row.try_get::<Vec<u8>, _>("public_key")?;
    let sns_ct_materials =
        row.try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;

    assert_eq!(decryption_id, U256::ONE);
    assert_eq!(
        sns_ct_materials,
        vec![SnsCiphertextMaterialDbItem::default()]
    );
    assert_eq!(rand_user_addr, user_address);
    assert_eq!(rand_pub_key, pub_key);
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_preprocess_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking PreprocessKeygenRequest on Anvil...");
    let pending_tx = test_instance
        .kms_management_contract()
        .preprocessKeygenRequest(String::new())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_keygen_request_id, fhe_params_digest FROM preprocess_keygen_requests",
    )
    .fetch_one(test_instance.db())
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_keygen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_preprocess_kskgen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking PreprocessKskgenRequest on Anvil...");
    let pending_tx = test_instance
        .kms_management_contract()
        .preprocessKskgenRequest(String::new())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_kskgen_request_id, fhe_params_digest FROM preprocess_kskgen_requests",
    )
    .fetch_one(test_instance.db())
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_kskgen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking KeygenRequest on Anvil...");
    let rand_id = rand_u256();
    let pending_tx = test_instance
        .kms_management_contract()
        .keygenRequest(rand_id)
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT pre_key_id, fhe_params_digest FROM keygen_requests")
        .fetch_one(test_instance.db())
        .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_key_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, rand_id);
    assert_eq!(digest, U256::default());
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_kskgen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking KskgenRequest on Anvil...");
    let rand_id = rand_u256();
    let rand_source_key_id = rand_u256();
    let rand_dest_key_id = rand_u256();
    let pending_tx = test_instance
        .kms_management_contract()
        .kskgenRequest(rand_id, rand_source_key_id, rand_dest_key_id)
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_ksk_id, source_key_id, dest_key_id, fhe_params_digest FROM kskgen_requests",
    )
    .fetch_one(test_instance.db())
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_ksk_id")?);
    let source_key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("source_key_id")?);
    let dest_key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("dest_key_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, rand_id);
    assert_eq!(rand_source_key_id, source_key_id);
    assert_eq!(rand_dest_key_id, dest_key_id);
    assert_eq!(digest, U256::default());
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_publish_crsgen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    info!("Mocking CrsgenRequest on Anvil...");
    let pending_tx = test_instance
        .kms_management_contract()
        .crsgenRequest(String::new())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(600)).await; // Waiting for the gw_listener to process event
    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT crsgen_request_id, fhe_params_digest FROM crsgen_requests")
        .fetch_one(test_instance.db())
        .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("crsgen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
#[ignore = "flaky tests to be fixed"]
async fn test_catchup() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();

    info!("Mocking PublicDecryptionRequest on Anvil...");
    let pending_tx1 = test_instance
        .decryption_contract()
        .publicDecryptionRequest(vec![], Bytes::new())
        .send()
        .await?;
    let receipt1 = pending_tx1.get_receipt().await?;
    let tx1 = test_instance
        .provider()
        .get_transaction_by_hash(receipt1.transaction_hash)
        .await?
        .unwrap();
    info!(
        "Tx1 successfully sent in block {}!",
        tx1.block_number.unwrap()
    );

    let pending_tx2 = test_instance
        .decryption_contract()
        .publicDecryptionRequest(vec![], Bytes::new())
        .send()
        .await?;
    let receipt2 = pending_tx2.get_receipt().await?;
    let tx2 = test_instance
        .provider()
        .get_transaction_by_hash(receipt2.transaction_hash)
        .await?
        .unwrap();
    info!(
        "Tx2 successfully sent in block {}!",
        tx2.block_number.unwrap()
    );

    // Ensure that the two transactions are in different blocks.
    assert_ne!(tx1.block_number, tx2.block_number);

    // Start the listener after the transactions are sent.
    let gw_listener_task = start_test_listener(
        &test_instance,
        cancel_token.clone(),
        Some(tx1.block_number.unwrap()),
    );

    loop {
        info!("Checking event is stored in DB...");
        let row =
            sqlx::query("SELECT decryption_id, sns_ct_materials FROM public_decryption_requests")
                .fetch_all(test_instance.db())
                .await?;

        if row.len() < 2 {
            info!("Not all events found yet, retrying...");
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        }

        let decryption_id1 = U256::from_le_bytes(row[0].try_get::<[u8; 32], _>("decryption_id")?);
        let sns_ct_materials =
            row[0].try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;
        assert_eq!(decryption_id1, U256::from(1));
        assert_eq!(
            sns_ct_materials,
            vec![SnsCiphertextMaterialDbItem::default()]
        );

        let decryption_id2 = U256::from_le_bytes(row[1].try_get::<[u8; 32], _>("decryption_id")?);
        let sns_ct_materials =
            row[1].try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;
        assert_eq!(decryption_id2, U256::from(2));
        assert_eq!(
            sns_ct_materials,
            vec![SnsCiphertextMaterialDbItem::default()]
        );
        break;
    }

    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

fn start_test_listener(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
    from_block_number: Option<u64>,
) -> anyhow::Result<JoinHandle<()>> {
    let publisher = DbEventPublisher::new(test_instance.db().clone());

    let mut config = Config::default();
    config.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
    config.kms_management_contract.address = KMS_MANAGEMENT_MOCK_ADDRESS;
    config.from_block_number = from_block_number;
    let gw_listener = GatewayListener::new(&config, test_instance.provider().clone(), publisher);

    Ok(tokio::spawn(gw_listener.start(cancel_token)))
}
