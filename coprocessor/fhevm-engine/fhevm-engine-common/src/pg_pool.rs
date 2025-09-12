use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::{Pool, Postgres};
use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info, Instrument};

pub struct PostgresPoolManager {
    params: Params,
}

impl PostgresPoolManager {
    pub fn new(
        url: &str,
        acquire_timeout: Duration,
        max_connections: u32,
        retry_db_conn_interval: Duration,
        diagnostics_enable_auto_explain: bool,
    ) -> Self {
        Self {
            params: Params {
                url: url.to_string(),
                acquire_timeout,
                max_connections,
                retry_db_conn_interval,
                diagnostics_enable_auto_explain,
            },
        }
    }

    /// Spawn a new task that runs the given operation with a database connection,
    /// retrying on transient errors.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlx::{Pool, Postgres};
    /// use std::time::Duration;
    /// use your_crate::{PostgresPoolManager, ServiceError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ServiceError> {
    ///     // Initialize the runner with DB params
    ///     let db = PostgresPoolManager::new(
    ///         "postgres://postgres:password@localhost/dbname",
    ///         Duration::from_secs(5),   // acquire timeout
    ///         10,                        // max connections
    ///         Duration::from_secs(2),    // retry interval
    ///         false,
    ///     );
    ///
    ///     // Define an operation to run with the database pool
    ///     let op = |pool: Pool<Postgres>| async move {
    ///         let row: (i64,) = sqlx::query_as("SELECT 1")
    ///             .fetch_one(&pool)
    ///             .await?; // If fails, it will be retried
    ///         println!("Query result: {}", row.0);
    ///         Ok(())
    ///     };
    ///
    ///     // Spawn the operation in the background
    ///     let handle = db.spawn_with_db_retry(op).await;
    ///
    ///     // Wait for the task to finish (or let it run in background)
    ///     handle.await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn spawn_with_db_retry<F, Fut>(&self, op: F, span_label: &str) -> JoinHandle<()>
    where
        F: Fn(Pool<Postgres>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ServiceError>> + Send + 'static,
    {
        let fut = Self::run_with_db_retry(op, self.params.clone());
        let span = tracing::info_span!("op", target = span_label);
        tokio::spawn(
            async move {
                if let Err(err) = fut.await {
                    error!(error = %err, "operation failed");
                }
            }
            .instrument(span),
        )
    }

    /// Run the given operation with a database connection, retrying on transient errors.
    pub async fn blocking_with_db_retry<F, Fut>(&self, op: F)
    where
        F: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<(), ServiceError>>,
    {
        Self::run_with_db_retry(op, self.params.clone())
            .await
            .unwrap_or_else(|err| {
                error!(error = %err, "WithDatabase operation failed");
            });
    }

    async fn run_with_db_retry<F, Fut>(operation: F, params: Params) -> Result<(), ServiceError>
    where
        F: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<(), ServiceError>>,
    {
        let is_auto_explain_enabled = params.diagnostics_enable_auto_explain;

        loop {
            let pool = PgPoolOptions::new()
                .max_connections(params.max_connections)
                .acquire_timeout(params.acquire_timeout)
                .after_connect(move |conn, _meta| {
                    Box::pin(async move {
                        if is_auto_explain_enabled {
                            if let Err(err) = enable_auto_explain(conn).await {
                                error!(target: "worker", error=%err, "Failed to enable auto_explain");
                            } else {
                                info!(target: "worker", "Enabled auto_explain for diagnostics");
                            }
                        }
                        Result::<_, sqlx::Error>::Ok(())
                    })
                })
                .connect(&params.url)
                .await;

            let pool = match pool {
                Result::Ok(p) => p,
                Err(err) => {
                    error!(target: "worker", error=%err, "Failed to create DB pool; retrying...");
                    sleep(params.retry_db_conn_interval).await;
                    continue;
                }
            };

            if let Err(err) = operation(pool).await {
                error!(target: "worker", error=%err, "operation failed; retrying...");
                match err {
                    ServiceError::Database(sqlx::Error::PoolTimedOut) => {
                        // PoolTimedOut is considered a transient error; retry after sleeping.
                        sleep(params.retry_db_conn_interval).await;
                    }
                    ServiceError::Database(sqlx::Error::Io(_) | sqlx::Error::Tls(_)) => {
                        error!(target: "worker", "Transient DB error occurred; backing off");
                        sleep(params.retry_db_conn_interval).await;
                    }
                    ServiceError::Database(_) => {
                        error!(target: "worker", error=%err, "Non-transient DB error; longer backoff");
                        sleep(params.retry_db_conn_interval * 10).await;
                    }
                    _ => {
                        return Err(err);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Params {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub retry_db_conn_interval: Duration,

    pub diagnostics_enable_auto_explain: bool,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    /// Represents errors returned by the database layer, such as connection issues or query failures.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Represents internal errors within the service that are not related to the database.
    #[error("Internal error: {0}")]
    InternalError(String),
}

async fn enable_auto_explain(conn: &mut sqlx::PgConnection) -> Result<(), sqlx::Error> {
    // Enable auto_explain for diagnostics
    // Note: auto_explain requires superuser privileges
    // in PostgreSQL. Ensure the connecting user has the necessary rights.
    conn.execute("LOAD 'auto_explain';").await?;
    conn.execute("SET auto_explain.log_analyze = on;").await?;
    conn.execute("SET auto_explain.log_nested_statements = on;")
        .await?;
    conn.execute("SET auto_explain.log_verbose = on;").await?;
    conn.execute("SET auto_explain.log_format = 'json';")
        .await?;
    conn.execute("SET auto_explain.log_buffers = on;").await?;
    conn.execute("SET auto_explain.log_min_duration = '10ms';")
        .await?;
    Ok(())
}
