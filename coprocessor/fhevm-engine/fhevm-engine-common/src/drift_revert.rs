//! Auto drift recovery: shared coordination logic.
//!
//! When the gw-listener detects ciphertext drift, it writes a row to
//! `drift_revert_signal`. All coprocessor services poll that table and
//! re-exec themselves when a signal appears. On startup, all fresh
//! processes check for a pending signal:
//!  * gw-listener runs the revert SQL,
//!  * other services wait until it's done, then all proceed normally

use std::sync::LazyLock;
use std::time::Duration;

use prometheus::{register_int_counter_vec, IntCounterVec};
use sqlx::{Pool, Postgres, Row};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

const REVERT_SQL_TEMPLATE: &str =
    include_str!("../../db-migration/db-scripts/revert_coprocessor_db_state.sql");

/// How often services poll `drift_revert_signal` for state changes.
pub const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Per-iteration bound on each signal-poll query.
pub const POLL_QUERY_TIMEOUT: Duration = Duration::from_secs(5);

/// Cumulative limit on DB unreachability. If the watcher cannot reach the
/// DB (no successful poll) for longer than this, the process exits so the
/// supervisor restarts it fresh. Prevents a service from running with stale
/// in-memory state through a long DB outage during which the drift runner
/// may have completed a revert without it noticing.
pub const DRIFT_REVERT_DB_DOWN_LIMIT: Duration = Duration::from_secs(60);

/// The revert runner's grace period must be at least this many times
/// `DRIFT_REVERT_DB_DOWN_LIMIT`.
pub const MIN_GRACE_PERIOD_MULTIPLIER: u32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct WatcherTimeouts {
    pub poll_query_timeout: Duration,
    pub db_down_limit: Duration,
}

impl Default for WatcherTimeouts {
    fn default() -> Self {
        Self {
            poll_query_timeout: POLL_QUERY_TIMEOUT,
            db_down_limit: DRIFT_REVERT_DB_DOWN_LIMIT,
        }
    }
}

static SIGNAL_CREATED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_drift_revert_signal_created_counter",
        "Number of drift-revert signal recordings (one per detected consensus drift; \
         includes both new signals and existing pending signals lowered to an earlier block)",
        &["host_chain_id"]
    )
    .unwrap()
});

static REVERT_SUCCESS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_drift_revert_success_counter",
        "Number of drift reverts that ran successfully (SQL completed and signal marked Done)",
        &["host_chain_id"]
    )
    .unwrap()
});

static REVERT_FAILURE_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_drift_revert_failure_counter",
        "Number of drift reverts that failed during SQL execution (signal marked Failed)",
        &["host_chain_id"]
    )
    .unwrap()
});

static TOO_MANY_ATTEMPTS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_drift_revert_too_many_attempts_counter",
        "Number of times the revert runner refused to revert because too many \
         successful reverts already happened in the recent window — indicates a \
         deterministic loop where reverts succeed but drift keeps recurring",
        &["host_chain_id"]
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalStatus {
    Pending,
    Reverting,
    Done,
    Failed(String),
}

const FAILED_DB_PREFIX: &str = "failed: ";

impl SignalStatus {
    pub fn as_db_str(&self) -> String {
        match self {
            Self::Pending => "pending".to_owned(),
            Self::Reverting => "reverting".to_owned(),
            Self::Done => "done".to_owned(),
            Self::Failed(reason) => format!("{FAILED_DB_PREFIX}{reason}"),
        }
    }

    fn from_db_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "reverting" => Self::Reverting,
            "done" => Self::Done,
            other => Self::Failed(
                other
                    .strip_prefix(FAILED_DB_PREFIX)
                    .unwrap_or(other)
                    .to_owned(),
            ),
        }
    }

    fn failed_like_pattern() -> String {
        format!("{FAILED_DB_PREFIX}%")
    }
}

