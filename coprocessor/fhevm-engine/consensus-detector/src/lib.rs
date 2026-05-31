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
pub mod state_hash;

use crate::s3::S3Service;
use crate::state_hash::state_hash_key;

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
    /// This operator's S3 bucket. Empty disables GCS uploads (read-only).
    pub my_bucket: String,
    /// S3 endpoint override (e.g. `http://minio:9000`).
    pub s3_endpoint: Option<String>,
    /// Max pending blocks processed per state_hash sweep.
    pub state_hash_batch_limit: i64,
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
            my_bucket: String::new(),
            s3_endpoint: None,
            state_hash_batch_limit: 256,
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

/// HTTP-GETs each operator's `state_hash` blob for `(chain_id, block_height)`.
/// If any operator is missing (404 / transport error), returns an empty `Vec`
/// so `all_identical` rejects the round.
async fn fetch_state_commitments(
    http: &reqwest::Client,
    s3_urls: &[String],
    chain_id: i64,
    block_height: i64,
) -> Vec<Vec<u8>> {
    let key = state_hash_key(chain_id, block_height);
    let mut out = Vec::with_capacity(s3_urls.len());
    for url in s3_urls {
        let path = format!("{}/{key}", url.trim_end_matches('/'));
        let bytes = match http.get(&path).send().await {
            Ok(r) if r.status().is_success() => r.bytes().await.ok().map(|b| b.to_vec()),
            _ => None,
        };
        let Some(bytes) = bytes else { return vec![] };
        out.push(bytes);
    }
    out
}

/// Returns true when `commitments` is non-empty and every entry is identical.
fn all_identical(commitments: &[Vec<u8>]) -> bool {
    let Some(first) = commitments.first() else {
        return false;
    };
    commitments.iter().all(|c| c == first)
}

/// Returns `(start_block, end_block)` for the GCS dry-run when it's active.
/// `None` otherwise. Scoped to `stack_role = 'GCS'` because BCS also stays
/// `status='in_progress'` during the upgrade and doesn't own the replay window.
pub(crate) async fn active_upgrade_window(
    pool: &Pool<Postgres>,
) -> Result<Option<(i64, i64)>, Error> {
    let row: Option<(String, Option<i64>, Option<i64>)> = sqlx::query_as(
        "SELECT state, start_block, end_block FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .fetch_optional(pool)
    .await?;

    let Some((state, start_block, end_block)) = row else { return Ok(None) };
    if !matches!(state.as_str(), "UpgradeActivated" | "DryRunStarted") {
        debug!(state = %state, "GCS not in UpgradeActivated/DryRunStarted — ignoring");
        return Ok(None);
    }
    Ok(start_block.zip(end_block))
}

/// Run the polling loop for one `(chain_id, block_height, block_hash)` event.
///
/// Emits `unanimity_consensus` on agreement, `unanimity_consensus_timeout`
/// after `commitment_timeout`. Returns early if the cancellation token fires.
#[allow(clippy::too_many_arguments)]
async fn run_unanimity_poll(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    http: &reqwest::Client,
    s3_urls: &[String],
    payload: &NewBlockPayload,
    start_block: i64,
    end_block: i64,
    commitment_poll_interval: Duration,
    commitment_timeout: Duration,
) -> Result<(), Error> {
    info!(
        chain_id = payload.chain_id,
        block_height = payload.block_height,
        block_hash = %payload.block_hash,
        start_block,
        end_block,
        operator_count = s3_urls.len(),
        poll_interval = ?commitment_poll_interval,
        timeout = ?commitment_timeout,
        "starting unanimity poll"
    );

    let deadline = tokio::time::Instant::now() + commitment_timeout;
    let mut ticker = tokio::time::interval(commitment_poll_interval);
    // First tick fires immediately — we want to attempt straight away.
    ticker.tick().await;

    // Require unanimity on every block in [start_block, end_block]; cache matches.
    let window_size = end_block.saturating_sub(start_block) as usize + 1;
    let mut confirmed: std::collections::HashSet<i64> =
        std::collections::HashSet::with_capacity(window_size);

    loop {
        for block_height in start_block..=end_block {
            if confirmed.contains(&block_height) {
                continue;
            }
            let commitments =
                fetch_state_commitments(http, s3_urls, payload.chain_id, block_height).await;
            if all_identical(&commitments) {
                confirmed.insert(block_height);
            }
        }

        if confirmed.len() == window_size {
            info!(
                chain_id = payload.chain_id,
                start_block,
                end_block,
                block_hash = %payload.block_hash,
                "unanimity reached for the whole window — emitting unanimity_consensus"
            );
            return notify_unanimity(
                pool,
                UNANIMITY_CONSENSUS_CHANNEL,
                &NewBlockPayload {
                    block_height: end_block,
                    ..payload.clone()
                },
            )
            .await;
        }

        if tokio::time::Instant::now() >= deadline {
            warn!(
                chain_id = payload.chain_id,
                block_height = payload.block_height,
                block_hash = %payload.block_hash,
                start_block,
                end_block,
                confirmed = confirmed.len(),
                window_size,
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
#[allow(clippy::too_many_arguments)]
async fn handle_new_block<P>(
    pool: &Pool<Postgres>,
    cancel: &CancellationToken,
    http: &reqwest::Client,
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

    let Some((start_block, end_block)) = active_upgrade_window(pool).await? else {
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
        http,
        &urls_snapshot,
        &payload,
        start_block,
        end_block,
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

async fn build_s3_client(config: &Config) -> aws_sdk_s3::Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
    if let Some(endpoint) = config.s3_endpoint.as_deref() {
        loader = loader.endpoint_url(endpoint);
    }
    let sdk_config = loader.load().await;
    let mut builder = aws_sdk_s3::config::Builder::from(&sdk_config);
    if config.s3_endpoint.is_some() {
        // path-style addressing is required by minio / localstack
        builder = builder.force_path_style(true);
    }
    aws_sdk_s3::Client::from_conf(builder.build())
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
        my_bucket = %config.my_bucket,
        "starting consensus-detector"
    );

    let gw_chain_id: i64 = provider
        .get_chain_id()
        .await
        .map_err(|e| anyhow::anyhow!("failed to fetch Gateway chain id: {e}"))?
        .try_into()
        .map_err(|e| anyhow::anyhow!("Gateway chain id does not fit in i64: {e}"))?;
    info!(gw_chain_id, "resolved Gateway chain id from provider");

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

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    // GCS upload only when --my-bucket is set; BCS hash compute always runs.
    let s3 = if config.my_bucket.is_empty() {
        info!("--my-bucket not set; GCS upload disabled");
        None
    } else {
        Some(Arc::new(build_s3_client(&config).await))
    };
    {
        let pool = pool.clone();
        let cancel = cancel.child_token();
        let bucket = config.my_bucket.clone();
        let batch_limit = config.state_hash_batch_limit;
        tokio::spawn(async move {
            if let Err(e) = state_hash::run(pool, s3, bucket, gw_chain_id, batch_limit, cancel).await {
                error!(error = %e, "state_hash worker exited with error");
            }
        });
    }

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
                                    &http,
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
