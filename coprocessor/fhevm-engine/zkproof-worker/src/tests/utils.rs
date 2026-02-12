use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::crs::CrsCache;
use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::utils::{safe_serialize, DatabaseURL};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use test_harness::instance::{DBInstance, ImportMode};
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::auxiliary::ZkData;
use crate::verifier::MAX_CACHED_KEYS;

pub(crate) const DEFAULT_HOST_CHAIN_ID: i64 = 12345;
pub(crate) const SECOND_HOST_CHAIN_ID: i64 = 22345;
pub(crate) const UNKNOWN_HOST_CHAIN_ID: i64 = 999_999;
pub(crate) const SECOND_CHAIN_ACL_CONTRACT_ADDR: &str =
    "0x3333333333333333333333333333333333333333";

pub(crate) async fn setup_pool() -> anyhow::Result<(PostgresPoolManager, DBInstance)> {
    let _ = tracing_subscriber::fmt().json().with_level(true).try_init();
    let test_instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");

    let pool_mngr = PostgresPoolManager::connect_pool(
        test_instance.parent_token.child_token(),
        test_instance.db_url.as_str(),
        Duration::from_secs(15),
        10,
        Duration::from_secs(2),
        None,
    )
    .await
    .expect("pool manager created");

    sqlx::query("TRUNCATE TABLE verify_proofs")
        .execute(&pool_mngr.pool())
        .await
        .expect("verify_proofs truncated");

    Ok((pool_mngr, test_instance))
}

pub(crate) fn build_test_conf(
    database_url: DatabaseURL,
    host_chain_id: Option<i64>,
    pg_polling_interval: u32,
) -> crate::Config {
    crate::Config {
        database_url,
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval,
        worker_thread_count: 1,
        host_chain_id,
        pg_timeout: Duration::from_secs(15),
        pg_auto_explain_with_min_duration: None,
    }
}

pub(crate) fn spawn_worker(
    pool_mngr: PostgresPoolManager,
    conf: crate::Config,
) -> tokio::task::JoinHandle<Result<(), crate::ExecutionError>> {
    let last_active_at = Arc::new(RwLock::new(SystemTime::now()));
    tokio::spawn(async move {
        crate::verifier::execute_verify_proofs_loop(pool_mngr, conf, last_active_at).await
    })
}

pub(crate) async fn setup() -> anyhow::Result<(PostgresPoolManager, DBInstance)> {
    let (pool_mngr, test_instance) = setup_pool().await?;
    let conf = build_test_conf(test_instance.db_url.clone(), None, 60);
    let _worker = spawn_worker(pool_mngr.clone(), conf);

    sleep(Duration::from_secs(2)).await;

    Ok((pool_mngr, test_instance))
}

pub(crate) async fn insert_second_host_chain(pool: &sqlx::PgPool) {
    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, $2, $3)",
    )
    .bind(SECOND_HOST_CHAIN_ID)
    .bind("second test chain")
    .bind(SECOND_CHAIN_ACL_CONTRACT_ADDR)
    .execute(pool)
    .await
    .expect("insert second host chain");
}

pub(crate) async fn insert_valid_proof_for_chain(
    pool: &sqlx::PgPool,
    request_id: i64,
    host_chain_id: i64,
) -> i64 {
    let acl_contract_address = if host_chain_id == SECOND_HOST_CHAIN_ID {
        SECOND_CHAIN_ACL_CONTRACT_ADDR
    } else {
        ACL_CONTRACT_ADDR
    };

    let (mut aux, _) = aux_fixture(acl_contract_address.to_string());
    aux.chain_id = ChainId::try_from(host_chain_id).expect("valid test chain id");
    let aux_bytes = aux.assemble().expect("assemble aux");
    let proof = generate_sample_zk_pok(pool, &aux_bytes).await;
    insert_proof(pool, request_id, &proof, &aux)
        .await
        .expect("insert proof")
}

