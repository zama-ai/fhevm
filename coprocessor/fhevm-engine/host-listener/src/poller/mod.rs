mod http_client;
mod metrics;

use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::rpc::types::Log;
use alloy::transports::http::reqwest::Url;
use anyhow::{anyhow, Context, Result};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::database::connect_pool_with_options;
use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};
use fhevm_engine_common::versioning::{run_stack_version_listener, StackMode};
use sqlx::postgres::PgPoolOptions;

use crate::cmd::block_history::BlockSummary;
use crate::database::ingest::{
    ingest_block_logs, update_finalized_blocks_aux, BlockLogs, IngestOptions,
};
use crate::database::tfhe_event_propagate::Database;
use crate::health_check::HealthCheck;
use crate::kms_generation::aws_s3::AwsS3Client;
use crate::kms_generation::process_kms_generation_activations;
use crate::poller::http_client::HttpChainClient;
use crate::poller::metrics::{
    inc_blocks_processed, inc_db_errors, inc_rpc_errors,
};

const MAX_DB_RETRIES: u64 = 10;
/// Exit after this many consecutive RPC failures (after retries exhausted).
/// Orchestrator will restart with fresh state.
const MAX_CONSECUTIVE_RPC_FAILURES: u64 = 3;

fn reorg_replay_anchor(
    last_caught_up_block: u64,
    reorg_maximum_duration_in_blocks: u64,
) -> u64 {
    last_caught_up_block.saturating_sub(reorg_maximum_duration_in_blocks)
}

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

/// Seed for `host_listener_poller_state` when no anchor row exists yet; an
/// existing row always wins in the caller, so restarts never rewind or skip.
/// A non-negative value is an absolute height (0 = genesis, explicitly), a
/// negative value means that many blocks behind the current head. Unset is an
/// error: the first start for a chain must state where to begin.
#[derive(Debug, PartialEq, Eq)]
enum StartAnchor {
    Block(i64),
    BehindHead(u64),
}

fn resolve_start_anchor(seed_start_block: Option<i64>) -> Result<StartAnchor> {
    match seed_start_block {
        Some(block) if block >= 0 => Ok(StartAnchor::Block(block)),
        Some(delta) => Ok(StartAnchor::BehindHead(delta.unsigned_abs())),
        None => Err(anyhow!(
            "no poller state for this chain and no --seed-start-block; \
             set it explicitly (0 for genesis)"
        )),
    }
}

#[cfg(test)]
mod start_anchor_tests {
    use super::{resolve_start_anchor, StartAnchor};

    #[test]
    fn non_negative_flag_is_absolute() {
        assert_eq!(
            resolve_start_anchor(Some(7)).unwrap(),
            StartAnchor::Block(7)
        );
        assert_eq!(
            resolve_start_anchor(Some(0)).unwrap(),
            StartAnchor::Block(0)
        );
    }

    #[test]
    fn negative_flag_is_behind_head() {
        assert_eq!(
            resolve_start_anchor(Some(-10_000)).unwrap(),
            StartAnchor::BehindHead(10_000)
        );
    }

    #[test]
    fn no_flag_is_an_error() {
        assert!(resolve_start_anchor(None).is_err());
    }
}

#[derive(Clone, Debug)]
pub struct PollerConfig {
    pub url: String,
    pub acl_address: Address,
    pub tfhe_address: Address,
    pub kms_generation_address: Option<Address>,
    pub protocol_config_address: Option<Address>,
    pub confidential_bridge_address: Option<Address>,
    pub database_url: DatabaseURL,
    pub finality_lag: u64,
    pub settlement_finality_lag: u64,
    pub reorg_maximum_duration_in_blocks: u64,
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
    /// Initial sync anchor when no poller state exists yet
    /// (initialization-only; >= 0 absolute, negative from head).
    pub seed_start_block: Option<i64>,
    // Dependence chain settings
    pub dependence_cache_size: u16,
    pub dependence_cross_block: bool,
    pub dependent_ops_max_per_chain: u32,
    pub gcs_mode: bool,
    pub canonical_protocol_config_chain_id: Option<u64>,
}

