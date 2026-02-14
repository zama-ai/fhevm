use crate::daemon_cli::Args;
use fhevm_engine_common::crs::{Crs, CrsCache};
use fhevm_engine_common::db_keys::{DbKey, DbKeyCache};
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use std::collections::BTreeMap;
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
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, LOCAL_DB_URL).await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn start_coprocessor(rx: Receiver<bool>, db_url: &str) {
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 1000,
        generate_fhe_keys: false,
        work_items_batch_size: 40,
        dependence_chains_per_batch: 10,
        key_cache_size: 4,
        coprocessor_fhe_threads: 4,
        tokio_threads: 2,
        pg_pool_max_connections: 2,
        metrics_addr: None,
        database_url: Some(db_url.into()),
        service_name: "coprocessor".to_string(),
        log_level: Level::INFO,
        health_check_port: 8081,
        metric_rerand_batch_latency: MetricsConfig::default(),
        metric_fhe_batch_latency: MetricsConfig::default(),
        worker_id: None,
        dcid_ttl_sec: 30,
        disable_dcid_locking: true,
        dcid_timeslice_sec: 90,
        dcid_cleanup_interval_sec: 0,
        processed_dcid_ttl_sec: 0,
        dcid_max_no_progress_cycles: 2,
        dcid_ignore_dependency_count_threshold: 100,
    };

    std::thread::spawn(move || {
        crate::start_runtime(args, Some(rx));
    });

    // wait until worker starts
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
}

async fn setup_test_app_custom_docker() -> Result<TestInstance, Box<dyn std::error::Error>> {
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
    start_coprocessor(rx, &db_url).await;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        db_url,
    })
}

pub async fn wait_until_all_allowed_handles_computed(
    test_instance: &TestInstance,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(test_instance.db_url())
        .await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let count = sqlx::query!(
            "SELECT count(1) FROM computations WHERE is_allowed = TRUE AND is_completed = FALSE"
        )
        .fetch_one(&pool)
        .await?;
        let current_count = count.count.unwrap();
        if current_count == 0 {
            println!("All computations completed");
            break;
        } else {
            println!("{current_count} computations remaining, waiting...");
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
            .fetch_latest(pool)
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
    let cts = sqlx::query!(
        "
            SELECT ciphertext, ciphertext_type, handle
            FROM ciphertexts
            WHERE handle = ANY($1::BYTEA[])
            AND ciphertext_version = $2
        ",
        &input,
        current_ciphertext_version()
    )
    .fetch_all(pool)
    .await?;
    if cts.is_empty() {
        panic!("ciphertext not found");
    }

    let mut values = tokio::task::spawn_blocking(move || {
        let client_key = key.cks.unwrap();
        #[cfg(not(feature = "gpu"))]
        let sks = key.sks;
        #[cfg(feature = "gpu")]
        let sks = key.csks.decompress();
        tfhe::set_server_key(sks);

        let mut decrypted: Vec<(Vec<u8>, DecryptionResult)> = Vec::with_capacity(cts.len());
        for ct in cts {
            let deserialized =
                SupportedFheCiphertexts::decompress_no_memcheck(ct.ciphertext_type, &ct.ciphertext)
                    .unwrap();
            decrypted.push((
                ct.handle,
                DecryptionResult {
                    output_type: ct.ciphertext_type,
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
