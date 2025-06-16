use crate::error::{Error, Result};
use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, U256},
    providers::{PendingTransactionBuilder, Provider},
    rpc::types::TransactionRequest,
};
use fhevm_gateway_rust_bindings::decryption::Decryption;
use std::{sync::Arc, time::Duration};
use tracing::{debug, info, warn};

/// The max value for the `gas_limit` of a transaction.
const INFINITE_GAS_LIMIT: u64 = u64::MAX;

/// The time to wait between two transactions attempt.
const TX_INTERVAL: Duration = Duration::from_secs(3);

/// Adapter for decryption operations
#[derive(Clone)]
pub struct DecryptionAdapter<P> {
    decryption_address: Address,
    provider: Arc<P>,
}

impl<P: Provider + Clone> DecryptionAdapter<P> {
    /// Create a new decryption adapter
    pub fn new(decryption_address: Address, provider: Arc<P>) -> Self {
        Self {
            decryption_address,
            provider,
        }
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

        info!(
            decryption_id = ?id,
            signature = ?signature,
            "Using Core's EIP-712 signature for public decryption"
        );

        debug!(
            decryption_id = ?id,
            result_len = result.len(),
            signature = ?signature,
            "Sending public decryption response"
        );

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        let mut call = contract
            .publicDecryptionResponse(id, result, signature.into())
            .into_transaction_request();
        call.gas = Some(INFINITE_GAS_LIMIT);
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 Public Decryption response sent with tx receipt: {:?}", receipt);
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
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            )));
        }

        info!(
            decryption_id = ?id,
            signature = ?signature,
            "Using Core's EIP-712 signature for user decryption"
        );

        debug!(
            decryption_id = ?id,
            result_len = result.len(),
            signature = ?signature,
            "Sending user decryption response"
        );

        let contract = Decryption::new(self.decryption_address, self.provider.clone());

        // Create and send transaction
        let mut call = contract
            .userDecryptionResponse(id, result, signature.into())
            .into_transaction_request();
        call.gas = Some(INFINITE_GAS_LIMIT);
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 User Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }

    /// Sends the requested transactions with one retry.
    async fn send_tx_with_retry(
        &self,
        call: TransactionRequest,
    ) -> Result<PendingTransactionBuilder<Ethereum>> {
        match self.provider.send_transaction(call.clone()).await {
            Ok(tx) => Ok(tx),
            Err(e) => {
                warn!(
                    "Retrying to send transaction in {}s after failure: {}",
                    TX_INTERVAL.as_secs(),
                    e
                );

                tokio::time::sleep(TX_INTERVAL).await;

                self.provider
                    .send_transaction(call)
                    .await
                    .map_err(|e| Error::Contract(e.to_string()))
            }
        }
    }
}
