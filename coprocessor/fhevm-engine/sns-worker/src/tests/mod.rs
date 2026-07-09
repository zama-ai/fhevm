use crate::{
    aws_upload::{enqueue_unverified_settled_publications, fetch_pending_uploads},
    executor::{garbage_collect, query_sns_tasks, Order},
    keyset::fetch_client_key,
    squash_noise::safe_deserialize,
    BigCiphertext, Ciphertext128Format, Config, DBConfig, S3Config, S3MigrationMode, S3RetryPolicy,
    SchedulePolicy, CURRENT_S3_FORMAT_VERSION, DEFAULT_S3_MIGRATION_MAX_RETRIES,
};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{B256, U256};
use anyhow::{anyhow, Ok};
use aws_config::BehaviorVersion;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextFormat, S3_METADATA_ATTESTATION_KEY,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::utils::{to_hex, DatabaseURL};
use fhevm_engine_common::{branch::advance_settled_height, db_keys::DbKeyId};
use serde::{Deserialize, Serialize};
use serial_test::serial;
use sqlx::Row;
use std::{
    fs::File,
    io::{Read, Write},
    str::FromStr,
    sync::{Arc, OnceLock},
    time::Duration,
};

use test_harness::{
    db_utils::truncate_tables,
    instance::{setup_test_db, DBInstance, ImportMode},
    localstack::{LocalstackContainer, LOCALSTACK_PORT},
    s3_utils,
};
use tfhe::{
    prelude::FheDecrypt, ClientKey, CompressedSquashedNoiseCiphertextList, SquashedNoiseFheUint,
};
use tokio::{
    sync::mpsc,
    time::{sleep, timeout},
};
use tracing::{info, Level};

const LISTEN_CHANNEL: &str = "sns_worker_chan";
static TRACING_INIT: OnceLock<()> = OnceLock::new();

mod s3_migration;
mod s3_migration_dry_run;

pub fn init_tracing() {
    TRACING_INIT.get_or_init(|| {
        tracing_subscriber::fmt().json().with_level(true).init();
    });
}

#[tokio::test]
#[ignore = "disabled in CI"]
async fn test_fhe_ciphertext128_with_compression() {
    const WITH_COMPRESSION: bool = true;
    let test_env = setup(WITH_COMPRESSION).await.expect("valid setup");
    let tf: TestFile = read_test_file("ciphertext64.json");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64.clone(),
        tf.cleartext,
        true,
        WITH_COMPRESSION,
    )
    .await
    .expect("test_fhe_ciphertext128_with_compression, first_fhe_computation = true");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64,
        tf.cleartext,
        false,
        WITH_COMPRESSION,
    )
    .await
    .expect("test_fhe_ciphertext128_with_compression, first_fhe_computation = false");
}

/// Tests batch execution of SnS computations with compression.
/// Inserts a batch of identical ciphertext64 entries with unique handles,
/// triggers the SNS worker to convert them, and verifies that all resulting
/// ciphertext128 entries are correctly computed and uploaded to S3.
#[tokio::test]
#[serial(db)]
async fn test_batch_execution() {
    const WITH_COMPRESSION: bool = true;
    let test_env = setup(WITH_COMPRESSION).await.expect("valid setup");
    let tf: TestFile = read_test_file("ciphertext64.json");

    let batch_size = std::env::var("BATCH_SIZE")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(100);

    info!("Batch size: {}", batch_size);

    run_batch_computations(
        &test_env,
        &tf.handle,
        batch_size,
        &tf.ciphertext64.clone(),
        tf.cleartext,
        WITH_COMPRESSION,
    )
    .await
    .expect("run_batch_computations should succeed");
}

#[tokio::test]
#[ignore = "disabled in CI"]
async fn test_fhe_ciphertext128_no_compression() {
    const NO_COMPRESSION: bool = false;
    let test_env = setup(NO_COMPRESSION).await.expect("valid setup");
    let tf: TestFile = read_test_file("ciphertext64.json");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64.clone(),
        tf.cleartext,
        true,
        NO_COMPRESSION,
    )
    .await
    .expect("test_decryptable, first_fhe_computation = true");
}

async fn test_decryptable(
    test_env: &TestEnvironment,
    handle: &Vec<u8>,
    ciphertext: &Vec<u8>,
    expected_result: i64,
    first_fhe_computation: bool, // first insert ciphertext64 in DB
    with_compression: bool,
) -> anyhow::Result<()> {
    let pool = &test_env.pool;

    clean_up(pool).await?;

    if first_fhe_computation {
        // insert into ciphertexts
        insert_ciphertext64(pool, handle, ciphertext).await?;
        insert_into_pbs_computations(pool, test_env.host_chain_id, handle).await?;
    } else {
        // insert into pbs_computations
        insert_into_pbs_computations(pool, test_env.host_chain_id, handle).await?;
        insert_ciphertext64(pool, handle, ciphertext).await?;
    }

    assert_ciphertext128(test_env, with_compression, handle, expected_result).await?;

    Ok(())
}

