use crate::config::settings::GatewayConfig;
use crate::gateway::arbitrum::transaction::engine::{CustomFillers, TransactionEngine};
use crate::orchestrator::HealthCheck;
use crate::{core::errors::EventProcessingError, core::job_id::JobId, metrics};
use alloy::network::AnyTransactionReceipt;
use alloy::network::Ethereum;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::SolEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::info;

pub type TxResult = AnyTransactionReceipt;

pub type GatewayTransactionEngine = TransactionEngine<CustomFillers, RootProvider, Ethereum>;

#[derive(Debug)]
pub struct TransactionHelper {
    tx_engine: Arc<GatewayTransactionEngine>,
    pub chain_id: u64,
    health_timeout: Duration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionType {
    UserDecryptRequest,
    InputRequest,
    PublicDecryptRequest,
}

impl TransactionType {
    fn as_metrics_type(&self) -> metrics::TransactionType {
        match self {
            TransactionType::UserDecryptRequest => metrics::TransactionType::UserDecryptRequest,
            TransactionType::InputRequest => metrics::TransactionType::InputRequest,
            TransactionType::PublicDecryptRequest => metrics::TransactionType::PublicDecryptRequest,
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::InputRequest => write!(f, "input_request"),
            TransactionType::UserDecryptRequest => write!(f, "user_decrypt_request"),
            TransactionType::PublicDecryptRequest => write!(f, "public_decrypt_request"),
        }
    }
}

/// Outcome of the compare-and-set that claims the tx-in-flight transition: at most one caller
/// among concurrent sends for one job proceeds. The race it arbitrates: the sweep's claim
/// resets a `tx_in_flight` row to `processing` and re-dispatches it, with no way to tell
/// whether the previous owner's send is still executing. Within one pod the CAS separates the
/// two; across pods the row's `owner_epoch` fence does. Neither stops the old pod's send
/// itself - that duplicate is absorbed downstream (see `on_tx_in_flight`'s callers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxClaimOutcome {
    /// This caller won the claim: it owns the row and must send the transaction.
    Claimed,
    /// Another actor already claimed the row; this caller must not send.
    ClaimLost,
}

/// Outcome of recording a receipt this pod's own send obtained. The send succeeded either
/// way - the chain has a real transaction - so this never decides a retry; it says whether
/// the DB write recording the receipt landed, which the epoch fence refuses once a successor
/// has claimed the row. Callers must not treat `Refused` as `Recorded`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptRecordOutcome {
    /// The receipt was recorded.
    Recorded,
    /// Refused by the epoch fence; not recorded.
    Refused,
}

#[async_trait::async_trait]
pub trait TxLifecycleHooks: Send + Sync {
    async fn on_tx_in_flight(&self, job_id: &JobId)
        -> Result<TxClaimOutcome, EventProcessingError>;

    async fn on_receipt_received(
        &self,
        job_id: &JobId,
        receipt: &TxResult,
    ) -> Result<ReceiptRecordOutcome, EventProcessingError>;

    async fn on_failure(
        &self,
        job_id: &JobId,
        err_reason: &str,
    ) -> Result<(), EventProcessingError>;
}

impl TransactionHelper {
    pub fn new(config: GatewayConfig, tx_engine: Arc<GatewayTransactionEngine>) -> Self {
        Self {
            tx_engine,
            chain_id: config.blockchain_rpc.chain_id,
            health_timeout: Duration::from_secs(
                config.blockchain_rpc.http_health_check_timeout_secs,
            ),
        }
    }

