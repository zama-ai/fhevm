//! Upgrade Controller (`upgrade-controller`) — drives the upgrade FSM in Postgres.
//!
//! Listens for `upgrade_activated` and `unanimity_consensus` notifications via
//! `pg_notify` and mutates rows in the `upgrade_state` table accordingly. The
//! `unanimity_consensus` channel is produced by `consensus-detector` once every
//! operator publishes the same state commitment at the upgrade's `end_block`.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
use fhevm_engine_common::gcs_activation::{EVENT_DRY_RUN_ROLLED_BACK, WORK_AVAILABLE_CHANNEL};
use fhevm_engine_common::synthetic_input::SYNTHETIC_ZK_PROOF_ID_BASE;
use fhevm_engine_common::utils::DatabaseURL;
use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard};
use serde::Deserialize;
use sqlx::{postgres::PgListener, Pool, Postgres, Transaction};
use thiserror::Error;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn, Level};

mod coprocessor_tables;
pub use coprocessor_tables::{
    CoprocessorTable, CIPHERTEXT_TABLES, COMPUTATION_TABLES, COPROCESSOR_TABLES,
    PBS_COMPUTATION_TABLES,
};

pub const UPGRADE_ACTIVATED_CHANNEL: &str =
    fhevm_engine_common::gcs_activation::EVENT_UPGRADE_ACTIVATED;
/// Must stay in sync with `consensus_detector::UNANIMITY_CONSENSUS_CHANNEL`.
pub const UNANIMITY_CONSENSUS_CHANNEL: &str = "event_unanimity_consensus";
/// Sent when a dry-run never reached agreement; triggers the rollback.
pub const UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL: &str = "event_unanimity_consensus_timeout";
/// Re-triggers the GCS dry-run readiness check. Must stay in sync with the
/// names emitted by `host-listener::ingest_block_logs` and the FHE workers.
pub const NEW_BLOCK_CHANNEL: &str = "event_new_block";
pub const EVENT_CIPHERTEXT_COMPUTED_CHANNEL: &str = "event_ciphertext_computed";
/// Emitted by `transition_to_dry_run_started` once the GCS row enters
/// `DryRunStarted`, unpausing the GCS-fleet workers. Single-sourced from the
/// common crate, which the workers also use.
pub const DRY_RUN_STARTED_CHANNEL: &str =
    fhevm_engine_common::gcs_activation::EVENT_DRY_RUN_STARTED;

/// Emitted by `gw-listener` on each ingested Gateway block; wakes the
/// Gateway-side readiness loop. Single-sourced from the common crate.
pub const GW_NEW_BLOCK_CHANNEL: &str = fhevm_engine_common::gcs_activation::EVENT_GW_NEW_BLOCK;

/// Emitted by `transition_to_gw_dry_run_started` once the GCS gw-listener has
/// reached `gw_start_block` and pre-start rows have been pruned from
/// `gcs.verify_proofs`, releasing the GCS `zkproof-worker`. Single-sourced from
/// the common crate, which the worker's activation watcher also uses.
pub const GW_DRY_RUN_STARTED_CHANNEL: &str =
    fhevm_engine_common::gcs_activation::EVENT_GW_DRY_RUN_STARTED;

/// Channel emitted by `execute_cutover`, atomically with the `versioning`
/// bump, telling every service to re-evaluate its mode. Re-exported from the
/// common crate so services and the controller agree on the name.
pub use fhevm_engine_common::versioning::EVENT_STACK_VERSION_UPGRADED;

/// Number of host-chain blocks below `start_block` whose computations must
/// also be fully settled before GCS can leave `UpgradeActivated`. Hard-coded
/// for now; expected to become configurable.
const READINESS_CONFIRMATIONS: i64 = 100;
const NO_READINESS_ATTEMPT: i64 = -2;

/// Retry budget and backoff for [`execute_cutover`]. Cutover is the step that promotes
/// the green stack, so a *transient* failure must not drop the upgrade on the floor:
/// the whole thing is one transaction, so a failure rolls back cleanly and leaves the
/// GCS rows in `UpgradeAuthorized`, i.e. safe to run again. Realistic transients are a
/// deadlock abort against a writer that took the digest locks in the opposite order, a
/// dropped connection, and a serialization failure.
///
/// The budget is bounded on purpose. A *permanent* failure (say a merge whose column
/// list no longer matches the `gcs.*` snapshot) would otherwise spin against the
/// database forever. Giving up here costs no liveness: the rows stay
/// `UpgradeAuthorized`, so `reconcile` re-enters on the next `poll_interval` tick and
/// the retry cycle starts over, indefinitely but at a calm cadence.
///
/// 10 attempts, backoff 0.5, 1, 2, 4, 8, 16, 32, 60, 60 (seconds): about 3 minutes of
/// retrying, then `reconcile`'s 30s poll tick keeps retrying forever.
const CUTOVER_RETRY_ATTEMPTS: u32 = 10;
const CUTOVER_RETRY_BASE_DELAY: Duration = Duration::from_millis(500);
const CUTOVER_RETRY_MAX_DELAY: Duration = Duration::from_secs(60);

struct GcsReadinessAttempt {
    proposal_id: Vec<u8>,
    proposal_block: i64,
    /// (chain_id, start_block) for every host chain in the proposal.
    host_chains: Vec<(i64, i64)>,
    gw_start_block: i64,
}

/// PostgreSQL advisory-lock key used to serialize controller changes against
/// writes. Cutover and rollback take the exclusive form; write transactions take
/// the shared form through [`fhevm_engine_common::versioning::begin_write_guarded`].
/// Re-exported from the common crate so the controller and all writers agree on
/// one canonical value.
pub use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;

#[derive(Debug, Clone)]
pub struct Config {
    pub service_name: String,
    pub database_url: DatabaseURL,
    pub database_pool_size: u32,
    /// When true, the service operates as the Green Coprocessor Stack (GCS) —
    /// it gates `execute_cutover` and runs the GCS-side dry-run readiness loop.
    /// When false, it operates as the Blue Coprocessor Stack (BCS).
    /// Auto-detected at startup from the `versioning` table, like the other
    /// coprocessor services (see `fhevm_engine_common::versioning::resolve_gcs_mode`).
    pub gcs_mode: bool,
    pub log_level: Level,
    /// Fallback poll interval used while waiting for notifications, so a missed
    /// NOTIFY (e.g. dropped connection) still gets re-checked eventually.
    pub poll_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            service_name: "upgrade-controller".to_owned(),
            database_url: DatabaseURL::default(),
            database_pool_size: 4,
            gcs_mode: false,
            log_level: Level::INFO,
            poll_interval: Duration::from_secs(30),
        }
    }
}

/// Payload published over `upgrade_activated`.
///
/// Fields are decoded from JSON; the producing component (governance / gw
/// indexer) is responsible for emitting them in this shape via `pg_notify`.
#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeActivatedPayload {
    /// Hex-encoded bytes (e.g. "0xabcd..."). Stored as BYTEA in `upgrade_state`.
    pub proposal_id: String,
    /// Host chain this activation belongs to. Required so the GCS-side
    /// readiness loop can scope queries (and the unpause notify) to one chain.
    pub chain_id: i64,
    pub start_block: i64,
    pub end_block: i64,
    pub gw_start_block: i64,
    /// Optional — included for forward-compat with the schema's `version` column.
    #[serde(default)]
    pub version: Option<String>,
}

/// Payload published over `unanimity_consensus` by `consensus-detector`.
///
/// `proposal_id` identifies the active upgrade.
#[derive(Debug, Clone, Deserialize)]
pub struct UnanimityConsensusPayload {
    pub proposal_id: Vec<u8>,
    /// The block containing the ProtocolConfig proposal. Optional only so a
    /// rolling deployment can safely ignore notifications from an older
    /// detector that did not yet carry attempt identity.
    #[serde(default)]
    pub proposal_block: Option<i64>,
    pub chain_id: i64,
    pub block_height: i64,
    pub block_hash: String,
}

/// Payload published over `event_new_block` by `host-listener::ingest_block_logs`.
///
/// JSON shape must stay in sync with that producer (and
/// `consensus_detector::NewBlockPayload`). Only `block_height` is used here, to
/// log the block that re-triggered the readiness check.
#[derive(Debug, Clone, Deserialize)]
pub struct NewBlockPayload {
    pub chain_id: i64,
    pub block_height: i64,
    pub block_hash: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("invalid notification payload: {0}")]
    Payload(String),

    #[error("invalid hex in proposal_id: {0}")]
    Hex(String),

    /// Blue has not finished uploading its pre-window ciphertexts to S3 yet, so cutover
    /// must not retire it.
    ///
    /// **Transient by nature**: blue's own sns-worker clears it, so the retry loop and
    /// `reconcile`'s poll tick keep re-checking until it does. The per-chain pending counts
    /// are warned by `assert_bcs_ciphertexts_uploaded` on every attempt;
    /// `Error::is_transient` marks the variant so callers can tell "not yet" apart from a
    /// real failure.
    #[error("{pending} pre-start_block BCS ciphertext(s) are still awaiting S3 upload")]
    PendingBcsUploads { pending: i64 },
}

impl Error {
    /// True for conditions that clear on their own, where retrying is the correct response
    /// and nothing has actually gone wrong.
    fn is_transient(&self) -> bool {
        matches!(self, Error::PendingBcsUploads { .. })
    }
}

/// Handle an `event_upgrade_activated` notification. The host-listener writes the
/// `upgrade_state` row in the same transaction that decodes `CoprocessorUpgradeProposed`,
/// so this notification is only a wake-up: drive the FSM from the persisted row
/// (via `reconcile`), not from the payload. A missed notification is recovered by the
/// boot/poll-tick reconcile in `run`.
pub async fn handle_upgrade_activated(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    readiness: &Arc<AtomicI64>,
    gcs_mode: bool,
    raw_payload: &str,
) -> Result<(), Error> {
    let payload: UpgradeActivatedPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    info!(
        gcs_mode,
        proposal_id = %payload.proposal_id,
        "event_upgrade_activated received — reconciling from persisted upgrade_state row"
    );

    reconcile(pool, cancel, readiness, gcs_mode).await
}

fn spawn_gcs_dry_run_readiness(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    readiness: &Arc<AtomicI64>,
    attempt: GcsReadinessAttempt,
) {
    let GcsReadinessAttempt {
        proposal_id,
        proposal_block,
        host_chains,
        gw_start_block,
    } = attempt;

    let mut active = readiness.load(Ordering::SeqCst);
    loop {
        if active >= proposal_block {
            return;
        }
        match readiness.compare_exchange(active, proposal_block, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(_) => break,
            Err(next) => active = next,
        }
    }
    info!(
        chains = host_chains.len(),
        gw_start_block, "arming GCS dry-run readiness"
    );
    let pool = pool.clone();
    let gw_cancel = cancel.child_token();
    let host_cancel = cancel.child_token();
    let readiness = readiness.clone();
    tokio::spawn(async move {
        // Gateway gate for the zkproof-worker: gw_start_block is a Gateway block,
        // a separate clock from the host start_block.
        let gw_gate = async {
            match wait_until_gw_dry_run_ready(
                pool.clone(),
                gw_cancel,
                &proposal_id,
                proposal_block,
                gw_start_block,
            )
            .await
            {
                Ok(true) => {
                    match prune_gcs_verify_proofs_before_start(
                        &pool,
                        &proposal_id,
                        proposal_block,
                        gw_start_block,
                    )
                    .await
                    {
                        Ok(deleted) => info!(
                            gw_start_block,
                            deleted, "pruned pre-gw_start_block rows from gcs.verify_proofs"
                        ),
                        Err(e) => {
                            error!(error = %e, "failed to prune gcs.verify_proofs; skipping gw release");
                            return;
                        }
                    }
                    if let Err(e) =
                        transition_to_gw_dry_run_started(&pool, &proposal_id, proposal_block).await
                    {
                        error!(error = %e, "failed to transition GCS gw_dry_run_started");
                    }
                }
                Ok(false) => info!(
                    gw_start_block,
                    "gw readiness loop exited without satisfying readiness — skipping prune and release"
                ),
                Err(e) => error!(error = %e, "GCS gateway dry-run readiness loop failed"),
            }
        };
        // Host gate: every chain must settle to its start_block and be pruned
        // before flipping the whole proposal to DryRunStarted.
        let host_gate = async {
            for &(chain_id, start_block) in &host_chains {
                match wait_until_dry_run_ready(
                    pool.clone(),
                    host_cancel.child_token(),
                    &proposal_id,
                    proposal_block,
                    chain_id,
                    start_block,
                )
                .await
                {
                    Ok(true) => match prune_gcs_computations_before_start(
                        &pool,
                        &proposal_id,
                        proposal_block,
                        chain_id,
                        start_block,
                    )
                    .await
                    {
                        Ok(deleted) => info!(
                            chain_id,
                            start_block,
                            deleted,
                            "pruned pre-start_block rows from gcs.computations"
                        ),
                        Err(e) => {
                            error!(chain_id, start_block, error = %e, "failed to prune gcs.computations; skipping transition");
                            return;
                        }
                    },
                    Ok(false) => {
                        info!(
                            chain_id,
                            start_block,
                            "readiness loop exited without satisfying readiness — skipping transition"
                        );
                        return;
                    }
                    Err(e) => {
                        error!(chain_id, start_block, error = %e, "GCS dry-run readiness loop failed");
                        return;
                    }
                }
            }
            if let Err(e) = transition_to_dry_run_started(&pool, &proposal_id, proposal_block).await
            {
                error!(error = %e, "failed to transition GCS to DryRunStarted");
            }
        };
        tokio::join!(gw_gate, host_gate);
        let _ = readiness.compare_exchange(
            proposal_block,
            NO_READINESS_ATTEMPT,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
    });
}

/// Create the GCS schema with an empty copy of each duplicated table. Returns
/// their names.
/// Notify triggers recreated inside the GCS schema, as `(table, trigger, trigger function)`.
///
/// `CREATE TABLE ... (LIKE public.X INCLUDING ALL)` copies defaults, constraints, indexes and
/// storage - but *not* triggers. Without these the GCS clones are silent: the host-listener runs
/// the same unqualified `INSERT INTO computations`, it lands in `gcs.computations` via
/// `search_path`, and no `work_available` fires. The tfhe- and sns-workers then only ever wake on
/// their polling interval, which was observed as a 60s stall between the host-listener ingesting
/// synthetic ops and the tfhe-worker acquiring the dependence chain.
///
/// Only the legacy tables carry a trigger. The host-listener writes the legacy and `_branch`
/// forms in the same transaction, so one notification per channel already wakes the worker, and
/// a second trigger would only add duplicate wake-ups.
///
/// The functions themselves live in `public` and are shared, so they are referenced
/// schema-qualified rather than relying on the connection's `search_path`.
const GCS_NOTIFY_TRIGGERS: &[(&str, &str, &str)] = &[
    // NOTIFY work_available -> tfhe-worker
    (
        "computations",
        "work_updated_trigger_from_computations_insertions",
        "public.notify_work_available",
    ),
    // NOTIFY event_pbs_computations -> sns-worker
    (
        "pbs_computations",
        "on_insert_notify_event_pbs_computations",
        "public.notify_event_pbs_computations",
    ),
];

async fn create_gcs_tables(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<&'static str>, Error> {
    let create_schema = format!("CREATE SCHEMA IF NOT EXISTS {GCS_SCHEMA_QUOTED}");
    sqlx::query(&create_schema).execute(&mut **tx).await?;

    let duplicated: Vec<&str> = COPROCESSOR_TABLES
        .iter()
        .filter(|t| t.duplicated)
        .map(|t| t.name)
        .collect();

    for name in &duplicated {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {GCS_SCHEMA_QUOTED}.{name} \
             (LIKE public.{name} INCLUDING ALL)"
        );
        sqlx::query(&sql).execute(&mut **tx).await?;
    }

    // Restore the notify triggers `INCLUDING ALL` left behind, so a GCS insert wakes the
    // workers exactly as the same insert does in `public`.
    for (table, trigger, function) in GCS_NOTIFY_TRIGGERS {
        // Postgres has no `CREATE TRIGGER IF NOT EXISTS`, and this function must stay
        // idempotent (it runs on every activation), so drop first.
        let drop_sql = format!("DROP TRIGGER IF EXISTS {trigger} ON {GCS_SCHEMA_QUOTED}.{table}");
        sqlx::query(&drop_sql).execute(&mut **tx).await?;

        let create_sql = format!(
            "CREATE TRIGGER {trigger} AFTER INSERT ON {GCS_SCHEMA_QUOTED}.{table} \
             FOR EACH STATEMENT EXECUTE FUNCTION {function}()"
        );
        sqlx::query(&create_sql).execute(&mut **tx).await?;
        info!(
            schema = GCS_SCHEMA_QUOTED,
            table, trigger, "created GCS notify trigger"
        );
    }

    Ok(duplicated)
}

/// Idempotently create the versioned GCS schema ([`GCS_SCHEMA_QUOTED`]) and clone every
/// `duplicated = true` [`COPROCESSOR_TABLES`] table into it with `LIKE public.X INCLUDING ALL`.
pub async fn create_gcs_schema(pool: &Pool<Postgres>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    let duplicated = create_gcs_tables(&mut tx).await?;
    tx.commit().await?;
    info!(
        schema = GCS_SCHEMA_QUOTED,
        tables = ?duplicated,
        "GCS schema created with empty table duplicates"
    );
    Ok(())
}

/// Drop the GCS schema and recreate it empty, on `tx` — discards the dry-run's
/// writes while leaving the still-tailing GCS listeners a valid write target, so
/// the upgrade can be rerun without restarting the GCS stack.
async fn reset_gcs_schema(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    let drop_sql = format!("DROP SCHEMA IF EXISTS {GCS_SCHEMA_QUOTED} CASCADE");
    sqlx::query(&drop_sql).execute(&mut **tx).await?;
    let duplicated = create_gcs_tables(tx).await?;
    info!(
        schema = GCS_SCHEMA_QUOTED,
        tables = ?duplicated,
        "GCS schema dropped and recreated empty (rollback)"
    );
    Ok(())
}

