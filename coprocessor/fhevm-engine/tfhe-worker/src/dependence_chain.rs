use chrono::{DateTime, Utc};
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use sqlx::Postgres;
use std::{fmt, sync::LazyLock, time::SystemTime};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

static ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_tfhe_worker_dcid_counter",
        "Number of acquired dependence chain IDs in tfhe-worker"
    )
    .unwrap()
});

static ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram!(
        "coprocessor_tfhe_worker_query_acquire_dcid_seconds",
        "Histogram of query-time spent acquiring dependence chain IDs in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap()
});

static EXTEND_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram!(
        "coprocessor_tfhe_worker_query_extend_dcid_seconds",
        "Histogram of query-time spent extending dependence_chain lock in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap()
});

const CLEANUP_INTERVAL_SECS: u32 = 300;
const CLEANUP_BATCH_SIZE: i64 = 1000;
const CLEANUP_AGE_THRESHOLD_SECONDS: u32 = 48 * 60 * 60; // 48 hours
const DEPENDENCY_COUNT_FALLBACK_INTERVAL: u32 = 100;

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
    lock: Option<(DatabaseChainLock, SystemTime)>,
    pending_error: Option<String>,
    normal_acquisitions_since_fallback: u32,
    dependency_count_fallback_interval: u32,

    // Configurations
    lock_ttl_sec: i64,
    lock_timeslice_sec: Option<i64>,
    disable_locking: bool,
    cleanup_interval_sec: Option<u32>,
    processed_dcid_ttl_sec: Option<u32>,

    last_cleanup_at: Option<SystemTime>,
}

