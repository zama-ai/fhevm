use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy_primitives::LogData;
use anyhow::Result;
use tokio::sync::RwLock;
use tokio::time::{interval_at, Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::drift_revert::SignalStatus as DriftStatus;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};
use fhevm_engine_common::versioning::{run_stack_version_listener, StackMode};
use primitives::event::BlockFlow;

use crate::cmd::block_history::BlockSummary;
use crate::consumer::metrics::{
    inc_blocks_duplicated, inc_blocks_missing, inc_db_errors,
    observe_legacy_insert_delay_seconds,
};
use crate::database::ingest::IngestOptions;
use crate::database::tfhe_event_propagate::Database;
use crate::health_check::HealthCheck;

use consumer::{
    AckDecision, BlockPayload, Broker, HandlerError, ListenerConsumer,
};
pub mod catchup;
mod ingestion;
mod metrics;

const STATS_REPORT_INTERVAL: Duration = Duration::from_secs(60);
static SLOW_LANE_PROMOTION_ATTEMPTED: AtomicBool = AtomicBool::new(false);
const STATS_FINALIZATION_MARGIN: i64 = 5;

#[derive(Clone, Debug)]
pub struct ConsumerConfig {
    pub manual_catchup: catchup::ManualCatchupArgs,
    pub url: String,
    pub acl_address: Address,
    pub tfhe_address: Address,
    pub kms_generation_address: Option<Address>,
    pub protocol_config_address: Option<Address>,
    pub confidential_bridge_address: Option<Address>,
    pub database_url: DatabaseURL,
    pub database_retry_interval: Duration,
    pub service_name: String,
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
    config.manual_catchup.validate()?;
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
    let manual_catchup = config.manual_catchup.clone();
    let consumer_id = format!("{}.{}", config.service_name, config.chain_id);
    let client =
        ListenerConsumer::new(&broker, chain_id.as_u64(), &consumer_id);

    let db = Database::new_with_gcs_mode(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
        config.gcs_mode,
    )
    .await?;

    db.tick.update();

    info!("Consumer ensure queues");
    client.ensure_consumer().await?;
    if manual_catchup.enabled() {
        client.ensure_catchup_consumer().await?;
    }
    info!("Consumer registering contracts");
    client.register_contracts(&contracts).await?;

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
    let stack_mode = StackMode::new(config.gcs_mode);
    {
        let pool = db.pool().await;
        let stack_mode = stack_mode.clone();
        let cancel = client.cancel_token.clone();
        tokio::spawn(async move {
            if let Err(err) =
                run_stack_version_listener(pool, stack_mode, cancel).await
            {
                error!(error = %err, "stack-version listener exited with error");
            }
        });
    }

    let last_known_drift = Arc::new(RwLock::new(STARTING_DRIFT));
    let manual_drift_boundary = catchup::DriftBoundary::default();
    let chain_id_str = config.chain_id.to_string();
    let stats_task = tokio::spawn(run_consumer_stats_observer(
        db.clone(),
        chain_id_str.clone(),
        client.cancel_token.clone(),
    ));
    let (live_reference, head_rx) =
        catchup::LiveReference::new(chain_id.as_u64());
    let ingestor = Arc::new(ingestion::BlockIngestor {
        db: db.clone(),
        chain_id,
        config: config.clone(),
        options: ingest_options,
    });
    // Both flows use identical ingestion; the live reference and drift gate
    // remain explicit so recovery can later pause live independently.
    let handle_block =
        move |payload: BlockPayload, _cancel: CancellationToken| {
            if payload.flow != BlockFlow::Catchup {
                blockchain_tick.update();
            }
            let db = db.clone();
            let last_known_drift = last_known_drift.clone();
            let manual_drift_boundary = manual_drift_boundary.clone();
            let ingestor = ingestor.clone();
            let stack_mode = stack_mode.clone();
            let live_reference = live_reference.clone();
            async move {
                // Paused (retired blue stack after cutover): no-op — ack and drop
                // the block without writing anything to the DB.
                if stack_mode.is_paused() {
                    return Ok(AckDecision::Ack);
                }
                if payload.chain_id != chain_id.as_u64() {
                    error!(
                    payload_chain_id = payload.chain_id,
                    configured_chain_id = chain_id.as_u64(),
                    block_number = payload.block_number,
                    "Block delivered for wrong chain — broker routing misconfigured; dropping"
                );
                    return Ok(AckDecision::Ack);
                }
                live_reference.observe(&payload).await;
                if payload.flow == BlockFlow::Catchup {
                    // Temporary: discard the drifted block and its tail, even
                    // after cleanup. A later commit adds replay restart and
                    // generation isolation. Never mutate the live drift gate.
                    let pool = db.pool().await;
                    let signal = fhevm_engine_common::drift_revert::latest_signal_for_chain(
                        &pool, chain_id.as_u64() as i64,
                    ).await.map_err(|err| HandlerError::Transient(err.into()))?;
                    if manual_drift_boundary
                        .should_skip(
                            payload.block_number,
                            signal.map(|signal| {
                                signal.offending_host_block_number
                            }),
                        )
                        .await
                    {
                        info!(block_number = payload.block_number, "Skipping manual catchup block at or after drift boundary; recovery restart is not implemented yet");
                        return Ok(AckDecision::Ack);
                    }
                    return ingestor.ingest(payload).await;
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
                ingestor.ingest(payload).await
            }
        };

    info!(chain_id = %config.chain_id, "Starting host-listener consumer");
    let mut tasks = tokio::task::JoinSet::new();
    let live = client.consume(handle_block.clone());
    tasks.spawn(async move { live.await.map_err(anyhow::Error::from) });
    if manual_catchup.enabled() {
        let catchup = client.consume_catchup(handle_block);
        tasks.spawn(async move { catchup.await.map_err(anyhow::Error::from) });
    }

    let startup = async {
        if manual_catchup.enabled() {
            let reference = head_rx.await.map_err(|_| anyhow::anyhow!("Live consumer ended before providing the catchup reference"))?;
            let (start, end) = manual_catchup.resolve(reference)?;
            info!(reference, start, end, "Requesting manual catchup (inclusive); live processing continues");
            client.request_catchup(start, end).await?;
            info!(start, end, "Manual catchup request published");
        }
        Ok::<_, anyhow::Error>(())
    };
    // Supervise workers even while waiting for the first live block or while
    // publishing the request. A startup error must not leave detached workers.
    let result = tokio::select! {
        result = startup => match result {
            Err(error) => Err(error),
            Ok(()) => tokio::select! {
                result = tasks.join_next() => consumer_task_result(result),
                _ = tokio::signal::ctrl_c() => Ok(()),
                _ = client.cancel_token.cancelled() => Ok(()),
            },
        },
        result = tasks.join_next() => consumer_task_result(result),
        _ = tokio::signal::ctrl_c() => Ok(()),
        _ = client.cancel_token.cancelled() => Ok(()),
    };
    client.cancel();
    while let Some(result) = tasks.join_next().await {
        if let Err(error) = consumer_task_result(Some(result)) {
            error!(%error, "Consumer failed during shutdown");
        }
    }
    if let Err(err) = stats_task.await {
        error!(error = %err, "Consumer stats background task failed");
    }
    result
}

fn consumer_task_result(
    result: Option<Result<Result<()>, tokio::task::JoinError>>,
) -> Result<()> {
    match result {
        Some(Ok(result)) => result,
        Some(Err(error)) => Err(error.into()),
        _ => anyhow::bail!("Consumer task exited unexpectedly"),
    }
}