/// Delete from `gcs.computations` every row for `chain_id` whose `block_number`
/// is below `start_block`. The GCS stack starts tailing the chain before
/// activation, so its schema may hold computations for blocks that precede the
/// upgrade window; clearing them makes the dry-run snapshot start cleanly at
/// `start_block`. Rows with a NULL `block_number` (not yet bound to a block) are
/// left untouched. Returns the number of rows removed. Idempotent.
async fn prune_gcs_computations_before_start(
    pool: &Pool<Postgres>,
    proposal_id: &[u8],
    proposal_block: i64,
    chain_id: i64,
    start_block: i64,
) -> Result<u64, Error> {
    let WriteGuard::Proceed(mut tx) =
        begin_write_guarded(pool, true, GcsRollbackPolicy::Skip).await?
    else {
        return Ok(0);
    };

    let sql = format!(
        "WITH current_attempt AS (
             SELECT 1 FROM upgrade_state
              WHERE stack_role = 'GCS'
                AND state = 'UpgradeActivated'
                AND proposal_id = $3
                AND COALESCE(proposal_block, -1) = $4
              FOR SHARE
         )
         DELETE FROM {GCS_SCHEMA_QUOTED}.computations \
         WHERE host_chain_id = $1 \
           AND block_number IS NOT NULL \
           AND block_number < $2 \
           AND EXISTS (SELECT 1 FROM current_attempt)"
    );
    let result = sqlx::query(&sql)
        .bind(chain_id)
        .bind(start_block)
        .bind(proposal_id)
        .bind(proposal_block)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    info!(
        chain_id,
        start_block,
        deleted = result.rows_affected(),
        "pruned pre-start_block rows from gcs.computations"
    );

    Ok(result.rows_affected())
}

/// True iff for every block in `[start_block - READINESS_CONFIRMATIONS, start_block]`
/// on the given chain, either `fhe_event_count = 0` (block had no FHE events)
/// or every computation in that block has `is_completed = true` AND
/// `is_error = false`. An errored computation in the window blocks readiness.
///
/// Also requires the BCS host-listener to have reached at least `start_block`
/// (via `MAX(block_number)` in `host_chain_blocks_valid`) — otherwise the
/// predicate would be vacuously true for un-ingested blocks above the watermark.
async fn check_dry_run_ready(
    pool: &Pool<Postgres>,
    chain_id: i64,
    start_block: i64,
) -> Result<bool, sqlx::Error> {
    let from_block = start_block.saturating_sub(READINESS_CONFIRMATIONS);
    let (ready,): (bool,) = sqlx::query_as(
        r#"
        SELECT
          COALESCE(
            (SELECT MAX(block_number) FROM public.host_chain_blocks_valid WHERE chain_id = $1),
            -1
          ) >= $3
          AND NOT EXISTS (
              SELECT 1 FROM public.host_chain_blocks_valid hcbv
              WHERE hcbv.chain_id = $1
                AND hcbv.block_number BETWEEN $2 AND $3
                AND hcbv.fhe_event_count > 0
                AND EXISTS (
                    SELECT 1 FROM public.computations c
                    WHERE c.host_chain_id = $1
                      AND c.block_number = hcbv.block_number
                      AND (c.is_completed = false OR c.is_error = true)
                )
          )
        "#,
    )
    .bind(chain_id)
    .bind(from_block)
    .bind(start_block)
    .fetch_one(pool)
    .await?;
    Ok(ready)
}

/// GCS-only loop. Polls `check_dry_run_ready`, re-triggered by every
/// `event_new_block` and `event_ciphertext_computed` notification.
///
/// Returns `Ok(true)` once readiness is satisfied — the caller then prunes the
/// GCS snapshot and performs the `DryRunStarted` transition (the internal
/// "upgrade activated" spawn). Returns `Ok(false)` if it exits for any other
/// reason: cancellation, or another path having already moved the GCS row out
/// of `UpgradeActivated`. In the `false` case the caller skips pruning and the
/// transition.
async fn wait_until_dry_run_ready(
    pool: Pool<Postgres>,
    cancel: CancellationToken,
    proposal_id: &[u8],
    proposal_block: i64,
    chain_id: i64,
    start_block: i64,
) -> Result<bool, Error> {
    let from_block = start_block.saturating_sub(READINESS_CONFIRMATIONS);
    info!(
        chain_id,
        from_block,
        start_block,
        confirmations = READINESS_CONFIRMATIONS,
        "Starting GCS dry-run readiness loop"
    );

    // Dedicated listener so this loop is decoupled from the main run() listener.
    let mut listener = PgListener::connect_with(&pool).await?;
    listener
        .listen_all([
            NEW_BLOCK_CHANNEL,
            EVENT_CIPHERTEXT_COMPUTED_CHANNEL,
            EVENT_DRY_RUN_ROLLED_BACK,
        ])
        .await?;

    loop {
        if cancel.is_cancelled() {
            info!("readiness loop cancelled");
            return Ok(false);
        }

        let current_state: Option<(String,)> = sqlx::query_as(
            "SELECT state FROM upgrade_state
                  WHERE stack_role = 'GCS'
                    AND proposal_id = $1
                    AND COALESCE(proposal_block, -1) = $2
                  ORDER BY host_chain_id
                  LIMIT 1",
        )
        .bind(proposal_id)
        .bind(proposal_block)
        .fetch_optional(&pool)
        .await?;
        match current_state.as_ref().map(|(s,)| s.as_str()) {
            Some("UpgradeActivated") => {}
            Some(other) => {
                info!(
                    state = other,
                    "GCS state is not UpgradeActivated — readiness loop exiting"
                );
                return Ok(false);
            }
            None => {
                warn!("No GCS row in upgrade_state — readiness loop exiting");
                return Ok(false);
            }
        }

        match check_dry_run_ready(&pool, chain_id, start_block).await {
            Ok(true) => {
                info!(chain_id, start_block, "Dry-run readiness satisfied");
                return Ok(true);
            }
            Ok(false) => {
                debug!(
                    chain_id,
                    from_block,
                    start_block,
                    "Dry-run readiness not yet satisfied; waiting for next notification"
                );
            }
            Err(e) => {
                error!(error = %e, "Readiness check query failed; will retry on next notification");
            }
        }

        select! {
            _ = cancel.cancelled() => {
                info!("readiness loop cancelled");
                return Ok(false);
            }
            recv = listener.recv() => {
                match recv {
                    Ok(notification) => {
                        let block_height = if notification.channel() == NEW_BLOCK_CHANNEL {
                            match serde_json::from_str::<NewBlockPayload>(notification.payload()) {
                                Ok(payload) => Some(payload.block_height),
                                Err(e) => {
                                    warn!(
                                        channel = notification.channel(),
                                        payload = notification.payload(),
                                        error = %e,
                                        "failed to parse new_block payload"
                                    );
                                    None
                                }
                            }
                        } else {
                            None
                        };
                        info!(channel = notification.channel(), start_block = start_block, block_height, "readiness loop trigger");
                    }
                    Err(e) => {
                        warn!(error = %e, "readiness listener recv error; sleeping before retry");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}

async fn transition_to_dry_run_started(
    pool: &Pool<Postgres>,
    proposal_id: &[u8],
    proposal_block: i64,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
        UPDATE upgrade_state
        SET state = 'DryRunStarted', updated_at = NOW()
        WHERE stack_role = 'GCS'
          AND state = 'UpgradeActivated'
          AND proposal_id = $1
          AND COALESCE(proposal_block, -1) = $2
        "#,
    )
    .bind(proposal_id)
    .bind(proposal_block)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        warn!(
            "transition_to_dry_run_started: GCS not in UpgradeActivated for this proposal — skipping unpause notify"
        );
        return Ok(());
    }

    // Unpause the GCS-fleet workers, which stay parked until they observe the
    // GCS row in `DryRunStarted` (i.e. BCS has settled to start_block and
    // pre-start rows have been pruned). The payload is unused — each worker's
    // activation watcher re-reads upgrade_state on wake.
    //
    // `WORK_AVAILABLE_CHANNEL` goes out in the same statement, after the unpause, so the
    // tfhe-worker re-fetches pending uncomputed ops instead of idling until its next poll
    // tick. It is needed because `gcs.computations` carries no
    // `work_updated_trigger_from_computations_insertions`: the GCS tables are cloned with
    // `LIKE ... INCLUDING ALL`, which does not copy triggers. Everything queued while the
    // fleet was parked — the host-listener's synthetic ops at `start_block + 1`, plus any
    // real traffic in the window — therefore notified nobody on insert.
    //
    // Both in one statement purely to save a round trip; the relative order does not matter.
    //
    // This wake is BEST-EFFORT, not a guarantee. The worker's activation watcher and its work
    // loop are separate tasks on separate LISTEN connections, so this can land while the work
    // loop is still parked in its gated sleep. It survives that case (the loop never reads the
    // socket while gated, so the notification just queues), but it can still be missed if the
    // LISTEN connection resets in between. Nothing is lost when that happens - the
    // `computations` rows persist and the next poll tick picks them up - so the only cost of a
    // missed wake is one polling interval of latency.
    //
    // The durable fix is to give `gcs.computations` its own
    // `work_updated_trigger_from_computations_insertions`, so every insert notifies the way it
    // does in `public` and this explicit kick stops being load-bearing.
    sqlx::query("SELECT pg_notify($1, $3), pg_notify($2, $3)")
        .bind(DRY_RUN_STARTED_CHANNEL)
        .bind(WORK_AVAILABLE_CHANNEL)
        .bind("")
        .execute(pool)
        .await?;
    info!(
        work_available_channel = WORK_AVAILABLE_CHANNEL,
        "transition_to_dry_run_started: GCS now in DryRunStarted; unpause + work_available notifies sent"
    );

    Ok(())
}

/// True once the GCS gw-listener has reached `gw_start_block` — i.e.
/// `gcs."gw_listener_last_block".last_block_num >= gw_start_block`. Reads the
/// GCS schema's watermark explicitly (not `public`), since the green
/// gw-listener tails the Gateway into the GCS schema from startup. A missing
/// watermark row reads as `-1`, so the predicate is not vacuously true before
/// the GCS gw-listener has written any progress.
async fn check_gw_dry_run_ready(
    pool: &Pool<Postgres>,
    gw_start_block: i64,
) -> Result<bool, sqlx::Error> {
    let sql = format!(
        "SELECT COALESCE(
                  (SELECT last_block_num FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block
                   WHERE dummy_id = true),
                  -1
                ) >= $1"
    );
    let (ready,): (bool,) = sqlx::query_as(&sql)
        .bind(gw_start_block)
        .fetch_one(pool)
        .await?;
    Ok(ready)
}

/// GCS-only loop, the Gateway analogue of [`wait_until_dry_run_ready`]. Polls
/// [`check_gw_dry_run_ready`], re-triggered by every [`GW_NEW_BLOCK_CHANNEL`]
/// notification.
///
/// Returns `Ok(true)` once the GCS gw-listener has reached `gw_start_block` —
/// the caller then prunes pre-start proofs and releases the zkproof-worker.
/// Returns `Ok(false)` on cancellation, if the GCS row left the gw-gateable
/// states, or if `gw_dry_run_started` is already set (another firing won the
/// race); the caller then skips the prune and release.
async fn wait_until_gw_dry_run_ready(
    pool: Pool<Postgres>,
    cancel: CancellationToken,
    proposal_id: &[u8],
    proposal_block: i64,
    gw_start_block: i64,
) -> Result<bool, Error> {
    info!(
        gw_start_block,
        "Starting GCS gateway dry-run readiness loop"
    );

    // Dedicated listener so this loop is decoupled from the main run() listener
    // and the host-chain readiness loop.
    let mut listener = PgListener::connect_with(&pool).await?;
    listener
        .listen_all([GW_NEW_BLOCK_CHANNEL, EVENT_DRY_RUN_ROLLED_BACK])
        .await?;

    loop {
        if cancel.is_cancelled() {
            info!("gw readiness loop cancelled");
            return Ok(false);
        }

        let row: Option<(String, bool)> = sqlx::query_as(
            "SELECT state, gw_dry_run_started FROM upgrade_state
              WHERE stack_role = 'GCS'
                AND proposal_id = $1
                AND COALESCE(proposal_block, -1) = $2
              ORDER BY host_chain_id
              LIMIT 1",
        )
        .bind(proposal_id)
        .bind(proposal_block)
        .fetch_optional(&pool)
        .await?;
        match row {
            Some((_, true)) => {
                info!("GCS gw_dry_run_started already set — gw readiness loop exiting");
                return Ok(false);
            }
            Some((state, false)) if state == "UpgradeActivated" || state == "DryRunStarted" => {}
            Some((state, false)) => {
                info!(
                    state,
                    "GCS state is past the gw-gateable window — gw readiness loop exiting"
                );
                return Ok(false);
            }
            None => {
                warn!("No GCS row in upgrade_state — gw readiness loop exiting");
                return Ok(false);
            }
        }

        match check_gw_dry_run_ready(&pool, gw_start_block).await {
            Ok(true) => {
                info!(gw_start_block, "Gateway dry-run readiness satisfied");
                return Ok(true);
            }
            Ok(false) => {
                debug!(
                    gw_start_block,
                    "Gateway dry-run readiness not yet satisfied; waiting for next gw block"
                );
            }
            Err(e) => {
                error!(error = %e, "gw readiness check query failed; will retry on next notification");
            }
        }

        select! {
            _ = cancel.cancelled() => {
                info!("gw readiness loop cancelled");
                return Ok(false);
            }
            recv = listener.recv() => {
                match recv {
                    Ok(notification) => {
                        debug!(
                            channel = notification.channel(),
                            payload = notification.payload(),
                            gw_start_block,
                            "gw readiness loop trigger"
                        );
                    }
                    Err(e) => {
                        warn!(error = %e, "gw readiness listener recv error; sleeping before retry");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}

/// Delete from `gcs.verify_proofs` every proof whose Gateway `block_number` is
/// below `gw_start_block`. The GCS gw-listener accumulates proof requests from
/// startup, so the GCS schema may hold proofs for Gateway blocks that precede
/// the re-randomization switchover; clearing them makes the zkproof-worker's
/// dry-run snapshot start cleanly at `gw_start_block`. Rows with a NULL
/// `block_number` are left untouched (mirrors the computations prune). Returns
/// the number of rows removed. Idempotent.
async fn prune_gcs_verify_proofs_before_start(
    pool: &Pool<Postgres>,
    proposal_id: &[u8],
    proposal_block: i64,
    gw_start_block: i64,
) -> Result<u64, Error> {
    // Skip policy: a rollback that paused the GCS row means there is no
    // attempt left to prune for.
    let WriteGuard::Proceed(mut tx) =
        begin_write_guarded(pool, true, GcsRollbackPolicy::Skip).await?
    else {
        return Ok(0);
    };

    let sql = format!(
        "WITH current_attempt AS (
             SELECT 1 FROM upgrade_state
              WHERE stack_role = 'GCS'
                AND state IN ('UpgradeActivated', 'DryRunStarted')
                AND gw_dry_run_started = FALSE
                AND proposal_id = $2
                AND COALESCE(proposal_block, -1) = $3
              FOR SHARE
         )
         DELETE FROM {GCS_SCHEMA_QUOTED}.verify_proofs \
         WHERE block_number IS NOT NULL \
           AND block_number < $1 \
           AND EXISTS (SELECT 1 FROM current_attempt)"
    );
    let result = sqlx::query(&sql)
        .bind(gw_start_block)
        .bind(proposal_id)
        .bind(proposal_block)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    info!(
        gw_start_block,
        deleted = result.rows_affected(),
        "pruned pre-gw_start_block rows from gcs.verify_proofs"
    );

    Ok(result.rows_affected())
}

/// Conditional UPDATE: marks the GCS row's `gw_dry_run_started` and notifies the
/// zkproof-worker. Only flips a GCS row still in the gw-gateable window with the
/// flag unset, so a duplicate firing is a no-op.
async fn transition_to_gw_dry_run_started(
    pool: &Pool<Postgres>,
    proposal_id: &[u8],
    proposal_block: i64,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
        UPDATE upgrade_state
        SET gw_dry_run_started = TRUE, updated_at = NOW()
        WHERE stack_role = 'GCS'
          AND gw_dry_run_started = FALSE
          AND state IN ('UpgradeActivated', 'DryRunStarted')
          AND proposal_id = $1
          AND COALESCE(proposal_block, -1) = $2
        "#,
    )
    .bind(proposal_id)
    .bind(proposal_block)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        warn!(
            "transition_to_gw_dry_run_started: GCS row not eligible (already set, past window, or other proposal) — skipping notify"
        );
        return Ok(());
    }

    // Release the GCS zkproof-worker, which stays parked until it observes
    // `gw_dry_run_started`. Payload unused — the worker's gw activation watcher
    // re-reads upgrade_state on wake.
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(GW_DRY_RUN_STARTED_CHANNEL)
        .bind("")
        .execute(pool)
        .await?;
    info!(
        "transition_to_gw_dry_run_started: GCS gw_dry_run_started=true; zkproof-worker release notify sent"
    );

    Ok(())
}

/// Delete every BCS row inside the host-chain upgrade windows at cutover. BCS runs
/// ahead of GCS, so results GCS never ingested would otherwise survive the merge and
/// leave each operator with a different set. Rather than diff the two stacks handle by
/// handle, drop blue's whole in-window output — ciphertexts plus the `computations` /
/// `pbs_computations` rows behind them, in both their legacy and `*_branch` form — and
/// let the merge re-establish green's copy from `gcs.*`. What green never ingested stays
/// deleted and is re-derived from the chain once green is live.
///
/// MUST run inside the cutover transaction and *before* [`merge_gcs_table`]: the
/// deletes are unguarded, so after the merge they would wipe the rows it just
/// brought over.
async fn delete_bcs_chains_leftovers(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    // One row per host-chain since the per-chain windows migration, each with its own
    // start_block, so every chain has to be walked — a single-row read would silently
    // take whichever row the scan happens to return first and skip the rest.
    let rows: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT host_chain_id, start_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND start_block IS NOT NULL
          ORDER BY host_chain_id",
    )
    .fetch_all(&mut **tx)
    .await?;

    if rows.is_empty() {
        warn!("delete_bcs_chains_leftovers: no host-chain rows found in upgrade_state — skipping");
        return Ok(());
    }

    for (chain_id, start_block) in rows {
        delete_bcs_chain_leftovers(tx, chain_id, start_block).await?;
    }
    Ok(())
}

/// Delete one host-chain's in-window BCS rows, per [`delete_bcs_chains_leftovers`].
///
/// Every table is cleared in both its legacy and its `*_branch` form
/// ([`CIPHERTEXT_TABLES`], [`COMPUTATION_TABLES`], [`PBS_COMPUTATION_TABLES`], plus
/// `ciphertext_digest_branch`). The branch tables are the canonical ones once the
/// branch-context migration completes, so leaving blue's in-window rows there would
/// reintroduce exactly the per-operator divergence these deletes exist to prevent.
///
/// Statement order is load-bearing: the ciphertext deletes reach their rows through the
/// computation tables, so those are cleared last.
async fn delete_bcs_chain_leftovers(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    start_block: i64,
) -> Result<(), Error> {
    for ct_table in CIPHERTEXT_TABLES {
        for comp_table in COMPUTATION_TABLES {
            let sql = format!(
                "DELETE FROM public.{ct_table} ct
                   USING public.{comp_table} comp
                  WHERE ct.handle = comp.output_handle
                    AND comp.host_chain_id = $1 AND comp.block_number >= $2"
            );
            let deleted = sqlx::query(&sql)
                .bind(chain_id)
                .bind(start_block)
                .execute(&mut **tx)
                .await?
                .rows_affected();

            info!(
                host_chain_id = chain_id,
                start_block,
                ct_table,
                comp_table,
                deleted,
                "delete_bcs_leftovers: deleted BCS-leftover ciphertexts"
            );
        }
    }

    for pbs_table in PBS_COMPUTATION_TABLES {
        let sql = format!(
            "DELETE FROM public.{pbs_table} p
              WHERE p.host_chain_id = $1 AND p.block_number >= $2"
        );
        let deleted = sqlx::query(&sql)
            .bind(chain_id)
            .bind(start_block)
            .execute(&mut **tx)
            .await?
            .rows_affected();
        info!(
            host_chain_id = chain_id,
            start_block, pbs_table, deleted, "delete_bcs_leftovers: deleted BCS-leftover pbs rows"
        );
    }

    // `ciphertext_digest_branch`, whose legacy sibling is deliberately left alone (the
    // `txn_is_sent` restore in `execute_cutover` reads `public.ciphertext_digest`, and
    // what happens to already-published handles is still an open decision).
    //
    // The `pbs_computations_branch` delete above already drops the digest rows sharing
    // its branch context, via the `mirror_ciphertext_digest_pbs_context` trigger, so
    // this is usually a no-op. It stays explicit so cutover does not depend on a
    // trigger defined in an unrelated migration for its own correctness.
    //
    // Branchless rows carry `block_number IS NULL` (enforced by
    // `ciphertext_digest_branch_producer_block_number_check`) and are the mirror of
    // `public.ciphertext_digest`; `block_number >= $2` skips them, so the two stay in
    // sync.
    let deleted_digests = sqlx::query!(
        "DELETE FROM public.ciphertext_digest_branch d
          WHERE d.host_chain_id = $1 AND d.block_number >= $2",
        chain_id,
        start_block,
    )
    .execute(&mut **tx)
    .await?
    .rows_affected();
    info!(
        host_chain_id = chain_id,
        start_block,
        deleted_digests,
        "delete_bcs_leftovers: deleted BCS-leftover ciphertext_digest_branch rows"
    );

    // Last: the ciphertext deletes above reach their rows through these tables.
    //
    // `RETURNING` hands the deleted rows' dependence chains straight to
    // `cutover_touched_chains`, which `cleanup_orphaned_dependence_chains` drains once
    // every chain has been walked. `dependence_chain` has no chain or block column, so
    // this is the only way to scope it to the upgrade window.
    for comp_table in COMPUTATION_TABLES {
        let sql = format!(
            "WITH deleted AS (
                 DELETE FROM public.{comp_table} c
                  WHERE c.host_chain_id = $1 AND c.block_number >= $2
                 RETURNING c.dependence_chain_id
             ),
             touched AS (
                 INSERT INTO cutover_touched_chains (dependence_chain_id)
                 SELECT DISTINCT dependence_chain_id FROM deleted
                  WHERE dependence_chain_id IS NOT NULL
                 ON CONFLICT DO NOTHING
             )
             SELECT COUNT(*) FROM deleted"
        );
        let deleted: i64 = sqlx::query_scalar(&sql)
            .bind(chain_id)
            .bind(start_block)
            .fetch_one(&mut **tx)
            .await?;
        info!(
            host_chain_id = chain_id,
            start_block,
            comp_table,
            deleted,
            "delete_bcs_leftovers: deleted BCS-leftover computation rows"
        );
    }

    Ok(())
}

/// Undo everything blue wrote inside the upgrade windows, so the merge decides what
/// survives instead of blue's head start.
///
/// The complete BCS revert, in one place:
///   1. create `cutover_touched_chains`, which [`delete_bcs_chain_leftovers`] fills with
///      the dependence chains of the computations it deletes;
///   2. delete each host chain's in-window rows;
///   3. delete the gateway/input side's leftovers;
///   4. drop the dependence chains no computation needs any more, blue's or green's.
///
/// **Must run inside the cutover transaction and before [`merge_gcs_table`].** These
/// deletes are unguarded, so after the merge they would wipe the rows it just brought
/// over. Step 4 is safe here only because it checks both schemas — see
/// [`cleanup_orphaned_dependence_chains`].
async fn revert_bcs_state(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    sqlx::query(
        "CREATE TEMP TABLE cutover_touched_chains (dependence_chain_id BYTEA PRIMARY KEY)
         ON COMMIT DROP",
    )
    .execute(&mut **tx)
    .await?;

    delete_bcs_chains_leftovers(tx).await?;
    delete_bcs_gw_leftovers(tx).await?;
    cleanup_orphaned_dependence_chains(tx).await?;
    Ok(())
}

/// Delete the dependence chains whose only computations were blue's in-window rows.
///
/// Step 4 of [`revert_bcs_state`], which is its only caller.
///
/// `dependence_chain` is scheduler bookkeeping keyed by `dependence_chain_id` with no
/// chain or block column, so it cannot be scoped by the upgrade window directly. Instead
/// [`delete_bcs_chain_leftovers`] collects the deleted computations' chains via
/// `DELETE ... RETURNING` into the `cutover_touched_chains` temp table, and this deletes
/// the ones nothing points at any more.
///
/// The check spans **both schemas**, and that is what lets it run before the merge. After
/// the merge, `public.computations` would hold blue's survivors plus green's merged rows;
/// before it, that same set is `public` ∪ `gcs`. Looking at both therefore gives the
/// identical answer one step earlier. Checking only `public` here would be wrong: a chain
/// referenced solely by green's not-yet-merged computations would look orphaned, and
/// deleting it could leave green's merged rows pointing at a chain id with no row — which
/// the scheduler needs to acquire that work.
///
/// Scoping to the recorded set also keeps the blast radius small: a plain table scan for
/// orphans would sweep up rows unrelated to the upgrade, including a chain a concurrent
/// writer had inserted before its computations.
async fn cleanup_orphaned_dependence_chains(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), Error> {
    let mut sql = String::from(
        "DELETE FROM public.dependence_chain d
          WHERE EXISTS (SELECT 1 FROM cutover_touched_chains t
                         WHERE t.dependence_chain_id = d.dependence_chain_id)",
    );
    for schema in ["public", GCS_SCHEMA_QUOTED] {
        for comp_table in COMPUTATION_TABLES {
            sql.push_str(&format!(
                " AND NOT EXISTS (SELECT 1 FROM {schema}.{comp_table} c
                                   WHERE c.dependence_chain_id = d.dependence_chain_id)"
            ));
        }
    }
    let deleted = sqlx::query(&sql).execute(&mut **tx).await?.rows_affected();
    info!(
        deleted,
        "cutover: deleted dependence chains left with no computations"
    );
    Ok(())
}

/// Delete the gateway/zkproof-input side's BCS leftovers at cutover. Green re-verifies
/// proofs to the *same* input handles but *different* ciphertext bytes (the
/// re-randomization strategy switches at cutover), so blue's input ciphertexts must not
/// outlive the dry-run: those belonging to gw-window handles green never reproduced are
/// deleted, then every `input_handles` row at/after `gw_start_block` goes, with the
/// merge restoring the ones green did reproduce.
///
/// `gw_start_block` is a Gateway block, not a host-chain block, so this window is
/// proposal-wide rather than per-chain. Like [`delete_bcs_chains_leftovers`], must run
/// before the merge and the schema drop, while `gcs.*` still exists.
///
/// Note: `verify_proofs` is left exactly as blue wrote it, so a proof green never
/// re-verified stays `verified = TRUE` and the zkproof-worker, which only picks up
/// `verified IS NULL`, will not re-derive the input ciphertexts deleted here.
async fn delete_bcs_gw_leftovers(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT gw_start_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND gw_start_block IS NOT NULL",
    )
    .fetch_optional(&mut **tx)
    .await?;
    let Some((gw_start_block,)) = row else {
        warn!("delete_bcs_gw_leftovers: no gw_start_block found in upgrade_state — skipping");
        return Ok(());
    };
    // Narrowed to handles absent from `gcs.input_handles`: green's own input
    // ciphertexts are about to be merged back anyway, so only blue's orphans have to
    // go.
    for ct_table in CIPHERTEXT_TABLES {
        let sql = format!(
            "DELETE FROM public.{ct_table} ct
              WHERE EXISTS (
                  SELECT 1 FROM public.input_handles ih
                   WHERE ih.handle = ct.handle
                     AND ih.block_number >= $1
                     AND NOT EXISTS (
                         SELECT 1 FROM {GCS_SCHEMA_QUOTED}.input_handles g
                          WHERE g.handle = ih.handle))"
        );
        sqlx::query(&sql)
            .bind(gw_start_block)
            .execute(&mut **tx)
            .await?;
    }

    // Clear the whole gw window last: the ciphertext deletes above reach their rows
    // through this table, and the merge restores every handle green reproduced.
    let deleted_input_handles = sqlx::query!(
        "DELETE FROM public.input_handles ih
          WHERE ih.block_number >= $1",
        gw_start_block,
    )
    .execute(&mut **tx)
    .await?
    .rows_affected();

    info!(
        gw_start_block,
        rows_deleted = deleted_input_handles,
        "delete_bcs_gw_leftovers: deleted BCS-leftover ciphertexts and input_handles"
    );

    Ok(())
}

