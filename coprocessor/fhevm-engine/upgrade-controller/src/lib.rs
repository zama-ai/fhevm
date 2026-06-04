//! Upgrade Controller (`upgrade-controller`) — drives the upgrade FSM in Postgres.
//!
//! Listens for `upgrade_activated` and `unanimity_consensus` notifications via
//! `pg_notify` and mutates rows in the `upgrade_state` table accordingly. The
//! `unanimity_consensus` channel is produced by `consensus-detector` once every
//! operator publishes the same state commitment at the upgrade's `end_block`.

use std::time::Duration;

use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
use fhevm_engine_common::utils::DatabaseURL;
use serde::Deserialize;
use sqlx::{postgres::PgListener, Pool, Postgres};
use thiserror::Error;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn, Level};

pub const UPGRADE_ACTIVATED_CHANNEL: &str = "event_upgrade_activated";
/// Must stay in sync with `consensus_detector::UNANIMITY_CONSENSUS_CHANNEL`.
pub const UNANIMITY_CONSENSUS_CHANNEL: &str = "event_unanimity_consensus";
/// Re-triggers the GCS dry-run readiness check. Must stay in sync with the
/// names emitted by `host-listener::ingest_block_logs` and the FHE workers.
pub const NEW_BLOCK_CHANNEL: &str = "event_new_block";
pub const EVENT_CIPHERTEXT_COMPUTED_CHANNEL: &str = "event_ciphertext_computed";

/// Channel emitted by `execute_cutover`, atomically with the `versioning`
/// bump, telling every service to re-evaluate its mode. Re-exported from the
/// common crate so services and the controller agree on the name.
pub use fhevm_engine_common::versioning::EVENT_STACK_VERSION_UPGRADED;

/// Number of host-chain blocks below `start_block` whose computations must
/// also be fully settled before GCS can leave `UpgradeActivated`. Hard-coded
/// for now; expected to become configurable.
const READINESS_CONFIRMATIONS: i64 = 100;

/// PostgreSQL advisory-lock key used to serialize cutover against in-flight
/// BCS writes. `execute_cutover` takes the exclusive form; the BCS-mode
/// tfhe-worker takes the shared form inside every write tx. Must match the
/// constant of the same name in `tfhe_worker::tfhe_worker`. Chosen to be
/// recognizable in logs (`hex(0x46484556_43555456) == "FHEV" || "CUTV"`).
pub const CUTOVER_LOCK_ID: i64 = 0x4648_4556_4355_5456;

/// Returns the `upgrade_state.stack_role` value for a given `gcs_mode` flag:
/// `true` → `"GCS"` (green), `false` (default) → `"BCS"` (blue).
pub fn stack_role(gcs_mode: bool) -> &'static str {
    if gcs_mode {
        "GCS"
    } else {
        "BCS"
    }
}

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
    /// Ciphertext version the upgrade target uses. Persisted on the
    /// `upgrade_state` row and later promoted into the `versioning` singleton
    /// row by `execute_cutover` inside the exclusive advisory-lock tx.
    pub ciphertext_version: i16,
    /// Optional — included for forward-compat with the schema's `version` column.
    #[serde(default)]
    pub version: Option<String>,
}

/// Payload published over `unanimity_consensus` by `consensus-detector`.
///
/// `block_height` is matched against the in-DB FSM row's `end_block` to ensure
/// the event belongs to the upgrade currently in flight — otherwise a late or
/// replayed event could trigger a cutover for the wrong block window.
#[derive(Debug, Clone, Deserialize)]
pub struct UnanimityConsensusPayload {
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

/// Decode a hex string (with or without `0x` prefix) into bytes.
fn decode_hex(s: &str) -> Result<Vec<u8>, Error> {
    let trimmed = s.strip_prefix("0x").unwrap_or(s);
    // Minimal local hex decoder to avoid pulling in another crate; payloads are short.
    if !trimmed.len().is_multiple_of(2) {
        return Err(Error::Hex("odd-length hex string".into()));
    }
    (0..trimmed.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&trimmed[i..i + 2], 16).map_err(|e| Error::Hex(e.to_string())))
        .collect()
}