async fn run_batch_computations(
    test_env: &TestEnvironment,
    base_handle: &[u8],
    batch_size: u16,
    ciphertext: &Vec<u8>,
    expected_cleartext: i64,
    with_compression: bool,
) -> anyhow::Result<()> {
    let pool = &test_env.pool;
    let bucket128 = &test_env.conf.s3.bucket_ct128;
    let bucket64 = &test_env.conf.s3.bucket_ct64;

    clean_up(pool).await?;

    assert_ciphertext_s3_object_count(test_env, bucket128, 0i64).await;
    assert_ciphertext_s3_object_count(test_env, bucket64, 0i64).await;

    info!(batch_size, "Inserting ciphertexts ...");

    let mut handles = Vec::new();
    let host_chain_id = test_env.host_chain_id;
    for i in 0..batch_size {
        let mut handle = base_handle.to_owned();

        // Modify first two bytes of the handle to make it unique
        // However the ciphertext64 will be the same
        handle[0] = (i >> 8) as u8;
        handle[1] = (i & 0xFF) as u8;
        test_harness::db_utils::insert_ciphertext64(pool, &handle, ciphertext).await?;
        test_harness::db_utils::insert_into_pbs_computations(pool, host_chain_id, &handle).await?;
        handles.push(handle);
    }

    info!(batch_size, "Inserted batch");

    // Send notification only after the batch was fully inserted
    // NB. Use db transaction instead
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(LISTEN_CHANNEL)
        .execute(pool)
        .await?;

    info!("Sent pg_notify to SnS worker");

    let start = std::time::Instant::now();
    let mut set = tokio::task::JoinSet::new();
    for handle in handles.iter() {
        let test_env = test_env.clone();
        let handle = handle.clone();
        set.spawn(async move {
            assert_ciphertext128(&test_env, with_compression, &handle, expected_cleartext).await
        });
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    let elapsed = start.elapsed();
    info!(elapsed = ?elapsed, batch_size, "Batch execution completed");

    // Assert that all ciphertext objects are uploaded to S3
    assert_ciphertext_s3_object_count(test_env, bucket128, batch_size as i64 + 1).await;
    assert_ciphertext_s3_object_count(test_env, bucket64, batch_size as i64).await;

    anyhow::Result::<()>::Ok(())
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_lifo_mode() {
    init_tracing();

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .unwrap();

    const HANDLES_COUNT: usize = 30;
    const BATCH_SIZE: usize = 4;
    let key_id_gw: DbKeyId = vec![0u8; 32];

    let host_chain_id: i64 = 1;
    for i in 0..HANDLES_COUNT {
        // insert into ciphertexts
        test_harness::db_utils::insert_ciphertext64(
            &pool,
            &Vec::from([i as u8; 32]),
            &Vec::from([i as u8; 32]),
        )
        .await
        .unwrap();

        test_harness::db_utils::insert_into_pbs_computations(
            &pool,
            host_chain_id,
            &Vec::from([i as u8; 32]),
        )
        .await
        .unwrap();
    }

    let mut trx = pool.begin().await.unwrap();
    if let Result::Ok(Some(tasks)) =
        query_sns_tasks(&mut trx, BATCH_SIZE as u32, 0, Order::Desc, &key_id_gw).await
    {
        assert!(
            tasks.len() == BATCH_SIZE,
            "Expected {} tasks, got {}",
            BATCH_SIZE,
            tasks.len()
        );

        // print handles of tasks
        for (i, task) in tasks.iter().enumerate() {
            assert!(
                task.handle == [(HANDLES_COUNT - (i + 1)) as u8; 32],
                "Task (desc) handle does not match expected value"
            );

            info!("Desc Task handle: {}", hex::encode(&task.handle));
        }
    } else {
        panic!("No tasks found in Desc order");
    }

    let mut trx = pool.begin().await.unwrap();
    if let Result::Ok(Some(tasks)) =
        query_sns_tasks(&mut trx, BATCH_SIZE as u32, 0, Order::Asc, &key_id_gw).await
    {
        assert!(
            tasks.len() == BATCH_SIZE,
            "Expected {} tasks, got {}",
            BATCH_SIZE,
            tasks.len()
        );

        // print handles of tasks
        for (i, task) in tasks.iter().enumerate() {
            assert!(
                task.handle == [i as u8; 32],
                "Task (asc) handle does not match expected value"
            );
            info!("Asc Task handle: {}", to_hex(&task.handle));
        }
    } else {
        panic!("No tasks found in Asc order");
    }
}

/// Reorg cleanup deletes the pbs_computations and ciphertext_digest rows of
/// handles that lived solely on an orphaned fork. An sns task that fetched
/// its work before the cleanup must not resurrect the digest row afterwards
/// (that would drive a phantom addCiphertextMaterial publication), and a
/// mark-uploaded landing after the cleanup must be a no-op, not an error.
#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn enqueue_upload_task_skips_after_reorg_cleanup() {
    init_tracing();

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .unwrap();

    // Persistent-DB (COPROCESSOR_TEST_LOCALHOST) runs reuse the database:
    // start from a clean slate so query_sns_tasks below fetches OUR row and
    // this test leaves nothing behind for the next one.
    clean_up(&pool).await.unwrap();

    let host_chain_id: i64 = 1;
    let handle = vec![0x42u8; 32];
    let key_id_gw: DbKeyId = vec![0u8; 32];

    test_harness::db_utils::insert_ciphertext64(&pool, &handle, &vec![0xAB; 32])
        .await
        .unwrap();
    test_harness::db_utils::insert_into_pbs_computations(&pool, host_chain_id, &handle)
        .await
        .unwrap();

    // Acquire the task the same way the worker does, then release the lock.
    let mut trx = pool.begin().await.unwrap();
    let task = query_sns_tasks(&mut trx, 1, 0, Order::Asc, &key_id_gw)
        .await
        .unwrap()
        .expect("one task")
        .remove(0);
    trx.rollback().await.unwrap();

    // Live provenance: the digest row is enqueued.
    let mut trx = pool.begin().await.unwrap();
    assert!(task.enqueue_upload_task(&mut trx).await.unwrap());
    trx.commit().await.unwrap();
    let digest_rows = || async {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1")
            .bind(&handle)
            .fetch_one(&pool)
            .await
            .unwrap()
    };
    assert_eq!(digest_rows().await, 1);

    // Simulate the reorg cleanup for an orphan-only handle.
    for sql in [
        "DELETE FROM pbs_computations WHERE handle = $1",
        "DELETE FROM ciphertext_digest WHERE handle = $1",
    ] {
        sqlx::query(sql).bind(&handle).execute(&pool).await.unwrap();
    }

    // The in-flight task must not resurrect the digest row...
    let mut trx = pool.begin().await.unwrap();
    assert!(!task.enqueue_upload_task(&mut trx).await.unwrap());
    trx.commit().await.unwrap();
    assert_eq!(digest_rows().await, 0, "digest row must not be resurrected");

    // ...and a late mark-uploaded is a cancelled no-op, not an error.
    let mut trx = pool.begin().await.unwrap();
    task.mark_ciphertexts_uploaded(&mut trx, vec![0xC1; 32], vec![0xC2; 32], 1)
        .await
        .expect("mark after cleanup must be a no-op");
    trx.commit().await.unwrap();
    assert_eq!(digest_rows().await, 0);

    // Bridge-retraction variant: the pbs row survives (allow events created
    // it) but the copied ciphertexts row was retracted. The ciphertext
    // witness must veto the enqueue on its own.
    test_harness::db_utils::insert_into_pbs_computations(&pool, host_chain_id, &handle)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ciphertexts WHERE handle = $1")
        .bind(&handle)
        .execute(&pool)
        .await
        .unwrap();
    let mut trx = pool.begin().await.unwrap();
    assert!(
        !task.enqueue_upload_task(&mut trx).await.unwrap(),
        "missing ciphertexts row must veto the enqueue"
    );
    trx.commit().await.unwrap();
    assert_eq!(digest_rows().await, 0);

    // Leave the shared (localhost-mode) database as we found it.
    clean_up(&pool).await.unwrap();
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_query_sns_tasks_reads_branch_rows() {
    init_tracing();

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .unwrap();

    let host_chain_id: i64 = 1;
    let key_id_gw: DbKeyId = vec![0u8; 32];
    let first_handle = vec![0x09_u8; 32];
    let second_handle = vec![0x0A_u8; 32];
    let branchful_without_exact_handle = vec![0x0B_u8; 32];
    let branchless_handle = vec![0x0C_u8; 32];
    let branchful_null_block_handle = vec![0x0D_u8; 32];
    let branchless_producer_pre_cutover_handle = vec![0x0E_u8; 32];
    let first_hash = vec![0x19_u8; 32];
    let second_hash = vec![0x1A_u8; 32];
    let missing_exact_hash = vec![0x1B_u8; 32];
    let null_block_hash = vec![0x1C_u8; 32];
    let first_event_hash = vec![0x29_u8; 32];
    let second_event_hash = vec![0x2A_u8; 32];
    let missing_event_hash = vec![0x2B_u8; 32];
    let null_block_event_hash = vec![0x2C_u8; 32];
    let branchless_producer_event_hash = vec![0x2D_u8; 32];

    for (handle, producer_block_hash, block_hash, block_number) in [
        (&first_handle, &first_hash, &first_event_hash, 9_i64),
        (&second_handle, &second_hash, &second_event_hash, 10_i64),
    ] {
        sqlx::query(
            "INSERT INTO pbs_computations_branch(
                handle, host_chain_id, block_number, producer_block_hash, block_hash
             )
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(handle)
        .bind(host_chain_id)
        .bind(block_number)
        .bind(producer_block_hash)
        .bind(block_hash)
        .execute(&pool)
        .await
        .expect("insert pbs_computations_branch");

        sqlx::query(
            "INSERT INTO ciphertexts_branch(
                handle, producer_block_hash, block_number, ciphertext, ciphertext_version, ciphertext_type
             )
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(handle)
        .bind(producer_block_hash)
        .bind(block_number)
        .bind(vec![0x42_u8; 32])
        .bind(current_ciphertext_version())
        .bind(0_i16)
        .execute(&pool)
        .await
        .expect("insert ciphertexts_branch");
    }

    sqlx::query(
        "INSERT INTO pbs_computations_branch(
            handle, host_chain_id, block_number, producer_block_hash, block_hash
         )
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&branchful_without_exact_handle)
    .bind(host_chain_id)
    .bind(11_i64)
    .bind(&missing_exact_hash)
    .bind(&missing_event_hash)
    .execute(&pool)
    .await
    .expect("insert branchful pbs_computations_branch without exact ciphertext");
    sqlx::query(
        "INSERT INTO ciphertexts_branch(
            handle, producer_block_hash, ciphertext, ciphertext_version, ciphertext_type
         )
         VALUES ($1, ''::bytea, $2, $3, $4)",
    )
    .bind(&branchful_without_exact_handle)
    .bind(vec![0x43_u8; 32])
    .bind(current_ciphertext_version())
    .bind(0_i16)
    .execute(&pool)
    .await
    .expect("insert branchless ciphertext candidate");

    sqlx::query(
        "INSERT INTO pbs_computations_branch(
            handle, host_chain_id, block_number, producer_block_hash
         )
         VALUES ($1, $2, NULL, ''::bytea)",
    )
    .bind(&branchless_handle)
    .bind(host_chain_id)
    .execute(&pool)
    .await
    .expect("insert branchless pbs_computations_branch");
    sqlx::query(
        "INSERT INTO ciphertexts_branch(
            handle, producer_block_hash, ciphertext, ciphertext_version, ciphertext_type
         )
         VALUES ($1, ''::bytea, $2, $3, $4)",
    )
    .bind(&branchless_handle)
    .bind(vec![0x44_u8; 32])
    .bind(current_ciphertext_version())
    .bind(0_i16)
    .execute(&pool)
    .await
    .expect("insert branchless ciphertext");

    let branchful_null_insert = sqlx::query(
        "INSERT INTO pbs_computations_branch(
            handle, host_chain_id, block_number, producer_block_hash, block_hash
         )
         VALUES ($1, $2, NULL, $3, $4)",
    )
    .bind(&branchful_null_block_handle)
    .bind(host_chain_id)
    .bind(&null_block_hash)
    .bind(&null_block_event_hash)
    .execute(&pool)
    .await;
    assert!(
        branchful_null_insert.is_err(),
        "branchful pbs rows must carry a producer block number"
    );

    sqlx::query(
        "INSERT INTO pbs_computations_branch(
            handle, host_chain_id, block_number, producer_block_hash, block_hash
         )
         VALUES ($1, $2, 9, ''::bytea, $3)",
    )
    .bind(&branchless_producer_pre_cutover_handle)
    .bind(host_chain_id)
    .bind(&branchless_producer_event_hash)
    .execute(&pool)
    .await
    .expect("insert pre-cutover pbs row consuming branchless input");
    sqlx::query(
        "INSERT INTO ciphertexts_branch(
            handle, producer_block_hash, ciphertext, ciphertext_version, ciphertext_type
         )
         VALUES ($1, ''::bytea, $2, $3, $4)",
    )
    .bind(&branchless_producer_pre_cutover_handle)
    .bind(vec![0x46_u8; 32])
    .bind(current_ciphertext_version())
    .bind(0_i16)
    .execute(&pool)
    .await
    .expect("insert branchless ciphertext for pre-cutover pbs row");

    let mut trx = pool.begin().await.unwrap();
    let tasks = query_sns_tasks(&mut trx, 10, 0, Order::Asc, &key_id_gw)
        .await
        .expect("query_sns_tasks")
        .expect("branch tasks");

    assert_eq!(tasks.len(), 4);
    assert_eq!(tasks[0].handle, first_handle);
    assert_eq!(tasks[0].block_number, Some(9));
    assert_eq!(tasks[0].producer_block_hash, first_hash);
    assert_eq!(tasks[0].block_hash, first_event_hash);
    assert_eq!(tasks[1].handle, second_handle);
    assert_eq!(tasks[1].block_number, Some(10));
    assert_eq!(tasks[1].producer_block_hash, second_hash);
    assert_eq!(tasks[1].block_hash, second_event_hash);
    assert!(tasks
        .iter()
        .all(|task| task.handle != branchful_without_exact_handle));
    assert!(tasks
        .iter()
        .all(|task| task.handle != branchful_null_block_handle));
    assert!(tasks
        .iter()
        .any(|task| task.handle == branchless_producer_pre_cutover_handle));
    let branchless_task = tasks
        .iter()
        .find(|task| task.handle == branchless_handle)
        .expect("branchless PBS row should select branchless ciphertext");
    assert_eq!(branchless_task.producer_block_hash, Vec::<u8>::new());
    assert_eq!(branchless_task.block_hash, Vec::<u8>::new());

    let upload_task = tasks[0].clone();
    trx.rollback().await.expect("rollback zero-cutover fetch");

    let mut limit_trx = pool.begin().await.unwrap();
    let limited_post_cutover_tasks = query_sns_tasks(&mut limit_trx, 1, 10, Order::Asc, &key_id_gw)
        .await
        .expect("limited query_sns_tasks with nonzero cutover")
        .expect("limited post-cutover branch task");
    limit_trx
        .rollback()
        .await
        .expect("rollback limited post-cutover fetch");
    assert_eq!(limited_post_cutover_tasks.len(), 1);
    assert_eq!(limited_post_cutover_tasks[0].handle, first_handle);

    let mut cutover_trx = pool.begin().await.unwrap();
    let post_cutover_tasks = query_sns_tasks(&mut cutover_trx, 10, 10, Order::Asc, &key_id_gw)
        .await
        .expect("query_sns_tasks with nonzero cutover")
        .expect("post-cutover branch tasks");
    cutover_trx
        .rollback()
        .await
        .expect("rollback post-cutover fetch");
    assert_eq!(post_cutover_tasks.len(), 4);
    assert!(
        post_cutover_tasks
            .iter()
            .all(|task| task.producer_block_hash.is_empty() || task.block_number.is_some()),
        "sns-worker must keep branchless and pre-cutover rows drainable"
    );
    assert!(post_cutover_tasks
        .iter()
        .any(|task| task.handle == first_handle));
    assert!(post_cutover_tasks
        .iter()
        .all(|task| task.handle != branchful_null_block_handle));
    assert!(post_cutover_tasks
        .iter()
        .any(|task| task.handle == branchless_producer_pre_cutover_handle));
    assert!(post_cutover_tasks
        .iter()
        .any(|task| task.handle == second_handle));
    assert!(post_cutover_tasks
        .iter()
        .any(|task| task.handle == branchless_handle));

    let mut trx = pool.begin().await.unwrap();
    let mut upload_task = upload_task;
    upload_task.ct128 = Arc::new(BigCiphertext::new(
        vec![0x55_u8; 32],
        Ciphertext128Format::CompressedOnCpu,
    ));
    upload_task
        .enqueue_upload_task(&mut trx)
        .await
        .expect("enqueue upload task");

    let sibling_event_hash = vec![0x2E_u8; 32];
    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id, key_id_gw, handle, producer_block_hash, block_hash, block_number
         )
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&upload_task.handle)
    .bind(&upload_task.producer_block_hash)
    .bind(&sibling_event_hash)
    .bind(upload_task.block_number)
    .execute(trx.as_mut())
    .await
    .expect("insert sibling digest row");

    let ct64_digest = vec![0x77_u8; 32];
    let ct128_digest = vec![0x88_u8; 32];
    let marked = upload_task
        .mark_ciphertexts_uploaded(
            &mut trx,
            ct64_digest.clone(),
            ct128_digest.clone(),
            CURRENT_S3_FORMAT_VERSION,
        )
        .await
        .expect("mark exact digest row uploaded");
    assert!(marked);

    let rows = sqlx::query(
        "SELECT block_hash, ciphertext, ciphertext128
         FROM ciphertext_digest_branch
         WHERE handle = $1
           AND producer_block_hash = $2",
    )
    .bind(&upload_task.handle)
    .bind(&upload_task.producer_block_hash)
    .fetch_all(trx.as_mut())
    .await
    .expect("fetch digest rows");
    assert_eq!(rows.len(), 2);
    for row in rows {
        let block_hash: Vec<u8> = row.try_get("block_hash").expect("block_hash");
        let ciphertext: Option<Vec<u8>> = row.try_get("ciphertext").expect("ciphertext");
        let ciphertext128: Option<Vec<u8>> = row.try_get("ciphertext128").expect("ciphertext128");
        if block_hash == upload_task.block_hash {
            assert_eq!(ciphertext, Some(ct64_digest.clone()));
            assert_eq!(ciphertext128, Some(ct128_digest.clone()));
        } else {
            assert_eq!(block_hash, sibling_event_hash);
            assert!(ciphertext.is_none());
            assert!(ciphertext128.is_none());
        }
    }
}

