use fhevm_engine_common::utils::DatabaseURL;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, Connection, Executor, PgConnection};
use std::sync::Arc;
use test_harness::db_utils::setup_test_key;
use test_harness::instance::ImportMode;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::{OnceCell, Semaphore, SemaphorePermit};
use tracing::{info, warn};

const PG_URL_ENV: &str = "COPROCESSOR_TEST_PG_URL";

const POSTGRES_PORT: u16 = 5432;

/// Any fixed number, the same in every process.
const TEMPLATE_ADVISORY_LOCK: i64 = 0x7f6e_5d4c_3b2a_1908;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

static SERVER: OnceCell<SharedServer> = OnceCell::const_new();

struct SharedServer {
    /// Kept so the container outlives the tests.
    _container: Option<Arc<testcontainers::ContainerAsync<GenericImage>>>,
    admin_url: String,
}

async fn server() -> Result<&'static SharedServer, BoxError> {
    SERVER
        .get_or_try_init(|| async {
            // Blank counts as unset: CI sets it for every job, fills it in for some.
            if let Some(url) = std::env::var(PG_URL_ENV)
                .ok()
                .filter(|u| !u.trim().is_empty())
            {
                info!("Using external test Postgres from {PG_URL_ENV}");
                return Ok(SharedServer {
                    _container: None,
                    admin_url: url,
                });
            }

            let container = GenericImage::new("postgres", "15.7")
                .with_exposed_port(POSTGRES_PORT.into())
                .with_wait_for(WaitFor::message_on_stderr(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_USER", "postgres")
                .with_env_var("POSTGRES_PASSWORD", "postgres")
                .start()
                .await?;

            let host = container.get_host().await?;
            let port = container.get_host_port_ipv4(POSTGRES_PORT).await?;
            info!("Started shared test Postgres container on {host}:{port}");

            Ok(SharedServer {
                _container: Some(Arc::new(container)),
                admin_url: format!("postgresql://postgres:postgres@{host}:{port}/postgres"),
            })
        })
        .await
}

fn template_name(mode: &ImportMode) -> &'static str {
    match mode {
        ImportMode::SkipMigrations => "tmpl_bare",
        ImportMode::None => "tmpl_migrated",
        ImportMode::WithKeysNoSns => "tmpl_keys_no_sns",
        ImportMode::WithAllKeys => "tmpl_keys_all",
    }
}

fn with_database(admin_url: &str, db_name: &str) -> Result<String, BoxError> {
    let opts: PgConnectOptions = admin_url.parse()?;
    Ok(opts.database(db_name).to_url_lossy().to_string())
}

/// A database of this test's own, held by an advisory lock that Postgres frees on drop,
/// panic or kill. `sweep_orphans` reclaims whatever is left unlocked.
pub struct ClonedDb {
    pub db_url: DatabaseURL,
    db_name: String,
    _owner: PgConnection,
    _keyed_slot: Option<SemaphorePermit<'static>>,
}

impl ClonedDb {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }
}

const MAX_KEYED_TESTS: usize = 4;
static KEYED_TESTS: Semaphore = Semaphore::const_new(MAX_KEYED_TESTS);

