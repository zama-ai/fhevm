use crate::daemon_cli::Args;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::utils::{safe_deserialize, safe_deserialize_key};
use rand::Rng;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU16, Ordering};
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::watch::Receiver;
use tracing::Level;

pub struct TestInstance {
    // just to destroy container
    _container: Option<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
    // send message to this on destruction to stop the app
    app_close_channel: Option<tokio::sync::watch::Sender<bool>>,
    app_url: String,
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
    pub fn app_url(&self) -> &str {
        self.app_url.as_str()
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

pub fn default_tenant_id() -> i32 {
    1
}

pub fn default_dependence_cache_size() -> u16 {
    128
}

pub fn random_handle() -> u64 {
    rand::rng().random()
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
        app_url: "http://127.0.0.1:50051".to_string(),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let app_port = get_app_port();
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, app_port, LOCAL_DB_URL).await;
    Ok(TestInstance {
        _container: None,
        app_close_channel: Some(app_close_channel),
        app_url: format!("http://127.0.0.1:{app_port}"),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

async fn start_coprocessor(rx: Receiver<bool>, app_port: u16, db_url: &str) {
    let args: Args = Args {
        run_bg_worker: true,
        worker_polling_interval_ms: 1000,
        run_server: true,
        generate_fhe_keys: false,
        server_maximum_ciphertexts_to_schedule: 5000,
        server_maximum_ciphertexts_to_get: 5000,
        work_items_batch_size: 40,
        dependence_chains_per_batch: 10,
        tenant_key_cache_size: 4,
        coprocessor_fhe_threads: 4,
        maximum_handles_per_input: 255,
        tokio_threads: 2,
        pg_pool_max_connections: 2,
        server_addr: format!("127.0.0.1:{app_port}"),
        metrics_addr: None,
        database_url: Some(db_url.to_string()),
        maximum_compact_inputs_upload: 10,
        coprocessor_private_key: "./coprocessor.key".to_string(),
        service_name: "coprocessor".to_string(),
        log_level: Level::INFO,
        health_check_port: 8081,
    };

    std::thread::spawn(move || {
        crate::start_runtime(args, Some(rx));
    });

    // wait until app port is opened
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
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
    let app_port = get_app_port();

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
    println!("Creating test user");
    setup_test_user(&pool).await?;
    println!("DB prepared");

    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, app_port, &db_url).await;
    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        app_url: format!("http://127.0.0.1:{app_port}"),
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

pub async fn setup_test_user(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let (sks, cks, pks, pp) = if !cfg!(feature = "gpu") {
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
    let sks = tokio::fs::read(sks).await.expect("can't read sks key");
    let pks = tokio::fs::read(pks).await.expect("can't read pks key");
    let cks = tokio::fs::read(cks).await.expect("can't read cks key");
    let public_params = tokio::fs::read(pp).await.expect("can't read public params");
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
        &pks,
        &sks,
        &public_params,
        &cks,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn decrypt_ciphertexts(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    input: Vec<Vec<u8>>,
) -> Result<Vec<DecryptionResult>, Box<dyn std::error::Error>> {
    let mut keys = sqlx::query!(
        "
            SELECT cks_key, sks_key
            FROM tenants
            WHERE tenant_id = $1
        ",
        tenant_id
    )
    .fetch_all(pool)
    .await?;
    if keys.is_empty() || keys[0].cks_key.is_none() {
        panic!("tenant keys not found");
    }

    let mut ct_indexes: BTreeMap<&[u8], usize> = BTreeMap::new();
    for (idx, h) in input.iter().enumerate() {
        ct_indexes.insert(h.as_slice(), idx);
    }
    let cts = sqlx::query!(
        "
            SELECT ciphertext, ciphertext_type, handle
            FROM ciphertexts
            WHERE tenant_id = $1
            AND handle = ANY($2::BYTEA[])
            AND ciphertext_version = $3
        ",
        tenant_id,
        &input,
        current_ciphertext_version()
    )
    .fetch_all(pool)
    .await?;
    if cts.is_empty() {
        panic!("ciphertext not found");
    }

    assert_eq!(keys.len(), 1);
    let keys = keys.pop().unwrap();

    let mut values = tokio::task::spawn_blocking(move || {
        let cks = keys.cks_key.clone().unwrap();
        let client_key: tfhe::ClientKey = safe_deserialize(&cks).unwrap();
        #[cfg(not(feature = "gpu"))]
        let sks: tfhe::ServerKey = safe_deserialize_key(&keys.sks_key).unwrap();
        #[cfg(feature = "gpu")]
        let sks = {
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&keys.sks_key).unwrap();
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
