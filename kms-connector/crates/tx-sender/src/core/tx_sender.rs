use crate::{
    core::{
        Config, DbKmsResponsePicker, DbKmsResponseRemover, KmsResponsePicker, KmsResponseRemover,
    },
    monitoring::{
        health::State,
        metrics::{GATEWAY_TX_SENT_COUNTER, GATEWAY_TX_SENT_ERRORS},
    },
};
use alloy::{
    network::Ethereum,
    providers::{PendingTransactionBuilder, Provider},
    rpc::types::TransactionRequest,
};
use anyhow::anyhow;
use connector_utils::{
    conn::{WalletGatewayProvider, connect_to_db, connect_to_gateway_with_wallet},
    tasks::spawn_with_limit,
    types::{KmsResponse, PublicDecryptionResponse, UserDecryptionResponse},
};
use fhevm_gateway_rust_bindings::decryption::Decryption::{self, DecryptionInstance};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

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
                Ok(responses) => self.spawn_response_handling_tasks(responses).await,
                Err(e) => warn!("Error while picking responses: {e}"),
            };
        }
    }

    /// Spawns a new task to handle each response.
    async fn spawn_response_handling_tasks(&self, responses: Vec<KmsResponse>) {
        for response in responses {
            let inner = self.inner.clone();
            let response_remover = self.response_remover.clone();
            spawn_with_limit(async move {
                Self::handle_response(inner, response_remover, response).await
            })
            .await;
        }
    }

    /// Handles a response coming from the  KMS Core.
    #[tracing::instrument(skip(inner, response_remover), fields(response = %response))]
    async fn handle_response(
        inner: TransactionSenderInner<P>,
        response_remover: R,
        response: KmsResponse,
    ) {
        if inner.send_to_gateway(response.clone()).await.is_err() {
            response_remover.mark_response_as_pending(response).await;
        } else if let Err(e) = response_remover.remove_response(&response).await {
            error!("Failed to remove response: {e}");
        }
    }
}

impl TransactionSender<DbKmsResponsePicker, WalletGatewayProvider, DbKmsResponseRemover> {
    /// Creates a new `TransactionSender` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<(Self, State)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker = DbKmsResponsePicker::connect(db_pool.clone(), &config).await?;
        let response_remover = DbKmsResponseRemover::new(db_pool.clone());

        let provider = connect_to_gateway_with_wallet(&config.gateway_url, config.wallet).await?;
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());
        let inner = TransactionSenderInner::new(
            provider.clone(),
            decryption_contract,
            config.tx_retries,
            config.tx_retry_interval,
            config.gas_multiplier_percent,
        );

        let state = State::new(db_pool, provider, config.healthcheck_timeout);
        let tx_sender = TransactionSender::new(response_picker, inner, response_remover);
        Ok((tx_sender, state))
    }
}

/// The expected length of an EIP712 signature.
pub const EIP712_SIGNATURE_LENGTH: usize = 65;

/// The internal struct used to send transaction to the Gateway.
pub struct TransactionSenderInner<P: Provider> {
    /// The `Provider` used to interact with the Gateway
    provider: P,

    /// The `Decryption` contract instance of the Gateway.
    decryption_contract: DecryptionInstance<P>,

    /// The number of retries to send a transaction to the Gateway.
    tx_retries: u8,

    /// The time to wait between two transactions attempt.
    tx_retry_interval: Duration,

    /// The gas multiplier percentage after each transaction attempt.
    gas_multiplier_percent: usize,
}

impl<P: Provider> TransactionSenderInner<P> {
    pub fn new(
        provider: P,
        decryption_contract: DecryptionInstance<P>,
        tx_retries: u8,
        tx_retry_interval: Duration,
        gas_multiplier_percent: usize,
    ) -> Self {
        Self {
            provider,
            decryption_contract,
            tx_retries,
            tx_retry_interval,
            gas_multiplier_percent,
        }
    }

    #[tracing::instrument(skip_all)]
    /// Sends a KMS Core's response to the Gateway.
    async fn send_to_gateway(&self, response: KmsResponse) -> anyhow::Result<()> {
        info!("Sending response to the Gateway...");
        match response {
            KmsResponse::PublicDecryption(response) => {
                self.send_public_decryption_response(response).await
            }
            KmsResponse::UserDecryption(response) => {
                self.send_user_decryption_response(response).await
            }
        }
        .inspect_err(|e| {
            GATEWAY_TX_SENT_ERRORS.inc();
            error!("Failed to send response to the Gateway: {e}");
        })
        .inspect(|_| {
            GATEWAY_TX_SENT_COUNTER.inc();
            info!("Response successfully sent to the Gateway!");
        })
    }

    /// Sends a PublicDecryptionResponse to the Gateway.
    pub async fn send_public_decryption_response(
        &self,
        response: PublicDecryptionResponse,
    ) -> anyhow::Result<()> {
        if response.signature.len() != EIP712_SIGNATURE_LENGTH {
            return Err(anyhow!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                response.signature.len()
            ));
        }

        // Create and send transaction
        info!("Sending public decryption response to the Gateway...");
        let call_builder = self.decryption_contract.publicDecryptionResponse(
            response.decryption_id,
            response.decrypted_result.into(),
            response.signature.into(),
            response.extra_data.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
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
        response: UserDecryptionResponse,
    ) -> anyhow::Result<()> {
        if response.signature.len() != EIP712_SIGNATURE_LENGTH {
            return Err(anyhow!(
                "Invalid EIP-712 signature length: {}, expected 65 bytes",
                response.signature.len()
            ));
        }

        // Create and send transaction
        info!("Sending user decryption response to the Gateway...");
        let call_builder = self.decryption_contract.userDecryptionResponse(
            response.decryption_id,
            response.user_decrypted_shares.into(),
            response.signature.into(),
            response.extra_data.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        let tx = self.send_tx_with_retry(call).await?;

        // TODO: optimize for low latency
        let receipt = tx.get_receipt().await?;
        info!("Response sent successfully!");
        debug!("Transaction receipt: {:?}", receipt);
        Ok(())
    }

    /// Increases the `gas_limit` for the upcoming transaction.
    async fn overprovision_gas(&self, call: &mut TransactionRequest) {
        let current_gas = match call.gas {
            Some(gas) => gas,
            None => match self
                .decryption_contract
                .provider()
                .estimate_gas(call.clone())
                .await
            {
                Ok(estimation) => estimation,
                Err(e) => return warn!("Failed to estimate gas for the tx: {e}"),
            },
        };
        let new_gas = (current_gas as u128 * self.gas_multiplier_percent as u128 / 100) as u64;
        call.gas = Some(new_gas);
        info!("Initial gas estimation for the tx: {current_gas}. Increased to {new_gas}");
    }

    /// Sends the requested transactions with one retry.
    async fn send_tx_with_retry(
        &self,
        mut call: TransactionRequest,
    ) -> anyhow::Result<PendingTransactionBuilder<Ethereum>> {
        for i in 1..=self.tx_retries {
            self.overprovision_gas(&mut call).await;

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
            gas_multiplier_percent: self.gas_multiplier_percent,
        }
    }
}
