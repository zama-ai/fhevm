use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy_primitives::LogData;
use anyhow::Result;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{error, info, warn};

use fhevm_engine_common::drift_revert::SignalStatus as DriftStatus;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};

use crate::cmd::block_history::BlockSummary;
use crate::consumer::metrics::{inc_blocks_processed, inc_db_errors};
use crate::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use crate::database::tfhe_event_propagate::Database;
use crate::health_check::HealthCheck;

use consumer::{
    AckDecision, BlockPayload, Broker, HandlerError, ListenerConsumer,
};
mod metrics;

const MAX_DB_RETRIES: u64 = 10;

#[derive(Clone, Debug)]
pub struct ConsumerConfig {
    pub url: String,
    pub acl_address: Address,
    pub tfhe_address: Address,
    pub kms_generation_address: Address,
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
                    > last_known_drift.catchup_to
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
    return Ok(false);
}


pub async fn promote_once_all_chains_to_fast(db: &Database, dependent_ops_max_per_chain: u32) {
    if dependent_ops_max_per_chain == 0 {
        let count = match db.promote_all_dep_chains_to_fast_priority().await {
            Ok(count) => count,
            Err(err) => {
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

pub async fn run_consumer(config: ConsumerConfig) -> Result<()> {
    info!("Starting consumer with config: {:?}", config);
    let contracts = vec![config.acl_address, config.tfhe_address, config.kms_generation_address];
    let chain_id: u64 = config.chain_id.parse()?;
    let chain_id = ChainId::try_from(chain_id)?;

    let blockchain_tick = HeartBeat::new();
    let blockchain_timeout_tick = HeartBeat::new();
    let blockchain_provider = Arc::new(RwLock::new(None));

    let broker_url = config.url; // e.g."amqp://user:pass@localhost:5672";
    let broker = Broker::from_url(&broker_url).await?;
    let consumer_id = format!("{}.{}", config.service_name, config.chain_id);
    let client =
        ListenerConsumer::new(&broker, chain_id.as_u64(), &consumer_id);

    let db = Database::new(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
    )
    .await?;

    db.tick.update();

    info!("Consumer registering contracts");
    client.register_contracts(&contracts).await?;
    info!("Consumer ensure queue");
    client.ensure_consumer().await?;

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
    };

    let last_known_drift = Arc::new(RwLock::new(STARTING_DRIFT));
    let chain_id_str = config.chain_id.to_string();
    let consumer_task = client.consume(move |payload, _cancel| {
        blockchain_tick.update();
        let mut db = db.clone();
        let chain_id_str = chain_id_str.clone();
        let last_known_drift = last_known_drift.clone();
        async move {
            let drift_revert_is_over = check_if_drift_revert_is_over(
                &db,
                chain_id.as_u64() as i64,
                last_known_drift,
                payload.block_number,
            ).await;
            match drift_revert_is_over {
                Ok(false) => return Err(HandlerError::Transient(Box::from("Drift in progress"))),
                Ok(true) => (), // all good
                Err(err) => error!(%err, "Can't check drift-revert status"),
            }
            promote_once_all_chains_to_fast(&db, ingest_options.dependent_ops_max_per_chain).await;
            let block_summary = BlockSummary {
                number: payload.block_number,
                hash: payload.block_hash,
                parent_hash: payload.parent_hash,
                timestamp: payload.timestamp,
            };
            let logs = collect_logs(&payload);
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
                finalized: false,
            };
            match ingest_with_retry(
                chain_id,
                &mut db,
                &block_logs,
                config.acl_address,
                config.tfhe_address,
                config.kms_generation_address,
                config.database_retry_interval,
                ingest_options,
            )
            .await
            {
                Ok(_) => {
                    db.tick.update();
                    inc_blocks_processed(&chain_id_str, 1);
                    Ok(AckDecision::Ack)
                }
                Err((err, retries)) => {
                    inc_db_errors(&chain_id_str, 1);
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
    });

    info!(
        chain_id = %config.chain_id,
        "Starting host-listener consumer"
    );
    let consumer_run = tokio::spawn(consumer_task);
    let consumer_result = consumer_run.await;
    info!(
        chain_id = %config.chain_id,
        "Host listener consumer graceful stop"
    );
    client.cancel();
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

#[allow(clippy::too_many_arguments)]
async fn ingest_with_retry(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_address: Address,
    tfhe_address: Address,
    kms_generation_address: Address,
    retry_interval: Duration,
    options: IngestOptions,
) -> Result<u64, (sqlx::Error, u64)> {
    let mut errors = 0;
    let acl = Some(acl_address);
    let tfhe = Some(tfhe_address);
    let kms_generation = Some(kms_generation_address);
    loop {
        match ingest_block_logs(
            chain_id,
            db,
            block_logs,
            &acl,
            &tfhe,
            &kms_generation,
            options,
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
