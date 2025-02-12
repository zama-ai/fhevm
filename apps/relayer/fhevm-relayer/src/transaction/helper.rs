use crate::errors::{EventProcessingError, TransactionServiceError};
use crate::transaction::{TransactionService, TxConfig};
use alloy::primitives::{Address, Bytes};
use alloy::rpc::types::TransactionReceipt;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

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
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self
                .try_send_transaction(operation_name, target, &prepare_calldata)
                .await
            {
                Ok(receipt) => {
                    return receipt_processor.process(&receipt);
                }
                Err(e) => {
                    if attempt < MAX_RETRIES - 1 {
                        error!(
                            ?e,
                            attempt,
                            operation = operation_name,
                            "Transaction failed, retrying..."
                        );
                        tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Max retries exceeded".to_string(),
        ))
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
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self
                .try_send_transaction(operation_name, target, &prepare_calldata)
                .await
            {
                Ok(_) => {
                    info!(
                        operation = operation_name,
                        "Transaction confirmed successfully"
                    );
                    return Ok(());
                }
                Err(e) => {
                    if attempt < MAX_RETRIES - 1 {
                        error!(
                            ?e,
                            attempt,
                            operation = operation_name,
                            "Transaction failed, retrying..."
                        );
                        tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Max retries exceeded".to_string(),
        ))
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

        let receipt = self
            .tx_service
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(EventProcessingError::from)?
            .ok_or_else(|| {
                TransactionServiceError::Failed("Transaction receipt not found".into())
            })?;

        if !receipt.status() {
            return Err(TransactionServiceError::Failed("Transaction reverted".into()).into());
        }

        Ok(receipt)
    }
}
