//! Consensus Detector (`consensus-detector`) — watches operator S3 buckets for
//! unanimous state-commitment agreement at the end of an active upgrade window.
//!
//! At startup the service queries the on-chain `GatewayConfig` contract for the
//! set of coprocessor signers and resolves each one's `s3BucketUrl`. The list of
//! URLs is held in memory and is the input to `fetch_state_commitments`
//! (currently a placeholder).
//!
//! The main loop is event-driven on Postgres `pg_notify`:
//!
//!   * `new_block` — on every new block we inspect `upgrade_state`. If exactly
//!     one row is `status='in_progress'` and `state='UpgradeActivated'` and its
//!     `end_block` equals the notification's `block_height`, we start the
//!     unanimity poll.
//!   * `new_operator_added` — placeholder for re-fetching the operator set
//!     from `GatewayConfig` when membership changes.
//!
//! The unanimity poll calls `fetch_state_commitments(&s3_urls)` every 5 seconds.
//! If every operator returns the same bytes, the service emits
//! `unanimity_consensus(chain_id, block_height, block_hash)` via `pg_notify`.
//! If the poll runs for more than 1 minute without unanimity, the service emits
//! `unanimity_consensus_timeout(chain_id, block_height, block_hash)` instead.
//! Note: `unanimity_consensus` is only meaningful as a signal for the upgrade
//! procedure; nothing else consumes it.

use std::sync::Arc;
use std::time::Duration;

use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use fhevm_engine_common::utils::DatabaseURL;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, Pool, Postgres};
use thiserror::Error;
use tokio::select;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn, Level};

pub mod s3;

use crate::s3::S3Service;

/// pg_notify channels this service listens on.
pub const NEW_BLOCK_CHANNEL: &str = "event_new_block";
pub const NEW_OPERATOR_ADDED_CHANNEL: &str = "event_new_operator_added";

/// pg_notify channels this service emits.
///
/// `unanimity_consensus` is consumed only by the upgrade procedure — no other
/// service should treat it as a generic "agreement" signal.
pub const UNANIMITY_CONSENSUS_CHANNEL: &str = "event_unanimity_consensus";
pub const UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL: &str = "event_unanimity_consensus_timeout";

#[derive(Debug, Clone)]
pub struct Config {
    pub service_name: String,
    pub database_url: DatabaseURL,
    pub database_pool_size: u32,
    /// On-chain `GatewayConfig` contract address. Queried at startup to
    /// resolve the operator S3 buckets.
    pub gateway_config_address: Address,
    pub log_level: Level,
    /// Fallback poll interval used while waiting for notifications so a missed
    /// NOTIFY (e.g. dropped connection) still gets re-checked eventually.
    pub poll_interval: Duration,
    /// How often to re-call `fetch_state_commitments` while waiting for
    /// unanimity.
    pub commitment_poll_interval: Duration,
    /// Hard cap on how long we wait for unanimity before giving up and
    /// emitting `unanimity_consensus_timeout`.
    pub commitment_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            service_name: "consensus-detector".to_owned(),
            database_url: DatabaseURL::default(),
            database_pool_size: 4,
            gateway_config_address: Address::ZERO,
            log_level: Level::INFO,
            poll_interval: Duration::from_secs(30),
            commitment_poll_interval: Duration::from_secs(5),
            commitment_timeout: Duration::from_secs(60),
        }
    }
}

/// Payload of `new_block` notifications.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewBlockPayload {
    pub chain_id: i64,
    pub block_height: i64,
    pub block_hash: String,
}

/// Payload emitted on `unanimity_consensus` / `unanimity_consensus_timeout`.
#[derive(Debug, Clone, Serialize)]
struct UnanimityPayload<'a> {
    chain_id: i64,
    block_height: i64,
    block_hash: &'a str,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("invalid notification payload: {0}")]
    Payload(String),

    #[error("gateway RPC error: {0}")]
    Gateway(String),
}

/// Placeholder for the per-operator state-commitment fetch. Future work will
/// HTTP-GET each URL and return the bytes. For now it returns an empty vec
/// per URL — never unanimous, so the polling loop always hits the timeout.
async fn fetch_state_commitments(s3_urls: &[String]) -> Vec<Vec<u8>> {
    // TODO: real implementation — fetch bytes from each S3 bucket URL.
    debug!(
        operator_count = s3_urls.len(),
        "fetch_state_commitments (placeholder) — returning empty commitments"
    );
    vec![Vec::new(); s3_urls.len()]
}