/// Merge every row from `gcs.<table>` into `public.<table>`, letting the GCS
/// rows win on collisions (`ON CONFLICT (<conflict_cols>) DO UPDATE`) — GCS is
/// the canonical writer for its dry-run window. Driven by [`execute_cutover`]
/// over the [`COPROCESSOR_TABLES`] entries where [`CoprocessorTable::is_merged`].
///
/// The column list is read from the live catalog rather than hard-coded: these
/// tables have accreted many columns across migrations, and a stale
/// hand-maintained list would silently drop a column or fail the whole cutover
/// transaction. `conflict_cols` must name an existing unique/primary-key
/// constraint on `public.<table>`. Generated / identity columns are excluded
/// (they cannot appear in an INSERT column list). Returns the number of rows
/// merged.
async fn merge_gcs_table(
    tx: &mut Transaction<'_, Postgres>,
    table: &str,
    conflict_cols: &[&str],
) -> Result<u64, Error> {
    let cols: Vec<String> = sqlx::query_scalar(
        "SELECT column_name
           FROM information_schema.columns
          WHERE table_schema = 'public'
            AND table_name = $1
            AND is_generated = 'NEVER'
            AND is_identity = 'NO'
          ORDER BY ordinal_position",
    )
    .bind(table)
    .fetch_all(&mut **tx)
    .await?;

    if cols.is_empty() {
        return Err(Error::Payload(format!(
            "cannot merge gcs.{table}: no insertable columns found for public.{table}"
        )));
    }

    let col_list = cols.join(", ");
    let set_clause = cols
        .iter()
        .filter(|c| !conflict_cols.contains(&c.as_str()))
        .map(|c| format!("{c} = EXCLUDED.{c}"))
        .collect::<Vec<_>>()
        .join(", ");
    let conflict = conflict_cols.join(", ");

    // If every column is part of the conflict key the SET would be empty; the
    // row already matches, so DO NOTHING is the correct degenerate case.
    let action = if set_clause.is_empty() {
        "DO NOTHING".to_string()
    } else {
        format!("DO UPDATE SET {set_clause}")
    };

    let sql = format!(
        "INSERT INTO public.{table} ({col_list})
         SELECT {col_list} FROM {GCS_SCHEMA_QUOTED}.{table}
         ON CONFLICT ({conflict}) {action}"
    );
    let merged = sqlx::query(&sql).execute(&mut **tx).await?;
    info!(
        table,
        merged = merged.rows_affected(),
        "merged gcs table into public"
    );
    Ok(merged.rows_affected())
}

/// Read the GCS proposal rows under the cutover transaction and decide whether to
/// proceed, returning the stack version to promote.
///
/// `Ok(None)` means the proposal is not `UpgradeAuthorized`, so there is nothing to do —
/// normally because a previous attempt already committed. This is what makes
/// [`execute_cutover`] idempotent under retry, so the caller must treat `None` as success
/// and stop, not as an error.
async fn authorized_stack_version(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<Option<String>, Error> {
    let rows = sqlx::query!(
        "SELECT state, start_block, version
           FROM public.upgrade_state
          WHERE stack_role = 'GCS'
          ORDER BY host_chain_id",
    )
    .fetch_all(&mut **tx)
    .await?;

    let Some(first) = rows.first() else {
        return Err(Error::Payload(
            "no GCS rows in upgrade_state — cannot run cutover".to_string(),
        ));
    };
    if rows.iter().any(|row| row.state != first.state) {
        return Err(Error::Payload(
            "GCS upgrade_state rows disagree on state".to_string(),
        ));
    }
    if first.state != "UpgradeAuthorized" {
        info!(state = %first.state, "cutover: GCS proposal is not UpgradeAuthorized — skipping (already cut over)");
        return Ok(None);
    }
    if rows.iter().any(|row| row.start_block.is_none()) {
        return Err(Error::Payload(
            "a GCS upgrade_state row is missing start_block".to_string(),
        ));
    }
    if rows.iter().any(|row| row.version != first.version) {
        return Err(Error::Payload(
            "GCS upgrade_state rows disagree on version".to_string(),
        ));
    }
    Ok(Some(first.version.clone().unwrap_or_default()))
}

/// Refuse to cut over while blue still has pre-window ciphertexts waiting to reach S3.
///
/// Handles produced *before* `start_block` are blue's alone: green pruned them out of its
/// snapshot and never recomputed them, so the merge cannot supply them and cutover does not
/// delete them. If blue is retired with their S3 upload still pending, the only stack that
/// was going to finish that upload is gone — `assert_not_retired` fails its next write —
/// and the digest published on-chain would point at an object that was never written.
///
/// "Pending" is the resubmit loop's own predicate (`sns-worker/src/aws_upload.rs`): the ct64
/// digest is NULL, or the ct128 digest is NULL *while* the ct128 bytes exist. The second
/// half matters — a handle that never went through SnS has no ct128 and must not be counted
/// as pending forever.
///
/// Returns an error rather than skipping, so the caller's retry and `reconcile`'s poll tick
/// keep re-checking: this is a "not yet" condition that blue itself clears. A permanently
/// stuck upload therefore blocks the upgrade, visibly, instead of silently retiring blue
/// with unwritten objects.
///
/// Scope limit: this counts only handles that already have a `ciphertext_digest` row, which
/// is exactly what the resubmit loop can act on. A pre-window handle whose digest row was
/// never created at all is not detected here.
async fn assert_bcs_ciphertexts_uploaded(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    // Driven from the un-uploaded digests, not from the window. `ciphertext_digest` carries
    // partial indexes on exactly these two NULL predicates
    // (`idx_ciphertext_digest_ciphertext_null` / `..._ciphertext128_null`), and when uploads
    // are keeping up the set is near-empty, so it is by far the cheapest starting point.
    // MATERIALIZED pins that: without it Postgres may inline the CTE and instead drive from
    // the window's computations, which is the whole history below start_block.
    let origins = COMPUTATION_TABLES
        .iter()
        .map(|comp_table| {
            format!(
                "SELECT comp.host_chain_id, p.handle
                   FROM pending p
                   JOIN public.{comp_table} comp ON comp.output_handle = p.handle
                   JOIN windows w ON w.host_chain_id = comp.host_chain_id
                  WHERE comp.block_number < w.start_block"
            )
        })
        .collect::<Vec<_>>()
        // UNION, not UNION ALL: a handle in both the legacy and branch table counts once.
        .join(" UNION ");

    let sql = format!(
        "WITH pending AS MATERIALIZED (
             SELECT d.handle
               FROM public.ciphertext_digest d
              WHERE d.ciphertext IS NULL
                 OR (d.ciphertext128 IS NULL
                     AND EXISTS (SELECT 1 FROM public.ciphertexts128 c
                                  WHERE c.handle = d.handle
                                    AND c.ciphertext IS NOT NULL))
         ),
         windows AS (
             SELECT host_chain_id, start_block
               FROM public.upgrade_state
              WHERE stack_role = 'GCS' AND start_block IS NOT NULL
         ),
         origins AS ({origins})
         SELECT host_chain_id, COUNT(*) FROM origins
          GROUP BY host_chain_id
          ORDER BY host_chain_id"
    );
    let per_chain: Vec<(i64, i64)> = sqlx::query_as(&sql).fetch_all(&mut **tx).await?;

    if per_chain.is_empty() {
        info!("cutover: BCS pre-window ciphertexts are fully uploaded to S3");
        return Ok(());
    }

    let mut pending_total: i64 = 0;
    for (chain_id, pending) in &per_chain {
        warn!(
            host_chain_id = chain_id,
            pending,
            "cutover blocked: BCS ciphertexts below start_block are not yet uploaded to S3"
        );
        pending_total += pending;
    }
    Err(Error::PendingBcsUploads {
        pending: pending_total,
    })
}

/// Cutover routine — run once every GCS row is `UpgradeAuthorized`, from the
/// unanimity handler or from `reconcile`. Idempotent via the under-lock re-read.
///
/// Runs atomically inside one transaction holding `pg_advisory_xact_lock(CUTOVER_LOCK_ID)`
/// in exclusive mode. The exclusive lock blocks until every BCS write tx
/// (which takes the same lock in shared mode at the top of each tx) has
/// committed, and conversely prevents any new BCS write tx from starting
/// until cutover commits.
///
/// Sequence:
///   0. `assert_bcs_ciphertexts_uploaded` with no lock held, so an attempt that is only
///      going to defer never queues the whole fleet behind the exclusive lock first.
///      Nothing here may take a row or table lock: requesting the advisory lock while
///      holding one inverts the order the host-listener uses (advisory shared, then
///      `upgrade_state`) and deadlocks against activation.
///   1. Take the exclusive advisory lock, then `authorized_stack_version`: read every GCS
///      chain row; no-op unless `UpgradeAuthorized`, else take its `version`. A plain read
///      suffices under the lock — activation and other controllers are excluded by it, and
///      every ungated FSM update is guarded on an earlier state.
///   2. UPDATE `versioning` to the new stack_version.
///   3. Snapshot the handles BCS already committed on-chain.
///   4. `revert_bcs_state`: delete blue's in-window rows, host chains then the
///      gateway/input side, and the dependence chains left with no computations.
///   5. Merge every `gcs.<table>` marked [`CoprocessorTable::is_merged`] into
///      `public`, then restore the snapshotted `txn_is_sent` flags.
///   6. DROP SCHEMA gcs CASCADE.
///   7. Mark the GCS rows LIVE/completed.
///   8. NOTIFY every service that the live stack version changed.
///
/// After commit, any BCS write tx that was waiting on the shared lock
/// acquires it, re-reads its FSM state, sees `PAUSED`, and exits cleanly.
/// Tables that hold one row per ciphertext handle, in delete-safe order.
///
/// Both synthetic cleanups sweep these: the Gateway input's handles come from
/// `verify_proofs.handles`, the host ops' from `computations.output_handle`. Anything missing
/// here survives the merge and becomes live production data - the concrete failure being the
/// newly-live green transaction-sender publishing a ciphertext digest for a handle that exists
/// on no chain.
///
/// `ciphertexts128*` are included because the sns-worker may already have squashed a synthetic
/// handle; `pbs_computations*` because that is the sns queue and is keyed on `handle`, not on
/// `transaction_id`.
const SYNTHETIC_HANDLE_TABLES: &[&str] = &[
    "ciphertext_digest",
    "ciphertext_digest_branch",
    "pbs_computations",
    "pbs_computations_branch",
    "ciphertexts128",
    "ciphertexts128_branch",
    "ciphertexts",
    "ciphertexts_branch",
    "allowed_handles",
    "allowed_handles_branch",
    "input_handles",
];

