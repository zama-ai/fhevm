//! Upgrade Controller (`upgrade-controller`) — drives the upgrade FSM in Postgres.
//!
//! Listens for `upgrade_activated` and `unanimity_consensus` notifications via
//! `pg_notify` and mutates rows in the `upgrade_state` table accordingly. The
//! `unanimity_consensus` channel is produced by `consensus-detector` once every
//! operator publishes the same state commitment at the upgrade's `end_block`.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

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
pub use coprocessor_tables::{CoprocessorTable, COPROCESSOR_TABLES};

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

/// Cutover routine — run once the GCS row is `UpgradeAuthorized`, from the
/// unanimity handler or from `reconcile`. Idempotent via the under-lock re-read.
///
/// Runs atomically inside one transaction holding `pg_advisory_xact_lock(CUTOVER_LOCK_ID)`
/// in exclusive mode. The exclusive lock blocks until every BCS write tx
/// (which takes the same lock in shared mode at the top of each tx) has
/// committed, and conversely prevents any new BCS write tx from starting
/// until cutover commits.
///
/// Sequence:
///   1. Re-read state under the lock; no-op unless `UpgradeAuthorized`, else take its `version`.
///   2. UPDATE `versioning` to the new stack_version.
///   3. Merge `gcs.ciphertexts` → `public.ciphertexts`.
///   4. DROP SCHEMA gcs CASCADE.
///   5. Mark the GCS row LIVE/completed.
///
/// After commit, any BCS write tx that was waiting on the shared lock
/// acquires it, re-reads its FSM state, sees `PAUSED`, and exits cleanly.
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

    // Every table the verified input fans out into. `ciphertexts128*` are included because the
    // sns-worker may already have produced a 128-bit form for the synthetic handle.
    for table in [
        "ciphertexts",
        "ciphertexts_branch",
        "ciphertexts128",
        "ciphertexts128_branch",
        "ciphertext_digest",
        "ciphertext_digest_branch",
        "pbs_computations",
        "pbs_computations_branch",
        "input_handles",
    ] {
        let sql = format!(
            "DELETE FROM {GCS_SCHEMA_QUOTED}.{table}              WHERE handle IN (SELECT handle FROM synthetic_input_handles)"
        );
        let deleted = sqlx::query(&sql).execute(&mut **tx).await?.rows_affected();
        if deleted > 0 {
            info!(table, deleted, "cutover: deleted synthetic input rows");
        }
    }

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

    let mut tx = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    info!(
        lock_id = CUTOVER_LOCK_ID,
        "cutover acquired exclusive advisory lock"
    );

    // Re-read every chain row under a table lock: a concurrent activation or
    // cutover cannot replace only part of the proposal while it is promoted.
    sqlx::query("LOCK TABLE upgrade_state IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await?;
    let rows: Vec<(String, Option<i64>, Option<String>)> = sqlx::query_as(
        "SELECT state, start_block, version
           FROM upgrade_state
          WHERE stack_role = 'GCS'
          ORDER BY host_chain_id
          FOR UPDATE",
    )
    .fetch_all(&mut *tx)
    .await?;
    let Some((first_state, _, first_version)) = rows.first() else {
        return Err(Error::Payload(
            "no GCS rows in upgrade_state — cannot run cutover".to_string(),
        ));
    };
    if rows.iter().any(|(state, _, _)| state != first_state) {
        return Err(Error::Payload(
            "GCS upgrade_state rows disagree on state".to_string(),
        ));
    }
    if first_state != "UpgradeAuthorized" {
        info!(state = %first_state, "cutover: GCS proposal is not UpgradeAuthorized — skipping (already cut over)");
        return Ok(());
    }
    if rows.iter().any(|(_, start, _)| start.is_none()) {
        return Err(Error::Payload(
            "a GCS upgrade_state row is missing start_block".to_string(),
        ));
    }
    if rows.iter().any(|(_, _, version)| version != first_version) {
        return Err(Error::Payload(
            "GCS upgrade_state rows disagree on version".to_string(),
        ));
    }
    let stack_version = first_version.clone().unwrap_or_default();

    // 2. Promote the new stack version inside the cutover tx. This is the
    //    source of truth read by `resolve_gcs_mode` / `reconcile_stack_mode`:
    //    the green stack becomes live and the retired blue stack pauses.
    sqlx::query(
        "UPDATE versioning
         SET stack_version = $1, updated_at = NOW()
         WHERE singleton = TRUE",
    )
    .bind(&stack_version)
    .execute(&mut *tx)
    .await?;
    info!(stack_version, "versioning row updated");

    // 3a. Drop the GCS stack's synthetic Gateway input before anything merges:
    //     it exists only to anchor dry-run consensus and must not become live data.
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

    // 5. Drop the gcs schema (and everything in it) now that its data has been

    //    merged back into public.
    let drop_sql = format!("DROP SCHEMA {GCS_SCHEMA_QUOTED} CASCADE");
    sqlx::query(&drop_sql).execute(&mut *tx).await?;
    info!(schema = GCS_SCHEMA_QUOTED, "dropped gcs schema");

    // 6. Flip the FSM row.
    sqlx::query(
        "UPDATE upgrade_state
         SET state = 'LIVE', status = 'completed', updated_at = NOW()
         WHERE stack_role = 'GCS'",
    )
    .execute(&mut *tx)
    .await?;

    // 7. Notify every service that the live stack version changed. Queued in
    //    the SAME transaction as the `versioning` UPDATE above, so the notify
    //    is atomic with the version bump — it is only delivered if the cutover
    //    commits. On receipt, each service re-evaluates its mode (the green
    //    stack leaves GCS mode to become live; the retired blue stack pauses
    //    into no-op mode).
    let payload = serde_json::json!({
        "new_version_number": stack_version,
    })
    .to_string();
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(EVENT_STACK_VERSION_UPGRADED)
        .bind(&payload)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    info!(
        channel = EVENT_STACK_VERSION_UPGRADED,
        stack_version, "execute_cutover() committed; stack-version-upgraded notify delivered"
    );
    Ok(())
}

