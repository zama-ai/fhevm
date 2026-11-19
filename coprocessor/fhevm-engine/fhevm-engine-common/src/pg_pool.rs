use crate::database::{
    apply_gcs_mode_search_path, connect_pool_with_options_and_connect_options, PoolRefreshHandle,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use sqlx::{Pool, Postgres};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::task::{AbortHandle, JoinHandle, JoinSet};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn, Instrument};

/// PostgreSQL's `deadlock_detected` SQLSTATE.
/// See <https://www.postgresql.org/docs/current/errcodes-appendix.html>.
const PG_DEADLOCK_DETECTED: &str = "40P01";

/// True if the DB connection is lost/unusable, so we should exit and let k8s
/// restart us rather than retry on a dead pool.
pub fn is_fatal_connection_error(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::Protocol(_)
        | sqlx::Error::PoolClosed
        | sqlx::Error::WorkerCrashed => true,
        sqlx::Error::Database(db) => db.code().as_deref().is_some_and(fatal_connection_sqlstate),
        _ => false,
    }
}

/// Postgres SQLSTATEs for a dead connection/session (e.g. 28000, the auth
/// rejection from the RDS failover). Deadlock (40P01) is excluded, it's retryable.
fn fatal_connection_sqlstate(code: &str) -> bool {
    matches!(
        code,
        "28000" | "28P01" | "08006" | "08001" | "08004" | "57P01" | "53300"
    )
}

