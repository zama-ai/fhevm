//! Shared live/replay ingestion and per-block guards. The runner owns restarts.
use super::{catchup::LiveReference, drift_recovery};
use alloy::{primitives::Address, rpc::types::Log};
use consumer::{AckDecision, BlockPayload, HandlerError};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::{
    drift_revert::DriftRevertSignal, utils::HeartBeat, versioning::StackMode,
};
use primitives::event::BlockFlow;
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
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

/// Shared state and processing for live and replay deliveries in one subscription.
pub(super) struct BlockIngestor {
    pub db: Database,
    pub chain_id: ChainId,
    pub config: ConsumerConfig,
    pub options: IngestOptions,
    pub live_reference: LiveReference,
    pub checkpoint: Option<DriftRevertSignal>,
    pub drift_tx: mpsc::UnboundedSender<DriftRevertSignal>,
    pub mode: Arc<StackMode>,
    pub tick: HeartBeat,
    pub cancel: CancellationToken,
    // For graceful shutdown, each payload handler (and KMS pass) holds a read
    // lock until its processing future exits. Any held read lock means work
    // has not finished yet. After cancellation, acquiring the write lock in
    // stop() confirms that all such work has exited before replay restarts.
    // Later calls observe the cancelled token and skip processing. This does
    // not wait for broker acknowledgement tasks, only our processing futures.
    pub in_flight_handlers: RwLock<()>,
}

impl BlockIngestor {
    pub async fn ingest(
        &self,
        payload: BlockPayload,
    ) -> Result<AckDecision, HandlerError> {
        let _active = self.in_flight_handlers.read().await;
        tokio::select! {
            biased;
            _ = self.cancel.cancelled() => Ok(AckDecision::Ack),
            result = self.handle_block(payload) => result,
        }
    }

    pub(super) async fn stop(&self) {
        self.cancel.cancel();
        let _drained = self.in_flight_handlers.write().await;
    }

    async fn handle_block(
        &self,
        payload: BlockPayload,
    ) -> Result<AckDecision, HandlerError> {
        if !self.ready().await? {
            return Ok(AckDecision::Ack);
        }
        if payload.chain_id != self.chain_id.as_u64() {
            error!(
                chain_id = payload.chain_id,
                "Dropping block for wrong chain"
            );
            return Ok(AckDecision::Ack);
        }
        if payload.flow == BlockFlow::Live {
            self.tick.update();
        }
        self.live_reference.observe(&payload).await;
        self.ingest_block(payload).await
    }

    async fn ready(&self) -> Result<bool, HandlerError> {
        if self.mode.is_paused() {
            return Ok(false);
        }
        let signal = drift_recovery::read_signal(&self.db)
            .await
            .map_err(|error| HandlerError::Transient(error.into()))?;
        if !drift_recovery::same_checkpoint(
            signal.as_ref(),
            self.checkpoint.as_ref(),
        ) {
            if let Some(signal) = signal {
                let _ = self.drift_tx.send(signal);
            }
            return Err(HandlerError::Transient(
                "Drift changed; subscription must restart".into(),
            ));
        }
        Ok(true)
    }

    /// One supervised worker per delivery subscription; retry even on quiet chains.
    pub(super) async fn process_kms(&self, cancel: CancellationToken) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        interval
            .set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = interval.tick() => {},
            }
            let _active = self.in_flight_handlers.read().await;
            let result = tokio::select! {
                biased;
                _ = self.cancel.cancelled() => return,
                result = self.process_kms_once() => result,
            };
            if let Err(error) = result {
                warn!(%error, "KMS activation pass failed; retrying on next tick");
            }
        }
    }

    async fn process_kms_once(&self) -> Result<(), HandlerError> {
        if self.ready().await? {
            crate::kms_generation::process_kms_generation_activations(
                self.db.pool().await,
                crate::kms_generation::aws_s3::AwsS3Client {},
                None,
            )
            .await
            .map_err(|error| HandlerError::Transient(error.into()))?;
        }
        Ok(())
    }

    async fn ingest_block(
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
            Some(config.kms_generation_address),
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
