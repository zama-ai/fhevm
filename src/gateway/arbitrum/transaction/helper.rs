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

#[async_trait::async_trait]
pub trait TxLifecycleHooks: Send + Sync {
    async fn on_tx_in_flight(&self, job_id: &JobId) -> Result<(), EventProcessingError>;

    async fn on_receipt_received(
        &self,
        job_id: &JobId,
        receipt: &TxResult,
    ) -> Result<(), EventProcessingError>;

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

        // updating tx with tx_in_flight status.
        hook.on_tx_in_flight(&job_id).await?;

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

        hook.on_receipt_received(&job_id, &receipt).await?;
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
