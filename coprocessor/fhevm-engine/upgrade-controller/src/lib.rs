//! Upgrade Controller (`upgrade-controller`) — drives the upgrade FSM in Postgres.
//!
//! Listens for `upgrade_activated` and `quorum_reached` notifications via
//! `pg_notify` and mutates rows in the `upgrade_state` table accordingly.

use std::{fmt, str::FromStr, time::Duration};

use fhevm_engine_common::utils::DatabaseURL;
use serde::Deserialize;
use sqlx::{postgres::PgListener, Pool, Postgres};
use thiserror::Error;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn, Level};

pub const UPGRADE_ACTIVATED_CHANNEL: &str = "upgrade_activated";
pub const QUORUM_REACHED_CHANNEL: &str = "quorum_reached";

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
    pub start_block: i64,
    pub end_block: i64,
    pub gw_start_block: i64,
    /// Optional — included for forward-compat with the schema's `version` column.
    #[serde(default)]
    pub version: Option<String>,
}

/// Payload published over `quorum_reached`.
///
/// `proposal_id` is matched against the in-DB FSM row to ensure the quorum
/// event belongs to the same upgrade that activated this row — otherwise a
/// late or replayed event could trigger a cutover for the wrong proposal.
#[derive(Debug, Clone, Deserialize)]
pub struct QuorumReachedPayload {
    /// Hex-encoded bytes; must match `upgrade_state.proposal_id`.
    pub proposal_id: String,
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
        .map(|i| {
            u8::from_str_radix(&trimmed[i..i + 2], 16)
                .map_err(|e| Error::Hex(e.to_string()))
        })
        .collect()
}

/// Handle an `upgrade_activated` notification: parse payload and upsert the
/// FSM row with `state='UpgradeActivated'`, `status='in_progress'`.
pub async fn handle_upgrade_activated(
    pool: &Pool<Postgres>,
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
        start_block = payload.start_block,
        end_block = payload.end_block,
        gw_start_block = payload.gw_start_block,
        "upgrade_activated received — inserting upgrade_state row"
    );

    sqlx::query(
        r#"
        INSERT INTO upgrade_state (
            stack_role, state, status, proposal_id, version,
            start_block, end_block, gw_start_block, updated_at
        )
        VALUES ($1, 'UpgradeActivated', 'in_progress', $2, $3, $4, $5, $6, NOW())
        ON CONFLICT (stack_role) DO UPDATE
        SET state          = EXCLUDED.state,
            status         = EXCLUDED.status,
            proposal_id    = EXCLUDED.proposal_id,
            version        = EXCLUDED.version,
            start_block    = EXCLUDED.start_block,
            end_block      = EXCLUDED.end_block,
            gw_start_block = EXCLUDED.gw_start_block,
            last_error     = NULL,
            updated_at     = NOW()
        "#,
    )
    .bind(stack_role)
    .bind(&proposal_id_bytes)
    .bind(payload.version.as_deref())
    .bind(payload.start_block)
    .bind(payload.end_block)
    .bind(payload.gw_start_block)
    .execute(pool)
    .await?;

    Ok(())
}

/// Placeholder cutover routine — invoked when `quorum_reached` fires and the
/// FSM is in the right shape. Real logic lands later.
pub async fn execute_cutover(pool: &Pool<Postgres>) -> Result<(), Error> {
    info!("execute_cutover() invoked (placeholder)");
    // TODO: implement the actual cutover. For now we only record progress.
    let _ = pool;
    Ok(())
}

/// Handle a `quorum_reached` notification. The cutover is gated on:
///   - service started with `initial_mode = gcs`, AND
///   - current `upgrade_state` row for stack_role='GCS' is in state
///     'UpgradeActivated', AND
///   - the payload's `proposal_id` matches the FSM row's `proposal_id`
///     (guards against late/replayed events for a prior upgrade).
pub async fn handle_quorum_reached(
    pool: &Pool<Postgres>,
    mode: InitialMode,
    raw_payload: &str,
) -> Result<(), Error> {
    if mode != InitialMode::Gcs {
        debug!(
            mode = %mode,
            "quorum_reached: service not in gcs mode, ignoring"
        );
        return Ok(());
    }

    let payload: QuorumReachedPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;
    let payload_proposal_id = decode_hex(&payload.proposal_id)?;

    let row: Option<(String, Option<Vec<u8>>)> =
        sqlx::query_as("SELECT state, proposal_id FROM upgrade_state WHERE stack_role = 'GCS'")
            .fetch_optional(pool)
            .await?;

    match row {
        Some((state, stored_proposal_id)) if state == "UpgradeActivated" => {
            match stored_proposal_id {
                Some(stored) if stored == payload_proposal_id => {
                    info!(
                        proposal_id = %payload.proposal_id,
                        "quorum_reached: GCS UpgradeActivated and proposal_id matches — running cutover"
                    );
                    execute_cutover(pool).await?;
                }
                Some(stored) => {
                    warn!(
                        payload_proposal_id = %payload.proposal_id,
                        stored_proposal_id = %hex_encode(&stored),
                        "quorum_reached: proposal_id mismatch — skipping cutover"
                    );
                }
                None => {
                    warn!(
                        payload_proposal_id = %payload.proposal_id,
                        "quorum_reached: GCS row has no proposal_id — skipping cutover"
                    );
                }
            }
        }
        Some((state, _)) => {
            warn!(
                state,
                "quorum_reached: GCS state is not UpgradeActivated — skipping cutover"
            );
        }
        None => {
            warn!("quorum_reached: no GCS row in upgrade_state — skipping cutover");
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
        .listen_all([UPGRADE_ACTIVATED_CHANNEL, QUORUM_REACHED_CHANNEL])
        .await?;
    info!(
        channels = ?[UPGRADE_ACTIVATED_CHANNEL, QUORUM_REACHED_CHANNEL],
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
                                handle_upgrade_activated(&pool, config.initial_mode, payload).await
                            }
                            QUORUM_REACHED_CHANNEL => {
                                // pg_notify('quorum_reached', json_build_object('proposal_id', encode(proposal_id, 'hex'))::text)
                                handle_quorum_reached(&pool, config.initial_mode, payload).await
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
            "start_block": 100,
            "end_block": 200,
            "gw_start_block": 150
        }"#;
        let p: UpgradeActivatedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.proposal_id, "0x01ab");
        assert_eq!(p.start_block, 100);
        assert_eq!(p.end_block, 200);
        assert_eq!(p.gw_start_block, 150);
        assert!(p.version.is_none());
    }

    #[test]
    fn parses_quorum_reached_payload() {
        let json = r#"{"proposal_id": "0xdeadbeef"}"#;
        let p: QuorumReachedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.proposal_id, "0xdeadbeef");
        assert_eq!(
            decode_hex(&p.proposal_id).unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
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
