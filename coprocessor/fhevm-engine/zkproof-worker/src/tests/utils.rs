use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::{tenant_keys, utils::safe_serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use test_harness::instance::{DBInstance, ImportMode};
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::auxiliary::ZkData;

pub async fn setup() -> anyhow::Result<(PostgresPoolManager, DBInstance)> {
    let _ = tracing_subscriber::fmt().json().with_level(true).try_init();
    let test_instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");

    let conf = crate::Config {
        database_url: test_instance.db_url.clone(),
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval: 60,
        worker_thread_count: 1,
        pg_timeout: Duration::from_secs(15),
        pg_auto_explain_with_min_duration: None,
    };

    let pool_mngr = PostgresPoolManager::connect_pool(
        test_instance.parent_token.child_token(),
        conf.database_url.as_str(),
        conf.pg_timeout,
        conf.pg_pool_connections,
        Duration::from_secs(2),
        conf.pg_auto_explain_with_min_duration,
    )
    .await
    .unwrap();

    let pmngr = pool_mngr.clone();

    sqlx::query("TRUNCATE TABLE verify_proofs")
        .execute(&pmngr.pool())
        .await
        .unwrap();

    let last_active_at = Arc::new(RwLock::new(SystemTime::now()));

    tokio::spawn(async move {
        crate::verifier::execute_verify_proofs_loop(pmngr, conf.clone(), last_active_at.clone())
            .await
            .unwrap();
    });

    sleep(Duration::from_secs(2)).await;

    Ok((pool_mngr, test_instance))
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
    let keys: Vec<tenant_keys::TfheTenantKeys> =
        tenant_keys::query_tenant_keys(vec![1], pool, true)
            .await
            .map_err(|e| {
                let e: Box<dyn std::error::Error> = e;
                e
            })
            .unwrap();
    let keys = &keys[0];

    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
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
        .build_with_proof_packed(
            &keys.public_params,
            aux_data,
            tfhe::zk::ZkComputeLoad::Proof,
        )
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
            "INSERT INTO verify_proofs (zk_proof_id, input, chain_id, contract_address, user_address, verified)
            VALUES ($1, $2, $3, $4, $5, NULL )" 
        ).bind(request_id)
        .bind(zk_pok)
        .bind(aux.chain_id)
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
        chain_id: 12345,
    };

    (
        zk_data.clone(),
        zk_data.assemble().expect("Failed to assemble ZkData"),
    )
}
