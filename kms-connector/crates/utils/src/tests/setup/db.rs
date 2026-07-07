use crate::types::{DEFAULT_EPOCH_ID, TESTING_KMS_CONTEXT};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, types::chrono::Utc};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use tracing::info;

const POSTGRES_PORT: u16 = 5432;

/// Admin connection string used when no `DATABASE_URL` is set in the environment.
const DEFAULT_ADMIN_DB_URL: &str = "postgresql://postgres:postgres@localhost:5432/postgres";

/// Per-process counter used to build globally-unique per-test database names.
static DB_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct DbInstance {
    /// Set only when the database is backed by a dedicated container (e.g. for tests that need
    /// to `pause()`/`unpause()` the server). `None` when running against an external server.
    pub db_container: Option<ContainerAsync<GenericImage>>,
    pub db: Pool<Postgres>,
    pub url: String,
    /// `(admin_url, db_name)` of an externally-hosted per-test database, used for best-effort
    /// cleanup on drop. `None` for the containerized variant (the container is reaped instead).
    external_cleanup: Option<(String, String)>,
}

impl DbInstance {
    /// Test DB setup backed by a dedicated Postgres container.
    ///
    /// Use this when the test needs control over the container lifecycle (e.g. pausing the
    /// server to exercise health checks). Otherwise prefer [`DbInstance::setup_external`].
    pub async fn setup_container() -> anyhow::Result<Self> {
        info!("Starting Postgres container...");
        let container = GenericImage::new("postgres", "17.5")
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .start()
            .await?;
        info!("Postgres container ready!");

        let cont_host = container.get_host().await?;
        let host_port = container.get_host_port_ipv4(POSTGRES_PORT).await?;
        let admin_db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{host_port}/postgres");
        let db_url =
            format!("postgresql://postgres:postgres@{cont_host}:{host_port}/kms-connector");

        info!("Creating KMS Connector db...");
        let admin_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&admin_db_url)
            .await?;
        sqlx::query("CREATE DATABASE \"kms-connector\";")
            .execute(&admin_pool)
            .await?;
        info!("KMS Connector DB url: {db_url}");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await?;

        prepare_db(&pool).await?;

        Ok(DbInstance {
            db_container: Some(container),
            db: pool,
            url: db_url,
            external_cleanup: None,
        })
    }

    /// Test DB setup backed by an already-running, shared Postgres server.
    ///
    /// Each call creates a fresh, uniquely-named database on the server pointed at by the
    /// `DATABASE_URL` env var (defaulting to [`DEFAULT_ADMIN_DB_URL`]), so tests stay isolated
    /// without paying the cost of starting a container each time. The database is dropped on a
    /// best-effort basis when the returned instance is dropped.
    pub async fn setup_external() -> anyhow::Result<Self> {
        let admin_db_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_ADMIN_DB_URL.to_string());

        // Use the pid + the counter for unique db_name across parallel test processes
        let db_name = format!(
            "kms_connector_test_{}_{}",
            std::process::id(),
            DB_COUNTER.fetch_add(1, Ordering::Relaxed),
        );

        info!("Creating per-test database '{db_name}' on shared Postgres server...");
        let admin_pool = connect_admin_pool(&admin_db_url).await?;

        sqlx::query(&format!("CREATE DATABASE \"{db_name}\";"))
            .execute(&admin_pool)
            .await?;
        admin_pool.close().await;

        let db_url = replace_db_in_url(&admin_db_url, &db_name);
        info!("KMS Connector per-test DB url: {db_url}");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect(&db_url)
            .await?;

        prepare_db(&pool).await?;

        Ok(DbInstance {
            db_container: None,
            db: pool,
            url: db_url,
            external_cleanup: Some((admin_db_url, db_name)),
        })
    }
}

impl Drop for DbInstance {
    fn drop(&mut self) {
        // Only externally-hosted per-test databases need explicit cleanup; the containerized
        // variant is torn down when its `ContainerAsync` field is dropped.
        let Some((admin_db_url, db_name)) = self.external_cleanup.take() else {
            return;
        };

        // We may be dropped from within a current-thread Tokio runtime (the default for
        // `#[tokio::test]`), which forbids `block_on`. Run the cleanup on a dedicated thread with
        // its own runtime instead, and join so the database is gone before the test process exits.
        let _ = std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };
            rt.block_on(async move {
                if let Ok(pool) = PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&admin_db_url)
                    .await
                {
                    // `WITH (FORCE)` (Postgres 13+) terminates any lingering connections from the
                    // test pool so the drop can't be blocked.
                    let query = format!("DROP DATABASE IF EXISTS \"{db_name}\" WITH (FORCE);");
                    if let Err(e) = sqlx::query(&query).execute(&pool).await {
                        info!("Failed to drop per-test database '{db_name}': {e}");
                    }
                }
            });
        })
        .join();
    }
}

async fn connect_admin_pool(admin_db_url: &str) -> anyhow::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(admin_db_url)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "Could not connect to the Postgres server at '{admin_db_url}'. \n\
                 The DB-backed tests run against an already-running, shared Postgres server.\n\
                 Start one from the `kms-connector/connector-db` folder with:\n\
                 \n    docker compose up -d kms-connector-db\n\n\
                 (or set the DATABASE_URL env var to point at your own server).\n\
                 Underlying error: {e}"
            )
        })
}

/// Runs migrations and inserts the testing KMS context shared by all integration tests.
async fn prepare_db(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    info!("Running migrations...");
    sqlx::migrate!("../../connector-db/migrations")
        .run(pool)
        .await?;
    info!("KMS Connector DB ready!");

    info!("Inserting context #{TESTING_KMS_CONTEXT}, epoch #{DEFAULT_EPOCH_ID}) for tests...");
    let now = Utc::now();
    sqlx::query!(
        "INSERT INTO kms_context(id, epoch_id, is_valid, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        TESTING_KMS_CONTEXT.as_le_slice(),
        DEFAULT_EPOCH_ID.as_le_slice(),
        true,
        now,
        now,
    )
    .execute(pool)
    .await?;
    info!("Context #{TESTING_KMS_CONTEXT} is ready for tests!");
    Ok(())
}

/// Swaps the database segment of a Postgres connection URL for `db_name`.
///
/// Assumes the URL has no query string (true for the admin URLs used here).
fn replace_db_in_url(base_url: &str, db_name: &str) -> String {
    match base_url.rsplit_once('/') {
        Some((prefix, _db)) => format!("{prefix}/{db_name}"),
        None => format!("{base_url}/{db_name}"),
    }
}