#[derive(Debug, Clone)]
pub struct DriftRevertSignal {
    pub id: i64,
    pub host_chain_id: i64,
    pub offending_host_block_number: i64,
    pub status: SignalStatus,
}

/// Record a drift revert signal for `host_chain_id`. Atomically:
///   - INSERT a new Pending signal if no in-flight signal exists for this chain.
///   - Else, if a Pending signal exists pointing at a later block, lower its
///     `offending_host_block_number` to the new (earlier) value.
///   - Else (Reverting or Pending already at earlier-or-equal
///     block) no-op.
///
/// Lowering matters because drifts can be observed out of host-block order.
/// The runner commits to whatever block is set when the grace period ends, so
/// lowering during the grace window pulls the revert target back to the
/// earliest known drift.
///
/// In practice the `lower` branch only fires in the original gw-listener
/// process, between signal creation and re-exec.
/// After re-exec, the new process runs the revert during `init` before the
/// drift detector restarts, meaning this function cannot be called.
///
/// Returns `Some(id)` for either action (created or lowered), `None` for
/// no-op.
pub async fn create_revert_signal(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    offending_host_block_number: i64,
) -> anyhow::Result<Option<i64>> {
    let row = sqlx::query(
        "WITH ins AS ( \
            INSERT INTO drift_revert_signal (host_chain_id, offending_host_block_number, status) \
            SELECT $1, $2, $3 \
            WHERE NOT EXISTS ( \
                SELECT 1 FROM drift_revert_signal \
                WHERE host_chain_id = $1 AND (status = $3 OR status = $4) \
            ) \
            RETURNING id \
         ), upd AS ( \
            UPDATE drift_revert_signal \
            SET offending_host_block_number = $2, updated_at = NOW() \
            WHERE host_chain_id = $1 \
              AND status = $3 \
              AND offending_host_block_number > $2 \
              AND NOT EXISTS (SELECT 1 FROM ins) \
            RETURNING id \
         ) \
         SELECT id FROM ins UNION ALL SELECT id FROM upd",
    )
    .bind(host_chain_id)
    .bind(offending_host_block_number)
    .bind(SignalStatus::Pending.as_db_str())
    .bind(SignalStatus::Reverting.as_db_str())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get("id")))
}

/// Fetch the latest signal row (by id), if any.
pub async fn latest_signal(pool: &Pool<Postgres>) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status \
         FROM drift_revert_signal ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

/// Fetch the latest signal row (by id) for a specific chain, if any.
pub async fn latest_signal_for_chain(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status \
         FROM drift_revert_signal
         WHERE host_chain_id = $1
         ORDER BY id DESC LIMIT 1",
    )
    .bind(host_chain_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

/// Fetch the a specific signal row.
pub async fn drift_signal_for_chain(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    drift_id: i64,
) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status \
         FROM drift_revert_signal
         WHERE host_chain_id = $1
         AND id = $2
         ORDER BY id DESC LIMIT 1",
    )
    .bind(host_chain_id)
    .bind(drift_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

/// Fetch the oldest in-flight (Pending or Reverting) signal, if any. The
/// runner processes signals in this order so no chain's drift is dropped
/// when multiple chains report drift concurrently.
pub async fn oldest_in_flight_signal(
    pool: &Pool<Postgres>,
) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status \
         FROM drift_revert_signal \
         WHERE status = $1 OR status = $2 \
         ORDER BY id ASC LIMIT 1",
    )
    .bind(SignalStatus::Pending.as_db_str())
    .bind(SignalStatus::Reverting.as_db_str())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

/// Fetch the most recent Failed signal across any host chain.
pub async fn latest_failed_signal(
    pool: &Pool<Postgres>,
) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status
         FROM drift_revert_signal
         WHERE status LIKE $1
         ORDER BY id DESC LIMIT 1",
    )
    .bind(SignalStatus::failed_like_pattern())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