/// Reclaims databases no test holds a lock on. Runs before each clone.
async fn sweep_orphans(conn: &mut PgConnection) -> Result<(), BoxError> {
    let names: Vec<String> =
        sqlx::query_scalar(r"SELECT datname FROM pg_database WHERE datname LIKE 'test\_%'")
            .fetch_all(&mut *conn)
            .await
            .unwrap_or_default();

    let mut reclaimed = 0usize;
    for name in names {
        let taken: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock(hashtext($1))")
            .bind(&name)
            .fetch_one(&mut *conn)
            .await
            .unwrap_or(false);
        if !taken {
            continue; // a test still owns it
        }
        match conn
            .execute(format!(r#"DROP DATABASE IF EXISTS "{name}" WITH (FORCE)"#).as_str())
            .await
        {
            Ok(_) => reclaimed += 1,
            Err(err) => warn!(db_name = %name, error = %err, "could not reclaim database"),
        }
        let _ = sqlx::query("SELECT pg_advisory_unlock(hashtext($1))")
            .bind(&name)
            .execute(&mut *conn)
            .await;
    }
    if reclaimed > 0 {
        info!(reclaimed, "reclaimed abandoned test databases");
    }
    Ok(())
}

/// `None` if absent, else whether the import finished.
async fn template_state(conn: &mut PgConnection, name: &str) -> Result<Option<bool>, BoxError> {
    let found: Option<bool> =
        sqlx::query_scalar("SELECT datistemplate FROM pg_database WHERE datname = $1")
            .bind(name)
            .fetch_optional(&mut *conn)
            .await?;
    Ok(found)
}

/// Imports the template unless it exists. The lock is in Postgres because each test is
/// a separate process.
async fn ensure_template(admin_url: &str, mode: &ImportMode) -> Result<String, BoxError> {
    let template = template_name(mode);

    let mut conn = PgConnection::connect(admin_url).await?;
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(TEMPLATE_ADVISORY_LOCK)
        .execute(&mut conn)
        .await?;

    let result = seed_template_locked(admin_url, mode, template, &mut conn).await;

    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(TEMPLATE_ADVISORY_LOCK)
        .execute(&mut conn)
        .await?;
    conn.close().await?;

    result.map(|()| template.to_string())
}

async fn seed_template_locked(
    admin_url: &str,
    mode: &ImportMode,
    template: &str,
    conn: &mut PgConnection,
) -> Result<(), BoxError> {
    match template_state(conn, template).await? {
        Some(true) => return Ok(()),
        Some(false) => {
            warn!(template, "Discarding half-seeded template database");
            conn.execute(format!(r#"DROP DATABASE IF EXISTS "{template}" WITH (FORCE)"#).as_str())
                .await?;
        }
        None => {}
    }

    info!(template, "Seeding test template database (once per server)");
    conn.execute(format!(r#"CREATE DATABASE "{template}""#).as_str())
        .await?;

    let template_url = with_database(admin_url, template)?;
    {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&template_url)
            .await?;

        match mode {
            ImportMode::SkipMigrations => {}
            ImportMode::None => {
                sqlx::migrate!("./migrations").run(&pool).await?;
            }
            ImportMode::WithKeysNoSns => {
                sqlx::migrate!("./migrations").run(&pool).await?;
                setup_test_key(&pool, false)
                    .await
                    .map_err(|err| format!("setup_test_key: {err}"))?;
            }
            ImportMode::WithAllKeys => {
                sqlx::migrate!("./migrations").run(&pool).await?;
                setup_test_key(&pool, true)
                    .await
                    .map_err(|err| format!("setup_test_key: {err}"))?;
            }
        }

        // Postgres will not copy a database while anything is connected.
        pool.close().await;
    }

    // Blocks new connections so nothing can block a copy, and marks the import
    // finished, so it must come last.
    conn.execute(
        format!(
            "UPDATE pg_database SET datistemplate = true, datallowconn = false \
             WHERE datname = '{template}'"
        )
        .as_str(),
    )
    .await?;

    info!(template, "Template ready");
    Ok(())
}

fn unique_db_name() -> String {
    format!("test_{:016x}", rand::random::<u64>())
}

/// A database of the caller's own. The first call imports, the rest copy.
pub async fn cloned_test_db(mode: ImportMode) -> Result<ClonedDb, Box<dyn std::error::Error>> {
    cloned_test_db_inner(mode)
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { err.to_string().into() })
}

async fn cloned_test_db_inner(mode: ImportMode) -> Result<ClonedDb, BoxError> {
    // Before anything else, so a test waits rather than piling on another key.
    let keyed_slot = match mode {
        ImportMode::WithKeysNoSns | ImportMode::WithAllKeys => Some(KEYED_TESTS.acquire().await?),
        ImportMode::None | ImportMode::SkipMigrations => None,
    };

    let server = server().await?;
    let template = ensure_template(&server.admin_url, &mode).await?;
    let db_name = unique_db_name();

    let mut owner = PgConnection::connect(&server.admin_url).await?;
    let _ = sweep_orphans(&mut owner).await;

    // Before the database exists, or another sweep could see it unlocked and drop it.
    sqlx::query("SELECT pg_advisory_lock(hashtext($1))")
        .bind(&db_name)
        .execute(&mut owner)
        .await?;

    // Postgres copies one at a time.
    let mut attempt = 0;
    loop {
        let created = owner
            .execute(format!(r#"CREATE DATABASE "{db_name}" TEMPLATE "{template}""#).as_str())
            .await;
        match created {
            Ok(_) => break,
            Err(err) if attempt < 10 => {
                attempt += 1;
                warn!(attempt, error = %err, "clone contended, retrying");
                tokio::time::sleep(std::time::Duration::from_millis(500 * attempt)).await;
            }
            Err(err) => return Err(err.into()),
        }
    }

    let db_url = with_database(&server.admin_url, &db_name)?;
    info!(db_name, "Cloned test database from {template}");

    Ok(ClonedDb {
        db_url: db_url.into(),
        db_name,
        _owner: owner,
        _keyed_slot: keyed_slot,
    })
}
