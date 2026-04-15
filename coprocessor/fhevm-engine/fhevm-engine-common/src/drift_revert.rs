//! Auto drift recovery: shared coordination logic.
//!
//! When the gw-listener detects ciphertext drift, it writes a row to
//! `drift_revert_signal`. All coprocessor services poll that table and
//! re-exec themselves when a signal appears. On startup, all fresh
//! processes check for a pending signal:
//!  * gw-listener runs the revert SQL,
//!  * other services wait until it's done, then all proceed normally

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

const REVERT_SQL_TEMPLATE: &str =
    include_str!("../../db-migration/db-scripts/revert_coprocessor_db_state.sql");

/// How often services poll `drift_revert_signal` for state changes.
pub const POLL_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalStatus {
    Pending,
    Reverting,
    Done,
    Failed(String),
}

impl SignalStatus {
    fn as_db_str(&self) -> String {
        match self {
            Self::Pending => "pending".to_owned(),
            Self::Reverting => "reverting".to_owned(),
            Self::Done => "done".to_owned(),
            Self::Failed(reason) => format!("failed: {reason}"),
        }
    }

    fn from_db_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "reverting" => Self::Reverting,
            "done" => Self::Done,
            other => Self::Failed(other.strip_prefix("failed: ").unwrap_or(other).to_owned()),
        }
    }

    pub fn is_in_flight(&self) -> bool {
        matches!(self, Self::Pending | Self::Reverting)
    }
}

#[derive(Debug, Clone)]
pub struct DriftRevertSignal {
    pub id: i64,
    pub host_chain_id: i64,
    pub offending_host_block_number: i64,
    pub status: SignalStatus,
}

/// Create a new drift revert signal.
/// Returns `Some(id)` if created, `None` if there's already an in-flight
/// revert for this host chain.
pub async fn create_revert_signal(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    offending_host_block_number: i64,
) -> anyhow::Result<Option<i64>> {
    let row = sqlx::query(
        "INSERT INTO drift_revert_signal (host_chain_id, offending_host_block_number, status) \
         SELECT $1, $2, $3 \
         WHERE NOT EXISTS ( \
             SELECT 1 FROM drift_revert_signal \
             WHERE host_chain_id = $1 AND (status = $3 OR status = $4) \
         ) \
         RETURNING id",
    )
    .bind(host_chain_id)
    .bind(offending_host_block_number)
    .bind(SignalStatus::Pending.as_db_str())
    .bind(SignalStatus::Reverting.as_db_str())
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => {
            let id: i64 = r.get("id");
            Ok(Some(id))
        }
        None => Ok(None),
    }
}

/// Fetch the latest signal row, if any.
pub async fn latest_signal(pool: &Pool<Postgres>) -> anyhow::Result<Option<DriftRevertSignal>> {
    let row = sqlx::query(
        "SELECT id, host_chain_id, offending_host_block_number, status \
         FROM drift_revert_signal ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let status_str: String = r.get("status");
        DriftRevertSignal {
            id: r.get("id"),
            host_chain_id: r.get("host_chain_id"),
            offending_host_block_number: r.get("offending_host_block_number"),
            status: SignalStatus::from_db_str(&status_str),
        }
    }))
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
        "SELECT block_number FROM computations \
         WHERE output_handle = $1 AND host_chain_id = $2 \
         LIMIT 1",
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
            info!(
                host_chain_id,
                block, id, "Drift revert signal created successfully"
            );
        }
        Ok(None) => {
            warn!(
                host_chain_id,
                block, "Drift revert signal already in flight, skipping creation"
            );
        }
        Err(e) => {
            error!(error = %e, "Failed to create drift revert signal");
        }
    }
}