    pub async fn send_raw_transaction_sync<H>(
        &self,
        transaction_type: TransactionType,
        job_id: JobId,
        hook: &H,
        target: Address,
        calldata_bytes: Bytes,
    ) -> Result<(), EventProcessingError>
    where
        H: TxLifecycleHooks + ?Sized,
    {
        let tx_metric_type = transaction_type.as_metrics_type();

        info!(
            operation = %transaction_type,
            calldata = %format!("0x{}...", hex::encode(&calldata_bytes[..std::cmp::min(20, calldata_bytes.len())])),
            "Preparing transaction"
        );

        metrics::transaction::transaction_broadcast(tx_metric_type);
        let transaction_start_time = Instant::now();

        // Claim before any RPC work, not just before the send: `on_failure` accepts a row in
        // `processing` or `tx_in_flight`, so a loser whose gas estimation failed after a late
        // claim could mark the winner's row `failure` while its transaction is live on chain.
        match hook.on_tx_in_flight(&job_id).await? {
            TxClaimOutcome::Claimed => {}
            TxClaimOutcome::ClaimLost => {
                // The hook impl already logged it.
                metrics::transaction::transaction_claim_lost(tx_metric_type);
                return Ok(());
            }
        }

        let request = match self
            .tx_engine
            .prepare_transaction(&job_id, target, calldata_bytes, None)
            .await
        {
            Ok(req) => req,
            Err(error) => {
                metrics::transaction::transaction_failure(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                hook.on_failure(&job_id, &error.to_string()).await?;
                return Err(EventProcessingError::from(error));
            }
        };

        let receipt = match self
            .tx_engine
            .send_raw_transaction_sync_with_retries(&job_id, request)
            .await
        {
            Ok(rec) => rec,
            Err(error) => {
                metrics::transaction::transaction_failure(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                hook.on_failure(&job_id, &error.to_string()).await?;
                return Err(EventProcessingError::from(error));
            }
        };

        // A refused receipt is still a successful send - the chain has a real transaction, the
        // hook's own log already explains the refusal - so this only decides what gets counted
        // and logged as "confirmed", never whether to retry or error. See `ReceiptRecordOutcome`.
        match hook.on_receipt_received(&job_id, &receipt).await? {
            ReceiptRecordOutcome::Recorded => {
                metrics::transaction::transaction_confirmed(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                info!(
                    operation = %transaction_type,
                    tx_hash = ?receipt.transaction_hash,
                    block_number = ?receipt.block_number,
                    gas_used = ?receipt.gas_used,
                    "Transaction confirmed"
                );
            }
            ReceiptRecordOutcome::Refused => {
                metrics::transaction::transaction_receipt_refused(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                info!(
                    operation = %transaction_type,
                    tx_hash = ?receipt.transaction_hash,
                    block_number = ?receipt.block_number,
                    "Transaction sent and landed on chain, but the receipt was not recorded \
                     (refused by the epoch fence)"
                );
            }
        }

        Ok(())
    }

    /// Extract gateway reference ID from receipt by finding and decoding the specified event
    pub fn extract_gateway_id_from_receipt<T: SolEvent>(
        receipt: &AnyTransactionReceipt,
        expected_signature: FixedBytes<32>,
        extract_id_fn: impl Fn(&T) -> U256,
    ) -> Result<U256, EventProcessingError> {
        for log in receipt.inner.logs() {
            if let Some(topic_0) = log.topics().first() {
                if *topic_0 == expected_signature {
                    match T::decode_log_data(log.data()) {
                        Ok(decoded_event) => {
                            let gw_reference_id = extract_id_fn(&decoded_event);
                            return Ok(gw_reference_id);
                        }
                        Err(e) => {
                            return Err(EventProcessingError::EventDecodingFailed {
                                event_type: T::SIGNATURE.to_string(),
                                reason: e.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Err(EventProcessingError::ValidationFailed {
            field: "transaction_logs".to_string(),
            reason: format!("{} event not found", T::SIGNATURE),
        })
    }
}

#[async_trait::async_trait]
impl HealthCheck for TransactionHelper {
    async fn check(&self) -> anyhow::Result<()> {
        match tokio::time::timeout(
            self.health_timeout,
            self.tx_engine.provider.inner.get_block_number(),
        )
        .await
        {
            Err(_) => Err(anyhow::anyhow!(
                "Gateway RPC health check timed out after {:?}",
                self.health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!("Gateway RPC health check failed: {}", e)),
            Ok(Ok(_)) => Ok(()),
        }
    }
}

// TODO: add check with non-funded wallet
