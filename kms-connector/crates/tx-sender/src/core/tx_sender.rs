use crate::core::{
    Config, DbKmsResponsePicker, DbKmsResponseRemover, KmsResponsePicker, KmsResponseRemover,
};
use alloy::{
    primitives::{Bytes, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::{
    conn::{WalletGatewayProvider, connect_to_db, connect_to_gateway_with_wallet},
    types::KmsResponse,
};
use fhevm_gateway_rust_bindings::decryption::Decryption::{self, DecryptionInstance};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

/// TODO.
pub struct TransactionSender<L, P: Provider, R> {
    response_picker: L,
    decryption_contract: DecryptionInstance<(), P>,
    response_remover: R,
}

impl<L, P, R> TransactionSender<L, P, R>
where
    L: KmsResponsePicker,
    P: Provider,
    R: KmsResponseRemover,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(
        response_picker: L,
        decryption_contract: DecryptionInstance<(), P>,
        response_remover: R,
    ) -> Self {
        Self {
            response_picker,
            decryption_contract,
            response_remover,
        }
    }

    /// Starts the `TransactionSender`.
    pub async fn start(self, cancel_token: CancellationToken) {
        info!("Starting TransactionSender");
        tokio::select! {
            _ = cancel_token.cancelled() => info!("TransactionSender cancelled..."),
            _ = self.run() => (),
        }
        info!("TransactionSender stopped successfully!");
    }

    /// Runs the KMS Core's responses processing loop.
    async fn run(mut self) {
        loop {
            let response = match self.response_picker.pick_response().await {
                Ok(response) => response,
                Err(e) => {
                    error!("Error while picking response: {e}");
                    continue;
                }
            };

            let response_identifier = response.to_string();
            if let Err(e) = self.process(response).await {
                error!("Error while processing {response_identifier}: {e}");
            }
        }
    }

    /// Processes a KMS Core response.
    async fn process(&self, response: KmsResponse) -> anyhow::Result<()> {
        match response.clone() {
            KmsResponse::PublicDecryption {
                decryption_id: id,
                decrypted_result,
                signature,
            } => {
                self.send_public_decryption_response(id, decrypted_result.into(), signature)
                    .await?;
            }
            KmsResponse::UserDecryption {
                decryption_id: id,
                user_decrypted_shares,
                signature,
            } => {
                self.send_user_decryption_response(id, user_decrypted_shares.into(), signature)
                    .await?;
            }
        }
        self.response_remover.remove_response(&response).await
    }

    /// Sends a PublicDecryptionResponse to the Gateway.
    pub async fn send_public_decryption_response(
        &self,
        id: U256,
        result: Bytes,
        signature: Vec<u8>,
    ) -> anyhow::Result<()> {
        if signature.len() != EIP712_SIGNATURE_LENGTH {
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
        if signature.len() != EIP712_SIGNATURE_LENGTH {
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

pub const EIP712_SIGNATURE_LENGTH: usize = 65;

impl TransactionSender<DbKmsResponsePicker, WalletGatewayProvider, DbKmsResponseRemover> {
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker = DbKmsResponsePicker::connect(db_pool.clone()).await?;
        let response_remover = DbKmsResponseRemover::new(db_pool);

        let provider = connect_to_gateway_with_wallet(&config.gateway_url, config.wallet).await?;
        let decryption_contract = Decryption::new(config.decryption_contract.address, provider);

        Ok(Self::new(
            response_picker,
            decryption_contract,
            response_remover,
        ))
    }
}
