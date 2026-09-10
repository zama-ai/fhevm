use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use alloy::primitives::Address;
use alloy::rpc::types::Log;
use alloy_primitives::LogData;
use tokio::time::{interval_at, Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use fhevm_engine_common::utils::DatabaseURL;

use crate::cmd::block_history::BlockSummary;
use crate::consumer::metrics::{
    inc_blocks_duplicated, inc_blocks_missing, inc_db_errors,
    observe_legacy_insert_delay_seconds,
};
use crate::database::tfhe_event_propagate::Database;

use consumer::BlockPayload;
pub mod catchup;
mod drift_recovery;
#[cfg(test)]
mod finalization_tests;
mod ingestion;
mod runner;
pub use runner::run_consumer;
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
    pub kms_generation_address: Address,
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