/// Flip every GCS proposal row to `UpgradeAuthorized` and cut over once every
/// row has its host and Gateway consensus latches set. Guarded UPDATE, so
/// duplicates no-op.
async fn try_cutover_if_consensus(pool: &Pool<Postgres>) -> Result<(), Error> {
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
    execute_cutover(pool).await
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
               gw_dry_run_started = FALSE, updated_at = NOW()
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
            execute_cutover(pool).await?;
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
            try_cutover_if_consensus(pool).await?;
        }
        _ => {}
    }
    Ok(())
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
    gcs_mode: bool,
    raw_payload: &str,
) -> Result<(), Error> {
    info!("event_unanimity_consensus received — checking conditions for cutover execution");

    if !gcs_mode {
        debug!("event_unanimity_consensus: service not in gcs mode, ignoring");
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
            "event_unanimity_consensus: no in-progress GCS row in upgrade_state — skipping cutover"
        );
        return Ok(());
    };
    if state != "UpgradeActivated" && state != "DryRunStarted" {
        warn!(
            state,
            "event_unanimity_consensus: GCS state is not UpgradeActivated/DryRunStarted — skipping cutover"
        );
        return Ok(());
    }
    if proposal_id.as_deref() != Some(payload.proposal_id.as_slice()) {
        warn!("event_unanimity_consensus: proposal does not match — ignoring");
        return Ok(());
    }
    if payload.proposal_block != Some(proposal_block) {
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

    try_cutover_if_consensus(pool).await?;
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
                                handle_unanimity_consensus(&pool, config.gcs_mode, payload).await
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

        try_cutover_if_consensus(&pool).await.expect("try cutover");
        assert_eq!(
            gcs_state(&pool).await,
            ("DryRunStarted".into(), "in_progress".into()),
            "cutover must defer while a host chain is missing consensus"
        );

        // Chain 2 now reaches consensus → cutover fires for the whole proposal.
        seed_gcs_chain(&pool, 2, true, true).await;
        try_cutover_if_consensus(&pool)
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
        handle_unanimity_consensus(&pool, true, &consensus_payload(1, 150))
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
        handle_unanimity_consensus(&pool, true, &consensus_payload(999, 5))
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
        handle_unanimity_consensus(&pool, true, &payload)
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
