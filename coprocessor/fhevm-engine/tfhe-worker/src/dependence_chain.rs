use chrono::{DateTime, Utc};
use fhevm_engine_common::types::SchedulePriority;
use prometheus::{
    register_histogram, register_int_counter, register_int_gauge, Histogram, IntCounter, IntGauge,
};
use sqlx::Postgres;
use std::{collections::HashSet, fmt, sync::LazyLock, time::SystemTime};
use time::PrimitiveDateTime;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::tfhe_worker::RETRYABLE_STAMP_MARKER;

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
/// Chains re-armed into the slow lane per sweep. Bounded so one sweep cannot
/// flood the acquisition queue with demoted work, and so the statement stays
/// short: the sweep runs on the same cadence as the delete, holding no lease.
const SLOW_LANE_REARM_BATCH: i64 = 500;

/// Chains re-armed into the slow lane by the most recent sweep.
///
/// The number to alert on. A steady non-zero value means work is failing
/// repeatedly and being retried forever rather than condemned — which is the
/// intended behaviour, and exactly the thing that would otherwise be silent
/// now that nothing gets terminalised for running out of attempts.
static SLOW_LANE_CHAINS_GAUGE: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_worker_slow_lane_chains",
        "dependence chains re-armed into the slow lane by the last sweep"
    )
    .expect("slow lane gauge registration")
});
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
    /// Attempts a retryable stamp gets per lane pass before its row stops
    /// holding the chain open. Mirrors the work window's own bound, so the
    /// completion test and the selection test agree on what is outstanding.
    /// `i16::MAX` by default, i.e. no demotion unless configured.
    demote_threshold: i16,
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
            demote_threshold: i16::MAX,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_conf(
        worker_id: Uuid,
        pool: sqlx::Pool<Postgres>,
        lock_ttl_sec: u32,
        disable_locking: bool,
        lock_timeslice_sec: Option<u32>,
        cleanup_interval_sec: Option<u32>,
        processed_dcid_ttl_sec: Option<u32>,
        demote_threshold: i16,
    ) -> Self {
        let mut mgr = Self::new(worker_id, pool);
        mgr.lock_ttl_sec = lock_ttl_sec as i64;
        mgr.disable_locking = disable_locking;
        mgr.lock_timeslice_sec = lock_timeslice_sec.map(|v| v as i64);
        mgr.cleanup_interval_sec = cleanup_interval_sec;
        mgr.processed_dcid_ttl_sec = processed_dcid_ttl_sec;
        mgr.demote_threshold = demote_threshold;
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

    /// Shared bookkeeping for the single-row acquisition paths, which differ
    /// only in their candidate predicate. Keeping it in one place is what stops
    /// the lock push, the counter, the histogram and the log line drifting
    /// apart between them.
    fn note_acquisition(
        &mut self,
        row: DatabaseChainLock,
        started_at: SystemTime,
        log_msg: &str,
    ) -> (Option<Vec<u8>>, LockingReason) {
        self.locks.push((row.clone(), SystemTime::now()));
        ACQUIRED_DEPENDENCE_CHAIN_ID_COUNTER.inc();
        let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
        if elapsed > 0.0 {
            ACQUIRE_DEPENDENCE_CHAIN_ID_QUERY_HISTOGRAM.observe(elapsed);
        }
        info!(?row, query_elapsed = %elapsed, "{}", log_msg);
        (
            Some(row.dependence_chain_id),
            LockingReason::from(row.match_reason.as_str()),
        )
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
                )
                -- Never re-acquire a chain this worker already holds. The
                -- work-stealing branch above matches on expiry alone, so a
                -- lease that lapsed while still held in memory is otherwise
                -- stolen back by its own owner into a SECOND lock-set entry:
                -- the extend statement then renews one row for two ids and
                -- reports a permanent, false "Not all locks extended".
                --
                -- Excluding `worker_id = $1` instead would be wrong. With
                -- `--worker-id` configured the id is stable across restarts,
                -- so a chain still stamped with it after a crash could never
                -- be reclaimed by the only worker that will ever run.
                AND dependence_chain_id <> ALL($4)
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
        .bind(self.get_current_lock_ids())
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

        let Some(row) = row else {
            return Ok((None, LockingReason::Missing));
        };

        Ok(self.note_acquisition(row, started_at, "Acquired lock on earliest DCID"))
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

        Ok(self.note_acquisition(
            row,
            started_at,
            "Acquired lock on stranded DCID (repair path)",
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

    /// Release every lock held by this worker.
    /// If host-listener has marked the dependence chain as 'updated' in the meantime,
    /// we don't overwrite its status and last_updated_at
    pub async fn release_current_lock(
        &mut self,
        mark_as_processed: bool,
        update_at: Option<PrimitiveDateTime>,
    ) -> Result<u64, sqlx::Error> {
        let dep_chain_ids = self.get_current_lock_ids();
        self.release_locks(&dep_chain_ids, mark_as_processed, update_at)
            .await
    }

    /// Release a SUBSET of the held locks, dropping exactly those from the
    /// in-memory set. Callers that need to evict one misbehaving chain from a
    /// batch use this rather than dumping the whole batch with it.
    pub async fn release_locks(
        &mut self,
        dep_chain_ids: &[Vec<u8>],
        mark_as_processed: bool,
        update_at: Option<PrimitiveDateTime>,
    ) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled, skipping release_locks");
            return Ok(0);
        }

        if dep_chain_ids.is_empty() {
            debug!("No lock to release");
            return Ok(0);
        }

        // Since UPDATE always acquire a row-level lock internally,
        // this acts as atomic_exchange.
        //
        // The release and the dependents' decrement are ONE statement, and
        // the decrement is driven by the rows the release actually flipped
        // (`released.marked_processed`), never by the ids this worker merely
        // believes it holds. Decrementing off `dep_chain_ids` would fire even
        // when the UPDATE matched nothing — e.g. after another worker stole
        // and completed the lease — releasing a dependent while a sibling
        // producer is still outstanding.
        //
        // A dependent must be decremented once PER released parent: a single
        // UPDATE row-matches each dependent only once, so a chain gated on
        // two parents released in the same batch would keep
        // dependency_count = 1 forever. Aggregate the multiplicity first.
        //
        // One statement for both callers: the only difference is whether the
        // release stamps a fresh `last_updated_at`, which COALESCE expresses
        // without a second copy of the CTE chain to keep in sync.
        let (released_rows, dependents_updated, notified) = {
            let r = sqlx::query!(
                r#"
            WITH released AS (
                UPDATE dependence_chain
                SET
                    worker_id = NULL,
                    lock_acquired_at = NULL,
                    lock_expires_at = NULL,
                    last_updated_at = COALESCE($4::timestamp, last_updated_at),
                    status = CASE
                        WHEN status = 'processing' AND $3::bool THEN 'processed'       -- mark as processed
                        WHEN status = 'processing' AND NOT $3::bool THEN 'updated'     -- revert to updated so it can be re-acquired
                        ELSE status
                    END
                WHERE worker_id = $1
                AND dependence_chain_id = ANY($2)
                -- Keyed on the status this row ACTUALLY reached, not on the
                -- caller's intent: the CASE above declines to flip a chain the
                -- listener refreshed to 'updated' while it was owned, and that
                -- chain has new work rather than a discharged gate. Returning
                -- $3 here instead would decrement its dependents anyway, and a
                -- dependent could then open with a sibling producer unrun.
                -- Plain identifier: sqlx's `!` non-null annotation applies to
                -- a query's own output columns, not to a CTE's, where the
                -- quoted name would be taken literally.
                RETURNING dependence_chain_id, dependents,
                          status = 'processed' AS marked_processed
            ),
            decrements AS (
                SELECT dependent_id, count(*) AS n
                FROM (
                    SELECT unnest(released.dependents) AS dependent_id
                    FROM released
                    WHERE released.marked_processed
                ) AS parent_dependents
                GROUP BY dependent_id
            ),
            updated AS (
                UPDATE dependence_chain dc
                SET dependency_count = GREATEST(dc.dependency_count - decrements.n, 0)
                FROM decrements
                WHERE dc.dependence_chain_id = decrements.dependent_id
                RETURNING dc.dependency_count
            ),
            notified AS (
                SELECT pg_notify('work_available', '')
                WHERE EXISTS (SELECT 1 FROM updated WHERE dependency_count = 0)
            )
            SELECT
                (SELECT count(*) FROM released) AS "released!",
                (SELECT count(*) FROM updated) AS "dependents_updated!",
                (SELECT count(*) FROM notified) AS "notified!",
                -- Returned rather than pruned here: see the separate statement
                -- below for why it cannot be another CTE.
                (SELECT COALESCE(array_agg(dependence_chain_id), ARRAY[]::bytea[])
                   FROM released WHERE marked_processed) AS "retired!"
            "#,
                self.worker_id,
                dep_chain_ids,
                mark_as_processed,
                update_at,
            )
            .fetch_one(&self.pool)
            .await?;

            // Prune what this retirement discharged, in its OWN statement.
            //
            // `dependents` is otherwise append-only, so it grows without bound
            // and — because the decrement is keyed on retirement rather than on
            // the arming — a chain re-armed and retired again would decrement
            // the same historical children a second time, opening a gate while
            // a sibling producer is still unrun. The listener re-adds a child
            // when it gates on this chain again, re-incrementing its count, so
            // arming and discharge stay paired.
            //
            // It CANNOT be a CTE of the statement above. Both would target the
            // same `dependence_chain` rows the release already updated, and
            // Postgres applies at most one update per row per statement: the
            // second silently does nothing. `release_completed_locks` documents
            // the same rule and prunes separately for exactly this reason.
            // Measured on a scratch database: as a CTE the prune reported 0
            // rows and left the array intact.
            if !r.retired.is_empty() {
                let pruned = sqlx::query!(
                    "UPDATE dependence_chain SET dependents = ARRAY[]::bytea[] \
                     WHERE dependence_chain_id = ANY($1) AND dependents <> ARRAY[]::bytea[]",
                    &r.retired,
                )
                .execute(&self.pool)
                .await?
                .rows_affected();
                debug!(
                    retired = r.retired.len(),
                    pruned, "Pruned discharged dependents"
                );
            }
            (r.released, r.dependents_updated, r.notified)
        };

        self.locks
            .retain(|(lock, _)| !dep_chain_ids.contains(&lock.dependence_chain_id));
        info!(
            dcid_count = dep_chain_ids.len(),
            remaining_dcid_count = self.locks.len(),
            rows = released_rows,
            mark_as_processed,
            dependents_updated,
            dependents_notified = notified > 0,
            "Released locks"
        );

        Ok(released_rows as u64)
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

        // Two statements in ONE transaction, deliberately not one statement.
        //
        // The decrement must be able to see a child row the listener inserted
        // after this release began: the listener arms a cross-block gate only
        // by inserting a brand-new chain row (its ON CONFLICT branch never
        // touches dependency_count), and a row created by a transaction that
        // commits after a statement started is invisible to that statement.
        // EvalPlanQual only re-reads rows the snapshot already saw, so folding
        // the decrement into the release's CTE would read the parent's updated
        // `dependents` array — the child IS listed — and then fail to match the
        // child's row, silently dropping the decrement and stranding it.
        // Under READ COMMITTED the second statement takes a fresh snapshot and
        // sees the child.
        //
        // Splitting also removes the same-statement collision when a released
        // chain is itself a dependent of another released chain: two updates of
        // one row within a single statement silently drop one of them.
        //
        // The pair is atomic: a failure in the decrement rolls the release back
        // too, so a chain is never released without discharging its dependents.
        let mut tx = self.pool.begin().await?;

        // Take every `dependence_chain` row lock this transaction will need up
        // front, in one globally-deterministic order.
        //
        // Without this the transaction locks its OWN chains first and their
        // dependents second, and that order is not global: with a batch of
        // chains, two workers need no cycle in the DCID graph to deadlock —
        // just two independent cross-block edges pointing opposite ways.
        // Worker A owns {X1, X2} with X1 -> Y1; worker B owns {Y1, Y2} with
        // Y2 -> X2; A holds X1,X2 and waits on Y1 while B holds Y1,Y2 and waits
        // on X2. Both edges are legal in a DAG, and one owned chain per worker
        // (the pre-batching shape) could not produce it.
        //
        // FOR NO KEY UPDATE, not FOR UPDATE: it is exactly the strength a
        // non-key UPDATE takes anyway, so neither statement below upgrades a
        // lock. No SKIP LOCKED — this must acquire all of them; waiting is
        // fine and cannot cycle once every waiter sorts the same way.
        //
        // Residual: a dependent the listener arms between this snapshot and the
        // release below is not in the set, and gets locked out of order by the
        // decrement. That is a much narrower window than the unordered case,
        // and pre-locking a parent also makes the listener's own gate query —
        // which is FOR UPDATE SKIP LOCKED — skip it and not arm the gate at
        // all.
        let candidate_dependents: Vec<Vec<u8>> = sqlx::query_scalar!(
            r#"
            SELECT unnest(dependents) AS "dependent_id!"
            FROM dependence_chain
            WHERE worker_id = $1 AND dependence_chain_id = ANY($2)
            "#,
            self.worker_id,
            &dep_chain_ids,
        )
        .fetch_all(tx.as_mut())
        .await?;

        let mut lock_set: Vec<Vec<u8>> = dep_chain_ids
            .iter()
            .cloned()
            .chain(candidate_dependents)
            .collect();
        lock_set.sort_unstable();
        lock_set.dedup();

        sqlx::query!(
            r#"
            SELECT 1 AS "locked!"
            FROM dependence_chain
            WHERE dependence_chain_id = ANY($1)
            ORDER BY dependence_chain_id
            FOR NO KEY UPDATE
            "#,
            &lock_set,
        )
        .fetch_all(tx.as_mut())
        .await?;

        let released = sqlx::query!(
            r#"
            UPDATE dependence_chain AS dc
            SET
                worker_id = NULL,
                lock_acquired_at = NULL,
                lock_expires_at = NULL,
                status = CASE
                    -- If the listener refreshed this DCID while it was owned,
                    -- preserve its update so the new work is acquired again
                    -- instead of being hidden as processed.
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
                    -- Mirror the work window's predicate exactly. A RETRYABLE
                    -- stamp is NOT terminal: the window re-selects it and
                    -- success heals it, so while it still has attempts it must
                    -- keep the chain open — retiring on it would leave the row
                    -- unreachable to every acquisition predicate.
                    --
                    -- Past the demote threshold the window has stopped
                    -- selecting it, so it must stop counting here too, or the
                    -- chain never retires and its dependents stay gated on work
                    -- that is no longer being attempted in this lane. The row
                    -- is NOT abandoned: the slow sweep re-arms the chain and
                    -- resets the count.
                    AND (c.is_error = FALSE
                         OR (c.error_message LIKE '%' || $3 || '%'
                             AND c.error_retry_count < $4))
              )
            RETURNING
                dc.dependence_chain_id,
                dc.dependents,
                dc.status = 'processed' AS "marked_processed!"
            "#,
            self.worker_id,
            &dep_chain_ids,
            RETRYABLE_STAMP_MARKER,
            self.demote_threshold,
        )
        .fetch_all(tx.as_mut())
        .await?;

        if released.is_empty() {
            tx.rollback().await?;
            return Ok(0);
        }

        // Discharge a dependent's gate ONLY when the 'processing' -> 'processed'
        // flip actually landed.
        //
        // It is tempting to discharge on every drained release, since a
        // listener refresh means the chain has NEW work rather than that the
        // work the child was gated on is outstanding. That is wrong: the
        // refreshed chain is released as 'updated', is immediately
        // re-acquirable, and retires on a later cycle — so the child would be
        // decremented twice for one arming and could open with a SIBLING
        // producer still unrun. The flip happens exactly once per retirement,
        // which is the closest thing to an arming-scoped event available here.
        //
        // (This is still only approximately exactly-once: a chain re-armed with
        // new work and retired again decrements its historical dependents a
        // second time, because `dependents` is append-only and the decrement is
        // keyed on retirement rather than on the arming. Closing that needs the
        // discharged children pruned from the parent's `dependents`, which
        // changes what `acquire_stale_gated_lock`'s ground-truth anti-join
        // means and is left as a follow-up. GREATEST(..., 0) bounds the damage
        // to a gate opening early, never to a negative count.)
        //
        // Multiplicity is preserved across parents on purpose: a chain gated on
        // two parents retired together must observe both decrements, and one
        // UPDATE row-matches each dependent only once.
        let discharged: Vec<Vec<u8>> = released
            .iter()
            .filter(|row| row.marked_processed)
            .flat_map(|row| row.dependents.iter().cloned())
            .collect();

        let notify_ready = if discharged.is_empty() {
            false
        } else {
            sqlx::query_scalar!(
                r#"
                WITH decrements AS (
                    SELECT dependent_id, count(*) AS n
                    FROM unnest($1::bytea[]) AS dependent_id
                    GROUP BY dependent_id
                ),
                updated AS (
                    UPDATE dependence_chain AS dc
                    SET dependency_count = GREATEST(dc.dependency_count - decrements.n, 0)
                    FROM decrements
                    WHERE dc.dependence_chain_id = decrements.dependent_id
                    RETURNING dc.dependency_count
                )
                SELECT EXISTS (
                    SELECT 1 FROM updated WHERE dependency_count = 0
                ) AS "dependent_became_ready!"
                "#,
                &discharged,
            )
            .fetch_one(tx.as_mut())
            .await?
        };

        // Prune what was just discharged. `dependents` is otherwise
        // append-only — the listener unions new children in and nothing ever
        // removes them — which costs two things: the array grows without bound
        // for a long-lived chain, and the decrement is keyed on RETIREMENT
        // rather than on the arming, so a chain re-armed with new work and
        // retired again decrements its historical dependents a second time and
        // can open a gate while a sibling producer is still unrun.
        //
        // Clearing on retirement closes both. It is exactly the discharged set:
        // every child in the array is decremented once by the statement above,
        // so none is owed another decrement for this arming. A child gated on
        // this chain again later is re-added by the listener's upsert, which
        // also re-increments its count — arming and discharge stay paired.
        //
        // Safe for `acquire_stale_gated_lock`'s anti-join, which looks for
        // producers with `status <> 'processed'`: a retired parent is
        // 'processed', so it never matched that predicate anyway.
        let retired: Vec<Vec<u8>> = released
            .iter()
            .filter(|row| row.marked_processed)
            .map(|row| row.dependence_chain_id.clone())
            .collect();
        if !retired.is_empty() {
            sqlx::query!(
                "UPDATE dependence_chain SET dependents = ARRAY[]::bytea[] \
                 WHERE dependence_chain_id = ANY($1) AND dependents <> ARRAY[]::bytea[]",
                &retired,
            )
            .execute(tx.as_mut())
            .await?;
        }

        if notify_ready {
            sqlx::query!("SELECT pg_notify('work_available', '')")
                .execute(tx.as_mut())
                .await?;
        }

        tx.commit().await?;

        let processed_count = released.iter().filter(|row| row.marked_processed).count();
        let released_ids: Vec<_> = released
            .into_iter()
            .map(|row| row.dependence_chain_id)
            .collect();
        self.locks.retain(|(lock, _)| {
            !released_ids
                .iter()
                .any(|id| id == &lock.dependence_chain_id)
        });

        info!(
            dcid_count = released_ids.len(),
            processed_count,
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
    /// Record an informational error on the chain that OWNS the failing
    /// computation.
    ///
    /// Scoped by `owner`, because one computation's failure is not a fact about
    /// the other chains this worker happens to hold: with batching that is up
    /// to `--dependence-chains-per-batch` of them, and stamping all of them
    /// spreads one message across ~20 unrelated diagnostics. The column is
    /// informational only — nothing reads it to make a decision — so this is
    /// legibility, not correctness.
    pub async fn set_processing_error(
        &self,
        err: Option<String>,
        owner: Option<&[u8]>,
    ) -> Result<u64, sqlx::Error> {
        if self.disable_locking {
            debug!("Locking is disabled");
            return Ok(0);
        }

        let dep_chain_ids = self.get_current_lock_ids();
        if dep_chain_ids.is_empty() {
            warn!("No lock to set error on");
            return Ok(0);
        }

        let rows = sqlx::query!(
            r#"
            UPDATE dependence_chain
            SET
                error_message = CASE
                        WHEN status = 'processing' THEN $3
                        ELSE error_message
                        END
            WHERE worker_id = $1 AND dependence_chain_id = ANY($2)
              -- NULL owner keeps the old fan-out for callers that genuinely
              -- have no single owning chain.
              AND ($4::bytea IS NULL OR dependence_chain_id = $4)
            "#,
            self.worker_id,
            &dep_chain_ids,
            err.as_deref(),
            owner,
        )
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

        // Check timeslice, PER LOCK.
        //
        // Two things this must not do, both of which the batch-wide version
        // did. It must not measure the batch by its oldest member — one chain
        // that never finishes would then evict 19 healthy ones every slice.
        // And it must rotate what it evicts: releasing with `update_at = None`
        // leaves `last_updated_at` at its block-derived value, so the chain
        // goes back to the FIFO FRONT and this same worker re-acquires it
        // microseconds later — churn, not escape.
        //
        // With per-lock accounting and a rotation, the timeslice is the batch's
        // one working eviction path for a chain that occupies a slot without
        // ever progressing (a boundary input whose producer was never ingested
        // defers forever and stamps nothing, so it is invisible to the
        // no-progress counter, which any other chain in the batch resets).
        if let Some(timeslice) = self.lock_timeslice_sec {
            if enable_timeslice_check {
                let expired: Vec<Vec<u8>> = self
                    .locks
                    .iter()
                    .filter(|(_, created_at)| {
                        created_at
                            .elapsed()
                            .map(|d: std::time::Duration| d.as_secs())
                            .unwrap_or(0)
                            >= timeslice as u64
                    })
                    .map(|(lock, _)| lock.dependence_chain_id.clone())
                    .collect();

                if !expired.is_empty() {
                    warn!(
                        expired_dcid_count = expired.len(),
                        held_dcid_count = self.locks.len(),
                        timeslice = timeslice,
                        "Max lock timeslice exceeded, rotating the chains that consumed it"
                    );

                    // Release rather than extend: the slice is spent. Not
                    // processed, so the chain stays pending — but with a fresh
                    // last_updated_at, so oldest-first acquisition moves on to
                    // younger chains instead of handing it straight back.
                    let rotate_at = {
                        let offset = time::OffsetDateTime::now_utc();
                        PrimitiveDateTime::new(offset.date(), offset.time())
                    };
                    self.release_locks(&expired, false, Some(rotate_at)).await?;

                    if self.locks.is_empty() {
                        return Ok(None);
                    }
                }
            }
        }

        // max_lock_ttl_sec

        let dep_chain_ids = self.get_current_lock_ids();
        // Return what was renewed rather than counting it: the statement extends
        // every row this worker still owns, so a short count means some rows were
        // stolen, not that the renewal failed. Forgetting the whole set would
        // strand the rows it just extended — unreachable to this worker, and to
        // every other one, until the expiry this very statement pushed out.
        let renewed: Vec<Vec<u8>> = sqlx::query_scalar!(
            r#"
            UPDATE dependence_chain AS dc
                SET
                lock_expires_at = NOW() + make_interval(secs => $3)
            WHERE dependence_chain_id = ANY($1) AND worker_id = $2
            RETURNING dependence_chain_id
        "#,
            &dep_chain_ids,
            self.worker_id,
            self.lock_ttl_sec as f64,
        )
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
            // Before deleting: give demoted rows their next pass. Running the
            // sweep first means a chain re-armed here is no longer 'processed'
            // when the delete runs, so the two cannot race over it.
            let rearmed = rearm_demoted_chains(&self.pool, self.demote_threshold).await?;
            if rearmed > 0 {
                info!(
                    rearmed,
                    "Re-armed demoted dependence chains into the slow lane"
                );
            }
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

    /// Deduplicated: `extend_current_locks` compares this length against the
    /// rows its UPDATE renewed, and one row can only be returned once, so a
    /// repeated id would report a lock as lost on every cycle forever.
    pub fn get_current_lock_ids(&self) -> Vec<Vec<u8>> {
        let mut ids: Vec<Vec<u8>> = self
            .locks
            .iter()
            .map(|(lock, _)| lock.dependence_chain_id.clone())
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
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

/// Give demoted rows another pass, in the slow lane.
///
/// A row whose retryable stamp has spent its attempts stops being selected by
/// the work window and stops holding its chain open, so the chain retires and
/// its dependents discharge — but the row is still pending and may still be
/// healable. This is what stops that being a quiet abandonment: on a long
/// cadence, every chain still holding such a row is re-armed and its counts
/// reset, so the row is attempted again.
///
/// `SchedulePriority::Slow` is what keeps this from costing anything: the
/// acquisition query already orders by it, so a re-armed chain is picked up
/// only once every ordinary chain has been. The listener's upsert raises
/// priority with `GREATEST`, never lowers it, so the demotion survives a
/// refresh — a chain that has needed the slow lane once keeps yielding to
/// fresh work.
///
/// Nothing here writes a verdict. A permanently failing row costs a bounded
/// trickle of retries per sweep interval for as long as it exists, and is
/// visible as a non-zero slow-lane gauge rather than as a condemned cone.
pub(crate) async fn rearm_demoted_chains(
    pool: &sqlx::Pool<Postgres>,
    demote_threshold: i16,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        WITH demoted AS (
            SELECT DISTINCT c.dependence_chain_id
            FROM computations c
            JOIN dependence_chain dc
              ON dc.dependence_chain_id = c.dependence_chain_id
            WHERE c.is_allowed = TRUE
              AND c.is_completed = FALSE
              AND c.is_error = TRUE
              AND c.error_message LIKE '%' || $1 || '%'
              AND c.error_retry_count >= $2
              -- Only chains nothing is working on. A 'processing' chain is
              -- owned and will retire on its own terms; re-arming it under its
              -- owner would strand the lease.
              AND dc.status = 'processed'
              AND dc.worker_id IS NULL
            LIMIT $3
        ),
        rearmed AS (
            UPDATE dependence_chain dc
            SET status = 'updated',
                dependency_count = 0,
                schedule_priority = $4,
                last_updated_at = NOW()
            FROM demoted
            WHERE dc.dependence_chain_id = demoted.dependence_chain_id
              -- Re-checked under this statement's own row lock. Between the
              -- SELECT above and here the listener can re-arm the chain with
              -- new work; without these two predicates the sweep would zero a
              -- freshly-armed dependency_count and open a gate early.
              AND dc.status = 'processed'
              AND dc.worker_id IS NULL
            RETURNING dc.dependence_chain_id
        ),
        reset AS (
            -- The count means "attempts in the current lane pass", so it has
            -- to go back to zero or the work window would skip the row it was
            -- just re-armed for.
            UPDATE computations c
            SET error_retry_count = 0
            FROM rearmed
            WHERE c.dependence_chain_id = rearmed.dependence_chain_id
              AND c.is_completed = FALSE
              AND c.is_error = TRUE
              AND c.error_message LIKE '%' || $1 || '%'
            RETURNING 1
        )
        SELECT
            (SELECT count(*) FROM rearmed) AS "rearmed!",
            (SELECT count(*) FROM reset) AS "reset!"
        "#,
        RETRYABLE_STAMP_MARKER,
        demote_threshold,
        SLOW_LANE_REARM_BATCH,
        i16::from(SchedulePriority::Slow),
    )
    .fetch_one(pool)
    .await?;

    SLOW_LANE_CHAINS_GAUGE.set(result.rearmed);
    Ok(result.rearmed as u64)
}

/// Delete old processed dependence chains from the database
///
/// - `limit` specifies the maximum number of DCIDs to delete
/// - `threshold_sec` specifies the age threshold in seconds to avoid deleting recent DCIDs
pub(crate) async fn delete_old_processed_dependence_chains(
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
        FROM dependence_chain dc
        WHERE status = 'processed'
            AND last_updated_at < NOW() - make_interval(secs => $2)
            -- Never delete a chain that still owns unfinished work. A chain
            -- retires as 'processed' while holding demoted rows, and this row
            -- is the ONLY handle on them: nothing TTL-deletes `computations`,
            -- so dropping the chain would orphan the payload and put it beyond
            -- the slow sweep's reach permanently.
            AND NOT EXISTS (
                SELECT 1
                FROM computations c
                WHERE c.dependence_chain_id = dc.dependence_chain_id
                  AND c.is_allowed = TRUE
                  AND c.is_completed = FALSE
                  -- Only PENDING or DEMOTED work may hold a chain back. A
                  -- terminal verdict is is_completed = false forever — the work
                  -- window never re-selects it so no bytes ever arrive, and the
                  -- sweep never touches it because it carries no RETRYABLE
                  -- marker. Without this clause such a chain matches the guard
                  -- on every pass, is never deleted, and accumulates at the
                  -- head of the `last_updated_at ASC` scan: an unbounded
                  -- residue that the 48 h TTL used to clear.
                  AND (c.is_error = FALSE OR c.error_message LIKE '%' || $3 || '%')
            )
        ORDER BY last_updated_at ASC
        LIMIT $1
        FOR UPDATE SKIP LOCKED
    )
    DELETE FROM dependence_chain
    USING to_delete
    WHERE dependence_chain.dependence_chain_id = to_delete.dependence_chain_id
    "#,
        limit,
        threshold_sec as i64,
        RETRYABLE_STAMP_MARKER,
    )
    .execute(pool)
    .await?;

    let elapsed = started_at.elapsed().map(|d| d.as_secs_f64()).unwrap_or(0.0);
    info!(rows_deleted = result.rows_affected(), query_elapsed = %elapsed, threshold_sec,
        "Deleted old processed dependence chains");

    Ok(result.rows_affected())
}
