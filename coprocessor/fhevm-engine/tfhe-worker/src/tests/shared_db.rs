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

/// Fixed so repeated runs share one container rather than adding another each time.
const FIXTURE_CONTAINER: &str = "tfhe-worker-test-pg";
const FIXTURE_PORT: u16 = 55432;

const TEST_DB_PREFIX: &str = "tfhe_worker_test_";
const TEST_DB_MARKER: &str = "fhevm:tfhe-worker:test-database";
const TEMPLATE_MARKER_PREFIX: &str = "fhevm:tfhe-worker:test-template";

fn fixture_url() -> String {
    format!("postgresql://postgres:postgres@127.0.0.1:{FIXTURE_PORT}/postgres")
}

async fn running_fixture_url() -> Option<String> {
    let url = fixture_url();
    PgConnection::connect(&url).await.ok()?.close().await.ok()?;
    Some(url)
}

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

            // A fixed name and port, so every run and every test process reuses one
            // container instead of leaving one behind each time.
            if let Some(url) = running_fixture_url().await {
                info!("Reusing test Postgres container {FIXTURE_CONTAINER}");
                return Ok(SharedServer {
                    _container: None,
                    admin_url: url,
                });
            }

            let container = GenericImage::new("postgres", "15.7")
                .with_wait_for(WaitFor::message_on_stderr(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_USER", "postgres")
                .with_env_var("POSTGRES_PASSWORD", "postgres")
                .with_mapped_port(FIXTURE_PORT, POSTGRES_PORT.into())
                .with_container_name(FIXTURE_CONTAINER)
                .start()
                .await;

            match container {
                Ok(container) => {
                    let host = container.get_host().await?;
                    info!("Started test Postgres container on {host}:{FIXTURE_PORT}");
                    Ok(SharedServer {
                        _container: Some(Arc::new(container)),
                        admin_url: fixture_url(),
                    })
                }
                // Another process won the race and created it first.
                Err(err) => match running_fixture_url().await {
                    Some(url) => Ok(SharedServer {
                        _container: None,
                        admin_url: url,
                    }),
                    None => Err(err.into()),
                },
            }
        })
        .await
}

fn template_name(mode: &ImportMode) -> &'static str {
    match mode {
        ImportMode::SkipMigrations => "tfhe_worker_tmpl_bare",
        ImportMode::None => "tfhe_worker_tmpl_migrated",
        ImportMode::WithKeysNoSns => "tfhe_worker_tmpl_keys_no_sns",
        ImportMode::WithAllKeys => "tfhe_worker_tmpl_keys_all",
    }
}

fn template_marker() -> Result<String, BoxError> {
    let metadata = std::fs::metadata(std::env::current_exe()?)?;
    let modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?;
    Ok(format!(
        "{TEMPLATE_MARKER_PREFIX}:{}:{}",
        metadata.len(),
        modified.as_nanos()
    ))
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

// Key-loading tests at once per process.
const MAX_CONCURRENT_KEY_TESTS: usize = 4;
static KEY_TESTS: Semaphore = Semaphore::const_new(MAX_CONCURRENT_KEY_TESTS);

/// Reclaims databases no test holds a lock on. Runs before each clone.
async fn sweep_orphans(conn: &mut PgConnection) -> Result<(), BoxError> {
    let names: Vec<String> = sqlx::query_scalar(
        "SELECT datname FROM pg_database \
         WHERE datname LIKE $1 \
           AND shobj_description(oid, 'pg_database') = $2",
    )
    .bind(format!("{TEST_DB_PREFIX}%"))
    .bind(TEST_DB_MARKER)
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

/// `None` if absent, else whether the import finished and its ownership marker.
async fn template_state(
    conn: &mut PgConnection,
    name: &str,
) -> Result<Option<(bool, Option<String>)>, BoxError> {
    let found = sqlx::query_as(
        "SELECT datistemplate, shobj_description(oid, 'pg_database') \
         FROM pg_database WHERE datname = $1",
    )
    .bind(name)
    .fetch_optional(&mut *conn)
    .await?;
    Ok(found)
}

async fn comment_database(
    conn: &mut PgConnection,
    name: &str,
    marker: &str,
) -> Result<(), BoxError> {
    conn.execute(format!(r#"COMMENT ON DATABASE "{name}" IS '{marker}'"#).as_str())
        .await?;
    Ok(())
}

async fn seed_template_locked(
    admin_url: &str,
    mode: &ImportMode,
    template: &str,
    marker: &str,
    conn: &mut PgConnection,
) -> Result<(), BoxError> {
    match template_state(conn, template).await? {
        Some((true, Some(found))) if found == marker => return Ok(()),
        Some((_, Some(found))) if found.starts_with(TEMPLATE_MARKER_PREFIX) => {
            warn!(template, "Discarding stale test template database");
            conn.execute(
                format!(
                    "UPDATE pg_database SET datistemplate = false, datallowconn = false \
                     WHERE datname = '{template}'"
                )
                .as_str(),
            )
            .await?;
            conn.execute(format!(r#"DROP DATABASE IF EXISTS "{template}" WITH (FORCE)"#).as_str())
                .await?;
        }
        Some(_) => {
            return Err(format!(
                "refusing to replace database {template}: test-template marker is missing"
            )
            .into());
        }
        None => {}
    }

    info!(template, "Seeding test template database (once per server)");
    conn.execute(format!(r#"CREATE DATABASE "{template}""#).as_str())
        .await?;
    comment_database(conn, template, marker).await?;

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
    format!("{TEST_DB_PREFIX}{:016x}", rand::random::<u64>())
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
        ImportMode::WithKeysNoSns | ImportMode::WithAllKeys => Some(KEY_TESTS.acquire().await?),
        ImportMode::None | ImportMode::SkipMigrations => None,
    };

    let server = server().await?;
    let template = template_name(&mode);
    let marker = template_marker()?;
    let db_name = unique_db_name();

    let mut owner = PgConnection::connect(&server.admin_url).await?;
    let _ = sweep_orphans(&mut owner).await;

    // Keep the template stable until this clone finishes. The lock is in Postgres
    // because nextest runs every test in a separate process.
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(TEMPLATE_ADVISORY_LOCK)
        .execute(&mut owner)
        .await?;

    let clone_result = async {
        seed_template_locked(&server.admin_url, &mode, template, &marker, &mut owner).await?;

        // Before the database exists, or another sweep could see it unlocked and drop it.
        sqlx::query("SELECT pg_advisory_lock(hashtext($1))")
            .bind(&db_name)
            .execute(&mut owner)
            .await?;

        owner
            .execute(format!(r#"CREATE DATABASE "{db_name}" TEMPLATE "{template}""#).as_str())
            .await?;
        comment_database(&mut owner, &db_name, TEST_DB_MARKER).await
    }
    .await;

    let unlock_result = sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(TEMPLATE_ADVISORY_LOCK)
        .execute(&mut owner)
        .await;
    clone_result?;
    unlock_result?;

    let db_url = with_database(&server.admin_url, &db_name)?;
    info!(db_name, "Cloned test database from {template}");

    Ok(ClonedDb {
        db_url: db_url.into(),
        db_name,
        _owner: owner,
        _keyed_slot: keyed_slot,
    })
}
