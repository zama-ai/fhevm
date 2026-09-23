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

/// Salt XORed into the chain_id to derive the cleaner lock key.
/// Keeps cleaner mutual exclusion independent from the fetch/reorg lock
/// (raw chain_id): the cleaner holds its lock across the cron sleep and
/// must never block or be blocked by the cursor flows.
const CLEANER_LOCK_SALT: i64 = 0x434C_4541_4E45_5221; // "CLEANER!"

/// Salt XORed into the chain_id to derive the finality-flow lock key.
/// Keeps finality mutual exclusion independent from the fetch/reorg lock
/// (raw chain_id) and the cleaner lock: a stalled finality flow must never
/// block or be blocked by the live cursor.
const FINALITY_LOCK_SALT: i64 = 0x4649_4E41_4C49_5459; // "FINALITY"

/// Salt XORed into the chain_id to derive the final-cleaner lock key.
/// Keeps final-cleaner mutual exclusion independent from every other flow
/// lock: it holds its lock across the cron sleep and must never block or be
/// blocked by the cursor, finality, or cleaner flows.
const FINAL_CLEANER_LOCK_SALT: i64 = 0x464E_434C_4541_4E21; // "FNCLEAN!"

/// Non-blocking distributed lock backed by `pg_try_advisory_lock`.
///
/// Provides mutual exclusion per `chain_id` across all pods sharing the
/// same PostgreSQL database. The fetch/reorg lock key IS the `chain_id`
/// ([`new`](Self::new)); the cleaner, finality, and final-cleaner flows use
/// salted keys ([`new_cleaner`](Self::new_cleaner),
/// [`new_finality`](Self::new_finality),
/// [`new_final_cleaner`](Self::new_final_cleaner)) so the flows never contend
/// with each other.
/// Different chains on the same database are completely independent.
///
/// Used to prevent concurrent execution of fetch, reorg, cleaner, finality,
/// and final-cleaner flows for the same chain under HPA (Horizontal Pod
/// Autoscaling).
#[derive(Clone)]
pub struct FlowLock {
    client: Arc<PgClient>,
    lock_key: i64,
}

impl FlowLock {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self {
            client,
            lock_key: chain_id,
        }
    }

    /// Lock for the cleaner flow: same mutual-exclusion semantics as
    /// [`new`](Self::new) but on a salted key, so the cleaner (which holds
    /// its lock across the cron sleep) never starves the cursor flows.
    pub fn new_cleaner(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self {
            client,
            lock_key: chain_id ^ CLEANER_LOCK_SALT,
        }
    }

    /// Lock for the finality flow: same mutual-exclusion semantics as
    /// [`new`](Self::new) but on a salted key, so the finality loop never
    /// blocks — or is blocked by — the live cursor and reorg flows.
    pub fn new_finality(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self {
            client,
            lock_key: chain_id ^ FINALITY_LOCK_SALT,
        }
    }

    /// Lock for the final-cleaner flow: same mutual-exclusion semantics as
    /// [`new`](Self::new) but on a salted key, so the final cleaner (which
    /// holds its lock across the cron sleep) never starves any other flow.
    pub fn new_final_cleaner(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self {
            client,
            lock_key: chain_id ^ FINAL_CLEANER_LOCK_SALT,
        }
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
            .bind(self.lock_key)
            .fetch_one(&mut *conn)
            .await?;

        if acquired {
            debug!(lock_key = self.lock_key, "Advisory lock acquired");
            Ok(Some(FlowLockGuard {
                conn: Some(conn),
                lock_key: self.lock_key,
            }))
        } else {
            debug!(
                lock_key = self.lock_key,
                "Advisory lock held by another session"
            );
            Ok(None)
        }
    }
}
