//! Upgrade Controller (`upgrade-controller`) — drives the upgrade FSM in Postgres.
//!
//! Listens for `upgrade_activated` and `unanimity_consensus` notifications via
//! `pg_notify` and mutates rows in the `upgrade_state` table accordingly. The
//! `unanimity_consensus` channel is produced by `consensus-detector` once every
//! operator publishes the same state commitment at the upgrade's `end_block`.

use std::{fmt, str::FromStr, time::Duration};

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
/// Emitted once GCS transitions to `DryRunStarted`. Consumed by the GCS-side
/// host-listener / gw-listener to flip them out of `--paused`.
pub const UNPAUSE_CHANNEL: &str = "event_unpause";

/// Number of host-chain blocks below `start_block` whose computations must
/// also be fully settled before GCS can leave `UpgradeActivated`. Hard-coded
/// for now; expected to become configurable.
const READINESS_CONFIRMATIONS: i64 = 10;

/// PostgreSQL advisory-lock key used to serialize cutover against in-flight
/// BCS writes. `execute_cutover` takes the exclusive form; the BCS-mode
/// tfhe-worker takes the shared form inside every write tx. Must match the
/// constant of the same name in `tfhe_worker::tfhe_worker`. Chosen to be
/// recognizable in logs (`hex(0x46484556_43555456) == "FHEV" || "CUTV"`).
pub const CUTOVER_LOCK_ID: i64 = 0x4648_4556_4355_5456;

/// Which stack this service instance represents. Maps 1:1 to the
/// `upgrade_state.stack_role` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InitialMode {
    /// "bcs" — base coprocessor stack (default)
    #[default]
    Bcs,
    /// "gcs" — green coprocessor stack
    Gcs,
}

impl InitialMode {
    pub fn stack_role(self) -> &'static str {
        match self {
            InitialMode::Bcs => "BCS",
            InitialMode::Gcs => "GCS",
        }
    }
}

impl fmt::Display for InitialMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitialMode::Bcs => write!(f, "bcs"),
            InitialMode::Gcs => write!(f, "gcs"),
        }
    }
}

