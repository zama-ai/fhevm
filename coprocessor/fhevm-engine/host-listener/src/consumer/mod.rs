use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy_primitives::LogData;
use anyhow::Result;
use tokio::sync::RwLock;
use tokio::time::{interval_at, sleep, Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::drift_revert::SignalStatus as DriftStatus;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};
use fhevm_engine_common::versioning::{is_retired, StackMode};
use fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION;
use primitives::event::BlockFlow;

use crate::cmd::block_history::BlockSummary;
use crate::consumer::metrics::{
    inc_blocks_duplicated, inc_blocks_missing, inc_blocks_processed,
    inc_db_errors, observe_legacy_insert_delay_seconds,
};
use crate::consumer::migration::spawn_id_migration;
use crate::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use crate::database::tfhe_event_propagate::{
    spawn_stack_version_listener, Database,
};
use crate::health_check::HealthCheck;
use crate::kms_generation::aws_s3::{AwsS3Client, AwsS3Interface};
use crate::kms_generation::process_kms_generation_activations;

use consumer::{
    AckDecision, BlockPayload, Broker, HandlerError, ListenerConsumer,
};
#[cfg(test)]
mod finalization_tests;
mod metrics;
mod migration;

const MAX_DB_RETRIES: u64 = 10;
const STATS_REPORT_INTERVAL: Duration = Duration::from_secs(60);
static SLOW_LANE_PROMOTION_ATTEMPTED: AtomicBool = AtomicBool::new(false);
const STATS_FINALIZATION_MARGIN: i64 = 5;
/// How often the retirement watcher re-reads the paused flag.
const RETIREMENT_POLL_INTERVAL: Duration = Duration::from_secs(10);

/// Data-plane identity of this listener on the broker.
///
/// Together with the chain id this names the `filters.consumer_id` row the
/// publisher fans out to, and the four stream keys those events land on. It is
/// a constant on purpose: it must not move when a deployment is renamed, when
/// an operator sets a different OTLP service name, or when blue and green run
/// side by side. Blue and green share this identity and are told apart by the
/// consumer group suffix below, not by the stream they read.
///
/// Changing this value orphans the previous streams and filter rows. That is a
/// migration (see `--migrate-from-service-name`), not a configuration change.
const DEFAULT_CONSUMER_ID: &str = "host-listener-consumer";

#[derive(Clone, Debug)]
pub struct ConsumerConfig {
    pub url: String,
    pub acl_address: Address,
    pub tfhe_address: Address,
    pub kms_generation_address: Option<Address>,
    pub protocol_config_address: Option<Address>,
    pub confidential_bridge_address: Option<Address>,
    pub database_url: DatabaseURL,
    pub database_retry_interval: Duration,
    pub health_port: u16,
    // Dependence chain settings
    pub dependence_cache_size: u16,
    pub dependence_by_connexity: bool,
    pub dependence_cross_block: bool,
    pub dependent_ops_max_per_chain: u32,
    pub chain_id: String,
    pub gcs_mode: bool,
    pub disable_synthetic_ops: bool,
    pub canonical_protocol_config_chain_id: Option<u64>,
    /// Service name this environment ran under when that name was also its
    /// broker identity, before [`DEFAULT_CONSUMER_ID`]. The chain id is
    /// appended to it to name the identity to retire, exactly as it is for the
    /// live one. Temporary — see [`migration`].
    pub migrate_from_service_name: Option<String>,
}

pub fn collect_logs(payload: &BlockPayload) -> Vec<Log> {
    let mut logs = vec![];
    for tx in &payload.transactions {
        for log in &tx.logs {
            logs.push(Log {
                inner: alloy_primitives::Log {
                    address: log.address,
                    data: LogData::new_unchecked(
                        log.topics.clone(),
                        log.data.clone(),
                    ),
                },
                block_number: Some(payload.block_number),
                block_hash: Some(payload.block_hash),
                block_timestamp: Some(payload.timestamp),
                transaction_hash: Some(tx.hash),
                transaction_index: Some(tx.transaction_index),
                log_index: Some(log.log_index),
                removed: false,
            });
        }
    }
    logs
}

#[derive(Copy, Debug, Clone)]
struct KnownDrift {
    id: i64,
    is_finished: bool,
    catchup_to: i64,
}

