use crate::cli::Args;
use std::sync::atomic::{AtomicU16, Ordering};
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::watch::Receiver;

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
    } else if std::env::var("COPROCESSOR_TEST_LOCAL_DB").is_ok() {
        setup_test_app_existing_db().await
    } else {
        setup_test_app_custom_docker().await
    }
}

const LOCAL_DB_URL: &str = "postgresql://postgres:postgres@127.0.0.1:5432/coprocessor";

async fn setup_test_app_existing_localhost() -> Result<TestInstance, Box<dyn std::error::Error>> {
    Ok(TestInstance {
        _container: None,
        app_close_channel: None,
        app_url: "http://127.0.0.1:50051".to_string(),
        db_url: LOCAL_DB_URL.to_string(),
    })
}

pub async fn setup_test_app_existing_db() -> Result<TestInstance, Box<dyn std::error::Error>> {
    let app_port = get_app_port();
    let (app_close_channel, rx) = tokio::sync::watch::channel(false);
    start_coprocessor(rx, app_port, &LOCAL_DB_URL).await;
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
        database_url: Some(db_url.to_string()),
        maximimum_compact_inputs_upload: 10,
        coprocessor_private_key: "./coprocessor.key".to_string(),
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
