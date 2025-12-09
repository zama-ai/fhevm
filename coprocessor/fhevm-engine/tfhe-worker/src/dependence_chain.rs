use chrono::{DateTime, Utc};
use sqlx::Postgres;
use tracing::{debug, info, warn};
use uuid::Uuid;

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
    expiration_duration_secs: i64,
}

/// Dependence chain lock data
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct DatabaseChainLock {
    pub dependence_chain_id: Vec<u8>,
    pub worker_id: Option<Uuid>,
    pub lock_acquired_at: Option<DateTime<Utc>>,
    pub lock_expires_at: Option<DateTime<Utc>>,
    pub last_updated_at: DateTime<Utc>,
    pub match_reason: String,
}

impl LockMngr {
    pub fn new(worker_id: Uuid, pool: sqlx::Pool<Postgres>) -> Self {
        Self {
            worker_id,
            pool,
            lock: None,
            expiration_duration_secs: 30,
        }
    }

    pub fn new_with_expiry(
        worker_id: Uuid,
        pool: sqlx::Pool<Postgres>,
        expiration_duration_secs: i64,
    ) -> Self {
        let mut mgr = Self::new(worker_id, pool);
        mgr.expiration_duration_secs = expiration_duration_secs;
        mgr
    }

    /// Acquire the next available dependence-chain entry for processing
    /// sorted by last_updated_at (FIFO).
    /// Returns the dependence_chain_id if a lock was acquired
    pub async fn acquire_next_lock(
        &mut self,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
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
        .bind(self.expiration_duration_secs)
        .fetch_optional(&self.pool)
        .await?;

        let row = if let Some(row) = row {
            row
        } else {
            return Ok((None, LockingReason::Missing));
        };

        self.lock.replace(row.clone());
        info!(target: "deps_chain", ?row, "Acquired lock");

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

        info!(target: "deps_chain", worker_id = %self.worker_id,
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
                debug!(target: "deps_chain", "No lock to release");
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

        info!(target: "deps_chain", ?dep_chain_id, "Released lock");

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
                warn!(target: "deps_chain", "No lock to set error on");
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

        info!(target: "deps_chain", ?dep_chain_id, error = ?err, "Set error on lock");
        Ok(rows.rows_affected())
    }

    /// Extend the lock expiration time on the current dependence chain
    pub async fn extend_current_lock(
        &self,
    ) -> Result<Option<(Vec<u8>, LockingReason)>, sqlx::Error> {
        let dependence_chain_id = match &self.lock {
            Some(lock) => lock.dependence_chain_id.clone(),
            None => {
                info!(target: "deps_chain", "No lock to extend");
                return Ok(None);
            }
        };

        sqlx::query!(
            r#"
            UPDATE dependence_chain
                SET 
                lock_expires_at = NOW() + make_interval(secs => $3)
            WHERE dependence_chain_id = $1 AND worker_id = $2
        "#,
            dependence_chain_id,
            self.worker_id,
            self.expiration_duration_secs as f64
        )
        .execute(&self.pool)
        .await?;

        info!(target: "deps_chain", ?dependence_chain_id, "Extended lock");

        Ok(Some((dependence_chain_id, LockingReason::ExtendedLock)))
    }

    pub fn get_current_lock(&self) -> Option<DatabaseChainLock> {
        self.lock.clone()
    }

    pub fn worker_id(&self) -> Uuid {
        self.worker_id
    }
}