const STARTING_DRIFT: KnownDrift = KnownDrift {
    id: -1,
    is_finished: true,
    catchup_to: 0,
};

const DRIFT_BLOCK_MARGIN_TO_RESTART: i64 = 5;

async fn check_if_drift_revert_is_over(
    db: &Database,
    host_chain_id: i64,
    last_known_drift_locked: Arc<RwLock<KnownDrift>>,
    current_block: u64,
) -> anyhow::Result<bool> {
    let last_known_drift = *last_known_drift_locked.read().await;
    let pool = db.pool().await;
    if !last_known_drift.is_finished {
        let last_drift =
            fhevm_engine_common::drift_revert::drift_signal_for_chain(
                &pool,
                host_chain_id,
                last_known_drift.id,
            )
            .await?;
        let status = last_drift.map(|ds| ds.status);
        if status.is_none() {
            error!("Drift-revert with id {} for chain {} is not found in db, please handle manually", last_known_drift.id, host_chain_id);
        }
        match status {
            Some(DriftStatus::Done) | None => {
                // db cleaning is done, let's check if catchup is over
                let tip_block = db.read_last_valid_block().await.unwrap_or(0);
                let is_finished = tip_block
                    >= last_known_drift.catchup_to
                        + DRIFT_BLOCK_MARGIN_TO_RESTART;
                if is_finished {
                    last_known_drift_locked.write().await.is_finished = true;
                    info!(
                        tip_block = tip_block,
                        catchup_to = last_known_drift.catchup_to,
                        "Drift-revert catchup done, going back to realtime blocks"
                    );
                } else {
                    info!(
                        tip_block = tip_block,
                        catchup_to = last_known_drift.catchup_to,
                        "Drift-revert catchup in progress, waiting for more blocks to be processed"
                    );
                }
                return Ok(is_finished);
            }
            Some(DriftStatus::Pending) | Some(DriftStatus::Reverting) => {
                info!(
                    drift_id = last_known_drift.id,
                    block_number = current_block,
                    "Drift-revert in progress with status {:?}, waiting for it to be resolved before processing new blocks",
                    status.unwrap()
                );
                return Ok(false);
            }
            Some(DriftStatus::Failed(msg)) => {
                error!("Drift-revert with id {} for chain {} has failed with error: {}, please handle manually", last_known_drift.id, host_chain_id, &msg);
                return Ok(false);
            }
        }
    }
    let pool = db.pool().await;
    let Some(last_drift) =
        fhevm_engine_common::drift_revert::latest_signal_for_chain(
            &pool,
            host_chain_id,
        )
        .await?
    else {
        // never has a drift
        return Ok(true);
    };
    if last_drift.id == last_known_drift.id {
        // same old drift already finished
        return Ok(true);
    }
    // we have a new drift let's save it and assumeit's not finished yet
    *last_known_drift_locked.write().await = KnownDrift {
        id: last_drift.id,
        catchup_to: current_block as i64,
        is_finished: false,
    };
    Ok(false)
}

pub async fn promote_once_all_chains_to_fast(
    db: &Database,
    dependent_ops_max_per_chain: u32,
) {
    if dependent_ops_max_per_chain == 0 {
        if SLOW_LANE_PROMOTION_ATTEMPTED
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }
        let count = match db.promote_all_dep_chains_to_fast_priority().await {
            Ok(count) => count,
            Err(err) => {
                SLOW_LANE_PROMOTION_ATTEMPTED.store(false, Ordering::Release);
                error!(error = %err, "Failed to initially promote dependence chains to fast priority on startup");
                return;
            }
        };
        if count > 0 {
            info!(
                count,
                "Slow-lane disabled: promoted all chains to fast on startup"
            );
        }
    }
}

async fn observe_consumer_block_timing(
    db: &Database,
    chain_id: &str,
    block_summary: &BlockSummary,
    catchup: bool,
) {
    match db
        .mark_block_as_seen_by_consumer(block_summary, catchup)
        .await
    {
        Ok(delay_seconds) => {
            observe_legacy_insert_delay_seconds(chain_id, delay_seconds)
        }
        Err(err) => {
            inc_db_errors(chain_id, 1);
            warn!(
                block_number = block_summary.number,
                block_hash = ?block_summary.hash,
                error = %err,
                "Failed to record host-listener consumer block timing"
            );
        }
    }
}

