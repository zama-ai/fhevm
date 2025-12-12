use chrono::{DateTime, Utc};
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use sqlx::Postgres;
use std::{fmt, sync::LazyLock, time::SystemTime};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub(crate) static ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_tfhe_worker_dcid_counter",
            "Number of acquired dependence chain IDs in tfhe-worker"
        )
        .unwrap()
    });

pub static ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram!(
        "coprocessor_tfhe_worker_query_acquire_dcid_seconds",
        "Histogram of query-time spent acquiring dependence chain IDs in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap()
});

pub static EXTEND_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram!(
        "coprocessor_tfhe_worker_query_extend_dcid_seconds",
        "Histogram of query-time spent extending dependence_chain lock in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockingReason {
    UpdatedUnowned, // Normal lock acquisition
    ExpiredLock,    // Work-stealing
    ExtendedLock,   // Lock extension
    Missing,        // No lock acquired
}

impl From<&str> for LockingReason {
    fn from(s: &str) -> Self {
        match s {
            "updated_unowned" => LockingReason::UpdatedUnowned,
            "expired_lock" => LockingReason::ExpiredLock,
            "extended_lock" => LockingReason::ExtendedLock,
            _ => LockingReason::Missing,
        }
    }
}

/// Manages a non-blocking, distributed locking mechanism
/// that coordinates dependence-chain processing across multiple workers
#[derive(Clone)]
pub struct LockMngr {
    pool: sqlx::Pool<Postgres>,
    worker_id: Uuid,
    lock: Option<DatabaseChainLock>,
    lock_ttl_sec: i64,
}

/// Dependence chain lock data
#[derive(sqlx::FromRow, Clone)]
pub struct DatabaseChainLock {
    pub dependence_chain_id: Vec<u8>,
    pub worker_id: Option<Uuid>,
    pub lock_acquired_at: Option<DateTime<Utc>>,
    pub lock_expires_at: Option<DateTime<Utc>>,
    pub last_updated_at: DateTime<Utc>,
    pub match_reason: String,
}

#[derive(Debug, sqlx::FromRow)]
struct LockExpiresAt {
    lock_expires_at: Option<DateTime<Utc>>,
}

impl fmt::Debug for DatabaseChainLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseChainLock")
            .field("dcid", &hex::encode(&self.dependence_chain_id))
            .field("worker_id", &self.worker_id)
            .field("lock_acquired_at", &self.lock_acquired_at)
            .field("lock_expires_at", &self.lock_expires_at)
            .field("last_updated_at", &self.last_updated_at)
            .field("match_reason", &self.match_reason)
            .finish()
    }
}

impl LockMngr {
    pub fn new(worker_id: Uuid, pool: sqlx::Pool<Postgres>) -> Self {
        Self {
            worker_id,
            pool,
            lock: None,
            lock_ttl_sec: 30,
        }
    }

    pub fn new_with_ttl(worker_id: Uuid, pool: sqlx::Pool<Postgres>, lock_ttl_sec: u32) -> Self {
        let mut mgr = Self::new(worker_id, pool);
        mgr.lock_ttl_sec = lock_ttl_sec as i64;
        mgr
    }

    /// Acquire the next available dependence-chain entry for processing
    /// sorted by last_updated_at (FIFO).
    /// Returns the dependence_chain_id if a lock was acquired
    pub async fn acquire_next_lock(
        &mut self,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
        let started_at = SystemTime::now();
        let row = sqlx::query_as::<_, DatabaseChainLock>(
            r#"
            WITH candidate AS (
                SELECT dependence_chain_id,
                    CASE
                        WHEN status = 'updated' AND worker_id IS NULL
                            THEN 'updated_unowned'
                        WHEN lock_expires_at < NOW()
                            THEN 'expired_lock'
                        END AS match_reason
                FROM dependence_chain
                WHERE
                        (
                            status = 'updated'      -- Marked as updated by host-listener
                            AND
                            worker_id IS NULL       -- Ensure no other workers own it
                        )                              
                    OR  (
                            lock_expires_at < NOW()  -- Work-stealing of expired locks
                        )
                ORDER BY last_updated_at ASC        -- FIFO
                FOR UPDATE SKIP LOCKED              -- Ensure no other worker is currently trying to lock it
                LIMIT 1
            )
            UPDATE dependence_chain AS dc
            SET
                worker_id = $1,
                status = 'processing',
                lock_acquired_at = NOW(),
                lock_expires_at = NOW() + make_interval(secs => $2)
            FROM candidate
            WHERE dc.dependence_chain_id = candidate.dependence_chain_id
            RETURNING dc.*, candidate.match_reason;
        "#,
        )
        .bind(self.worker_id)
        .bind(self.lock_ttl_sec)
        .fetch_optional(&self.pool)
        .await?;

        let row = if let Some(row) = row {
            row
        } else {
            return Ok((None, LockingReason::Missing));
        };

        self.lock.replace(row.clone());

        ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER.inc();

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        info!(?row, query_elapsed = %elapsed, "Acquired lock");

        Ok((
            Some(row.dependence_chain_id),
            LockingReason::from(row.match_reason.as_str()),
        ))
    }

