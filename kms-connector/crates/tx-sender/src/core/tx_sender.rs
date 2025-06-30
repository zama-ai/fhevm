use std::time::Duration;

use crate::core::{
    Config, DbKmsResponsePicker, DbKmsResponseRemover, KmsResponsePicker, KmsResponseRemover,
};
use alloy::{
    network::Ethereum,
    primitives::{Bytes, U256},
    providers::{PendingTransactionBuilder, Provider},
    rpc::types::TransactionRequest,
};
use anyhow::anyhow;
use connector_utils::{
    conn::{WalletGatewayProvider, connect_to_db, connect_to_gateway_with_wallet},
    types::KmsResponse,
};
use fhevm_gateway_rust_bindings::decryption::Decryption::{self, DecryptionInstance};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

/// Struct sending stored KMS Core's responses to the Gateway.
pub struct TransactionSender<L, P: Provider, R> {
    /// The entity used to collect stored KMS Core's responses.
    response_picker: L,

    /// The `Provider` used to interact with the Gateway
    provider: P,

    /// The `Decryption` contract instance of the Gateway.
    decryption_contract: DecryptionInstance<(), P>,

    /// The entity used to remove stored KMS Core's responses.
    response_remover: R,
}

/// The expected length of an EIP712 signature.
pub const EIP712_SIGNATURE_LENGTH: usize = 65;

/// The time to wait between two transactions attempt.
const TX_INTERVAL: Duration = Duration::from_secs(3);

impl<L, P, R> TransactionSender<L, P, R>
where
    L: KmsResponsePicker,
    P: Provider,
    R: KmsResponseRemover,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(
        response_picker: L,
        provider: P,
        decryption_contract: DecryptionInstance<(), P>,
        response_remover: R,
    ) -> Self {
        Self {
            response_picker,
            provider,
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
        info!("Processing {response}...");
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
        info!("{response} successfully sent to the Gateway!");

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
        let call_builder =
            self.decryption_contract
                .publicDecryptionResponse(id, result, signature.into());
        info!(decryption_id = ?id, "public decryption calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!(decryption_id = ?id, "🎯 Public Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }

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
        let call_builder =
            self.decryption_contract
                .userDecryptionResponse(id, result, signature.into());
        info!(decryption_id = ?id, "user decryption calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(id, &mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!(decryption_id = ?id, "🎯 User Decryption response sent with tx receipt: {:?}", receipt);
        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, id: U256, call: &mut TransactionRequest) {
        let gas_estimation = match self
            .decryption_contract
            .provider()
            .estimate_gas(call.clone())
            .await
        {
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

    /// Sends the requested transactions with one retry.
    async fn send_tx_with_retry(
        &self,
        call: TransactionRequest,
    ) -> anyhow::Result<PendingTransactionBuilder<Ethereum>> {
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
                    .map_err(anyhow::Error::from)
            }
        }
    }
}

impl TransactionSender<DbKmsResponsePicker, WalletGatewayProvider, DbKmsResponseRemover> {
    /// Creates a new `TransactionSender` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker = DbKmsResponsePicker::connect(db_pool.clone()).await?;
        let response_remover = DbKmsResponseRemover::new(db_pool);

        let provider = connect_to_gateway_with_wallet(&config.gateway_url, config.wallet).await?;
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());

        Ok(Self::new(
            response_picker,
            provider,
            decryption_contract,
            response_remover,
        ))
    }
}