async fn observe_consumer_stats(
    db: &Database,
    chain_id: &str,
    finalization_margin: i64,
) {
    info!("Checking recents blocks stats");

    let stats = db.detect_gap_seen_by_consumer(finalization_margin).await;
    let stats = match stats {
        Ok(stats) => stats,
        Err(err) => {
            inc_db_errors(chain_id, 1);
            warn!(
                finalization_margin,
                error = %err,
                "Failed to compute delayed consumer stats"
            );
            return;
        }
    };

    if stats.total_new_gap_size > 0 {
        error!(
            chain_id,
            nb_missing_blocks = stats.total_new_gap_size,
            nb_gaps = stats.number_of_new_gaps,
            finalization_margin,
            "Gaps detected for consumer",
        );
        inc_blocks_missing(chain_id, stats.total_new_gap_size as u64);
    }
    if stats.number_of_duplicated_inserts > 0 {
        warn!(
            chain_id,
            nb_missing_blocks = stats.total_new_gap_size,
            nb_gaps = stats.number_of_new_gaps,
            finalization_margin,
            "Duplicated insertion for consumer",
        );
        inc_blocks_duplicated(
            chain_id,
            stats.number_of_duplicated_inserts as u64,
        );
    }
}

async fn run_consumer_stats_observer(
    db: Database,
    chain_id: String,
    cancel_token: CancellationToken,
) {
    let mut tick = interval_at(
        Instant::now() + STATS_REPORT_INTERVAL,
        STATS_REPORT_INTERVAL,
    );
    tick.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => break,
            _ = tick.tick() => {
                observe_consumer_stats(
                    &db,
                    &chain_id,
                    STATS_FINALIZATION_MARGIN,
                )
                .await;
            }
        }
    }
}