    /// Release all locks held by this worker
    ///
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its status
    pub async fn release_all_owned_locks(&mut self) -> Result<u64, sqlx::Error> {
        // Since UPDATE always aquire a row-level lock internally,
        // this acts as atomic_exchange
        let rows = sqlx::query!(
            r#" 
            UPDATE dependence_chain
            SET 
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                status = CASE 
                        WHEN status = 'processing' THEN 'processed'
                        ELSE status
                        END
            WHERE worker_id = $1
        "#,
            self.worker_id
        )
        .execute(&self.pool)
        .await?;

        self.lock.take();

        info!(worker_id = %self.worker_id,
            count = rows.rows_affected(), "Released all locks");

        Ok(rows.rows_affected())
    }

    /// Release the lock held by this worker on the current dependence chain
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its status
    pub async fn release_current_lock(&mut self) -> Result<u64, sqlx::Error> {
        let dep_chain_id = match &self.lock {
            Some(lock) => lock.dependence_chain_id.clone(),
            None => {
                debug!("No lock to release");
                return Ok(0);
            }
        };

        let rows = sqlx::query!(
            r#"
        UPDATE dependence_chain 
        SET 
            worker_id = NULL,
            lock_acquired_at = NULL,
            lock_expires_at = NULL,
            status = CASE 
                    WHEN status = 'processing' THEN 'processed'
                    ELSE status
                    END
        WHERE worker_id = $1 AND dependence_chain_id = $2
        "#,
            self.worker_id,
            dep_chain_id,
        )
        .execute(&self.pool)
        .await?;

        self.lock.take();

        info!(dcid = %hex::encode(&dep_chain_id), "Released lock");

        Ok(rows.rows_affected())
    }

    /// Set error on the current dependence chain
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its error
    ///
    /// The error is only informational and does not affect the processing status
    pub async fn set_processing_error(&self, err: Option<String>) -> Result<u64, sqlx::Error> {
        let dep_chain_id: Vec<u8> = match &self.lock {
            Some(lock) => lock.dependence_chain_id.clone(),
            None => {
                warn!("No lock to set error on");
                return Ok(0);
            }
        };

        let rows = sqlx::query!(
            r#"
            UPDATE dependence_chain
            SET
                error_message = CASE
                        WHEN status = 'processing' THEN $3
                        ELSE error_message
                        END
            WHERE worker_id = $1 AND dependence_chain_id = $2
            "#,
            self.worker_id,
            dep_chain_id,
            err
        )
        .execute(&self.pool)
        .await?;

        info!(dcid = %hex::encode(&dep_chain_id), error = ?err, "Set error on lock");
        Ok(rows.rows_affected())
    }

    /// Extend the lock expiration time on the current dependence chain
    pub async fn extend_current_lock(
        &mut self,
    ) -> Result<Option<(Vec<u8>, LockingReason)>, sqlx::Error> {
        let started_at = SystemTime::now();
        let dependence_chain_id = match &self.lock {
            Some(lock) => lock.dependence_chain_id.clone(),
            None => {
                debug!("No lock to extend");
                return Ok(None);
            }
        };

        let row = sqlx::query_as!(
            LockExpiresAt,
            r#"
            UPDATE dependence_chain AS dc
                SET
                lock_expires_at = NOW() + make_interval(secs => $3)
            WHERE dependence_chain_id = $1 AND worker_id = $2
            RETURNING dc.lock_expires_at::timestamptz AS "lock_expires_at: chrono::DateTime<Utc>";
        "#,
            dependence_chain_id,
            self.worker_id,
            self.lock_ttl_sec as f64
        )
        .fetch_optional(&self.pool)
        .await?;

        let lock_expires_at = match row {
            Some(r) => r,
            None => {
                self.lock.take();
                error!(dcid = %hex::encode(&dependence_chain_id), "No lock extended");
                return Ok(None);
            }
        };

        // Update the in-memory lock
        if let Some(lock) = self.lock.as_mut() {
            lock.lock_expires_at = lock_expires_at.lock_expires_at;
            info!(dcid = %hex::encode(&dependence_chain_id), expires_at = ?lock.lock_expires_at, "Extended lock");
        }

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            EXTEND_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        Ok(Some((dependence_chain_id, LockingReason::ExtendedLock)))
    }

    pub fn get_current_lock(&self) -> Option<DatabaseChainLock> {
        self.lock.clone()
    }

    pub fn worker_id(&self) -> Uuid {
        self.worker_id
    }
}
