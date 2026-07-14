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

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
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
    /// This operator's S3 bucket. `None` disables GCS uploads (read-only).
    pub my_bucket: Option<String>,
    /// S3 endpoint override (e.g. `http://minio:9000`).
    pub s3_endpoint: Option<String>,
    /// Max pending blocks processed per state_hash pass.
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
            my_bucket: None,
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

/// HTTP-GETs `state_hash` for `(chain_id, block_height)` from each opera
/// whose slot is `None`, concurrently. Slots already populated by a prior
/// attempt are left untouched, so retries only re-request the missing
/// operators. Failure modes (404 / 5xx / transport) are logged distinctly
/// so "peer hasn't uploaded yet" is visually separable from "peer's S3 is
/// flaky".
async fn fetch_state_commitments(
    http: &reqwest::Client,
    s3_urls: &[String],
    chain_id: i64,
    block_height: i64,
    slots: &mut [Option<Vec<u8>>],
) {
    debug_assert_eq!(s3_urls.len(), slots.len());
    let key = state_hash_key(chain_id, block_height);
    let missing: Vec<usize> = slots
        .iter()
        .enumerate()
        .filter_map(|(idx, slot)| slot.is_none().then_some(idx))
        .collect();
    if missing.is_empty() {
        return;
    }
    let fetches = missing.into_iter().map(|idx| {
        let url = s3_urls[idx].clone();
        let path = format!("{}/{key}", url.trim_end_matches('/'));
        async move {
            let bytes = match http.get(&path).send().await {
                Ok(r) => {
                    let status = r.status();
                    if status.is_success() {
                        match r.bytes().await {
                            Ok(b) => Some(b.to_vec()),
                            Err(e) => {
                                warn!(operator = %url, error = %e, "operator body read failed");
                                None
                            }
                        }
                    } else if status == reqwest::StatusCode::NOT_FOUND {
                        debug!(operator = %url, "operator state_hash not yet uploaded");
                        None
                    } else if status.is_server_error() {
                        warn!(operator = %url, %status, "operator S3 server error");
                        None
                    } else {
                        warn!(operator = %url, %status, "operator returned unexpected status");
                        None
                    }
                }
                Err(e) => {
                    warn!(operator = %url, error = %e, "operator request failed (timeout/transport)");
                    None
                }
            };
            (idx, bytes)
        }
    });
    let results = futures::future::join_all(fetches).await;
    for (idx, bytes) in results {
        if bytes.is_some() {
            slots[idx] = bytes;
        }
    }
}

/// Returns true when every slot is `Some` and all bytes are identical.
/// Empty / partially filled slot sets return false.
fn all_some_and_identical(slots: &[Option<Vec<u8>>]) -> bool {
    let mut iter = slots.iter();
    let Some(Some(first)) = iter.next() else {
        return false;
    };
    iter.all(|s| matches!(s, Some(b) if b == first))
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

    let Some((state, start_block, end_block)) = row else {
        return Ok(None);
    };
    if !matches!(state.as_str(), "UpgradeActivated" | "DryRunStarted") {
        debug!(state = %state, "GCS not in UpgradeActivated/DryRunStarted — ignoring");
        return Ok(None);
    }
    Ok(start_block.zip(end_block))
}

