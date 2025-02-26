use std::time::Duration;

use fhevm_engine_common::{tenant_keys, utils::safe_serialize};
use test_harness::instance::DBInstance;
use tokio::time::sleep;

use crate::auxiliary::ZkData;

pub(crate) async fn setup() -> anyhow::Result<(sqlx::PgPool, DBInstance)> {
    tracing_subscriber::fmt().json().with_level(true).init();
    let test_instance = test_harness::instance::setup_test_db()
        .await
        .expect("valid db instance");

    let conf = crate::Config {
        database_url: test_instance.db_url().to_owned(),
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval: 60,
    };

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&conf.database_url)
        .await?;

    sqlx::query("TRUNCATE TABLE verify_proofs")
        .execute(&pool)
        .await
        .unwrap();

    tokio::spawn(async move {
        crate::verifier::execute_verify_proofs_loop(&conf)
            .await
            .unwrap();
    });

    sleep(Duration::from_secs(2)).await;

    Ok((pool, test_instance))
}

pub(crate) async fn is_valid(
    pool: &sqlx::PgPool,
    zk_proof_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT verified FROM verify_proofs WHERE zk_proof_id = $1",
        zk_proof_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.verified.unwrap_or(false))
}

pub(crate) async fn generate_zk_pok(
    pool: &sqlx::PgPool,
    aux_data: &[u8],
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

    println!("Building list");
    let mut builder = tfhe::ProvenCompactCiphertextList::builder(&keys.pks);
    let the_list = builder
        .push(false)
        .push(1u8)
        .push(2u16)
        .push(3u32)
        .push(4u64)
        .push(5u64)
        .build_with_proof_packed(
            &keys.public_params,
            aux_data,
            tfhe::zk::ZkComputeLoad::Proof,
        )
        .unwrap();

    safe_serialize(&the_list)
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

    sleep(Duration::from_secs(5)).await;

    Ok(request_id)
}

pub(crate) fn aux_fixture(acl_contract_address: String) -> (ZkData, [u8; 92]) {
    // Define  20-byte addresses
    let contract_address =
        "0x1111111111111111111111111111111111111111".to_string();
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
