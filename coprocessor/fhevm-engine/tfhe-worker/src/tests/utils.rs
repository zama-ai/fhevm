use crate::daemon_cli::Args;
use fhevm_engine_common::crs::{Crs, CrsCache};
use fhevm_engine_common::db_keys::{DbKey, DbKeyCache};
use fhevm_engine_common::drift_revert::WatcherTimeouts;
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use test_harness::db_utils::setup_test_key;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::watch::Receiver;
use tracing::Level;

pub struct TestInstance {
    // just to destroy container
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    // send message to this on destruction to stop the app
    app_close_channel: Option<tokio::sync::watch::Sender<bool>>,
    db_url: String,
    health_check_port: u16,
}

impl Drop for TestInstance {
    fn drop(&mut self) {
        println!("Shutting down the app with signal");
        if let Some(chan) = &self.app_close_channel {
            let _ = chan.send_replace(true);
        }
    }
}

impl TestInstance {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub fn db_docker_id(&self) -> Option<String> {
        self._container.as_ref().map(|c| c.id().to_string())
    }

    pub fn health_check_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.health_check_port)
    }
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    // Pool of 8: the worker holds the LISTEN connection and the work
    // transaction while the DCID lock manager issues its own pool queries, so
    // 2 connections deadlock the cycle into PoolTimedOut.
    setup_test_app_with_worker_config(0, 8).await
}

pub async fn setup_test_app_with_worker_config(
    branch_cutover_block: i64,
    pg_pool_max_connections: u32,
) -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db(branch_cutover_block, pg_pool_max_connections).await
    } else {
        setup_test_app_custom_docker(branch_cutover_block, pg_pool_max_connections).await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

const LOCAL_DB_RESET_SQL: &str = "
    TRUNCATE
        computations, ciphertexts, ciphertexts128, allowed_handles,
        pbs_computations, ciphertext_digest,
        computations_branch, ciphertexts_branch, ciphertexts128_branch,
        allowed_handles_branch, pbs_computations_branch, ciphertext_digest_branch,
        host_chain_blocks_valid, dependence_chain, coprocessor_settlement
    CASCADE
";

pub async fn reset_local_test_db_if_needed() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_err() {
        return Ok(());
    }

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(LOCAL_DB_URL)
        .await?;

    sqlx::query(LOCAL_DB_RESET_SQL).execute(&pool).await?;
    Ok(())
}

async fn setup_test_app_existing_db(
    branch_cutover_block: i64,
    pg_pool_max_connections: u32,
) -> Result<TestInstance, Box<dyn std::error::Error>> {
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    let health_check_port = start_coprocessor(
        rx,
        LOCAL_DB_URL,
        branch_cutover_block,
        pg_pool_max_connections,
    )
    .await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        db_url: LOCAL_DB_URL.to_string(),
        health_check_port,
    })
}

async fn start_coprocessor(
    rx: Receiver<bool>,
    db_url: &str,
    branch_cutover_block: i64,
    pg_pool_max_connections: u32,
) -> u16 {
    let health_check_port = test_harness::localstack::pick_free_port();
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 1000,
        bridge_polling_interval_ms: 1000,
        bridge_associate_batch_size: 128,
        generate_fhe_keys: false,
        key_cache_size: 4,
        coprocessor_fhe_threads: 4,
        tokio_threads: 2,
        pg_pool_max_connections,
        metrics_addr: None,
        database_url: Some(db_url.into()),
        service_name: "coprocessor".to_string(),
        log_level: Level::INFO,
        health_check_port,
        metric_fhe_batch_latency: MetricsConfig::default(),
        worker_id: None,
        dcid_ttl_sec: 30,
        dcid_timeslice_sec: 90,
        dcid_cleanup_interval_sec: 0,
        processed_dcid_ttl_sec: 0,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
        // Heavy FHE tests can saturate a test Postgres instance for tens of seconds;
        // relax the watcher's production-tuned thresholds so fail-fast doesn't trip
        // on slow polls during the test workload.
        branch_cutover_block,
        drift_revert_watcher_timeouts: WatcherTimeouts {
            poll_query_timeout: Duration::from_secs(300),
            db_down_limit: Duration::from_secs(1800),
        },
        stack_version: false,
    };

    std::thread::spawn(move || {
        crate::start_runtime(args, Some(rx));
    });

    // wait until worker starts
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    health_check_port
}

async fn setup_test_app_custom_docker(
    branch_cutover_block: i64,
    pg_pool_max_connections: u32,
) -> Result<TestInstance, Box<dyn std::error::Error>> {
    let container = GenericImage::new("postgres", "15.7")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("postgres started");
    println!("Postgres started...");
    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;
    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/coprocessor");
    println!("Creating coprocessor db...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_db_url)
        .await?;
    sqlx::query!("CREATE DATABASE coprocessor;")
        .execute(&admin_pool)
        .await?;
    println!("database url: {db_url}");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Creating test keys");
    setup_test_key(&pool, false).await?;
    println!("DB prepared");

    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    let health_check_port =
        start_coprocessor(rx, &db_url, branch_cutover_block, pg_pool_max_connections).await;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        db_url,
        health_check_port,
    })
}

