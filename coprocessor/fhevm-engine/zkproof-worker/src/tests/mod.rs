use serial_test::serial;
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use test_harness::instance::{DBInstance, ImportMode};
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::MAX_INPUT_INDEX;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

mod utils;

const DEFAULT_HOST_CHAIN_ID: i64 = 12345;
const SECOND_HOST_CHAIN_ID: i64 = 22345;
const UNKNOWN_HOST_CHAIN_ID: i64 = 999_999;
const SECOND_CHAIN_ACL_CONTRACT_ADDR: &str = "0x3333333333333333333333333333333333333333";

async fn setup_without_worker() -> anyhow::Result<(PostgresPoolManager, DBInstance)> {
    let _ = tracing_subscriber::fmt().json().with_level(true).try_init();
    let instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");

    let pool_mngr = PostgresPoolManager::connect_pool(
        instance.parent_token.child_token(),
        instance.db_url.as_str(),
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

    Ok((pool_mngr, instance))
}

fn build_conf(
    database_url: fhevm_engine_common::utils::DatabaseURL,
    host_chain_id: Option<i64>,
) -> crate::Config {
    crate::Config {
        database_url,
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval: 1,
        worker_thread_count: 1,
        host_chain_id,
        pg_timeout: Duration::from_secs(15),
        pg_auto_explain_with_min_duration: None,
    }
}

fn spawn_worker(
    pool_mngr: PostgresPoolManager,
    conf: crate::Config,
) -> tokio::task::JoinHandle<Result<(), crate::ExecutionError>> {
    let last_active_at = Arc::new(RwLock::new(SystemTime::now()));
    tokio::spawn(async move {
        crate::verifier::execute_verify_proofs_loop(pool_mngr, conf, last_active_at).await
    })
}

async fn get_verified_state(pool: &sqlx::PgPool, request_id: i64) -> Option<bool> {
    sqlx::query_scalar("SELECT verified FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(request_id)
        .fetch_one(pool)
        .await
        .expect("read verify_proofs.verified")
}

async fn assert_remains_unverified(
    pool: &sqlx::PgPool,
    request_id: i64,
    checks: usize,
    sleep_ms: u64,
) {
    for _ in 0..checks {
        assert_eq!(get_verified_state(pool, request_id).await, None);
        sleep(Duration::from_millis(sleep_ms)).await;
    }
}

async fn insert_second_host_chain(pool: &sqlx::PgPool) {
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

async fn insert_valid_proof_for_chain(
    pool: &sqlx::PgPool,
    request_id: i64,
    chain_id: i64,
    acl_contract_address: &str,
) -> i64 {
    let (mut aux, _) = utils::aux_fixture(acl_contract_address.to_string());
    aux.chain_id = ChainId::try_from(chain_id).expect("valid test chain id");
    let aux_bytes = aux.assemble().expect("assemble aux");
    let proof = utils::generate_sample_zk_pok(pool, &aux_bytes).await;
    utils::insert_proof(pool, request_id, &proof, &aux)
        .await
        .expect("insert proof")
}

#[tokio::test]
#[serial(db)]
async fn test_verify_proof() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    // Generate Valid ZkPok
    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&pool, &aux.1).await;
    // Insert ZkPok into database
    let request_id_valid = utils::insert_proof(&pool, 101, &zk_pok, &aux.0)
        .await
        .unwrap();

    // Generate ZkPok with invalid aux data
    let mut aux = aux.0.clone();
    aux.user_address = "0x".to_owned() + &"1".repeat(40);
    let request_id_invalid = utils::insert_proof(&pool, 102, &zk_pok, &aux)
        .await
        .unwrap();

    let max_retries = 1000;

    // Check if it's valid
    assert!(utils::is_valid(&pool, request_id_valid, max_retries)
        .await
        .unwrap(),);

    // Check if it's invalid
    assert!(!utils::is_valid(&pool, request_id_invalid, max_retries)
        .await
        .unwrap());
}

#[tokio::test]
#[serial(db)]
async fn test_verify_empty_input_list() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let input = utils::generate_empty_input_list(&pool, &aux.1).await;
    let request_id = utils::insert_proof(&pool, 101, &input, &aux.0)
        .await
        .unwrap();

    let max_retries = 50;

    assert!(utils::is_valid(&pool, request_id, max_retries)
        .await
        .unwrap());
}