#[tokio::test]
#[serial(db)]
async fn stale_upload_mark_enqueues_canonical_repair_target() {
    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .expect("connect test db");
    let pool = &pool;
    clean_up(pool).await.expect("clean db");

    let host_chain_id = 1_i64;
    let key_id_gw = vec![0x01_u8; 32];
    let handle = vec![0xA0_u8; 32];
    let stale_producer = vec![0xA1_u8; 32];
    let stale_event = vec![0xA2_u8; 32];
    let canonical_producer = vec![0xB1_u8; 32];
    let canonical_event = vec![0xB2_u8; 32];

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid(chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, 20, 'orphaned'), ($1, $3, 20, 'orphaned'), ($1, $4, 20, 'finalized'), ($1, $5, 20, 'finalized')",
    )
    .bind(host_chain_id)
    .bind(&stale_producer)
    .bind(&stale_event)
    .bind(&canonical_producer)
    .bind(&canonical_event)
    .execute(pool)
    .await
    .expect("insert block statuses");

    for (producer, event) in [
        (&stale_producer, &stale_event),
        (&canonical_producer, &canonical_event),
    ] {
        sqlx::query(
            "INSERT INTO ciphertext_digest_branch(
                host_chain_id, key_id_gw, handle, producer_block_hash, block_hash, block_number
             )
             VALUES ($1, $2, $3, $4, $5, 20)",
        )
        .bind(host_chain_id)
        .bind(&key_id_gw)
        .bind(&handle)
        .bind(producer)
        .bind(event)
        .execute(pool)
        .await
        .expect("insert digest row");
    }

    let stale_task = crate::HandleItem {
        host_chain_id: fhevm_engine_common::chain_id::ChainId::try_from(host_chain_id).unwrap(),
        key_id_gw: key_id_gw.clone(),
        handle: handle.clone(),
        producer_block_hash: stale_producer.clone(),
        block_hash: stale_event.clone(),
        block_number: Some(20),
        ct64_compressed: Arc::new(vec![0x11_u8; 32]),
        ct128: Arc::new(BigCiphertext::new(Vec::new(), Ciphertext128Format::Unknown)),
        ct64_digest: None,
        ct128_digest: None,
        s3_format_version: None,
        span: tracing::Span::none(),
        transaction_id: None,
    };

    let mut trx = pool.begin().await.expect("begin tx");
    assert!(!stale_task
        .is_upload_publishable(&mut trx)
        .await
        .expect("publishability check"));
    let marked = stale_task
        .mark_ciphertexts_uploaded(
            &mut trx,
            vec![0x55_u8; 32],
            vec![0_u8; 32],
            CURRENT_S3_FORMAT_VERSION,
        )
        .await
        .expect("stale mark should not error");
    assert!(!marked);
    assert!(stale_task
        .enqueue_canonical_repair(&mut trx, "test_stale_upload")
        .await
        .expect("enqueue repair"));
    trx.commit().await.expect("commit repair");

    let repair = sqlx::query(
        "SELECT target_producer_block_hash, target_block_hash
         FROM s3_canonical_repair_queue
         WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(host_chain_id)
    .bind(&handle)
    .fetch_one(pool)
    .await
    .expect("repair row");
    let target_producer: Vec<u8> = repair.try_get("target_producer_block_hash").unwrap();
    let target_event: Vec<u8> = repair.try_get("target_block_hash").unwrap();
    assert_eq!(target_producer, canonical_producer);
    assert_eq!(target_event, canonical_event);
}