/// Handle an `event_upgrade_activated` notification: parse payload and upsert
/// the FSM row with `state='UpgradeActivated'`, `status='in_progress'`. For
/// GCS, additionally spawns the dry-run readiness loop.
pub async fn handle_upgrade_activated(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    gcs_mode: bool,
    raw_payload: &str,
) -> Result<(), Error> {
    let payload: UpgradeActivatedPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    let proposal_id_bytes = decode_hex(&payload.proposal_id)?;
    let stack_role = stack_role(gcs_mode);

    info!(
        stack_role,
        proposal_id = %payload.proposal_id,
        chain_id = payload.chain_id,
        start_block = payload.start_block,
        end_block = payload.end_block,
        gw_start_block = payload.gw_start_block,
        ciphertext_version = payload.ciphertext_version,
        "event_upgrade_activated received — inserting upgrade_state row"
    );

    sqlx::query(
        r#"
        INSERT INTO upgrade_state (
            stack_role, state, status, proposal_id, version,
            start_block, end_block, gw_start_block, ciphertext_version, updated_at
        )
        VALUES ($1, 'UpgradeActivated', 'in_progress', $2, $3, $4, $5, $6, $7, NOW())
        ON CONFLICT (stack_role) DO UPDATE
        SET state              = EXCLUDED.state,
            status             = EXCLUDED.status,
            proposal_id        = EXCLUDED.proposal_id,
            version            = EXCLUDED.version,
            start_block        = EXCLUDED.start_block,
            end_block          = EXCLUDED.end_block,
            gw_start_block     = EXCLUDED.gw_start_block,
            ciphertext_version = EXCLUDED.ciphertext_version,
            last_error         = NULL,
            updated_at         = NOW()
        "#,
    )
    .bind(stack_role)
    .bind(&proposal_id_bytes)
    .bind(payload.version.as_deref())
    .bind(payload.start_block)
    .bind(payload.end_block)
    .bind(payload.gw_start_block)
    .bind(payload.ciphertext_version)
    .execute(pool)
    .await?;

    // Note: the `gcs` schema and its table duplicates are created once at
    // upgrade-controller startup (see `run`), not here — the GCS services start
    // tailing the chain before activation and need the write target to already
    // exist so their `search_path = gcs,public` writes don't fall back to the
    // live `public` schema.

    // Only GCS gates on the pre-snapshot completeness check; BCS keeps
    // serving live traffic untouched until cutover.
    if gcs_mode {
        let chain_id = payload.chain_id;
        let start_block = payload.start_block;

        // 1. Wait until BCS has fully settled every computation up to start_block.
        match wait_until_dry_run_ready(pool.clone(), cancel.child_token(), chain_id, start_block)
            .await
        {
            Ok(true) => {
                // 2. Prune: the GCS stack tails the chain before activation, so
                //    gcs.computations may hold rows for blocks below start_block.
                //    Clear them — after readiness, before the internal
                //    upgrade-activated spawn — so the dry-run snapshot begins
                //    cleanly at start_block.
                let deleted =
                    prune_gcs_computations_before_start(pool, chain_id, start_block).await?;
                info!(
                    chain_id,
                    start_block, deleted, "pruned pre-start_block rows from gcs.computations"
                );

                // 3. Spawn upgrade_activated internally: flip the GCS row to
                //    DryRunStarted, releasing the GCS stack into the dry-run.
                transition_to_dry_run_started(pool).await?;
            }
            Ok(false) => {
                info!(
                    chain_id,
                    start_block,
                    "readiness loop exited without satisfying readiness — skipping prune and transition"
                );
            }
            Err(e) => {
                error!(error = %e, "GCS dry-run readiness loop failed");
            }
        }
    }

    Ok(())
}

/// Tables that the GCS stack writes to during the dry-run phase. Each gets
/// duplicated into the `gcs` schema with `CREATE TABLE gcs.X (LIKE public.X
/// INCLUDING ALL)` at upgrade-controller startup (gated on `gcs_mode`), then
/// merged back into `public.X` at cutover.
///
/// `INCLUDING ALL` copies defaults, identity, constraints (incl. PKs/uniques),
/// generated columns, indexes, statistics, storage, comments, and compression
/// — but NOT triggers, rules, foreign keys, or ownership. That's by design:
/// the `enforce_ciphertext_version` trigger on `public.ciphertexts` does NOT
/// propagate to `gcs.ciphertexts`, which lets the GCS worker write V_new
/// while `versioning` still reads V_old.
///
/// To add a new table to the dry-run, list it here.
pub const GCS_DUPLICATED_TABLES: &[&str] = &[
    "ciphertexts",
    "ciphertexts128",
    "ciphertext_digest",
    "computations",
    "pbs_computations",
    "state_hash",
    "input_handles",
    "verify_proofs",
    "transactions",
    "allowed_handles",
    "host_chain_blocks_valid",
    "dependence_chain",
    "kms_key_activation_events",
    "kms_crs_activation_events",
    "gw_listener_last_block",
];

