use crate::{
    create_s3_client,
    executor::{garbage_collect, query_sns_tasks, Order},
    keyset::fetch_keys,
    squash_noise::safe_deserialize,
    Config, DBConfig, SchedulePolicy, UploadJob,
};
use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    time::Duration,
};

use test_harness::instance::{DBInstance, ImportMode};
use tfhe::{
    prelude::FheDecrypt, ClientKey, CompressedSquashedNoiseCiphertextList, SquashedNoiseFheUint,
};
use tokio::{sync::mpsc, time::sleep};
use tracing::Level;

const LISTEN_CHANNEL: &str = "sns_worker_chan";
const TENANT_API_KEY: &str = "a1503fb6-d79b-4e9e-826d-44cf262f3e05";

#[tokio::test]
#[ignore = "requires valid SnS keys in CI"]
async fn test_fhe_ciphertext128_with_compression() {
    const WITH_COMPRESSION: bool = true;
    let (conn, client_key, _rx, _test_instance) =
        setup(WITH_COMPRESSION).await.expect("valid setup");
    let tf: TestFile = read_test_file("ciphertext64.bin");

    test_decryptable(
        &conn,
        &client_key,
        &tf.handle.into(),
        &tf.ciphertext64.clone(),
        tf.decrypted,
        true,
        WITH_COMPRESSION,
    )
    .await
    .expect("test_decryptable, first_fhe_computation = true");
    test_decryptable(
        &conn,
        &client_key,
        &tf.handle.into(),
        &tf.ciphertext64,
        tf.decrypted,
        false,
        WITH_COMPRESSION,
    )
    .await
    .expect("test_decryptable, first_fhe_computation = false");
}

#[tokio::test]
#[ignore = "requires valid SnS keys in CI"]
async fn test_fhe_ciphertext128_no_compression() {
    const NO_COMPRESSION: bool = false;
    let (conn, client_key, _rx, _test_instance) = setup(NO_COMPRESSION).await.expect("valid setup");
    let tf: TestFile = read_test_file("ciphertext64.bin");

    test_decryptable(
        &conn,
        &client_key,
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
    pool: &sqlx::PgPool,
    client_key: &Option<ClientKey>,
    handle: &Vec<u8>,
    ciphertext: &Vec<u8>,
    expected_result: i64,
    first_fhe_computation: bool, // first insert ciphertext64 in DB
    with_compression: bool,
) -> anyhow::Result<()> {
    clean_up(pool, handle).await?;

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

    // wait until ciphertext.large_ct is not NULL
    let data = test_harness::db_utils::wait_for_ciphertext(pool, tenant_id, handle, 10).await?;

    println!("Ciphertext data len: {:?}", data.len());

    let cleartext = if with_compression {
        let list: CompressedSquashedNoiseCiphertextList = safe_deserialize(&data)?;
        let v: SquashedNoiseFheUint = list
            .get(0)?
            .ok_or_else(|| anyhow::anyhow!("Failed to get the first element from the list"))?;
        let r: u128 = v.decrypt(
            client_key
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Client key is not available for decryption"))?,
        );
        r
    } else {
        let v: SquashedNoiseFheUint = safe_deserialize(&data)?;
        let r: u128 = v.decrypt(client_key.as_ref().unwrap());
        r
    };

    println!("Cleartext: {cleartext}");

    assert!(
        cleartext == expected_result as u128,
        "Cleartext value does not match expected value",
    );

    anyhow::Result::<()>::Ok(())
}

#[tokio::test]
async fn test_lifo_mode() {
    tracing_subscriber::fmt().json().with_level(true).init();
    let test_instance = test_harness::instance::setup_test_db(ImportMode::None)
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

            println!("Desc Task handle: {}", hex::encode(&task.handle));
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

            println!("Asc Task handle: {}", hex::encode(&task.handle));
        }
    } else {
        panic!("No tasks found in Asc order");
    }
}

#[tokio::test]
async fn test_garbage_collect() {
    let test_instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");

    const CONCURRENT_TASKS: usize = 20;
    const HANDLES_COUNT: u32 = 1000;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(CONCURRENT_TASKS as u32)
        .connect(test_instance.db_url())
        .await
        .unwrap();

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
    let res_ = tokio::time::timeout(Duration::from_secs(5), async {
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

async fn setup(
    enable_compression: bool,
) -> anyhow::Result<(
    sqlx::PgPool,
    Option<ClientKey>,
    tokio::sync::mpsc::Receiver<UploadJob>,
    DBInstance,
)> {
    tracing_subscriber::fmt().json().with_level(true).init();
    let test_instance = test_harness::instance::setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("valid db instance");

    let conf = Config {
        tenant_api_key: TENANT_API_KEY.to_string(),
        db: DBConfig {
            url: test_instance.db_url().to_owned(),
            listen_channels: vec![LISTEN_CHANNEL.to_string()],
            notify_channel: "fhevm".to_string(),
            batch_limit: 10,
            gc_batch_limit: 30,
            polling_interval: 60000,
            cleanup_interval: Duration::from_secs(10),
            max_connections: 5,
            timeout: Duration::from_secs(5),
            lifo: false,
        },
        s3: crate::S3Config::default(),
        service_name: "test-sns-worker".to_owned(),
        log_level: Level::INFO,
        health_checks: crate::HealthCheckConfig {
            liveness_threshold: Duration::from_secs(10),
            port: 8080,
        },
        enable_compression,
        schedule_policy: SchedulePolicy::RayonParallel,
    };

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.timeout)
        .connect(&conf.db.url)
        .await?;

    let (upload_tx, upload_rx) = mpsc::channel::<UploadJob>(10);

    let token = test_instance.parent_token.child_token();
    let (client_key, _) = fetch_keys(&pool, &TENANT_API_KEY.to_owned()).await?;
    let (client, _) = create_s3_client(&conf).await;

    tokio::spawn(async move {
        crate::compute_128bit_ct(conf, upload_tx, token, client)
            .await
            .expect("valid worker");
        Ok(())
    });

    // TODO: Replace this with notification from the worker when it's in ready-state
    sleep(Duration::from_secs(5)).await;

    Ok((pool, client_key, upload_rx, test_instance))
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

/// Deletes all records from `pbs_computations` and `ciphertexts` where `handle`
/// matches.
async fn clean_up(pool: &sqlx::PgPool, handle: &Vec<u8>) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM pbs_computations WHERE handle = $1")
        .bind(handle)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM ciphertexts WHERE handle = $1")
        .bind(handle)
        .execute(pool)
        .await?;

    Ok(())
}
