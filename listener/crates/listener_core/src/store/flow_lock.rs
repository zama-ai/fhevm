use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use std::sync::Arc;
use tracing::{debug, warn};

use super::client::PgClient;
use super::error::SqlResult;

/// RAII guard for a PostgreSQL session-level advisory lock.
///
/// Holds a dedicated [`PoolConnection`] for the duration of the lock.
/// The lock is released when [`release()`](Self::release) is called or
/// when the guard is dropped (connection close triggers PG auto-release).
pub struct FlowLockGuard {
    conn: Option<PoolConnection<Postgres>>,
    lock_key: i64,
}

impl FlowLockGuard {
    /// Explicitly release the advisory lock and return the connection to the pool.
    pub async fn release(mut self) -> SqlResult<()> {
        if let Some(mut conn) = self.conn.take() {
            let released: bool = sqlx::query_scalar::<_, bool>("SELECT pg_advisory_unlock($1)")
                .bind(self.lock_key)
                .fetch_one(&mut *conn)
                .await?;
            if !released {
                warn!(
                    lock_key = self.lock_key,
                    "pg_advisory_unlock returned false — lock was not held by this session"
                );
            }
            debug!(lock_key = self.lock_key, "Advisory lock released");
        }
        Ok(())
    }
}

impl Drop for FlowLockGuard {
    fn drop(&mut self) {
        if self.conn.is_some() {
            warn!(
                lock_key = self.lock_key,
                "FlowLockGuard dropped without explicit release — PG will auto-release on session close"
            );
            // Connection is dropped here, PG auto-releases the advisory lock.
        }
    }
}

/// Non-blocking distributed lock backed by `pg_try_advisory_lock`.
///
/// Provides mutual exclusion per `chain_id` across all pods sharing the
/// same PostgreSQL database. The lock key IS the `chain_id`, so different
/// chains on the same database are completely independent.
///
/// Used to prevent concurrent execution of fetch and reorg flows for the
/// same chain under HPA (Horizontal Pod Autoscaling).
#[derive(Clone)]
pub struct FlowLock {
    client: Arc<PgClient>,
    chain_id: i64,
}

impl FlowLock {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self { client, chain_id }
    }

    /// Attempt to acquire the advisory lock (non-blocking).
    ///
    /// Returns `Some(guard)` if the lock was acquired, `None` if another
    /// session already holds it. The guard holds a [`PoolConnection`] — the
    /// lock remains held until [`FlowLockGuard::release()`] is called or
    /// the guard is dropped.
    pub async fn try_acquire(&self) -> SqlResult<Option<FlowLockGuard>> {
        let mut conn = self.client.acquire().await?;

        let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
            .bind(self.chain_id)
            .fetch_one(&mut *conn)
            .await?;

        if acquired {
            debug!(chain_id = self.chain_id, "Advisory lock acquired");
            Ok(Some(FlowLockGuard {
                conn: Some(conn),
                lock_key: self.chain_id,
            }))
        } else {
            debug!(
                chain_id = self.chain_id,
                "Advisory lock held by another session"
            );
            Ok(None)
        }
    }
}