pub async fn errors_on_allowed_handles(
    test_instance: &TestInstance,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(test_instance.db_url())
        .await?;
    use sqlx::Row;
    let records = sqlx::query(
        "SELECT error_message FROM computations_branch WHERE is_allowed = TRUE AND is_error = TRUE",
    )
    .fetch_all(&pool)
    .await?;
    let mut errors: Vec<String> = vec![];
    for row in &records {
        let error_message: Option<String> = row.try_get("error_message")?;
        errors.push(error_message.unwrap_or_else(|| "No error message".to_string()));
    }
    Ok(errors)
}

pub async fn wait_until_all_allowed_handles_computed(
    test_instance: &TestInstance,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(test_instance.db_url())
        .await?;

    let timeout = std::env::var("FHEVM_TEST_WAIT_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(90));
    let started_at = Instant::now();

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let (current_count,): (i64,) = sqlx::query_as(
            "SELECT count(1) FROM computations_branch WHERE is_allowed = TRUE AND is_completed = FALSE AND is_error = FALSE",
        )
        .fetch_one(&pool)
        .await?;
        if current_count == 0 {
            println!("All computations completed");
            break;
        } else {
            println!("{current_count} computations remaining, waiting...");
        }

        if started_at.elapsed() >= timeout {
            let pending = sqlx::query_as::<_, (String, String, Option<String>, Option<i64>)>(
                "
                SELECT
                    encode(output_handle, 'hex'),
                    encode(producer_block_hash, 'hex'),
                    encode(transaction_id, 'hex'),
                    block_number
                FROM computations_branch
                WHERE is_allowed = TRUE
                  AND is_completed = FALSE
                  AND is_error = FALSE
                ORDER BY block_number NULLS LAST, producer_block_hash, output_handle
                LIMIT 10
                ",
            )
            .fetch_all(&pool)
            .await?;
            return Err(std::io::Error::other(format!(
                "timed out after {:?} waiting for allowed computations to finish; pending rows: {:?}",
                timeout, pending
            ))
            .into());
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecryptionResult {
    pub value: String,
    pub output_type: i16,
}

pub async fn latest_db_key(pool: &sqlx::PgPool) -> (DbKey, Crs) {
    let db_key_cache = DbKeyCache::new(100).unwrap();
    let crc_cache = CrsCache::load(pool).await.expect("load crs cache");
    (
        db_key_cache
            .fetch_latest_from_pool(pool)
            .await
            .expect("fetch latest db key"),
        crc_cache.get_latest().expect("fetch latest CRS").clone(),
    )
}

pub async fn decrypt_ciphertexts(
    pool: &sqlx::PgPool,
    input: Vec<Vec<u8>>,
) -> Result<Vec<DecryptionResult>, Box<dyn std::error::Error>> {
    let (key, _) = latest_db_key(pool).await;

    let mut ct_indexes: BTreeMap<&[u8], usize> = BTreeMap::new();
    for (idx, h) in input.iter().enumerate() {
        ct_indexes.insert(h.as_slice(), idx);
    }
    use sqlx::Row;
    let rows = sqlx::query(
        "
            SELECT ciphertext, ciphertext_type, handle
            FROM ciphertexts_branch
            WHERE handle = ANY($1::BYTEA[])
              AND ciphertext_version = $2
        ",
    )
    .bind(&input)
    .bind(current_ciphertext_version())
    .fetch_all(pool)
    .await?;
    if rows.is_empty() {
        panic!("ciphertext not found");
    }

    let cts: Vec<(Vec<u8>, i16, Vec<u8>)> = rows
        .into_iter()
        .map(|row| -> Result<_, sqlx::Error> {
            Ok((
                row.try_get::<Vec<u8>, _>("ciphertext")?,
                row.try_get::<i16, _>("ciphertext_type")?,
                row.try_get::<Vec<u8>, _>("handle")?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut values = tokio::task::spawn_blocking(move || {
        let client_key = key.cks.unwrap();
        let sks = key.sks;
        tfhe::set_server_key(sks);

        let mut decrypted: Vec<(Vec<u8>, DecryptionResult)> = Vec::with_capacity(cts.len());
        for (ciphertext, ciphertext_type, handle) in cts {
            let deserialized =
                SupportedFheCiphertexts::decompress_no_memcheck(ciphertext_type, &ciphertext)
                    .unwrap();
            decrypted.push((
                handle,
                DecryptionResult {
                    output_type: ciphertext_type,
                    value: deserialized.decrypt(&client_key),
                },
            ));
        }

        decrypted
    })
    .await
    .unwrap();

    values.sort_by_key(|(h, _)| ct_indexes.get(h.as_slice()).unwrap());

    let values = values.into_iter().map(|i| i.1).collect::<Vec<_>>();
    Ok(values)
}