/// Create the versioned GCS schema (e.g. `"gcs-0.14.0"`) and a
/// `CREATE TABLE <schema>.X (LIKE public.X INCLUDING ALL)` for every table
/// listed in [`GCS_DUPLICATED_TABLES`]. The schema name is [`GCS_SCHEMA_QUOTED`]
/// so it stays in lockstep with the GCS services' `search_path`. Idempotent.
pub async fn create_gcs_schema(pool: &Pool<Postgres>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;

    let create_schema = format!("CREATE SCHEMA IF NOT EXISTS {GCS_SCHEMA_QUOTED}");
    sqlx::query(&create_schema).execute(&mut *tx).await?;

    for table in GCS_DUPLICATED_TABLES {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {GCS_SCHEMA_QUOTED}.{table} \
             (LIKE public.{table} INCLUDING ALL)"
        );
        sqlx::query(&sql).execute(&mut *tx).await?;
    }

    tx.commit().await?;
    info!(
        schema = GCS_SCHEMA_QUOTED,
        tables = ?GCS_DUPLICATED_TABLES,
        "GCS schema created with empty table duplicates"
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
    chain_id: i64,
    start_block: i64,
) -> Result<u64, Error> {
    let sql = format!(
        "DELETE FROM {GCS_SCHEMA_QUOTED}.computations \
         WHERE host_chain_id = $1 \
           AND block_number IS NOT NULL \
           AND block_number < $2"
    );
    let result = sqlx::query(&sql)
        .bind(chain_id)
        .bind(start_block)
        .execute(pool)
        .await?;
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
        .listen_all([NEW_BLOCK_CHANNEL, EVENT_CIPHERTEXT_COMPUTED_CHANNEL])
        .await?;

    loop {
        if cancel.is_cancelled() {
            info!("readiness loop cancelled");
            return Ok(false);
        }

        // Idempotency: if a parallel firing of event_upgrade_activated already
        // advanced the GCS row, exit silently.
        let current_state: Option<(String,)> =
            sqlx::query_as("SELECT state FROM upgrade_state WHERE stack_role = 'GCS'")
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

/// Conditional UPDATE: only flips if the GCS row is still in `UpgradeActivated`.
async fn transition_to_dry_run_started(pool: &Pool<Postgres>) -> Result<(), Error> {
    let result = sqlx::query(
        r#"
        UPDATE upgrade_state
        SET state = 'DryRunStarted', updated_at = NOW()
        WHERE stack_role = 'GCS' AND state = 'UpgradeActivated'
        "#,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        warn!(
            "transition_to_dry_run_started: GCS not in UpgradeActivated — skipping unpause notify"
        );
        return Ok(());
    }

    Ok(())
}

/// Cutover routine — invoked when `event_unanimity_consensus` fires and the
/// FSM has been transitioned from `DryRunStarted` to `UpgradeAuthorized`.
///
/// Runs atomically inside one transaction holding `pg_advisory_xact_lock(CUTOVER_LOCK_ID)`
/// in exclusive mode. The exclusive lock blocks until every BCS write tx
/// (which takes the same lock in shared mode at the top of each tx) has
/// committed, and conversely prevents any new BCS write tx from starting
/// until cutover commits.
///
/// Sequence:
///   1. Read `start_block` and `ciphertext_version` from the GCS upgrade row.
///   2. DELETE post-snapshot BCS rows from `public.ciphertexts` (matching
///      the old version still in `versioning`).
///   3. UPDATE `versioning` to the new ciphertext_version.
///   4. Merge `gcs.ciphertexts` → `public.ciphertexts` (re-stamping the new
///      ciphertext_version so the `enforce_ciphertext_version` trigger accepts
///      the rows).
///   5. DROP SCHEMA gcs CASCADE.
///   6. Mark GCS row LIVE/completed and BCS row PAUSED/completed.
///
/// After commit, any BCS write tx that was waiting on the shared lock
/// acquires it, re-reads its FSM state, sees `PAUSED`, and exits cleanly.
pub async fn execute_cutover(pool: &Pool<Postgres>) -> Result<(), Error> {
    info!("execute_cutover() starting");

    let row: Option<(Option<i64>, Option<i16>, Option<String>)> = sqlx::query_as(
        "SELECT start_block, ciphertext_version, version
         FROM upgrade_state
         WHERE stack_role = 'GCS'",
    )
    .fetch_optional(pool)
    .await?;

    let (_start_block, new_ciphertext_version, stack_version) = match row {
        Some((Some(s), Some(v), version)) => (s, v, version.unwrap_or_default()),
        Some((s, v, _)) => {
            return Err(Error::Payload(format!(
                "GCS upgrade_state row is missing required fields: start_block={s:?}, ciphertext_version={v:?}"
            )));
        }
        None => {
            return Err(Error::Payload(
                "no GCS row in upgrade_state — cannot run cutover".to_string(),
            ));
        }
    };

    let mut tx = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    info!(
        lock_id = CUTOVER_LOCK_ID,
        "cutover acquired exclusive advisory lock"
    );

    // 0. Refuse a no-op upgrade. If `new_ciphertext_version` equals the current
    //    `versioning.ciphertext_version`, the parent PK (tenant_id, handle,
    //    ciphertext_version) is shared between staging and parent rows for the
    //    same handle, and the merge would collide. A real upgrade always bumps
    //    the version; this guard makes the failure mode loud rather than
    //    silently overwriting parent rows.
    let (current_ciphertext_version,): (i16,) =
        sqlx::query_as("SELECT ciphertext_version FROM versioning WHERE singleton = TRUE")
            .fetch_one(&mut *tx)
            .await?;
    if new_ciphertext_version == current_ciphertext_version {
        return Err(Error::Payload(format!(
            "refusing cutover: new ciphertext_version ({new_ciphertext_version}) equals \
             current versioning.ciphertext_version — upgrades must bump the version"
        )));
    }

    // 2. Promote the new version BEFORE inserting gcs rows so the
    //    enforce_ciphertext_version() trigger lets the new-version rows through.
    sqlx::query(
        "UPDATE versioning
         SET ciphertext_version = $1, stack_version = $2, updated_at = NOW()
         WHERE singleton = TRUE",
    )
    .bind(new_ciphertext_version)
    .bind(&stack_version)
    .execute(&mut *tx)
    .await?;
    info!(
        ciphertext_version = new_ciphertext_version,
        stack_version, "versioning row updated"
    );

    // 4. Merge gcs.ciphertexts → public.ciphertexts. The SELECT re-stamps
    //    `ciphertext_version` with the upgrade target so the
    //    `enforce_ciphertext_version` trigger accepts the rows regardless of
    //    what the GCS worker binary's `current_ciphertext_version()` was at
    //    write time — the `versioning` singleton (updated above) is the source
    //    of truth, not the worker.
    //
    //    `ON CONFLICT DO UPDATE` lets the GCS rows win on PK collisions: GCS is
    //    the canonical writer for its window.
    let merge_sql = format!(
        "INSERT INTO public.ciphertexts
             (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type,
              input_blob_hash, input_blob_index, created_at, ciphertext128, is_input)
         SELECT tenant_id, handle, ciphertext, $1::smallint, ciphertext_type,
                input_blob_hash, input_blob_index, created_at, ciphertext128, is_input
         FROM {GCS_SCHEMA_QUOTED}.ciphertexts
         ON CONFLICT (tenant_id, handle, ciphertext_version) DO UPDATE
         SET ciphertext       = EXCLUDED.ciphertext,
             ciphertext_type  = EXCLUDED.ciphertext_type,
             input_blob_hash  = EXCLUDED.input_blob_hash,
             input_blob_index = EXCLUDED.input_blob_index,
             created_at       = EXCLUDED.created_at,
             ciphertext128    = EXCLUDED.ciphertext128,
             is_input         = EXCLUDED.is_input"
    );
    let merged = sqlx::query(&merge_sql)
        .bind(new_ciphertext_version)
        .execute(&mut *tx)
        .await?;
    info!(
        merged = merged.rows_affected(),
        ciphertext_version = new_ciphertext_version,
        "merged gcs.ciphertexts into public.ciphertexts"
    );

    // 5. Drop the gcs schema (and everything in it) now that its data has been

    //    merged back into public.
    let drop_sql = format!("DROP SCHEMA {GCS_SCHEMA_QUOTED} CASCADE");
    sqlx::query(&drop_sql).execute(&mut *tx).await?;
    info!(schema = GCS_SCHEMA_QUOTED, "dropped gcs schema");

    // 6. Flip FSM rows.
    sqlx::query(
        "UPDATE upgrade_state
         SET state = 'LIVE', status = 'completed', updated_at = NOW()
         WHERE stack_role = 'GCS'",
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE upgrade_state
         SET state = 'PAUSED', status = 'completed', updated_at = NOW()
         WHERE stack_role = 'BCS'",
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
        "ciphertext_version": new_ciphertext_version,
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
        stack_version,
        ciphertext_version = new_ciphertext_version,
        "execute_cutover() committed; stack-version-upgraded notify delivered"
    );
    Ok(())
}

/// Handle an `event_unanimity_consensus` notification. The cutover is gated on:
///   - service is running in GCS mode (i.e. `gcs_mode = true`), AND
///   - current `upgrade_state` row for stack_role='GCS' is in state
///     'DryRunStarted', AND
///   - the payload's `block_height` is within the FSM row's
///     `[start_block, end_block]` window (guards against late/replayed
///     events for a prior upgrade window).
///
/// When all gates pass, the row is transitioned to 'UpgradeAuthorized'
/// (conditional UPDATE) before `execute_cutover` runs — so a second firing
/// of the notify becomes a no-op.
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

    let row: Option<(String, Option<i64>, Option<i64>, Option<Vec<u8>>)> = sqlx::query_as(
        "SELECT state, start_block, end_block, proposal_id FROM upgrade_state WHERE stack_role = 'GCS'",
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some((state, start_block, end_block, proposal_id)) if state == "DryRunStarted" => {
            match (start_block, end_block) {
                (Some(start), Some(end)) if (start..=end).contains(&payload.block_height) => {
                    info!(
                        chain_id = payload.chain_id,
                        block_height = payload.block_height,
                        block_hash = %payload.block_hash,
                        start_block = start,
                        end_block = end,
                        proposal_id = ?proposal_id.as_deref().map(hex_encode),
                        "event_unanimity_consensus: GCS DryRunStarted and block_height within [start_block, end_block] — transitioning to UpgradeAuthorized and running cutover"
                    );
                    // Conditional UPDATE: only flips if still in DryRunStarted,
                    // so a parallel firing of the notify is a no-op for cutover.
                    let result = sqlx::query(
                        r#"
                        UPDATE upgrade_state
                        SET state = 'UpgradeAuthorized', updated_at = NOW()
                        WHERE stack_role = 'GCS' AND state = 'DryRunStarted'
                        "#,
                    )
                    .execute(pool)
                    .await?;
                    if result.rows_affected() == 0 {
                        warn!(
                            "event_unanimity_consensus: GCS row was no longer in DryRunStarted — skipping cutover"
                        );
                        return Ok(());
                    }
                    execute_cutover(pool).await?;
                }
                (Some(start), Some(end)) => {
                    warn!(
                        payload_block_height = payload.block_height,
                        start_block = start,
                        end_block = end,
                        "event_unanimity_consensus: block_height is outside [start_block, end_block] — skipping cutover"
                    );
                }
                _ => {
                    warn!(
                        payload_block_height = payload.block_height,
                        ?start_block,
                        ?end_block,
                        "event_unanimity_consensus: GCS row missing start_block or end_block — skipping cutover"
                    );
                }
            }
        }
        Some((state, _, _, _)) => {
            warn!(
                state,
                "event_unanimity_consensus: GCS state is not DryRunStarted — skipping cutover"
            );
        }
        None => {
            warn!("event_unanimity_consensus: no GCS row in upgrade_state — skipping cutover");
        }
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
    listener
        .listen_all([UPGRADE_ACTIVATED_CHANNEL, UNANIMITY_CONSENSUS_CHANNEL])
        .await?;
    info!(
        channels = ?[UPGRADE_ACTIVATED_CHANNEL, UNANIMITY_CONSENSUS_CHANNEL],
        "Listening for notifications"
    );

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
                                handle_upgrade_activated(&pool, &cancel, config.gcs_mode, payload).await
                            }
                            UNANIMITY_CONSENSUS_CHANNEL => {
                                // Emitted by consensus-detector when every operator publishes
                                // the same state commitment at the upgrade's end_block.
                                handle_unanimity_consensus(&pool, config.gcs_mode, payload).await
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
                debug!("poll tick — no notification activity");
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
            "gw_start_block": 150,
            "ciphertext_version": 1
        }"#;
        let p: UpgradeActivatedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.proposal_id, "0x01ab");
        assert_eq!(p.chain_id, 12345);
        assert_eq!(p.start_block, 100);
        assert_eq!(p.end_block, 200);
        assert_eq!(p.gw_start_block, 150);
        assert_eq!(p.ciphertext_version, 1);
        assert!(p.version.is_none());
    }

    #[test]
    fn parses_unanimity_consensus_payload() {
        let json = r#"{
            "chain_id": 12345,
            "block_height": 200,
            "block_hash": "0xabc0000000000000000000000000000000000000000000000000000000000001"
        }"#;
        let p: UnanimityConsensusPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.chain_id, 12345);
        assert_eq!(p.block_height, 200);
        assert_eq!(
            p.block_hash,
            "0xabc0000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn hex_encode_round_trips() {
        let bytes = vec![0x00, 0x01, 0xab, 0xff];
        let s = hex_encode(&bytes);
        assert_eq!(s, "0001abff");
        assert_eq!(decode_hex(&s).unwrap(), bytes);
    }
}