/// Delete every row in [`SYNTHETIC_HANDLE_TABLES`] whose handle is in `handle_source`, a
/// already-populated temp table with a single `handle BYTEA` column.
///
/// Logs the statement text, the row count and `marker` per table, so the cutover log carries an
/// auditable record of exactly what was removed. The SQL is logged as sent, with its bind
/// parameters reported separately - building an interpolated string just to log it would invite
/// exactly the injection-shaped mistake that binds exist to prevent.
async fn delete_synthetic_rows_by_handle(
    tx: &mut Transaction<'_, Postgres>,
    handle_source: &str,
    marker: &str,
) -> Result<u64, Error> {
    let mut total = 0u64;
    for table in SYNTHETIC_HANDLE_TABLES {
        let sql = format!(
            "DELETE FROM {GCS_SCHEMA_QUOTED}.{table} \
             WHERE handle IN (SELECT handle FROM {handle_source})"
        );
        let deleted = sqlx::query(&sql).execute(&mut **tx).await?.rows_affected();
        total = total.saturating_add(deleted);
        if deleted > 0 {
            info!(table, deleted, marker, sql = %sql, "cutover: deleted synthetic rows by handle");
        }
    }
    Ok(total)
}

/// Delete the synthetic FHE work the GCS host-listener injected into each host chain's dry-run
/// window, together with every ciphertext derived from it, before the merge carries `gcs.*` into
/// `public`.
///
/// Keyed on `upgrade_state.synthetic_txn_hash`, written by the injector at `start_block + 1`.
/// `computations.transaction_id` *is* the log's transaction hash, so that column is the join key
/// on the computation side, and the handles it yields drive the by-handle sweep.
///
/// Runs inside the cutover transaction and before the merge loop, so it is atomic with the
/// cutover: if the cutover rolls back, so does this.
async fn delete_gcs_synthetic_ops(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    // Snapshot the markers so they can be logged and reused; one per host chain that reached its
    // injection block. A window that never got that far has NULL and contributes nothing.
    let markers: Vec<(i64, Vec<u8>)> = sqlx::query_as(
        "SELECT host_chain_id, synthetic_txn_hash
           FROM upgrade_state
          WHERE stack_role = 'GCS'
            AND synthetic_txn_hash IS NOT NULL
          ORDER BY host_chain_id",
    )
    .fetch_all(&mut **tx)
    .await?;

    if markers.is_empty() {
        info!("cutover: no synthetic host ops recorded — nothing to delete");
        return Ok(());
    }

    let hashes: Vec<Vec<u8>> = markers.iter().map(|(_, h)| h.clone()).collect();
    for (host_chain_id, hash) in &markers {
        info!(
            host_chain_id,
            synthetic_txn_hash = %hex_encode(hash),
            "cutover: deleting synthetic host ops for chain"
        );
    }

    // Resolve the handles from BOTH computation tables before deleting anything: the legacy and
    // branch forms are written together, but a reorg can leave a handle in only one of them.
    let snapshot_sql = format!(
        "CREATE TEMP TABLE synthetic_op_handles ON COMMIT DROP AS
         SELECT DISTINCT output_handle AS handle
           FROM {GCS_SCHEMA_QUOTED}.computations
          WHERE transaction_id = ANY($1::bytea[])
          UNION
         SELECT DISTINCT output_handle AS handle
           FROM {GCS_SCHEMA_QUOTED}.computations_branch
          WHERE transaction_id = ANY($1::bytea[])"
    );
    sqlx::query(&snapshot_sql)
        .bind(&hashes)
        .execute(&mut **tx)
        .await?;
    let handle_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM synthetic_op_handles")
        .fetch_one(&mut **tx)
        .await?;
    info!(
        handle_count,
        sql = %snapshot_sql,
        "cutover: resolved synthetic op handles"
    );

    // Snapshot the dependence chains before the computations that reference them are gone,
    // otherwise there is nothing left to identify them by.
    let chains_sql = format!(
        "CREATE TEMP TABLE synthetic_op_chains ON COMMIT DROP AS
         SELECT DISTINCT dependence_chain_id
           FROM {GCS_SCHEMA_QUOTED}.computations
          WHERE transaction_id = ANY($1::bytea[])
            AND dependence_chain_id IS NOT NULL"
    );
    sqlx::query(&chains_sql)
        .bind(&hashes)
        .execute(&mut **tx)
        .await?;

    let by_handle = delete_synthetic_rows_by_handle(tx, "synthetic_op_handles", "host_ops").await?;

    // Computations after their ciphertexts, so a failure part-way leaves the marker and the
    // computations in place to retry from rather than orphaning ciphertexts.
    let mut computation_rows = 0u64;
    for table in ["computations", "computations_branch"] {
        let sql = format!(
            "DELETE FROM {GCS_SCHEMA_QUOTED}.{table} WHERE transaction_id = ANY($1::bytea[])"
        );
        let deleted = sqlx::query(&sql)
            .bind(&hashes)
            .execute(&mut **tx)
            .await?
            .rows_affected();
        computation_rows = computation_rows.saturating_add(deleted);
        if deleted > 0 {
            info!(table, deleted, sql = %sql, "cutover: deleted synthetic computations");
        }
    }

    // The telemetry row for the transaction. Keyed `id`, NOT `transaction_id` - the column name
    // differs from every other table here, which is exactly why this delete is separate.
    let txn_sql =
        format!("DELETE FROM {GCS_SCHEMA_QUOTED}.transactions WHERE id = ANY($1::bytea[])");
    let txn_deleted = sqlx::query(&txn_sql)
        .bind(&hashes)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    computation_rows = computation_rows.saturating_add(txn_deleted);
    if txn_deleted > 0 {
        info!(
            table = "transactions",
            deleted = txn_deleted,
            sql = %txn_sql,
            "cutover: deleted synthetic transaction telemetry"
        );
    }

    // Dependence chains last, and only those no computation still references. The NOT EXISTS
    // spans `public` as well as the GCS schema because this runs pre-merge: a chain id can be
    // shared with work that is staying.
    let orphan_sql = format!(
        "DELETE FROM {GCS_SCHEMA_QUOTED}.dependence_chain d
          WHERE d.dependence_chain_id IN (SELECT dependence_chain_id FROM synthetic_op_chains)
            AND NOT EXISTS (
                SELECT 1 FROM {GCS_SCHEMA_QUOTED}.computations c
                 WHERE c.dependence_chain_id = d.dependence_chain_id)
            AND NOT EXISTS (
                SELECT 1 FROM {GCS_SCHEMA_QUOTED}.computations_branch c
                 WHERE c.dependence_chain_id = d.dependence_chain_id)
            AND NOT EXISTS (
                SELECT 1 FROM public.computations c
                 WHERE c.dependence_chain_id = d.dependence_chain_id)
            AND NOT EXISTS (
                SELECT 1 FROM public.computations_branch c
                 WHERE c.dependence_chain_id = d.dependence_chain_id)"
    );
    let chains_deleted = sqlx::query(&orphan_sql)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    info!(
        chains_deleted,
        sql = %orphan_sql,
        "cutover: deleted orphaned synthetic dependence chains"
    );

    // Clear the markers so a later attempt cannot match this window's work.
    sqlx::query(
        "UPDATE upgrade_state SET synthetic_txn_hash = NULL, updated_at = NOW()
          WHERE stack_role = 'GCS' AND synthetic_txn_hash IS NOT NULL",
    )
    .execute(&mut **tx)
    .await?;

    info!(
        chains = markers.len(),
        by_handle, computation_rows, chains_deleted, "cutover: synthetic host ops removed"
    );
    Ok(())
}

/// Delete the synthetic Gateway input the GCS gw-listener injected for consensus anchoring,
/// and everything derived from it, before the merge carries `gcs.*` into `public`.
///
/// Why this is mandatory: `verify_proofs`, `ciphertexts`, `ciphertexts_branch`,
/// `ciphertext_digest`, `input_handles` and `pbs_computations` are all merged at cutover. Left
/// alone, the synthetic input's handles become live production rows, and the now-live green
/// `transaction-sender` starts trying to publish ciphertext digests for a handle that belongs
/// to no contract on any chain.
///
/// Keyed on `zk_proof_id >= SYNTHETIC_ZK_PROOF_ID_BASE` — the marker
/// [`fhevm_engine_common::synthetic_input::synthetic_zk_proof_id`] guarantees. The handle list
/// comes from `verify_proofs.handles`, a concatenation of 32-byte handles, unpacked with
/// `generate_series`.
///
/// Runs inside the cutover transaction and before the merge loop, so it is atomic with the
/// cutover: if the cutover rolls back, so does this.
async fn delete_gcs_synthetic_inputs(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
    // Unpack the 32-byte handles out of every synthetic row's `handles` blob. `handles` is
    // NULL until the zkproof-worker verifies the proof, so a synthetic input that never got
    // verified contributes no handles — only its `verify_proofs` row, deleted at the end.
    let snapshot_sql = format!(
        "CREATE TEMP TABLE synthetic_input_handles ON COMMIT DROP AS
         SELECT DISTINCT substring(v.handles FROM g.pos FOR 32) AS handle
           FROM {GCS_SCHEMA_QUOTED}.verify_proofs v
           CROSS JOIN LATERAL generate_series(1, octet_length(v.handles), 32) AS g(pos)
          WHERE v.zk_proof_id >= $1
            AND v.handles IS NOT NULL
            AND octet_length(v.handles) > 0"
    );
    sqlx::query(&snapshot_sql)
        .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
        .execute(&mut **tx)
        .await?;

    delete_synthetic_rows_by_handle(tx, "synthetic_input_handles", "gw_input").await?;

    // The `verify_proofs` row last, so a failure above leaves the marker in place for a retry
    // rather than orphaning the derived rows.
    let sql = format!("DELETE FROM {GCS_SCHEMA_QUOTED}.verify_proofs WHERE zk_proof_id >= $1");
    let deleted = sqlx::query(&sql)
        .bind(SYNTHETIC_ZK_PROOF_ID_BASE)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    info!(
        deleted,
        "cutover: deleted synthetic Gateway inputs from gcs.verify_proofs"
    );

    Ok(())
}

pub async fn execute_cutover(pool: &Pool<Postgres>) -> Result<(), Error> {
    info!("execute_cutover() starting");

    let started = Instant::now();
    let mut tx = pool.begin().await?;

    // 0. Pre-flight checks, no lock held: uploads must be complete first
    assert_bcs_ciphertexts_uploaded(&mut tx).await?;

    let acquire_start = Instant::now();
    info!("execute_cutover() pre-flight checks passed; acquiring exclusive advisory lock ...");

    // 1. Acquire the exclusive advisory lock, then read the GCS proposal rows to
    //    decide whether to proceed. The lock blocks until every BCS write tx has
    //    committed, and conversely prevents any new BCS write tx from starting until
    //    cutover commits.
    //
    // NB: The exclusive request waits only for the shared locks already granted
    // at the moment it queues.
    sqlx::query!("SELECT pg_advisory_xact_lock($1)", CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    info!(
        lock_id = CUTOVER_LOCK_ID,
        elapsed = acquire_start.elapsed().as_millis(),
        "cutover acquired exclusive advisory lock"
    );

    // Read the GCS proposal rows under the lock, and return early if they are
    // not `UpgradeAuthorized` (normally because a previous attempt already committed).
    let Some(stack_version) = authorized_stack_version(&mut tx).await? else {
        warn!("cutover: GCS proposal is not UpgradeAuthorized — skipping (already cut over)");
        return Ok(());
    };

    // 2. Promote the new stack version inside the cutover tx. This is the
    //    source of truth read by `resolve_gcs_mode` / `reconcile_stack_mode`:
    //    the green stack becomes live and the retired blue stack pauses.
    sqlx::query!(
        "UPDATE public.versioning
         SET stack_version = $1, updated_at = NOW()
         WHERE singleton = TRUE",
        stack_version.as_str(),
    )
    .execute(&mut *tx)
    .await?;
    info!(stack_version, "versioning row updated");

    // 3. Snapshot the handles BCS already committed on-chain: the merge would copy
    //    GCS's `txn_is_sent = false` onto them and the tx-sender would re-broadcast,
    //    reverting with `CoprocessorAlreadyAdded`. The flag is restored right after
    //    the merge.
    //    Only handles green also has can collide in the merge, so the snapshot is scoped to
    //    `gcs.ciphertext_digest` instead of copying the whole committed history. The join
    //    drives from the green side, which holds just the dry-run window.
    let snapshot = format!(
        "CREATE TEMP TABLE committed_before_cutover ON COMMIT DROP AS
         SELECT d.handle
           FROM {GCS_SCHEMA_QUOTED}.ciphertext_digest g
           JOIN public.ciphertext_digest d ON d.handle = g.handle
          WHERE d.txn_is_sent = TRUE"
    );
    sqlx::query(&snapshot).execute(&mut *tx).await?;

    // 4. Undo blue's in-window work, so the merge below decides what survives instead of
    //    blue's head start. Must stay ahead of the merge.
    revert_bcs_state(&mut tx).await?;

    // 5. Merge the GCS-canonical tables back into public before dropping the schema.
    //    Each merge lets the GCS rows win on PK collisions (GCS is the canonical
    //    writer for its dry-run window), then the committed flags snapshotted in
    //    step 3 are put back.
    // 3a. Drop the GCS stack's synthetic Gateway input before anything merges:
    //     it exists only to anchor dry-run consensus and must not become live data.
    // 3a. Drop the GCS stack's synthetic work before anything merges: it exists only to anchor
    //     dry-run consensus and must not become live data. Host ops first, so their by-handle
    //     sweep runs before the Gateway input's temp table shadows the name.
    delete_gcs_synthetic_ops(&mut tx).await?;
    delete_gcs_synthetic_inputs(&mut tx).await?;

    // 3. Merge the GCS-canonical tables back into public before dropping the
    //    schema. Each merge lets the GCS rows win on PK collisions (GCS is the
    //    canonical writer for its dry-run window).
    info!(stack_version, "cutover: merging gcs tables into public");
    for table in COPROCESSOR_TABLES {
        if !table.is_merged() {
            continue;
        }
        merge_gcs_table(&mut tx, table.name, table.conflict_cols).await?;
    }

    // This is to avoid CoprocessorAlreadyAdded (false negatives) for handles
    // that were already committed by BCS on-chain before cutover.
    // NB: This query must be deleted later on.
    let resent_guard = sqlx::query(
        "UPDATE public.ciphertext_digest d SET txn_is_sent = TRUE
           FROM committed_before_cutover c
          WHERE d.handle = c.handle AND d.txn_is_sent = FALSE",
    )
    .execute(&mut *tx)
    .await?
    .rows_affected();
    info!(
        resent_guard,
        "cutover: preserved txn_is_sent for already-committed handles"
    );

    // 6. Drop the gcs schema (and everything in it) now that its data has been
    //    merged back into public.
    let drop_sql = format!("DROP SCHEMA {GCS_SCHEMA_QUOTED} CASCADE");
    sqlx::query(&drop_sql).execute(&mut *tx).await?;
    info!(schema = GCS_SCHEMA_QUOTED, "dropped gcs schema");

    // 7. Flip every chain's FSM row.
    sqlx::query!(
        "UPDATE public.upgrade_state
         SET state = 'LIVE', status = 'completed', updated_at = NOW()
         WHERE stack_role = 'GCS'",
    )
    .execute(&mut *tx)
    .await?;

    // 8. Notify every service that the live stack version changed. Queued in
    //    the SAME transaction as the `versioning` UPDATE above, so the notify
    //    is atomic with the version bump — it is only delivered if the cutover
    //    commits. On receipt, each service re-evaluates its mode (the green
    //    stack leaves GCS mode to become live; the retired blue stack pauses
    //    into no-op mode).
    let payload = serde_json::json!({
        "new_version_number": stack_version,
    })
    .to_string();
    sqlx::query!(
        "SELECT pg_notify($1, $2)",
        EVENT_STACK_VERSION_UPGRADED,
        payload.as_str(),
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    info!(
        channel = EVENT_STACK_VERSION_UPGRADED,
        stack_version,
        elapsed_ms = started.elapsed().as_millis(),
        "execute_cutover() committed; stack-version-upgraded notify delivered"
    );
    Ok(())
}

/// [`execute_cutover`] with the [`CUTOVER_RETRY_ATTEMPTS`] budget and exponential
/// backoff. Use this everywhere instead of calling `execute_cutover` directly.
async fn execute_cutover_with_retry(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
) -> Result<(), Error> {
    retry_cutover(
        pool,
        cancel,
        CUTOVER_RETRY_ATTEMPTS,
        CUTOVER_RETRY_BASE_DELAY,
    )
    .await
}

/// [`execute_cutover_with_retry`] with the schedule spelled out, so tests can drive the
/// same loop without waiting out the production backoff.
///
/// Retries are safe to repeat: `execute_cutover` re-reads the FSM rows under the
/// exclusive lock and returns `Ok(())` untouched unless they are still
/// `UpgradeAuthorized`, so a retry that races a cutover which actually committed is a
/// no-op rather than a second merge. Returns the last error once the budget is spent, or
/// on cancellation.
async fn retry_cutover(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    attempts: u32,
    base_delay: Duration,
) -> Result<(), Error> {
    let mut attempt: u32 = 1;
    let mut delay = base_delay;
    loop {
        let err = match execute_cutover(pool).await {
            Ok(()) => return Ok(()),
            Err(err) => err,
        };

        let transient = err.is_transient();

        if attempt >= attempts {
            error!(
                attempt,
                attempts,
                error = %err,
                "cutover failed on the final attempt; GCS rows stay UpgradeAuthorized and \
                 the next reconcile tick will retry"
            );
            return Err(err);
        }

        warn!(
            attempt,
            attempts,
            backoff_ms = delay.as_millis(),
            reason = %err,
            transient,
            "cutover deferred; retrying after backoff"
        );

        select! {
            _ = cancel.cancelled() => {
                info!(
                    attempt,
                    "cancelled mid-retry; GCS rows stay UpgradeAuthorized and cutover \
                     resumes from the boot reconcile"
                );
                return  Err(err);
            }
            _ = tokio::time::sleep(delay) => {}
        }

        attempt += 1;
        delay = delay.saturating_mul(2).min(CUTOVER_RETRY_MAX_DELAY);
    }
}

/// Flip every GCS proposal row to `UpgradeAuthorized` and cut over once every
/// row has its host and Gateway consensus latches set. Guarded UPDATE, so
/// duplicates no-op.
async fn try_cutover_if_consensus(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
        WITH eligible_attempt AS (
            SELECT proposal_id, COALESCE(proposal_block, -1) AS proposal_block
              FROM upgrade_state
             WHERE stack_role = 'GCS' AND status = 'in_progress'
             GROUP BY proposal_id, COALESCE(proposal_block, -1)
            HAVING COUNT(*) > 0
               AND BOOL_AND(state = 'DryRunStarted')
               AND BOOL_AND(host_consensus_reached)
               AND BOOL_AND(gw_consensus_reached)
        )
        UPDATE upgrade_state u
           SET state = 'UpgradeAuthorized', updated_at = NOW()
          FROM eligible_attempt e
         WHERE u.stack_role = 'GCS'
           AND u.proposal_id = e.proposal_id
           AND COALESCE(u.proposal_block, -1) = e.proposal_block
        "#,
    )
    .execute(pool)
    .await?;
    if result.rows_affected() == 0 {
        debug!("cutover deferred — gateway or a host chain's consensus latch not yet set");
        return Ok(());
    }
    info!("gateway + all host chains reached consensus — transitioning to UpgradeAuthorized and running cutover");

    execute_cutover_with_retry(pool, cancel).await
}

/// Roll back a dry-run under the write lock: PAUSED/failed, reset schema, wake
/// workers. Scoped to both the proposal and its maximum end block, which is the
/// attempt identity carried by the detector's timeout notification. Returns
/// whether it acted.
async fn rollback_dry_run(
    pool: &Pool<Postgres>,
    proposal_id: &[u8],
    proposal_block: i64,
    timeout_end_block: i64,
) -> Result<bool, Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    info!(
        lock_id = CUTOVER_LOCK_ID,
        "rollback acquired exclusive advisory lock"
    );

    let claimed = sqlx::query(
        r#"
        UPDATE upgrade_state u
           SET state = 'PAUSED', status = 'failed',
               last_error = 'unanimity_consensus_timeout',
               host_consensus_reached = FALSE, gw_consensus_reached = FALSE,
               gw_dry_run_started = FALSE,
               -- Cleared with the latches: the rolled-back window's schema is dropped, and the
               -- next attempt injects at a different block (or fork) and so derives a different
               -- hash. A stale marker would leave cutover matching nothing while the real rows
               -- merged through.
               synthetic_txn_hash = NULL,
               updated_at = NOW()
         WHERE u.stack_role = 'GCS'
           AND u.state IN ('UpgradeActivated', 'DryRunStarted')
           AND u.proposal_id = $1
           AND COALESCE(u.proposal_block, -1) = $2
           AND $3 = (
               SELECT MAX(w.end_block)
                 FROM upgrade_state w
                WHERE w.stack_role = u.stack_role
                  AND w.proposal_id = u.proposal_id
                  AND COALESCE(w.proposal_block, -1) =
                      COALESCE(u.proposal_block, -1)
           )
        "#,
    )
    .bind(proposal_id)
    .bind(proposal_block)
    .bind(timeout_end_block)
    .execute(&mut *tx)
    .await?;
    if claimed.rows_affected() == 0 {
        tx.rollback().await?;
        return Ok(false);
    }
    reset_gcs_schema(&mut tx).await?;
    sqlx::query("SELECT pg_notify($1, '')")
        .bind(EVENT_DRY_RUN_ROLLED_BACK)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(true)
}