#[tokio::test]
#[serial(db)]
async fn canonical_repair_queue_is_returned_as_upload_work() {
    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .expect("connect test db");
    let pool = &pool;
    clean_up(pool).await.expect("clean db");

    let host_chain_id = 1_i64;
    let key_id_gw = vec![0x02_u8; 32];
    let handle = vec![0xC0_u8; 32];
    let producer = vec![0xC1_u8; 32];
    let event = vec![0xC2_u8; 32];
    let ct64 = vec![0xC3_u8; 32];
    let ct64_digest = vec![0xC4_u8; 32];

    sqlx::query(
        "INSERT INTO ciphertexts_branch(
            handle, producer_block_hash, block_number, ciphertext, ciphertext_version, ciphertext_type
         )
         VALUES ($1, $2, 30, $3, $4, 0)",
    )
    .bind(&handle)
    .bind(&producer)
    .bind(&ct64)
    .bind(current_ciphertext_version())
    .execute(pool)
    .await
    .expect("insert ct64 bytes");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id, key_id_gw, handle, producer_block_hash, block_hash, block_number,
            ciphertext, ciphertext128, s3_format_version
         )
         VALUES ($1, $2, $3, $4, $5, 30, $6, $7, $8)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&handle)
    .bind(&producer)
    .bind(&event)
    .bind(&ct64_digest)
    .bind(vec![0_u8; 32])
    .bind(CURRENT_S3_FORMAT_VERSION)
    .execute(pool)
    .await
    .expect("insert complete digest row");

    sqlx::query(
        "INSERT INTO s3_canonical_repair_queue(
            host_chain_id, handle, target_producer_block_hash, target_block_hash,
            target_block_number, reason
         )
         VALUES ($1, $2, $3, $4, 30, 'test')",
    )
    .bind(host_chain_id)
    .bind(&handle)
    .bind(&producer)
    .bind(&event)
    .execute(pool)
    .await
    .expect("insert repair row");

    let jobs = fetch_pending_uploads(pool, 10)
        .await
        .expect("fetch pending repairs");
    assert_eq!(jobs.len(), 1);
    match &jobs[0] {
        crate::UploadJob::Normal(item) => {
            assert_eq!(item.handle, handle);
            assert_eq!(item.producer_block_hash, producer);
            assert_eq!(item.block_hash, event);
            assert_eq!(item.ct64_compressed.as_ref(), &ct64);
            assert_eq!(item.ct64_digest.as_deref(), Some(ct64_digest.as_slice()));
        }
        crate::UploadJob::DatabaseLock(_) => panic!("repair work should use normal upload path"),
    }

    let (locked, attempts): (bool, i32) = sqlx::query_as(
        "SELECT locked_at IS NOT NULL, attempts
         FROM s3_canonical_repair_queue
         WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(host_chain_id)
    .bind(&handle)
    .fetch_one(pool)
    .await
    .expect("repair lock timestamp");
    assert!(locked);
    assert_eq!(attempts, 1);
}

