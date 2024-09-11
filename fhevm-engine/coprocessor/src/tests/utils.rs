use crate::cli::Args;
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, deserialize_fhe_ciphertext};
use rand::RngCore;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU16, Ordering};
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};

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
}

pub fn default_api_key() -> &'static str {
    "a1503fb6-d79b-4e9e-826d-44cf262f3e05"
}

pub fn default_tenant_id() -> i32 {
    1
}

pub async fn setup_test_app() -> Result<TestInstance, Box<dyn std::error::Error>> {
    if std::env::var("COPROCESSOR_TEST_LOCALHOST").is_ok() {
        setup_test_app_existing_localhost().await
    } else {
        setup_test_app_custom_docker().await
    }
}

pub async fn setup_test_app_existing_localhost() -> Result<TestInstance, Box<dyn std::error::Error>>
{
    Ok(TestInstance {
        _container: None,
        app_close_channel: None,
        app_url: "http://127.0.0.1:50051".to_string(),
        db_url: "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor".to_string(),
    })
}

pub async fn setup_test_app_custom_docker() -> Result<TestInstance, Box<dyn std::error::Error>> {
    static PORT_COUNTER: AtomicU16 = AtomicU16::new(10000);

    let app_port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
    // wrap around, if we ever have that many tests?
    if app_port >= 50000 {
        PORT_COUNTER.store(10000, Ordering::SeqCst);
    }

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

    println!("DB prepared");

    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    let args: Args = Args {
        run_bg_worker: true,
        run_server: true,
        generate_fhe_keys: false,
        server_maximum_ciphertexts_to_schedule: 5000,
        work_items_batch_size: 40,
        tenant_key_cache_size: 4,
        coprocessor_fhe_threads: 4,
        maximum_handles_per_input: 255,
        tokio_threads: 2,
        pg_pool_max_connections: 2,
        server_addr: format!("127.0.0.1:{app_port}"),
        database_url: Some(db_url.clone()),
        maximimum_compact_inputs_upload: 10,
        coprocessor_private_key: "./coprocessor.key".to_string(),
    };

    std::thread::spawn(move || {
        crate::start_runtime(args, Some(rx));
    });

    // wait until app port is opened
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    Ok(TestInstance {
        _container: Some(container),
        app_close_channel: Some(app_close_channel),
        app_url: format!("http://127.0.0.1:{app_port}"),
        db_url,
    })
}

pub async fn wait_until_all_ciphertexts_computed(
    test_instance: &TestInstance,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(test_instance.db_url())
        .await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        let count = sqlx::query!("SELECT count(*) FROM computations WHERE NOT is_completed")
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

pub async fn decrypt_ciphertexts(
    pool: &sqlx::PgPool,
    tenant_id: i32,
    input: Vec<Vec<u8>>,
) -> Result<Vec<DecryptionResult>, Box<dyn std::error::Error>> {
    let mut priv_key = sqlx::query!(
        "
            SELECT cks_key
            FROM tenants
            WHERE tenant_id = $1
        ",
        tenant_id
    )
    .fetch_all(pool)
    .await?;

    if priv_key.is_empty() || priv_key[0].cks_key.is_none() {
        panic!("tenant private key not found");
    }

    let mut ct_indexes: BTreeMap<&[u8], usize> = BTreeMap::new();
    for (idx, h) in input.iter().enumerate() {
        ct_indexes.insert(h.as_slice(), idx);
    }

    assert_eq!(priv_key.len(), 1);

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

    let priv_key = priv_key.pop().unwrap().cks_key.unwrap();

    let mut values = tokio::task::spawn_blocking(move || {
        let client_key: tfhe::ClientKey = bincode::deserialize(&priv_key).unwrap();

        let mut decrypted: Vec<(Vec<u8>, DecryptionResult)> = Vec::with_capacity(cts.len());
        for ct in cts {
            let deserialized =
                deserialize_fhe_ciphertext(ct.ciphertext_type, &ct.ciphertext).unwrap();
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

pub fn random_handle_start() -> u64 {
    rand::thread_rng().next_u64()
}
