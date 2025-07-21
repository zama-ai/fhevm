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

/// The time to wait between two transactions attempt.
const TX_INTERVAL: Duration = Duration::from_millis(100);

/// Gas price escalation multiplier for transaction retry (20x = 2000%)
const GAS_PRICE_ESCALATION_MULTIPLIER: u128 = 5;
const MAX_GAS_PRICE_CAP: u128 = 900_000_000_000_000_000; // 0.9 ETH in wei

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

        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(
            "[TRX SUCCESS] PublicDecryptionResponse-{id} sent with trx receipt: {}",
            receipt.transaction_hash
        );
        info!(
            "[GAS] consumed by PublicDecryptionResponse-{id}: {}",
            receipt.gas_used
        );
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

        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| Error::Contract(e.to_string()))?;
        info!(
            "[TRX SUCCESS] UserDecryptionResponse-{id} sent with trx hash: {}",
            receipt.transaction_hash
        );
        info!(
            "[GAS] consumed by UserDecryptionResponse-{id}: {}",
            receipt.gas_used
        );
        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, id: U256, call: &mut TransactionRequest) {
        let gas_estimation = match self.provider.estimate_gas(call.clone()).await {
            Ok(estimation) => estimation,
            Err(e) => return warn!(decryption_id = ?id, "Failed to estimate gas for the tx: {e}"),
        };
        info!(decryption_id = ?id, "Initial gas estimation for the tx: {gas_estimation}");

        // Increase estimation to 300%
        // TODO: temporary workaround for out-of-gas errors
        // Our automatic estimation fails during gas pikes.
        // (see https://zama-ai.slack.com/archives/C0915Q59CKG/p1749843623276629?thread_ts=1749828466.079719&cid=C0915Q59CKG)
        let new_gas_value = gas_estimation.saturating_mul(3);

        info!(decryption_id = ?id, "Updating `gas_limit` to {new_gas_value}");
        call.gas = Some(new_gas_value);
    }

    /// Escalate gas prices for transaction retry
    fn escalate_gas_prices(&self, call: &mut TransactionRequest) {
        // Clear nonce to force fresh fetch
        call.nonce = None;

        // Escalate gas prices with 5x multiplier, capped at 0.9 ETH
        let escalated_gas_price = (call.gas_price.unwrap_or(50_000_000_000)
            * GAS_PRICE_ESCALATION_MULTIPLIER)
            .min(MAX_GAS_PRICE_CAP);
        let escalated_max_fee = (call.max_fee_per_gas.unwrap_or(100_000_000_000)
            * GAS_PRICE_ESCALATION_MULTIPLIER)
            .min(MAX_GAS_PRICE_CAP);
        let escalated_priority_fee = (call.max_priority_fee_per_gas.unwrap_or(10_000_000_000)
            * GAS_PRICE_ESCALATION_MULTIPLIER)
            .min(MAX_GAS_PRICE_CAP);

        call.gas_price = Some(escalated_gas_price);
        call.max_fee_per_gas = Some(escalated_max_fee);
        call.max_priority_fee_per_gas = Some(escalated_priority_fee);

        warn!(
            "Escalated gas prices (5x multiplier, 0.9 ETH cap): gas={} gwei, max_fee={} gwei, priority={} gwei",
            call.gas_price.unwrap() / 1_000_000_000, // Safe to unwrap as we set fallbacks
            call.max_fee_per_gas.unwrap() / 1_000_000_000, // Safe to unwrap as we set fallbacks
            call.max_priority_fee_per_gas.unwrap() / 1_000_000_000
        ); // Safe to unwrap as we set fallbacks
    }

    /// Generic transaction retry with escalation
    async fn send_tx_with_retry(
        &self,
        mut call: TransactionRequest,
    ) -> Result<PendingTransactionBuilder<Ethereum>> {
        // First attempt
        match self.provider.send_transaction(call.clone()).await {
            Ok(tx) => return Ok(tx),
            Err(e) => {
                warn!(
                    "Transaction failed, retrying with escalated gas prices: {}",
                    e
                );
                tokio::time::sleep(TX_INTERVAL).await;
            }
        }

        // Retry with escalated gas prices
        self.escalate_gas_prices(&mut call);
        self.provider
            .send_transaction(call)
            .await
            .map_err(|e| Error::Contract(e.to_string()))
    }
}