#[tokio::test]
#[serial(db)]
async fn unverified_settled_publication_is_enqueued_for_repair() {
    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .expect("connect test db");
    let pool = &pool;
    clean_up(pool).await.expect("clean db");

    let host_chain_id = 1_i64;
    let key_id_gw = vec![0x03_u8; 32];
    let handle = vec![0xD0_u8; 32];
    let producer = vec![0xD1_u8; 32];
    let event = vec![0xD2_u8; 32];
    let ct64_digest = vec![0xD3_u8; 32];

    sqlx::query(
        "INSERT INTO coprocessor_settlement(chain_id, settled_height)
         VALUES ($1, 40)",
    )
    .bind(host_chain_id)
    .execute(pool)
    .await
    .expect("insert settlement");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id, key_id_gw, handle, producer_block_hash, block_hash, block_number,
            ciphertext, ciphertext128, s3_format_version
         )
         VALUES ($1, $2, $3, $4, $5, 40, $6, $7, $8)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&handle)
    .bind(&producer)
    .bind(&event)
    .bind(&ct64_digest)
    .bind(vec![0_u8; 32])
    .bind(CURRENT_S3_FORMAT_VERSION)
    .execute(pool)
    .await
    .expect("insert complete digest row");

    let enqueued = enqueue_unverified_settled_publications(pool, 10)
        .await
        .expect("enqueue unverified settled publication");
    assert_eq!(enqueued, 1);

    let target: (Vec<u8>, Vec<u8>) = sqlx::query_as(
        "SELECT target_producer_block_hash, target_block_hash
         FROM s3_canonical_repair_queue
         WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(host_chain_id)
    .bind(&handle)
    .fetch_one(pool)
    .await
    .expect("repair queue target");
    assert_eq!(target, (producer, event));
}

#[tokio::test]
#[serial(db)]
async fn settlement_waits_for_verified_s3_publication_marker() {
    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(test_instance.db_url())
        .await
        .expect("connect test db");
    let pool = &pool;
    clean_up(pool).await.expect("clean db");

    let host_chain_id = 1_i64;
    let key_id_gw = vec![0x04_u8; 32];
    let handle = vec![0xE0_u8; 32];
    let producer = vec![0xE1_u8; 32];
    let event = vec![0xE2_u8; 32];
    let ct64_digest = vec![0xE3_u8; 32];

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid(chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, 10, 'finalized')",
    )
    .bind(host_chain_id)
    .bind(&event)
    .execute(pool)
    .await
    .expect("insert finalized block");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id, key_id_gw, handle, producer_block_hash, block_hash, block_number,
            ciphertext, ciphertext128, s3_format_version
         )
         VALUES ($1, $2, $3, $4, $5, 10, $6, $7, $8)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&handle)
    .bind(&producer)
    .bind(&event)
    .bind(&ct64_digest)
    .bind(vec![0_u8; 32])
    .bind(CURRENT_S3_FORMAT_VERSION)
    .execute(pool)
    .await
    .expect("insert complete digest row");

    let mut tx = pool.begin().await.expect("begin settlement tx");
    let settled = advance_settled_height(&mut tx, host_chain_id, 10, 10)
        .await
        .expect("advance unsettled");
    tx.commit().await.expect("commit settlement");
    assert_eq!(settled, 9);

    sqlx::query(
        "UPDATE ciphertext_digest_branch
         SET s3_publication_verified_at = NOW(),
             s3_publication_verified_digest = ciphertext,
             s3_publication_verified_producer_block_hash = producer_block_hash
         WHERE host_chain_id = $1 AND handle = $2",
    )
    .bind(host_chain_id)
    .bind(&handle)
    .execute(pool)
    .await
    .expect("mark publication verified");

    let mut tx = pool.begin().await.expect("begin settlement tx");
    let settled = advance_settled_height(&mut tx, host_chain_id, 10, 10)
        .await
        .expect("advance verified");
    tx.commit().await.expect("commit settlement");
    assert_eq!(settled, 10);
}

#[tokio::test]
#[serial(db)]
#[cfg(not(feature = "gpu"))]
async fn test_garbage_collect() {
    init_tracing();

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");

    const CONCURRENT_TASKS: usize = 20;
    const HANDLES_COUNT: u32 = 1000;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(CONCURRENT_TASKS as u32)
        .connect(test_instance.db_url())
        .await
        .unwrap();

    clean_up(&pool).await.unwrap();

    let host_chain_id: i64 = 1;
    let key_id_gw: Vec<u8> = vec![0u8; 32];
    for i in 0..HANDLES_COUNT {
        // insert into ciphertexts
        let mut handle = [0u8; 32];
        handle[..4].copy_from_slice(&i.to_le_bytes());

        let producer_block_hash = vec![i as u8; 32];
        let _ = sqlx::query(
            "INSERT INTO ciphertexts128_branch(handle, producer_block_hash, block_number, ciphertext)
                VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING;",
        )
        .bind(&handle[..])
        .bind(&producer_block_hash)
        .bind(i as i64 + 1)
        .bind(&[i as u8; 32][..])
        .execute(&pool)
        .await
        .expect("insert into ciphertexts128_branch");

        let _ = sqlx::query(
            "INSERT INTO ciphertext_digest_branch(host_chain_id, key_id_gw, handle, producer_block_hash, block_number, ciphertext, ciphertext128)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT DO NOTHING;",
        )
        .bind(host_chain_id)
        .bind(&key_id_gw)
        .bind(&handle[..])
        .bind(&producer_block_hash)
        .bind(i as i64 + 1)
        .bind(&[i as u8; 32][..])
        .bind(&[i as u8; 32][..])
        .execute(&pool)
        .await
        .expect("insert into ciphertext_digest_branch");
    }

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ciphertexts128_branch where ciphertext IS not NULL",
    )
    .fetch_one(&pool)
    .await
    .expect("count ciphertexts128_branch");
    assert_eq!(
        count, HANDLES_COUNT as i64,
        "ciphertext128 should not be empty before garbage_collect"
    );

    let partial_handle = vec![0xffu8; 32];
    let partial_producer_block_hash = vec![0xeeu8; 32];
    let partial_ciphertext = vec![0xffu8; 32];
    let partial_digest = vec![0xffu8; 32];
    let complete_block_hash = vec![0x11u8; 32];
    let incomplete_block_hash = vec![0x12u8; 32];
    sqlx::query(
        "INSERT INTO ciphertexts128_branch(handle, producer_block_hash, block_number, ciphertext)
            VALUES ($1, $2, 99, $3)
        ON CONFLICT DO NOTHING;",
    )
    .bind(&partial_handle)
    .bind(&partial_producer_block_hash)
    .bind(&partial_ciphertext)
    .execute(&pool)
    .await
    .expect("insert partial ciphertexts128_branch");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(host_chain_id, key_id_gw, handle, producer_block_hash, block_number, ciphertext128)
            VALUES ($1, $2, $3, $4, 99, $5)
        ON CONFLICT DO NOTHING;",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&partial_handle)
    .bind(&partial_producer_block_hash)
    .bind(&partial_digest)
    .execute(&pool)
    .await
    .expect("insert partial ciphertext_digest_branch");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id,
            key_id_gw,
            handle,
            producer_block_hash,
            block_hash,
            block_number,
            ciphertext,
            ciphertext128
        )
        VALUES ($1, $2, $3, $4, $5, 99, $6, $7)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&partial_handle)
    .bind(&partial_producer_block_hash)
    .bind(&complete_block_hash)
    .bind(&partial_digest)
    .bind(&partial_digest)
    .execute(&pool)
    .await
    .expect("insert complete sibling ciphertext_digest_branch");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id,
            key_id_gw,
            handle,
            producer_block_hash,
            block_hash,
            block_number,
            ciphertext
        )
        VALUES ($1, $2, $3, $4, $5, 99, $6)",
    )
    .bind(host_chain_id)
    .bind(&key_id_gw)
    .bind(&partial_handle)
    .bind(&partial_producer_block_hash)
    .bind(&incomplete_block_hash)
    .bind(&partial_digest)
    .execute(&pool)
    .await
    .expect("insert incomplete sibling ciphertext_digest_branch");

    let handles: Vec<_> = (0..CONCURRENT_TASKS)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                garbage_collect(&pool, 100)
                    .await
                    .expect("garbage_collect should succeed");
            })
        })
        .collect();

    // Wait for all tasks to complete or a timeout
    let res_ = tokio::time::timeout(Duration::from_secs(10), async {
        for handle in handles {
            handle.await.expect("Task failed");
        }
    })
    .await;

    assert!(
        res_.is_ok(),
        "garbage_collect tasks did not complete in time"
    );

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts128_branch")
        .fetch_one(&pool)
        .await
        .expect("ciphertexts128_branch has been GCd");
    assert!(
        count <= 100,
        "ciphertext128 should have less entries than threshold after garbage_collect"
    );

    let partial_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertexts128_branch
         WHERE handle = $1
           AND producer_block_hash = $2",
    )
    .bind(&partial_handle)
    .bind(&partial_producer_block_hash)
    .fetch_one(&pool)
    .await
    .expect("partial ciphertexts128_branch should remain queryable");
    assert_eq!(
        partial_count, 1,
        "garbage_collect should keep ct128 material until both upload digests are committed"
    );
}