/// Dependence chain lock data
#[derive(sqlx::FromRow, Clone)]
pub struct DatabaseChainLock {
    pub dependence_chain_id: Vec<u8>,
    pub worker_id: Option<Uuid>,
    pub lock_acquired_at: Option<DateTime<Utc>>,
    pub lock_expires_at: Option<DateTime<Utc>>,
    pub last_updated_at: DateTime<Utc>,
    pub block_height: Option<i64>,
    pub block_timestamp: Option<DateTime<Utc>>,
    pub schedule_priority: i16,
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
            .field("block_height", &self.block_height)
            .field("block_ts", &self.block_timestamp)
            .field("schedule_priority", &self.schedule_priority)
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
            pending_error: None,
            normal_acquisitions_since_fallback: 0,
            dependency_count_fallback_interval: DEPENDENCY_COUNT_FALLBACK_INTERVAL,
            lock_ttl_sec: 30,
            lock_timeslice_sec: None,
            disable_locking: false,
            last_cleanup_at: None,
            cleanup_interval_sec: None,
            processed_dcid_ttl_sec: None,
        }
    }

    pub fn new_with_conf(
        worker_id: Uuid,
        pool: sqlx::Pool<Postgres>,
        lock_ttl_sec: u32,
        disable_locking: bool,
        lock_timeslice_sec: Option<u32>,
        cleanup_interval_sec: Option<u32>,
        processed_dcid_ttl_sec: Option<u32>,
    ) -> Self {
        let mut mgr = Self::new(worker_id, pool);
        mgr.lock_ttl_sec = lock_ttl_sec as i64;
        mgr.disable_locking = disable_locking;
        mgr.lock_timeslice_sec = lock_timeslice_sec.map(|v| v as i64);
        mgr.cleanup_interval_sec = cleanup_interval_sec;
        mgr.processed_dcid_ttl_sec = processed_dcid_ttl_sec;
        mgr
    }

    pub fn set_dependency_count_fallback_interval(&mut self, acquisitions: u32) {
        self.dependency_count_fallback_interval = acquisitions.max(1);
    }

    /// Acquire the next available dependence-chain entry for processing
    /// sorted by lane and last_updated_at (FIFO). If no chain with satisfied
    /// dependencies is available, try the oldest chain without trusting its
    /// dependency count: reorgs and interrupted releases can leave stale counts.
    /// Returns the dependence_chain_id if a lock was acquired
    pub async fn acquire_next_lock(
        &mut self,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok((None, LockingReason::Missing));
        }

        // Rescue stale counters even while ordinary chains continue making
        // progress. Successful work must not postpone this fallback forever.
        if self.normal_acquisitions_since_fallback >= self.dependency_count_fallback_interval {
            return self.acquire_early_lock().await;
        }

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
                            AND
                            dependency_count = 0    -- No pending dependencies
                        )                              
                    OR  (
                            lock_expires_at < NOW()  -- Work-stealing of expired locks
                            AND
                            dependency_count = 0     -- No pending dependencies
                        )
                ORDER BY schedule_priority ASC, last_updated_at ASC -- highest priority first
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
            return self.acquire_early_lock().await;
        };

        self.lock.replace((row.clone(), SystemTime::now()));
        self.normal_acquisitions_since_fallback =
            self.normal_acquisitions_since_fallback.saturating_add(1);
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

    /// Acquire the earliest dependence-chain entry for processing
    /// sorted by last_updated_at (FIFO), ignoring lane priority. Here we ignore
    /// dependency_count as reorgs can lead to incorrect counts and
    /// set of dependents until we add block hashes to transaction
    /// hashes to uniquely identify transactions.
    /// Returns the dependence_chain_id if a lock was acquired
    pub async fn acquire_early_lock(
        &mut self,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok((None, LockingReason::Missing));
        }

        let started_at = SystemTime::now();
        let row = sqlx::query_as::<_, DatabaseChainLock>(
            r#"
            WITH candidate AS (
                SELECT dependence_chain_id,
                    CASE WHEN worker_id IS NULL THEN 'updated_unowned'
                         ELSE 'expired_lock' END AS match_reason,
                    dependency_count
                FROM dependence_chain
                WHERE
                    (status = 'updated' AND worker_id IS NULL)
                    OR lock_expires_at < NOW()
                ORDER BY last_updated_at ASC, schedule_priority ASC
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
            RETURNING dc.*, candidate.match_reason, candidate.dependency_count;
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

        self.lock.replace((row.clone(), SystemTime::now()));
        self.normal_acquisitions_since_fallback = 0;
        ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER.inc();

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        info!(?row, query_elapsed = %elapsed, "Acquired lock on earliest DCID");

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
        let rows = sqlx::query!(
            r#" 
            UPDATE dependence_chain
            SET 
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                status = CASE 
                        WHEN status = 'processing' THEN 'updated'     -- revert to updated so it can be re-acquired
                        ELSE status
                        END
            WHERE worker_id = $1
        "#,
            self.worker_id
        )
        .execute(&self.pool)
        .await?;

        self.take_lock();
        info!(worker_id = %self.worker_id,
            count = rows.rows_affected(), "Released all locks");

        Ok(rows.rows_affected())
    }

    /// Release the lock held by this worker on the current dependence chain
    /// Preserve a host-listener wakeup, and notify dependents only when this
    /// worker actually transitions an owned processing chain to processed.
    /// The transition and dependency-count updates commit together so cleanup
    /// or a connection failure cannot lose a dependent notification.
    pub async fn release_current_lock(
        &mut self,
        mark_as_processed: bool,
        defer: bool,
    ) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled, skipping release_current_lock");
            return Ok(0);
        }

        let dep_chain_id = match &self.lock {
            Some((lock, _)) => lock.dependence_chain_id.clone(),
            None => {
                debug!("No lock to release");
                return Ok(0);
            }
        };

        let mut tx = self.pool.begin().await?;
        let owned = sqlx::query_as::<_, (String, Vec<Vec<u8>>)>(
            r#"
            SELECT status, dependents
            FROM dependence_chain
            WHERE worker_id = $1 AND dependence_chain_id = $2
            FOR UPDATE
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_id)
        .fetch_optional(&mut *tx)
        .await?;
        let Some((status, dependents)) = owned else {
            tx.commit().await?;
            self.take_lock();
            return Ok(0);
        };
        let completed = mark_as_processed && status == "processing";
        let rows = sqlx::query(
            r#"
            UPDATE dependence_chain
            SET
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                last_updated_at = CASE WHEN $4 THEN clock_timestamp() ELSE last_updated_at END,
                status = CASE
                    WHEN status = 'processing' AND $3::bool THEN 'processed'       -- mark as processed
                    WHEN status = 'processing' AND NOT $3::bool THEN 'updated'     -- revert to updated so it can be re-acquired
                    ELSE status
                END
            WHERE worker_id = $1
            AND dependence_chain_id = $2
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_id)
        .bind(mark_as_processed)
        .bind(defer)
        .execute(&mut *tx)
        .await?;

        let mut dependents_updated = 0;
        if completed {
            // Get all dependents of a given dependence chain ID and decrement their dependency count
            // If any dependent's dependency count reaches zero, notify work_available
            dependents_updated = sqlx::query(
                r#"
                WITH updated AS (
                    UPDATE dependence_chain
                    SET
                        dependency_count = GREATEST(dependency_count - 1, 0)
                    WHERE dependence_chain_id = ANY ($1::bytea[])
                        RETURNING dependence_chain_id, dependency_count
                ),
                ready_dcid_available AS (
                    SELECT 1
                    FROM updated
                    WHERE dependency_count = 0
                    LIMIT 1
                )
                SELECT
                    pg_notify('work_available', '')
                FROM   ready_dcid_available;
                "#,
            )
            .bind(&dependents)
            .execute(&mut *tx)
            .await?
            .rows_affected();
        }

        tx.commit().await?;
        self.take_lock();
        info!(dcid = %hex::encode(&dep_chain_id), rows = rows.rows_affected(), mark_as_processed, dependents_updated,  "Released lock");

        Ok(rows.rows_affected())
    }

    /// Set error on the current dependence chain
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its error
    ///
    /// The error is only informational and does not affect the processing status
    pub async fn set_processing_error(
        &self,
        tx: &mut sqlx::Transaction<'_, Postgres>,
        err: Option<String>,
    ) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok(0);
        }

        let dep_chain_id: Vec<u8> = match &self.lock {
            Some((lock, _)) => lock.dependence_chain_id.clone(),
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
        .execute(&mut **tx)
        .await?;

        info!(dcid = %hex::encode(&dep_chain_id), error = ?err, "Set error on lock");
        Ok(rows.rows_affected())
    }

    /// Delay informational chain writes until immediately before the computation
    /// transaction commits. Holding this row during FHE execution would block
    /// ingestion, and updating it through another connection can deadlock.
    pub fn record_processing_error(&mut self, err: String) {
        self.pending_error = Some(err);
    }

    pub async fn flush_processing_error(
        &mut self,
        tx: &mut sqlx::Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        if let Some(err) = &self.pending_error {
            self.set_processing_error(tx, Some(err.clone())).await?;
        }
        Ok(())
    }

    pub fn clear_processing_error(&mut self) {
        self.pending_error.take();
    }

    /// Extend the lock expiration time on the current dependence chain
    ///
    /// If `enable_timeslice_check` is true,
    /// release the current lock when the computation time exceeds the timeslice
    pub async fn extend_or_release_current_lock(
        &mut self,
        enable_timeslice_check: bool,
    ) -> Result<Option<(Vec<u8>, LockingReason)>, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled, skipping extend_current_lock");
            return Ok(None);
        }

        let started_at = SystemTime::now();
        let (dependence_chain_id, created_at) = match &self.lock {
            Some((lock, created_at)) => (lock.dependence_chain_id.clone(), *created_at),
            None => {
                debug!("No lock to extend");
                return Ok(None);
            }
        };

        // Check timeslice
        if let Some(timeslice) = self.lock_timeslice_sec {
            if enable_timeslice_check
                && created_at
                    .elapsed()
                    .map(|d: std::time::Duration| d.as_secs())
                    .unwrap_or(0)
                    >= timeslice as u64
            {
                warn!(dcid = %hex::encode(&dependence_chain_id), timeslice = timeslice, "Max lock timeslice exceeded, releasing lock");

                // Release the lock instead of extending it as the timeslice's been consumed
                // Do not mark as processed so it can be re-acquired
                self.release_current_lock(false, true).await?;
                return Ok(None);
            }
        }

        // max_lock_ttl_sec

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
                self.take_lock();
                error!(dcid = %hex::encode(&dependence_chain_id), "No lock extended");
                return Ok(None);
            }
        };

        // Update the in-memory lock
        if let Some((lock, _)) = self.lock.as_mut() {
            lock.lock_expires_at = lock_expires_at.lock_expires_at;
            info!(dcid = %hex::encode(&dependence_chain_id), expires_at = ?lock.lock_expires_at, "Extended lock");
        }

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            EXTEND_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        Ok(Some((dependence_chain_id, LockingReason::ExtendedLock)))
    }

    pub async fn do_cleanup(&mut self) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            return Ok(0);
        }

        let should_run_cleanup = self
            .last_cleanup_at
            .map(|t| {
                t.elapsed().is_ok_and(|d| {
                    d.as_secs() as u32 >= self.cleanup_interval_sec.unwrap_or(CLEANUP_INTERVAL_SECS)
                })
            })
            .unwrap_or(true);

        let mut deleted = 0;

        if should_run_cleanup {
            self.last_cleanup_at = Some(SystemTime::now());
            info!("Performing cleanup of old processed dependence chains");
            deleted = delete_old_processed_dependence_chains(
                &self.pool,
                CLEANUP_BATCH_SIZE,
                self.processed_dcid_ttl_sec
                    .unwrap_or(CLEANUP_AGE_THRESHOLD_SECONDS),
            )
            .await?;
        }

        Ok(deleted)
    }

    pub fn get_current_lock(&self) -> Option<DatabaseChainLock> {
        self.lock.as_ref().map(|(lock, _)| lock.clone())
    }

    pub fn worker_id(&self) -> Uuid {
        self.worker_id
    }

    pub fn enabled(&self) -> bool {
        !self.disable_locking
    }

    /// Clear the current lock without releasing it in the database
    fn take_lock(&mut self) {
        self.lock.take();
        self.pending_error.take();
    }
}