#[tokio::test]
#[serial(db)]
async fn test_max_input_index() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());

    // Ensure this fails because we exceed the MAX_INPUT_INDEX constraint
    let inputs = vec![utils::ZkInput::U8(1); MAX_INPUT_INDEX as usize + 2];

    assert!(!utils::is_valid(
        &pool,
        utils::insert_proof(
            &pool,
            101,
            &utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        5000
    )
    .await
    .expect("non-expired db query"));

    // Test with highest number of inputs - 255
    let inputs = vec![utils::ZkInput::U64(2); MAX_INPUT_INDEX as usize + 1];
    assert!(utils::is_valid(
        &pool,
        utils::insert_proof(
            &pool,
            102,
            &utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        5000
    )
    .await
    .expect("non-expired db query"));
}

#[tokio::test]
#[serial(db)]
async fn processes_only_configured_host_chain() {
    let (pool_mngr, instance) = setup_without_worker().await.expect("valid setup");
    let pool = pool_mngr.pool();

    insert_second_host_chain(&pool).await;

    let selected_chain_request_id =
        insert_valid_proof_for_chain(&pool, 201, DEFAULT_HOST_CHAIN_ID, ACL_CONTRACT_ADDR).await;
    let other_chain_request_id = insert_valid_proof_for_chain(
        &pool,
        202,
        SECOND_HOST_CHAIN_ID,
        SECOND_CHAIN_ACL_CONTRACT_ADDR,
    )
    .await;

    let worker = spawn_worker(
        pool_mngr.clone(),
        build_conf(instance.db_url.clone(), Some(DEFAULT_HOST_CHAIN_ID)),
    );

    assert!(utils::is_valid(&pool, selected_chain_request_id, 300)
        .await
        .expect("selected proof status"));
    assert_remains_unverified(&pool, other_chain_request_id, 15, 100).await;

    instance.parent_token.cancel();
    let worker_result = worker.await.expect("worker task join");
    assert!(worker_result.is_ok(), "worker result: {worker_result:?}");
}

#[tokio::test]
#[serial(db)]
async fn processes_all_chains_when_filter_unset() {
    let (pool_mngr, instance) = setup_without_worker().await.expect("valid setup");
    let pool = pool_mngr.pool();

    insert_second_host_chain(&pool).await;

    let first_request_id =
        insert_valid_proof_for_chain(&pool, 301, DEFAULT_HOST_CHAIN_ID, ACL_CONTRACT_ADDR).await;
    let second_request_id = insert_valid_proof_for_chain(
        &pool,
        302,
        SECOND_HOST_CHAIN_ID,
        SECOND_CHAIN_ACL_CONTRACT_ADDR,
    )
    .await;

    let worker = spawn_worker(pool_mngr.clone(), build_conf(instance.db_url.clone(), None));

    assert!(utils::is_valid(&pool, first_request_id, 300)
        .await
        .expect("first proof status"));
    assert!(utils::is_valid(&pool, second_request_id, 300)
        .await
        .expect("second proof status"));

    instance.parent_token.cancel();
    let worker_result = worker.await.expect("worker task join");
    assert!(worker_result.is_ok(), "worker result: {worker_result:?}");
}

#[tokio::test]
#[serial(db)]
async fn fails_startup_for_unknown_configured_host_chain() {
    let (pool_mngr, instance) = setup_without_worker().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let request_id =
        insert_valid_proof_for_chain(&pool, 401, DEFAULT_HOST_CHAIN_ID, ACL_CONTRACT_ADDR).await;

    let result = crate::verifier::execute_verify_proofs_loop(
        pool_mngr,
        build_conf(instance.db_url.clone(), Some(UNKNOWN_HOST_CHAIN_ID)),
        Arc::new(RwLock::new(SystemTime::now())),
    )
    .await;

    match result {
        Err(crate::ExecutionError::UnknownChainId(chain_id)) => {
            assert_eq!(chain_id, UNKNOWN_HOST_CHAIN_ID);
        }
        other => panic!("expected UnknownChainId({UNKNOWN_HOST_CHAIN_ID}), got {other:?}"),
    }

    assert_eq!(get_verified_state(&pool, request_id).await, None);
}