impl FromStr for InitialMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bcs" => Ok(InitialMode::Bcs),
            "gcs" => Ok(InitialMode::Gcs),
            other => Err(format!(
                "invalid initial_mode '{other}', expected 'bcs' or 'gcs'"
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub service_name: String,
    pub database_url: DatabaseURL,
    pub database_pool_size: u32,
    pub initial_mode: InitialMode,
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
            initial_mode: InitialMode::default(),
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
    mode: InitialMode,
    raw_payload: &str,
) -> Result<(), Error> {
    let payload: UpgradeActivatedPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    let proposal_id_bytes = decode_hex(&payload.proposal_id)?;
    let stack_role = mode.stack_role();

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

    // Only GCS gates on the pre-snapshot completeness check; BCS keeps
    // serving live traffic untouched until cutover.
    if mode == InitialMode::Gcs {
        let pool = pool.clone();
        let cancel = cancel.child_token();
        let chain_id = payload.chain_id;
        let start_block = payload.start_block;

        if let Err(e) = wait_until_dry_run_ready(pool, cancel, chain_id, start_block).await {
            error!(error = %e, "GCS dry-run readiness loop failed");
        }
    }

    Ok(())
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
            (SELECT MAX(block_number) FROM host_chain_blocks_valid WHERE chain_id = $1),
            -1
          ) >= $3
          AND NOT EXISTS (
              SELECT 1 FROM host_chain_blocks_valid hcbv
              WHERE hcbv.chain_id = $1
                AND hcbv.block_number BETWEEN $2 AND $3
                AND hcbv.fhe_event_count > 0
                AND EXISTS (
                    SELECT 1 FROM computations c
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
/// `event_new_block` and `event_ciphertext_computed` notification. On success,
/// flips `upgrade_state` to `DryRunStarted` and emits `event_unpause`.
/// Exits on cancellation, or if another path has already moved the GCS row
/// out of `UpgradeActivated`.
async fn wait_until_dry_run_ready(
    pool: Pool<Postgres>,
    cancel: CancellationToken,
    chain_id: i64,
    start_block: i64,
) -> Result<(), Error> {
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
            return Ok(());
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
                return Ok(());
            }
            None => {
                warn!("No GCS row in upgrade_state — readiness loop exiting");
                return Ok(());
            }
        }

        match check_dry_run_ready(&pool, chain_id, start_block).await {
            Ok(true) => {
                info!(
                    chain_id,
                    start_block, "Dry-run readiness satisfied — transitioning state"
                );
                transition_to_dry_run_started(&pool).await?;
                return Ok(());
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
                return Ok(());
            }
            recv = listener.recv() => {
                match recv {
                    Ok(notification) => {
                        debug!(channel = notification.channel(), "readiness loop trigger");
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
/// Always followed by an `event_unpause` notify with `{chain_id, start_block}`.
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
///   2. DELETE post-snapshot BCS rows from parent `ciphertexts` (matching the
///      old version still in `versioning`).
///   3. UPDATE `versioning` to the new ciphertext_version.
///   4. INSERT staging → parent for ciphertexts, ciphertexts128, state_hash.
///   5. DROP the three staging tables.
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

    let (start_block, new_ciphertext_version, stack_version) = match row {
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
    info!(lock_id = CUTOVER_LOCK_ID, "cutover acquired exclusive advisory lock");

    // 1. Drop BCS-produced rows for post-snapshot blocks from the parent.
    //    Filter on the current (still-old) ciphertext_version in versioning.
    let deleted = sqlx::query(
        "DELETE FROM ONLY ciphertexts
         WHERE ciphertext_version = (SELECT ciphertext_version FROM versioning WHERE singleton = TRUE)
           AND handle IN (
               SELECT output_handle FROM computations
               WHERE block_number >= $1 AND output_handle IS NOT NULL
           )",
    )
    .bind(start_block)
    .execute(&mut *tx)
    .await?;
    info!(deleted = deleted.rows_affected(), start_block, "purged BCS post-snapshot ciphertexts");

    // 2. Promote the new version BEFORE inserting staging rows so the
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

    // 3. Merge staging → parent for all three table families and drop staging.
    for (parent, staging) in [
        ("ciphertexts", "ciphertexts_staging"),
        ("ciphertexts128", "ciphertexts128_staging"),
        ("state_hash", "state_hash_staging"),
    ] {
        let merged = sqlx::query(&format!(
            "INSERT INTO ONLY {parent} SELECT * FROM {staging}"
        ))
        .execute(&mut *tx)
        .await?;
        info!(merged = merged.rows_affected(), parent, staging, "merged staging into parent");

        sqlx::query(&format!("DROP TABLE {staging}"))
            .execute(&mut *tx)
            .await?;
        info!(staging, "dropped staging table");
    }

    // 4. Flip FSM rows.
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

    tx.commit().await?;
    info!("execute_cutover() committed");
    Ok(())
}

/// Handle an `event_unanimity_consensus` notification. The cutover is gated on:
///   - service started with `initial_mode = gcs`, AND
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
    mode: InitialMode,
    raw_payload: &str,
) -> Result<(), Error> {
    info!("event_unanimity_consensus received — checking conditions for cutover execution");

    if mode != InitialMode::Gcs {
        debug!(
            mode = %mode,
            "event_unanimity_consensus: service not in gcs mode, ignoring"
        );
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
        initial_mode = %config.initial_mode,
        "Starting upgrade-controller"
    );

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
                                handle_upgrade_activated(&pool, &cancel, config.initial_mode, payload).await
                            }
                            UNANIMITY_CONSENSUS_CHANNEL => {
                                // Emitted by consensus-detector when every operator publishes
                                // the same state commitment at the upgrade's end_block.
                                handle_unanimity_consensus(&pool, config.initial_mode, payload).await
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