/// Fetch the signal that should block service startup, in priority order:
///   1. Most recent Failed signal — caller must bail.
///   2. Oldest in-flight (Pending or Reverting) — caller must wait.
///   3. None — caller can proceed.
async fn blocking_signal(pool: &Pool<Postgres>) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status FROM (
            (SELECT 1 AS priority, id, host_chain_id, offending_host_block_number, status
             FROM drift_revert_signal
             WHERE status LIKE $1
             ORDER BY id DESC LIMIT 1)
            UNION ALL
            (SELECT 2 AS priority, id, host_chain_id, offending_host_block_number, status
             FROM drift_revert_signal
             WHERE status = $2 OR status = $3
             ORDER BY id ASC LIMIT 1)
         ) sq
         ORDER BY priority ASC LIMIT 1",
    )
    .bind(SignalStatus::failed_like_pattern())
    .bind(SignalStatus::Pending.as_db_str())
    .bind(SignalStatus::Reverting.as_db_str())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(signal_from_row))
}

fn signal_from_row(r: sqlx::postgres::PgRow) -> DriftRevertSignal {
    let status_str: String = r.get("status");
    DriftRevertSignal {
        id: r.get("id"),
        host_chain_id: r.get("host_chain_id"),
        offending_host_block_number: r.get("offending_host_block_number"),
        status: SignalStatus::from_db_str(&status_str),
    }
}

/// Update the status of a signal row.
pub async fn update_signal_status(
    pool: &Pool<Postgres>,
    signal_id: i64,
    status: &SignalStatus,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE drift_revert_signal SET status = $1, updated_at = NOW() WHERE id = $2")
        .bind(status.as_db_str())
        .bind(signal_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Called by the drift detector when it detects a ciphertext drift for a
/// given handle. Looks up the host chain block where the drifted computation
/// originated and creates a revert signal so all services can coordinate.
///
/// Transient failures (DB errors) are logged and swallowed so they don't
/// cascade into the drift detector — the next detection cycle will retry.
pub async fn on_drift_detected(pool: &Pool<Postgres>, handle: &[u8], host_chain_id: i64) {
    // Byte 21 is 0xff for compute outputs; inputs encode the ciphertext index
    // in the proof (0x00..=0xfe). Input drift is out of scope for auto-recovery.
    if handle.len() != 32 || handle[21] != 0xff {
        warn!(
            host_chain_id,
            "Drifted handle is a ZK input; auto-recovery is not supported for input handles"
        );
        return;
    }

    let host_block: Option<i64> = match sqlx::query_scalar(
        "SELECT MIN(block_number) FROM computations \
         WHERE output_handle = $1 AND host_chain_id = $2",
    )
    .bind(handle)
    .bind(host_chain_id)
    .fetch_optional(pool)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            error!(error = %e, host_chain_id, "Failed to look up computation for drifted handle");
            return;
        }
    };

    let Some(block) = host_block else {
        error!(
            host_chain_id,
            "Cannot create revert signal: no computation found for drifted handle"
        );
        return;
    };

    match create_revert_signal(pool, host_chain_id, block).await {
        Ok(Some(id)) => {
            SIGNAL_CREATED_COUNTER
                .with_label_values(&[&host_chain_id.to_string()])
                .inc();
            info!(
                host_chain_id,
                block, id, "Drift revert signal recorded (created or lowered to earlier block)"
            );
        }
        Ok(None) => {
            warn!(
                host_chain_id,
                block,
                "Drift revert signal not recorded: revert already in flight or pending at earlier block"
            );
        }
        Err(e) => {
            error!(error = %e, "Failed to record drift revert signal");
        }
    }
}

