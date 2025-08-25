use crate::core::backpressure::BackpressureSignal;
use crate::core::config::Config;
use crate::core::utils::nonce_manager::SequentialNonceManager;
use crate::error::{Error, Result};
use alloy::{
    primitives::{Address, Bytes, U256},
    providers::Provider,
};
use fhevm_gateway_rust_bindings::decryption::Decryption;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

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
        let response_id = format!("PublicDecryptionResponse-{id}");

        info!(
            "[TRX INIT] {}: Starting transaction processing",
            response_id
        );

        tokio::spawn(async move {
            // Gas estimation + 30% boost handled centrally by nonce manager
            let _original_gas_limit: Option<u64> = None; // No longer needed with decoupled processing
            // Send initial transaction
            match nonce_manager
                .send_transaction_queued_decoupled(call, Some(response_id.clone()))
                .await
            {
                Ok(()) => {
                    info!(
                        "[TRX QUEUED DECOUPLED] {}: Transaction queued successfully - processing in background",
                        response_id
                    );

                    // Transaction processing is now fully decoupled - nonce manager handles
                    // receipt analysis, retries, gas bumping, and all transaction lifecycle
                    // management internally with tokio::spawn for throughput improvements
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
        let response_id = format!("UserDecryptionResponse-{id}");

        info!(
            "[TRX INIT] {}: Starting transaction processing",
            response_id
        );

        tokio::spawn(async move {
            // Send initial transaction
            match nonce_manager
                .send_transaction_queued_decoupled(call, Some(response_id.clone()))
                .await
            {
                Ok(()) => {
                    info!(
                        "[TRX QUEUED DECOUPLED] {}: Transaction queued successfully - processing in background",
                        response_id
                    );

                    // Transaction processing is now fully decoupled - nonce manager handles
                    // receipt analysis, retries, gas bumping, and all transaction lifecycle
                    // management internally with tokio::spawn for throughput improvements
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
