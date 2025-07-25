use crate::core::backpressure::BackpressureSignal;
use crate::core::config::Config;
use crate::core::utils::nonce_manager::SequentialNonceManager;
use crate::error::{Error, Result};
use alloy::{
    primitives::{Address, Bytes, TxHash, U256},
    providers::Provider,
    rpc::types::TransactionReceipt,
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
        let call = call_builder.into_transaction_request();

        // Spawn non-blocking transaction sending and retry logic
        let nonce_manager = self.nonce_manager.clone();
        let provider = self.provider.clone();
        let decryption_address = self.decryption_address;
        let response_id = format!("PublicDecryptionResponse-{id}");

        info!(
            "[TRX INIT] {}: Starting transaction processing",
            response_id
        );

        tokio::spawn(async move {
            // Gas estimation + 30% boost handled centrally by nonce manager
            let original_gas_limit = None; // Will be set by nonce manager
            // Send initial transaction
            match nonce_manager
                .send_transaction_queued(call, Some(response_id.clone()))
                .await
            {
                Ok(tx_hash) => {
                    info!(
                        "[TRX SENT] {}: Transaction sent successfully - hash: {}",
                        response_id, tx_hash
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
                        ReceiptAnalysisResult::Success => {
                            info!(
                                "[TRX SUCCESS] {}: Transaction confirmed successfully - hash: {}",
                                response_id, tx_hash
                            );
                        }
                        ReceiptAnalysisResult::NonRetryableFailure { reason, .. } => {
                            warn!(
                                "[TRX FAILED] {}: Non-retryable failure - hash: {}, reason: {}",
                                response_id, tx_hash, reason
                            );
                        }
                        retry_result => {
                            // Handle retry with fresh gas estimation (30% boost applied automatically)
                            let retry_reason = match retry_result {
                                ReceiptAnalysisResult::OutOfGas {
                                    gas_used,
                                    gas_limit,
                                    ..
                                } => {
                                    warn!(
                                        "[TRX RETRY] {}: Out of gas - hash: {}, gas_used: {}, gas_limit: {} ({:.1}%) - preparing retry",
                                        response_id,
                                        tx_hash,
                                        gas_used,
                                        gas_limit,
                                        (gas_used as f64 / gas_limit as f64) * 100.0
                                    );
                                    "out-of-gas"
                                }
                                ReceiptAnalysisResult::ImmediateRetry { reason } => {
                                    warn!(
                                        "[TRX RETRY] {}: Immediate retry needed - hash: {}, reason: {} - preparing retry",
                                        response_id, tx_hash, reason
                                    );
                                    "immediate retry"
                                }
                                _ => return, // Non-retryable or success cases
                            };

                            // Rebuild transaction (gas estimation + 30% boost handled by nonce manager)
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call = contract
                                .publicDecryptionResponse(
                                    id,
                                    result.clone(),
                                    signature.clone().into(),
                                )
                                .into_transaction_request();
                            // Note: No gas set - nonce manager will estimate fresh + apply 30% boost

                            // Send retry (gas estimation + 30% boost applied automatically by nonce manager)
                            let retry_id = format!("{response_id}-retry");
                            info!("[TRX RETRY] {}: Submitting retry transaction", retry_id);

                            if let Err(e) = nonce_manager
                                .send_transaction_queued(retry_call, Some(retry_id.clone()))
                                .await
                            {
                                error!(
                                    "[TRX RETRY] {}: Failed to submit retry after {}: {}",
                                    retry_id, retry_reason, e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "[TRX FAILED] {}: Failed to send initial transaction: {}",
                        response_id, e
                    );
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

        // Build transaction request
        let contract = Decryption::new(self.decryption_address, self.provider.clone());
        let call_builder =
            contract.userDecryptionResponse(id, result.clone(), signature.clone().into());
        let call = call_builder.into_transaction_request();

        // Spawn non-blocking transaction sending and retry logic
        let nonce_manager = self.nonce_manager.clone();
        let provider = self.provider.clone();
        let decryption_address = self.decryption_address;
        let response_id = format!("UserDecryptionResponse-{id}");

        info!(
            "[TRX INIT] {}: Starting transaction processing",
            response_id
        );

        tokio::spawn(async move {
            // Gas estimation + 30% boost handled centrally by nonce manager
            let original_gas_limit = None; // Will be set by nonce manager
            // Send initial transaction
            match nonce_manager
                .send_transaction_queued(call, Some(response_id.clone()))
                .await
            {
                Ok(tx_hash) => {
                    info!(
                        "[TRX SENT] {}: Transaction sent successfully - hash: {}",
                        response_id, tx_hash
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
                        ReceiptAnalysisResult::Success => {
                            info!(
                                "[TRX SUCCESS] {}: Transaction confirmed successfully - hash: {}",
                                response_id, tx_hash
                            );
                        }
                        ReceiptAnalysisResult::NonRetryableFailure { reason, .. } => {
                            warn!(
                                "[TRX FAILED] {}: Non-retryable failure - hash: {}, reason: {}",
                                response_id, tx_hash, reason
                            );
                        }
                        retry_result => {
                            // Handle retry with fresh gas estimation (30% boost applied automatically)
                            let retry_reason = match retry_result {
                                ReceiptAnalysisResult::OutOfGas {
                                    gas_used,
                                    gas_limit,
                                    ..
                                } => {
                                    warn!(
                                        "[TRX RETRY] {}: Out of gas - hash: {}, gas_used: {}, gas_limit: {} ({:.1}%) - preparing retry",
                                        response_id,
                                        tx_hash,
                                        gas_used,
                                        gas_limit,
                                        (gas_used as f64 / gas_limit as f64) * 100.0
                                    );
                                    "out-of-gas"
                                }
                                ReceiptAnalysisResult::ImmediateRetry { reason } => {
                                    warn!(
                                        "[TRX RETRY] {}: Immediate retry needed - hash: {}, reason: {} - preparing retry",
                                        response_id, tx_hash, reason
                                    );
                                    "immediate retry"
                                }
                                _ => return, // Non-retryable or success cases
                            };

                            // Rebuild transaction (gas estimation + 30% boost handled by nonce manager)
                            let contract = Decryption::new(decryption_address, provider.clone());
                            let retry_call = contract
                                .userDecryptionResponse(
                                    id,
                                    result.clone(),
                                    signature.clone().into(),
                                )
                                .into_transaction_request();
                            // Note: No gas set - nonce manager will estimate fresh + apply 30% boost

                            // Send retry (gas estimation + 30% boost applied automatically by nonce manager)
                            let retry_id = format!("{response_id}-retry");
                            info!("[TRX RETRY] {}: Submitting retry transaction", retry_id);

                            if let Err(e) = nonce_manager
                                .send_transaction_queued(retry_call, Some(retry_id.clone()))
                                .await
                            {
                                error!(
                                    "[TRX RETRY] {}: Failed to submit retry after {}: {}",
                                    retry_id, retry_reason, e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "[TRX FAILED] {}: Failed to send initial transaction: {}",
                        response_id, e
                    );
                }
            }
        });

        Ok(())
    }
}

/// Result of receipt analysis for retry decision making
#[derive(Debug, Clone)]
enum ReceiptAnalysisResult {
    /// Transaction succeeded
    Success,
    /// Transaction failed due to out of gas
    OutOfGas {
        #[allow(dead_code)]
        receipt: TransactionReceipt,
        gas_used: u64,
        gas_limit: u64,
    },
    /// Transaction failed for non-retryable reasons
    NonRetryableFailure {
        #[allow(dead_code)]
        receipt: TransactionReceipt,
        reason: String,
    },
    /// Receipt polling error or timeout - should trigger immediate transaction retry
    ImmediateRetry { reason: String },
}

/// Enhanced receipt analysis with retry decision logic
async fn analyze_receipt_with_retry_logic<P: Provider + Clone + Send + Sync + 'static>(
    provider: P,
    tx_hash: TxHash,
    id: U256,
    original_gas_limit: Option<u64>,
) -> ReceiptAnalysisResult {
    const MAX_RETRIES: u32 = 15; // Increased for better receipt detection
    const INITIAL_DELAY_MS: u64 = 200; // Start with 200ms
    const MAX_DELAY_MS: u64 = 1600; // Cap at 1.6 seconds
    const OUT_OF_GAS_THRESHOLD: f64 = 0.98; // 98% gas usage indicates likely out-of-gas

    for attempt in 1..=MAX_RETRIES {
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => {
                info!(
                    "Transaction receipt found for {}: {} (attempt {})",
                    id, tx_hash, attempt
                );

                // Extract gas usage for reporting
                let gas_used = Into::<u64>::into(receipt.gas_used);
                let gas_limit = original_gas_limit.unwrap_or(0);
                let gas_percentage = if gas_limit > 0 {
                    (gas_used as f64 / gas_limit as f64) * 100.0
                } else {
                    0.0
                };

                // Check if transaction succeeded
                if receipt.status() {
                    info!(
                        "DecryptionResponse-{} SUCCESS: gas_used={} gas_limit={} ({:.1}%) tx_hash={} block={:?} tx_index={:?}",
                        id,
                        gas_used,
                        gas_limit,
                        gas_percentage,
                        tx_hash,
                        receipt.block_number,
                        receipt.transaction_index
                    );
                    return ReceiptAnalysisResult::Success;
                }

                // Transaction failed - analyze the failure reason
                // (gas_used already extracted above for reporting)

                // Use the original gas limit if provided for out-of-gas detection
                if let Some(gas_limit) = original_gas_limit {
                    // Check for out-of-gas condition using the provided gas limit
                    if gas_used as f64 >= (gas_limit as f64 * OUT_OF_GAS_THRESHOLD) {
                        warn!(
                            "DecryptionResponse-{} OUT-OF-GAS: used {} / {} gas ({:.1}%) tx_hash={} block={:?}",
                            id,
                            gas_used,
                            gas_limit,
                            (gas_used as f64 / gas_limit as f64) * 100.0,
                            receipt.transaction_hash,
                            receipt.block_number
                        );
                        return ReceiptAnalysisResult::OutOfGas {
                            receipt,
                            gas_used,
                            gas_limit,
                        };
                    }
                } else {
                    debug!(
                        "Cannot determine out-of-gas for {} (gas used: {}) - original gas limit not provided",
                        id, gas_used
                    );
                }

                // Other execution failure (or indeterminate)
                warn!(
                    "DecryptionResponse-{} FAILED: gas_used={} gas_limit={} ({:.1}%) tx_hash={} block={:?} reason=execution_failed",
                    id, gas_used, gas_limit, gas_percentage, tx_hash, receipt.block_number
                );
                return ReceiptAnalysisResult::NonRetryableFailure {
                    receipt,
                    reason: "Transaction execution failed".to_string(),
                };
            }
            Ok(None) => {
                if attempt < MAX_RETRIES {
                    // Exponential backoff: 200ms → 400ms → 800ms → 1600ms (capped)
                    let delay_ms =
                        std::cmp::min(INITIAL_DELAY_MS * (1 << (attempt - 1)), MAX_DELAY_MS);
                    debug!(
                        "Transaction receipt not yet available for {}: {} (attempt {}), retrying in {}ms...",
                        id, tx_hash, attempt, delay_ms
                    );
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    continue;
                } else {
                    warn!(
                        "Transaction receipt timeout for {}: {} after {} attempts - may be stuck in mempool",
                        id, tx_hash, MAX_RETRIES
                    );
                    return ReceiptAnalysisResult::ImmediateRetry {
                        reason: format!("Receipt timeout after {MAX_RETRIES} attempts"),
                    };
                }
            }
            Err(e) => {
                // Immediate retry strategy: Any receipt polling error triggers immediate transaction retry
                warn!(
                    "Receipt polling error for {}: {} - triggering immediate retry",
                    tx_hash, e
                );
                return ReceiptAnalysisResult::ImmediateRetry {
                    reason: format!("Receipt polling error: {e}"),
                };
            }
        }
    }

    unreachable!()
}
