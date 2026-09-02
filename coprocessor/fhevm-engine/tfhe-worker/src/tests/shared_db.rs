//! Gives each test its own database, cheaply.
//!
//! Importing the FHE keys takes about 26 seconds, so doing it per test used to cost
//! more than the tests themselves. Instead we import once into a "template"
//! database, and every test gets a copy of it — Postgres copies the files directly,
//! which takes a second or two. Tests stay fully isolated, so they can run at the
//! same time.
//!
//! Set `COPROCESSOR_TEST_PG_URL` to reuse a running Postgres (CI starts one per job,
//! so all tests share the same import). Leave it unset and we start a container.

use fhevm_engine_common::utils::DatabaseURL;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, Connection, Executor, PgConnection};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use test_harness::db_utils::setup_test_key;
use test_harness::instance::ImportMode;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};
use tokio::sync::OnceCell;
use tracing::{info, warn};

/// Admin URL of a Postgres to reuse, e.g.
/// `postgresql://postgres:postgres@localhost:5432/postgres`.
const PG_URL_ENV: &str = "COPROCESSOR_TEST_PG_URL";

const POSTGRES_PORT: u16 = 5432;

/// Any fixed number; every process must use the same one to take the same lock.
const TEMPLATE_ADVISORY_LOCK: i64 = 0x7f6e_5d4c_3b2a_1908;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

static SERVER: OnceCell<SharedServer> = OnceCell::const_new();

struct SharedServer {
    /// Held so the container stays up for as long as the tests need it.
    /// `None` when reusing a Postgres we did not start.
    _container: Option<Arc<testcontainers::ContainerAsync<GenericImage>>>,
    admin_url: String,
}

async fn server() -> Result<&'static SharedServer, BoxError> {
    SERVER
        .get_or_try_init(|| async {
            // Blank counts as unset: CI defines the variable for every job but only
            // fills it in where a server is started.
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

/// One template per mode, since each mode imports different content.
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

/// A database of this test's own, deleted when dropped.
pub struct ClonedDb {
    pub db_url: DatabaseURL,
    db_name: String,
    admin_url: String,
}

impl ClonedDb {
    pub fn db_url(&self) -> &str {
        self.db_url.as_str()
    }

    pub fn db_name(&self) -> &str {
        &self.db_name
    }
}

impl Drop for ClonedDb {
    fn drop(&mut self) {
        // Copies are ~855 MB each, so failing to delete them fills the disk within
        // one run. The delete must finish before we return, and the obvious ways do
        // not work: a spawned task never runs, because the test process exits as soon
        // as the test does, and `block_in_place` panics on the single-threaded runtime
        // that `#[tokio::test]` sets up. So do it on a thread we wait for.
        let (admin_url, db_name) = (self.admin_url.clone(), self.db_name.clone());
        let worker = std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(err) => {
                    warn!(db_name, error = %err, "no runtime to drop test database");
                    return;
                }
            };
            if let Err(err) = runtime.block_on(drop_database(&admin_url, &db_name)) {
                warn!(db_name, error = %err, "could not drop test database");
            }
        });
        // A leftover database is not worth failing a passing test over.
        if worker.join().is_err() {
            warn!(db_name = %self.db_name, "test database cleanup thread panicked");
        }
    }
}

async fn drop_database(admin_url: &str, db_name: &str) -> Result<(), BoxError> {
    let mut conn = PgConnection::connect(admin_url).await?;
    // Never hang the end of a test: FORCE disconnects anything still attached, and
    // the timeout caps the wait if that is not enough.
    conn.execute("SET statement_timeout = '30s'").await?;
    conn.execute(format!(r#"DROP DATABASE IF EXISTS "{db_name}" WITH (FORCE)"#).as_str())
        .await?;
    conn.close().await?;
    Ok(())
}

/// `None` if the template does not exist, else whether its import finished.
async fn template_state(conn: &mut PgConnection, name: &str) -> Result<Option<bool>, BoxError> {
    let found: Option<bool> =
        sqlx::query_scalar("SELECT datistemplate FROM pg_database WHERE datname = $1")
            .bind(name)
            .fetch_optional(&mut *conn)
            .await?;
    Ok(found)
}

/// Imports the template for `mode` unless it already exists.
///
/// Uses a Postgres lock, not one in memory: each test is a separate process, so they
/// would otherwise all try to import at once.
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
    // A template without the flag was left behind by a process that died partway
    // through importing. Copying it would hand out databases with no keys, so throw
    // it away and import again.
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
                // Flatten to a string: this error type cannot cross an await.
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

        // Postgres refuses to copy a database while anything is connected to it.
        pool.close().await;
    }

    // Marking it a template blocks new connections, so nothing can get in the way of
    // a copy. This also signals that the import finished, so it must come last.
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
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    // Process id and counter are not enough on their own, because the system reuses
    // process ids and a name that is already taken cannot be retried.
    format!(
        "test_{}_{}_{:08x}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed),
        rand::random::<u32>()
    )
}

/// Gives the caller a database of its own, set up according to `mode`.
///
/// The first call imports the data; later calls copy it.
pub async fn cloned_test_db(mode: ImportMode) -> Result<ClonedDb, Box<dyn std::error::Error>> {
    cloned_test_db_inner(mode)
        .await
        .map_err(|err| -> Box<dyn std::error::Error> { err.to_string().into() })
}

async fn cloned_test_db_inner(mode: ImportMode) -> Result<ClonedDb, BoxError> {
    let server = server().await?;
    let template = ensure_template(&server.admin_url, &mode).await?;
    let db_name = unique_db_name();

    let mut conn = PgConnection::connect(&server.admin_url).await?;
    // Postgres handles one copy at a time, so retry rather than fail when several
    // tests start together.
    let mut attempt = 0;
    loop {
        let created = conn
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
    conn.close().await?;

    let db_url = with_database(&server.admin_url, &db_name)?;
    info!(db_name, "Cloned test database from {template}");

    Ok(ClonedDb {
        db_url: db_url.into(),
        db_name,
        admin_url: server.admin_url.clone(),
    })
}
