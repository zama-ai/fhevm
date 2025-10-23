use std::sync::Arc;

use crate::db_utils::setup_test_user;
use fhevm_engine_common::utils::DatabaseURL;
use sqlx::postgres::types::Oid;
use sqlx::Row;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Clone)]
pub struct DBInstance {
    _container: Option<Arc<testcontainers::ContainerAsync<testcontainers::GenericImage>>>,
    pub db_url: DatabaseURL,
    pub parent_token: CancellationToken,
}

impl DBInstance {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }
}

/// Sets up a test database instance.
///
/// If `COPROCESSOR_TEST_LOCALHOST` is set, it sets up a test database using an existing local PostgreSQL instance.
/// Otherwise, it sets up a test database using a custom Docker container running PostgreSQL 15.7.
///
/// # Returns
///
/// A `Result` containing a `DBInstance` on success. Dropping this instance terminates the database container.
///
///
/// # Examples
///
/// ```ignore
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db_instance = setup_test_db().await?;
///     println!("Test DB URL: {}", db_instance.db_url());
///     Ok(())
/// }
/// ```
pub async fn setup_test_db(mode: ImportMode) -> Result<DBInstance, Box<dyn std::error::Error>> {
    let is_localhost: bool = std::env::var("COPROCESSOR_TEST_LOCALHOST").is_ok();

    // Drop and recreate the database in localhost mode
    // This is useful for running tests locally with applying latest migrations
    let is_localhost_with_reset = std::env::var("COPROCESSOR_TEST_LOCALHOST_RESET").is_ok();

    if is_localhost || is_localhost_with_reset {
        setup_test_app_existing_localhost(is_localhost_with_reset, mode).await
    } else {
        setup_test_app_custom_docker(mode).await
    }
}

async fn setup_test_app_existing_localhost(
    with_reset: bool,
    mode: ImportMode,
) -> Result<DBInstance, Box<dyn std::error::Error>> {
    let db_url = DatabaseURL::default();

    if with_reset {
        info!("Resetting local database at {db_url}");
        let admin_db_url = db_url.to_string().replace("coprocessor", "postgres");
        create_database(&admin_db_url, db_url.as_str(), mode).await?;
    }

    info!("Using existing local database at {db_url}");

    let _ = get_sns_pk_size(&sqlx::PgPool::connect(db_url.as_str()).await?, 12345).await;

    Ok(DBInstance {
        _container: None,
        db_url,
        parent_token: CancellationToken::new(),
    })
}

async fn setup_test_app_custom_docker(
    mode: ImportMode,
) -> Result<DBInstance, Box<dyn std::error::Error>> {
    let container = GenericImage::new("postgres", "15.7")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await
        .expect("postgres started");

    info!("Postgres container started");

    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;

    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/coprocessor");
    create_database(&admin_db_url, &db_url, mode).await?;

    Ok(DBInstance {
        _container: Some(Arc::new(container)),
        db_url: db_url.into(),
        parent_token: CancellationToken::new(),
    })
}

pub enum ImportMode {
    None,
    WithKeysNoSns,
    WithAllKeys,
}

async fn create_database(
    admin_db_url: &str,
    db_url: &str,
    mode: ImportMode,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Creating coprocessor db...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(admin_db_url)
        .await?;

    sqlx::query!("DROP DATABASE IF EXISTS coprocessor;")
        .execute(&admin_pool)
        .await?;

    sqlx::query!("CREATE DATABASE coprocessor;")
        .execute(&admin_pool)
        .await?;

    info!(db_url, "Created database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    match mode {
        ImportMode::None => {
            info!("No keys imported");
        }
        ImportMode::WithKeysNoSns => {
            info!("Creating test user with keys, without SnS key...");
            setup_test_user(&pool, false).await?;
        }
        ImportMode::WithAllKeys => {
            info!("Creating test user with all keys...");
            setup_test_user(&pool, true).await?;
        }
    }

    info!("Database initialized");

    Ok(())
}

pub async fn get_sns_pk_size(pool: &sqlx::PgPool, chain_id: i64) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT sns_pk FROM tenants WHERE chain_id = $1")
        .bind(chain_id)
        .fetch_one(pool)
        .await?;

    let oid: Oid = row.try_get(0)?;
    info!(oid = ?oid, chain_id, "Found sns_pk oid");
    let row = sqlx::query_scalar(
        "SELECT COALESCE(SUM(octet_length(data))::bigint, 0) FROM pg_largeobject WHERE loid = $1",
    )
    .bind(oid)
    .fetch_one(pool)
    .await?;

    info!(size = ?bytesize::ByteSize::b(row as u64), "Found sns_pk large object");
    Ok(row)
}