/// Poll `drift_revert_signal` until any signal is in flight (Pending or
/// Reverting) on any host chain. Used by `run_signal_watcher` to detect that
/// a drift revert is happening so the service can re-exec.
///
/// Transient DB errors are logged and skipped — the watcher must stay
/// alive. However, if the DB is unreachable for longer than
/// `DRIFT_REVERT_DB_DOWN_LIMIT`, the process exits so the supervisor can
/// restart it with fresh in-memory state. Each individual poll is bounded
/// by `POLL_QUERY_TIMEOUT`.
pub async fn wait_for_in_flight_signal(
    pool: &Pool<Postgres>,
    timeouts: WatcherTimeouts,
) -> DriftRevertSignal {
    let mut last_success = std::time::Instant::now();
    loop {
        match tokio::time::timeout(timeouts.poll_query_timeout, oldest_in_flight_signal(pool)).await
        {
            Ok(Ok(Some(signal))) => return signal,
            Ok(Ok(None)) => {
                last_success = std::time::Instant::now();
            }
            Ok(Err(e)) => {
                error!(error = %e, "Drift-revert watcher poll failed");
            }
            Err(_) => {
                error!(
                    timeout = ?timeouts.poll_query_timeout,
                    "Drift-revert watcher poll timed out"
                );
            }
        }

        let elapsed = last_success.elapsed();
        if elapsed >= timeouts.db_down_limit {
            error!(
                elapsed = ?elapsed,
                limit = ?timeouts.db_down_limit,
                "Drift-revert watcher could not reach the DB for too long; exiting \
                 for supervisor restart so in-memory state cannot drift past a revert"
            );
            std::process::exit(1);
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Poll until there are no in-flight (Pending or Reverting) signals across
/// any host chain, or until `cancel_token` fires. Used by non-runner services
/// to wait for all pending reverts to finish on startup.
///
/// Returns an error if any host chain has an unresolved `Failed` signal at
/// any point — the waiter must not let the service start on a DB where any
/// chain's revert failed.
pub async fn wait_for_revert_done(
    pool: &Pool<Postgres>,
    cancel_token: &CancellationToken,
) -> anyhow::Result<()> {
    let Some(signal) = blocking_signal(pool).await? else {
        return Ok(());
    };
    bail_if_failed(
        &signal,
        "Unresolved Failed drift revert signal — refusing to start",
    )?;
    info!(
        signal_id = signal.id,
        host_chain_id = signal.host_chain_id,
        status = signal.status.as_db_str(),
        "Waiting for drift revert to complete"
    );

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => return Ok(()),
            _ = tokio::time::sleep(POLL_INTERVAL) => {}
        }
        match blocking_signal(pool).await? {
            None => {
                info!("Drift revert complete, resuming normal operation");
                return Ok(());
            }
            Some(s) => bail_if_failed(&s, "Drift revert reached Failed state while waiting")?,
        }
    }
}

/// Logs and bails if `signal` has `Failed` status. No-op otherwise.
fn bail_if_failed(signal: &DriftRevertSignal, message: &str) -> anyhow::Result<()> {
    if let SignalStatus::Failed(reason) = &signal.status {
        error!(
            signal_id = signal.id,
            host_chain_id = signal.host_chain_id,
            reason,
            "{message}"
        );
        anyhow::bail!("drift revert signal {} is Failed: {reason}", signal.id);
    }
    Ok(())
}

/// Prepare the revert SQL by replacing psql-specific syntax with concrete
/// parameter values so the script can be executed via sqlx as raw SQL.
fn prepare_revert_sql(chain_id: i64, to_block_number: i64) -> String {
    REVERT_SQL_TEMPLATE
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Strip psql directives
            !trimmed.starts_with("\\set")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace(":'chain_id'", &chain_id.to_string())
        .replace(":'to_block_number'", &to_block_number.to_string())
}

/// Execute the revert SQL script against the database.
///
/// `to_block = offending_host_block_number - 1` — we subtract 1 because the
/// script reverts everything strictly greater than `to_block`, and we want the
/// offending block itself gone. If ciphertexts from earlier blocks also
/// drifted (due to out-of-order processing), the drift detector will catch
/// them eventually in subsequent rounds.
///
/// Refuses to revert when `offending <= 1`: the SQL script rejects
/// `to_block <= 0`, and clamping to `to_block = 1` would silently leave
/// block 1's drifted state in place (the SQL only deletes blocks `> to_block`).
/// Drift on block 1 is realistic only on fresh chains / test environments and
/// requires operator intervention to wipe the chain explicitly.
pub async fn execute_revert(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    offending_host_block_number: i64,
) -> anyhow::Result<()> {
    if offending_host_block_number <= 1 {
        anyhow::bail!(
            "refusing auto-revert for offending block {offending_host_block_number} on \
             chain {host_chain_id}: cannot delete block <= 1 via the revert script — \
             operator must wipe the chain manually"
        );
    }
    let to_block_number = offending_host_block_number - 1;

    info!(
        host_chain_id,
        offending_host_block_number, to_block_number, "Starting DB state revert"
    );

    let sql = prepare_revert_sql(host_chain_id, to_block_number);
    sqlx::raw_sql(&sql).execute(pool).await?;

    info!(host_chain_id, to_block_number, "DB state revert completed");
    Ok(())
}

/// Trait for re-exec behavior, allowing tests to substitute a mock.
pub trait ReExec: Send + Sync {
    fn re_exec(&self);
}

pub struct ProcessReExec;

impl ProcessReExec {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessReExec {
    fn default() -> Self {
        Self::new()
    }
}

impl ReExec for ProcessReExec {
    fn re_exec(&self) {
        use std::os::unix::process::CommandExt;

        let exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(e) => {
                error!(error = %e, "Failed to resolve executable path for re-exec, exiting");
                std::process::exit(1);
            }
        };
        let args: Vec<String> = std::env::args().collect();

        info!(?exe, ?args, "Re-execing process for drift recovery");

        let err = std::process::Command::new(&exe).args(&args[1..]).exec();

        // exec() only returns on error — process may be in a broken state, exit immediately.
        error!(error = %err, "re-exec failed, exiting");
        std::process::exit(1);
    }
}

pub struct RevertRunnerConfig {
    /// How long to wait after detecting a pending signal at startup before
    /// running the revert SQL. Gives other services time to also re-exec.
    pub grace_period: Duration,
    /// Maximum number of successful reverts allowed for a host chain within
    /// `recent_attempts_window`. Once exceeded, the next signal is marked
    /// `Failed` with reason "too many recent attempts" instead of running
    /// another (likely futile) revert. Catches deterministic recovery loops
    /// where each revert succeeds but the underlying drift recurs.
    pub max_recent_attempts: u32,
    /// Time window over which `max_recent_attempts` is counted.
    pub recent_attempts_window: Duration,
}

/// Watch for a pending drift-revert signal during normal service operation.
/// When a signal is detected → re-exec immediately. The re-execed process is
/// responsible for handling the revert (via `handle_pending_signal_on_startup`).
///
/// Services run this alongside their main loop. Exits cleanly if the cancel
/// token fires (e.g., SIGTERM). On success, this never returns (re-exec
/// replaces the process).
pub async fn run_signal_watcher(
    pool: &Pool<Postgres>,
    cancel_token: CancellationToken,
    re_exec_fn: &dyn ReExec,
    timeouts: WatcherTimeouts,
) {
    let signal = tokio::select! {
        _ = cancel_token.cancelled() => return,
        s = wait_for_in_flight_signal(pool, timeouts) => s,
    };

    info!(
        signal_id = signal.id,
        host_chain_id = signal.host_chain_id,
        offending_host_block_number = signal.offending_host_block_number,
        "Drift revert signal detected, re-execing"
    );

    // Never returns (exec replaces process, or exit on failure).
    re_exec_fn.re_exec();
}

/// Called on service startup BEFORE the main loop begins. If a pending signal
/// exists, handles it: the revert runner (gw-listener) waits a grace period
/// then runs the revert SQL and marks it done; waiters (other services) block
/// until the signal is done. Returns when it's safe for the service to
/// proceed with normal operation.
///
/// - `runner_cfg`: `Some` for the revert runner (gw-listener), `None` for all
///   other services (they just wait for the runner to finish).
/// - `cancel_token`: exits the wait early on shutdown.
///
/// Returns an error if the latest signal is `Failed` — the service must not
/// serve traffic on a DB where drift recovery failed. Operator must either
/// re-drive the signal (Failed → Pending) or acknowledge it (Failed → Done)
/// before the service can start.
pub async fn handle_pending_signal_on_startup(
    pool: &Pool<Postgres>,
    runner_cfg: Option<RevertRunnerConfig>,
    cancel_token: &CancellationToken,
) -> anyhow::Result<()> {
    if let Some(signal) = latest_failed_signal(pool).await? {
        let reason = match &signal.status {
            SignalStatus::Failed(r) => r.as_str(),
            _ => "unknown",
        };
        error!(
            signal_id = signal.id,
            host_chain_id = signal.host_chain_id,
            reason,
            "Refusing to start: an unresolved Failed drift revert signal exists — operator \
             must investigate (mark Failed → Pending to retry, or Failed → Done to acknowledge)"
        );
        anyhow::bail!("drift revert signal {} is Failed: {reason}", signal.id);
    }

    if let Some(cfg) = runner_cfg {
        run_all_pending_as_runner(pool, &cfg, cancel_token).await
    } else {
        wait_for_revert_done(pool, cancel_token).await
    }
}

/// Counts successful reverts (status = Done) for `host_chain_id` within the
/// recent `window`.
async fn count_recent_done_signals(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    window: Duration,
) -> anyhow::Result<i64> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM drift_revert_signal \
         WHERE host_chain_id = $1 \
           AND status = $2 \
           AND updated_at > NOW() - make_interval(secs => $3)",
    )
    .bind(host_chain_id)
    .bind(SignalStatus::Done.as_db_str())
    .bind(window.as_secs() as i64)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

