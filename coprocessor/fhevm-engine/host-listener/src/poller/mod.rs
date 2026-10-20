mod http_client;
mod metrics;
#[cfg(test)]
mod recovery_tests;

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
use fhevm_engine_common::versioning::{reconcile_stack_mode, StackMode};
use sqlx::postgres::PgPoolOptions;

use crate::cmd::block_history::BlockSummary;
use crate::cmd::wait_while_paused;
use crate::database::ingest::{
    ingest_block_logs, update_finalized_blocks_aux, BlockLogs, IngestOptions,
};
use crate::database::tfhe_event_propagate::{
    spawn_stack_version_listener, Database,
};
use crate::health_check::HealthCheck;
use crate::kms_generation::aws_s3::AwsS3Client;
use crate::kms_generation::process_kms_generation_activations;
use crate::poller::http_client::HttpChainClient;
use crate::poller::metrics::{
    inc_blocks_processed, inc_db_errors, inc_rpc_errors,
};

/// Separate RPC calls may observe different forks. Validate every log against
/// the requested canonical header before ingestion or a cursor write.
fn validate_log_provenance(
    requested_block: u64,
    header: &BlockSummary,
    logs: &[Log],
) -> Result<()> {
    anyhow::ensure!(
        header.number == requested_block,
        "RPC header has the wrong height"
    );
    for log in logs {
        anyhow::ensure!(
            !log.removed
                && log.block_number == Some(requested_block)
                && log.block_hash == Some(header.hash),
            "RPC log does not belong to canonical block {requested_block}"
        );
    }
    Ok(())
}

#[cfg(test)]
mod provenance_tests {
    use super::*;
    use alloy::primitives::FixedBytes;

    #[test]
    fn only_canonical_logs_are_admitted() {
        let header = BlockSummary {
            number: 7,
            hash: FixedBytes::with_last_byte(7),
            parent_hash: FixedBytes::with_last_byte(6),
            timestamp: 100,
        };
        let canonical = Log {
            block_number: Some(7),
            block_hash: Some(header.hash),
            ..Default::default()
        };
        assert!(validate_log_provenance(7, &header, &[]).is_ok());
        assert!(validate_log_provenance(
            7,
            &header,
            std::slice::from_ref(&canonical)
        )
        .is_ok());
        assert!(validate_log_provenance(
            8,
            &header,
            std::slice::from_ref(&canonical)
        )
        .is_err());
        for invalid in [
            Log {
                block_hash: None,
                ..canonical.clone()
            },
            Log {
                block_number: None,
                ..canonical.clone()
            },
            Log {
                block_hash: Some(FixedBytes::with_last_byte(8)),
                ..canonical.clone()
            },
            Log {
                block_number: Some(8),
                ..canonical.clone()
            },
            Log {
                removed: true,
                ..canonical.clone()
            },
        ] {
            assert!(validate_log_provenance(
                7,
                &header,
                &[canonical.clone(), invalid]
            )
            .is_err());
        }
    }
}

const MAX_DB_RETRIES: u64 = 10;

/// A canonical replay may repair an omission on an unchanged block. It must
/// never merge another fork into previously committed or finalized history.
async fn validate_committed_header(
    pool: &sqlx::PgPool,
    chain_id: ChainId,
    header: &BlockSummary,
) -> Result<()> {
    use fhevm_engine_common::chain_recovery::{
        host_role_tx, is_halted_tx, lock_history_tx, record_halt,
        HistoryMismatch, ListenerSource,
    };
    use fhevm_engine_common::versioning::{is_retired, CUTOVER_LOCK_ID};
    // Resolve both the role and history in one fenced transaction: a Green
    // pool may already route to public while its startup mode is still Green.
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(CUTOVER_LOCK_ID)
        .execute(&mut *tx)
        .await?;
    if is_retired(&mut tx).await? || is_halted_tx(&mut tx).await? {
        tx.rollback().await?;
        return Ok(());
    }
    let gcs_mode = host_role_tx(&mut tx).await?;
    lock_history_tx(&mut tx, ListenerSource::Host, gcs_mode, chain_id.as_i64())
        .await?;
    let height = i64::try_from(header.number)?;
    let expected: Option<Vec<u8>> = sqlx::query_scalar(
        "SELECT block_hash FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_number = $2
           AND block_status = 'finalized'
           AND block_hash <> $3 LIMIT 1",
    )
    .bind(chain_id.as_i64())
    .bind(height)
    .bind(header.hash.as_slice())
    .fetch_optional(&mut *tx)
    .await?;
    // record_halt takes the exclusive cutover fence; release the shared one.
    tx.rollback().await?;
    if let Some(expected) = expected {
        record_halt(pool, HistoryMismatch {
            source: ListenerSource::Host, gcs_mode, chain_id: chain_id.as_i64(),
            block_number: height, expected_hash: Some(&expected),
            observed_hash: Some(header.hash.as_slice()),
            reason: "canonical host header changed after commitment or finalization",
        }).await?;
        anyhow::bail!("Committed host history changed at block {height}; operator recovery required");
    }
    Ok(())
}
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

