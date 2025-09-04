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
    hex,
    network::Ethereum,
    providers::{PendingTransactionBuilder, Provider, ext::DebugApi},
    rpc::types::{
        TransactionReceipt, TransactionRequest,
        trace::geth::{CallConfig, GethDebugTracingOptions},
    },
};
use anyhow::anyhow;
use connector_utils::{
    conn::{WalletGatewayProvider, connect_to_db, connect_to_gateway_with_wallet},
    tasks::spawn_with_limit,
    types::{KmsResponse, PublicDecryptionResponse, UserDecryptionResponse},
};
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
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
            TransactionSenderInnerConfig {
                tx_retries: config.tx_retries,
                tx_retry_interval: config.tx_retry_interval,
                trace_reverted_tx: config.trace_reverted_tx,
                gas_multiplier_percent: config.gas_multiplier_percent,
            },
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
    provider: P,
    decryption_contract: DecryptionInstance<P>,
    config: TransactionSenderInnerConfig,
}

#[derive(Clone, Default)]
pub struct TransactionSenderInnerConfig {
    pub tx_retries: u8,
    pub tx_retry_interval: Duration,
    pub trace_reverted_tx: bool,
    pub gas_multiplier_percent: usize,
}

impl<P: Provider> TransactionSenderInner<P> {
    pub fn new(
        provider: P,
        decryption_contract: DecryptionInstance<P>,
        inner_config: TransactionSenderInnerConfig,
    ) -> Self {
        Self {
            provider,
            decryption_contract,
            config: inner_config,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn send_to_gateway(&self, response: KmsResponse) -> anyhow::Result<()> {
        info!("Sending response to the Gateway: {response:?}");
        let tx_result = match response {
            KmsResponse::PublicDecryption(response) => {
                self.send_public_decryption_response(response).await
            }
            KmsResponse::UserDecryption(response) => {
                self.send_user_decryption_response(response).await
            }
        };

        let receipt = tx_result.inspect_err(|e| {
            GATEWAY_TX_SENT_ERRORS.inc();
            error!("Failed to send response to the Gateway: {e}");
        })?;

        debug!("Transaction receipt: {:?}", receipt);
        if receipt.status() {
            GATEWAY_TX_SENT_COUNTER.inc();
            info!(
                tx_hash = hex::encode(receipt.transaction_hash),
                block_hash = receipt.block_hash.map(hex::encode),
                "Response successfully sent to the Gateway!"
            );
            Ok(())
        } else {
            GATEWAY_TX_SENT_ERRORS.inc();
            let revert_reason = self
                .get_revert_reason(&receipt)
                .await
                .unwrap_or_else(|e| e.to_string());
            error!(
                tx_hash = hex::encode(receipt.transaction_hash),
                "Failed to send response to the Gateway: {revert_reason}"
            );
            Err(anyhow!(
                "Failed to send response to the Gateway: {revert_reason}"
            ))
        }
    }

    /// Sends a PublicDecryptionResponse to the Gateway.
    pub async fn send_public_decryption_response(
        &self,
        response: PublicDecryptionResponse,
    ) -> anyhow::Result<TransactionReceipt> {
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
        tx.get_receipt().await.map_err(anyhow::Error::from)
    }

    /// Sends a UserDecryptionResponse to the Gateway.
    pub async fn send_user_decryption_response(
        &self,
        response: UserDecryptionResponse,
    ) -> anyhow::Result<TransactionReceipt> {
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
        tx.get_receipt().await.map_err(anyhow::Error::from)
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
        let new_gas =
            (current_gas as u128 * self.config.gas_multiplier_percent as u128 / 100) as u64;
        call.gas = Some(new_gas);
        info!("Initial gas estimation for the tx: {current_gas}. Increased to {new_gas}");
    }

    /// Sends the requested transaction with retries.
    ///
    /// The `gas_limit` is increased at each attempts.
    async fn send_tx_with_retry(
        &self,
        mut call: TransactionRequest,
    ) -> anyhow::Result<PendingTransactionBuilder<Ethereum>> {
        for i in 1..=self.config.tx_retries {
            self.overprovision_gas(&mut call).await;

            match self.provider.send_transaction(call.clone()).await {
                Ok(tx) => return Ok(tx),
                Err(e) => {
                    warn!(
                        "Transaction attempt #{}/{} failed: {}. Retrying in {}ms...",
                        i,
                        self.config.tx_retries,
                        e,
                        self.config.tx_retry_interval.as_millis()
                    );
                    tokio::time::sleep(self.config.tx_retry_interval).await;
                }
            }
        }
        Err(anyhow!("All transactions attempt failed"))
    }

    /// Tries to use the `debug_trace_transaction` RPC call to find the cause of a reverted tx.
    async fn get_revert_reason(&self, receipt: &TransactionReceipt) -> anyhow::Result<String> {
        if !self.config.trace_reverted_tx {
            return Err(anyhow!(
                "Reverted transaction tracing is disabled. See configuration documentation to enable it."
            ));
        }

        let trace = self
            .provider
            .debug_trace_transaction(
                receipt.transaction_hash,
                GethDebugTracingOptions::call_tracer(CallConfig::default()),
            )
            .await
            .map_err(|e| {
                anyhow!("Unable to use `debug_trace_transaction` to get revert reason: {e}")
            })?
            .try_into_call_frame()
            .map_err(|e| anyhow!("Unable to retrieve revert reason: {e}"))?;

        debug!("`debug_trace_transaction` result: {trace:?}");
        trace
            .clone()
            .revert_reason
            .or_else(|| trace.calls.iter().find_map(|c| c.error.clone()))
            .ok_or_else(|| anyhow!("Unable to find revert reason in trace: {trace:?}"))
    }
}

impl<P: Provider + Clone> Clone for TransactionSenderInner<P> {
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            decryption_contract: self.decryption_contract.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Address, TxHash},
        providers::{ProviderBuilder, mock::Asserter},
        rpc::types::trace::geth::GethTrace,
    };
    use connector_utils::tests::rand::{rand_signature, rand_u256};
    use serde::de::DeserializeOwned;
    use std::fs::File;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_send_tx_out_of_gas() -> anyhow::Result<()> {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());