#[allow(dead_code)]
#[derive(Clone)]
struct TestEnvironment {
    pub pool: sqlx::PgPool,
    pub client_key: Option<ClientKey>,
    pub key_id_gw: DbKeyId,
    pub host_chain_id: i64,
    pub db_instance: DBInstance,
    pub s3_instance: Option<Arc<LocalstackContainer>>, // If None, the global LocalStack is used
    pub s3_client: aws_sdk_s3::Client,
    pub conf: Config,
    pub private_key: Vec<u8>,
}

async fn setup(enable_compression: bool) -> anyhow::Result<TestEnvironment> {
    init_tracing();

    let db_instance = setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("valid db instance");

    let conf = build_test_config(db_instance.db_url.clone(), enable_compression);

    // Set up the database connection pool
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.timeout)
        .connect(conf.db.url.as_str())
        .await?;

    // Set up S3 storage
    let (s3_instance, s3_client) = if cfg!(feature = "gpu") {
        info!("GPU feature is enabled, avoid testing S3-related functionality");
        (
            None,
            aws_sdk_s3::Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await),
        )
    } else {
        setup_localstack(&conf).await?
    };

    let token = db_instance.parent_token.child_token();
    let mut config: Config = conf.clone();
    let signer = PrivateKeySigner::random();
    let private_key = signer.to_bytes().to_vec();
    config.private_key = Some(hex::encode(&private_key));

    let key_id_gw = fetch_latest_key_id_gw(&pool).await;
    let host_chain_id = fetch_host_chain_id(&pool).await;
    let client_key: Option<ClientKey> = fetch_client_key(&pool, &key_id_gw).await?;

    let (events_tx, mut events_rx) = mpsc::channel::<&'static str>(10);
    tokio::spawn(async move {
        crate::run_all(config, token, Some(events_tx))
            .await
            .expect("valid worker run");
    });

    // Wait until the keys are loaded with timeout of 1 min
    let load_keys = timeout(Duration::from_secs(60), events_rx.recv()).await;
    if let Result::Ok(Some(event)) = load_keys {
        info!(event = %event, "Proceeding with tests");
    } else {
        return Err(anyhow!("Timeout waiting for keys to be loaded"));
    }

    Ok(TestEnvironment {
        pool,
        client_key,
        key_id_gw,
        host_chain_id,
        db_instance,
        s3_instance,
        s3_client,
        conf,
        private_key,
    })
}

/// Deploys a LocalStack instance and creates S3 buckets for ciphertext128 and ciphertext64
///
/// # Returns
/// A tuple containing the LocalStack instance and the S3 client
async fn setup_localstack(
    conf: &Config,
) -> anyhow::Result<(Option<Arc<LocalstackContainer>>, aws_sdk_s3::Client)> {
    let (localstack, host_port) =
        if std::env::var("TEST_GLOBAL_LOCALSTACK").unwrap_or("0".to_string()) == "1" {
            (None, LOCALSTACK_PORT)
        } else {
            let localstack_instance = Arc::new(test_harness::localstack::start_localstack().await?);
            let host_port = localstack_instance.host_port;
            (Some(localstack_instance), host_port)
        };

    tracing::info!("LocalStack started on port: {}", host_port);

    let endpoint_url = format!("http://127.0.0.1:{}", host_port);
    std::env::set_var("AWS_ENDPOINT_URL", endpoint_url.clone());
    std::env::set_var("AWS_REGION", "us-east-1");
    std::env::set_var("AWS_ACCESS_KEY_ID", "test");
    std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");

    let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(&aws_conf);

    recreate_bucket(&client, &conf.s3.bucket_ct128).await?;
    recreate_bucket(&client, &conf.s3.bucket_ct64).await?;

    Ok((localstack, client))
}

async fn recreate_bucket(s3_client: &aws_sdk_s3::Client, bucket_name: &str) -> anyhow::Result<()> {
    empty_bucket(s3_client, bucket_name).await?;

    s3_client
        .delete_bucket()
        .set_bucket(Some(bucket_name.to_string()))
        .send()
        .await
        .ok(); // Ignore error if bucket does not exist

    s3_client
        .create_bucket()
        .set_bucket(Some(bucket_name.to_string()))
        .send()
        .await
        .expect("Failed to create bucket");

    Ok(())
}