/// Runner path: process all in-flight signals (oldest first) until none remain.
/// Multiple chains may have concurrent drifts; each gets its own revert run.
async fn run_all_pending_as_runner(
    pool: &Pool<Postgres>,
    cfg: &RevertRunnerConfig,
    cancel_token: &CancellationToken,
) -> anyhow::Result<()> {
    let mut waited_grace = false;
    while let Some(signal) = oldest_in_flight_signal(pool).await? {
        info!(
            signal_id = signal.id,
            host_chain_id = signal.host_chain_id,
            offending_host_block_number = signal.offending_host_block_number,
            status = signal.status.as_db_str(),
            "Found pending drift revert signal on startup"
        );

        // Refuse to retry if too many successful reverts already happened on
        // this chain in the recent window — drift keeps recurring, the revert
        // alone isn't fixing it. Mark Failed and bail; operator must investigate.
        let recent_dones =
            count_recent_done_signals(pool, signal.host_chain_id, cfg.recent_attempts_window)
                .await?;
        if recent_dones >= cfg.max_recent_attempts as i64 {
            let reason = format!(
                "too many recent attempts: {recent_dones} successful reverts on chain {} \
                 in the last {}s (threshold {})",
                signal.host_chain_id,
                cfg.recent_attempts_window.as_secs(),
                cfg.max_recent_attempts,
            );
            error!(
                signal_id = signal.id,
                host_chain_id = signal.host_chain_id,
                reason = %reason,
                "Refusing to revert: too many recent attempts on this chain"
            );
            update_signal_status(pool, signal.id, &SignalStatus::Failed(reason.clone())).await?;
            TOO_MANY_ATTEMPTS_COUNTER
                .with_label_values(&[&signal.host_chain_id.to_string()])
                .inc();
            return Err(anyhow::anyhow!(reason));
        }

        // Grace period: give other services time to re-exec too.
        // Only wait it once per process startup — if we're on the 2nd signal,
        // services have already had time to re-exec during the first revert.
        if matches!(signal.status, SignalStatus::Pending) {
            if !waited_grace {
                info!(grace_period = ?cfg.grace_period, "Waiting grace period before revert");
                tokio::select! {
                    _ = cancel_token.cancelled() => return Ok(()),
                    _ = tokio::time::sleep(cfg.grace_period) => {}
                }
                waited_grace = true;
            }
            update_signal_status(pool, signal.id, &SignalStatus::Reverting).await?;
        }

        if let Err(e) = execute_revert(
            pool,
            signal.host_chain_id,
            signal.offending_host_block_number,
        )
        .await
        {
            REVERT_FAILURE_COUNTER
                .with_label_values(&[&signal.host_chain_id.to_string()])
                .inc();
            // Mark Failed and bail — we must not let the service start on a
            // DB where drift recovery failed. Operator intervention required
            // (mark Failed → Pending to retry, or Failed → Done to acknowledge).
            error!(
                error = %e,
                signal_id = signal.id,
                host_chain_id = signal.host_chain_id,
                "Drift revert failed"
            );
            update_signal_status(pool, signal.id, &SignalStatus::Failed(e.to_string())).await?;
            return Err(e);
        }

        // Test-only hook: keep status=reverting for a few seconds so E2E
        // tests can observe the DB state after the revert ran but before
        // services resume. Production leaves the env var unset.
        if let Ok(secs) = std::env::var("DRIFT_REVERT_TEST_HOLD_SECS") {
            if let Ok(secs) = secs.parse::<u64>() {
                if secs > 0 {
                    info!(
                        hold_secs = secs,
                        "Holding reverting status for test observation"
                    );
                    tokio::time::sleep(Duration::from_secs(secs)).await;
                }
            }
        }

        update_signal_status(pool, signal.id, &SignalStatus::Done).await?;
        REVERT_SUCCESS_COUNTER
            .with_label_values(&[&signal.host_chain_id.to_string()])
            .inc();
        info!(signal_id = signal.id, "Drift revert complete");
    }
    Ok(())
}

