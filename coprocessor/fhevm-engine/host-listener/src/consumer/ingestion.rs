//! Shared live/replay ingestion. Flow coordination stays in the consumer runner.
use alloy::{primitives::Address, rpc::types::Log};
use consumer::{AckDecision, BlockPayload, HandlerError};
use fhevm_engine_common::chain_id::ChainId;
use primitives::event::BlockFlow;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use super::metrics::{inc_blocks_processed, inc_db_errors};
use super::{
    collect_logs, observe_consumer_block_timing,
    promote_once_all_chains_to_fast, ConsumerConfig,
};
use crate::cmd::block_history::BlockSummary;
use crate::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use crate::database::tfhe_event_propagate::Database;

const MAX_DB_RETRIES: u64 = 10;

pub(super) struct BlockIngestor {
    pub db: Database,
    pub chain_id: ChainId,
    pub config: ConsumerConfig,
    pub options: IngestOptions,
}

impl BlockIngestor {
    pub async fn ingest(
        &self,
        payload: BlockPayload,
    ) -> Result<AckDecision, HandlerError> {
        let mut db = self.db.clone();
        let chain_id = self.chain_id;
        let chain_id_str = &self.config.chain_id;
        let config = &self.config;
        let ingest_options = &self.options;
        promote_once_all_chains_to_fast(
            &db,
            ingest_options.dependent_ops_max_per_chain,
        )
        .await;
        let block_summary = BlockSummary {
            number: payload.block_number,
            hash: payload.block_hash,
            parent_hash: payload.parent_hash,
            timestamp: payload.timestamp,
        };
        let catchup = payload.flow == BlockFlow::Catchup;
        // Historical and finalized deliveries do not measure live ingestion timing.
        if payload.flow == BlockFlow::Live {
            observe_consumer_block_timing(
                &db,
                chain_id_str,
                &block_summary,
                false,
            )
            .await;
        }
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
            catchup,
            finalized: false,
        };
        match ingest_with_retry(
            chain_id,
            &mut db,
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
