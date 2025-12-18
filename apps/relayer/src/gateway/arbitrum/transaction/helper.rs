use crate::config::settings::GatewayConfig;
use crate::gateway::arbitrum::transaction::engine::{CustomFillers, TransactionEngine};
use crate::orchestrator::HealthCheck;
use crate::{core::errors::EventProcessingError, metrics};
use alloy::network::AnyTransactionReceipt;
use alloy::network::Ethereum;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::SolEvent;
use serde::{Deserialize, Serialize};
use std::fmt;
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

    pub async fn send_raw_transaction_sync<F>(
        &self,
        transaction_type: TransactionType,
        target: Address,
        prepare_calldata: F,
    ) -> Result<TxResult, EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
    {
        let calldata = prepare_calldata()?;

        info!(
            operation = %transaction_type,
            calldata = %format!("0x{}...", hex::encode(&calldata[..std::cmp::min(20, calldata.len())])),
            "Preparing transaction"
        );

        let tx_metric_type = transaction_type.as_metrics_type();
        metrics::transaction::transaction_broadcast(tx_metric_type);

        let transaction_start_time = Instant::now();
        let request = self
            .tx_engine
            .prepare_transaction(target, calldata, None)
            .await
            .map_err(|error| {
                metrics::transaction::transaction_failure(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                EventProcessingError::from(error)
            })?;

        // TODO: Update the status to tx in-flight.
        let receipt = self
            .tx_engine
            .send_raw_transaction_sync_with_retries(request)
            .await
            .map_err(|error| {
                metrics::transaction::transaction_failure(
                    tx_metric_type,
                    transaction_start_time.elapsed().as_millis() as f64,
                );
                EventProcessingError::from(error)
            })?;

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

        Ok(receipt)
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
