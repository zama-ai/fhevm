use crate::transaction::{TransactionService, TxConfig};
use crate::{
    core::errors::{EventProcessingError, TransactionServiceError},
    metrics,
};
use alloy::network::{AnyTransactionReceipt, ReceiptResponse};
use alloy::primitives::{Address, Bytes, B256};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
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

#[derive(Debug)]
pub struct TransactionHelper {
    tx_service: Arc<TransactionService>,
    pub tx_config: TxConfig,
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
    pub fn new(tx_service: Arc<TransactionService>, tx_config: TxConfig, chain_id: u64) -> Self {
        Self {
            tx_service,
            tx_config,
            chain_id,
        }
    }

    /// Send a transaction with receipt processing
    pub async fn send_transaction<F, P>(
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
        // Prepare the calldata
        let calldata = prepare_calldata()?;

        info!(
            operation = %transaction_type,
            calldata = %format!("0x{}...", hex::encode(&calldata[..std::cmp::min(20, calldata.len())])),
            "Preparing transaction"
        );

        let tx_metric_type = transaction_type.as_metrics_type(self.chain_id);
        metrics::transaction::transaction_broadcast(tx_metric_type);
        // Use the new submit_and_wait method that handles the full flow
        let receipt = self
            .tx_service
            .submit_and_wait(target, calldata, self.tx_config.clone())
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

    /// Send a simple transaction without waiting for confirmation
    pub async fn send_transaction_simple<F>(
        &self,
        transaction_type: TransactionType,
        target: Address,
        prepare_calldata: F,
    ) -> Result<(), EventProcessingError>
    where
        F: Fn() -> Result<Bytes, EventProcessingError>,
    {
        // TODO: we don't have metrics here
        let calldata = prepare_calldata()?;

        info!(
            operation = %transaction_type,
            calldata = %format!("0x{}...", hex::encode(&calldata[..std::cmp::min(500, calldata.len())])),
            "Submitting transaction without waiting"
        );

        let tx_metric_type = transaction_type.as_metrics_type(self.chain_id);
        metrics::transaction::transaction_broadcast(tx_metric_type);
        let tx_hash = self
            .tx_service
            .submit_transaction(target, calldata, self.tx_config.clone())
            .await
            .map_err(|error| {
                metrics::transaction::transaction_failure(tx_metric_type);
                EventProcessingError::from(error)
            })?;
        metrics::transaction::transaction_confirmed(tx_metric_type);

        info!(
            operation = %transaction_type,
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
    ) -> Result<AnyTransactionReceipt, EventProcessingError> {
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

// TODO: add check with non-funded wallet
#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, FixedBytes, U256};
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::Signer;

    #[tokio::test]
    async fn test_tx_helper() {
        let node_rpc_url = "ws://localhost:8756";
        let private_key = std::env::var("TEST_PRIVATE_KEY").unwrap_or_else(|_| {
            "34aacca926bab195601bcf5702786d35cab968159b718ae671b226de11b9afee".to_string()
        });
        let mut local_signer: PrivateKeySigner = private_key.parse().unwrap();
        local_signer.set_chain_id(Some(123456));

        let tx_service = TransactionService::new(node_rpc_url, Arc::new(local_signer.clone()))
            .await
            .unwrap();

        // let tx_config = TxConfig::default();
        // let tx_helper = TransactionHelper::new(tx_service, tx_config);
        // println!("{:?}", tx_helper);

        // Now let's do some stuff with this tx-helper
        // What we should test:
        // - catch nonce issue
        // - catch revert
        // - catch stuck tx
        // To do so we could probably create another wallet with no protections that would emit
        // failing transactions
        //

        // TODO: return tx-nonce used
        // Most generally we should return information about the tx itself
        // since lots of important stuff will come from the provider fillers
        let result = tx_service
            .submit_and_wait(
                local_signer.address(),
                Bytes::from(FixedBytes::from(U256::ZERO)),
                TxConfig::default(),
            )
            .await;

        println!(
            "Canceling transaction from {:?} resulted in {:?}",
            local_signer.address(),
            result
        );
    }
}