/// Seed for `host_listener_poller_state` when no anchor row exists yet; an
/// existing row remains the durable cursor; restarts separately replay the
/// configured bounded window without lowering it.
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
    /// Recent committed blocks re-fetched on every process start (1..=10_000).
    pub replay_window: u64,
    // Dependence chain settings
    pub dependence_cache_size: u16,
    pub dependence_by_connexity: bool,
    pub dependence_cross_block: bool,
    pub dependent_ops_max_per_chain: u32,
    pub gcs_mode: bool,
    pub disable_synthetic_ops: bool,
    pub canonical_protocol_config_chain_id: Option<u64>,
}

pub async fn run_poller(config: PollerConfig) -> Result<()> {
    anyhow::ensure!(
        (1..=10_000).contains(&config.replay_window),
        "poller replay window must be between 1 and 10000 blocks"
    );
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
        protocol_config_address,
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
            config.protocol_config_address,
        )?;
    blockchain_timeout_tick.update();

    let stack_mode = StackMode::new(config.gcs_mode);
    let mut db = Database::new_with_stack_mode(
        &config.database_url,
        chain_id,
        config.dependence_cache_size,
        stack_mode.clone(),
    )
    .await?;
    let aws_s3_client = AwsS3Client {};

    let health_check = HealthCheck {
        blockchain_timeout_tick: blockchain_timeout_tick.clone(),
        blockchain_tick: blockchain_tick.clone(),
        blockchain_provider: blockchain_provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
        recovery_paused: Default::default(),
    };
    let cancel_token = CancellationToken::new();
    let _cancel_on_exit = cancel_token.clone().drop_guard();
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check.clone()),
        config.health_port,
        cancel_token.clone(),
    );
    tokio::spawn(async move {
        if let Err(err) = health_check_server.start().await {
            error!(error = %err, "Health check server failed");
        }
    });

    let work = async {
        // Drift-revert: must run before any DB state reads so we don't read
        // pre-revert state.
        let (drift_revert_pool, _pool_refresh_handle) =
            connect_pool_with_options(
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

        // Runtime stack mode + `event_stack_version_upgraded` listener: at cutover
        // this (blue) stack is retired and `stack_mode` flips to paused, turning
        // the poll loop below into a no-op (stops polling/producing blocks).
        spawn_stack_version_listener(
            db.clone(),
            stack_mode.clone(),
            cancel_token.clone(),
        );

        // Seed RPC resolution must not hold the cutover fence. Each attempt re-reads
        // the routed cursor, so promotion and concurrent initialization win over a
        // seed resolved before them. Replay ownership is returned with its token.
        let mut seed = None;
        let mut promoted = config.dependent_ops_max_per_chain != 0;
        let (mut last_caught_up_block, replay) = loop {
            use fhevm_engine_common::chain_recovery::{
                prepare_host_replay, HostReplayStartup,
            };
            let pool = db.pool.read().await.clone();
            // A retired process may start after the cutover notification. Pause
            // before either guarded startup write, keeping liveness healthy.
            reconcile_stack_mode(&pool, &stack_mode).await?;
            wait_while_paused(&stack_mode, &blockchain_tick, &cancel_token)
                .await;
            if cancel_token.is_cancelled() {
                return Ok(());
            }
            let startup: Result<HostReplayStartup> = async {
            if !promoted {
                let count = db.promote_all_dep_chains_to_fast_priority().await?;
                if count > 0 {
                    info!(count, "Slow-lane disabled: promoted all chains to fast on startup");
                }
                promoted = true;
            }
            prepare_host_replay(
                &pool,
                db.gcs_mode(),
                chain_id.as_i64(),
                config.replay_window,
                seed,
            )
            .await
        }
        .await;
            match startup {
                Ok(HostReplayStartup::Ready { cursor, replay }) => {
                    db.tick.update();
                    break (u64::try_from(cursor)?, replay);
                }
                Ok(HostReplayStartup::NeedsSeed) => {
                    let initial = match resolve_start_anchor(
                        config.seed_start_block,
                    )? {
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
                    seed = Some(initial);
                }
                Err(err) => {
                    // Retirement can also race the check above while a write is
                    // waiting for the cutover fence. Only retirement becomes a
                    // pause; containment and other startup errors still fail.
                    reconcile_stack_mode(&pool, &stack_mode).await?;
                    if !stack_mode.is_paused() {
                        return Err(err);
                    }
                }
            }
        };
        let mut replay_pending = replay.is_some();
        let mut next_block = match &replay {
            Some(replay) => u64::try_from(replay.progress.from_block)?,
            None => last_caught_up_block.saturating_add(1),
        };

        info!(
            chain_id = %chain_id,
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
            wait_while_paused(&stack_mode, &blockchain_tick, &cancel_token)
                .await;
            if cancel_token.is_cancelled() {
                return Ok(());
            }
            let pool = db.pool.read().await.clone();
            if fhevm_engine_common::chain_recovery::is_halted(&pool).await? {
                anyhow::bail!(
                    "Chain history is contained; operator recovery is required"
                );
            }
            let latest = match client.latest_block_number().await {
                Ok(block) => block,
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
            consecutive_rpc_failures = 0;
            // Check the committed tip even with no new blocks. Its hash commits to
            // its entire ancestry; a changed descendant also detects a deeper fork.
            if last_caught_up_block > 0 {
                let header = match client
                    .header_for_block(last_caught_up_block)
                    .await
                {
                    Ok(header) => header,
                    Err(err) => {
                        warn!(%err, "Cannot verify committed tip; leaving progress unchanged");
                        sleep(config.retry_interval).await;
                        continue;
                    }
                };
                anyhow::ensure!(
                    header.number == last_caught_up_block,
                    "RPC returned the wrong committed-tip height"
                );
                validate_committed_header(&pool, chain_id, &header.into())
                    .await?;
            }
            let safe_tip = latest.saturating_sub(config.finality_lag);
            let rpc_finalized_block = if kms_generation_address.is_some() {
                match client.finalized_block_number().await {
                    Ok(block) => i64::try_from(block).ok(),
                    Err(err) => {
                        warn!(%err, "Cannot resolve finalized block, postponing compressed key migration");
                        None
                    }
                }
            } else {
                None
            };
            blockchain_timeout_tick.update();
            let client_ref = &client;
            let finalization = update_finalized_blocks_aux(
                &mut db,
                latest,
                config.finality_lag,
                |block_number| async move {
                    client_ref
                        .header_for_block(block_number)
                        .await
                        .map(|header| header.hash)
                },
            )
            .await;
            if let Some(missing) = finalization.missing_canonical_block {
                // Recover disconnected pending forks even when they precede the
                // restart overlap. Use the ordinary coherent block/log ingestion
                // below; the saved high watermark remains monotonic.
                next_block = next_block.min(missing);
            }

            if safe_tip < next_block {
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

            let iteration_start = next_block;
            let target = safe_tip.min(
                next_block.saturating_add(config.batch_size.saturating_sub(1)),
            );
            let blocks_to_process = target - next_block + 1;

            let mut processed_blocks = 0;
            let mut db_errors = 0;
            let mut rpc_errors = 0;

            for block in iteration_start..=target {
                let block_logs = match client.block_logs_for_number(block).await
                {
                    Ok(block_logs) => {
                        consecutive_rpc_failures = 0;
                        block_logs
                    }
                    Err(err) => {
                        handle_rpc_failure(
                        &mut consecutive_rpc_failures,
                        Some(block),
                        &err,
                        "Failed to fetch a coherent block snapshot after retries",
                    )?;
                        rpc_errors += 1;
                        break;
                    }
                };

                validate_log_provenance(
                    block,
                    &block_logs.summary,
                    &block_logs.logs,
                )?;
                validate_committed_header(&pool, chain_id, &block_logs.summary)
                    .await?;

                let ingest_options = IngestOptions {
                    dependence_by_connexity: config.dependence_by_connexity,
                    dependence_cross_block: config.dependence_cross_block,
                    dependent_ops_max_per_chain: config
                        .dependent_ops_max_per_chain,
                    is_protocol_config_listener,
                    disable_synthetic_ops: config.disable_synthetic_ops,
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
                        next_block = block.saturating_add(1);
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
                            rpc_finalized_block,
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

            let new_anchor =
                last_caught_up_block.max(next_block.saturating_sub(1));
            let blocks_failed =
                blocks_to_process.saturating_sub(processed_blocks);

            if new_anchor > last_caught_up_block {
                let anchor = i64::try_from(new_anchor)
                    .context("last_caught_up_block overflow")?;
                db.poller_set_last_caught_up_block(chain_id, anchor).await?;
                db.tick.update();
                last_caught_up_block = new_anchor;
            }
            if let Some(replay) = replay.as_ref().filter(|replay| {
                replay_pending && next_block > replay.progress.to_block as u64
            }) {
                replay_pending = replay
                    .complete(
                        &pool,
                        i64::try_from(next_block.saturating_sub(1))?,
                    )
                    .await?;
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
                first_requested_block = iteration_start,
                last_caught_up_block_after = last_caught_up_block,
                blocks_processed = processed_blocks,
                blocks_failed = blocks_failed,
                db_errors = db_errors,
                rpc_errors = rpc_errors,
                "Host listener poller iteration complete"
            );

            sleep(config.poll_interval).await;
        }
    };
    health_check.run_with_containment(&cancel_token, work).await
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
