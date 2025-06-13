use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, FixedBytes, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
};
use connector_utils::{conn::GatewayProvider, types::db::SnsCiphertextMaterialDbItem};
use fhevm_gateway_rust_bindings::{
    decryption::{Decryption, IDecryption::RequestValidity},
    kmsmanagement::KmsManagement,
};
use gw_listener::core::{Config, DbEventPublisher, GatewayListener};
use rand::Rng;
use sqlx::{Pool, Postgres, Row};
use std::time::Duration;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn test_publish_public_decryption() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking PublicDecryptionRequest on Anvil...");
    let contract = Decryption::new(DECRYPTION_MOCK_ADDRESS, test_instance.provider.clone());
    let pending_tx = contract
        .publicDecryptionRequest(vec![])
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT decryption_id, sns_ct_materials FROM public_decryption_requests")
        .fetch_one(&test_instance.db)
        .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let sns_ct_materials =
        row.try_get::<Vec<SnsCiphertextMaterialDbItem>, _>("sns_ct_materials")?;
    assert_eq!(decryption_id, U256::ONE);
    assert_eq!(
        sns_ct_materials,
        vec![SnsCiphertextMaterialDbItem::default()]
    );
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_user_decryption() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking UserDecryptionRequest on Anvil...");
    let rand_user_addr = rand_address();
    let rand_pub_key = rand::thread_rng().r#gen::<[u8; 32]>().to_vec();
    let contract = Decryption::new(DECRYPTION_MOCK_ADDRESS, test_instance.provider.clone());
    let pending_tx = contract
        .userDecryptionRequest(
            vec![],
            RequestValidity::default(),
            U256::default(),
            vec![],
            rand_user_addr,
            rand_pub_key.clone().into(),
            vec![].into(),
        )
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT decryption_id, sns_ct_materials, user_address, public_key FROM user_decryption_requests")
        .fetch_one(&test_instance.db)
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
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_preprocess_keygen() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking PreprocessKeygenRequest on Anvil...");
    let contract = KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, test_instance.provider.clone());
    let pending_tx = contract
        .preprocessKeygenRequest(String::new())
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_keygen_request_id, fhe_params_digest FROM preprocess_keygen_requests",
    )
    .fetch_one(&test_instance.db)
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_keygen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_preprocess_kskgen() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking PreprocessKskgenRequest on Anvil...");
    let contract = KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, test_instance.provider.clone());
    let pending_tx = contract
        .preprocessKskgenRequest(String::new())
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_kskgen_request_id, fhe_params_digest FROM preprocess_kskgen_requests",
    )
    .fetch_one(&test_instance.db)
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_kskgen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_keygen() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking KeygenRequest on Anvil...");
    let contract = KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, test_instance.provider.clone());
    let rand_id = rand_u256();
    let pending_tx = contract
        .keygenRequest(rand_id)
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT pre_key_id, fhe_params_digest FROM keygen_requests")
        .fetch_one(&test_instance.db)
        .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_key_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, rand_id);
    assert_eq!(digest, U256::default());
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_kskgen() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking KskgenRequest on Anvil...");
    let contract = KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, test_instance.provider.clone());
    let rand_id = rand_u256();
    let rand_source_key_id = rand_u256();
    let rand_dest_key_id = rand_u256();
    let pending_tx = contract
        .kskgenRequest(rand_id, rand_source_key_id, rand_dest_key_id)
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query(
        "SELECT pre_ksk_id, source_key_id, dest_key_id, fhe_params_digest FROM kskgen_requests",
    )
    .fetch_one(&test_instance.db)
    .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("pre_ksk_id")?);
    let source_key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("source_key_id")?);
    let dest_key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("dest_key_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, rand_id);
    assert_eq!(rand_source_key_id, source_key_id);
    assert_eq!(rand_dest_key_id, dest_key_id);
    assert_eq!(digest, U256::default());
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

#[tokio::test]
async fn test_publish_crsgen() -> anyhow::Result<()> {
    let test_instance = setup_test_instance().await?;
    let cancel_token = CancellationToken::new();
    let gw_listener_task = start_test_listener(&test_instance, cancel_token.clone());
    tokio::time::sleep(Duration::from_millis(200)).await; // Waiting for the gw_listener to subscribe events

    println!("Mocking CrsgenRequest on Anvil...");
    let contract = KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, test_instance.provider.clone());
    let pending_tx = contract
        .crsgenRequest(String::new())
        .from(test_instance.anvil.addresses()[0])
        .send()
        .await?;
    let receipt = pending_tx.get_receipt().await?;
    let _tx = test_instance
        .provider
        .get_transaction_by_hash(receipt.transaction_hash)
        .await?
        .unwrap();
    println!("Tx successfully sent!");

    tokio::time::sleep(Duration::from_millis(400)).await; // Waiting for the gw_listener to process event
    println!("Checking event is stored in DB...");
    let row = sqlx::query("SELECT crsgen_request_id, fhe_params_digest FROM crsgen_requests")
        .fetch_one(&test_instance.db)
        .await?;

    let id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("crsgen_request_id")?);
    let digest = U256::from_le_bytes(row.try_get::<[u8; 32], _>("fhe_params_digest")?);
    assert_eq!(id, U256::ONE);
    assert_eq!(digest, U256::default());
    println!("Event successfully stored! Stopping GatewayListener...");

    cancel_token.cancel();
    Ok(gw_listener_task?.await?)
}

