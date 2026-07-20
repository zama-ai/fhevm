//! Consensus Detector (`consensus-detector`) — watches operator S3 buckets for
//! unanimous state-hash agreement during an active GCS blue-green upgrade.
//!
//! At startup the service queries the on-chain `GatewayConfig` contract for the
//! set of coprocessor signers and resolves each one's `s3BucketUrl`. That URL
//! list is the input to the consensus poll.
//!
//! Consensus runs **per-block and eagerly** over two independent tracks, each
//! keyed by its own `chain_id` in S3:
//!
//!   * **host chain** — for each finalized, fully-computed, *non-trivial* block
//!     in `[start_block, end_block]` (one carrying a real FHE op, not only
//!     `trivialEncrypt`), poll every operator's per-block state-hash blob.
//!   * **Gateway inputs** — likewise for each sealed Gateway block carrying
//!     re-randomized ZK-proof input ciphertexts.
//!
//! The poll runs on a `commitment_poll_interval` ticker (and opportunistically
//! on `new_block` / `gw_new_block` notifications), caching per-operator bytes so
//! each pass only re-requests operators it hasn't heard from — a slow operator
//! simply fills in on a later tick. The FIRST block on a track whose blob is
//! unanimous across all operators anchors that track: the service emits
//! `unanimity_consensus(chain_id, block_height, block_hash)` and stops polling
//! it. One unanimous non-trivial block is a sufficient anchor because a
//! determinism-breaking upgrade is systematic — any such block reveals it
//! (RFC 021).
//!
//! `unanimity_consensus` is consumed only by the upgrade-controller, which
//! authorizes cutover once BOTH tracks have anchored.
//!
//! **Timeout.** If a track is still un-anchored `commitment_timeout` after `end_block`,
//! `unanimity_consensus_timeout` is emitted once and the upgrade-controller rolls the dry-run back.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
/// Fired by gw-listener on each ingested Gateway block (payload = the new
/// Gateway tip). Drives the per-Gateway-block consensus track.
pub const GW_NEW_BLOCK_CHANNEL: &str = fhevm_engine_common::gcs_activation::EVENT_GW_NEW_BLOCK;

/// pg_notify channels this service emits.
///
/// `unanimity_consensus` is consumed only by the upgrade procedure — no other
/// service should treat it as a generic "agreement" signal.
pub const UNANIMITY_CONSENSUS_CHANNEL: &str = "event_unanimity_consensus";
/// Emitted once per window when the host reaches `end_block` without both tracks
/// anchoring within `commitment_timeout`. Consumed by the upgrade-controller.
pub const UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL: &str = "event_unanimity_consensus_timeout";

/// `SupportedFheOperations::FheTrivialEncrypt` opcode (fhevm-engine-common
/// `types.rs`), as stored in `computations.fhe_operation`. trivialEncrypt
/// outputs are deterministic across operators, so a block whose only FHE ops are
/// trivial encrypts carries no consensus signal and is not used as an anchor.
const FHE_TRIVIAL_ENCRYPT_OPCODE: i16 = 24;

/// Cap on how many earliest candidate blocks a track polls per pass. We only
/// need ONE unanimous block to anchor, so a small window is enough; a few (not
/// exactly one) gives robustness if a single block stalls on one operator.
const MAX_ANCHOR_CANDIDATES: i64 = 8;

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

