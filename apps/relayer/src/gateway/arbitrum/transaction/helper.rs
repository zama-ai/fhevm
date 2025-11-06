use crate::gateway::arbitrum::transaction::engine::{CustomFillers, TransactionEngine};
use crate::{core::errors::EventProcessingError, metrics};
use alloy::network::{AnyTransactionReceipt, Ethereum};
use alloy::primitives::{Address, Bytes};
use alloy::providers::RootProvider;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use tracing::info;

pub trait ReceiptProcessor {
    type Output;
    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError>;
}

// Default processor that just returns the receipt
pub struct DefaultProcessor;

impl ReceiptProcessor for DefaultProcessor {
    type Output = AnyTransactionReceipt;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        Ok(receipt.clone())
    }
}

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

    pub async fn send_raw_transaction_sync<F, P>(
        &self,
        transaction_type: TransactionType,
        target: Address,
        prepare_calldata: F,
        // NOTE: add error manager? -> something  to  allow for retries
        receipt_processor: &P,
    ) -> Result<P::Output, EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
        P: ReceiptProcessor,
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

        // Process receipt with provided processor
        info!(
            operation = %transaction_type,
            tx_hash = ?receipt.transaction_hash,
            block_number = ?receipt.block_number,
            gas_used = ?receipt.gas_used,
            "Transaction confirmed"
        );

        receipt_processor.process(&receipt)
    }
}

// TODO: add check with non-funded wallet
