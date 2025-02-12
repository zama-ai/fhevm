use crate::errors::{EventProcessingError, TransactionServiceError};
use crate::transaction::{TransactionService, TxConfig};
use alloy::primitives::{Address, Bytes};
use alloy::rpc::types::TransactionReceipt;
use std::sync::Arc;

use tracing::info;

pub trait ReceiptProcessor {
    type Output;
    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError>;
}

// Default processor that just returns the receipt
pub struct DefaultProcessor;

impl ReceiptProcessor for DefaultProcessor {
    type Output = TransactionReceipt;

    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError> {
        Ok(receipt.clone())
    }
}

#[derive(Debug)]
pub struct TransactionHelper {
    tx_service: Arc<TransactionService>,
    tx_config: TxConfig,
}

impl TransactionHelper {
    pub fn new(tx_service: Arc<TransactionService>, tx_config: TxConfig) -> Self {
        Self {
            tx_service,
            tx_config,
        }
    }

    /// Send a transaction with receipt processing
    pub async fn send_transaction<F, P>(
        &self,
        operation_name: &str,
        target: Address,
        prepare_calldata: F,
        receipt_processor: &P,
    ) -> Result<P::Output, EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
        P: ReceiptProcessor,
    {
        // Single attempt using service's retry mechanism
        let receipt = self
            .try_send_transaction(operation_name, target, &prepare_calldata)
            .await?;

        // Process receipt
        receipt_processor.process(&receipt)
    }

    /// Send a simple transaction without receipt processing
    pub async fn send_transaction_simple<F>(
        &self,
        operation_name: &str,
        target: Address,
        prepare_calldata: F,
    ) -> Result<(), EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
    {
        let calldata = prepare_calldata()?;

        let tx_hash = self
            .tx_service
            .submit_transaction(target, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        // Log success but don't wait for receipt
        info!(
            operation = operation_name,
            ?tx_hash,
            "Transaction submitted"
        );

        Ok(())
    }

    async fn try_send_transaction<F>(
        &self,
        operation_name: &str,
        target: Address,
        prepare_calldata: &F,
    ) -> Result<TransactionReceipt, EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
    {
        let calldata = prepare_calldata()?;

        info!(
            operation = operation_name,
            calldata = %format!("0x{}...", hex::encode(&calldata[..20])),
            "Submitting transaction"
        );

        // Submit transaction using service
        let tx_hash = self
            .tx_service
            .submit_transaction(target, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        info!(
            ?tx_hash,
            operation = operation_name,
            "Transaction submitted, waiting for confirmation"
        );

        // Wait for receipt
        let receipt = self
            .tx_service
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(EventProcessingError::from)?
            .ok_or_else(|| {
                TransactionServiceError::Failed("Transaction receipt not found".into())
            })?;

        // Check transaction status
        if !receipt.status() {
            return Err(TransactionServiceError::Failed("Transaction reverted".into()).into());
        }

        Ok(receipt)
    }
}