pub async fn run_consumer(config: ConsumerConfig) -> Result<()> {
    info!("Starting consumer with config: {:?}", config);
    let mut contracts = vec![config.acl_address, config.tfhe_address];
    if let Some(protocol_config_address) = config.protocol_config_address {
        contracts.push(protocol_config_address);
    }
    if let Some(kms_generation_address) = config.kms_generation_address {
        contracts.push(kms_generation_address);
    }
    if let Some(confidential_bridge_address) =
        config.confidential_bridge_address
    {
        contracts.push(confidential_bridge_address);
    }
    let chain_id: u64 = config.chain_id.parse()?;
    let chain_id = ChainId::try_from(chain_id)?;
    let is_protocol_config_listener =
        crate::protocol_config::resolve_protocol_config_listener(
            config.canonical_protocol_config_chain_id,
            chain_id.as_u64(),
            config.protocol_config_address,
        )?;

    let blockchain_tick = HeartBeat::new();
    let blockchain_timeout_tick = HeartBeat::new();
    let blockchain_provider = Arc::new(RwLock::new(None));

    let broker_url = config.url.clone(); // e.g."amqp://user:pass@localhost:5672";
    let broker = Broker::from_url(&broker_url).await?;
    // Identity is a constant, not the telemetry name: see `DEFAULT_CONSUMER_ID`.
    let consumer_id = format!("{}.{}", DEFAULT_CONSUMER_ID, config.chain_id);
    // Blue and green share the streams and the filter row, and are separated by
    // the consensus version they were compiled against. A build only ever reads
    // the group matching its own version, so a cutover cannot deliver a block to
    // a stack that would disagree about how to execute it. This is compiled in
    // rather than configured: no operator flag can make two stacks collide, and
    // no stack can be talked into reading another version's group.
    let client =
        ListenerConsumer::new(&broker, chain_id.as_u64(), &consumer_id)
            .with_group_suffix(format!("v{CONSENSUS_PROTOCOL_VERSION}"));

    let stack_mode = StackMode::new(config.gcs_mode);
    let db = Database::new_with_stack_mode(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
        stack_mode.clone(),
    )
    .await?;

    db.tick.update();

    // A stack whose consensus version is behind the live one is retired: every
    // guarded transaction it attempts is refused and the consume handler below
    // acks and drops. Reading would only move a cursor nobody acts on, so it
    // does not create a group to begin with. Checked here because
    // `resolve_gcs_mode` cannot tell a retired build from an ordinary blue one
    // — compiled-older is not compiled-newer — so without this a retired pod
    // would create a group on every restart and leave it behind.
    let retired = {
        let pool = db.pool.read().await;
        let mut conn = pool.acquire().await?;
        is_retired(&mut conn).await?
    };

    if retired {
        warn!(
            chain_id = %config.chain_id,
            consensus_version = CONSENSUS_PROTOCOL_VERSION,
            "Consensus version is behind the live one: this stack is retired \
             and will not consume"
        );
    } else {
        // Queue before filters, in that order: registering a contract is what
        // starts the listener publishing, and a group created afterwards starts
        // at the end of the stream and never sees what was published in
        // between.
        info!("Consumer ensure queues");
        client.ensure_consumer().await?;
        client.ensure_final_consumer().await?;
        info!("Consumer registering contracts");
        client.register_contracts(&contracts).await?;
        client.register_final_contracts(&contracts).await?;
    }

    let health_check = HealthCheck {
        blockchain_timeout_tick: blockchain_timeout_tick.clone(),
        blockchain_tick: blockchain_tick.clone(),
        blockchain_provider: blockchain_provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
    };
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check),
        config.health_port,
        client.cancel_token.clone(),
    );
    tokio::spawn(async move {
        if let Err(err) = health_check_server.start().await {
            error!(error = %err, "Health check server failed");
        }
    });

    let ingest_options = IngestOptions {
        dependence_by_connexity: config.dependence_by_connexity,
        dependence_cross_block: config.dependence_cross_block,
        dependent_ops_max_per_chain: config.dependent_ops_max_per_chain,
        is_protocol_config_listener,
        disable_synthetic_ops: config.disable_synthetic_ops,
    };

    // Runtime stack mode + `event_stack_version_upgraded` listener: at cutover
    // this (blue) stack is retired and `stack_mode` flips to paused; the
    // consume handler then drops incoming blocks without writing to the DB.
    spawn_stack_version_listener(
        db.clone(),
        stack_mode.clone(),
        client.cancel_token.clone(),
    );

    // ...and when it does, stop reading. Only the live flow is cancelled: the
    // stats observer and the identity migration below are children of the
    // parent token and keep running until shutdown. The group cannot be
    // released while something is still blocked in `XREADGROUP` on it, so the
    // release happens after this loop has returned, not here.
    spawn_retirement_watcher(
        stack_mode.clone(),
        client.clone(),
        client.cancel_token.clone(),
    );

    // Temporary: retires whatever identity this environment used before
    // `DEFAULT_CONSUMER_ID`. Removed once every environment has been through
    // the release that introduced it. See `migration`.
    if let Some(old_service_name) = config.migrate_from_service_name.clone() {
        // Composed exactly as the live identity is above. The chain id belongs
        // to this process, which serves one chain, so it is not something an
        // operator should have to restate and so cannot be restated wrongly.
        let old_consumer_id =
            format!("{}.{}", old_service_name, config.chain_id);
        spawn_id_migration(
            broker.clone(),
            client.clone(),
            old_consumer_id,
            contracts.clone(),
            client.cancel_token.clone(),
        );
    }

    let last_known_drift = Arc::new(RwLock::new(STARTING_DRIFT));
    let chain_id_str = config.chain_id.to_string();
    let stats_task = tokio::spawn(run_consumer_stats_observer(
        db.clone(),
        chain_id_str.clone(),
        client.cancel_token.clone(),
    ));
    // Resume pending downloads and activations without waiting for a block.
    if config.kms_generation_address.is_some() {
        spawn_kms_activations(
            db.clone(),
            None,
            AwsS3Client {},
            client.cancel_token.clone(),
        );
    }
    // Live and finalized deliveries share this handler. A finalized delivery
    // re-ingests the block idempotently and marks it finalized.
    // The closure below takes the mode; keep a handle for the check after
    // the loop has returned.
    let retirement_mode = stack_mode.clone();
    let handler_config = config.clone();
    let kms_cancel = client.cancel_token.clone();
    let handle_block = move |payload: BlockPayload, _cancel| {
        let finalized = payload.flow == BlockFlow::Final;
        if !finalized {
            blockchain_tick.update();
        }
        let mut db = db.clone();
        let chain_id_str = chain_id_str.clone();
        let last_known_drift = last_known_drift.clone();
        let ingest_options = ingest_options.clone();
        let stack_mode = stack_mode.clone();
        let config = handler_config.clone();
        let kms_cancel = kms_cancel.clone();
        async move {
            // Paused (retired blue stack after cutover): no-op — ack and drop
            // the block without writing anything to the DB.
            if stack_mode.is_paused() {
                return Ok(AckDecision::Ack);
            }
            let drift_revert_is_over = check_if_drift_revert_is_over(
                &db,
                chain_id.as_u64() as i64,
                last_known_drift,
                payload.block_number,
            )
            .await;
            match drift_revert_is_over {
                Ok(false) => {
                    return Err(HandlerError::Transient(Box::from(
                        "Drift in progress",
                    )))
                }
                Ok(true) => (), // all good
                Err(err) => {
                    error!(%err, "Can't check drift-revert status");
                    return Err(HandlerError::Transient(err.into()));
                }
            }
            promote_once_all_chains_to_fast(
                &db,
                ingest_options.dependent_ops_max_per_chain,
            )
            .await;
            if payload.chain_id != chain_id.as_u64() {
                error!(
                    payload_chain_id = payload.chain_id,
                    configured_chain_id = chain_id.as_u64(),
                    block_number = payload.block_number,
                    "Block delivered for wrong chain — broker routing misconfigured; dropping"
                );
                return Ok(AckDecision::Ack);
            }
            ingest_payload(
                &mut db,
                &config,
                &ingest_options,
                &chain_id_str,
                &payload,
                AwsS3Client {},
                &kms_cancel,
            )
            .await
        }
    };

    info!(
        chain_id = %config.chain_id,
        "Starting host-listener consumer"
    );
    let consumer_result = if retired {
        // Nothing to read, but the process stays up: its pod is managed like
        // any other and exiting here would just crash-loop until the operator
        // removes it.
        client.cancel_token.cancelled().await;
        Ok(Ok(()))
    } else {
        let live_run = tokio::spawn(client.consume(handle_block.clone()));
        let final_run = tokio::spawn(client.consume_final(handle_block));
        // Either flow ending stops the consumer; cancelling below stops the other.
        tokio::select! {
            result = live_run => result,
            result = final_run => result,
        }
    };
    info!(
        chain_id = %config.chain_id,
        "Host listener consumer graceful stop"
    );
    client.cancel();
    if let Err(err) = stats_task.await {
        error!(error = %err, "Consumer stats background task failed");
    }

    // The read loop has returned, so nothing is on these groups any more and
    // they can go. A retired build's cursors never move again, and the broker's
    // trimmer will not reclaim past the furthest-behind group on a stream, so
    // leaving them pins the backlog until the hard ceiling cuts it. Only this
    // build's own groups are released: the streams, their entries and the
    // incoming build's groups are shared with it and must survive.
    if retired || retirement_mode.is_paused() {
        match client.destroy_own_groups().await {
            Ok(0) => {}
            Ok(released) => {
                info!(released, "Retired stack released its consumer groups")
            }
            Err(err) => warn!(
                error = %err,
                "Retired stack failed to release its consumer groups; they \
                 will be released on the next shutdown"
            ),
        }
    }
    match consumer_result {
        Ok(Ok(())) => {
            info!("Consumer task completed successfully");
            Ok(())
        }
        Ok(Err(err)) => {
            error!(error = %err, "Consumer broker error");
            anyhow::bail!("Consumer broker error: {}", err)
        }
        Err(err) => {
            error!(error = %err, "Consumer spawn error");
            anyhow::bail!("Consumer spawn error: {}", err)
        }
    }
}

