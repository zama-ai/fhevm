use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::{Pool, Postgres};
use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tokio::task::{AbortHandle, JoinHandle, JoinSet};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Instrument};

const CODE_DEADLOCK_DETECTED: &str = "40P01";

#[derive(Clone)]
pub struct PostgresPoolManager {
    pool: Pool<Postgres>,
    cancel_token: CancellationToken,
    params: Params,
}

impl PostgresPoolManager {
    /// Create a new PostgresPoolManager with the given configuration.
    /// This function will attempt to connect to the database, retrying on failure indefinitely.
    /// If `auto_explain_with_min_duration` is set, it will enable the auto_explain extension
    /// on each new connection for diagnostics.
    pub async fn connect_pool(
        cancel_token: CancellationToken,
        url: &str,
        acquire_timeout: Duration,
        max_connections: u32,
        retry_db_conn_interval: Duration,
        auto_explain_with_min_duration: Option<Duration>,
    ) -> Self {
        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(max_connections)
                .acquire_timeout(acquire_timeout)
                .after_connect(move |conn, _meta| {
                    info!(auto_explain = ?auto_explain_with_min_duration, "New DB connection established");
                    Box::pin(async move {
                        if let Some(min_duration) = auto_explain_with_min_duration {
                            if let Err(err) = enable_auto_explain(conn, min_duration).await {
                                error!(error=%err, "Failed to enable auto_explain");
                            } else {
                                info!(min_duration = ?min_duration, "Enabled auto_explain for diagnostics");
                            }
                        }
                        Result::<_, sqlx::Error>::Ok(())
                    })
                })
                .connect(url)
                .await {
                    Ok(p) => break p,
                    Err(err) => {
                        error!( error=%err, "Failed to create initial DB pool; retrying...");
                        sleep(retry_db_conn_interval).await;
                        continue;
                    }
                }
        };

        Self {
            params: Params {
                url: url.to_string(),
                acquire_timeout,
                max_connections,
                retry_db_conn_interval,
                auto_explain_with_min_duration,
            },
            pool,
            cancel_token,
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
    /// use fhevm_engine_common::pg_pool::{PostgresPoolManager, ServiceError};
    /// use tokio_util::sync::CancellationToken;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ServiceError> {
    ///     // Initialize the runner with DB params
    ///     let db = PostgresPoolManager::connect_pool(
    ///         CancellationToken::new(),
    ///         "postgres://postgres:password@localhost/dbname",
    ///         Duration::from_secs(5),   // acquire timeout
    ///         10,                        // max connections
    ///         Duration::from_secs(2),    // retry interval
    ///         None,
    ///     ).await;
    ///
    ///     // Define an operation to run with the database pool
    ///     let op = |pool: Pool<Postgres>, cancel_token: CancellationToken| async move {
    ///         let row: (i64,) = sqlx::query_as("SELECT 1")
    ///             .fetch_one(&pool)
    ///             .await?; // If fails, it will be retried
    ///         println!("Query result: {}", row.0);
    ///         Ok(())
    ///     };
    ///
    ///     // Spawn the operation in the background
    ///     let handle = db.spawn_with_db_retry(op, "my_task").await;
    ///
    ///     // Wait for the task to finish (or let it run in background)
    ///     handle.await.unwrap();
    ///     Ok(())
    /// }
    /// ```
    pub async fn spawn_with_db_retry<F, Fut>(&self, op: F, name: &str) -> JoinHandle<()>
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ServiceError>> + Send + 'static,
    {
        let pool_mngr = self.clone();
        let fut = pool_mngr.run_with_db_retry(op);

        tokio::spawn(
            async move {
                let _ = fut.await;
            }
            .instrument(Self::span(name)),
        )
    }

    /// Calls run_with_db_retry on the specific JoinSet
    pub async fn spawn_join_set_with_db_retry<F, Fut>(
        &self,
        op: F,
        join_set: &mut JoinSet<()>,
        name: &str,
    ) -> AbortHandle
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ServiceError>> + Send + 'static,
    {
        let pool_mngr = self.clone();
        let fut = pool_mngr.run_with_db_retry(op);

        join_set.spawn(
            async move {
                let _ = fut.await;
            }
            .instrument(Self::span(name)),
        )
    }

    /// Run the given closure with a database pool, retrying on transient errors.
    pub async fn blocking_with_db_retry<F, Fut>(
        &self,
        op: F,
        name: &str,
    ) -> Result<(), ServiceError>
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut,
        Fut: Future<Output = Result<(), ServiceError>>,
    {
        let pool_mngr = self.clone();
        pool_mngr
            .run_with_db_retry(op)
            .instrument(Self::span(name))
            .await
    }

    async fn run_with_db_retry<F, Fut>(self, operation: F) -> Result<(), ServiceError>
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut,
        Fut: Future<Output = Result<(), ServiceError>>,
    {
        let ct = self.cancel_token.child_token();
        let retry_delay = self.params.retry_db_conn_interval;
        let mut backoff_delay = self.params.retry_db_conn_interval;

        loop {
            if ct.is_cancelled() {
                info!("Cancellation requested, stopping DB loop");
                return Ok(());
            }

            backoff_delay = std::cmp::min(backoff_delay * 2, Duration::from_secs(60));

            if let Err(err) = operation(self.pool.clone(), ct.clone()).await {
                error!(error=%err, "service failure; retrying...");
                match err {
                    ServiceError::Database(sqlx::Error::PoolTimedOut) => {
                        // PoolTimedOut is considered a transient error; retry after sleeping.
                        cancellable_sleep(&ct, retry_delay).await;
                    }
                    ServiceError::Database(
                        sqlx::Error::Io(_) | sqlx::Error::Protocol(_) | sqlx::Error::Tls(_),
                    ) => {
                        // IO, Protocol, and TLS errors are usually transient (e.g., network issues)
                        cancellable_sleep(&ct, backoff_delay).await;
                    }
                    ServiceError::Database(sqlx::Error::Database(ref db_err)) => {
                        // Only retry on transient database errors (deadlock, etc.)
                        let code = db_err.code().unwrap_or("".into());
                        if code == CODE_DEADLOCK_DETECTED {
                            error!(error=%err, code=%code, "Transient DB error; retrying...");
                        } else {
                            error!(error=%db_err, code=%code, "Non-transient DB error; not retrying");
                            return Err(err);
                        }
                    }
                    ServiceError::Database(other) => {
                        error!(error=%other, "Non-transient DB error; longer backoff");
                        cancellable_sleep(&ct, backoff_delay).await;
                    }
                    _ => {
                        // Non-database errors are returned immediately.
                        error!(error = %err, "unrecoverable error, a restart required");
                        return Err(err);
                    }
                }
            }
        }
    }

    pub fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }

    fn span(name: &str) -> tracing::Span {
        tracing::trace_span!("task", target = name)
    }
}

#[derive(Clone)]
pub struct Params {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub retry_db_conn_interval: Duration,

    pub auto_explain_with_min_duration: Option<Duration>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    /// Represents errors returned by the database layer, such as connection issues or query failures.
    #[error("DB: {0}")]
    Database(#[from] sqlx::Error),

    /// Represents internal errors within the service that are not related to the database.
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Enable the auto_explain extension on the given connection with the specified minimum duration.
/// Note: auto_explain requires superuser privileges
async fn enable_auto_explain(
    conn: &mut sqlx::PgConnection,
    min_duration: Duration,
) -> Result<(), sqlx::Error> {
    // The auto_explain module provides a means for logging execution plans of slow statements automatically,
    // without having to run EXPLAIN by hand.
    // This is especially helpful for tracking down un-optimized queries in large applications
    conn.execute("LOAD 'auto_explain';").await?;
    conn.execute("SET auto_explain.log_analyze = on;").await?;
    conn.execute("SET auto_explain.log_nested_statements = on;")
        .await?;
    conn.execute("SET auto_explain.log_buffers = on;").await?;

    // all statements that run min_duration or longer will be logged
    conn.execute(
        format!(
            "SET auto_explain.log_min_duration = {};",
            min_duration.as_millis()
        )
        .as_str(),
    )
    .await?;

    conn.execute("SET auto_explain.log_verbose = on;").await?;
    conn.execute("SET auto_explain.log_format = 'json';")
        .await?;

    Ok(())
}

async fn cancellable_sleep(cancel_token: &CancellationToken, duration: Duration) {
    tokio::select! {
        _ = cancel_token.cancelled() => {
            info!("Sleep cancelled");
        }
        _ = sleep(duration) => {
            // Sleep completed
        }
    }
}