/// True iff every operator has responded (no `None` slots). Distinguishes
/// "every operator answered but they disagree" (an upgrade-blocking divergence)
/// from "still waiting for a slow operator" — the two look identical to
/// [`all_some_and_identical`], which returns false for both.
fn all_slots_filled(slots: &[Option<Vec<u8>>]) -> bool {
    !slots.is_empty() && slots.iter().all(Option::is_some)
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

/// Per-track eager consensus state. We only need ONE unanimous block to anchor
/// a track (RFC 021: a determinism-breaking change is systematic, so any
/// non-trivial block reveals it), after which `anchor_emitted` short-circuits
/// the track for the rest of the window. `partial` caches per-operator bytes per
/// candidate block, so each poll only re-requests operators we haven't heard
/// from — a slow operator simply fills in on a later tick.
struct ConsensusTrack {
    chain_id: i64,
    anchor_emitted: bool,
    partial: HashMap<i64, Vec<Option<Vec<u8>>>>,
    /// Blocks we've already logged a divergence warning for. State hashes are
    /// deterministic per block, so a divergence is permanent — warn once per
    /// block instead of re-warning on every poll tick.
    divergence_warned: HashSet<i64>,
}

impl ConsensusTrack {
    fn new(chain_id: i64) -> Self {
        Self {
            chain_id,
            anchor_emitted: false,
            partial: HashMap::new(),
            divergence_warned: HashSet::new(),
        }
    }

    /// Clear state for a fresh upgrade window.
    fn reset(&mut self) {
        self.anchor_emitted = false;
        self.partial.clear();
        self.divergence_warned.clear();
    }
}

/// Poll `candidates` for one track: top up each candidate's cached slots and, on
/// the FIRST block that reaches unanimity across all operators, emit
/// `event_unanimity_consensus` (with this track's `chain_id`) and mark the track
/// anchored. No-op once anchored, or with no operators/candidates. `block_hash`
/// is empty — the consumer gates on `(chain_id, block_height)` only.
async fn poll_track(
    pool: &Pool<Postgres>,
    http: &reqwest::Client,
    urls: &[String],
    track: &mut ConsensusTrack,
    candidates: &[i64],
) -> Result<(), Error> {
    if track.anchor_emitted || urls.is_empty() {
        return Ok(());
    }
    for &block in candidates {
        let slots = track
            .partial
            .entry(block)
            .or_insert_with(|| vec![None; urls.len()]);
        // Operator-set size changed (new_operator_added) — restart this block's
        // slots so indices line up with the current URL list.
        if slots.len() != urls.len() {
            *slots = vec![None; urls.len()];
        }
        fetch_state_commitments(http, urls, track.chain_id, block, slots).await;
        if all_some_and_identical(slots) {
            info!(
                chain_id = track.chain_id,
                block,
                operator_count = urls.len(),
                "unanimity anchor reached — emitting event_unanimity_consensus"
            );
            notify_unanimity(
                pool,
                UNANIMITY_CONSENSUS_CHANNEL,
                &NewBlockPayload {
                    chain_id: track.chain_id,
                    block_height: block,
                    block_hash: String::new(),
                },
            )
            .await?;
            track.anchor_emitted = true;
            track.partial.clear();
            return Ok(());
        }
        // Every operator responded but their state hashes disagree — the exact
        // determinism divergence the dry run exists to catch. Without this the
        // block stays a non-anchor forever and a stuck upgrade is
        // indistinguishable from one still waiting on a slow operator. Warn once
        // per block (divergence is permanent) and keep polling other candidates.
        if all_slots_filled(slots) && track.divergence_warned.insert(block) {
            warn!(
                chain_id = track.chain_id,
                block,
                operator_count = urls.len(),
                "state-hash divergence — all operators responded but hashes disagree; \
                 this block cannot anchor consensus"
            );
        }
    }
    Ok(())
}

/// Host-chain blocks eligible to anchor consensus: those in `[start, end]` that
/// have a locally produced `state_hash` (⇒ finalized + fully computed, per the
/// state_hash worker) AND carry at least one non-trivial, successful FHE op
/// (`fhe_operation <> trivialEncrypt`). Trivial-only / empty blocks are excluded
/// — their ciphertexts are deterministic across operators, so consensus on them
/// is vacuous. Capped to the earliest [`MAX_ANCHOR_CANDIDATES`].
async fn host_consensus_candidates(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    start: i64,
    end: i64,
) -> Result<Vec<i64>, Error> {
    let sql = format!(
        "SELECT sh.block_number
           FROM {GCS_SCHEMA_QUOTED}.state_hash sh
          WHERE sh.chain_id = $1
            AND sh.block_number >= $2 AND sh.block_number <= $3
            AND EXISTS (
                SELECT 1 FROM {GCS_SCHEMA_QUOTED}.computations c
                 WHERE c.host_chain_id = sh.chain_id
                   AND c.block_number = sh.block_number
                   AND c.is_error = false
                   AND c.fhe_operation <> {FHE_TRIVIAL_ENCRYPT_OPCODE})
          ORDER BY sh.block_number
          LIMIT {MAX_ANCHOR_CANDIDATES}"
    );
    let rows: Vec<(i64,)> = sqlx::query_as(&sql)
        .bind(host_chain_id)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(b,)| b).collect())
}

/// True once blocks up to `end_block` have been stored. Starts the timeout clock.
async fn host_reached_end_block(
    pool: &Pool<Postgres>,
    host_chain_id: i64,
    end_block: i64,
) -> Result<bool, Error> {
    let sql = format!(
        "SELECT COALESCE(
                  (SELECT MAX(block_number) FROM {GCS_SCHEMA_QUOTED}.host_chain_blocks_valid WHERE chain_id = $1),
                  -1
                ) >= $2"
    );
    let (reached,): (bool,) = sqlx::query_as(&sql)
        .bind(host_chain_id)
        .bind(end_block)
        .fetch_one(pool)
        .await?;
    Ok(reached)
}

