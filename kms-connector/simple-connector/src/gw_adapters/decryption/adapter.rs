use crate::core::config::Config;
use crate::core::backpressure::BackpressureSignal;
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
use tracing::{debug, info, warn};

/// Adapter for decryption operations with sequential nonce management
#[derive(Clone)]
pub struct DecryptionAdapter<P> {
    decryption_address: Address,
    provider: Arc<P>,
    nonce_manager: Arc<SequentialNonceManager<P>>,
}

impl<P: Provider + Clone + Send + Sync + 'static> DecryptionAdapter<P> {
    /// Create a new decryption adapter with sequential nonce management and backpressure signaling
    pub fn new(decryption_address: Address, provider: Arc<P>, config: &Config) -> (Self, broadcast::Receiver<BackpressureSignal>) {
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
            provider,
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
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        debug!(
            signature = ?signature,
            "Using Core's EIP-712 signature for PublicDecryptionResponse-{id}"
        );

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        let call_builder = contract.publicDecryptionResponse(id, result, signature.into());
        debug!(
            "PublicDecryptionResponse-{id} calldata length {}",
            call_builder.calldata().len()
        );

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;

        let tx_hash = self.send_tx_with_retry(call, id).await?;

        // Log transaction sent immediately (non-blocking)
        info!(
            "[TRX SENT] PublicDecryptionResponse-{id} sent with hash: {}",
            tx_hash
        );

        // Spawn background task for receipt polling (non-blocking)
        let provider = self.provider.clone();
        tokio::spawn(async move {
            match wait_for_receipt_background(provider, tx_hash, id).await {
                Ok(receipt) => {
                    info!(
                        "[TRX SUCCESS] PublicDecryptionResponse-{id} confirmed with receipt: {}",
                        tx_hash
                    );
                    info!(
                        "[GAS] consumed by PublicDecryptionResponse-{id}: {}",
                        receipt.gas_used
                    );
                }
                Err(e) => {
                    warn!(
                        "[TRX WARNING] Failed to get receipt for PublicDecryptionResponse-{id} ({}): {}",
                        tx_hash, e
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

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        // Create and send transaction
        let call_builder = contract.userDecryptionResponse(id, result, signature.into());
        debug!(
            "UserDecryptionResponse-{id} calldata length {}",
            call_builder.calldata().len()
        );

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;

        let tx_hash = self.send_tx_with_retry(call, id).await?;

        // Log transaction sent immediately (non-blocking)
        info!(
            "[TRX SENT] UserDecryptionResponse-{id} sent with hash: {}",
            tx_hash
        );

        // Spawn background task for receipt polling (non-blocking)
        let provider = self.provider.clone();
        tokio::spawn(async move {
            match wait_for_receipt_background(provider, tx_hash, id).await {
                Ok(receipt) => {
                    info!(
                        "[TRX SUCCESS] UserDecryptionResponse-{id} confirmed with receipt: {}",
                        tx_hash
                    );
                    info!(
                        "[GAS] consumed by UserDecryptionResponse-{id}: {}",
                        receipt.gas_used
                    );
                }
                Err(e) => {
                    warn!(
                        "[TRX WARNING] Failed to get receipt for UserDecryptionResponse-{id} ({}): {}",
                        tx_hash, e
                    );
                }
            }
        });
        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, id: U256, call: &mut TransactionRequest) {
        let gas_estimation = match self.provider.estimate_gas(call.clone()).await {
            Ok(estimation) => estimation,
            Err(e) => return warn!(decryption_id = ?id, "Failed to estimate gas for the tx: {e}"),
        };
        info!(decryption_id = ?id, "Initial gas estimation for the tx: {gas_estimation}");

        // Use the estimated gas limit as-is; nonce manager handles gas price optimization
        call.gas = Some(gas_estimation);
    }
}

/// Standalone function for background receipt polling
async fn wait_for_receipt_background<P: Provider + Clone + Send + Sync + 'static>(
    provider: P,
    tx_hash: TxHash,
    id: U256,
) -> Result<TransactionReceipt> {
    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY: Duration = Duration::from_millis(500);

    for attempt in 1..=MAX_RETRIES {
        match provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => {
                info!(
                    "Transaction receipt found for {}: {} (attempt {})",
                    id, tx_hash, attempt
                );
                return Ok(receipt);
            }
            Ok(None) => {
                if attempt < MAX_RETRIES {
                    info!(
                        "Transaction receipt not yet available for {}: {} (attempt {}), retrying...",
                        id, tx_hash, attempt
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                    continue;
                } else {
                    return Err(Error::Contract(format!(
                        "Transaction receipt not found after {MAX_RETRIES} attempts for {id}: {tx_hash}"
                    )));
                }
            }
            Err(e) => {
                if attempt < MAX_RETRIES {
                    warn!(
                        "Error fetching receipt for {}: {} (attempt {}): {}, retrying...",
                        id, tx_hash, attempt, e
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                    continue;
                } else {
                    return Err(Error::Contract(format!(
                        "Failed to get transaction receipt after {MAX_RETRIES} attempts for {id}: {tx_hash}: {e}"
                    )));
                }
            }
        }
    }

    unreachable!()
}

impl<P: Provider + Clone + Send + Sync + 'static> DecryptionAdapter<P> {
    async fn send_tx_with_retry(&self, call: TransactionRequest, id: U256) -> Result<TxHash> {
        info!(decryption_id = ?id, "Using SequentialNonceManager for sequential transaction processing");

        // Gas estimation already done by caller, proceed with transaction
        // The SequentialNonceManager will:
        // 1. Queue transactions per wallet address
        // 2. Process them sequentially
        // 3. Handle nonce management and retries automatically
        // 4. Return the transaction hash when complete (decoupled process to not stall the whole logic)
        self.nonce_manager.send_transaction_queued(call).await
    }
}