type ReconcileRow = (
    String,
    Option<Vec<u8>>,
    i64,
    Option<i64>,
    bool,
    i64,
    Option<i64>,
);

/// Advance the upgrade from durable state: re-arm readiness, resume a cutover, or
/// cut over once both consensus signals are in. A stopped readiness task is
/// recovered on the next tick (`readiness` keeps it to one attempt at a time).
async fn reconcile(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    readiness: &Arc<AtomicI64>,
    gcs_mode: bool,
) -> Result<(), Error> {
    if !gcs_mode {
        return Ok(());
    }
    let rows: Vec<ReconcileRow> = sqlx::query_as(
        "SELECT state, proposal_id, COALESCE(proposal_block, -1),
                gw_start_block, gw_dry_run_started, host_chain_id, start_block
           FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'
          ORDER BY host_chain_id",
    )
    .fetch_all(pool)
    .await?;
    let Some((state, proposal_id, proposal_block, gw_start_block, gw_dry_run_started, _, _)) =
        rows.first()
    else {
        return Ok(());
    };
    if rows.iter().any(
        |(row_state, row_id, row_block, row_gw_start, row_gw_started, _, _)| {
            row_state != state
                || row_id != proposal_id
                || row_block != proposal_block
                || row_gw_start != gw_start_block
                || row_gw_started != gw_dry_run_started
        },
    ) {
        return Err(Error::Payload(
            "in-progress GCS upgrade_state rows do not describe one proposal".to_string(),
        ));
    }
    let host_chains: Vec<(i64, i64)> = rows
        .iter()
        .map(|(_, _, _, _, _, chain_id, start)| {
            start.map(|start| (*chain_id, start)).ok_or_else(|| {
                Error::Payload(format!("GCS chain {chain_id} is missing start_block"))
            })
        })
        .collect::<Result<_, _>>()?;
    let (state, proposal_id, proposal_block, gw_start_block, gw_dry_run_started) = (
        state.clone(),
        proposal_id.clone(),
        *proposal_block,
        *gw_start_block,
        *gw_dry_run_started,
    );
    match state.as_str() {
        // Re-arm readiness (no-op if already running).
        "UpgradeActivated" => {
            if let (Some(proposal_id), Some(gw_start), false) =
                (proposal_id, gw_start_block, host_chains.is_empty())
            {
                spawn_gcs_dry_run_readiness(
                    pool,
                    cancel,
                    readiness,
                    GcsReadinessAttempt {
                        proposal_id,
                        proposal_block,
                        host_chains,
                        gw_start_block: gw_start,
                    },
                );
            }
        }
        "UpgradeAuthorized" => {
            info!("reconcile: GCS in UpgradeAuthorized — resuming cutover");
            execute_cutover_with_retry(pool, cancel).await?;
        }
        "DryRunStarted" => {
            // Restore an incomplete Gateway gate after a controller restart.
            if !gw_dry_run_started {
                if let (Some(proposal_id), Some(gw_start), false) =
                    (proposal_id, gw_start_block, host_chains.is_empty())
                {
                    spawn_gcs_dry_run_readiness(
                        pool,
                        cancel,
                        readiness,
                        GcsReadinessAttempt {
                            proposal_id,
                            proposal_block,
                            host_chains,
                            gw_start_block: gw_start,
                        },
                    );
                }
            }
            // Guarded on both latches, so it no-ops until they're set.
            try_cutover_if_consensus(pool, cancel).await?;
        }
        _ => {}
    }
    Ok(())
}

/// Per-track latch state for one upgrade attempt, for logging.
struct ConsensusTrackStatus {
    /// `chain_id=bool` per host chain, comma separated and ordered by chain id. `tracing` field
    /// names must be static, so per-chain latches cannot each be their own field.
    host: String,
    /// The Gateway track. Set on every chain row at once, so any row answers for all of them.
    gateway: bool,
}

/// Read the consensus-track latches for the given proposal attempt.
async fn consensus_track_status(
    pool: &Pool<Postgres>,
    proposal_id: Option<&[u8]>,
    proposal_block: i64,
) -> Result<ConsensusTrackStatus, Error> {
    let rows: Vec<(i64, bool, bool)> = sqlx::query_as(
        "SELECT host_chain_id, host_consensus_reached, gw_consensus_reached
           FROM upgrade_state
          WHERE stack_role = 'GCS'
            AND proposal_id IS NOT DISTINCT FROM $1
            AND COALESCE(proposal_block, -1) = $2
          ORDER BY host_chain_id",
    )
    .bind(proposal_id)
    .bind(proposal_block)
    .fetch_all(pool)
    .await?;

    Ok(ConsensusTrackStatus {
        host: rows
            .iter()
            .map(|(chain_id, host_reached, _)| format!("{chain_id}={host_reached}"))
            .collect::<Vec<_>>()
            .join(","),
        // Every chain row carries the same value, so ALL is the same as ANY here; ALL states
        // the intent and is safe on an empty set.
        gateway: !rows.is_empty() && rows.iter().all(|(_, _, gw_reached)| *gw_reached),
    })
}

