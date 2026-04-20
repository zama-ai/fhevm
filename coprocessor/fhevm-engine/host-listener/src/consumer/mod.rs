use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy_primitives::LogData;
use anyhow::Result;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

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

pub async fn run_consumer(config: ConsumerConfig) -> Result<()> {
    info!("Starting consumer with config: {:?}", config);
    let contracts = vec![config.acl_address, config.tfhe_address];
    let chain_id: u64 = config.chain_id.parse()?;
    let chain_id = ChainId::try_from(chain_id)?;

    let blockchain_tick = HeartBeat::new();
    let blockchain_timeout_tick = HeartBeat::new();
    let blockchain_provider = Arc::new(RwLock::new(None));

    let broker_url = config.url; // e.g."amqp://user:pass@localhost:5672";
    let broker = Broker::from_url(&broker_url).await?;
    let consumer_id =
        format!("{}.{}", "copro-eth-host-consumer", config.chain_id);
    let client =
        ListenerConsumer::new(&broker, chain_id.as_u64(), &consumer_id);

    let db = Database::new(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
    )
    .await?;
    if config.dependent_ops_max_per_chain == 0 {
        let promoted = db.promote_all_dep_chains_to_fast_priority().await?;
        if promoted > 0 {
            info!(
                count = promoted,
                "Slow-lane disabled: promoted all chains to fast on startup"
            );
        }
    }
    db.tick.update();

    info!("Consumer registering contracts");
    client.register_contracts(&contracts).await.unwrap();
    info!("Consumer ensure queue");
    client.ensure_consumer().await.unwrap();

    let health_check = HealthCheck {
        blockchain_timeout_tick: blockchain_timeout_tick.clone(),
        blockchain_tick: blockchain_tick.clone(),
        blockchain_provider: blockchain_provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
    };
    let health_check_cancel_token = CancellationToken::new();
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check),
        config.health_port,
        health_check_cancel_token.clone(),
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

    let chain_id_str = config.chain_id.to_string();
    let consumer_task = client.consume(move |payload, _cancel| {
        blockchain_tick.update();
        let mut db = db.clone();
        let chain_id_str = chain_id_str.clone();
        async move {
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
    match consumer_result {
        Ok(Ok(())) => info!("Consumer task completed successfully"),
        Ok(Err(err)) => error!(error = %err, "Consumer task failed with error"),
        Err(err) => error!(error = %err, "Consumer task panicked"),
    }
    client.cancel();
    health_check_cancel_token.cancel();
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn ingest_with_retry(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_address: Address,
    tfhe_address: Address,
    retry_interval: Duration,
    options: IngestOptions,
) -> Result<u64, (sqlx::Error, u64)> {
    let mut errors = 0;
    let acl = Some(acl_address);
    let tfhe = Some(tfhe_address);
    loop {
        match ingest_block_logs(chain_id, db, block_logs, &acl, &tfhe, options)
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