/// Delete old processed dependence chains from the database
///
/// - `limit` specifies the maximum number of DCIDs to delete
/// - `threshold_sec` specifies the age threshold in seconds to avoid deleting recent DCIDs
async fn delete_old_processed_dependence_chains(
    pool: &sqlx::Pool<Postgres>,
    limit: i64,
    threshold_sec: u32,
) -> Result<u64, sqlx::Error> {
    if limit <= 0 {
        debug!("Limit is zero or negative, skipping deletion");
        return Ok(0);
    }

    let started_at = SystemTime::now();
    let result = sqlx::query!(
        r#"
    WITH to_delete AS (
        SELECT dependence_chain_id
        FROM dependence_chain
        WHERE status = 'processed'
            AND last_updated_at < NOW() - make_interval(secs => $2)
        ORDER BY last_updated_at ASC
        LIMIT $1
        FOR UPDATE SKIP LOCKED
    )
    DELETE FROM dependence_chain
    USING to_delete
    WHERE dependence_chain.dependence_chain_id = to_delete.dependence_chain_id
    "#,
        limit,
        threshold_sec as i64
    )
    .execute(pool)
    .await?;

    let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
    info!(rows_deleted = result.rows_affected(), query_elapsed = %elapsed, threshold_sec, 
        "Deleted old processed dependence chains");

    Ok(result.rows_affected())
}