/// Handle an `event_unanimity_consensus` notification. consensus-detector emits
/// this for TWO independent tracks, distinguished by the payload `chain_id`:
///   - a **host chain** (`chain_id` present in `upgrade_state`), over the
///     host-block state hashes, valid only for `block_height` within that
///     chain's `[start_block, end_block]` window; and
///   - the **Gateway** (any other `chain_id`), over the re-randomized input
///     ciphertexts, emitted per Gateway block.
///
/// Cutover requires unanimity on every host-chain track plus the Gateway track.
/// Each host latch lives on its chain row; the Gateway latch is repeated across
/// every row. The guarded `DryRunStarted` transition fires cutover exactly once.
///
/// Latches are *recorded* in either `UpgradeActivated` or `DryRunStarted`, but
/// cutover only *fires* in `DryRunStarted`. This matters because the Gateway
/// zkproof-worker is released independently during `UpgradeActivated` (before
/// the host stack, which unpauses ~`READINESS_CONFIRMATIONS` blocks later at
/// the `DryRunStarted` transition). So a Gateway anchor commonly arrives while
/// the row is still `UpgradeActivated`; if we ignored it there, the detector's
/// per-track anchor latches permanently and never re-emits, wedging cutover
/// forever. Recording `gw_consensus_reached` early (it survives the
/// state transition, which never resets latches — only activation does) lets
/// the later host anchor, arriving in `DryRunStarted`, complete the pair.
pub async fn handle_unanimity_consensus(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    gcs_mode: bool,
    raw_payload: &str,
) -> Result<(), Error> {
    if !gcs_mode {
        debug!(
            gcs_mode = false,
            "event_unanimity_consensus received — service not in gcs mode, ignoring"
        );
        return Ok(());
    }

    let payload: UnanimityConsensusPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    type GcsBaseRow = (String, Option<Vec<u8>>, i64, Option<i64>);
    let base: Option<GcsBaseRow> = sqlx::query_as(
        "SELECT state, proposal_id, COALESCE(proposal_block, -1), gw_start_block
           FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'
          ORDER BY host_chain_id
          LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    let Some((state, proposal_id, proposal_block, gw_start_block)) = base else {
        warn!(
            gcs_row_present = false,
            "event_unanimity_consensus: no in-progress GCS row in upgrade_state — skipping cutover"
        );
        return Ok(());
    };
    let state_eligible = state == "UpgradeActivated" || state == "DryRunStarted";
    let proposal_matches = proposal_id.as_deref() == Some(payload.proposal_id.as_slice());
    let attempt_matches = payload.proposal_block == Some(proposal_block);

    // Only the consensus-track latches, since those are what actually gate cutover: one host
    // track per host chain plus the single Gateway track. Read for the *stored* proposal, not
    // the payload's, so the line always describes the active attempt even when a stale event
    // arrives. State as of arrival — this event's own latch is set further down.
    //
    // Rejection reasons are not repeated here; each gate below warns with its own detail.
    let tracks = consensus_track_status(pool, proposal_id.as_deref(), proposal_block).await?;
    info!(
        host_tracks = tracks.host.as_str(),
        gw_track = tracks.gateway,
        "event_unanimity_consensus received — checking conditions for cutover execution"
    );

    if !state_eligible {
        warn!(
            state,
            "event_unanimity_consensus: GCS state is not UpgradeActivated/DryRunStarted — skipping cutover"
        );
        return Ok(());
    }
    if !proposal_matches {
        warn!("event_unanimity_consensus: proposal does not match — ignoring");
        return Ok(());
    }
    if !attempt_matches {
        warn!(
            payload_proposal_block = payload.proposal_block,
            proposal_block, "event_unanimity_consensus: proposal attempt does not match — ignoring"
        );
        return Ok(());
    }

    // Classify the event: it's a host track iff its chain_id is one of the
    // proposal's host chains; everything else is the Gateway track.
    let host_window: Option<(i64, i64)> = sqlx::query_as(
        "SELECT start_block, end_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND proposal_id = $1
            AND COALESCE(proposal_block, -1) = $2
            AND host_chain_id = $3",
    )
    .bind(&payload.proposal_id)
    .bind(proposal_block)
    .bind(payload.chain_id)
    .fetch_optional(pool)
    .await?;

    match host_window {
        // Host track: set only this chain's latch, and only within its window.
        Some((start, end)) if (start..=end).contains(&payload.block_height) => {
            let set = sqlx::query(
                "UPDATE upgrade_state
                    SET host_consensus_reached = TRUE, updated_at = NOW()
                  WHERE stack_role = 'GCS'
                    AND state IN ('UpgradeActivated', 'DryRunStarted')
                    AND proposal_id = $1
                    AND COALESCE(proposal_block, -1) = $2
                    AND host_chain_id = $3 AND NOT host_consensus_reached",
            )
            .bind(&payload.proposal_id)
            .bind(proposal_block)
            .bind(payload.chain_id)
            .execute(pool)
            .await?;
            if set.rows_affected() > 0 {
                info!(
                    chain_id = payload.chain_id,
                    block_height = payload.block_height,
                    start_block = start,
                    end_block = end,
                    proposal_id = %hex_encode(&payload.proposal_id),
                    "event_unanimity_consensus: host-track unanimity — host_consensus_reached set for chain"
                );
            }
        }
        Some((start, end)) => {
            warn!(
                chain_id = payload.chain_id,
                payload_block_height = payload.block_height,
                start_block = start,
                end_block = end,
                "event_unanimity_consensus: host block_height outside [start_block, end_block] — ignoring"
            );
            return Ok(());
        }
        // Not a host chain → Gateway track. Only counts at/after gw_start_block.
        None => match gw_start_block {
            Some(gw_start) if payload.block_height >= gw_start => {
                let set = sqlx::query(
                    "UPDATE upgrade_state SET gw_consensus_reached = TRUE, updated_at = NOW()
                      WHERE stack_role = 'GCS'
                        AND state IN ('UpgradeActivated', 'DryRunStarted')
                        AND proposal_id = $1
                        AND COALESCE(proposal_block, -1) = $2
                        AND NOT gw_consensus_reached",
                )
                .bind(&payload.proposal_id)
                .bind(proposal_block)
                .execute(pool)
                .await?;
                if set.rows_affected() > 0 {
                    info!(
                        chain_id = payload.chain_id,
                        block_height = payload.block_height,
                        gw_start_block = gw_start,
                        "event_unanimity_consensus: gateway-track unanimity — gw_consensus_reached set"
                    );
                }
            }
            Some(gw_start) => {
                warn!(
                    payload_block_height = payload.block_height,
                    gw_start_block = gw_start,
                    "event_unanimity_consensus: gateway block_height below gw_start_block — ignoring"
                );
                return Ok(());
            }
            None => {
                warn!(
                    payload_block_height = payload.block_height,
                    "event_unanimity_consensus: GCS row missing gw_start_block — ignoring gateway consensus"
                );
                return Ok(());
            }
        },
    }

    try_cutover_if_consensus(pool, cancel).await?;
    Ok(())
}

/// The dry-run timed out without agreement: reset the GCS stack so the upgrade
/// can be rerun. Rolls back the failed dry-run and wipes its schema. Only acts
/// while still dry-running, so a late timeout can't undo a cutover. BCS is
/// untouched.
pub async fn handle_unanimity_consensus_timeout(
    pool: &Pool<Postgres>,
    gcs_mode: bool,
    raw_payload: &str,
) -> Result<(), Error> {
    info!("event_unanimity_consensus_timeout received — evaluating rollback");

    if !gcs_mode {
        debug!("event_unanimity_consensus_timeout: service not in gcs mode, ignoring");
        return Ok(());
    }

    let payload: UnanimityConsensusPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    let Some(proposal_block) = payload.proposal_block else {
        warn!(
            proposal_id = %hex_encode(&payload.proposal_id),
            "event_unanimity_consensus_timeout: missing proposal_block — ignoring legacy timeout"
        );
        return Ok(());
    };

    if rollback_dry_run(
        pool,
        &payload.proposal_id,
        proposal_block,
        payload.block_height,
    )
    .await?
    {
        warn!(
            chain_id = payload.chain_id,
            block_height = payload.block_height,
            "event_unanimity_consensus_timeout: rolled back GCS dry-run — schema reset, upgrade may be rerun"
        );
    } else {
        info!(
            chain_id = payload.chain_id,
            block_height = payload.block_height,
            "event_unanimity_consensus_timeout: GCS row not in a rollback-eligible state — skipping rollback"
        );
    }

    Ok(())
}

/// Lowercase hex without `0x` prefix; only used for log lines, kept private
/// to avoid pulling in another crate for a few bytes' worth of formatting.
fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Main service loop: listens on both channels and dispatches handlers.
///
/// Returns when the cancel token fires. Transient errors are logged and the
/// loop keeps running; a fatal listener error bubbles up.
pub async fn run(
    config: Config,
    pool: Pool<Postgres>,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    info!(
        service_name = %config.service_name,
        gcs_mode = config.gcs_mode,
        "Starting upgrade-controller"
    );

    // Create the GCS schema (empty duplicates of every BCS-owned data table)
    // once at startup, gated on gcs_mode. The GCS services begin tailing the
    // chain in paused mode before any activation, writing via
    // `search_path = gcs,public`; the `gcs.*` tables must already exist or those
    // writes would silently fall back to the live `public` schema. Idempotent —
    // only the GCS stack owns this schema; BCS leaves it untouched.
    if config.gcs_mode {
        create_gcs_schema(&pool).await?;
    }

    let mut listener = PgListener::connect_with(&pool).await?;
    let channels = [
        UPGRADE_ACTIVATED_CHANNEL,
        UNANIMITY_CONSENSUS_CHANNEL,
        UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL,
    ];
    listener.listen_all(channels).await?;
    info!(?channels, "Listening for notifications");

    let readiness = Arc::new(AtomicI64::new(NO_READINESS_ATTEMPT));

    // Boot reconcile: recover an upgrade whose NOTIFY was missed while down (LISTEN already registered).
    if let Err(e) = reconcile(&pool, &cancel, &readiness, config.gcs_mode).await {
        error!(error = %e, "boot reconcile failed");
    }

    let mut poll = tokio::time::interval(config.poll_interval);
    // First tick fires immediately; skip it so we don't double-trigger on startup.
    poll.tick().await;

    loop {
        select! {
            _ = cancel.cancelled() => {
                info!("Cancellation received — upgrade-controller shutting down");
                return Ok(());
            }
            recv = listener.recv() => {
                match recv {
                    Ok(notification) => {
                        let channel = notification.channel();
                        let payload = notification.payload();
                        debug!(channel, payload, "notification received");

                        let result = match channel {
                            UPGRADE_ACTIVATED_CHANNEL => {
                                handle_upgrade_activated(&pool, &cancel, &readiness, config.gcs_mode, payload).await
                            }
                            UNANIMITY_CONSENSUS_CHANNEL => {
                                // Emitted by consensus-detector when every operator publishes
                                // the same state commitment at the upgrade's end_block.
                                handle_unanimity_consensus(&pool, &cancel, config.gcs_mode, payload).await
                            }
                            UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL => {
                                // Window never reached unanimity — roll back the dry-run.
                                handle_unanimity_consensus_timeout(&pool, config.gcs_mode, payload)
                                    .await
                            }
                            other => {
                                warn!(channel = other, "ignoring notification on unexpected channel");
                                Ok(())
                            }
                        };

                        if let Err(e) = result {
                            error!(channel, error = %e, "failed to handle notification");
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "listener recv error; sleeping before retry");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            _ = poll.tick() => {
                // Fallback for a missed NOTIFY: re-derive the next step from the row.
                if let Err(e) = reconcile(&pool, &cancel, &readiness, config.gcs_mode).await {
                    error!(error = %e, "poll reconcile failed");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_upgrade_activated_payload() {
        let json = r#"{
            "proposal_id": "0x01ab",
            "chain_id": 12345,
            "start_block": 100,
            "end_block": 200,
            "gw_start_block": 150
        }"#;
        let p: UpgradeActivatedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.proposal_id, "0x01ab");
        assert_eq!(p.chain_id, 12345);
        assert_eq!(p.start_block, 100);
        assert_eq!(p.end_block, 200);
        assert_eq!(p.gw_start_block, 150);
        assert!(p.version.is_none());
    }

    #[test]
    fn parses_unanimity_consensus_payload() {
        let json = r#"{
            "proposal_id": [2],
            "proposal_block": 10,
            "chain_id": 12345,
            "block_height": 200,
            "block_hash": "0xabc0000000000000000000000000000000000000000000000000000000000001"
        }"#;
        let p: UnanimityConsensusPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.proposal_id, vec![2]);
        assert_eq!(p.proposal_block, Some(10));
        assert_eq!(p.chain_id, 12345);
        assert_eq!(p.block_height, 200);
        assert_eq!(
            p.block_hash,
            "0xabc0000000000000000000000000000000000000000000000000000000000001"
        );
    }

    /// Every statement in the synthetic cleanups must be valid against the *real* GCS schema.
    ///
    /// This is the test that matters: the deletes are `format!`-built and reference eleven
    /// by-handle tables plus three keyed on the transaction, and a single wrong column name is a
    /// hard SQL error that aborts the whole cutover. Only executing them against actual cloned
    /// tables proves the names are right. It already caught `transactions`, whose key is `id` and
    /// not `transaction_id` like every other table in the sweep.
    #[tokio::test]
    async fn synthetic_cleanups_run_against_the_real_gcs_schema() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");

        let txn_hash = vec![0xABu8; 32];
        let handle = vec![0xCDu8; 32];
        let chain_id = vec![0xEFu8; 32];

        // A GCS window that reached its injection block, so the cleanup has a marker to act on.
        sqlx::query(
            "INSERT INTO upgrade_state (
                 stack_role, state, status, proposal_id, version, start_block, end_block,
                 host_chain_id, proposal_block, synthetic_txn_hash, updated_at
             ) VALUES ('GCS', 'UpgradeAuthorized', 'in_progress', $1, '0.15.0', 100, 200,
                       12345, 100, $2, NOW())",
        )
        .bind(vec![7u8; 32])
        .bind(&txn_hash)
        .execute(&pool)
        .await
        .expect("seed upgrade_state");

        // Synthetic work in the GCS schema, reachable exactly the way the cleanup walks it:
        // computations -> output_handle -> the by-handle tables, plus a dependence chain.
        let seed = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.dependence_chain
                  (dependence_chain_id, status, last_updated_at)
                  VALUES ($3, 'processed', NOW());
             INSERT INTO {GCS_SCHEMA_QUOTED}.computations
                  (tenant_id, output_handle, transaction_id, dependencies, fhe_operation,
                   is_completed, is_scalar, host_chain_id, dependence_chain_id)
                  VALUES (1, $2, $1, ARRAY[$2], 1, true, false, 12345, $3);
             INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertexts
                  (handle, ciphertext, ciphertext_version, ciphertext_type)
                  VALUES ($2, '\\x01'::bytea, 0, 5);
             INSERT INTO {GCS_SCHEMA_QUOTED}.input_handles (handle, block_number)
                  VALUES ($2, 101);
             INSERT INTO {GCS_SCHEMA_QUOTED}.transactions (id, chain_id, block_number)
                  VALUES ($1, 12345, 101);"
        );
        sqlx::raw_sql(
            &seed
                .replace("$1", &format!("'\\x{}'::bytea", hex_encode(&txn_hash)))
                .replace("$2", &format!("'\\x{}'::bytea", hex_encode(&handle)))
                .replace("$3", &format!("'\\x{}'::bytea", hex_encode(&chain_id))),
        )
        .execute(&pool)
        .await
        .expect("seed synthetic work");

        // Run both cleanups in one transaction, exactly as execute_cutover does — this also
        // exercises the two temp tables coexisting in the same transaction.
        let mut tx = pool.begin().await.expect("begin");
        delete_gcs_synthetic_ops(&mut tx)
            .await
            .expect("delete_gcs_synthetic_ops");
        delete_gcs_synthetic_inputs(&mut tx)
            .await
            .expect("delete_gcs_synthetic_inputs");
        tx.commit().await.expect("commit");

        // Nothing synthetic may survive into the merge.
        for (table, column, value) in [
            ("computations", "transaction_id", &txn_hash),
            ("transactions", "id", &txn_hash),
            ("ciphertexts", "handle", &handle),
            ("input_handles", "handle", &handle),
            ("dependence_chain", "dependence_chain_id", &chain_id),
        ] {
            let sql =
                format!("SELECT COUNT(*) FROM {GCS_SCHEMA_QUOTED}.{table} WHERE {column} = $1");
            let remaining: i64 = sqlx::query_scalar(&sql)
                .bind(value)
                .fetch_one(&pool)
                .await
                .unwrap_or_else(|e| panic!("count {table}: {e}"));
            assert_eq!(remaining, 0, "{table}.{column} still holds synthetic rows");
        }

        // The marker is cleared, so a later attempt cannot match this window's work.
        let marker: Option<Vec<u8>> = sqlx::query_scalar(
            "SELECT synthetic_txn_hash FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("read marker");
        assert!(marker.is_none(), "synthetic_txn_hash was not cleared");
    }

    /// With no marker recorded the cleanup must be a clean no-op, not an error: a window that
    /// never reached `start_block + 1` has no synthetic work at all.
    #[tokio::test]
    async fn synthetic_ops_cleanup_without_marker_is_a_noop() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;

        let mut tx = pool.begin().await.expect("begin");
        delete_gcs_synthetic_ops(&mut tx)
            .await
            .expect("no-op must not error");
        tx.commit().await.expect("commit");
    }

    /// A trigger can only be created on a table that exists in the GCS schema, i.e. one
    /// classified `duplicated = true`. Reclassifying either table would otherwise turn
    /// `create_gcs_tables` into a hard error at activation time.
    #[test]
    fn gcs_notify_trigger_tables_are_duplicated() {
        for (table, trigger, _) in GCS_NOTIFY_TRIGGERS {
            let entry = crate::coprocessor_tables::COPROCESSOR_TABLES
                .iter()
                .find(|t| t.name == *table)
                .unwrap_or_else(|| {
                    panic!("trigger {trigger} targets {table}, which is not a COPROCESSOR_TABLES entry")
                });
            assert!(
                entry.duplicated,
                "trigger {trigger} targets {table}, which is not duplicated into the GCS schema"
            );
        }
    }

    #[test]
    fn hex_encode_formats_bytes() {
        let bytes = vec![0x00, 0x01, 0xab, 0xff];
        assert_eq!(hex_encode(&bytes), "0001abff");
    }

    /// Regression test for the cutover merge `ON CONFLICT` targets drifting away
    /// from the live primary keys. After `collapse_overlapping_unique_keys`, the
    /// PKs on `public.ciphertexts` and `public.ciphertext_digest` became
    /// tenant-free (`(handle, ciphertext_version)` and `(handle)`), but the
    /// `execute_cutover` merges still referenced the old tenant-prefixed columns,
    /// so Postgres rejected them at planning time with "there is no unique or
    /// exclusion constraint matching the ON CONFLICT specification" — failing
    /// every cutover. The merge `ON CONFLICT` clauses are planned even over empty
    /// gcs tables, so this exercises all three merges without seeding rows (which
    /// also keeps the test stable as the merged tables' columns evolve).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn execute_cutover_merges_match_live_unique_keys() {
        use sqlx::Row;

        let (_instance, pool) = test_pool().await;

        // The GCS row's `version` drives the cutover's stack_version bump.
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id, updated_at
            )
            VALUES ('GCS', 'UpgradeAuthorized', 'in_progress', $1, 'v0.15',
                    100, 200, 1, 1, NOW())
            ON CONFLICT (stack_role, host_chain_id) DO UPDATE
            SET state = EXCLUDED.state, status = EXCLUDED.status,
                version = EXCLUDED.version, updated_at = NOW()
            "#,
        )
        .bind(&[0x02u8][..])
        .execute(&pool)
        .await
        .expect("seed GCS row");

        create_gcs_schema(&pool).await.expect("create gcs schema");

        // The bug surfaced exactly here: a planning-time ON CONFLICT error.
        execute_cutover(&pool).await.expect("cutover succeeds");

        // versioning bumped to the new stack version inside the cutover tx.
        let (sv,): (String,) =
            sqlx::query_as("SELECT stack_version FROM versioning WHERE singleton = TRUE")
                .fetch_one(&pool)
                .await
                .expect("versioning row");
        assert_eq!(sv, "v0.15", "cutover should bump versioning.stack_version");

        // GCS row flipped LIVE and the gcs schema was dropped.
        let row = sqlx::query("SELECT state FROM upgrade_state WHERE stack_role = 'GCS'")
            .fetch_one(&pool)
            .await
            .expect("GCS row");
        assert_eq!(row.try_get::<String, _>("state").unwrap(), "LIVE");

        let (schema_exists,): (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
        )
        .bind(fhevm_engine_common::database::GCS_SCHEMA)
        .fetch_one(&pool)
        .await
        .expect("schema lookup");
        assert!(!schema_exists, "cutover should drop the gcs schema");
    }

    async fn test_pool() -> (test_harness::instance::DBInstance, Pool<Postgres>) {
        use sqlx::postgres::PgPoolOptions;
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::WithKeysNoSns)
            .await
            .expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(instance.db_url())
            .await
            .expect("pool");
        (instance, pool)
    }

    fn timeout_payload() -> String {
        serde_json::json!({ "proposal_id": [2], "proposal_block": 10, "chain_id": 1_i64, "block_height": 200_i64, "block_hash": "0x00" })
            .to_string()
    }

    /// Seed one GCS proposal row with all latches set.
    async fn seed_gcs_row(pool: &Pool<Postgres>, state: &str, status: &str) {
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id,
                host_consensus_reached, gw_consensus_reached, gw_dry_run_started,
                proposal_block, updated_at
            )
            VALUES ('GCS', $1, $2, $3, 'v0.15', 100, 200, 1, 1,
                    TRUE, TRUE, TRUE, 10, NOW())
            ON CONFLICT (stack_role, host_chain_id) DO UPDATE
            SET state = EXCLUDED.state, status = EXCLUDED.status,
                proposal_id = EXCLUDED.proposal_id,
                start_block = EXCLUDED.start_block,
                end_block = EXCLUDED.end_block,
                host_chain_id = EXCLUDED.host_chain_id,
                host_consensus_reached = EXCLUDED.host_consensus_reached,
                gw_consensus_reached   = EXCLUDED.gw_consensus_reached,
                gw_dry_run_started     = EXCLUDED.gw_dry_run_started,
                proposal_block         = EXCLUDED.proposal_block,
                updated_at = NOW()
            "#,
        )
        .bind(state)
        .bind(status)
        .bind(&[0x02u8][..])
        .execute(pool)
        .await
        .expect("seed GCS row");
    }

    /// A `gcs` table NOT in `COPROCESSOR_TABLES`: only `DROP SCHEMA … CASCADE`
    /// removes it (the recreate won't restore it), so it proves the reset ran.
    async fn create_marker(pool: &Pool<Postgres>) {
        sqlx::query(&format!(
            "CREATE TABLE {GCS_SCHEMA_QUOTED}.rollback_marker (x int)"
        ))
        .execute(pool)
        .await
        .expect("create marker");
    }

    async fn marker_exists(pool: &Pool<Postgres>) -> bool {
        let (exists,): (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables
                             WHERE table_schema = $1 AND table_name = 'rollback_marker')",
        )
        .bind(fhevm_engine_common::database::GCS_SCHEMA)
        .fetch_one(pool)
        .await
        .expect("marker lookup");
        exists
    }

    /// A timeout mid dry-run resets the schema and flips the GCS row to PAUSED/failed; a second timeout is a no-op.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_rolls_back_dry_run_and_is_idempotent() {
        use sqlx::Row;
        use tokio::time::{sleep, timeout, Duration};

        let (_instance, pool) = test_pool().await;

        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await;
        sqlx::query("UPDATE upgrade_state SET gw_dry_run_started = FALSE")
            .execute(&pool)
            .await
            .expect("reset gateway flag");
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;
        assert!(marker_exists(&pool).await, "marker present before rollback");

        let host_pool = pool.clone();
        let host_readiness = tokio::spawn(async move {
            wait_until_dry_run_ready(host_pool, CancellationToken::new(), &[0x02], 10, 1, 100).await
        });
        let gateway_pool = pool.clone();
        let gateway_readiness = tokio::spawn(async move {
            wait_until_gw_dry_run_ready(gateway_pool, CancellationToken::new(), &[0x02], 10, 1)
                .await
        });
        sleep(Duration::from_millis(200)).await;

        let payload = timeout_payload();

        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("rollback ok");

        assert!(!timeout(Duration::from_secs(10), host_readiness)
            .await
            .expect("host readiness did not stop")
            .expect("host readiness task panicked")
            .expect("host readiness failed"));
        assert!(!timeout(Duration::from_secs(10), gateway_readiness)
            .await
            .expect("gateway readiness did not stop")
            .expect("gateway readiness task panicked")
            .expect("gateway readiness failed"));

        // Marker gone and schema dropped
        assert!(
            !marker_exists(&pool).await,
            "rollback should DROP SCHEMA CASCADE, removing the marker"
        );
        // the duplicated tables recreated empty
        let (ct_exists,): (bool,) = sqlx::query_as(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables
                             WHERE table_schema = $1 AND table_name = 'computations')",
        )
        .bind(fhevm_engine_common::database::GCS_SCHEMA)
        .fetch_one(&pool)
        .await
        .expect("computations lookup");
        assert!(ct_exists, "rollback should recreate the empty gcs schema");

        // Rerunnable state: PAUSED/failed.
        let row = sqlx::query(
            "SELECT state, status, last_error, host_consensus_reached,
                    gw_consensus_reached, gw_dry_run_started
               FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("GCS row");
        assert_eq!(row.try_get::<String, _>("state").unwrap(), "PAUSED");
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "failed");
        assert_eq!(
            row.try_get::<String, _>("last_error").unwrap(),
            "unanimity_consensus_timeout"
        );
        assert!(!row.try_get::<bool, _>("host_consensus_reached").unwrap());
        assert!(!row.try_get::<bool, _>("gw_consensus_reached").unwrap());
        assert!(!row.try_get::<bool, _>("gw_dry_run_started").unwrap());

        // Second timeout is a no-op: the marker survives (no second reset).
        create_marker(&pool).await;
        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("second timeout no-op");
        assert!(
            marker_exists(&pool).await,
            "a duplicate timeout must not reset the schema again"
        );
    }

    /// A late timeout must never undo a cutover: rollback is refused once the row is `UpgradeAuthorized`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_does_not_undo_authorized_cutover() {
        use sqlx::Row;

        let (_instance, pool) = test_pool().await;

        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;

        let payload = timeout_payload();

        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("handler ok");

        assert!(
            marker_exists(&pool).await,
            "a timeout must not reset the schema once the row is UpgradeAuthorized"
        );
        let row = sqlx::query("SELECT state, status FROM upgrade_state WHERE stack_role = 'GCS'")
            .fetch_one(&pool)
            .await
            .expect("GCS row");
        assert_eq!(
            row.try_get::<String, _>("state").unwrap(),
            "UpgradeAuthorized",
            "the FSM state must be left intact"
        );
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "in_progress");
    }

    /// A timeout on a BCS-mode controller is ignored (BCS never left LIVE).
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_bcs_mode_is_noop() {
        let (_instance, pool) = test_pool().await;

        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await;
        let payload = timeout_payload();

        handle_unanimity_consensus_timeout(&pool, false, &payload)
            .await
            .expect("bcs no-op");

        let (state,): (String,) =
            sqlx::query_as("SELECT state FROM upgrade_state WHERE stack_role = 'GCS'")
                .fetch_one(&pool)
                .await
                .expect("GCS row");
        assert_eq!(
            state, "DryRunStarted",
            "BCS-mode timeout must not mutate state"
        );
    }

    async fn gcs_state(pool: &Pool<Postgres>) -> (String, String) {
        sqlx::query_as(
            "SELECT state, status FROM upgrade_state
              WHERE stack_role = 'GCS'
              ORDER BY host_chain_id
              LIMIT 1",
        )
        .fetch_one(pool)
        .await
        .expect("GCS row")
    }

    /// Boot/poll reconcile resumes a cutover interrupted in `UpgradeAuthorized`.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn reconcile_resumes_interrupted_cutover() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;
        create_gcs_schema(&pool).await.expect("create gcs schema");

        reconcile(
            &pool,
            &CancellationToken::new(),
            &Arc::new(AtomicI64::new(NO_READINESS_ATTEMPT)),
            true,
        )
        .await
        .expect("reconcile");

        assert_eq!(gcs_state(&pool).await, ("LIVE".into(), "completed".into()));
        let (sv,): (String,) =
            sqlx::query_as("SELECT stack_version FROM versioning WHERE singleton = TRUE")
                .fetch_one(&pool)
                .await
                .expect("versioning");
        assert_eq!(sv, "v0.15");
    }

    /// Reconcile cuts over on both latches when the unanimity NOTIFY was missed.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn reconcile_cuts_over_when_both_latches_set() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await; // latches TRUE
        create_gcs_schema(&pool).await.expect("create gcs schema");

        reconcile(
            &pool,
            &CancellationToken::new(),
            &Arc::new(AtomicI64::new(NO_READINESS_ATTEMPT)),
            true,
        )
        .await
        .expect("reconcile");

        assert_eq!(gcs_state(&pool).await, ("LIVE".into(), "completed".into()));
    }

    /// Seed one GCS chain row with an explicit host latch (aligned window
    /// 100..200, proposal 0x02).
    async fn seed_gcs_chain(
        pool: &Pool<Postgres>,
        chain_id: i64,
        host_reached: bool,
        gw_reached: bool,
    ) {
        sqlx::query(
            r#"
            INSERT INTO upgrade_state (
                stack_role, state, status, proposal_id, version,
                start_block, end_block, gw_start_block, host_chain_id,
                host_consensus_reached, gw_consensus_reached, gw_dry_run_started,
                proposal_block, updated_at
            )
            VALUES ('GCS', 'DryRunStarted', 'in_progress', $1, 'v0.15',
                    100, 200, 1, $2, $3, $4, TRUE, 10, NOW())
            ON CONFLICT (stack_role, host_chain_id) DO UPDATE
            SET state = EXCLUDED.state,
                status = EXCLUDED.status,
                proposal_id = EXCLUDED.proposal_id,
                proposal_block = EXCLUDED.proposal_block,
                start_block = EXCLUDED.start_block,
                end_block = EXCLUDED.end_block,
                host_consensus_reached = EXCLUDED.host_consensus_reached,
                gw_consensus_reached = EXCLUDED.gw_consensus_reached,
                updated_at = NOW()
            "#,
        )
        .bind(&[0x02u8][..])
        .bind(chain_id)
        .bind(host_reached)
        .bind(gw_reached)
        .execute(pool)
        .await
        .expect("seed GCS chain row");
    }

    fn consensus_payload(chain_id: i64, block_height: i64) -> String {
        serde_json::json!({
            "proposal_id": [2], "proposal_block": 10, "chain_id": chain_id,
            "block_height": block_height, "block_hash": "0x00"
        })
        .to_string()
    }

    async fn host_reached(pool: &Pool<Postgres>, chain_id: i64) -> bool {
        let (v,): (bool,) = sqlx::query_as(
            "SELECT host_consensus_reached FROM upgrade_state
              WHERE stack_role = 'GCS' AND host_chain_id = $1",
        )
        .bind(chain_id)
        .fetch_one(pool)
        .await
        .expect("host latch");
        v
    }

    /// Multi-chain: cutover is withheld until *every* host chain has
    /// reached consensus, then fires exactly once for all chains.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn try_cutover_defers_until_all_host_chains_reach_consensus() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");

        // Two chains, gateway agreed on both, but only chain 1 has host consensus.
        seed_gcs_chain(&pool, 1, true, true).await;
        seed_gcs_chain(&pool, 2, false, true).await;

        try_cutover_if_consensus(&pool, &CancellationToken::new())
            .await
            .expect("try cutover");
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into()),
            "cutover must defer while a host chain is missing consensus"
        );

        // Chain 2 now reaches consensus → cutover fires for the whole proposal.
        seed_gcs_chain(&pool, 2, true, true).await;
        try_cutover_if_consensus(&pool, &CancellationToken::new())
            .await
            .expect("try cutover 2");

        assert_eq!(gcs_state(&pool).await, ("LIVE".into(), "completed".into()));
    }

    /// Multi-chain: a host anchor for one chain sets ONLY that chain's
    /// latch — a second host chain must not be marked as reached.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn host_unanimity_sets_only_the_matching_chain_latch() {
        let (_instance, pool) = test_pool().await;

        seed_gcs_chain(&pool, 1, false, false).await;
        seed_gcs_chain(&pool, 2, false, false).await;

        // Host-track anchor for chain 1, block within [100, 200].
        handle_unanimity_consensus(
            &pool,
            &CancellationToken::new(),
            true,
            &consensus_payload(1, 150),
        )
        .await
        .expect("handle chain 1");

        assert!(host_reached(&pool, 1).await, "chain 1 latch must be set");
        assert!(
            !host_reached(&pool, 2).await,
            "chain 2 latch must NOT be set by a chain-1 anchor"
        );
        // Not all chains agreed → still dry-running.
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into())
        );
    }

    /// Multi-chain: a gateway anchor sets the proposal-level Gateway
    /// latch on every proposal row without changing any host latch.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn gateway_unanimity_sets_latch_on_all_chain_rows() {
        let (_instance, pool) = test_pool().await;

        seed_gcs_chain(&pool, 1, false, false).await;
        seed_gcs_chain(&pool, 2, false, false).await;

        // Gateway chain id (999) is not a host chain; block >= gw_start_block (1).
        handle_unanimity_consensus(
            &pool,
            &CancellationToken::new(),
            true,
            &consensus_payload(999, 5),
        )
        .await
        .expect("handle gateway");

        let gw: bool = sqlx::query_scalar(
            "SELECT COALESCE(BOOL_AND(gw_consensus_reached), FALSE)
               FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("gw latch");
        assert!(gw, "gateway latch set on every proposal row");
        assert!(!host_reached(&pool, 1).await);
        assert!(!host_reached(&pool, 2).await);
    }

    /// Multi-chain: a consensus timeout rolls back the proposal and
    /// resets every per-chain host latch.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn timeout_rolls_back_all_chain_rows() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");

        seed_gcs_chain(&pool, 1, true, true).await;
        seed_gcs_chain(&pool, 2, false, true).await;

        handle_unanimity_consensus_timeout(&pool, true, &consensus_payload(1, 200))
            .await
            .expect("rollback");

        let row: (String, String, bool) = sqlx::query_as(
            "SELECT state, status, gw_consensus_reached
               FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("header");
        assert_eq!(row.0, "PAUSED");
        assert_eq!(row.1, "failed");
        assert!(!row.2, "gateway latch reset on rollback");
        let host_latches: Vec<bool> = sqlx::query_scalar(
            "SELECT host_consensus_reached FROM upgrade_state
              WHERE stack_role = 'GCS' ORDER BY host_chain_id",
        )
        .fetch_all(&pool)
        .await
        .expect("host latches");
        assert_eq!(host_latches, vec![false, false]);
    }

    /// A restart after the host gate finishes must restore an incomplete
    /// Gateway gate instead of leaving the zkproof-worker paused forever.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn reconcile_rearms_gateway_gate_from_dry_run_started() {
        use tokio::time::{sleep, timeout, Duration};

        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await;
        sqlx::query(
            "UPDATE upgrade_state
                SET host_consensus_reached = FALSE,
                    gw_consensus_reached = FALSE,
                    gw_dry_run_started = FALSE
              WHERE stack_role = 'GCS'",
        )
        .execute(&pool)
        .await
        .expect("reset latches and Gateway gate");
        create_gcs_schema(&pool).await.expect("create gcs schema");
        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.gw_listener_last_block
                    (dummy_id, last_block_num)
             VALUES (TRUE, 1)"
        ))
        .execute(&pool)
        .await
        .expect("seed Gateway watermark");

        let cancel = CancellationToken::new();
        reconcile(
            &pool,
            &cancel,
            &Arc::new(AtomicI64::new(NO_READINESS_ATTEMPT)),
            true,
        )
        .await
        .expect("reconcile");

        timeout(Duration::from_secs(30), async {
            loop {
                let (started,): (bool,) = sqlx::query_as(
                    "SELECT gw_dry_run_started
                       FROM upgrade_state WHERE stack_role = 'GCS'",
                )
                .fetch_one(&pool)
                .await
                .expect("Gateway gate");
                if started {
                    break;
                }
                sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("Gateway gate was not restored");
        cancel.cancel();
    }

    /// A BCS-mode controller never reconciles GCS state.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn reconcile_bcs_mode_is_noop() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;

        reconcile(
            &pool,
            &CancellationToken::new(),
            &Arc::new(AtomicI64::new(NO_READINESS_ATTEMPT)),
            false,
        )
        .await
        .expect("reconcile");

        assert_eq!(gcs_state(&pool).await.0, "UpgradeAuthorized");
    }

    /// A stale timeout from a different window must not roll back the current dry-run.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_ignores_other_window() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await; // end_block = 200
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;

        // Timeout for a different window (block_height 999 != end_block 200).
        let payload =
            serde_json::json!({ "proposal_id": [2], "proposal_block": 10, "chain_id": 1_i64, "block_height": 999_i64, "block_hash": "0x00" })
                .to_string();
        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("handler ok");

        assert!(
            marker_exists(&pool).await,
            "a timeout for another window must not reset the schema"
        );
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into())
        );
    }

    /// Reusing a caller-supplied proposal id and numeric window must not let a
    /// delayed timeout from the prior on-chain attempt roll back the retry.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_ignores_other_attempt() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await; // proposal_block = 10
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;

        let payload = serde_json::json!({
            "proposal_id": [2],
            "proposal_block": 9,
            "chain_id": 1_i64,
            "block_height": 200_i64,
            "block_hash": "0x00"
        })
        .to_string();
        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("handler ok");

        assert!(
            marker_exists(&pool).await,
            "a timeout for another attempt must not reset the schema"
        );
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into())
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_timeout_ignores_other_proposal() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;

        let payload = serde_json::json!({
            "proposal_id": [0x99],
            "proposal_block": 10,
            "chain_id": 1_i64,
            "block_height": 200_i64,
            "block_hash": "0x00"
        })
        .to_string();
        handle_unanimity_consensus_timeout(&pool, true, &payload)
            .await
            .expect("handler ok");

        assert!(marker_exists(&pool).await);
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into())
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handle_consensus_ignores_other_proposal() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await;
        sqlx::query(
            "UPDATE upgrade_state
                SET host_consensus_reached = FALSE, gw_consensus_reached = FALSE
              WHERE stack_role = 'GCS'",
        )
        .execute(&pool)
        .await
        .expect("reset latches");

        let payload = serde_json::json!({
            "proposal_id": [0x99],
            "proposal_block": 10,
            "chain_id": 1_i64,
            "block_height": 150_i64,
            "block_hash": "0x00"
        })
        .to_string();
        handle_unanimity_consensus(&pool, &CancellationToken::new(), true, &payload)
            .await
            .expect("handler ok");

        let latches: (bool, bool) = sqlx::query_as(
            "SELECT host_consensus_reached, gw_consensus_reached
               FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_one(&pool)
        .await
        .expect("latches");
        assert_eq!(latches, (false, false));
        assert!(!host_reached(&pool, 1).await);
    }

    /// Blue's in-window rows for one handle, dual-written to the legacy and the branch
    /// form of every table cutover clears. `producer_block_hash` is non-empty so the
    /// rows carry a real branch context (a branchless `''` row is the legacy mirror and
    /// is deliberately left alone).
    async fn seed_bcs_handle(pool: &Pool<Postgres>, handle: &[u8], block: i64, ct_byte: u8) {
        let branch = &[0x11u8; 32][..];
        sqlx::query(
            "INSERT INTO computations
                (output_handle, dependencies, fhe_operation,
                 is_scalar, is_completed, host_chain_id, block_number)
             VALUES ($1, ARRAY[]::bytea[], 0, false, TRUE, 1, $2)",
        )
        .bind(handle)
        .bind(block)
        .execute(pool)
        .await
        .expect("seed computation");
        sqlx::query(
            "INSERT INTO computations_branch
                (output_handle, dependencies, fhe_operation,
                 is_scalar, is_completed, host_chain_id, block_number, producer_block_hash)
             VALUES ($1, ARRAY[]::bytea[], 0, false, TRUE, 1, $2, $3)",
        )
        .bind(handle)
        .bind(block)
        .bind(branch)
        .execute(pool)
        .await
        .expect("seed computations_branch");
        sqlx::query(
            "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
             VALUES ($1, $2, 0, 0)",
        )
        .bind(handle)
        .bind(&[ct_byte][..])
        .execute(pool)
        .await
        .expect("seed ciphertext");
        // `block_number` is NOT NULL exactly for a non-empty producer_block_hash
        // (`ciphertexts_branch_producer_block_number_check`).
        sqlx::query(
            "INSERT INTO ciphertexts_branch
                (handle, ciphertext, ciphertext_version, ciphertext_type,
                 producer_block_hash, block_number)
             VALUES ($1, $2, 0, 0, $3, $4)",
        )
        .bind(handle)
        .bind(&[ct_byte][..])
        .bind(branch)
        .bind(block)
        .execute(pool)
        .await
        .expect("seed ciphertexts_branch");
        sqlx::query(
            "INSERT INTO pbs_computations
                (handle, is_completed, host_chain_id, block_number)
             VALUES ($1, TRUE, 1, $2)",
        )
        .bind(handle)
        .bind(block)
        .execute(pool)
        .await
        .expect("seed pbs_computation");
        sqlx::query(
            "INSERT INTO pbs_computations_branch
                (handle, is_completed, host_chain_id, block_number,
                 producer_block_hash, block_hash)
             VALUES ($1, TRUE, 1, $2, $3, $3)",
        )
        .bind(handle)
        .bind(block)
        .bind(branch)
        .execute(pool)
        .await
        .expect("seed pbs_computations_branch");
        // After the pbs branch row: its INSERT trigger rebuilds digest branch rows from
        // `public.ciphertext_digest`, which this handle has none of, so it is a no-op.
        sqlx::query(
            "INSERT INTO ciphertext_digest_branch
                (handle, host_chain_id, block_number, producer_block_hash, block_hash)
             VALUES ($1, 1, $2, $3, $3)",
        )
        .bind(handle)
        .bind(block)
        .bind(branch)
        .execute(pool)
        .await
        .expect("seed ciphertext_digest_branch");
    }

    /// Every table cutover clears, in both forms: legacy first, then `*_branch`.
    async fn leftover_counts(pool: &Pool<Postgres>, handle: &[u8]) -> [i64; 8] {
        let row: (i64, i64, i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM ciphertexts WHERE handle = $1),
                    (SELECT COUNT(*) FROM computations WHERE output_handle = $1),
                    (SELECT COUNT(*) FROM pbs_computations WHERE handle = $1),
                    (SELECT COUNT(*) FROM ciphertexts_branch WHERE handle = $1),
                    (SELECT COUNT(*) FROM computations_branch WHERE output_handle = $1),
                    (SELECT COUNT(*) FROM pbs_computations_branch WHERE handle = $1),
                    (SELECT COUNT(*) FROM ciphertext_digest_branch
                      WHERE handle = $1 AND block_number IS NOT NULL),
                    (SELECT COUNT(*) FROM ciphertext_digest_branch
                      WHERE handle = $1 AND block_number IS NULL)",
        )
        .bind(handle)
        .fetch_one(pool)
        .await
        .expect("leftover counts");
        [row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7]
    }

    /// Cutover backfill, eager case: every in-window row blue wrote (block 160 <
    /// `end_block` 200) is deleted — ciphertext, `computations`, `pbs_computations`, and
    /// the `*_branch` form of each — and only what green also holds in `gcs.*` comes back
    /// with the merge. A handle GCS never ingested therefore ends up gone, to be
    /// re-derived from the chain.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_backfill_deletes_bcs_leftovers_eager() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await; // start=100, end=200, chain=1

        let leftover = &[0xAAu8; 32][..]; // BCS wrote it, GCS never ingested (block 160)
        let ingested = &[0xBBu8; 32][..]; // GCS ingested it (block 150)
        for (handle, block) in [(leftover, 160_i64), (ingested, 150_i64)] {
            seed_bcs_handle(&pool, handle, block, 0x00).await;
        }

        // Green's own copy of the ingested handle: the cutover deletes blue's rows
        // unconditionally, so only what lives in `gcs.*` survives the merge.
        let seed_gcs = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.computations
                (output_handle, dependencies, fhe_operation,
                 is_scalar, is_completed, host_chain_id, block_number)
             VALUES ($1, ARRAY[]::bytea[], 0, false, TRUE, 1, 150)"
        );
        sqlx::query(&seed_gcs)
            .bind(ingested)
            .execute(&pool)
            .await
            .expect("seed gcs computation");
        let seed_gcs_ct = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertexts
                (handle, ciphertext, ciphertext_version, ciphertext_type)
             VALUES ($1, $2, 0, 0)"
        );
        sqlx::query(&seed_gcs_ct)
            .bind(ingested)
            .bind(&[0x01u8][..])
            .execute(&pool)
            .await
            .expect("seed gcs ciphertext");
        let seed_gcs_pbs = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.pbs_computations
                (handle, is_completed, host_chain_id, block_number)
             VALUES ($1, TRUE, 1, 150)"
        );
        sqlx::query(&seed_gcs_pbs)
            .bind(ingested)
            .execute(&pool)
            .await
            .expect("seed gcs pbs_computation");

        // Green's branch-form copies of the same handle, on the same branch context blue
        // used, so the merge restores exactly the rows the deletes removed.
        let branch = &[0x11u8; 32][..];
        for sql in [
            format!(
                "INSERT INTO {GCS_SCHEMA_QUOTED}.computations_branch
                    (output_handle, dependencies, fhe_operation, is_scalar,
                     is_completed, host_chain_id, block_number, producer_block_hash)
                 VALUES ($1, ARRAY[]::bytea[], 0, false, TRUE, 1, 150, $2)"
            ),
            format!(
                "INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertexts_branch
                    (handle, ciphertext, ciphertext_version, ciphertext_type,
                     producer_block_hash, block_number)
                 VALUES ($1, '\\x01'::bytea, 0, 0, $2, 150)"
            ),
            format!(
                "INSERT INTO {GCS_SCHEMA_QUOTED}.pbs_computations_branch
                    (handle, is_completed, host_chain_id, block_number,
                     producer_block_hash, block_hash)
                 VALUES ($1, TRUE, 1, 150, $2, $2)"
            ),
            format!(
                "INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertext_digest_branch
                    (handle, host_chain_id, block_number, producer_block_hash, block_hash)
                 VALUES ($1, 1, 150, $2, $2)"
            ),
        ] {
            sqlx::query(&sql)
                .bind(ingested)
                .bind(branch)
                .execute(&pool)
                .await
                .expect("seed gcs branch row");
        }

        execute_cutover(&pool).await.expect("cutover succeeds");

        // Leftover (no gcs row): every legacy and branch row deleted, so the green stack
        // re-derives the handle from the chain. The branch tables are the canonical ones
        // after the branch-context migration, so a survivor there would be exactly the
        // per-operator divergence the deletes exist to prevent.
        assert_eq!(
            leftover_counts(&pool, leftover).await,
            [0; 8],
            "a BCS leftover GCS never ingested must be deleted from both the legacy and \
             the branch tables, not inherited by green"
        );

        // Green's rows come back for the handle it ingested. The digest branch row is
        // restored on its real branch context; the branchless (`block_number IS NULL`)
        // slot stays empty because nothing wrote `public.ciphertext_digest`.
        assert_eq!(
            leftover_counts(&pool, ingested).await,
            [1, 1, 1, 1, 1, 1, 1, 0],
            "a handle GCS ingested must be restored by the merge in both forms"
        );
    }

    /// Gateway/zkproof-side backfill: the input ciphertext of a gw-window proof GCS
    /// never re-verified is deleted along with its `input_handles` row; the handle GCS
    /// did reproduce comes back with the merge.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_backfill_deletes_gw_leftovers() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await; // gw_start_block = 1

        let h_left = &[0xC1u8; 32][..]; // input handle of the leftover proof
        let h_repro = &[0xC2u8; 32][..]; // input handle of the reproduced proof

        for (id, block) in [(100_i64, 5_i64), (200_i64, 3_i64)] {
            sqlx::query(
                "INSERT INTO verify_proofs
                    (zk_proof_id, chain_id, contract_address, user_address, verified, block_number)
                 VALUES ($1, 1, '0xc', '0xu', TRUE, $2)",
            )
            .bind(id)
            .bind(block)
            .execute(&pool)
            .await
            .expect("seed verify_proofs");
        }
        for (handle, block) in [(h_left, 5_i64), (h_repro, 3_i64)] {
            sqlx::query("INSERT INTO input_handles (handle, block_number) VALUES ($1, $2)")
                .bind(handle)
                .bind(block)
                .execute(&pool)
                .await
                .expect("seed input_handles");
            sqlx::query(
                "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
                 VALUES ($1, $2, 0, 0)",
            )
            .bind(handle)
            .bind(&[0x00u8][..])
            .execute(&pool)
            .await
            .expect("seed ciphertext");
        }

        // GCS re-verified proof 200 and reproduced its input handle.
        let gcs_vp = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.verify_proofs
                (zk_proof_id, chain_id, contract_address, user_address, verified, block_number)
             VALUES (200, 1, '0xc', '0xu', TRUE, 3)"
        );
        sqlx::query(&gcs_vp)
            .execute(&pool)
            .await
            .expect("seed gcs verify_proofs");
        let gcs_ih = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.input_handles (handle, block_number) VALUES ($1, 3)"
        );
        sqlx::query(&gcs_ih)
            .bind(h_repro)
            .execute(&pool)
            .await
            .expect("seed gcs input_handles");

        execute_cutover(&pool).await.expect("cutover succeeds");

        let (cts, inputs): (i64, i64) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM ciphertexts WHERE handle = $1),
                    (SELECT COUNT(*) FROM input_handles WHERE handle = $1)",
        )
        .bind(h_left)
        .fetch_one(&pool)
        .await
        .expect("leftover counts");
        assert_eq!(
            (cts, inputs),
            (0, 0),
            "an input handle GCS never reproduced must not outlive the dry-run"
        );

        let (cts, inputs): (i64, i64) = sqlx::query_as(
            "SELECT (SELECT COUNT(*) FROM ciphertexts WHERE handle = $1),
                    (SELECT COUNT(*) FROM input_handles WHERE handle = $1)",
        )
        .bind(h_repro)
        .fetch_one(&pool)
        .await
        .expect("reproduced counts");
        assert_eq!(
            (cts, inputs),
            (1, 1),
            "a handle GCS reproduced must be restored by the merge"
        );

        // `delete_bcs_gw_leftovers` deliberately leaves `verify_proofs` alone, so the
        // proof green never re-verified stays verified and its deleted input
        // ciphertext is not re-derived. Pinned here so re-arming the zkproof-worker
        // has to update this expectation.
        let verified: Option<bool> =
            sqlx::query_scalar("SELECT verified FROM verify_proofs WHERE zk_proof_id = 100")
                .fetch_one(&pool)
                .await
                .expect("proof 100");
        assert_eq!(verified, Some(true), "proof 100 is left as BCS wrote it");
    }

    /// A handle BCS already committed (`txn_is_sent = true`) must stay committed
    /// after cutover, so the merge's unsent flag can't trigger a re-broadcast; a
    /// handle BCS never committed stays unsent so green commits it once.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_preserves_committed_txn_is_sent() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;

        let committed = &[0xD1u8; 32][..]; // BCS committed on-chain
        let uncommitted = &[0xD2u8; 32][..]; // BCS never committed

        sqlx::query(
            "INSERT INTO ciphertext_digest (handle, txn_is_sent) VALUES ($1, TRUE), ($2, FALSE)",
        )
        .bind(committed)
        .bind(uncommitted)
        .execute(&pool)
        .await
        .expect("seed public digests");

        // GCS's dry-run rows are unsent; the merge would copy that onto public.
        let gcs = format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.ciphertext_digest (handle, txn_is_sent)
             VALUES ($1, FALSE), ($2, FALSE)"
        );
        sqlx::query(&gcs)
            .bind(committed)
            .bind(uncommitted)
            .execute(&pool)
            .await
            .expect("seed gcs digests");

        execute_cutover(&pool).await.expect("cutover succeeds");

        let sent: bool =
            sqlx::query_scalar("SELECT txn_is_sent FROM ciphertext_digest WHERE handle = $1")
                .bind(committed)
                .fetch_one(&pool)
                .await
                .expect("committed digest");
        assert!(
            sent,
            "an already-committed handle must stay sent so it is not re-broadcast"
        );

        let sent: bool =
            sqlx::query_scalar("SELECT txn_is_sent FROM ciphertext_digest WHERE handle = $1")
                .bind(uncommitted)
                .fetch_one(&pool)
                .await
                .expect("uncommitted digest");
        assert!(
            !sent,
            "a never-committed handle stays unsent so green commits it once"
        );
    }

    async fn live_stack_version(pool: &Pool<Postgres>) -> String {
        sqlx::query_scalar("SELECT stack_version FROM versioning WHERE singleton = TRUE")
            .fetch_one(pool)
            .await
            .expect("versioning row")
    }

    /// A cutover that fails for a transient reason is retried until it succeeds. The
    /// stand-in for the transient fault is a missing GCS schema, which makes
    /// `delete_bcs_gw_leftovers` fail on `gcs.input_handles`; a background task creates
    /// the schema part-way through the backoff, and a later attempt then commits.
    ///
    /// Timings have wide margins: the schema appears at ~500ms while the first attempt
    /// runs at t=0 and takes tens of milliseconds, so attempt 1 always fails. The
    /// elapsed-time assertion is what proves a retry happened rather than a lucky
    /// first attempt.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_retries_transient_failure_until_it_succeeds() {
        use std::time::Instant;

        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;
        assert_eq!(live_stack_version(&pool).await, "v0.14");

        let schema_pool = pool.clone();
        let heal = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            create_gcs_schema(&schema_pool)
                .await
                .expect("create gcs schema");
        });

        let base_delay = Duration::from_millis(200);
        let started = Instant::now();
        retry_cutover(&pool, &CancellationToken::new(), 6, base_delay)
            .await
            .expect("cutover must succeed once the transient fault clears");
        let elapsed = started.elapsed();
        heal.await.expect("healer task");

        assert!(
            elapsed >= base_delay * 2,
            "cutover returned in {elapsed:?}, too fast to have backed off twice — the \
             test proved nothing about retrying"
        );
        assert_eq!(gcs_state(&pool).await, ("LIVE".into(), "completed".into()));
        assert_eq!(
            live_stack_version(&pool).await,
            "v0.15",
            "the successful retry must promote the new stack version"
        );
    }

    /// A permanently failing cutover gives up after its attempt budget and leaves the
    /// rows exactly as it found them, so the next reconcile tick retries. Because the
    /// whole cutover is one transaction, "unchanged" includes the `versioning` bump.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_retry_gives_up_after_the_budget_leaving_state_retryable() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;

        // No GCS schema, and nothing creates one: every attempt fails identically.
        let err = retry_cutover(
            &pool,
            &CancellationToken::new(),
            3,
            Duration::from_millis(10),
        )
        .await
        .expect_err("a permanently failing cutover must surface the error");
        assert!(matches!(err, Error::Db(_)), "unexpected error: {err:?}");

        assert_eq!(
            gcs_state(&pool).await,
            ("UpgradeAuthorized".into(), "in_progress".into()),
            "giving up must leave the rows retryable by the next reconcile tick"
        );
        assert_eq!(
            live_stack_version(&pool).await,
            "v0.14",
            "a rolled-back cutover must not have promoted the stack version"
        );
    }

    /// Shutdown must not be held up by the backoff: an already-cancelled token makes the
    /// retry return after its current attempt instead of sleeping out the schedule.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_retry_stops_on_cancellation() {
        use std::time::Instant;

        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await;

        let cancel = CancellationToken::new();
        cancel.cancel();

        // A 60s backoff the retry must NOT wait out.
        let started = Instant::now();
        retry_cutover(&pool, &cancel, 6, Duration::from_secs(60))
            .await
            .expect_err("cancellation surfaces the last error");
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "cancelled retry waited on the backoff"
        );
        assert_eq!(
            gcs_state(&pool).await,
            ("UpgradeAuthorized".into(), "in_progress".into()),
        );
    }

    /// Cutover must refuse while a pre-window BCS handle still owes S3 an upload, and go
    /// through once it is uploaded. An in-window handle with a NULL digest must NOT block:
    /// those are green's, re-armed on purpose for the post-cutover backfill.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn cutover_waits_for_bcs_pre_window_s3_uploads() {
        let (_instance, pool) = test_pool().await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        seed_gcs_row(&pool, "UpgradeAuthorized", "in_progress").await; // start=100

        let pre_window = &[0xE1u8; 32][..]; // block 50, blue's alone
        let in_window = &[0xE2u8; 32][..]; // block 150, green re-arms it
        for (handle, block) in [(pre_window, 50_i64), (in_window, 150_i64)] {
            sqlx::query(
                "INSERT INTO computations
                    (output_handle, dependencies, fhe_operation,
                     is_scalar, is_completed, host_chain_id, block_number)
                 VALUES ($1, ARRAY[]::bytea[], 0, false, TRUE, 1, $2)",
            )
            .bind(handle)
            .bind(block)
            .execute(&pool)
            .await
            .expect("seed computation");
            // ciphertext digest NULL = upload still pending.
            sqlx::query("INSERT INTO ciphertext_digest (handle, host_chain_id) VALUES ($1, 1)")
                .bind(handle)
                .execute(&pool)
                .await
                .expect("seed digest");
        }

        let err = execute_cutover(&pool)
            .await
            .expect_err("cutover must refuse while a pre-window upload is pending");
        assert!(
            matches!(err, Error::PendingBcsUploads { pending: 1 }),
            "expected one pending upload, got: {err:?}"
        );
        assert!(
            err.is_transient(),
            "a pending upload is a wait, not a failure"
        );
        assert_eq!(
            gcs_state(&pool).await,
            ("UpgradeAuthorized".into(), "in_progress".into()),
            "a blocked cutover must stay retryable"
        );
        assert_eq!(live_stack_version(&pool).await, "v0.14");

        // The retry wrapper keeps re-checking and leaves the row retryable. It never cuts
        // over regardless: the condition clears only when blue finishes its upload.
        let err = retry_cutover(
            &pool,
            &CancellationToken::new(),
            2,
            Duration::from_millis(10),
        )
        .await
        .expect_err("retry must not cut over while the upload is pending");
        assert!(err.is_transient(), "unexpected error: {err:?}");
        assert_eq!(
            gcs_state(&pool).await,
            ("UpgradeAuthorized".into(), "in_progress".into()),
            "the deferred cutover must not have advanced the FSM"
        );

        // Blue finishes the pre-window upload. The in-window NULL digest stays NULL.
        sqlx::query("UPDATE ciphertext_digest SET ciphertext = '\\x01'::bytea WHERE handle = $1")
            .bind(pre_window)
            .execute(&pool)
            .await
            .expect("mark uploaded");

        execute_cutover(&pool)
            .await
            .expect("cutover proceeds once the pre-window upload landed");
        assert_eq!(gcs_state(&pool).await, ("LIVE".into(), "completed".into()));
    }

    /// A readiness task left over from an old proposal must not advance a newer one.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn transition_ignores_other_proposal() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await; // proposal_id = [0x02]

        // Stale proposal: no-op.
        transition_to_dry_run_started(&pool, &[0x99], 10)
            .await
            .expect("transition");
        assert_eq!(gcs_state(&pool).await.0, "UpgradeActivated");

        transition_to_dry_run_started(&pool, &[0x02], 9)
            .await
            .expect("transition");
        assert_eq!(gcs_state(&pool).await.0, "UpgradeActivated");

        // Matching proposal: advances.
        transition_to_dry_run_started(&pool, &[0x02], 10)
            .await
            .expect("transition");
        assert_eq!(gcs_state(&pool).await.0, "DryRunStarted");
    }

    /// Same guard on the gateway track: a stale proposal must not set gw_dry_run_started.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn gw_transition_ignores_other_proposal() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await; // proposal_id = [0x02]
        sqlx::query("UPDATE upgrade_state SET gw_dry_run_started = FALSE WHERE stack_role = 'GCS'")
            .execute(&pool)
            .await
            .expect("reset flag");

        let gw_flag = |pool: Pool<Postgres>| async move {
            let (g,): (bool,) = sqlx::query_as(
                "SELECT gw_dry_run_started FROM upgrade_state WHERE stack_role = 'GCS'",
            )
            .fetch_one(&pool)
            .await
            .expect("gw flag");
            g
        };

        // Stale proposal: no-op.
        transition_to_gw_dry_run_started(&pool, &[0x99], 10)
            .await
            .expect("gw transition");
        assert!(
            !gw_flag(pool.clone()).await,
            "stale proposal must not release gw"
        );

        transition_to_gw_dry_run_started(&pool, &[0x02], 9)
            .await
            .expect("gw transition");
        assert!(
            !gw_flag(pool.clone()).await,
            "stale attempt must not release gw"
        );

        transition_to_gw_dry_run_started(&pool, &[0x02], 10)
            .await
            .expect("gw transition");
        assert!(gw_flag(pool.clone()).await, "matching proposal releases gw");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn prune_requires_matching_attempt() {
        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await;
        sqlx::query("UPDATE upgrade_state SET gw_dry_run_started = FALSE")
            .execute(&pool)
            .await
            .expect("reset flag");
        create_gcs_schema(&pool).await.expect("create gcs schema");

        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                host_chain_id, block_number
             ) VALUES ($1, ARRAY[]::BYTEA[], 0, FALSE, 1, 50)"
        ))
        .bind(&[1_u8][..])
        .execute(&pool)
        .await
        .expect("insert computation");
        sqlx::query(&format!(
            "INSERT INTO {GCS_SCHEMA_QUOTED}.verify_proofs (
                zk_proof_id, chain_id, contract_address, user_address, block_number
             ) VALUES (1, 1, '', '', 50)"
        ))
        .execute(&pool)
        .await
        .expect("insert proof");

        assert_eq!(
            prune_gcs_computations_before_start(&pool, &[0x02], 9, 1, 100)
                .await
                .expect("stale computation prune"),
            0
        );
        assert_eq!(
            prune_gcs_verify_proofs_before_start(&pool, &[0x02], 9, 100)
                .await
                .expect("stale proof prune"),
            0
        );
        assert_eq!(
            prune_gcs_computations_before_start(&pool, &[0x02], 10, 1, 100)
                .await
                .expect("computation prune"),
            1
        );
        assert_eq!(
            prune_gcs_verify_proofs_before_start(&pool, &[0x02], 10, 100)
                .await
                .expect("proof prune"),
            1
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn gcs_rollback_policy_distinguishes_derived_and_raw_writes() {
        use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard};
        let (_instance, pool) = test_pool().await;

        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await;
        assert!(
            matches!(
                begin_write_guarded(&pool, true, GcsRollbackPolicy::Skip)
                    .await
                    .expect("guard"),
                WriteGuard::Proceed(_)
            ),
            "write proceeds while dry-running"
        );

        seed_gcs_row(&pool, "PAUSED", "failed").await;
        assert!(
            matches!(
                begin_write_guarded(&pool, true, GcsRollbackPolicy::Skip)
                    .await
                    .expect("guard"),
                WriteGuard::Skip
            ),
            "derived output is skipped after rollback"
        );
        assert!(
            matches!(
                begin_write_guarded(&pool, true, GcsRollbackPolicy::Continue)
                    .await
                    .expect("guard"),
                WriteGuard::Proceed(_)
            ),
            "raw ingestion continues after rollback"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn rollback_serializes_guarded_writes_and_pruning() {
        use fhevm_engine_common::versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard};
        use tokio::time::{timeout, Duration};

        let (_instance, pool) = test_pool().await;
        seed_gcs_row(&pool, "DryRunStarted", "in_progress").await;
        create_gcs_schema(&pool).await.expect("create gcs schema");
        create_marker(&pool).await;

        let guarded_tx = match begin_write_guarded(&pool, true, GcsRollbackPolicy::Continue)
            .await
            .expect("guard")
        {
            WriteGuard::Proceed(tx) => tx,
            WriteGuard::Stop | WriteGuard::Skip => panic!("write unexpectedly blocked"),
        };

        let rollback_pool = pool.clone();
        let mut rollback =
            tokio::spawn(async move { rollback_dry_run(&rollback_pool, &[0x02], 10, 200).await });

        assert!(
            timeout(Duration::from_millis(200), &mut rollback)
                .await
                .is_err(),
            "rollback must block while a writer holds the shared advisory lock"
        );
        assert!(
            marker_exists(&pool).await,
            "schema must not reset while the guarded write is in flight"
        );

        guarded_tx.commit().await.expect("release write guard");
        assert!(
            timeout(Duration::from_secs(10), rollback)
                .await
                .expect("rollback remained blocked")
                .expect("rollback task panicked")
                .expect("rollback failed"),
            "rollback should claim the active dry-run"
        );
        assert!(
            !marker_exists(&pool).await,
            "schema should reset after the guarded write commits"
        );
        assert_eq!(gcs_state(&pool).await, ("PAUSED".into(), "failed".into()));

        seed_gcs_row(&pool, "UpgradeActivated", "in_progress").await;
        create_gcs_schema(&pool).await.expect("create gcs schema");

        let mut rollback_tx = pool.begin().await.expect("rollback tx");
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(CUTOVER_LOCK_ID)
            .execute(&mut *rollback_tx)
            .await
            .expect("exclusive lock");
        sqlx::query(
            "UPDATE upgrade_state
                SET state = 'PAUSED', status = 'failed'
              WHERE stack_role = 'GCS'",
        )
        .execute(&mut *rollback_tx)
        .await
        .expect("claim rollback");

        let prune_pool = pool.clone();
        let mut prune = tokio::spawn(async move {
            prune_gcs_computations_before_start(&prune_pool, &[0x02], 10, 1, 100).await
        });
        assert!(
            timeout(Duration::from_millis(200), &mut prune)
                .await
                .is_err(),
            "prune must wait while rollback holds the exclusive advisory lock"
        );

        reset_gcs_schema(&mut rollback_tx)
            .await
            .expect("reset schema");
        rollback_tx.commit().await.expect("commit rollback");
        assert_eq!(
            timeout(Duration::from_secs(10), prune)
                .await
                .expect("prune remained blocked")
                .expect("prune task panicked")
                .expect("prune failed"),
            0
        );
    }
}