/// Returns true when `commitments` is non-empty and every entry is identical.
fn all_identical(commitments: &[Vec<u8>]) -> bool {
    let Some(first) = commitments.first() else {
        return false;
    };
    commitments.iter().all(|c| c == first)
}

/// Look up the single in-progress upgrade. Returns the `end_block` when there
/// is exactly one row with `status='in_progress'` and `state='UpgradeActivated'`,
/// `None` otherwise (no active upgrade, or in a state we shouldn't act on).
async fn fetch_active_upgrade_end_block(pool: &Pool<Postgres>) -> Result<Option<i64>, Error> {
    let rows: Vec<(String, Option<i64>)> =
        sqlx::query_as("SELECT state, end_block FROM upgrade_state WHERE status = 'in_progress'")
            .fetch_all(pool)
            .await?;

    match rows.len() {
        0 => Ok(None),
        1 => {
            let (state, end_block) = &rows[0];
            if state != "UpgradeActivated" {
                debug!(
                    state = %state,
                    "active upgrade row is not in UpgradeActivated — ignoring new_block"
                );
                return Ok(None);
            }
            Ok(*end_block)
        }
        n => {
            // Schema invariant per upgrade procedure: only one in_progress row at a time.
            warn!(
                count = n,
                "found multiple in_progress upgrade_state rows — refusing to act"
            );
            Ok(None)
        }
    }
}