pub async fn run_poller(config: PollerConfig) -> Result<()> {
    let acl_address = config.acl_address;
    let tfhe_address = config.tfhe_address;
    let kms_generation_address = config.kms_generation_address;
    let protocol_config_address = config.protocol_config_address;
    let confidential_bridge_address = config.confidential_bridge_address;

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
        kms_generation_address,
        confidential_bridge_address,
        config.retry_interval,
        config.max_http_retries,
        config.rpc_compute_units_per_second,
    )?;

    let chain_id = match client.chain_id().await {
        Ok(id) => ChainId::try_from(id)
            .context("chain id from provider is out of range")?,
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
    let is_protocol_config_listener =
        crate::protocol_config::resolve_protocol_config_listener(
            config.canonical_protocol_config_chain_id,
            chain_id.as_u64(),
        )?;
    blockchain_timeout_tick.update();

    let mut db = Database::new_with_gcs_mode(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
        config.gcs_mode,
    )
    .await?;
    let aws_s3_client = AwsS3Client {};

    let health_check = HealthCheck {
        blockchain_timeout_tick: blockchain_timeout_tick.clone(),
        blockchain_tick: blockchain_tick.clone(),
        blockchain_provider: blockchain_provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
    };
    let cancel_token = CancellationToken::new();
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check),
        config.health_port,
        cancel_token.clone(),
    );
    tokio::spawn(async move {
        if let Err(err) = health_check_server.start().await {
            error!(error = %err, "Health check server failed");
        }
    });

    // Drift-revert: must run before any DB state reads so we don't read
    // pre-revert state.
    let (drift_revert_pool, _pool_refresh_handle) = connect_pool_with_options(
        &config.database_url,
        PgPoolOptions::new().max_connections(1),
        Some(&cancel_token),
    )
    .await?;
    fhevm_engine_common::drift_revert::init(
        drift_revert_pool,
        cancel_token.clone(),
        None,
        fhevm_engine_common::drift_revert::WatcherTimeouts::default(),
    )
    .await?;

    let _branch_cleanup_worker =
        db.spawn_orphaned_branch_cleanup_worker(cancel_token.clone());

    if config.dependent_ops_max_per_chain == 0 {
        let promoted = db.promote_all_dep_chains_to_fast_priority().await?;
        if promoted > 0 {
            info!(
                count = promoted,
                "Slow-lane disabled: promoted all chains to fast on startup"
            );
        }
    }

    // Runtime stack mode + `event_stack_version_upgraded` listener: at cutover
    // this (blue) stack is retired and `stack_mode` flips to paused, turning
    // the poll loop below into a no-op (stops polling/producing blocks).
    let stack_mode = StackMode::new(config.gcs_mode);
    {
        let pool = db.pool().await;
        let stack_mode = stack_mode.clone();
        let cancel = cancel_token.clone();
        tokio::spawn(async move {
            if let Err(err) =
                run_stack_version_listener(pool, stack_mode, cancel).await
            {
                error!(error = %err, "stack-version listener exited with error");
            }
        });
    }

    let initial_anchor = db.poller_get_last_caught_up_block(chain_id).await?;
    db.tick.update();
    let effective_settlement_finality_lag = config
        .settlement_finality_lag
        .max(config.reorg_maximum_duration_in_blocks);
    let persisted_caught_up_block = match initial_anchor {
        Some(block) => u64::try_from(block)
            .context("last_caught_up_block cannot be negative")?,
        None => {
            let initial = match resolve_start_anchor(config.seed_start_block)? {
                StartAnchor::Block(block) => block,
                StartAnchor::BehindHead(delta) => {
                    let head = client.latest_block_number().await.context(
                        "Failed to fetch head to resolve negative seed-start-block",
                    )?;
                    i64::try_from(head.saturating_sub(delta)).context(
                        "start block computed from head is out of range",
                    )?
                }
            };
            db.poller_set_last_caught_up_block(chain_id, initial)
                .await?;
            db.tick.update();
            u64::try_from(initial)
                .context("initial last_caught_up_block cannot be negative")?
        }
    };
    // The websocket listener detects reorgs from its in-memory header
    // history. If it and the poller restart while the node switches branches,
    // that notification is lost and the durable forward-only poller anchor
    // would skip the rewritten blocks. Re-read the configured reorg window on
    // every startup; ingestion is idempotent, and canonical finalization then
    // marks any previously observed sibling branch orphaned.
    let mut last_caught_up_block = reorg_replay_anchor(
        persisted_caught_up_block,
        config.reorg_maximum_duration_in_blocks,
    );
    let mut durable_caught_up_block = persisted_caught_up_block;

    info!(
        chain_id = %chain_id,
        durable_caught_up_block = durable_caught_up_block,
        last_caught_up_block = last_caught_up_block,
        finality_lag = config.finality_lag,
        settlement_finality_lag = config.settlement_finality_lag,
        reorg_maximum_duration_in_blocks = config.reorg_maximum_duration_in_blocks,
        effective_settlement_finality_lag = effective_settlement_finality_lag,
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
        // Paused (retired blue stack after cutover): no-op — stop polling.
        if stack_mode.is_paused() {
            sleep(config.poll_interval).await;
            continue;
        }
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
        let client_ref = &client;
        update_finalized_blocks_aux(
            &mut db,
            latest,
            config.finality_lag,
            effective_settlement_finality_lag,
            |block_number| async move {
                client_ref
                    .header_for_block(block_number)
                    .await
                    .map(|header| header.hash)
            },
        )
        .await;

        if safe_tip <= last_caught_up_block {
            info!(
                chain_id = %chain_id,
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
                finalized: true,
            };

            let ingest_options = IngestOptions {
                dependence_cross_block: config.dependence_cross_block,
                dependent_ops_max_per_chain: config.dependent_ops_max_per_chain,
                is_protocol_config_listener,
            };
            match ingest_with_retry(
                chain_id,
                &mut db,
                &block_logs,
                acl_address,
                tfhe_address,
                kms_generation_address,
                protocol_config_address,
                confidential_bridge_address,
                config.retry_interval,
                ingest_options,
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
            if kms_generation_address.is_some() {
                let db_pool = db.pool.read().await.clone();
                tokio::spawn(async move {
                    if let Err(err) = process_kms_generation_activations(
                        db_pool,
                        aws_s3_client,
                    )
                    .await
                    {
                        error!(
                            error = %err,
                            "Error processing KMSGeneration activations"
                        );
                    }
                });
            }
        }

        let new_anchor = last_caught_up_block + processed_blocks;
        let blocks_failed = blocks_to_process.saturating_sub(processed_blocks);

        if new_anchor > last_caught_up_block {
            // Keep the durable anchor at its pre-replay high-water mark until
            // replay has caught up. Persisting an intermediate replay height
            // would make repeated restarts walk the anchor backwards by one
            // reorg window each time.
            if new_anchor > durable_caught_up_block {
                let anchor = i64::try_from(new_anchor)
                    .context("last_caught_up_block overflow")?;
                db.poller_set_last_caught_up_block(chain_id, anchor).await?;
                db.tick.update();
                durable_caught_up_block = new_anchor;
            }
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
            chain_id = %chain_id,
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
    use super::reorg_replay_anchor;

    #[test]
    fn replay_anchor_rewinds_by_the_reorg_window() {
        assert_eq!(reorg_replay_anchor(100, 8), 92);
    }

    #[test]
    fn replay_anchor_saturates_at_genesis() {
        assert_eq!(reorg_replay_anchor(5, 8), 0);
    }
}
