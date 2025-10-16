use crate::{
    executor::{garbage_collect, query_sns_tasks, Order},
    keyset::fetch_client_key,
    squash_noise::safe_deserialize,
    Config, DBConfig, S3Config, S3RetryPolicy, SchedulePolicy,
};
use anyhow::{anyhow, Ok};
use aws_config::BehaviorVersion;
use fhevm_engine_common::utils::{compact_hex, DatabaseURL};
use serde::{Deserialize, Serialize};
use serial_test::serial;
use sqlx::Database;
use std::{
    fs::File,
    io::{Read, Write},
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
use tokio::{sync::mpsc, time::timeout};
use tracing::{info, Level};

const LISTEN_CHANNEL: &str = "sns_worker_chan";
const TENANT_API_KEY: &str = "a1503fb6-d79b-4e9e-826d-44cf262f3e05";

static TRACING_INIT: OnceLock<()> = OnceLock::new();

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
    let tf: TestFile = read_test_file("ciphertext64.bin");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64.clone(),
        tf.decrypted,
        true,
        WITH_COMPRESSION,
    )
    .await
    .expect("test_fhe_ciphertext128_with_compression, first_fhe_computation = true");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64,
        tf.decrypted,
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
    let tf: TestFile = read_test_file("ciphertext64.bin");

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
        tf.decrypted,
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
    let tf: TestFile = read_test_file("ciphertext64.bin");

    test_decryptable(
        &test_env,
        &tf.handle.into(),
        &tf.ciphertext64.clone(),
        tf.decrypted,
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
        insert_into_pbs_computations(pool, handle).await?;
    } else {
        // insert into pbs_computations
        insert_into_pbs_computations(pool, handle).await?;
        insert_ciphertext64(pool, handle, ciphertext).await?;
    }

    let tenant_id = get_tenant_id_from_db(pool, TENANT_API_KEY).await;

    assert_ciphertext128(
        test_env,
        tenant_id,
        with_compression,
        handle,
        expected_result,
    )
    .await?;

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
    let tenant_id = get_tenant_id_from_db(pool, TENANT_API_KEY).await;
    for i in 0..batch_size {
        let mut handle = base_handle.to_owned();

        // Modify first two bytes of the handle to make it unique
        // However the ciphertext64 will be the same
        handle[0] = (i >> 8) as u8;
        handle[1] = (i & 0xFF) as u8;
        test_harness::db_utils::insert_ciphertext64(pool, tenant_id, &handle, ciphertext, &[])
            .await?;
        test_harness::db_utils::insert_into_pbs_computations(pool, tenant_id, &handle).await?;
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
            assert_ciphertext128(
                &test_env,
                tenant_id,
                with_compression,
                &handle,
                expected_cleartext,
            )
            .await
        });
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    let elapsed = start.elapsed();
    info!(elapsed = ?elapsed, batch_size, "Batch execution completed");

    // Assert that all ciphertext128 objects are uploaded to S3
    assert_ciphertext_s3_object_count(test_env, bucket128, batch_size as i64).await;
    assert_ciphertext_s3_object_count(test_env, bucket64, batch_size as i64).await;

    anyhow::Result::<()>::Ok(())
}

#[tokio::test]
#[serial(db)]
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

    for i in 0..HANDLES_COUNT {
        // insert into ciphertexts
        test_harness::db_utils::insert_ciphertext64(
            &pool,
            1,
            &Vec::from([i as u8; 32]),
            &Vec::from([i as u8; 32]),
            &[i as u8; 32],
        )
        .await
        .unwrap();

        test_harness::db_utils::insert_into_pbs_computations(&pool, 1, &Vec::from([i as u8; 32]))
            .await
            .unwrap();
    }

    let mut trx = pool.begin().await.unwrap();
    if let Result::Ok(Some(tasks)) = query_sns_tasks(&mut trx, BATCH_SIZE as u32, Order::Desc).await
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
    if let Result::Ok(Some(tasks)) = query_sns_tasks(&mut trx, BATCH_SIZE as u32, Order::Asc).await
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
            info!("Asc Task handle: {}", compact_hex(&task.handle));
        }
    } else {
        panic!("No tasks found in Asc order");
    }
}

#[tokio::test]
#[serial(db)]
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

    let tenant_id = 1;
    for i in 0..HANDLES_COUNT {
        // insert into ciphertexts
        let mut handle = [0u8; 32];
        handle[..4].copy_from_slice(&i.to_le_bytes());

        test_harness::db_utils::insert_ciphertext64(
            &pool,
            tenant_id,
            &handle.to_vec(),
            &handle.to_vec(),
            &[i as u8; 32],
        )
        .await
        .unwrap();

        test_harness::db_utils::insert_ciphertext_digest(
            &pool,
            tenant_id,
            &handle,
            &[i as u8; 32],
            &[i as u8; 32],
            0,
        )
        .await
        .unwrap();
    }

    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts where ciphertext128 IS not NULL")
            .fetch_one(&pool)
            .await
            .expect("count ciphertext_digest");
    assert_eq!(
        count, HANDLES_COUNT as i64,
        "ciphertext128 should not be empty before garbage_collect"
    );

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

    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts where ciphertext128 IS NULL")
            .fetch_one(&pool)
            .await
            .expect("count ciphertext_digest");
    assert_eq!(
        count, HANDLES_COUNT as i64,
        "ciphertext128 should be empty after garbage_collect"
    );
}

