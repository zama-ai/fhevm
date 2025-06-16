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

        let call_builder = contract.publicDecryptionResponse(id, result, signature.into());
        info!(decryption_id = ?id, "public decryption calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 Public Decryption response sent with tx receipt: {:?}", receipt);
        info!(decryption_id = ?id, "⛽ Gas consumed for Public Decryption: {}", receipt.gas_used);
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
        let call_builder = contract.userDecryptionResponse(id, result, signature.into());
        info!(decryption_id = ?id, "user decryption calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(decryption_id = ?id, "🎯 User Decryption response sent with tx receipt: {:?}", receipt);
        info!(decryption_id = ?id, "⛽ Gas consumed for User Decryption: {}", receipt.gas_used);
        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, id: U256, call: &mut TransactionRequest) {
        match self.provider.estimate_gas(call.clone()).await {
            Ok(gas) => info!(decryption_id = ?id, "Initial gas estimation for the tx: {gas}"),
            Err(e) => warn!(decryption_id = ?id, "Failed to estimate gas for the tx: {e}"),
        }

        // TODO: temporary workaround for out-of-gas errors
        // Our automatic estimation fails during gas pikes.
        // (see https://zama-ai.slack.com/archives/C0915Q59CKG/p1749843623276629?thread_ts=1749828466.079719&cid=C0915Q59CKG)
        info!(decryption_id = ?id, "Updating `gas_limit` to max value");
        call.gas = Some(INFINITE_GAS_LIMIT);
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