/// Run the polling loop for one `(chain_id, block_height, block_hash)` event.
///
/// Emits `unanimity_consensus` on agreement, `unanimity_consensus_timeout`
/// after `commitment_timeout`. Returns early if the cancellation token fires.
async fn run_unanimity_poll(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    s3_urls: &[String],
    payload: &NewBlockPayload,
    commitment_poll_interval: Duration,
    commitment_timeout: Duration,
) -> Result<(), Error> {
    info!(
        chain_id = payload.chain_id,
        block_height = payload.block_height,
        block_hash = %payload.block_hash,
        operator_count = s3_urls.len(),
        poll_interval = ?commitment_poll_interval,
        timeout = ?commitment_timeout,
        "starting unanimity poll"
    );

    let deadline = tokio::time::Instant::now() + commitment_timeout;
    let mut ticker = tokio::time::interval(commitment_poll_interval);
    // First tick fires immediately — we want to attempt straight away.
    ticker.tick().await;

    loop {
        let commitments = fetch_state_commitments(s3_urls).await;
        if all_identical(&commitments) {
            info!(
                chain_id = payload.chain_id,
                block_height = payload.block_height,
                block_hash = %payload.block_hash,
                "unanimity reached — emitting unanimity_consensus"
            );
            return notify_unanimity(pool, UNANIMITY_CONSENSUS_CHANNEL, payload).await;
        }

        if tokio::time::Instant::now() >= deadline {
            warn!(
                chain_id = payload.chain_id,
                block_height = payload.block_height,
                block_hash = %payload.block_hash,
                "unanimity poll timed out — emitting unanimity_consensus_timeout"
            );
            return notify_unanimity(pool, UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL, payload).await;
        }

        select! {
            _ = cancel.cancelled() => {
                info!("cancellation during unanimity poll — exiting without emitting");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }
    }
}

async fn notify_unanimity(
    pool: &Pool<Postgres>,
    channel: &str,
    payload: &NewBlockPayload,
) -> Result<(), Error> {
    let body = serde_json::to_string(&UnanimityPayload {
        chain_id: payload.chain_id,
        block_height: payload.block_height,
        block_hash: &payload.block_hash,
    })
    .map_err(|e| Error::Payload(e.to_string()))?;

    // pg_notify is a function call, not a statement — bind the channel and payload.
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(channel)
        .bind(body)
        .execute(pool)
        .await?;
    Ok(())
}

/// Handle a `new_block` notification.
async fn handle_new_block<P>(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    s3_urls: &Arc<RwLock<Vec<String>>>,
    raw_payload: &str,
    commitment_poll_interval: Duration,
    commitment_timeout: Duration,
    _s3_service: &S3Service<P>,
) -> Result<(), Error>
where
    P: Provider<Ethereum>,
{
    let payload: NewBlockPayload =
        serde_json::from_str(raw_payload).map_err(|e| Error::Payload(e.to_string()))?;

    info!(
        chain_id = payload.chain_id,
        block_height = payload.block_height,
        block_hash = %payload.block_hash,
        "new_block received"
    );

    let Some(end_block) = fetch_active_upgrade_end_block(pool).await? else {
        info!(
            chain_id = payload.chain_id,
            block_height = payload.block_height,
            "no active UpgradeActivated row — ignoring new_block"
        );
        return Ok(());
    };

    if payload.block_height != end_block {
        info!(
            chain_id = payload.chain_id,
            block_height = payload.block_height,
            end_block,
            "new_block does not match upgrade end_block — ignoring"
        );
        return Ok(());
    }

    // Snapshot the URL list so the poll uses a stable view even if a concurrent
    // `new_operator_added` swaps the shared list.
    let urls_snapshot = s3_urls.read().await.clone();
    run_unanimity_poll(
        pool,
        cancel,
        &urls_snapshot,
        &payload,
        commitment_poll_interval,
        commitment_timeout,
    )
    .await
}

/// Handle a `new_operator_added` notification.
///
/// Placeholder — the intended behaviour is to re-fetch the operator signer set
/// from `GatewayConfig` (via `S3Service::refresh_signer_urls`) and update the
/// shared URL list. The wiring is left here so the refresh can be turned on
/// in a follow-up without changing the dispatch surface.
async fn handle_new_operator_added<P>(
    s3_urls: &Arc<RwLock<Vec<String>>>,
    s3_service: &S3Service<P>,
    raw_payload: &str,
) -> Result<(), Error>
where
    P: Provider<Ethereum>,
{
    info!(
        payload = raw_payload,
        "new_operator_added received (placeholder — operator set refresh not yet wired)"
    );
    // Suppress unused-variable warnings without dropping the wiring.
    let _ = (s3_urls, s3_service);
    Ok(())
}

/// Main service loop.
pub async fn run<P>(
    config: Config,
    pool: Pool<Postgres>,
    provider: P,
    cancel: CancellationToken,
) -> anyhow::Result<()>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    info!(
        service_name = %config.service_name,
        gateway_config_address = %config.gateway_config_address,
        "starting consensus-detector"
    );

    let s3_service = S3Service::new(provider, config.gateway_config_address);
    let urls = s3_service.refresh_signer_urls().await?;
    if urls.is_empty() {
        return Err(anyhow::anyhow!(
            "GatewayConfig returned no operator S3 URLs at startup"
        ));
    }
    info!(
        operator_count = urls.len(),
        urls = ?urls,
        "fetched operator S3 URLs from GatewayConfig"
    );
    let s3_urls: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(urls));

    let mut listener = PgListener::connect_with(&pool).await?;
    listener
        .listen_all([NEW_BLOCK_CHANNEL, NEW_OPERATOR_ADDED_CHANNEL])
        .await?;
    info!(
        channels = ?[NEW_BLOCK_CHANNEL, NEW_OPERATOR_ADDED_CHANNEL],
        "listening for notifications"
    );

    let mut poll = tokio::time::interval(config.poll_interval);
    // First tick fires immediately; skip it so we don't double-trigger on startup.
    poll.tick().await;

    loop {
        select! {
            _ = cancel.cancelled() => {
                info!("cancellation received — consensus-detector shutting down");
                return Ok(());
            }
            recv = listener.recv() => {
                match recv {
                    Ok(notification) => {
                        let channel = notification.channel();
                        let payload = notification.payload();
                        debug!(channel, payload, "notification received");

                        let result = match channel {
                            NEW_BLOCK_CHANNEL => {
                                handle_new_block(
                                    &pool,
                                    &cancel,
                                    &s3_urls,
                                    payload,
                                    config.commitment_poll_interval,
                                    config.commitment_timeout,
                                    &s3_service,
                                ).await
                            }
                            NEW_OPERATOR_ADDED_CHANNEL => {
                                handle_new_operator_added(&s3_urls, &s3_service, payload).await
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
    fn parses_new_block_payload() {
        let json = r#"{
            "chain_id": 12345,
            "block_height": 9876,
            "block_hash": "0xabcdef"
        }"#;
        let p: NewBlockPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.chain_id, 12345);
        assert_eq!(p.block_height, 9876);
        assert_eq!(p.block_hash, "0xabcdef");
    }

    #[test]
    fn all_identical_rejects_empty() {
        let v: Vec<Vec<u8>> = vec![];
        assert!(!all_identical(&v));
    }

    #[test]
    fn all_identical_accepts_single() {
        let v = vec![vec![1u8, 2, 3]];
        assert!(all_identical(&v));
    }

    #[test]
    fn all_identical_detects_match() {
        let v = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        assert!(all_identical(&v));
    }

    #[test]
    fn all_identical_detects_mismatch() {
        let v = vec![vec![1, 2, 3], vec![1, 2, 3], vec![9, 9, 9]];
        assert!(!all_identical(&v));
    }
}
