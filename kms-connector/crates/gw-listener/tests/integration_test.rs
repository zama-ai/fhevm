use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use connector_utils::tests::{
    rand::{rand_address, rand_public_key, rand_u256},
    setup::{
        DECRYPTION_MOCK_ADDRESS, KMS_MANAGEMENT_MOCK_ADDRESS, TestInstance,
        shared::{clean_test_instance, run_with_shared_db_gw_setup},
    },
};
use connector_utils::types::db::SnsCiphertextMaterialDbItem;
use ctor::dtor;
use fhevm_gateway_rust_bindings::decryption::IDecryption::RequestValidity;
use gw_listener::core::{Config, DbEventPublisher, GatewayListener};
use rstest::rstest;
use serial_test::serial;
use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[dtor]
fn on_shutdown() {
    clean_test_instance();
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_public_decryption() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

        info!("Mocking PublicDecryptionRequest on Anvil...");
        let pending_tx = test_instance
            .decryption_contract()
            .publicDecryptionRequest(vec![])
            .send()
            .await?;
        let receipt = pending_tx.get_receipt().await?;
        let _tx = test_instance
            .provider()
            .get_transaction_by_hash(receipt.transaction_hash)
            .await?
            .unwrap();
        info!("Tx successfully sent!");

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT decryption_id, sns_ct_materials FROM public_decryption_requests",
        )
        .await?;

        let sns_ct_materials =
            row.try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;
        assert_eq!(
            sns_ct_materials,
            vec![SnsCiphertextMaterialDbItem::default()]
        );
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_user_decryption() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

        info!("Mocking UserDecryptionRequest on Anvil...");
        let rand_user_addr = rand_address();
        let rand_pub_key = rand_public_key();
        let pending_tx = test_instance
            .decryption_contract()
            .userDecryptionRequest(
                vec![],
                RequestValidity::default(),
                U256::default(),
                vec![],
                rand_user_addr,
                rand_pub_key.clone().into(),
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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(test_instance.db(), "SELECT decryption_id, sns_ct_materials, user_address, public_key FROM user_decryption_requests")
            .await?;

        let user_address = Address::from(row.try_get::<[u8; 20], _>("user_address")?);
        let pub_key = row.try_get::<Vec<u8>, _>("public_key")?;
        let sns_ct_materials =
            row.try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;

        assert_eq!(
            sns_ct_materials,
            vec![SnsCiphertextMaterialDbItem::default()]
        );
        assert_eq!(rand_user_addr, user_address);
        assert_eq!(rand_pub_key, pub_key);
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_preprocess_keygen() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT pre_keygen_request_id, fhe_params_digest FROM preprocess_keygen_requests",
        )
        .await?;

        let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_keygen_request_id")?);
        let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
        assert_eq!(id, U256::ONE);
        assert_eq!(digest, U256::default());
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_preprocess_kskgen() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT pre_kskgen_request_id, fhe_params_digest FROM preprocess_kskgen_requests",
        )
        .await?;

        let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_kskgen_request_id")?);
        let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
        assert_eq!(id, U256::ONE);
        assert_eq!(digest, U256::default());
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_keygen() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT pre_key_id, fhe_params_digest FROM keygen_requests",
        )
        .await?;

        let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_key_id")?);
        let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
        assert_eq!(id, rand_id);
        assert_eq!(digest, U256::default());
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_kskgen() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT pre_ksk_id, source_key_id, dest_key_id, fhe_params_digest FROM kskgen_requests",
        )
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
    })
}

#[rstest]
#[timeout(Duration::from_secs(90))]
#[serial]
fn test_publish_crsgen() -> anyhow::Result<()> {
    run_with_shared_db_gw_setup(async |test_instance| {
        let cancel_token = CancellationToken::new();
        let gw_listener_task = start_test_listener(test_instance, cancel_token.clone());
        tokio::time::sleep(Duration::from_millis(800)).await; // Waiting for the gw_listener to subscribe events

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

        info!("Checking event is stored in DB...");
        let row = query_with_retry(
            test_instance.db(),
            "SELECT crsgen_request_id, fhe_params_digest FROM crsgen_requests",
        )
        .await?;

        let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("crsgen_request_id")?);
        let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
        assert_eq!(id, U256::ONE);
        assert_eq!(digest, U256::default());
        info!("Event successfully stored! Stopping GatewayListener...");

        cancel_token.cancel();
        Ok(gw_listener_task?.await?)
    })
}

async fn query_with_retry(db: &Pool<Postgres>, query: &str) -> sqlx::Result<PgRow> {
    let mut error = sqlx::Error::RowNotFound;
    for i in 1..=5 {
        tokio::time::sleep(Duration::from_millis(200)).await;
        match sqlx::query(query).fetch_one(db).await {
            Ok(row) => return Ok(row),
            Err(e) => {
                warn!("{i}/5 attempt: {e}");
                error = e;
            }
        };
    }
    Err(error)
}

fn start_test_listener(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
) -> anyhow::Result<JoinHandle<()>> {
    let publisher = DbEventPublisher::new(test_instance.db().clone());

    let mut config = Config::default();
    config.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
    config.kms_management_contract.address = KMS_MANAGEMENT_MOCK_ADDRESS;
    let gw_listener = GatewayListener::new(&config, test_instance.provider().clone(), publisher);

    Ok(tokio::spawn(gw_listener.start(cancel_token)))
}