        // Used to mock all RPC responses of transaction sending operation
        let test_data_dir = test_data_dir();
        let estimate_gas: usize = parse_mock(&format!("{test_data_dir}/1_estimate_gas.json"))?;
        let send_tx: TxHash = parse_mock(&format!("{test_data_dir}/2_send_tx.json"))?;
        let get_receipt: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_get_receipt.json"))?;
        let debug_trace_tx: GethTrace =
            parse_mock(&format!("{test_data_dir}/4_debug_trace_tx.json"))?;
        asserter.push_success(&estimate_gas);
        asserter.push_success(&send_tx);
        asserter.push_success(&get_receipt);
        asserter.push_success(&get_receipt); // RPC call is made twice
        asserter.push_success(&debug_trace_tx);

        // Mock out of gas tx
        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                tx_retries: 1,
                trace_reverted_tx: true,
                ..Default::default()
            },
        );
        let result = inner_sender
            .send_to_gateway(KmsResponse::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
            .await
            .unwrap_err()
            .to_string();
        assert!(result.contains("Failed to send response to the Gateway: out of gas"));
        Ok(())
    }

    #[tokio::test]
    async fn test_disable_reverted_tx_tracing() {
        let asserter = Asserter::new();
        let mock_provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());
        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                trace_reverted_tx: false,
                ..Default::default()
            },
        );

        let test_data_dir = test_data_dir();
        let tx_receipt: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_get_receipt.json")).unwrap();

        let result = inner_sender
            .get_revert_reason(&tx_receipt)
            .await
            .unwrap_err()
            .to_string();
        assert!(result.contains("Reverted transaction tracing is disabled"));
    }

    fn parse_mock<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
        Ok(serde_json::from_reader::<_, T>(File::open(path)?)?)
    }

    fn test_data_dir() -> String {
        format!("{}/tests/data/tx_out_of_gas", env!("CARGO_MANIFEST_DIR"))
    }
}
