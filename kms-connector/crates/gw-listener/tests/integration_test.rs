use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use connector_utils::{tests::rand::rand_signature, types::db::SnsCiphertextMaterialDbItem};
use connector_utils::{
    tests::{
        rand::{rand_address, rand_public_key, rand_u256},
        setup::{
            DECRYPTION_MOCK_ADDRESS, KMS_MANAGEMENT_MOCK_ADDRESS, TestInstance, TestInstanceBuilder,
        },
    },
    types::db::ParamsTypeDb,
};
use fhevm_gateway_bindings::decryption::IDecryption::{ContractsInfo, RequestValidity};
use gw_listener::core::{Config, DbEventPublisher, GatewayListener};
use rstest::rstest;
use sqlx::Row;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_public_decryption() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);

    // Wait for gw-listener to be ready + 2 anvil blocks
    test_instance
        .wait_for_log("Waiting for next PublicDecryptionRequest...")
        .await;
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

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

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

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

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_user_decryption() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);

    // Wait for gw-listener to be ready + 2 anvil blocks
    test_instance
        .wait_for_log("Waiting for next UserDecryptionRequest...")
        .await;
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

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

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

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

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_prep_keygen() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);

    // Wait for gw-listener to be ready + 2 anvil blocks
    test_instance
        .wait_for_log("Waiting for next PrepKeygenRequest...")
        .await;
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    info!("Mocking PrepKeygenRequest on Anvil...");
    let pending_tx = test_instance
        .kms_management_contract()
        .keygen(ParamsTypeDb::Test as u8)
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT prep_keygen_id, epoch_id, params_type FROM prep_keygen_requests")
        .fetch_one(test_instance.db())
        .await?;

    let prep_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?);
    let epoch_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?);
    let params_type = row.try_get::<ParamsTypeDb, _>("params_type")?;
    assert_eq!(prep_id, U256::ONE);
    assert_eq!(epoch_id, U256::default());
    assert_eq!(params_type, ParamsTypeDb::Test);
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_keygen() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);

    // Wait for gw-listener to be ready + 2 anvil blocks
    test_instance
        .wait_for_log("Waiting for next KeygenRequest...")
        .await;
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    info!("Mocking KeygenRequest on Anvil...");
    let rand_prep_id = rand_u256();
    let rand_signature = rand_signature();
    let pending_tx = test_instance
        .kms_management_contract()
        .prepKeygenResponse(rand_prep_id, rand_signature.into())
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT prep_keygen_id, key_id FROM keygen_requests")
        .fetch_one(test_instance.db())
        .await?;

    let prep_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?);
    let key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?);
    assert_eq!(prep_id, rand_prep_id);
    assert_eq!(key_id, U256::ONE);
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_publish_crsgen() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone(), None);

    // Wait for gw-listener to be ready + 2 anvil blocks
    test_instance
        .wait_for_log("Waiting for next CrsgenRequest...")
        .await;
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    info!("Mocking CrsgenRequest on Anvil...");
    let rand_max_bit_length = rand_u256();
    let pending_tx = test_instance
        .kms_management_contract()
        .crsgenRequest(rand_max_bit_length, ParamsTypeDb::Test as u8)
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider()
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    info!("Tx successfully sent!");

    test_instance
        .wait_for_log("Event successfully stored in DB!")
        .await;

    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT crs_id, max_bit_length, params_type FROM crsgen_requests")
        .fetch_one(test_instance.db())
        .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?);
    let max_bit_length = U256::from_le_bytes(row.try_get::<[u8; 32], _>("max_bit_length")?);
    let params_type = row.try_get::<ParamsTypeDb, _>("params_type")?;
    assert_eq!(id, U256::ONE);
    assert_eq!(max_bit_length, rand_max_bit_length);
    assert_eq!(params_type, ParamsTypeDb::Test);
    info!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_catchup() -> anyhow::Result<()> {
    let mut test_instance = TestInstanceBuilder::db_gw_setup().await?;
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

    // Wait for two more anvil blocks
    tokio::time::sleep(2 * test_instance.anvil_block_time()).await;

    // Start the listener after the transactions are sent.
    let gw_listener_task = start_test_listener(
        &test_instance,
        cancel_token.clone(),
        Some(tx1.block_number.unwrap()),
    );

    for _ in 0..2 {
        test_instance
            .wait_for_log("Event successfully stored in DB!")
            .await;
    }

    info!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT decryption_id, sns_ct_materials FROM public_decryption_requests")
        .fetch_all(test_instance.db())
        .await?;

    if row.len() < 2 {
        panic!("Not all events found yet...");
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
    config.decryption_polling = Duration::from_millis(300);
    config.key_management_polling = Duration::from_millis(300);
    let gw_listener = GatewayListener::new(&config, test_instance.provider().clone(), publisher);

    Ok(tokio::spawn(gw_listener.start(cancel_token)))
}