const DECRYPTION_MOCK_ADDRESS: Address = Address(FixedBytes([
    184, 174, 68, 54, 92, 69, 167, 197, 37, 107, 20, 246, 7, 202, 226, 59, 192, 64, 195, 84,
]));
// const GATEWAY_CONFIG_MOCK_ADDRESS: Address = Address(FixedBytes([
//     159, 167, 153, 249, 90, 114, 37, 140, 4, 21, 223, 237, 216, 207, 118, 210, 97, 60, 117, 15,
// ]));
const KMS_MANAGEMENT_MOCK_ADDRESS: Address = Address(FixedBytes([
    200, 27, 227, 169, 24, 21, 210, 212, 9, 109, 174, 8, 26, 113, 22, 201, 250, 123, 223, 8,
]));

const TEST_MNEMONIC: &str =
    "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

fn start_test_listener(
    test_instance: &TestInstance,
    cancel_token: CancellationToken,
) -> anyhow::Result<JoinHandle<()>> {
    let publisher = DbEventPublisher::new(test_instance.db.clone());

    let mut config = Config::default();
    config.decryption_contract.address = DECRYPTION_MOCK_ADDRESS;
    config.kms_management_contract.address = KMS_MANAGEMENT_MOCK_ADDRESS;
    let gw_listener = GatewayListener::new(&config, test_instance.provider.clone(), publisher);

    Ok(tokio::spawn(gw_listener.start(cancel_token)))
}

async fn setup_test_instance() -> anyhow::Result<TestInstance> {
    let anvil = setup_anvil_gateway().await?;
    let (db_container, db) = setup_test_db_instance().await?;

    let provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;

    Ok(TestInstance {
        _db_container: db_container,
        db,
        anvil,
        provider,
    })
}

async fn setup_anvil_gateway() -> anyhow::Result<AnvilInstance> {
    let chain_id = rand::random::<u32>();
    let anvil = Anvil::new()
        .mnemonic(TEST_MNEMONIC)
        .block_time(1)
        .chain_id(chain_id as u64)
        .try_spawn()?;
    println!("Anvil started...");

    let _deploy_mock_container =
        GenericImage::new("ghcr.io/zama-ai/fhevm/gateway-contracts", "v0.7.0-rc0")
            .with_wait_for(WaitFor::message_on_stdout("Mock contract deployment done!"))
            .with_env_var("HARDHAT_NETWORK", "staging")
            .with_env_var("RPC_URL", anvil.endpoint_url().as_str())
            .with_env_var("CHAIN_ID_GATEWAY", format!("{chain_id}"))
            .with_env_var("MNEMONIC", TEST_MNEMONIC)
            .with_env_var(
                "DEPLOYER_ADDRESS",
                "0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634",
            ) // accounts[1]
            .with_env_var(
                "DEPLOYER_PRIVATE_KEY",
                "0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c",
            ) // accounts[1]
            .with_env_var(
                "PAUSER_ADDRESS",
                "0xfCefe53c7012a075b8a711df391100d9c431c468",
            )
            .with_network("host")
            .with_cmd(["npx hardhat task:deployGatewayMockContracts"])
            .start()
            .await?;
    println!("Mock contract successfully deployed on Anvil!");

    Ok(anvil)
}

fn rand_u256() -> U256 {
    U256::from_le_bytes(rand::thread_rng().r#gen::<[u8; 32]>())
}

fn rand_address() -> Address {
    Address::from(rand::thread_rng().r#gen::<[u8; 20]>())
}

async fn setup_test_db_instance() -> anyhow::Result<(ContainerAsync<GenericImage>, Pool<Postgres>)>
{
    let container = GenericImage::new("postgres", "17.5")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await?;
    println!("Postgres started...");

    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;
    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/kms-connector");

    println!("Creating KMS Connector db...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_db_url)
        .await?;
    sqlx::query("CREATE DATABASE \"kms-connector\";")
        .execute(&admin_pool)
        .await?;
    println!("KMS Connector DB url: {db_url}");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    println!("Running migrations...");
    sqlx::migrate!("../../connector-db/migrations")
        .run(&pool)
        .await?;
    println!("KMS Connector DB ready!");

    Ok((container, pool))
}

struct TestInstance {
    pub _db_container: ContainerAsync<GenericImage>,
    pub db: Pool<Postgres>,
    pub anvil: AnvilInstance,
    pub provider: GatewayProvider,
}