/// Host chain id of the active GCS upgrade (set by upgrade-controller on
/// activation). `None` when unset — host consensus is skipped until it appears.
async fn active_host_chain_id(pool: &Pool<Postgres>) -> Result<Option<i64>, Error> {
    let row: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT host_chain_id FROM upgrade_state
          WHERE stack_role = 'GCS' AND status = 'in_progress'",
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(v,)| v))
}

struct WindowState {
    window: Option<(i64, i64)>,
    timeout_deadline: Option<Instant>,
    timeout_emitted: bool,
}

/// One consensus pass over both tracks. Reads the active window, resets track
/// state when the window changes or closes (so a prior upgrade's anchors never
/// carry over), then polls the host and Gateway candidates. Emits at most one
/// anchor per track per window, and at most one `unanimity_consensus_timeout` per
/// window if the deadline elapses before both tracks anchor.
async fn consensus_pass(
    pool: &Pool<Postgres>,
    http: &reqwest::Client,
    s3_urls: &Arc<RwLock<Vec<String>>>,
    host: &mut ConsensusTrack,
    gateway: &mut ConsensusTrack,
    window_state: &mut WindowState,
    commitment_timeout: Duration,
) -> Result<(), Error> {
    let window = active_upgrade_window(pool).await?;
    if window_state.window != window {
        host.reset();
        gateway.reset();
        // The upgrade window changed, so reset the timeout for the new one.
        window_state.timeout_deadline = None;
        window_state.timeout_emitted = false;
        window_state.window = window;
    }
    let Some((start, end)) = window else {
        return Ok(());
    };

    // Snapshot the URL list so both tracks see a stable operator set this pass.
    let urls = s3_urls.read().await.clone();

    // Host track: needs the host chain id (set by upgrade-controller).
    if !host.anchor_emitted {
        if let Some(host_chain_id) = active_host_chain_id(pool).await? {
            host.chain_id = host_chain_id;
            let candidates = host_consensus_candidates(pool, host_chain_id, start, end).await?;
            poll_track(pool, http, &urls, host, &candidates).await?;
        }
    }

    // Gateway track: needs gw_start_block + the gw-listener tip (sealed blocks).
    if !gateway.anchor_emitted {
        if let (Some(gw_start), Some(gw_tip)) = (
            state_hash::gw_start_block(pool).await?,
            state_hash::gw_listener_tip(pool).await?,
        ) {
            let candidates =
                pending_gw_consensus_blocks(pool, gateway.chain_id, gw_start, gw_tip).await?;
            poll_track(pool, http, &urls, gateway, &candidates).await?;
        }
    }

    // If we reached the last block but didn't agree in time, give up so the
    // upgrade can be rerun.
    let both_anchored = host.anchor_emitted && gateway.anchor_emitted;
    if !window_state.timeout_emitted && !both_anchored {
        // Start the clock once we reach the last block (chain_id is 0 until then).
        if window_state.timeout_deadline.is_none()
            && host.chain_id != 0
            && host_reached_end_block(pool, host.chain_id, end).await?
        {
            window_state.timeout_deadline = Some(Instant::now() + commitment_timeout);
            info!(
                host_chain_id = host.chain_id,
                end_block = end,
                timeout_secs = commitment_timeout.as_secs(),
                "host chain reached end_block without unanimity — arming consensus timeout"
            );
        }

        if window_state
            .timeout_deadline
            .is_some_and(|d| Instant::now() >= d)
        {
            warn!(
                host_chain_id = host.chain_id,
                start_block = start,
                end_block = end,
                host_anchored = host.anchor_emitted,
                gateway_anchored = gateway.anchor_emitted,
                "consensus timeout elapsed without both-track unanimity — emitting event_unanimity_consensus_timeout"
            );
            notify_unanimity(
                pool,
                UNANIMITY_CONSENSUS_TIMEOUT_CHANNEL,
                &NewBlockPayload {
                    chain_id: host.chain_id,
                    block_height: end,
                    block_hash: String::new(),
                },
            )
            .await?;
            window_state.timeout_emitted = true;
        }
    }

    Ok(())
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

/// Sealed Gateway blocks for which this operator has already produced a local
/// `state_hash` row — the candidate set for the Gateway consensus poll. Bounded
/// to `[gw_start, gw_tip)` so only sealed blocks are polled, and capped to the
/// earliest [`MAX_ANCHOR_CANDIDATES`]. The poll reads every operator's S3 blob,
/// including our own; our own object is uploaded only for blocks we've hashed
/// locally (see `upload_pending_gw_state_hashes`). So a block with no local
/// `state_hash` row is one we'll never upload — our own slot stays empty and
/// unanimity is unreachable — hence restricting candidates to locally-produced
/// rows avoids polling peers for a block we ourselves can't contribute to.
async fn pending_gw_consensus_blocks(
    pool: &Pool<Postgres>,
    gw_chain_id: i64,
    gw_start: i64,
    gw_tip: i64,
) -> Result<Vec<i64>, Error> {
    let sql = format!(
        "SELECT block_number FROM {GCS_SCHEMA_QUOTED}.state_hash
          WHERE chain_id = $1 AND block_number >= $2 AND block_number < $3
          ORDER BY block_number
          LIMIT {MAX_ANCHOR_CANDIDATES}"
    );
    let rows: Vec<(i64,)> = sqlx::query_as(&sql)
        .bind(gw_chain_id)
        .bind(gw_start)
        .bind(gw_tip)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(b,)| b).collect())
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
    // Resolve the Gateway chain id from the connected provider (eth_chainId).
    // This is the authoritative source — it can't drift from the chain we're
    // actually watching — and mirrors how transaction-sender derives it. Must
    // run before `provider` is moved into the S3Service below. Kept as i64 to
    // match the `state_hash`/`upgrade_state` chain_id columns and the notify
    // payloads.
    let gw_chain_id: i64 = provider
        .get_chain_id()
        .await?
        .try_into()
        .map_err(|e| anyhow::anyhow!("gateway chain_id does not fit in i64: {e}"))?;

    info!(
        service_name = %config.service_name,
        gateway_config_address = %config.gateway_config_address,
        gw_chain_id,
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
            if let Err(e) =
                state_hash::run(pool, s3, bucket, batch_limit, gw_chain_id, worker_cancel).await
            {
                error!(error = %e, "state_hash worker exited with error; shutting down consensus-detector");
                parent_cancel.cancel();
            }
        });
    }

    let channels = [
        NEW_BLOCK_CHANNEL,
        NEW_OPERATOR_ADDED_CHANNEL,
        GW_NEW_BLOCK_CHANNEL,
    ];
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(channels).await?;
    info!(?channels, "listening for notifications");

    // Per-track eager consensus state. Host chain id is discovered from
    // upgrade_state each pass (starts unknown); the Gateway track is keyed by the
    // provider-resolved gw_chain_id.
    let mut host_track = ConsensusTrack::new(0);
    let mut gateway_track = ConsensusTrack::new(gw_chain_id);
    // Last-seen upgrade window; a change (incl. close) resets both tracks.
    // Timeout tracking: when the clock started, and whether we already gave up.
    let mut window_state = WindowState {
        window: None,
        timeout_deadline: None,
        timeout_emitted: false,
    };

    // The consensus poll cadence: re-attempt S3 every commitment_poll_interval,
    // caching partial per-operator responses so slow operators fill in later.
    let mut ticker = tokio::time::interval(config.commitment_poll_interval);
    // First tick fires immediately — attempt straight away on startup.

    loop {
        // Run one consensus pass over both tracks. Shared by the ticker and the
        // notification triggers below.
        macro_rules! consensus_pass {
            () => {
                if let Err(e) = consensus_pass(
                    &pool,
                    &http,
                    &s3_urls,
                    &mut host_track,
                    &mut gateway_track,
                    &mut window_state,
                    config.commitment_timeout,
                )
                .await
                {
                    error!(error = %e, "consensus pass failed");
                }
            };
        }

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

                        match channel {
                            // A new host or Gateway block may expose a fresh
                            // candidate — poll immediately for responsiveness.
                            NEW_BLOCK_CHANNEL | GW_NEW_BLOCK_CHANNEL => {
                                consensus_pass!();
                            }
                            NEW_OPERATOR_ADDED_CHANNEL => {
                                if let Err(e) =
                                    handle_new_operator_added(&s3_urls, &s3_service, payload).await
                                {
                                    error!(channel, error = %e, "failed to handle notification");
                                }
                            }
                            other => {
                                warn!(channel = other, "ignoring notification on unexpected channel");
                            }
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "listener recv error; sleeping before retry");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
            _ = ticker.tick() => {
                consensus_pass!();
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

    #[test]
    fn all_slots_filled_distinguishes_divergence_from_waiting() {
        // Every operator responded but hashes differ → filled (divergence).
        let divergent = vec![Some(vec![1, 2, 3]), Some(vec![9, 9, 9])];
        assert!(all_slots_filled(&divergent));
        assert!(!all_some_and_identical(&divergent));

        // Still waiting on an operator → not filled.
        let waiting = vec![Some(vec![1, 2, 3]), None];
        assert!(!all_slots_filled(&waiting));

        // Empty slot set → not filled.
        let empty: Vec<Option<Vec<u8>>> = vec![];
        assert!(!all_slots_filled(&empty));
    }
}
