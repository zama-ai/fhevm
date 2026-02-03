use crate::daemon_cli::Args;
use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::safe_deserialize_key;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::watch::Receiver;
use tracing::Level;

struct TestKeys {
    sks: Vec<u8>,
    pks: Vec<u8>,
    cks: Vec<u8>,
    public_params: Vec<u8>,
}

static TEST_KEYS: OnceLock<TestKeys> = OnceLock::new();

fn is_git_lfs_pointer(content: &[u8]) -> bool {
    content.starts_with(b"version https://git-lfs.github.com/spec/v1")
}

fn test_keys() -> &'static TestKeys {
    TEST_KEYS.get_or_init(|| {
        let (sks_path, cks_path, pks_path, pp_path) = if !cfg!(feature = "gpu") {
            (
                "../fhevm-keys/sks",
                "../fhevm-keys/cks",
                "../fhevm-keys/pks",
                "../fhevm-keys/pp",
            )
        } else {
            (
                "../fhevm-keys/gpu-csks",
                "../fhevm-keys/gpu-cks",
                "../fhevm-keys/gpu-pks",
                "../fhevm-keys/gpu-pp",
            )
        };

        let read_or_none = |path: &str| std::fs::read(path).ok();
        let keys_from_disk = (|| {
            let sks = read_or_none(sks_path)?;
            let pks = read_or_none(pks_path)?;
            let cks = read_or_none(cks_path)?;
            let public_params = read_or_none(pp_path)?;
            if [
                sks.as_slice(),
                pks.as_slice(),
                cks.as_slice(),
                public_params.as_slice(),
            ]
            .iter()
            .any(|b| is_git_lfs_pointer(b))
            {
                return None;
            }
            Some(TestKeys {
                sks,
                pks,
                cks,
                public_params,
            })
        })();

        if let Some(keys_from_disk) = keys_from_disk {
            return keys_from_disk;
        }

        let keys: SerializedFhevmKeys = FhevmKeys::new().into();
        TestKeys {
            #[cfg(not(feature = "gpu"))]
            sks: keys.server_key_without_ns,
            #[cfg(feature = "gpu")]
            sks: keys.compressed_server_key,
            pks: keys.compact_public_key,
            cks: keys
                .client_key
                .expect("client key should be present in tests"),
            public_params: keys.public_params,
        }
    })
}

pub struct TestInstance {
    // just to destroy container
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    // send message to this on destruction to stop the app
    app_close_channel: Option<tokio::sync::watch::Sender<bool>>,
    health_url: String,
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
    pub fn health_url(&self) -> &str {
        self.health_url.as_str()
    }

    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub fn db_docker_id(&self) -> Option<String> {
        self._container.as_ref().map(|c| c.id().to_string())
    }
}

pub fn default_api_key() -> &'static str {
    "a1503fb6-d79b-4e9e-826d-44cf262f3e05"
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCALHOST").is_ok() {
        setup_test_app_existing_localhost().await
    } else if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

pub async fn setup_test_app_existing_localhost() -> Result<TestInstance, Box<dyn std::error::Error>>
{
    Ok(TestInstance {
        _container: None,
        app_close_channel: None,
        health_url: "http://127.0.0.1:8080".to_string(),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let health_port = get_app_port();
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, health_port, LOCAL_DB_URL).await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        health_url: format!("http://127.0.0.1:{health_port}"),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn start_coprocessor(rx: Receiver<bool>, health_port: u16, db_url: &str) {
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 200,
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
        health_check_port: health_port,
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

    // Give the runtime a moment to spin up
    tokio::time::sleep(Duration::from_millis(500)).await;
}

fn get_app_port() -> u16 {
    static PORT_COUNTER: AtomicU16 = AtomicU16::new(10000);

    let app_port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
    // wrap around, if we ever have that many tests?
    if app_port >= 50000 {
        PORT_COUNTER.store(10000, Ordering::SeqCst);
    }
    app_port
}

async fn setup_test_app_custom_docker() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let health_port = get_app_port();

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
    start_coprocessor(rx, health_port, &db_url).await;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        health_url: format!("http://127.0.0.1:{health_port}"),
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

    let timeout = Duration::from_secs(120);
    let start = std::time::Instant::now();
    loop {
        if start.elapsed() > timeout {
            return Err("timeout waiting for allowed computations to complete".into());
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
        let current_count: i64 = sqlx::query_scalar(
            "SELECT count(1) FROM computations WHERE is_allowed = TRUE AND is_completed = FALSE AND is_error = FALSE",
        )
        .fetch_one(&pool)
        .await?;
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

pub async fn setup_test_user(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let keys = test_keys();

    sqlx::query!(
        "
            INSERT INTO tenants(tenant_api_key, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params, cks_key)
            VALUES (
                'a1503fb6-d79b-4e9e-826d-44cf262f3e05',
                12345,
                '0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2',
                '0x69dE3158643e738a0724418b21a35FAA20CBb1c5',
                $1,
                $2,
                $3,
                $4
            )
        ",
        &keys.pks,
        &keys.sks,
        &keys.public_params,
        &keys.cks,
    )
}

pub async fn decrypt_ciphertexts(
    pool: &sqlx::PgPool,
    input: Vec<Vec<u8>>,
) -> Result<Vec<DecryptionResult>, Box<dyn std::error::Error>> {
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

    let keys = test_keys();

    let mut values = tokio::task::spawn_blocking(move || {
        let client_key: tfhe::ClientKey = safe_deserialize_key(&keys.cks).unwrap();
        #[cfg(not(feature = "gpu"))]
        let sks: tfhe::ServerKey = safe_deserialize_key(&keys.sks).unwrap();
        #[cfg(feature = "gpu")]
        let sks = {
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&keys.sks).unwrap();
            csks.decompress()
        };
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
