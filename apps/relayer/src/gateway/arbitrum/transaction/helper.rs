use crate::gateway::arbitrum::transaction::engine::{CustomFillers, TransactionEngine};
use crate::http::HealthCheck;
use crate::{core::errors::EventProcessingError, metrics};
use alloy::network::AnyTransactionReceipt;
use alloy::network::Ethereum;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::providers::{Provider, RootProvider};
use alloy::sol_types::SolEvent;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use tracing::info;

pub type TxResult = AnyTransactionReceipt;

pub type GatewayTransactionEngine = TransactionEngine<CustomFillers, RootProvider, Ethereum>;

#[derive(Debug)]
pub struct TransactionHelper {
    tx_engine: Arc<GatewayTransactionEngine>,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionType {
    UserDecryptRequest,
    InputRequest,
    PublicDecryptRequest,
    UserDecryptResponse,
    InputResponse,
    PublicDecryptResponse,
    PublicDecryptCallback,
}

impl TransactionType {
    fn as_metrics_type(&self, chain_id: u64) -> metrics::TransactionType {
        match self {
            TransactionType::UserDecryptRequest => metrics::TransactionType::UserDecryptRequest,
            TransactionType::InputRequest => metrics::TransactionType::InputRequest,
            TransactionType::PublicDecryptRequest => metrics::TransactionType::PublicDecryptRequest,
            TransactionType::UserDecryptResponse => metrics::TransactionType::UserDecryptResponse,
            TransactionType::InputResponse => metrics::TransactionType::InputResponse,
            TransactionType::PublicDecryptResponse => {
                metrics::TransactionType::PublicDecryptResponse
            }
            TransactionType::PublicDecryptCallback => {
                metrics::TransactionType::PublicDecryptCallback(chain_id)
            }
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::InputRequest => write!(f, "input_request"),
            TransactionType::UserDecryptRequest => write!(f, "user_decrypt_request"),
            TransactionType::PublicDecryptRequest => write!(f, "public_decrypt_request"),
            TransactionType::InputResponse => write!(f, "input_response"),
            TransactionType::UserDecryptResponse => write!(f, "user_decrypt_response"),
            TransactionType::PublicDecryptResponse => write!(f, "public_decrypt_response"),
            TransactionType::PublicDecryptCallback => write!(f, "public_decrypt_callback"),
        }
    }
}

impl TransactionHelper {
    pub fn new(tx_engine: Arc<GatewayTransactionEngine>, chain_id: u64) -> Self {
        Self {
            tx_engine,
            chain_id,
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

        let tx_metric_type = transaction_type.as_metrics_type(self.chain_id);
        metrics::transaction::transaction_broadcast(tx_metric_type);

        let receipt = self
            .tx_engine
            .send_raw_transaction_sync(target, calldata, None)
            .await
            .map_err(|error| {
                metrics::transaction::transaction_failure(tx_metric_type);
                EventProcessingError::from(error)
            })?;

        metrics::transaction::transaction_confirmed(tx_metric_type);

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
                            return Err(EventProcessingError::HandlerError(format!(
                                "Failed to decode {} event: {}",
                                T::SIGNATURE,
                                e
                            )));
                        }
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(format!(
            "{} event not found in transaction logs",
            T::SIGNATURE
        )))
    }
}

#[async_trait::async_trait]
impl HealthCheck for TransactionHelper {
    async fn check(&self) -> anyhow::Result<()> {
        self.tx_engine
            .provider
            .inner
            .get_block_number()
            .await
            .map_err(|e| anyhow::anyhow!("Gateway RPC health check failed: {}", e))?;
        Ok(())
    }
}

// TODO: add check with non-funded wallet
