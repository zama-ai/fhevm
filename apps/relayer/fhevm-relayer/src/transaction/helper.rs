use crate::core::errors::{EventProcessingError, TransactionServiceError};
use crate::transaction::{TransactionService, TxConfig};
use alloy::primitives::{Address, Bytes, B256};
use alloy::rpc::types::TransactionReceipt;
use std::sync::Arc;
use std::time::Duration;
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
        // Prepare the calldata
        let calldata = prepare_calldata()?;

        info!(
            operation = operation_name,
            calldata = %format!("0x{}...", hex::encode(&calldata[..std::cmp::min(20, calldata.len())])),
            "Preparing transaction"
        );

        // Use the new submit_and_wait method that handles the full flow
        let receipt = self
            .tx_service
            .submit_and_wait(target, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        // Process receipt with provided processor
        info!(
            operation = operation_name,
            tx_hash = ?receipt.transaction_hash,
            block_number = ?receipt.block_number,
            gas_used = ?receipt.gas_used,
            "Transaction confirmed"
        );

        receipt_processor.process(&receipt)
    }

    /// Send a simple transaction without waiting for confirmation
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

        info!(
            operation = operation_name,
            calldata = %format!("0x{}...", hex::encode(&calldata[..std::cmp::min(20, calldata.len())])),
            "Submitting transaction without waiting"
        );

        let tx_hash = self
            .tx_service
            .submit_transaction(target, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        info!(
            operation = operation_name,
            ?tx_hash,
            "Transaction submitted successfully (not waiting for confirmation)"
        );

        Ok(())
    }

    /// Get transaction status - checks if a transaction was successful
    pub async fn get_transaction_status(
        &self,
        tx_hash: B256,
    ) -> Result<bool, EventProcessingError> {
        let receipt = self
            .tx_service
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(EventProcessingError::from)?;

        Ok(receipt.status())
    }

    /// Wait for an existing transaction to be confirmed
    pub async fn wait_for_transaction(
        &self,
        tx_hash: B256,
        operation_name: &str,
    ) -> Result<TransactionReceipt, EventProcessingError> {
        let timeout = Duration::from_secs(self.tx_config.timeout_secs.unwrap_or(60));

        info!(
            operation = operation_name,
            ?tx_hash,
            timeout_secs = ?timeout.as_secs(),
            "Waiting for transaction confirmation"
        );

        let receipt = self
            .tx_service
            .wait_for_receipt(tx_hash, timeout)
            .await
            .map_err(EventProcessingError::from)?;

        if !receipt.status() {
            return Err(
                TransactionServiceError::Failed("Transaction reverted on chain".into()).into(),
            );
        }

        info!(
            operation = operation_name,
            ?tx_hash,
            block_number = ?receipt.block_number,
            gas_used = ?receipt.gas_used,
            "Transaction confirmed successfully"
        );

        Ok(receipt)
    }
}