#[derive(Clone)]
pub struct PostgresPoolManager {
    pool: Pool<Postgres>,
    _pool_refresh_handle: Arc<PoolRefreshHandle>,
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
    ) -> Option<Self> {
        Self::connect_pool_with_gcs_mode(
            cancel_token,
            url,
            acquire_timeout,
            max_connections,
            retry_db_conn_interval,
            auto_explain_with_min_duration,
            false,
        )
        .await
    }

    /// Same as [`Self::connect_pool`] but, when `gcs_mode = true`, pins
    /// every connection in the pool to `search_path = gcs,public` so
    /// unqualified writes resolve to the `gcs` schema (with fallback to
    /// `public` for shared read-only tables).
    #[allow(clippy::too_many_arguments)]
    pub async fn connect_pool_with_gcs_mode(
        cancel_token: CancellationToken,
        url: &str,
        acquire_timeout: Duration,
        max_connections: u32,
        retry_db_conn_interval: Duration,
        auto_explain_with_min_duration: Option<Duration>,
        gcs_mode: bool,
    ) -> Option<Self> {
        let database_url = crate::utils::DatabaseURL::from(url.to_owned());
        let pool = loop {
            if cancel_token.is_cancelled() {
                return None;
            }

            let statement_timeout = std::cmp::max(acquire_timeout, Duration::from_secs(10));

            match connect_pool_with_options_and_connect_options(
                &database_url,
                PgPoolOptions::new()
                .max_connections(max_connections)
                .acquire_timeout(acquire_timeout)
                .after_connect(move |conn, _meta| {
                    info!(auto_explain = ?auto_explain_with_min_duration, "New DB connection established");
                    Box::pin(async move {
                        // Bound every statement on this pooled connection so a
                        // pathological query cannot run (or hold row locks)
                        // unbounded. Use the configured DB timeout, while
                        // preserving the historical 10s floor.
                        let statement_timeout_ms = statement_timeout.as_millis();
                        let set_statement_timeout =
                            format!("SET statement_timeout = '{}ms'", statement_timeout_ms);
                        if let Err(err) = (&mut *conn)
                            .execute(set_statement_timeout.as_str())
                            .await
                        {
                            error!(error=%err, "Failed to set statement_timeout on new DB connection");
                        }
                        if let Some(min_duration) = auto_explain_with_min_duration {
                            if let Err(err) = enable_auto_explain(conn, min_duration).await {
                                error!(error=%err, "Failed to enable auto_explain");
                            } else {
                                info!(min_duration = ?min_duration, "Enabled auto_explain for diagnostics");
                            }
                        }
                        Result::<_, sqlx::Error>::Ok(())
                    })
                }),
                Some(&cancel_token),
                apply_gcs_mode_search_path(gcs_mode),
            )
            .await
            {
                Ok((p, refresh_handle)) => {
                    break (p, Arc::new(refresh_handle));
                }
                Err(err) => {
                    error!( error=%err, "Failed to create initial DB pool; retrying...");
                    sleep(retry_db_conn_interval).await;
                    continue;
                }
                }
        };

        Some(Self {
            params: Params {
                url: database_url.as_str().to_owned(),
                acquire_timeout,
                max_connections,
                retry_db_conn_interval,
                auto_explain_with_min_duration,
            },
            pool: pool.0,
            _pool_refresh_handle: pool.1,
            cancel_token,
        })
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
    ///     ).await.unwrap();
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
    ///     handle.await.unwrap()?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn spawn_with_db_retry<F, Fut>(
        &self,
        op: F,
        name: &str,
    ) -> JoinHandle<Result<(), ServiceError>>
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ServiceError>> + Send + 'static,
    {
        let pool_mngr = self.clone();
        let fut = pool_mngr.run_with_db_retry(op);

        tokio::spawn(fut.instrument(Self::span(name)))
    }

    /// Calls run_with_db_retry on the specific JoinSet
    pub async fn spawn_join_set_with_db_retry<F, Fut>(
        &self,
        op: F,
        join_set: &mut JoinSet<Result<(), ServiceError>>,
        name: &str,
    ) -> AbortHandle
    where
        F: Fn(Pool<Postgres>, CancellationToken) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ServiceError>> + Send + 'static,
    {
        let pool_mngr = self.clone();
        let fut = pool_mngr.run_with_db_retry(op);

        join_set.spawn(fut.instrument(Self::span(name)))
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
        let mut backoff_delay = self.params.retry_db_conn_interval;

        loop {
            if ct.is_cancelled() {
                info!("Cancellation requested, stopping DB loop");
                return Ok(());
            }

            match operation(self.pool.clone(), ct.clone()).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    // Only a lost connection is fatal (a restart gives a fresh
                    // pool); retry everything else.
                    if let ServiceError::Database(db_err) = &err {
                        if is_fatal_connection_error(db_err) {
                            error!(error=%err, "Fatal DB connection error; not retrying");
                            return Err(err);
                        }
                        if db_err
                            .as_database_error()
                            .and_then(|err| err.code())
                            .as_deref()
                            == Some(PG_DEADLOCK_DETECTED)
                        {
                            // PostgreSQL aborts one victim and releases its transaction
                            // locks, breaking the cycle. The other participant does not
                            // get this deadlock error and can continue. Retry only the victim,
                            // without sleeping or advancing the error backoff.
                            // Further contention can still cause another deadlock.
                            warn!(error=%err, "Database deadlock; retrying immediately");
                            continue;
                        }
                    }
                    backoff_delay = std::cmp::min(backoff_delay * 2, Duration::from_secs(60));
                    error!(error=%err, "Service error; retrying...");
                    cancellable_sleep(&ct, backoff_delay).await;
                }
            }
        }
    }

    pub fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }

    fn span(name: &str) -> tracing::Span {
        tracing::error_span!("task", target = name)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{borrow::Cow, cell::RefCell, collections::VecDeque};
    use tokio::time::Instant;

    #[derive(Debug, Error)]
    #[error("test database error")]
    struct TestDatabaseError(&'static str);

    impl sqlx::error::DatabaseError for TestDatabaseError {
        fn message(&self) -> &str {
            "test database error"
        }

        fn code(&self) -> Option<Cow<'_, str>> {
            Some(Cow::Borrowed(self.0))
        }

        fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
            self
        }

        fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
            self
        }

        fn kind(&self) -> sqlx::error::ErrorKind {
            sqlx::error::ErrorKind::Other
        }
    }

    fn database_error(code: &'static str) -> ServiceError {
        sqlx::Error::Database(Box::new(TestDatabaseError(code))).into()
    }

    fn test_manager() -> PostgresPoolManager {
        let url = "postgresql://postgres@localhost/unused";
        PostgresPoolManager {
            pool: PgPoolOptions::new().connect_lazy(url).unwrap(),
            _pool_refresh_handle: Arc::new(PoolRefreshHandle::default()),
            cancel_token: CancellationToken::new(),
            params: Params {
                url: url.into(),
                max_connections: 1,
                acquire_timeout: Duration::from_secs(5),
                retry_db_conn_interval: Duration::from_secs(2),
                auto_explain_with_min_duration: None,
            },
        }
    }

    async fn retry_errors(errors: Vec<ServiceError>) -> (Vec<Duration>, Result<(), ServiceError>) {
        let manager = test_manager();
        let errors = RefCell::new(VecDeque::from(errors));
        let attempts = RefCell::new(Vec::new());
        let start = Instant::now();
        let result = manager
            .run_with_db_retry(|_, _| {
                attempts.borrow_mut().push(start.elapsed());
                std::future::ready(errors.borrow_mut().pop_front().map_or(Ok(()), Err))
            })
            .await;
        (attempts.into_inner(), result)
    }

    #[tokio::test(start_paused = true)]
    async fn deadlocks_do_not_sleep_or_increase_backoff() {
        // Enough deadlocks to reach the 60s cap if they advanced the backoff.
        let mut errors: Vec<_> = (0..8).map(|_| database_error("40P01")).collect();
        errors.push(ServiceError::InternalError("ordinary failure".into()));
        let (attempts, result) = retry_errors(errors).await;
        result.unwrap();
        let mut expected = vec![Duration::ZERO; 9];
        expected.push(Duration::from_secs(4));
        assert_eq!(attempts, expected);
    }

    #[tokio::test(start_paused = true)]
    async fn deadlocks_preserve_existing_backoff() {
        let (attempts, result) = retry_errors(vec![
            ServiceError::InternalError("first failure".into()),
            ServiceError::InternalError("second failure".into()),
            database_error("40P01"),
            database_error("40P01"),
            ServiceError::InternalError("third failure".into()),
        ])
        .await;
        result.unwrap();
        assert_eq!(attempts, [0, 4, 12, 12, 12, 28].map(Duration::from_secs));
    }

    #[tokio::test(start_paused = true)]
    async fn other_errors_still_use_backoff() {
        let (attempts, result) = retry_errors(vec![
            ServiceError::InternalError("deadlock detected".into()),
            database_error("40001"),
            database_error("23505"),
        ])
        .await;
        result.unwrap();
        assert_eq!(attempts, [0, 4, 12, 28].map(Duration::from_secs));
    }

    #[tokio::test(start_paused = true)]
    async fn fatal_errors_are_not_retried() {
        for error in [sqlx::Error::PoolClosed.into(), database_error("57P01")] {
            let (attempts, result) = retry_errors(vec![error]).await;
            assert!(result.is_err());
            assert_eq!(attempts, [Duration::ZERO]);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn immediate_deadlock_retry_honors_cancellation() {
        let manager = test_manager();
        let attempts = std::cell::Cell::new(0);
        let start = Instant::now();
        manager
            .run_with_db_retry(|_, cancel_token| {
                attempts.set(attempts.get() + 1);
                cancel_token.cancel();
                std::future::ready(Err(database_error("40P01")))
            })
            .await
            .unwrap();
        assert_eq!(attempts.get(), 1);
        assert_eq!(start.elapsed(), Duration::ZERO);
    }

    #[test]
    fn sqlstate_classification() {
        for code in [
            "28000", "28P01", "08006", "08001", "08004", "57P01", "53300",
        ] {
            assert!(fatal_connection_sqlstate(code), "{code} should be fatal");
        }
        // Deadlock is retryable; ordinary errors and unknown codes are not fatal.
        for code in ["40P01", "23505", "23503", ""] {
            assert!(
                !fatal_connection_sqlstate(code),
                "{code} should not be fatal"
            );
        }
    }

    #[test]
    fn error_variant_classification() {
        assert!(is_fatal_connection_error(&sqlx::Error::PoolClosed));
        assert!(is_fatal_connection_error(&sqlx::Error::WorkerCrashed));
        assert!(is_fatal_connection_error(&sqlx::Error::Io(
            std::io::Error::other("boom")
        )));
        assert!(is_fatal_connection_error(&sqlx::Error::Protocol(
            "bad".into()
        )));
        assert!(!is_fatal_connection_error(&sqlx::Error::PoolTimedOut));
        assert!(!is_fatal_connection_error(&sqlx::Error::RowNotFound));
    }
}