/// True iff every block in `[start_block, end_block]` on `chain_id` has zero FHE
/// ops (`fhe_event_count = 0`) — i.e. the whole dry-run window is vacuous. Such
/// a window produces only [`EMPTY_BLOCK_STATE_HASH`](crate::state_hash) sentinel
/// commitments, so unanimity would hold trivially and authorize a cutover
/// without ever validating that the new stack produces correct ciphertexts.
///
/// Reads the GCS `host_chain_blocks_valid` (the same source the state-hash
/// worker keys emptiness off), `COALESCE`-d to FALSE so a window with no rows is
/// treated as non-empty — we only suppress when we positively know the window
/// carries no FHE work.
pub(crate) async fn window_is_fully_empty(
    pool: &Pool<Postgres>,
    chain_id: i64,
    start_block: i64,
    end_block: i64,
) -> Result<bool, Error> {
    let sql = format!(
        "SELECT COALESCE(bool_and(fhe_event_count = 0), FALSE)
           FROM {GCS_SCHEMA_QUOTED}.host_chain_blocks_valid
          WHERE chain_id = $1 AND block_number BETWEEN $2 AND $3"
    );
    let (all_empty,): (bool,) = sqlx::query_as(&sql)
        .bind(chain_id)
        .bind(start_block)
        .bind(end_block)
        .fetch_one(pool)
        .await?;
    Ok(all_empty)
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

    // Require unanimity on every block in [start_block, end_block]. `confirmed`
    // tracks blocks that already reached unanimity (skipped on later ticks).
    // `partial` caches per-operator bytes for blocks still waiting on laggers,
    // so each tick only re-requests the operators we haven't heard from yet.
    let window_size = end_block.saturating_sub(start_block) as usize + 1;
    let mut confirmed: HashSet<i64> = HashSet::with_capacity(window_size);
    let mut partial: HashMap<i64, Vec<Option<Vec<u8>>>> = HashMap::new();

    loop {
        for block_height in start_block..=end_block {
            if confirmed.contains(&block_height) {
                continue;
            }
            let slots = partial
                .entry(block_height)
                .or_insert_with(|| vec![None; s3_urls.len()]);
            fetch_state_commitments(http, s3_urls, payload.chain_id, block_height, slots).await;
            let done = all_some_and_identical(slots);
            if done {
                confirmed.insert(block_height);
                partial.remove(&block_height);
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
        debug!(
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

    // A fully empty window (every block has zero FHE ops) yields only sentinel
    // commitments — unanimity would be vacuous and would authorize a cutover
    // with no real cross-operator validation. Refuse to signal consensus for it.
    if window_is_fully_empty(pool, payload.chain_id, start_block, end_block).await? {
        warn!(
            chain_id = payload.chain_id,
            start_block,
            end_block,
            "dry-run window has zero FHE ops in every block — not emitting unanimity_consensus"
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
        my_bucket = ?config.my_bucket,
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

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    // GCS upload only when --my-bucket is set.
    let s3 = if config.my_bucket.is_none() {
        info!("--my-bucket not set; GCS upload disabled");
        None
    } else {
        Some(Arc::new(build_s3_client(&config).await))
    };
    {
        let pool = pool.clone();
        let worker_cancel = cancel.child_token();
        let parent_cancel = cancel.clone();
        let bucket = config.my_bucket.clone().unwrap_or_default();
        let batch_limit = config.state_hash_batch_limit;
        tokio::spawn(async move {
            // The state_hash worker is required for the consensus poll: without
            // it, no GCS state hashes get uploaded and the poll always times
            // out. If it exits unexpectedly, cancel the parent so the service
            // crashes and is restarted by its supervisor.
            if let Err(e) = state_hash::run(pool, s3, bucket, batch_limit, worker_cancel).await {
                error!(error = %e, "state_hash worker exited with error; shutting down consensus-detector");
                parent_cancel.cancel();
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
    fn all_some_and_identical_rejects_empty() {
        let v: Vec<Option<Vec<u8>>> = vec![];
        assert!(!all_some_and_identical(&v));
    }

    #[test]
    fn all_some_and_identical_accepts_single() {
        let v = vec![Some(vec![1u8, 2, 3])];
        assert!(all_some_and_identical(&v));
    }

    #[test]
    fn all_some_and_identical_detects_match() {
        let v = vec![
            Some(vec![1, 2, 3]),
            Some(vec![1, 2, 3]),
            Some(vec![1, 2, 3]),
        ];
        assert!(all_some_and_identical(&v));
    }

    #[test]
    fn all_some_and_identical_detects_mismatch() {
        let v = vec![
            Some(vec![1, 2, 3]),
            Some(vec![1, 2, 3]),
            Some(vec![9, 9, 9]),
        ];
        assert!(!all_some_and_identical(&v));
    }

    #[test]
    fn all_some_and_identical_rejects_partial() {
        let v = vec![Some(vec![1, 2, 3]), None, Some(vec![1, 2, 3])];
        assert!(!all_some_and_identical(&v));
    }
}