/// Ingest one live or finalized payload. A finalized payload re-ingests the
/// block idempotently and marks it finalized. Then spawn a KMS activation pass.
async fn ingest_payload<A: AwsS3Interface + Clone + 'static>(
    db: &mut Database,
    config: &ConsumerConfig,
    ingest_options: &IngestOptions,
    chain_id_str: &str,
    payload: &BlockPayload,
    s3: A,
    kms_cancel: &CancellationToken,
) -> Result<AckDecision, HandlerError> {
    let chain_id = db.chain_id;
    let finalized = payload.flow == BlockFlow::Final;
    let block_summary = BlockSummary {
        number: payload.block_number,
        hash: payload.block_hash,
        parent_hash: payload.parent_hash,
        timestamp: payload.timestamp,
    };
    let catchup = payload.flow == BlockFlow::Catchup;
    // Finalized deliveries do not measure live ingestion timing.
    if !finalized {
        observe_consumer_block_timing(
            db,
            chain_id_str,
            &block_summary,
            catchup,
        )
        .await;
    }
    let logs = collect_logs(payload);
    info!(
        chain_id = %payload.chain_id,
        block_number = payload.block_number,
        block_hash = ?payload.block_hash,
        nb_tx = payload.transactions.len(),
        nb_logs = logs.len(),
        "Received new block payload"
    );
    let block_logs = BlockLogs {
        summary: block_summary,
        logs,
        catchup: false,
        finalized,
    };
    match ingest_with_retry(
        chain_id,
        db,
        &block_logs,
        config.acl_address,
        config.tfhe_address,
        config.kms_generation_address,
        config.protocol_config_address,
        config.confidential_bridge_address,
        config.database_retry_interval,
        ingest_options.clone(),
    )
    .await
    {
        Ok(_) => {
            db.tick.update();
            if finalized {
                if let Err(error) = db
                    .prune_finalized_block_history(block_summary.number as i64)
                    .await
                {
                    warn!(%error, "Could not prune finalized block history");
                }
            }
            // As the legacy listener does, retry KMS activations after
            // each block: they only depend on database state. A
            // finalized block also provides the finalized height.
            if config.kms_generation_address.is_some() {
                spawn_kms_activations(
                    db.clone(),
                    finalized.then_some(block_summary.number as i64),
                    s3,
                    kms_cancel.clone(),
                );
            }

            inc_blocks_processed(chain_id_str, 1);
            Ok(AckDecision::Ack)
        }
        Err((err, retries)) => {
            inc_db_errors(chain_id_str, 1);
            error!(
                block_number = block_summary.number,
                block_hash = ?block_logs.summary.hash,
                error = %err,
                retries = retries,
                "Failed to ingest block"
            );
            Err(HandlerError::Transient(err.into()))
        }
    }
}

