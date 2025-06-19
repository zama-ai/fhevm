use crate::core::Config;
use alloy::{
    primitives::{Bytes, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::conn::{GatewayProvider, connect_to_gateway};
use fhevm_gateway_rust_bindings::decryption::Decryption::{self, DecryptionInstance};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

/// TODO.
pub struct TransactionSender<P: Provider> {
    decryption_contract: DecryptionInstance<(), P>,
}

impl<P> TransactionSender<P>
where
    P: Provider,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(decryption_contract: DecryptionInstance<(), P>) -> Self {
        Self {
            decryption_contract,
        }
    }

    /// Starts the `TransactionSender`.
    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting TransactionSender");
        // let tx_sender = Arc::new(self);
        tokio::select! {
            _ = cancel_token.cancelled() => info!("TransactionSender cancelled..."),
            _ = self.run() => (),
        }
        info!("TransactionSender stopped successfully!");
    }

    /// Runs the KMS Core's responses processing loop.
    async fn run(self) {
        loop {
            todo!()
        }
    }

    // Code imported from `simple-connector` codebase -> remove this comment once used
    /// Sends a PublicDecryptionResponse to the Gateway.
    pub async fn send_public_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> anyhow::Result<()> {
        if signature.len() != 65 {
            return Err(anyhow!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            ));
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

        // Create and send transaction
        let call = self
            .decryption_contract
            .publicDecryptionResponse(id, result, signature.into());
        let tx = call.send().await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!(decryption_id = ?id, "🎯 Public Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }

    // Code imported from `simple-connector` codebase -> remove this comment once used
    /// Sends a UserDecryptionResponse to the Gateway.
    pub async fn send_user_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> anyhow::Result<()> {
        if signature.len() != 65 {
            return Err(anyhow!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                signature.len()
            ));
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

        // Create and send transaction
        let call = self
            .decryption_contract
            .userDecryptionResponse(id, result, signature.into());
        let tx = call.send().await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!(decryption_id = ?id, "🎯 User Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }
}

impl TransactionSender<GatewayProvider> {
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let provider = connect_to_gateway(&config.gateway_url).await?;
        let decryption_contract = Decryption::new(config.decryption_contract.address, provider);

        Ok(Self::new(decryption_contract))
    }
}
