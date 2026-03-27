use std::sync::Arc;

use crate::db_utils::setup_test_key;
use fhevm_engine_common::utils::DatabaseURL;
use sqlx::postgres::types::Oid;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, Row};
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

fn connect_options(db_url: &str) -> PgConnectOptions {
    db_url.parse().expect("database URL should be valid")
}

fn extract_db_name(db_url: &str) -> String {
    connect_options(db_url)
        .get_database()
        .expect("database URL must contain a database name")
        .to_owned()
}

fn admin_url_from(db_url: &str) -> String {
    connect_options(db_url)
        .database("postgres")
        .to_url_lossy()
        .to_string()
}

async fn setup_test_app_existing_localhost(
    with_reset: bool,
    mode: ImportMode,
) -> Result<DBInstance, Box<dyn std::error::Error>> {
    let db_url = DatabaseURL::default();

    // SkipMigrations always needs a fresh DB with no migrations.
    let needs_reset = with_reset || matches!(mode, ImportMode::SkipMigrations);

    if needs_reset {
        info!("Resetting local database at {db_url}");
        let admin_db_url = admin_url_from(db_url.as_str());
        create_database(&admin_db_url, db_url.as_str(), mode.clone()).await?;
    }

    info!("Using existing local database at {db_url}");

    let pool = sqlx::PgPool::connect(db_url.as_str()).await?;

    if !needs_reset {
        // Check if the schema is intact. A prior SkipMigrations test (in the
        // same binary) may have dropped+recreated the DB with partial migrations
        // via raw_sql, leaving _sqlx_migrations missing or incomplete.
        let migrator = sqlx::migrate!("./migrations");
        let expected = migrator
            .migrations
            .iter()
            .filter(|m| !m.migration_type.is_down_migration())
            .count() as i64;
        let applied: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        if applied != expected {
            info!(
                applied,
                expected, "Schema is incomplete, forcing drop+create..."
            );
            // Close our connection before dropping the database.
            pool.close().await;
            let admin_db_url = admin_url_from(db_url.as_str());
            create_database(&admin_db_url, db_url.as_str(), mode).await?;
            let pool = sqlx::PgPool::connect(db_url.as_str()).await?;
            let _ = get_sns_pk_size(&pool).await;
            return Ok(DBInstance {
                _container: None,
                db_url,
                parent_token: CancellationToken::new(),
            });
        }

        match mode {
            ImportMode::None => {
                info!("Truncating all data tables for clean test state...");
                sqlx::query(
                    "TRUNCATE keys, host_chains, crs, transactions, computations, \
                     allowed_handles, host_chain_blocks_valid, host_listener_poller_state, \
                     dependence_chain, delegate_user_decrypt, ciphertexts, ciphertexts128, \
                     ciphertext_digest, pbs_computations, input_blobs, verify_proofs CASCADE",
                )
                .execute(&pool)
                .await?;
            }
            ImportMode::WithKeysNoSns => {
                info!("Loading test keys (without SnS) into existing database...");
                sqlx::query("TRUNCATE keys, host_chains, crs CASCADE")
                    .execute(&pool)
                    .await?;
                setup_test_key(&pool, false).await?;
            }
            ImportMode::WithAllKeys => {
                info!("Loading test keys (with all keys) into existing database...");
                sqlx::query("TRUNCATE keys, host_chains, crs CASCADE")
                    .execute(&pool)
                    .await?;
                setup_test_key(&pool, true).await?;
            }
            _ => {}
        }
    }

    let _ = get_sns_pk_size(&pool).await;

    Ok(DBInstance {
        _container: None,
        db_url,
        parent_token: CancellationToken::new(),
    })
}

const POSTGRES_PORT: u16 = 5432;

async fn setup_test_app_custom_docker(
    mode: ImportMode,
) -> Result<DBInstance, Box<dyn std::error::Error>> {
    let container = GenericImage::new("postgres", "15.7")
        .with_exposed_port(POSTGRES_PORT.into())
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
    let cont_port = container.get_host_port_ipv4(POSTGRES_PORT).await?;

    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/coprocessor");
    let admin_db_url = admin_url_from(&db_url);
    create_database(&admin_db_url, &db_url, mode).await?;

    Ok(DBInstance {
        _container: Some(Arc::new(container)),
        db_url: db_url.into(),
        parent_token: CancellationToken::new(),
    })
}

#[derive(Clone)]
pub enum ImportMode {
    None,
    WithKeysNoSns,
    WithAllKeys,
    SkipMigrations,
}

async fn create_database(
    admin_db_url: &str,
    db_url: &str,
    mode: ImportMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_name = extract_db_name(db_url);
    info!(db_name, "Creating database...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(admin_db_url)
        .await?;

    sqlx::query(&format!("DROP DATABASE IF EXISTS \"{db_name}\""))
        .execute(&admin_pool)
        .await?;

    sqlx::query(&format!("CREATE DATABASE \"{db_name}\""))
        .execute(&admin_pool)
        .await?;

    info!(db_url, "Created database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    match mode {
        ImportMode::SkipMigrations => {
            info!("Skipping migrations");
        }
        ImportMode::None => {
            sqlx::migrate!("./migrations").run(&pool).await?;
            info!("No keys imported");
        }
        ImportMode::WithKeysNoSns => {
            sqlx::migrate!("./migrations").run(&pool).await?;
            info!("Creating test keys, without SnS key...");
            setup_test_key(&pool, false).await?;
        }
        ImportMode::WithAllKeys => {
            sqlx::migrate!("./migrations").run(&pool).await?;
            info!("Creating test keys with all keys...");
            setup_test_key(&pool, true).await?;
        }
    }

    info!("Database initialized");

    Ok(())
}

pub async fn get_sns_pk_size(pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT sns_pk FROM keys ORDER BY sequence_number DESC LIMIT 1")
        .fetch_optional(pool)
        .await?;

    let Some(row) = row else {
        info!("No sns_pk found in keys");
        return Ok(0);
    };

    let oid: Oid = row.try_get(0)?;
    info!(oid = ?oid, "Found sns_pk oid");
    let row = sqlx::query_scalar(
        "SELECT COALESCE(SUM(octet_length(data))::bigint, 0) FROM pg_largeobject WHERE loid = $1",
    )
    .bind(oid)
    .fetch_one(pool)
    .await?;

    info!(size = ?bytesize::ByteSize::b(row as u64), "Found sns_pk large object");
    Ok(row)
}