/// Spawn one KMS activation pass: cancel orphaned candidates, activate
/// finalized ready keys/CRS, and download and verify pending material.
/// The pass is skipped while a drift revert is not done, and stops on
/// shutdown, releasing its transaction.
fn spawn_kms_activations<A: AwsS3Interface + Clone + 'static>(
    db: Database,
    finalized_height: Option<i64>,
    s3: A,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        let pass = async {
            let pool = db.pool().await;
            // The cached drift state is only refreshed by block handling, so
            // read the latest signal: a revert may start after the block gate.
            let signal =
                fhevm_engine_common::drift_revert::latest_signal_for_chain(
                    &pool,
                    db.chain_id.as_i64(),
                )
                .await?;
            if let Some(signal) =
                signal.filter(|signal| signal.status != DriftStatus::Done)
            {
                info!(
                    drift_id = signal.id,
                    status = ?signal.status,
                    "Skipping KMSGeneration activations while a drift revert is not done"
                );
                return Ok(0);
            }
            process_kms_generation_activations(pool, s3, finalized_height).await
        };
        tokio::select! {
            _ = cancel.cancelled() => {}
            result = pass => {
                if let Err(error) = result {
                    error!(%error, "Error processing KMSGeneration activations");
                }
            }
        }
    });
}

/// Stop the live flow once the cutover retires this stack.
///
/// `spawn_stack_version_listener` sets `paused` but leaves the reader running,
/// which is correct while the stack is up: the handler acks and drops, so the
/// cursor keeps moving and nothing is pinned. It is only once the pods go that
/// a frozen cursor starts holding the stream. Stopping the reader here is what
/// lets the caller release the group afterwards.
///
/// Polled rather than signalled: `paused` is a plain flag, and a load every
/// [`RETIREMENT_POLL_INTERVAL`] costs nothing next to reacting to it late.
/// There is no path back — `paused` is only ever set, never cleared — so this
/// returns as soon as it fires.
fn spawn_retirement_watcher(
    stack_mode: Arc<StackMode>,
    client: ListenerConsumer,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        if awaiting_retirement(
            || stack_mode.is_paused(),
            &cancel,
            RETIREMENT_POLL_INTERVAL,
        )
        .await
        {
            warn!("Stack retired by cutover: stopping the consumer read loop");
            client.cancel_live();
        }
    });
}

