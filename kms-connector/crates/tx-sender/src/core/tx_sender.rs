use crate::{
    core::{
        Config, DbKmsResponsePicker, DbKmsResponseRemover, KmsResponsePicker, KmsResponseRemover,
    },
    metrics::{GATEWAY_TX_SENT_COUNTER, GATEWAY_TX_SENT_ERRORS},
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
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

/// Struct sending stored KMS Core's responses to the Gateway.
pub struct TransactionSender<L, P: Provider, R> {
    /// The entity used to collect stored KMS Core's responses.
    response_picker: L,

    /// The entity responsible to send transaction to the Gateway.
    inner: TransactionSenderInner<P>,

    /// The entity used to remove stored KMS Core's responses.
    response_remover: R,
}

impl<L, P, R> TransactionSender<L, P, R>
where
    L: KmsResponsePicker,
    P: Provider + Clone + 'static,
    R: KmsResponseRemover + Clone + 'static,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(response_picker: L, inner: TransactionSenderInner<P>, response_remover: R) -> Self {
        Self {
            response_picker,
            inner,
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
            match self.response_picker.pick_responses().await {
                Ok(responses) => self.spawn_response_handling_tasks(responses),
                Err(e) => warn!("Error while picking responses: {e}"),
            };
        }
    }

    /// Spawns a new task to handle each response.
    fn spawn_response_handling_tasks(&self, responses: Vec<KmsResponse>) {
        for response in responses {
            let inner = self.inner.clone();
            let response_remover = self.response_remover.clone();
            tokio::spawn(
                async move { Self::handle_response(inner, response_remover, response).await },
            );
        }
    }

    /// Handles a response coming from the  KMS Core.
    #[tracing::instrument(skip(inner, response_remover), fields(response = %response))]
    async fn handle_response(
        inner: TransactionSenderInner<P>,
        response_remover: R,
        response: KmsResponse,
    ) -> anyhow::Result<()> {
        inner.send_to_gateway(response.clone()).await?;
        response_remover.remove_response(&response).await
    }
}

impl TransactionSender<DbKmsResponsePicker, WalletGatewayProvider, DbKmsResponseRemover> {
    /// Creates a new `TransactionSender` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<Self> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker =
            DbKmsResponsePicker::connect(db_pool.clone(), config.responses_batch_size).await?;
        let response_remover = DbKmsResponseRemover::new(db_pool);

        let provider = connect_to_gateway_with_wallet(&config.gateway_url, config.wallet).await?;
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());

        let inner = TransactionSenderInner::new(
            provider,
            decryption_contract,
            config.tx_retries,
            config.tx_retry_interval,
        );

        Ok(Self::new(response_picker, inner, response_remover))
    }
}

/// The expected length of an EIP712 signature.
pub const EIP712_SIGNATURE_LENGTH: usize = 65;

/// The internal struct used to send transaction to the Gateway.
pub struct TransactionSenderInner<P: Provider> {
    /// The `Provider` used to interact with the Gateway
    provider: P,

    /// The `Decryption` contract instance of the Gateway.
    decryption_contract: DecryptionInstance<(), P>,

    /// The number of retries to send a transaction to the Gateway.
    tx_retries: u8,

    /// The time to wait between two transactions attempt.
    tx_retry_interval: Duration,
}

impl<P: Provider> TransactionSenderInner<P> {
    pub fn new(
        provider: P,
        decryption_contract: DecryptionInstance<(), P>,
        tx_retries: u8,
        tx_retry_interval: Duration,
    ) -> Self {
        Self {
            provider,
            decryption_contract,
            tx_retries,
            tx_retry_interval,
        }
    }

    #[tracing::instrument(skip_all)]
    /// Sends a KMS Core's response to the Gateway.
    async fn send_to_gateway(&self, response: KmsResponse) -> anyhow::Result<()> {
        info!("Sending response to the Gateway...");
        match response {
            KmsResponse::PublicDecryption {
                decryption_id: id,
                decrypted_result,
                signature,
            } => {
                self.send_public_decryption_response(id, decrypted_result.into(), signature)
                    .await
            }
            KmsResponse::UserDecryption {
                decryption_id: id,
                user_decrypted_shares,
                signature,
            } => {
                self.send_user_decryption_response(id, user_decrypted_shares.into(), signature)
                    .await
            }
        }
        .inspect_err(|_| GATEWAY_TX_SENT_ERRORS.inc())
        .inspect(|_| {
            GATEWAY_TX_SENT_COUNTER.inc();
            info!("Response successfully sent to the Gateway!");
        })
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

        // Create and send transaction
        info!("Sending public decryption response to the Gateway...");
        let call_builder =
            self.decryption_contract
                .publicDecryptionResponse(id, result, signature.into());
        debug!("Calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(&mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!("Response sent successfully!");
        debug!("Transaction receipt: {:?}", receipt);
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

        // Create and send transaction
        info!("Sending user decryption response to the Gateway...");
        let call_builder =
            self.decryption_contract
                .userDecryptionResponse(id, result, signature.into());
        debug!("Calldata length {}", call_builder.calldata().len());

        let mut call = call_builder.into_transaction_request();
        self.estimate_gas(&mut call).await;
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!("Response sent successfully!");
        debug!("Transaction receipt: {:?}", receipt);
        Ok(())
    }

    /// Estimates the `gas_limit` for the upcoming transaction.
    async fn estimate_gas(&self, call: &mut TransactionRequest) {
        let gas_estimation = match self
            .decryption_contract
            .provider()
            .estimate_gas(call.clone())
            .await
        {
            Ok(estimation) => estimation,
            Err(e) => return warn!("Failed to estimate gas for the tx: {e}"),
        };
        info!("Initial gas estimation for the tx: {gas_estimation}");

        // Increase estimation to 300%
        // TODO: temporary workaround for out-of-gas errors
        // Our automatic estimation fails during gas pikes.
        // (see https://zama-ai.slack.com/archives/C0915Q59CKG/p1749843623276629?thread_ts=1749828466.079719&cid=C0915Q59CKG)
        let new_gas_value = gas_estimation.saturating_mul(3);

        info!("Updating `gas_limit` to {new_gas_value}");
        call.gas = Some(new_gas_value);
    }

    /// Sends the requested transactions with one retry.
    async fn send_tx_with_retry(
        &self,
        call: TransactionRequest,
    ) -> anyhow::Result<PendingTransactionBuilder<Ethereum>> {
        for i in 1..=self.tx_retries {
            match self.provider.send_transaction(call.clone()).await {
                Ok(tx) => return Ok(tx),
                Err(e) => {
                    warn!(
                        "Transaction attempt #{}/{} failed: {}. Retrying in {}ms...",
                        i,
                        self.tx_retries,
                        e,
                        self.tx_retry_interval.as_millis()
                    );
                    tokio::time::sleep(self.tx_retry_interval).await;
                }
            }
        }
        Err(anyhow!("All transactions attempt failed"))
    }
}

impl<P: Provider + Clone> Clone for TransactionSenderInner<P> {
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            decryption_contract: self.decryption_contract.clone(),
            tx_retries: self.tx_retries,
            tx_retry_interval: self.tx_retry_interval,
        }
    }
}