pub(crate) async fn assert_verified_is_null(pool: &sqlx::PgPool, request_id: i64) {
    let state: Option<bool> =
        sqlx::query_scalar("SELECT verified FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(request_id)
            .fetch_one(pool)
            .await
            .expect("read verify_proofs.verified");
    assert_eq!(state, None);
}

/// Checks if the proof is valid by querying the database continuously.
pub(crate) async fn is_valid(
    pool: &sqlx::PgPool,
    zk_proof_id: i64,
    max_retries: usize,
) -> Result<bool, sqlx::Error> {
    for _ in 0..max_retries {
        sleep(Duration::from_millis(100)).await;
        let result = sqlx::query!(
            "SELECT verified FROM verify_proofs WHERE zk_proof_id = $1",
            zk_proof_id
        )
        .fetch_one(pool)
        .await?;

        match result.verified {
            Some(verified) => return Ok(verified),
            None => continue,
        }
    }

    Ok(false)
}

#[derive(Debug, Clone)]
pub(crate) enum ZkInput {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

pub(crate) async fn generate_zk_pok_with_inputs(
    pool: &sqlx::PgPool,
    aux_data: &[u8],
    inputs: &[ZkInput],
) -> Vec<u8> {
    let db_key_cache = DbKeyCache::new(MAX_CACHED_KEYS).expect("create db key cache");

    let latest_key = db_key_cache.fetch_latest(pool).await.unwrap();

    let latest_crs = CrsCache::load(pool)
        .await
        .unwrap()
        .get_latest()
        .cloned()
        .unwrap();

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&latest_key.pks);
    for v in inputs {
        match *v {
            ZkInput::Bool(b) => builder.push(b),
            ZkInput::U8(x) => builder.push(x),
            ZkInput::U16(x) => builder.push(x),
            ZkInput::U32(x) => builder.push(x),
            ZkInput::U64(x) => builder.push(x),
        };
    }

    let the_list = builder
        .build_with_proof_packed(&latest_crs.crs, aux_data, tfhe::zk::ZkComputeLoad::Proof)
        .unwrap();

    safe_serialize(&the_list)
}

pub(crate) async fn generate_sample_zk_pok(pool: &sqlx::PgPool, aux_data: &[u8]) -> Vec<u8> {
    let inputs = vec![
        ZkInput::Bool(true),
        ZkInput::U8(42),
        ZkInput::U16(12345),
        ZkInput::U32(67890),
        ZkInput::U64(1234567890),
    ];
    generate_zk_pok_with_inputs(pool, aux_data, &inputs).await
}

pub(crate) async fn generate_empty_input_list(pool: &sqlx::PgPool, aux_data: &[u8]) -> Vec<u8> {
    let inputs = Vec::new();
    generate_zk_pok_with_inputs(pool, aux_data, &inputs).await
}

pub(crate) async fn insert_proof(
    pool: &sqlx::PgPool,
    request_id: i64,
    zk_pok: &[u8],
    aux: &ZkData,
) -> Result<i64, sqlx::Error> {
    //  Insert ZkPok into database
    sqlx::query(
            "INSERT INTO verify_proofs (zk_proof_id, input, host_chain_id, contract_address, user_address, verified)
            VALUES ($1, $2, $3, $4, $5, NULL )" 
        ).bind(request_id)
        .bind(zk_pok)
        .bind(aux.chain_id.as_i64())
        .bind(aux.contract_address.clone())
        .bind(aux.user_address.clone())
        .execute(pool).await?;

    // pg_notify to trigger the worker

    sqlx::query("SELECT pg_notify($1, '')")
        .bind("fhevm")
        .execute(pool)
        .await
        .unwrap();

    Ok(request_id)
}

pub(crate) fn aux_fixture(acl_contract_address: String) -> (ZkData, [u8; 92]) {
    // Define  20-byte addresses
    let contract_address = "0x1111111111111111111111111111111111111111".to_string();
    let user_address = "0x2222222222222222222222222222222222222222".to_string();
    let zk_data = ZkData {
        contract_address,
        user_address,
        acl_contract_address,
        chain_id: ChainId::try_from(DEFAULT_HOST_CHAIN_ID as u64).unwrap(),
    };

    (
        zk_data.clone(),
        zk_data.assemble().expect("Failed to assemble ZkData"),
    )
}
