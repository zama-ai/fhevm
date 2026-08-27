use chrono::{DateTime, Utc};
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use sqlx::Postgres;
use std::{collections::HashSet, fmt, sync::LazyLock, time::SystemTime};
use time::PrimitiveDateTime;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockingReason {
    UpdatedUnowned, // Normal lock acquisition
    ExpiredLock,    // Work-stealing
    ExtendedLock,   // Lock extension
    Missing,        // No lock acquired
    /// Repair-path acquisition of a chain stranded by a stale
    /// dependency gate — distinct so repair activations are observable.
    StaleGateRepair,
}

impl From<&str> for LockingReason {
    fn from(s: &str) -> Self {
        match s {
            "updated_unowned" => LockingReason::UpdatedUnowned,
            "expired_lock" => LockingReason::ExpiredLock,
            "extended_lock" => LockingReason::ExtendedLock,
            "stale_gate_repair" => LockingReason::StaleGateRepair,
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
    // A worker normally owns one DCID. A bounded batch may own several; each
    // completed DCID is released independently so the next ready DCID can
    // refill its slot without waiting for the rest of the batch.
    locks: Vec<(DatabaseChainLock, SystemTime)>,
    last_stale_probe_at: Option<SystemTime>,

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
            locks: Vec::new(),
            last_stale_probe_at: None,
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

    /// Acquire the next available dependence-chain entry for processing
    /// sorted by last_updated_at (FIFO).
    /// Returns the dependence_chain_id if a lock was acquired
    pub async fn acquire_next_lock(
        &mut self,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
        Ok(self
            .acquire_next_locks(1)
            .await?
            .into_iter()
            .next()
            .unwrap_or((None, LockingReason::Missing)))
    }

    /// Acquire up to `limit` ready dependence chains in one atomic query.
    ///
    /// Completed entries can subsequently be released independently with
    /// `release_completed_locks`; this keeps a bounded batch full while work
    /// in other DCIDs is still pending.
    pub async fn acquire_next_locks(
        &mut self,
        limit: i32,
    ) -> Result<Vec<(Option<Vec<u8>>, LockingReason)>, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok(vec![(None, LockingReason::Missing)]);
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
                LIMIT $3
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
        .bind(limit.max(1))
        .fetch_all(&self.pool)
        .await?;

        if row.is_empty() {
            return Ok(vec![(None, LockingReason::Missing)]);
        }

        ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER.inc_by(row.len() as u64);

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        let acquired_at = SystemTime::now();
        let mut acquired = Vec::with_capacity(row.len());
        for lock in row {
            acquired.push((
                Some(lock.dependence_chain_id.clone()),
                LockingReason::from(lock.match_reason.as_str()),
            ));
            self.locks.push((lock, acquired_at));
        }
        info!(acquired_count = acquired.len(), query_elapsed = %elapsed, "Acquired locks");
        Ok(acquired)
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
                SELECT dependence_chain_id, 'updated_unowned' AS match_reason, dependency_count
                FROM dependence_chain
                WHERE
                    status = 'updated'      -- Marked as updated by host-listener
                    AND
                    worker_id IS NULL       -- Ensure no other workers own it
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

        self.locks.push((row.clone(), SystemTime::now()));
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

    /// Repair-path acquisition for chains STRANDED by a dependency_count
    /// that will never be decremented. The count is a live same-block gate:
    /// the listener arms it with the chain's in-block dependency count and
    /// each producer's mark-as-processed release decrements its dependents.
    /// That bookkeeping can be lost — the release status flip and the
    /// decrement are separate auto-commit statements (a worker crash between
    /// them skips the decrement), and reorgs can orphan a producer chain
    /// outright — leaving a chain whose producers are all processed (or
    /// gone) but whose count never reached zero. Such a chain matches
    /// neither normal acquisition predicate, and the no-progress escalation
    /// (`acquire_early_lock`) is only reachable through some OTHER acquired
    /// chain stalling, so in an otherwise idle pipeline it sits forever.
    ///
    /// Called when normal acquisition finds nothing. Strandedness is
    /// checked against ground truth — no producer chain naming this one a
    /// dependent is unprocessed — so a chain whose gate is legitimately
    /// live (producers still pending, e.g. during catchup where
    /// block-derived `last_updated_at` is arbitrarily old) is never picked
    /// up. The age gate on top avoids racing a listener transaction
    /// mid-arm. One race is accepted: a reorg-replay re-arming the
    /// producers between this statement's snapshot and its row lock is not
    /// seen by the NOT EXISTS (EvalPlanQual rechecks only the candidate's
    /// own quals), so the repair can zero a just-re-armed gate — bounded to
    /// an ordering hiccup, since missing boundary inputs defer at execution
    /// and the decrement clamp tolerates the lost count. Acquisition also resets the count to zero, which ground
    /// truth says it should be: the chain re-enters normal scheduling —
    /// including the expired-lock steal path, so a crash while holding this
    /// repair lock cannot re-strand it — and drains at full speed rather
    /// than one batch per age window.
    pub async fn acquire_stale_gated_lock(
        &mut self,
        min_age_secs: f64,
    ) -> Result<(Option<Vec<u8>>, LockingReason), sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok((None, LockingReason::Missing));
        }
        // This runs on every empty poll and its ground-truth anti-join has
        // no supporting index, so probe at most twice per age window —
        // repair tolerates minutes of latency by design, and during catchup
        // (block-derived last_updated_at) gated candidates are routine.
        let now = SystemTime::now();
        if let Some(last) = self.last_stale_probe_at {
            if now
                .duration_since(last)
                .map(|d| d.as_secs_f64() < min_age_secs / 2.0)
                .unwrap_or(false)
            {
                return Ok((None, LockingReason::Missing));
            }
        }
        self.last_stale_probe_at = Some(now);

        let started_at = SystemTime::now();
        let row = sqlx::query_as::<_, DatabaseChainLock>(
            r#"
            WITH candidate AS (
                SELECT dependence_chain_id, 'stale_gate_repair' AS match_reason
                FROM dependence_chain c
                WHERE
                    status = 'updated'      -- Marked as updated by host-listener
                    AND
                    worker_id IS NULL       -- Ensure no other workers own it
                    AND
                    dependency_count > 0    -- Gated: invisible to normal acquisition
                    AND
                    last_updated_at < NOW() - make_interval(secs => $3)
                    AND NOT EXISTS (        -- Ground truth: the gate is stale, every
                        SELECT 1            -- producer is processed or gone
                        FROM dependence_chain p
                        WHERE c.dependence_chain_id = ANY(p.dependents)
                          AND p.status <> 'processed'
                    )
                ORDER BY last_updated_at ASC, schedule_priority ASC
                FOR UPDATE SKIP LOCKED              -- Ensure no other worker is currently trying to lock it
                LIMIT 1
            )
            UPDATE dependence_chain AS dc
            SET
                worker_id = $1,
                status = 'processing',
                lock_acquired_at = NOW(),
                lock_expires_at = NOW() + make_interval(secs => $2),
                dependency_count = 0    -- what ground truth established above
            FROM candidate
            WHERE dc.dependence_chain_id = candidate.dependence_chain_id
            RETURNING dc.*, candidate.match_reason;
        "#,
        )
        .bind(self.worker_id)
        .bind(self.lock_ttl_sec)
        .bind(min_age_secs)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok((None, LockingReason::Missing));
        };
        // A successful repair must not consume the probe budget: a mass
        // stranding (one crashed producer, N dependents) should drain
        // back-to-back, throttled only once repairs stop finding work.
        self.last_stale_probe_at = None;