/// Poll `drift_revert_signal` until a signal is in flight (status Pending or
/// Reverting). Used by `run_signal_watcher` in non-revert-runner services to
/// detect that a drift revert is happening so they can re-exec.
pub async fn wait_for_in_flight_signal(pool: &Pool<Postgres>) -> anyhow::Result<DriftRevertSignal> {
    loop {
        if let Some(signal) = latest_signal(pool).await? {
            if signal.status.is_in_flight() {
                return Ok(signal);
            }
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Poll until the latest signal reaches a terminal status.
/// Used by non-gw-listener services to wait for the revert to finish.
/// On `Failed`, logs the reason and returns `Ok(())` — the service continues
/// startup despite a failed revert (operator can investigate via logs).
pub async fn wait_for_revert_done(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    loop {
        if let Some(signal) = latest_signal(pool).await? {
            match &signal.status {
                SignalStatus::Done => return Ok(()),
                SignalStatus::Failed(reason) => {
                    error!(
                        signal_id = signal.id,
                        host_chain_id = signal.host_chain_id,
                        reason = %reason,
                        "Drift revert failed; continuing service startup regardless"
                    );
                    return Ok(());
                }
                _ => {}
            }
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
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
/// `to_block = max(1, offending_host_block_number - 1)` — we subtract 1
/// because the script reverts everything strictly greater than `to_block`,
/// and we want the offending block itself gone. If ciphertexts from earlier
/// blocks also drifted (due to out-of-order processing), the drift detector
/// will catch them eventually in subsequent rounds.
pub async fn execute_revert(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    offending_host_block_number: i64,
) -> anyhow::Result<()> {
    let to_block_number = (offending_host_block_number - 1).max(1);

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
) -> anyhow::Result<()> {
    let signal = tokio::select! {
        _ = cancel_token.cancelled() => return Ok(()),
        r = wait_for_in_flight_signal(pool) => r?,
    };

    info!(
        signal_id = signal.id,
        host_chain_id = signal.host_chain_id,
        offending_host_block_number = signal.offending_host_block_number,
        "Drift revert signal detected, re-execing"
    );

    // Never returns (exec replaces process, or exit on failure).
    re_exec_fn.re_exec();

    Ok(())
}

/// Called on service startup BEFORE the main loop begins. If a pending signal
/// exists, handles it: the revert runner (gw-listener) waits a grace period
/// then runs the revert SQL and marks it done; waiters (other services) block
/// until the signal is done. Returns when it's safe for the service to
/// proceed with normal operation.
///
/// - `runner_cfg`: `Some` for the revert runner (gw-listener), `None` for all
///   other services (they just wait for the runner to finish).
pub async fn handle_pending_signal_on_startup(
    pool: &Pool<Postgres>,
    runner_cfg: Option<RevertRunnerConfig>,
) -> anyhow::Result<()> {
    let Some(signal) = latest_signal(pool).await? else {
        return Ok(());
    };

    match &signal.status {
        SignalStatus::Done => Ok(()),
        SignalStatus::Pending | SignalStatus::Reverting => {
            let is_revert_runner = runner_cfg.is_some();
            info!(
                signal_id = signal.id,
                host_chain_id = signal.host_chain_id,
                offending_host_block_number = signal.offending_host_block_number,
                is_revert_runner,
                status = signal.status.as_db_str(),
                "Found pending drift revert signal on startup"
            );

            if let Some(cfg) = runner_cfg {
                // Grace period: give other services time to re-exec too.
                info!(grace_period = ?cfg.grace_period, "Waiting grace period before revert");
                tokio::time::sleep(cfg.grace_period).await;

                update_signal_status(pool, signal.id, &SignalStatus::Reverting).await?;

                if let Err(e) = execute_revert(
                    pool,
                    signal.host_chain_id,
                    signal.offending_host_block_number,
                )
                .await
                {
                    error!(error = %e, "Drift revert failed");
                    update_signal_status(pool, signal.id, &SignalStatus::Failed(e.to_string()))
                        .await?;
                    return Err(e);
                }

                // Test-only hook: keep status=reverting for a few seconds so
                // E2E tests can observe the DB state after the revert ran but
                // before services resume. Production leaves the env var unset.
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
                info!("Drift revert complete, resuming normal operation");
            } else {
                wait_for_revert_done(pool).await?;
                info!("Drift revert complete, resuming normal operation");
            }

            Ok(())
        }
        SignalStatus::Failed(reason) => {
            // A previous auto-revert attempt failed. The DB may still contain
            // drifted data — operator intervention is required (manual revert
            // or other remediation). We proceed normally so the service stays
            // up, but flag the condition prominently.
            error!(
                signal_id = signal.id,
                host_chain_id = signal.host_chain_id,
                reason = reason,
                "Latest drift revert signal has failed status — manual intervention required, proceeding anyway"
            );
            Ok(())
        }
    }
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
/// `None` for all other services.
pub async fn init(
    database_url: &str,
    cancel_token: CancellationToken,
    runner_cfg: Option<RevertRunnerConfig>,
) -> anyhow::Result<()> {
    init_with_reexec(database_url, cancel_token, runner_cfg, ProcessReExec::new()).await
}

/// Like [`init`] but lets the caller inject a custom `ReExec` implementation.
/// Primarily used by tests to swap in a mock.
pub async fn init_with_reexec<R: ReExec + 'static>(
    database_url: &str,
    cancel_token: CancellationToken,
    runner_cfg: Option<RevertRunnerConfig>,
    re_exec: R,
) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await?;

    handle_pending_signal_on_startup(&pool, runner_cfg).await?;

    tokio::spawn(async move {
        if let Err(e) = run_signal_watcher(&pool, cancel_token, &re_exec).await {
            error!(error = %e, "Drift-revert signal watcher failed");
        }
    });

    Ok(())
}