/// Resolve to `true` once the stack is retired, or `false` if shutdown came
/// first.
///
/// Split out from the spawn above so the decision can be tested without a
/// broker: which of the two it returns is the whole of the logic, and acting on
/// it is one call. The check is a predicate rather than the mode itself because
/// `paused` has no public setter — a one-way latch should not have one — and a
/// test that cannot arrange the condition cannot assert on it.
async fn awaiting_retirement(
    retired: impl Fn() -> bool,
    cancel: &CancellationToken,
    poll_interval: Duration,
) -> bool {
    loop {
        if retired() {
            return true;
        }
        tokio::select! {
            _ = cancel.cancelled() => return false,
            _ = sleep(poll_interval) => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn ingest_with_retry(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_address: Address,
    tfhe_address: Address,
    kms_generation_address: Option<Address>,
    protocol_config_address: Option<Address>,
    confidential_bridge_address: Option<Address>,
    retry_interval: Duration,
    options: IngestOptions,
) -> Result<u64, (sqlx::Error, u64)> {
    let mut errors = 0;
    let acl = Some(acl_address);
    let tfhe = Some(tfhe_address);
    let protocol_config = protocol_config_address;
    loop {
        match ingest_block_logs(
            chain_id,
            db,
            block_logs,
            &acl,
            &tfhe,
            &kms_generation_address,
            &protocol_config,
            &confidential_bridge_address,
            options.clone(),
        )
        .await
        {
            Ok(_) => return Ok(errors),
            Err(err) => {
                errors += 1;
                if errors > MAX_DB_RETRIES {
                    return Err((err, errors));
                }
                warn!(
                    block = ?block_logs.summary.number,
                    retries = errors,
                    error = %err,
                    "Retrying block ingestion"
                );
                db.reconnect().await;
                sleep(retry_interval).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TICK: Duration = Duration::from_millis(5);

    /// A stack that is already retired is seen as such without waiting out a
    /// poll.
    ///
    /// The restart case: a pod coming back up after the cutover must not spend
    /// a poll interval behaving as though it were live.
    #[tokio::test]
    async fn retirement_already_in_effect_is_seen_immediately() {
        assert!(
            awaiting_retirement(|| true, &CancellationToken::new(), TICK).await
        );
    }

    /// Retirement arriving while the watcher is waiting is picked up.
    ///
    /// This is the cutover itself, and the reason the wait is a loop rather
    /// than a single read: `paused` is a plain flag with nothing to wake on, so
    /// a watcher that only looked once at startup would never fire.
    #[tokio::test]
    async fn retirement_arriving_later_is_picked_up_by_the_poll() {
        let retired = Arc::new(AtomicBool::new(false));
        let cancel = CancellationToken::new();

        let setter = {
            let retired = Arc::clone(&retired);
            tokio::spawn(async move {
                sleep(TICK * 4).await;
                retired.store(true, Ordering::SeqCst);
            })
        };

        assert!(tokio::time::timeout(
            Duration::from_secs(5),
            awaiting_retirement(
                || retired.load(Ordering::SeqCst),
                &cancel,
                TICK
            ),
        )
        .await
        .expect("the poll must observe a flag set while it waits"));
        setter.await.unwrap();
    }

    /// An ordinary shutdown is not a retirement.
    ///
    /// Reporting one here would have the caller destroy the consumer group of a
    /// stack that is merely restarting. On the way back up it would find no
    /// group, reseed at the tip of the stream, and silently skip whatever was
    /// published while it was down.
    #[tokio::test]
    async fn shutdown_without_a_cutover_is_not_a_retirement() {
        let cancel = CancellationToken::new();
        cancel.cancel();

        assert!(!awaiting_retirement(|| false, &cancel, TICK).await);
    }
}
