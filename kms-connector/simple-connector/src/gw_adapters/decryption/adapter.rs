use crate::core::backpressure::BackpressureSignal;
use crate::core::config::Config;
use crate::core::utils::nonce_manager::SequentialNonceManager;
use crate::error::{Error, Result};
use alloy::{
    primitives::{Address, Bytes, TxHash, U256},
    providers::Provider,
    rpc::types::{TransactionReceipt, TransactionRequest},
};
use fhevm_gateway_rust_bindings::decryption::Decryption;
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

/// Adapter for decryption operations with sequential nonce management
#[derive(Clone)]
pub struct DecryptionAdapter<P> {
    decryption_address: Address,
    provider: Arc<P>,
    nonce_manager: Arc<SequentialNonceManager<P>>,
}

impl<P: Provider + Clone + Send + Sync + 'static> DecryptionAdapter<P> {
    /// Create a new decryption adapter with sequential nonce management and backpressure signaling
    pub fn new(
        decryption_address: Address,
        provider: Arc<P>,
        config: &Config,
    ) -> (Self, broadcast::Receiver<BackpressureSignal>) {
        // Create backpressure channel for queue monitoring using configured channel size
        let (backpressure_tx, backpressure_rx) = broadcast::channel(config.channel_size);

        // Use backpressure-enabled nonce manager with config
        let nonce_manager = Arc::new(SequentialNonceManager::new_with_backpressure(
            provider.clone(),
            backpressure_tx,
            Arc::new(config.clone()),
        ));

        info!(
            "DecryptionAdapter initialized with backpressure-enabled nonce manager (channel_size: {})",
            config.channel_size
        );

        let adapter = Self {
            decryption_address,
            provider: provider.clone(),
            nonce_manager,
        };

        (adapter, backpressure_rx)
    }

    /// Get the provider
    pub fn provider(&self) -> &Arc<P> {
        &self.provider
    }

    /// Send a public decryption response
    pub async fn send_public_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> Result<()> {
        if signature.len() != 65 {
            return Err(Error::Contract(format!(
                "PublicDecryptionResponse-{id}: Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        debug!(
            signature = ?signature,
            "Using Core's EIP-712 signature for PublicDecryptionResponse-{id}"
        );

        // Build and send transaction (non-blocking with spawned task)
        let contract = Decryption::new(self.decryption_address, self.provider.clone());
        let call_builder =
            contract.publicDecryptionResponse(id, result.clone(), signature.clone().into());
        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let original_gas_limit = call.gas;

        // Spawn non-blocking transaction sending and retry logic
        let nonce_manager = self.nonce_manager.clone();
        let provider = self.provider.clone();
        let decryption_address = self.decryption_address;
        tokio::spawn(async move {
            // Send initial transaction (no upfront cloning)
            match nonce_manager.send_transaction_queued(call).await {
                Ok(tx_hash) => {
                    info!(
                        "[TRX SENT] PublicDecryptionResponse-{id} sent with hash: {}",
                        tx_hash
                    );

                    // Analyze receipt and handle retries
                    match analyze_receipt_with_retry_logic(
                        provider.clone(),
                        tx_hash,
                        id,
                        original_gas_limit,
                    )
                    .await
                    {
                        ReceiptAnalysisResult::Success(_receipt) => {
                            info!(
                                "[TRX SUCCESS] PublicDecryptionResponse-{id} confirmed: {}",
                                tx_hash
                            );
                        }
                        ReceiptAnalysisResult::OutOfGas {
                            gas_used,
                            gas_limit,
                            ..
                        } => {
                            warn!(
                                "[OUT OF GAS] PublicDecryptionResponse-{id} failed: gas_used={} gas_limit={} ({}%) - retrying with 25% gas bump",
                                gas_used,
                                gas_limit,
                                (gas_used as f64 / gas_limit as f64) * 100.0
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.publicDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 25)
                                .await
                            {
                                error!(
                                    "Failed to retry PublicDecryptionResponse-{id} after out-of-gas: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::InsufficientGasPrice { reason, .. } => {
                            warn!(
                                "PublicDecryptionResponse-{id} insufficient gas price: {} - retrying with 15% gas bump",
                                reason
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.publicDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 15)
                                .await
                            {
                                error!(
                                    "Failed to retry PublicDecryptionResponse-{id} after insufficient gas price: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::ReceiptTimeout => {
                            warn!(
                                "PublicDecryptionResponse-{id} receipt timeout - retrying with 10% gas bump"
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.publicDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 10)
                                .await
                            {
                                error!(
                                    "Failed to retry PublicDecryptionResponse-{id} after timeout: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::NonRetryableFailure { reason, .. } => {
                            warn!(
                                "PublicDecryptionResponse-{id} non-retryable failure: {}",
                                reason
                            );
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to send PublicDecryptionResponse-{id}: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Send a user decryption response
    pub async fn send_user_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> Result<()> {
        if signature.len() != 65 {
            return Err(Error::Contract(format!(
                "UserDecryptionResponse-{id}: Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        debug!(
            signature = ?signature,
            "Using Core's EIP-712 signature for UserDecryptionResponse-{id}"
        );

        // Build and send transaction (non-blocking with spawned task)
        let contract = Decryption::new(self.decryption_address, self.provider.clone());
        let call_builder =
            contract.userDecryptionResponse(id, result.clone(), signature.clone().into());
        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let original_gas_limit = call.gas;

        // Spawn non-blocking transaction sending and retry logic
        let nonce_manager = self.nonce_manager.clone();
        let provider = self.provider.clone();
        let decryption_address = self.decryption_address;
        tokio::spawn(async move {
            // Send initial transaction (no upfront cloning)
            match nonce_manager.send_transaction_queued(call).await {
                Ok(tx_hash) => {
                    info!(
                        "[TRX SENT] UserDecryptionResponse-{id} sent with hash: {}",
                        tx_hash
                    );

                    // Analyze receipt and handle retries
                    match analyze_receipt_with_retry_logic(
                        provider.clone(),
                        tx_hash,
                        id,
                        original_gas_limit,
                    )
                    .await
                    {
                        ReceiptAnalysisResult::Success(_receipt) => {
                            info!(
                                "[TRX SUCCESS] UserDecryptionResponse-{id} confirmed: {}",
                                tx_hash
                            );
                        }
                        ReceiptAnalysisResult::OutOfGas {
                            gas_used,
                            gas_limit,
                            ..
                        } => {
                            warn!(
                                "[OUT OF GAS] UserDecryptionResponse-{id} failed: gas_used={} gas_limit={} ({}%) - retrying with 25% gas bump",
                                gas_used,
                                gas_limit,
                                (gas_used as f64 / gas_limit as f64) * 100.0
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.userDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 25)
                                .await
                            {
                                error!(
                                    "Failed to retry UserDecryptionResponse-{id} after out-of-gas: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::InsufficientGasPrice { reason, .. } => {
                            warn!(
                                "UserDecryptionResponse-{id} insufficient gas price: {} - retrying with 15% gas bump",
                                reason
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.userDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 15)
                                .await
                            {
                                error!(
                                    "Failed to retry UserDecryptionResponse-{id} after insufficient gas price: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::ReceiptTimeout => {
                            warn!(
                                "UserDecryptionResponse-{id} receipt timeout - retrying with 10% gas bump"
                            );
                            // Reconstruct transaction only when retry is needed
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call_builder = contract.userDecryptionResponse(
                                id,
                                result.clone(),
                                signature.clone().into(),
                            );
                            let mut retry_call = retry_call_builder.into_transaction_request();
                            retry_call.gas = original_gas_limit; // Use original gas limit as base

                            if let Err(e) = nonce_manager
                                .retry_transaction_with_gas_bump(retry_call, 10)
                                .await
                            {
                                error!(
                                    "Failed to retry UserDecryptionResponse-{id} after timeout: {}",
                                    e
                                );
                            }
                        }
                        ReceiptAnalysisResult::NonRetryableFailure { reason, .. } => {
                            warn!(
                                "UserDecryptionResponse-{id} non-retryable failure: {}",
                                reason
                            );
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to send UserDecryptionResponse-{id}: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, id: U256, call: &mut TransactionRequest) {
        const GAS_ESTIMATION_BUFFER: f64 = 1.2; // 20% buffer for safety

        let gas_estimation = match self.provider.estimate_gas(call.clone()).await {
            Ok(estimation) => estimation,
            Err(e) => return warn!(decryption_id = ?id, "Failed to estimate gas for the tx: {e}"),
        };

        // Add 20% buffer to gas estimation to prevent out-of-gas from estimation inaccuracies
        let gas_with_buffer = (gas_estimation as f64 * GAS_ESTIMATION_BUFFER) as u64;

        info!(
            decryption_id = ?id,
            "Gas estimation: {} → {} (with {}% buffer)",
            gas_estimation, gas_with_buffer,
            ((GAS_ESTIMATION_BUFFER - 1.0) * 100.0) as u32
        );

        call.gas = Some(gas_with_buffer);
    }
}

/// Result of receipt analysis for retry decision making
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ReceiptAnalysisResult {
    /// Transaction succeeded
    Success(TransactionReceipt),
    /// Transaction failed due to out of gas
    OutOfGas {
        receipt: TransactionReceipt,
        gas_used: u64,
        gas_limit: u64,
    },
    /// Transaction failed due to insufficient gas price (stuck in mempool)
    InsufficientGasPrice {
        receipt: Option<TransactionReceipt>,
        reason: String,
    },
    /// Transaction failed for non-retryable reasons
    NonRetryableFailure {
        receipt: TransactionReceipt,
        reason: String,
    },
    /// Receipt polling timed out
    ReceiptTimeout,
}

/// Enhanced receipt analysis with retry decision logic
async fn analyze_receipt_with_retry_logic<P: Provider + Clone + Send + Sync + 'static>(
    provider: P,
    tx_hash: TxHash,
    id: U256,
    original_gas_limit: Option<u64>,
) -> ReceiptAnalysisResult {
    const MAX_RETRIES: u32 = 15; // Increased for better receipt detection
    const RETRY_DELAY: Duration = Duration::from_millis(400);
    const OUT_OF_GAS_THRESHOLD: f64 = 0.98; // 98% gas usage indicates likely out-of-gas

    for attempt in 1..=MAX_RETRIES {
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => {
                info!(
                    "Transaction receipt found for {}: {} (attempt {})",
                    id, tx_hash, attempt
                );

                // Check if transaction succeeded
                if receipt.status() {
                    return ReceiptAnalysisResult::Success(receipt);
                }

                // Transaction failed - analyze the failure reason
                let gas_used = Into::<u64>::into(receipt.gas_used);

                // Use the original gas limit if provided for out-of-gas detection
                if let Some(gas_limit) = original_gas_limit {
                    // Check for out-of-gas condition using the provided gas limit
                    if gas_used as f64 >= (gas_limit as f64 * OUT_OF_GAS_THRESHOLD) {
                        warn!(
                            "Out-of-gas detected for {}: used {} / {} gas ({:.1}%)",
                            id,
                            gas_used,
                            gas_limit,
                            (gas_used as f64 / gas_limit as f64) * 100.0
                        );
                        return ReceiptAnalysisResult::OutOfGas {
                            receipt,
                            gas_used,
                            gas_limit,
                        };
                    }
                } else {
                    // Without the original gas limit, we can't reliably detect out-of-gas
                    // This should not happen in our current implementation since we always pass gas_limit
                    debug!(
                        "Cannot determine out-of-gas for {} (gas used: {}) - original gas limit not provided",
                        id, gas_used
                    );
                }

                // Other execution failure (or indeterminate)
                return ReceiptAnalysisResult::NonRetryableFailure {
                    receipt,
                    reason: "Transaction execution failed".to_string(),
                };
            }
            Ok(None) => {
                if attempt < MAX_RETRIES {
                    debug!(
                        "Transaction receipt not yet available for {}: {} (attempt {}), retrying...",
                        id, tx_hash, attempt
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                    continue;
                } else {
                    warn!(
                        "Transaction receipt timeout for {}: {} after {} attempts - may be stuck in mempool",
                        id, tx_hash, MAX_RETRIES
                    );
                    return ReceiptAnalysisResult::ReceiptTimeout;
                }
            }
            Err(e) => {
                if attempt < MAX_RETRIES {
                    debug!(
                        "Error fetching receipt for {}: {} (attempt {}): {}, retrying...",
                        id, tx_hash, attempt, e
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                    continue;
                } else {
                    let error_msg = e.to_string().to_lowercase();

                    // Classify the error based on the actual error message
                    if error_msg.contains("underpriced") || error_msg.contains("gas") {
                        warn!(
                            "Transaction {} appears to have insufficient gas price: {}",
                            tx_hash, e
                        );
                        return ReceiptAnalysisResult::InsufficientGasPrice {
                            receipt: None,
                            reason: format!("Insufficient gas price: {e}"),
                        };
                    } else {
                        warn!(
                            "Failed to get transaction receipt after {} attempts for {}: {}: {}",
                            MAX_RETRIES, id, tx_hash, e
                        );
                        return ReceiptAnalysisResult::ReceiptTimeout;
                    }
                }
            }
        }
    }

    unreachable!()
}

/// Legacy function for backward compatibility
#[allow(dead_code)]
async fn wait_for_receipt_background<P: Provider + Clone + Send + Sync + 'static>(
    provider: P,
    tx_hash: TxHash,
    id: U256,
) -> Result<TransactionReceipt> {
    match analyze_receipt_with_retry_logic(provider, tx_hash, id, None).await {
        ReceiptAnalysisResult::Success(receipt) => Ok(receipt),
        ReceiptAnalysisResult::OutOfGas { receipt, .. } => Ok(receipt),
        ReceiptAnalysisResult::NonRetryableFailure { receipt, .. } => Ok(receipt),
        ReceiptAnalysisResult::InsufficientGasPrice { reason, .. } => {
            Err(Error::Contract(format!("Transaction failed: {reason}")))
        }
        ReceiptAnalysisResult::ReceiptTimeout => Err(Error::Contract(format!(
            "Transaction receipt timeout for {id}: {tx_hash}"
        ))),
    }
}