        self.locks.push((row.clone(), SystemTime::now()));
        ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER.inc();

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        info!(?row, query_elapsed = %elapsed, "Acquired lock on stranded DCID (repair path)");

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

        self.take_locks();
        info!(worker_id = %self.worker_id,
            count = rows.rows_affected(), "Released all locks");

        Ok(rows.rows_affected())
    }

    /// Release the lock held by this worker on the current dependence chain
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its status and last_updated_at
    pub async fn release_current_lock(
        &mut self,
        mark_as_processed: bool,
        update_at: Option<PrimitiveDateTime>,
    ) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled, skipping release_current_lock");
            return Ok(0);
        }

        let dep_chain_ids = self.get_current_lock_ids();
        if dep_chain_ids.is_empty() {
            debug!("No lock to release");
            return Ok(0);
        }

        // Since UPDATE always acquire a row-level lock internally,
        // this acts as atomic_exchange
        let rows = if let Some(update_at) = update_at {
            sqlx::query(
            r#"
            UPDATE dependence_chain
            SET
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                last_updated_at = $4::timestamp,
                status = CASE
                    WHEN status = 'processing' AND $3::bool THEN 'processed'       -- mark as processed
                    WHEN status = 'processing' AND NOT $3::bool THEN 'updated'     -- revert to updated so it can be re-acquired
                    ELSE status
                END
            WHERE worker_id = $1
            AND dependence_chain_id = ANY($2)
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_ids)
        .bind(mark_as_processed)
        .bind(update_at)
        .execute(&self.pool)
        .await?
        } else {
            sqlx::query(
            r#"
            UPDATE dependence_chain
            SET
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                status = CASE
                    WHEN status = 'processing' AND $3::bool THEN 'processed'       -- mark as processed
                    WHEN status = 'processing' AND NOT $3::bool THEN 'updated'     -- revert to updated so it can be re-acquired
                    ELSE status
                END
            WHERE worker_id = $1
            AND dependence_chain_id = ANY($2)
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_ids)
        .bind(mark_as_processed)
        .execute(&self.pool)
        .await?
        };

        let mut dependents_updated = 0;
        if mark_as_processed {
            // Get all dependents of a given dependence chain ID and decrement their dependency count
            // If any dependent's dependency count reaches zero, notify work_available
            //
            // A dependent must be decremented once PER released parent: a
            // single UPDATE row-matches each dependent only once, so a chain
            // gated on two parents released in the same batch would keep
            // dependency_count = 1 forever. Aggregate the multiplicity first.
            dependents_updated = sqlx::query(
                r#"
                WITH decrements AS (
                    SELECT dependent_id, count(*) AS n
                    FROM (
                        SELECT unnest(dependents) AS dependent_id
                        FROM dependence_chain
                        WHERE dependence_chain_id = ANY($1)
                    ) AS parent_dependents
                    GROUP BY dependent_id
                ),
                updated AS (
                    UPDATE dependence_chain dc
                    SET
                        dependency_count = GREATEST(dc.dependency_count - decrements.n, 0)
                    FROM decrements
                    WHERE dc.dependence_chain_id = decrements.dependent_id
                    RETURNING dc.dependence_chain_id, dc.dependency_count
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
            .bind(&dep_chain_ids)
            .execute(&self.pool)
            .await?
            .rows_affected();
        }

        self.take_locks();
        info!(
            dcid_count = dep_chain_ids.len(),
            rows = rows.rows_affected(),
            mark_as_processed,
            dependents_updated,
            "Released locks"
        );

        Ok(rows.rows_affected())
    }

    /// Release only owned DCIDs whose allowed work has reached a terminal
    /// state. The completion test, ownership release, and dependent-count
    /// decrement are one statement, so a slot cannot be released twice and a
    /// dependent observes exactly one decrement per completed parent.
    ///
    /// Call this only after the worker transaction that persisted the terminal
    /// computation state has committed. Pending, deferred, or concurrently
    /// re-updated DCIDs remain owned and continue through the normal lease
    /// lifecycle.
    pub async fn release_completed_locks(&mut self) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled, skipping release_completed_locks");
            return Ok(0);
        }

        let dep_chain_ids = self.get_current_lock_ids();
        if dep_chain_ids.is_empty() {
            return Ok(0);
        }

        let released: Vec<(Vec<u8>, bool)> = sqlx::query_as(
            r#"
            WITH completed AS (
                UPDATE dependence_chain AS dc
                SET
                    worker_id = NULL,
                    lock_acquired_at = NULL,
                    lock_expires_at = NULL,
                    status = CASE
                        -- If the listener refreshed this DCID while it was
                        -- owned, preserve its update so the new work is
                        -- acquired again instead of being hidden as processed.
                        WHEN dc.status = 'processing' THEN 'processed'
                        ELSE dc.status
                    END
                WHERE dc.worker_id = $1
                  AND dc.dependence_chain_id = ANY($2)
                  AND NOT EXISTS (
                      SELECT 1
                      FROM computations c
                      WHERE c.dependence_chain_id = dc.dependence_chain_id
                        AND c.is_allowed = TRUE
                        AND c.is_completed = FALSE
                        AND c.is_error = FALSE
                  )
                RETURNING
                    dc.dependence_chain_id,
                    dc.status = 'processed' AS marked_processed
            ),
            decrements AS (
                SELECT dependent_id, count(*) AS n
                FROM (
                    SELECT unnest(parent.dependents) AS dependent_id
                    FROM dependence_chain AS parent
                    JOIN completed
                      ON completed.dependence_chain_id = parent.dependence_chain_id
                     AND completed.marked_processed
                ) AS parent_dependents
                GROUP BY dependent_id
            ),
            updated AS (
                UPDATE dependence_chain AS dc
                SET dependency_count = GREATEST(dc.dependency_count - decrements.n, 0)
                FROM decrements
                WHERE dc.dependence_chain_id = decrements.dependent_id
                RETURNING dc.dependency_count
            )
            SELECT completed.dependence_chain_id,
                   EXISTS (SELECT 1 FROM updated WHERE dependency_count = 0)
                       AS dependent_became_ready
            FROM completed
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_ids)
        .fetch_all(&self.pool)
        .await?;

        if released.is_empty() {
            return Ok(0);
        }

        let notify_ready = released.iter().any(|(_, ready)| *ready);
        let released_ids: Vec<_> = released.into_iter().map(|(id, _)| id).collect();
        self.locks.retain(|(lock, _)| {
            !released_ids
                .iter()
                .any(|id| id == &lock.dependence_chain_id)
        });

        if notify_ready {
            sqlx::query("SELECT pg_notify('work_available', '')")
                .execute(&self.pool)
                .await?;
        }

        info!(
            dcid_count = released_ids.len(),
            remaining_dcid_count = self.locks.len(),
            dependents_notified = notify_ready,
            "Released completed locks"
        );
        Ok(released_ids.len() as u64)
    }

    /// Set error on the current dependence chain
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its error
    ///
    /// The error is only informational and does not affect the processing status
    pub async fn set_processing_error(&self, err: Option<String>) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok(0);
        }

        let dep_chain_ids = self.get_current_lock_ids();
        if dep_chain_ids.is_empty() {
            warn!("No lock to set error on");
            return Ok(0);
        }

        let rows = sqlx::query(
            r#"
            UPDATE dependence_chain
            SET
                error_message = CASE
                        WHEN status = 'processing' THEN $3
                        ELSE error_message
                        END
            WHERE worker_id = $1 AND dependence_chain_id = ANY($2)
            "#,
        )
        .bind(self.worker_id)
        .bind(&dep_chain_ids)
        .bind(&err)
        .execute(&self.pool)
        .await?;

        info!(dcid_count = dep_chain_ids.len(), error = ?err, "Set error on locks");
        Ok(rows.rows_affected())
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
        if self.locks.is_empty() {
            debug!("No lock to extend");
            return Ok(None);
        }

        // Check timeslice
        if let Some(timeslice) = self.lock_timeslice_sec {
            if enable_timeslice_check
                && self
                    .locks
                    .iter()
                    .map(|(_, created_at)| *created_at)
                    .min()
                    .unwrap_or_else(SystemTime::now)
                    .elapsed()
                    .map(|d: std::time::Duration| d.as_secs())
                    .unwrap_or(0)
                    >= timeslice as u64
            {
                warn!(
                    dcid_count = self.locks.len(),
                    timeslice = timeslice,
                    "Max lock timeslice exceeded, releasing locks"
                );

                // Release the lock instead of extending it as the timeslice's been consumed
                // Do not mark as processed so it can be re-acquired
                self.release_current_lock(false, None).await?;
                return Ok(None);
            }
        }

        // max_lock_ttl_sec

        let dep_chain_ids = self.get_current_lock_ids();
        // Return what was renewed rather than counting it: the statement extends
        // every row this worker still owns, so a short count means some rows were
        // stolen, not that the renewal failed. Forgetting the whole set would
        // strand the rows it just extended — unreachable to this worker, and to
        // every other one, until the expiry this very statement pushed out.
        let renewed: Vec<Vec<u8>> = sqlx::query_scalar(
            r#"
            UPDATE dependence_chain AS dc
                SET
                lock_expires_at = NOW() + make_interval(secs => $3)
            WHERE dependence_chain_id = ANY($1) AND worker_id = $2
            RETURNING dependence_chain_id
        "#,
        )
        .bind(&dep_chain_ids)
        .bind(self.worker_id)
        .bind(self.lock_ttl_sec as f64)
        .fetch_all(&self.pool)
        .await?;

        if renewed.len() != dep_chain_ids.len() {
            let renewed_ids: HashSet<&[u8]> = renewed.iter().map(|id| id.as_slice()).collect();
            self.locks
                .retain(|(lock, _)| renewed_ids.contains(lock.dependence_chain_id.as_slice()));
            error!(
                held = dep_chain_ids.len(),
                retained = renewed.len(),
                lost = dep_chain_ids.len() - renewed.len(),
                "Not all locks extended; dropping the stolen ones and keeping the rest"
            );
            if self.locks.is_empty() {
                return Ok(None);
            }
        } else {
            info!(dcid_count = dep_chain_ids.len(), "Extended locks");
        }

        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            EXTEND_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }

        Ok(self
            .get_current_lock_ids()
            .first()
            .cloned()
            .map(|id| (id, LockingReason::ExtendedLock)))
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
        self.locks.first().map(|(lock, _)| lock.clone())
    }

    pub fn get_current_lock_ids(&self) -> Vec<Vec<u8>> {
        self.locks
            .iter()
            .map(|(lock, _)| lock.dependence_chain_id.clone())
            .collect()
    }

    /// Stop extending the current lock without modifying its database row.
    /// The chain becomes eligible for work-stealing after its existing TTL.
    pub fn park_current_lock(&mut self) {
        if self.locks.is_empty() {
            debug!("No lock to park");
        } else {
            info!(
                dcid_count = self.locks.len(),
                "Parked locks until expiration"
            );
            self.take_locks();
        }
    }

    pub fn worker_id(&self) -> Uuid {
        self.worker_id
    }

    pub fn enabled(&self) -> bool {
        !self.disable_locking
    }

    /// Clear the current lock without releasing it in the database
    fn take_locks(&mut self) {
        self.locks.clear();
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