async fn empty_bucket(s3_client: &aws_sdk_s3::Client, bucket_name: &str) -> anyhow::Result<()> {
    let result = match s3_client.list_objects().bucket(bucket_name).send().await {
        std::result::Result::Ok(result) => result,
        Err(_) => return Ok(()),
    };

    for object in result.contents() {
        let Some(key) = object.key() else {
            continue;
        };

        s3_client
            .delete_object()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TestFile {
    pub handle: [u8; 32],
    pub ciphertext64: Vec<u8>,
    pub cleartext: i64,
}

/// Regenerates `ciphertext64.json` against the current
/// `fhevm-keys/xof-keyset` LFS fixture.
///
/// The on-disk fixture is bound to a specific `CompactPublicKey`, so
/// every rotation of `xof-keyset` invalidates it: SNS converts the
/// stored ciphertext64 under the new server key, but the decrypted
/// value no longer matches `cleartext` because the input bytes were
/// encrypted under the old key.
///
/// Re-run via:
///   `cargo test -p sns-worker --features test_decrypt_128 \
///       -- --ignored regenerate_ciphertext64_fixture`
/// then commit the updated `ciphertext64.json`.
fn write_test_file(filename: &str) {
    use fhevm_engine_common::types::SupportedFheCiphertexts;
    use fhevm_engine_common::utils::safe_deserialize_key;
    use tfhe::prelude::CiphertextList;
    use tfhe::xof_key_set::CompressedXofKeySet;

    let keyset_bytes = std::fs::read("../fhevm-keys/xof-keyset").expect("read xof-keyset fixture");
    let keyset: CompressedXofKeySet =
        safe_deserialize_key(&keyset_bytes).expect("deserialize CompressedXofKeySet");
    // Whole-keyset decompression: same XOF stream the production
    // readers (sns-worker, tfhe-worker GPU) traverse, so the
    // CompactPublicKey we encrypt under matches what the DB pks_key
    // column will carry.
    let (compact_public_key, server_key) = keyset
        .decompress()
        .expect("decompress xof keyset")
        .into_raw_parts();

    // CompactCiphertextList expansion and CompressedCiphertextList
    // build both consult the thread-local server key.
    tfhe::set_server_key(server_key);

    let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
    builder.push(0_u64);
    let compact_list = builder.build();
    let expanded = compact_list
        .expand()
        .expect("expand compact ciphertext list");
    let ct: tfhe::FheUint64 = expanded
        .get(0)
        .expect("get(0) from expanded list")
        .expect("expanded list has element 0");

    let ciphertext64 = SupportedFheCiphertexts::FheUint64(ct)
        .compress()
        .expect("compress FheUint64 to CompressedCiphertextList");

    // Handle preserved verbatim: byte 30 = 5 is the FheUint64 type
    // tag consumed by get_ct_type, and bytes 0-1 are overwritten per
    // batch entry by test_batch_execution; the rest is arbitrary
    // padding.
    let handle: [u8; 32] = [
        82, 179, 54, 227, 20, 74, 138, 57, 192, 160, 141, 228, 185, 10, 90, 70, 138, 165, 113, 249,
        28, 54, 93, 45, 102, 136, 242, 216, 124, 6, 5, 3,
    ];

    let v = TestFile {
        handle,
        ciphertext64,
        cleartext: 0,
    };

    File::create(filename)
        .expect("Failed to create file")
        .write_all(&serde_json::to_vec(&v).unwrap())
        .expect("Failed to write to file");
}

/// Run only when intentionally regenerating the LFS-bound fixture.
/// See [`write_test_file`] for the workflow.
#[test]
#[ignore = "regenerates ciphertext64.json against the current xof-keyset fixture"]
fn regenerate_ciphertext64_fixture() {
    write_test_file("ciphertext64.json");
}

fn read_test_file(filename: &str) -> TestFile {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    serde_json::from_slice(&buffer).expect("Failed to deserialize")
}

async fn fetch_latest_key_id_gw(pool: &sqlx::PgPool) -> DbKeyId {
    sqlx::query_scalar("SELECT key_id_gw FROM keys ORDER BY sequence_number DESC LIMIT 1")
        .fetch_one(pool)
        .await
        .expect("key_id_gw")
}

async fn fetch_host_chain_id(pool: &sqlx::PgPool) -> i64 {
    sqlx::query_scalar("SELECT chain_id FROM host_chains ORDER BY chain_id DESC LIMIT 1")
        .fetch_one(pool)
        .await
        .expect("host_chain_id")
}

async fn insert_ciphertext64(
    pool: &sqlx::PgPool,
    handle: &Vec<u8>,
    ciphertext: &Vec<u8>,
) -> anyhow::Result<()> {
    test_harness::db_utils::insert_ciphertext64(pool, handle, ciphertext).await?;

    // Notify sns_worker
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(LISTEN_CHANNEL)
        .execute(pool)
        .await?;

    Ok(())
}

async fn insert_into_pbs_computations(
    pool: &sqlx::PgPool,
    host_chain_id: i64,
    handle: &Vec<u8>,
) -> Result<(), anyhow::Error> {
    test_harness::db_utils::insert_into_pbs_computations(pool, host_chain_id, handle).await?;

    // Notify sns_worker
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(LISTEN_CHANNEL)
        .execute(pool)
        .await?;

    Ok(())
}

/// Cleans up the database by truncating specific tables
async fn clean_up(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    truncate_tables(
        pool,
        vec![
            "pbs_computations_branch",
            "ciphertexts_branch",
            "ciphertexts128_branch",
            "ciphertext_digest_branch",
            // insert_ciphertext64 dual-writes the legacy table; without
            // truncating it, persistent-DB runs pin first-writer bytes there
            // (ON CONFLICT DO NOTHING) and legacy/branch state diverges.
            "ciphertexts",
            "host_chain_blocks_valid",
            "coprocessor_settlement",
            "s3_canonical_repair_queue",
        ],
    )
    .await?;

    Ok(())
}

/// Verifies that the ciphertext for the given handle in the database decrypts to the expected value
///
/// It waits for the ciphertext to be available in the database, decrypts it using the client key,
/// and asserts that the decrypted value matches the expected cleartext value.
///
/// It also checks that the ciphertext is uploaded to S3 if the feature is enabled.
async fn assert_ciphertext128(
    test_env: &TestEnvironment,
    with_compression: bool,
    handle: &Vec<u8>,
    expected_value: i64,
) -> anyhow::Result<()> {
    let pool = &test_env.pool;
    let client_key = &test_env.client_key;
    let ct = test_harness::db_utils::wait_for_ciphertext(pool, handle, 100).await?;

    info!("Ciphertext len: {:?}", ct.len());

    let encrypted: SquashedNoiseFheUint = if with_compression {
        let res = safe_deserialize::<CompressedSquashedNoiseCiphertextList>(&ct);
        assert!(
            res.is_ok(),
            "Could not deserialize compressed ciphertext128. 
            This might indicate a failed squash_noise computation."
        );
        res?.get(0)?
            .ok_or_else(|| anyhow!("Failed to get the first element from the list"))?
    } else {
        let res = safe_deserialize::<SquashedNoiseFheUint>(&ct);
        assert!(
            res.is_ok(),
            "Could not deserialize ciphertext128. 
            This might indicate a failed squash_noise computation."
        );
        res?
    };

    // This feature is only enabled in local tests, never in CI
    // because the client key is not available in CI for now
    #[cfg(feature = "test_decrypt_128")]
    {
        let decrypted: u128 = encrypted.decrypt(
            client_key
                .as_ref()
                .ok_or_else(|| anyhow!("Client key is not available for decryption"))?,
        );

        info!("Decrypted value: {decrypted}");
        assert!(
            decrypted == expected_value as u128,
            "Decrypted value does not match expected value",
        );
    }

    // Assert that ciphertext128 is uploaded to S3
    // Note: The tests rely on the `test_s3_use_handle_as_key` feature,
    // which uses the handle/context-id S3 key instead of the digest.
    // This approach allows reusing the same ct128 when uploading a batch of ciphertexts to S3 under different keys.

    #[cfg(feature = "test_s3_use_handle_as_key")]
    {
        info!("Asserting ciphertext uploaded to S3");

        let expected_ct_format = if with_compression {
            crate::Ciphertext128Format::CompressedOnCpu
        } else {
            crate::Ciphertext128Format::UncompressedOnCpu
        };
        let expected_attestation_format = if with_compression {
            CiphertextFormat::CompressedOnCpu
        } else {
            CiphertextFormat::UncompressedOnCpu
        };
        let expected_ct_format = expected_ct_format.to_string();

        assert_ciphertext_uploaded(
            test_env,
            &test_env.conf.s3.bucket_ct128,
            handle,
            Some(ct.len() as i64),
            Some((&expected_ct_format, expected_attestation_format)),
        )
        .await?;
        assert_ciphertext_uploaded(test_env, &test_env.conf.s3.bucket_ct64, handle, None, None)
            .await?;
    }

    Ok(())
}

/// Asserts that ciphertext exists in S3
#[cfg(not(feature = "gpu"))]
async fn assert_ciphertext_uploaded(
    test_env: &TestEnvironment,
    bucket: &String,
    handle: &Vec<u8>,
    expected_ct_len: Option<i64>,
    expected_ct_format: Option<(&str, CiphertextFormat)>,
) -> anyhow::Result<()> {
    let ciphertext_key =
        crate::aws_upload::s3_ciphertext_key(handle, crate::aws_upload::COPROCESSOR_CONTEXT_ID_1);
    use crate::S3_FORMAT_VERSION_V1;

    let (ciphertext_digest, sns_ciphertext_digest, s3_format_version) =
        wait_for_ciphertext_digest_upload_state(&test_env.pool, handle, 100).await?;

    s3_utils::assert_key_exists(
        test_env.s3_client.to_owned(),
        bucket,
        &ciphertext_key,
        expected_ct_len,
        100,
    )
    .await;

    let output = test_env
        .s3_client
        .head_object()
        .bucket(bucket)
        .key(ciphertext_key)
        .send()
        .await
        .expect("head ciphertext object");
    let metadata = output.metadata().expect("ciphertext metadata");
    let key_id = hex::encode(&test_env.key_id_gw);
    let attestation_json = metadata
        .get(S3_METADATA_ATTESTATION_KEY)
        .expect("ciphertext object should include ct-attestation metadata");
    let attestation: CiphertextAttestation = serde_json::from_str(attestation_json)?;
    attestation.verify(
        B256::from_slice(handle),
        crate::aws_upload::COPROCESSOR_CONTEXT_ID_1,
    )?;

    let signer = PrivateKeySigner::from_str(&hex::encode(&test_env.private_key))?;

    assert_eq!(
        attestation.key_id,
        U256::from_be_slice(&test_env.key_id_gw),
        "attestation should include the expected key_id"
    );
    assert_eq!(
        attestation.ciphertext_digest,
        B256::from_slice(&ciphertext_digest),
        "attestation should include the expected ct64 digest"
    );
    assert_eq!(
        attestation.sns_ciphertext_digest,
        B256::from_slice(&sns_ciphertext_digest),
        "attestation should include the expected ct128 digest"
    );
    assert_eq!(
        attestation.signer,
        signer.address(),
        "attestation should include the expected signer"
    );
    assert_eq!(
        s3_format_version, S3_FORMAT_VERSION_V1,
        "ciphertext_digest should record the current S3 format version"
    );
    assert_eq!(
        metadata.get("key-id"),
        Some(&key_id),
        "ciphertext object should include Key-Id metadata"
    );
    assert!(
        metadata.contains_key("transaction-id"),
        "ciphertext object should include Transaction-Id metadata"
    );
    assert!(
        metadata.contains_key("signer"),
        "ciphertext object should include Signer metadata"
    );
    if let Some((expected_ct_format, expected_attestation_format)) = expected_ct_format {
        assert_eq!(
            attestation.format, expected_attestation_format,
            "ciphertext128 attestation should include the expected ct format"
        );
        assert_eq!(
            metadata.get("ct-format").map(String::as_str),
            Some(expected_ct_format),
            "ciphertext128 object should include Ct-Format metadata"
        );

        let digest_key = hex::encode(&sns_ciphertext_digest);
        s3_utils::assert_key_exists(
            test_env.s3_client.to_owned(),
            bucket,
            &digest_key,
            expected_ct_len,
            100,
        )
        .await;

        let digest_output = test_env
            .s3_client
            .head_object()
            .bucket(bucket)
            .key(digest_key)
            .send()
            .await
            .expect("head ciphertext128 digest object");
        let digest_metadata = digest_output
            .metadata()
            .expect("ciphertext128 digest object metadata");
        assert_eq!(
            digest_metadata.get("ct-format").map(String::as_str),
            Some(expected_ct_format),
            "ciphertext128 digest object should include Ct-Format metadata"
        );
    }

    Ok(())
}

async fn wait_for_ciphertext_digest_upload_state(
    pool: &sqlx::PgPool,
    handle: &Vec<u8>,
    retries: u64,
) -> anyhow::Result<(Vec<u8>, Vec<u8>, i16)> {
    for retry in 0..retries {
        // The upload path marks ciphertext_digest_branch (legacy
        // ciphertext_digest is no longer written by sns-worker); the tests
        // create one branch row per handle.
        let row = sqlx::query(
            "SELECT ciphertext, ciphertext128, s3_format_version
            FROM ciphertext_digest_branch
            WHERE handle = $1",
        )
        .bind(handle)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let ciphertext_digest: Option<Vec<u8>> = row.try_get("ciphertext")?;
            let sns_ciphertext_digest: Option<Vec<u8>> = row.try_get("ciphertext128")?;
            let s3_format_version: Option<i16> = row.try_get("s3_format_version")?;

            if let (Some(ciphertext_digest), Some(sns_ciphertext_digest), Some(s3_format_version)) =
                (ciphertext_digest, sns_ciphertext_digest, s3_format_version)
            {
                return Ok((ciphertext_digest, sns_ciphertext_digest, s3_format_version));
            }
        }

        info!(retry, "Waiting for ciphertext_digest upload state");
        sleep(Duration::from_millis(100)).await;
    }

    Err(anyhow!(
        "ciphertext_digest upload state was not complete for handle {}",
        to_hex(handle)
    ))
}

#[cfg(feature = "gpu")]
async fn assert_ciphertext_uploaded(
    _test_env: &TestEnvironment,
    _bucket: &String,
    _handle: &Vec<u8>,
    _expected_ct_len: Option<i64>,
    _expected_ct_format: Option<(&str, CiphertextFormat)>,
) -> anyhow::Result<()> {
    // No-op when GPU feature is enabled
    Ok(())
}

/// Asserts that the number of ciphertext128 objects in S3 matches the expected count
#[cfg(not(feature = "gpu"))]
async fn assert_ciphertext_s3_object_count(
    test_env: &TestEnvironment,
    bucket: &String,
    expected_count: i64,
) {
    s3_utils::assert_object_count(test_env.s3_client.to_owned(), bucket, expected_count as i32)
        .await;
}

#[cfg(feature = "gpu")]
async fn assert_ciphertext_s3_object_count(
    _te: &TestEnvironment,
    _bucket: &String,
    _expected_count: i64,
) {
    // No-op when GPU feature is enabled
}

fn build_test_config(url: DatabaseURL, enable_compression: bool) -> Config {
    let batch_limit = std::env::var("BATCH_LIMIT")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(100);

    let schedule_policy = std::env::var("SCHEDULE_POLICY")
        .ok()
        .map(SchedulePolicy::from)
        .unwrap_or(SchedulePolicy::RayonParallel);

    Config {
        db: DBConfig {
            url,
            listen_channels: vec![LISTEN_CHANNEL.to_string()],
            notify_channel: "fhevm".to_string(),
            batch_limit,
            gc_batch_limit: 0,
            polling_interval: 60000,
            cleanup_interval: Duration::from_hours(10),
            max_connections: 5,
            timeout: Duration::from_secs(5),
            lifo: false,
            branch_cutover_block: 0,
        },
        s3: S3Config {
            bucket_ct128: "ct128".to_owned(),
            bucket_ct64: "ct64".to_owned(),
            max_concurrent_uploads: 2000,
            retry_policy: S3RetryPolicy {
                max_retries_per_upload: 100,
                max_backoff: Duration::from_secs(10),
                max_retries_timeout: Duration::from_secs(120),
                recheck_duration: Duration::from_secs(2),
                regular_recheck_duration: Duration::from_secs(120),
            },
            verify_sha256_checksum: true,
        },
        service_name: "".to_owned(),
        log_level: Level::INFO,
        health_checks: crate::HealthCheckConfig {
            liveness_threshold: Duration::from_secs(10),
            port: 8080,
        },
        enable_compression,
        schedule_policy,
        pg_auto_explain_with_min_duration: Some(Duration::from_secs(1)),
        metrics: Default::default(),
        gcs_mode: false,
        private_key: None,
        signer_type: fhevm_engine_common::types::SignerType::PrivateKey,
        s3_migration: S3MigrationMode::No,
        s3_migration_sleep_duration: Duration::from_mins(5),
        s3_migration_max_retries: DEFAULT_S3_MIGRATION_MAX_RETRIES,
    }
}