#[allow(dead_code)]
#[derive(Clone)]
struct TestEnvironment {
    pub pool: sqlx::PgPool,
    pub client_key: Option<ClientKey>,
    pub db_instance: DBInstance,
    pub s3_instance: Option<Arc<LocalstackContainer>>, // If None, the global LocalStack is used
    pub s3_client: aws_sdk_s3::Client,
    pub conf: Config,
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
    let (s3_instance, s3_client) = setup_localstack(&conf).await?;

    let token = db_instance.parent_token.child_token();
    let config: Config = conf.clone();

    let client_key: Option<ClientKey> = fetch_client_key(&pool, &TENANT_API_KEY.to_owned()).await?;

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
        db_instance,
        s3_instance,
        s3_client,
        conf,
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

#[derive(Serialize, Deserialize)]
struct TestFile {
    pub handle: [u8; 32],
    pub ciphertext64: Vec<u8>,
    pub decrypted: i64,
}

/// Creates a test-file from handle, ciphertext64 and plaintext
/// Can be used to update/create_new ciphertext64.bin file
#[expect(dead_code)]
fn write_test_file(filename: &str) {
    let handle: [u8; 32] = hex::decode("TBD").unwrap().try_into().unwrap();
    let ciphertext64 = hex::decode("TBD").unwrap();
    let plaintext = 0;

    let v = TestFile {
        handle,
        ciphertext64,
        decrypted: plaintext,
    };

    // Write bytes to a file
    File::create(filename)
        .expect("Failed to create file")
        .write_all(&bincode::serialize(&v).unwrap())
        .expect("Failed to write to file");
}

fn read_test_file(filename: &str) -> TestFile {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    bincode::deserialize(&buffer).expect("Failed to deserialize")
}

async fn get_tenant_id_from_db(pool: &sqlx::PgPool, tenant_api_key: &str) -> i32 {
    let tenant_id: i32 =
        sqlx::query_scalar("SELECT tenant_id FROM tenants WHERE tenant_api_key = $1::uuid")
            .bind(tenant_api_key)
            .fetch_one(pool)
            .await
            .expect("tenant_id");

    tenant_id
}

async fn insert_ciphertext64(
    pool: &sqlx::PgPool,
    handle: &Vec<u8>,
    ciphertext: &Vec<u8>,
) -> anyhow::Result<()> {
    let tenant_id = get_tenant_id_from_db(pool, TENANT_API_KEY).await;
    test_harness::db_utils::insert_ciphertext64(pool, tenant_id, handle, ciphertext, &[]).await?;

    // Notify sns_worker
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(LISTEN_CHANNEL)
        .execute(pool)
        .await?;

    Ok(())
}

async fn insert_into_pbs_computations(
    pool: &sqlx::PgPool,
    handle: &Vec<u8>,
) -> Result<(), anyhow::Error> {
    let tenant_id = get_tenant_id_from_db(pool, TENANT_API_KEY).await;
    test_harness::db_utils::insert_into_pbs_computations(pool, tenant_id, handle).await?;

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
        vec!["pbs_computations", "ciphertexts", "ciphertext_digest"],
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
    tenant_id: i32,
    with_compression: bool,
    handle: &Vec<u8>,
    expected_value: i64,
) -> anyhow::Result<()> {
    let pool = &test_env.pool;
    let client_key = &test_env.client_key;
    let ct = test_harness::db_utils::wait_for_ciphertext(pool, tenant_id, handle, 100).await?;

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
    // which uses the handle as the key instead of the digest.
    // This approach allows reusing the same ct128 when uploading a batch of ciphertexts to S3 under different keys.

    #[cfg(feature = "test_s3_use_handle_as_key")]
    {
        info!("Asserting ciphertext uploaded to S3");

        assert_ciphertext_uploaded(
            test_env,
            &test_env.conf.s3.bucket_ct128,
            handle,
            Some(ct.len() as i64),
        )
        .await;
        assert_ciphertext_uploaded(test_env, &test_env.conf.s3.bucket_ct64, handle, None).await;
    }

    Ok(())
}

/// Asserts that ciphertext exists in S3
async fn assert_ciphertext_uploaded(
    test_env: &TestEnvironment,
    bucket: &String,
    handle: &Vec<u8>,
    expected_ct_len: Option<i64>,
) {
    s3_utils::assert_key_exists(
        test_env.s3_client.to_owned(),
        bucket,
        &hex::encode(handle),
        expected_ct_len,
        100,
    )
    .await;
}

/// Asserts that the number of ciphertext128 objects in S3 matches the expected count
async fn assert_ciphertext_s3_object_count(
    test_env: &TestEnvironment,
    bucket: &String,
    expected_count: i64,
) {
    s3_utils::assert_object_count(test_env.s3_client.to_owned(), bucket, expected_count as i32)
        .await;
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
        tenant_api_key: TENANT_API_KEY.to_string(),
        db: DBConfig {
            url,
            listen_channels: vec![LISTEN_CHANNEL.to_string()],
            notify_channel: "fhevm".to_string(),
            batch_limit,
            gc_batch_limit: 30,
            polling_interval: 60000,
            cleanup_interval: Duration::from_secs(10),
            max_connections: 5,
            timeout: Duration::from_secs(5),
            lifo: false,
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
    }
}
