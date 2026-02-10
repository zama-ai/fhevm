mod http_client;
mod metrics;

use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::rpc::types::Log;
use alloy::transports::http::reqwest::Url;
use anyhow::{anyhow, Context, Result};
use sqlx::types::Uuid;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};

use crate::cmd::block_history::BlockSummary;
use crate::database::ingest::{ingest_block_logs, BlockLogs};
use crate::database::tfhe_event_propagate::Database;
use crate::health_check::HealthCheck;
use crate::poller::http_client::HttpChainClient;
use crate::poller::metrics::{
    inc_blocks_processed, inc_db_errors, inc_rpc_errors,
};

const MAX_DB_RETRIES: u64 = 10;
/// Exit after this many consecutive RPC failures (after retries exhausted).
/// Orchestrator will restart with fresh state.
const MAX_CONSECUTIVE_RPC_FAILURES: u64 = 3;

fn handle_rpc_failure<E: std::fmt::Display>(
    consecutive_rpc_failures: &mut u64,
    block: Option<u64>,
    error: &E,
    message: &str,
) -> Result<()> {
    *consecutive_rpc_failures += 1;
    match block {
        Some(block) => error!(
            block = block,
            error = %error,
            consecutive_failures = *consecutive_rpc_failures,
            max_consecutive_failures = MAX_CONSECUTIVE_RPC_FAILURES,
            "{message}"
        ),
        None => error!(
            error = %error,
            consecutive_failures = *consecutive_rpc_failures,
            max_consecutive_failures = MAX_CONSECUTIVE_RPC_FAILURES,
            "{message}"
        ),
    };
    if *consecutive_rpc_failures >= MAX_CONSECUTIVE_RPC_FAILURES {
        Err(anyhow!(
            "Persistent RPC failure: {} consecutive failures, exiting for orchestrator restart",
            *consecutive_rpc_failures
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct PollerConfig {
    pub url: String,
    pub acl_address: Address,
    pub tfhe_address: Address,
    pub database_url: DatabaseURL,
    pub coprocessor_api_key: Uuid,
    pub finality_lag: u64,
    pub batch_size: u64,
    pub poll_interval: Duration,
    pub retry_interval: Duration,
    pub service_name: String,
    /// Maximum number of HTTP/RPC retries after the initial attempt.
    pub max_http_retries: u32,
    /// Rate limiting budget for RPC calls (compute units per second).
    /// Higher values = less throttling.
    pub rpc_compute_units_per_second: u64,
    pub health_port: u16,
    // Dependence chain settings
    pub dependence_cache_size: u16,
    pub dependence_by_connexity: bool,
    pub dependence_cross_block: bool,
}

pub async fn run_poller(config: PollerConfig) -> Result<()> {
    let _otel_guard = match telemetry::init_otel(&config.service_name) {
        Ok(otel_guard) => otel_guard,
        Err(err) => {
            error!(error = %err, "Failed to setup OTLP");
            None
        }
    };

    let acl_address = config.acl_address;
    let tfhe_address = config.tfhe_address;

    let blockchain_tick = HeartBeat::new();
    let blockchain_timeout_tick = HeartBeat::new();

    let rpc_url = Url::parse(&config.url)
        .context("Invalid url provided to host listener poller health check")?;
    let blockchain_provider = Arc::new(RwLock::new(Some(
        ProviderBuilder::new().connect_http(rpc_url.clone()),
    )));

    let client = HttpChainClient::new(
        &config.url,
        acl_address,
        tfhe_address,
        config.retry_interval,
        config.max_http_retries,
        config.rpc_compute_units_per_second,
    )?;

    let chain_id = match client.chain_id().await {
        Ok(id) => id,
        Err(err) => {
            error!(
                error = %err,
                "Failed to fetch chain id after retries"
            );
            return Err(anyhow!(
                "Failed to fetch chain id on startup: {}",
                err
            ));
        }
    };
    let chain_id_str = chain_id.to_string();
    blockchain_timeout_tick.update();

    let mut db = Database::new(
        &config.database_url,
        &config.coprocessor_api_key,
        config.dependence_cache_size,
    )
    .await?;

    if chain_id != db.chain_id {
        error!(
            chain_id_blockchain = ?chain_id,
            chain_id_db = ?db.chain_id,
            tenant_id = ?db.tenant_id,
            coprocessor_api_key = ?config.coprocessor_api_key,
            "Chain ID mismatch with database",
        );
        return Err(anyhow!(
            "Chain ID mismatch with database, blockchain: {} vs db: {}, tenant_id: {}, coprocessor_api_key: {}",
            chain_id,
            db.chain_id,
            db.tenant_id,
            config.coprocessor_api_key
        ));
    }

    let initial_anchor =
        db.poller_get_last_caught_up_block(chain_id as i64).await?;
    db.tick.update();
    let mut last_caught_up_block = match initial_anchor {
        Some(block) => u64::try_from(block)
            .context("last_caught_up_block cannot be negative")?,
        None => {
            let initial = db.read_last_valid_block().await.unwrap_or(0);
            db.poller_set_last_caught_up_block(chain_id as i64, initial)
                .await?;
            db.tick.update();
            u64::try_from(initial)
                .context("initial last_caught_up_block cannot be negative")?
        }
    };

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

    info!(
        chain_id = chain_id,
        last_caught_up_block = last_caught_up_block,
        finality_lag = config.finality_lag,
        batch_size = config.batch_size,
        poll_interval_ms = config.poll_interval.as_millis(),
        retry_interval_ms = config.retry_interval.as_millis(),
        max_http_retries = config.max_http_retries,
        max_consecutive_rpc_failures = MAX_CONSECUTIVE_RPC_FAILURES,
        "Starting host-listener poller"
    );

    // Track consecutive RPC failures to exit on persistent issues.
    let mut consecutive_rpc_failures: u64 = 0;

    loop {
        let latest = match client.latest_block_number().await {
            Ok(block) => {
                consecutive_rpc_failures = 0;
                block
            }
            Err(err) => {
                handle_rpc_failure(
                    &mut consecutive_rpc_failures,
                    None,
                    &err,
                    "Failed to fetch latest block number after retries",
                )?;
                sleep(config.retry_interval).await;
                continue;
            }
        };
        blockchain_timeout_tick.update();

        let safe_tip = latest.saturating_sub(config.finality_lag);
        if safe_tip <= last_caught_up_block {
            info!(
                chain_id = chain_id,
                latest_block = latest,
                safe_tip = safe_tip,
                last_caught_up_block = last_caught_up_block,
                "No new finalized blocks, sleeping"
            );
            sleep(config.poll_interval).await;
            continue;
        }

        let target = safe_tip
            .min(last_caught_up_block.saturating_add(config.batch_size));
        let blocks_to_process = target - last_caught_up_block;

        let mut processed_blocks = 0;
        let mut db_errors = 0;
        let mut rpc_errors = 0;

        for block in (last_caught_up_block + 1)..=target {
            let logs = match client.logs_for_block(block).await {
                Ok(logs) => {
                    consecutive_rpc_failures = 0;
                    logs
                }
                Err(err) => {
                    handle_rpc_failure(
                        &mut consecutive_rpc_failures,
                        Some(block),
                        &err,
                        "Failed to fetch logs for block after retries",
                    )?;
                    rpc_errors += 1;
                    break;
                }
            };

            let header = match client.header_for_block(block).await {
                Ok(header) => {
                    consecutive_rpc_failures = 0;
                    header
                }
                Err(err) => {
                    handle_rpc_failure(
                        &mut consecutive_rpc_failures,
                        Some(block),
                        &err,
                        "Failed to fetch header for block after retries",
                    )?;
                    rpc_errors += 1;
                    break;
                }
            };

            let summary: BlockSummary = header.into();
            let block_logs = BlockLogs {
                logs,
                summary,
                catchup: true,
            };

            match ingest_with_retry(
                chain_id,
                &mut db,
                &block_logs,
                acl_address,
                tfhe_address,
                config.retry_interval,
                config.dependence_by_connexity,
                config.dependence_cross_block,
            )
            .await
            {
                Ok(retries) => {
                    db_errors += retries;
                    processed_blocks += 1;
                    db.tick.update();
                }
                Err((err, retries)) => {
                    db_errors += retries;
                    error!(
                        block = block,
                        block_hash = ?block_logs.summary.hash,
                        error = %err,
                        retries = retries,
                        "Failed to ingest block"
                    );
                    break;
                }
            }
        }

        let new_anchor = last_caught_up_block + processed_blocks;
        let blocks_failed = blocks_to_process.saturating_sub(processed_blocks);

        if new_anchor > last_caught_up_block {
            let anchor = i64::try_from(new_anchor)
                .context("last_caught_up_block overflow")?;
            db.poller_set_last_caught_up_block(chain_id as i64, anchor)
                .await?;
            db.tick.update();
            last_caught_up_block = new_anchor;
        }

        if processed_blocks > 0 {
            blockchain_tick.update();
        }

        inc_blocks_processed(&chain_id_str, processed_blocks);
        if db_errors > 0 {
            inc_db_errors(&chain_id_str, db_errors);
        }
        if rpc_errors > 0 {
            inc_rpc_errors(&chain_id_str, rpc_errors);
        }

        info!(
            chain_id = chain_id,
            latest_block = latest,
            safe_tip = safe_tip,
            last_caught_up_block_before = new_anchor - processed_blocks,
            last_caught_up_block_after = last_caught_up_block,
            blocks_processed = processed_blocks,
            blocks_failed = blocks_failed,
            db_errors = db_errors,
            rpc_errors = rpc_errors,
            "Host listener poller iteration complete"
        );

        sleep(config.poll_interval).await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn ingest_with_retry(
    chain_id: u64,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_address: Address,
    tfhe_address: Address,
    retry_interval: Duration,
    dependency_by_connexity: bool,
    dependency_cross_block: bool,
) -> Result<u64, (sqlx::Error, u64)> {
    let mut errors = 0;
    let acl = Some(acl_address);
    let tfhe = Some(tfhe_address);
    loop {
        match ingest_block_logs(
            chain_id,
            db,
            block_logs,
            &acl,
            &tfhe,
            dependency_by_connexity,
            dependency_cross_block,
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