/// Initialize drift-revert handling for a service. Call once at startup,
/// before the main loop begins.
///
/// 1. If a pending signal exists, handles it (runner runs the revert,
///    waiters block until done).
/// 2. Spawns a background watcher that polls for future signals and
///    re-execs the process when one appears.
///
/// Pass `Some(RevertRunnerConfig)` for the revert runner (e.g. gw-listener),
/// `None` for all other services. `timeouts` controls the watcher's
/// fail-fast thresholds — production should pass `WatcherTimeouts::default()`;
/// heavy integration tests override with relaxed values.
pub async fn init(
    pool: Pool<Postgres>,
    cancel_token: CancellationToken,
    runner_cfg: Option<RevertRunnerConfig>,
    timeouts: WatcherTimeouts,
) -> anyhow::Result<()> {
    init_with_reexec(
        pool,
        cancel_token,
        runner_cfg,
        ProcessReExec::new(),
        timeouts,
    )
    .await
}

/// Like [`init`] but lets the caller inject a custom `ReExec` implementation.
/// Primarily used by tests to swap in a mock.
pub async fn init_with_reexec<R: ReExec + 'static>(
    pool: Pool<Postgres>,
    cancel_token: CancellationToken,
    runner_cfg: Option<RevertRunnerConfig>,
    re_exec: R,
    timeouts: WatcherTimeouts,
) -> anyhow::Result<()> {
    handle_pending_signal_on_startup(&pool, runner_cfg, &cancel_token).await?;

    tokio::spawn(async move {
        run_signal_watcher(&pool, cancel_token, &re_exec, timeouts).await;
    });

    Ok(())
}
