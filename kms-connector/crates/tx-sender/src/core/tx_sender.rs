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
    providers::{
        PendingTransactionError, Provider, RootProvider, ext::DebugApi, fillers::TxFiller,
    },
    rpc::types::{
        TransactionReceipt, TransactionRequest,
        trace::geth::{CallConfig, GethDebugTracingOptions},
    },
    transports::{RpcError, TransportErrorKind},
};
use anyhow::anyhow;
use connector_utils::{
    conn::{WalletGatewayProviderFillers, connect_to_db, connect_to_gateway_with_wallet},
    provider::NonceManagedProvider,
    tasks::spawn_with_limit,
    types::{
        CrsgenResponse, KeygenResponse, KmsResponse, KmsResponseKind, PrepKeygenResponse,
        PublicDecryptionResponse, UserDecryptionResponse,
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{self, DecryptionErrors, DecryptionInstance},
    gateway_config::GatewayConfig::GatewayConfigErrors,
    kms_generation::KMSGeneration::{self, KMSGenerationErrors, KMSGenerationInstance},
};
use std::time::Duration;
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct sending stored KMS Core's responses to the Gateway.
pub struct TransactionSender<L, F, P, R>
where
    F: TxFiller,
    P: Provider,
{
    /// The entity used to collect stored KMS Core's responses.
    response_picker: L,

    /// The entity responsible to send transaction to the Gateway.
    inner: TransactionSenderInner<F, P>,

    /// The entity used to remove stored KMS Core's responses.
    response_remover: R,
}

impl<L, F, P, R> TransactionSender<L, F, P, R>
where
    L: KmsResponsePicker,
    F: TxFiller + 'static,
    P: Provider + Clone + 'static,
    R: KmsResponseRemover + Clone + 'static,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(
        response_picker: L,
        inner: TransactionSenderInner<F, P>,
        response_remover: R,
    ) -> Self {
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
            _ = self.run(&cancel_token) => (),
        }
        info!("TransactionSender stopped successfully!");
    }

    /// Runs the KMS Core's responses forwarding loop.
    async fn run(mut self, cancel_token: &CancellationToken) {
        loop {
            match self.response_picker.pick_responses().await {
                Ok(responses) => {
                    self.spawn_response_forwarding_tasks(responses, cancel_token)
                        .await
                }
                Err(e) => break error!("Response picker is broken: {e}"),
            };
        }
    }

    /// Spawns a new task to forward each response to the Gateway.
    async fn spawn_response_forwarding_tasks(
        &self,
        responses: Vec<KmsResponse>,
        cancel_token: &CancellationToken,
    ) {
        for response in responses {
            let inner = self.inner.clone();
            let response_remover = self.response_remover.clone();
            let cloned_cancel_token = cancel_token.clone();
            spawn_with_limit(async move {
                Self::forward_response(inner, response_remover, response, cloned_cancel_token).await
            })
            .await;
        }
    }

    /// Handles a response coming from the  KMS Core.
    #[tracing::instrument(skip(inner, response_remover, cancel_token), fields(response = %response.kind))]
    async fn forward_response(
        inner: TransactionSenderInner<F, P>,
        response_remover: R,
        response: KmsResponse,
        cancel_token: CancellationToken,
    ) {
        tracing::Span::current().set_parent(response.otlp_context.extract());
        let response = response.kind;

        match inner.send_to_gateway(response.clone()).await {
            Err(Error::Recoverable(_)) => response_remover.mark_response_as_pending(response).await,
            Err(Error::Irrecoverable(_)) | Ok(()) => {
                if let Err(e) = response_remover.remove_response(&response).await {
                    error!("Failed to remove response: {e}");
                }
            }
            Err(Error::AlloyBackendGone) => {
                response_remover.mark_response_as_pending(response).await;
                cancel_token.cancel();
            }
        }
    }
}

impl
    TransactionSender<
        DbKmsResponsePicker,
        WalletGatewayProviderFillers,
        RootProvider,
        DbKmsResponseRemover,
    >
{
    /// Creates a new `TransactionSender` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<(Self, State)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker = DbKmsResponsePicker::connect(db_pool.clone(), &config).await?;
        let response_remover = DbKmsResponseRemover::new(db_pool.clone());

        let provider =
            connect_to_gateway_with_wallet(&config.gateway_url, config.chain_id, config.wallet)
                .await?;
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, provider.clone());
        let kms_generation_contract =
            KMSGeneration::new(config.kms_generation_contract.address, provider.clone());

        let inner = TransactionSenderInner::new(
            provider.clone(),
            decryption_contract,
            kms_generation_contract,
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

/// The internal struct used to send transaction to the Gateway.
pub struct TransactionSenderInner<F, P>
where
    F: TxFiller,
    P: Provider,
{
    provider: NonceManagedProvider<F, P>,
    decryption_contract: DecryptionInstance<NonceManagedProvider<F, P>>,
    kms_generation_contract: KMSGenerationInstance<NonceManagedProvider<F, P>>,
    config: TransactionSenderInnerConfig,
}

#[derive(Clone, Default)]
pub struct TransactionSenderInnerConfig {
    pub tx_retries: u8,
    pub tx_retry_interval: Duration,
    pub trace_reverted_tx: bool,
    pub gas_multiplier_percent: usize,
}

impl<F, P> TransactionSenderInner<F, P>
where
    F: TxFiller,
    P: Provider,
{
    pub fn new(
        provider: NonceManagedProvider<F, P>,
        decryption_contract: DecryptionInstance<NonceManagedProvider<F, P>>,
        kms_generation_contract: KMSGenerationInstance<NonceManagedProvider<F, P>>,
        inner_config: TransactionSenderInnerConfig,
    ) -> Self {
        Self {
            provider,
            decryption_contract,
            kms_generation_contract,
            config: inner_config,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn send_to_gateway(&self, response: KmsResponseKind) -> Result<(), Error> {
        info!("Sending response to the Gateway: {response:?}");
        let tx_result = match response {
            KmsResponseKind::PublicDecryption(response) => {
                self.send_public_decryption_response(response).await
            }
            KmsResponseKind::UserDecryption(response) => {
                self.send_user_decryption_response(response).await
            }
            KmsResponseKind::PrepKeygen(response) => self.send_prep_keygen_response(response).await,
            KmsResponseKind::Keygen(response) => self.send_keygen_response(response).await,
            KmsResponseKind::Crsgen(response) => self.send_crsgen_response(response).await,
        };

        let receipt = tx_result.inspect_err(|e| {
            GATEWAY_TX_SENT_ERRORS.inc();
            error!("Failed to send response to the Gateway: {e}");
        })?;

        debug!("Transaction receipt: {:?}", receipt);
        GATEWAY_TX_SENT_COUNTER.inc();
        info!(
            tx_hash = hex::encode(receipt.transaction_hash),
            block_hash = receipt.block_hash.map(hex::encode),
            "Response successfully sent to the Gateway!"
        );
        Ok(())
    }

    pub async fn send_public_decryption_response(
        &self,
        response: PublicDecryptionResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self.decryption_contract.publicDecryptionResponse(
            response.decryption_id,
            response.decrypted_result.into(),
            response.signature.into(),
            response.extra_data.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_sync_with_retry(call).await
    }

    pub async fn send_user_decryption_response(
        &self,
        response: UserDecryptionResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self.decryption_contract.userDecryptionResponse(
            response.decryption_id,
            response.user_decrypted_shares.into(),
            response.signature.into(),
            response.extra_data.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_sync_with_retry(call).await
    }

    pub async fn send_prep_keygen_response(
        &self,
        response: PrepKeygenResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self
            .kms_generation_contract
            .prepKeygenResponse(response.prep_keygen_id, response.signature.into());
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_sync_with_retry(call).await
    }

    pub async fn send_keygen_response(
        &self,
        response: KeygenResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self.kms_generation_contract.keygenResponse(
            response.key_id,
            response.key_digests.into_iter().map(|k| k.into()).collect(),
            response.signature.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_sync_with_retry(call).await
    }

    pub async fn send_crsgen_response(
        &self,
        response: CrsgenResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self.kms_generation_contract.crsgenResponse(
            response.crs_id,
            response.crs_digest.into(),
            response.signature.into(),
        );
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_sync_with_retry(call).await
    }

    /// Increases the `gas_limit` for the upcoming transaction.
    async fn overprovision_gas(&self, call: &mut TransactionRequest) -> Result<(), Error> {
        let current_gas = match call.gas {
            Some(gas) => gas,
            None => self
                .kms_generation_contract
                .provider()
                .estimate_gas(call.clone())
                .await
                .map_err(Error::from)?,
        };
        let new_gas =
            (current_gas as u128 * self.config.gas_multiplier_percent as u128 / 100) as u64;
        call.gas = Some(new_gas);
        info!("Initial gas estimation for the tx: {current_gas}. Increased to {new_gas}");
        Ok(())
    }

    /// Sends the requested transaction with retries.
    ///
    /// The `gas_limit` is increased at each attempts.
    async fn send_tx_sync_with_retry(
        &self,
        call: TransactionRequest,
    ) -> Result<TransactionReceipt, Error> {
        for i in 1..=self.config.tx_retries {
            match self
                .send_tx_sync_with_increased_gas_limit(call.clone())
                .await
            {
                Err(Error::Recoverable(e)) => {
                    warn!(
                        "Transaction attempt #{}/{} failed: {}. Retrying in {}ms...",
                        i,
                        self.config.tx_retries,
                        e,
                        self.config.tx_retry_interval.as_millis()
                    );
                    if i < self.config.tx_retries {
                        tokio::time::sleep(self.config.tx_retry_interval).await;
                    }
                }
                result => return result,
            }
        }
        Err(Error::Recoverable(anyhow!(
            "All transactions attempt failed"
        )))
    }

    async fn send_tx_sync_with_increased_gas_limit(
        &self,
        mut call: TransactionRequest,
    ) -> Result<TransactionReceipt, Error> {
        // Force a fresh gas estimation on each attempt to account for state drift
        call.gas = None;
        self.overprovision_gas(&mut call).await?;

        let receipt = self.provider.send_transaction_sync(call).await?;
        if !receipt.status() {
            let revert_reason = self
                .get_revert_reason(&receipt)
                .await
                .unwrap_or_else(|e| e.to_string());

            return Err(Error::Recoverable(anyhow!(
                "{revert_reason}. Tx hash {}",
                hex::encode(receipt.transaction_hash)
            )));
        }
        Ok(receipt)
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

impl<F, P> Clone for TransactionSenderInner<F, P>
where
    F: TxFiller,
    P: Provider + Clone,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            decryption_contract: self.decryption_contract.clone(),
            kms_generation_contract: self.kms_generation_contract.clone(),
            config: self.config.clone(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered irrecoverable error: {0}")]
    Irrecoverable(anyhow::Error),
    #[error("{0}")]
    Recoverable(anyhow::Error),
    #[error("Connection to the Gateway has been lost")]
    AlloyBackendGone,
}

impl From<RpcError<TransportErrorKind>> for Error {
    fn from(value: RpcError<TransportErrorKind>) -> Self {
        if matches!(value, RpcError::Transport(TransportErrorKind::BackendGone)) {
            return Self::AlloyBackendGone;
        }
        if let Some(decryption_error) = value
            .as_error_resp()
            .and_then(|e| e.as_decoded_interface_error::<DecryptionErrors>())
        {
            return Self::Irrecoverable(anyhow!("{decryption_error:?}"));
        }
        if let Some(kms_generation_error) = value
            .as_error_resp()
            .and_then(|e| e.as_decoded_interface_error::<KMSGenerationErrors>())
        {
            return Self::Irrecoverable(anyhow!("{kms_generation_error:?}"));
        }
        if let Some(gw_config_error) = value
            .as_error_resp()
            .and_then(|e| e.as_decoded_interface_error::<GatewayConfigErrors>())
        {
            return Self::Irrecoverable(anyhow!("{gw_config_error:?}"));
        }
        Self::Recoverable(value.into())
    }
}

impl From<PendingTransactionError> for Error {
    fn from(value: PendingTransactionError) -> Self {
        Self::Recoverable(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::{Ethereum, IntoWallet, Network, TransactionBuilder},
        primitives::Address,
        providers::{
            Identity, ProviderBuilder, SendableTx,
            fillers::{FillProvider, FillerControlFlow},
            mock::Asserter,
        },
        rpc::{json_rpc::ErrorPayload, types::trace::geth::GethTrace},
        transports::TransportResult,
    };
    use connector_utils::{
        config::KmsWallet,
        tests::rand::{rand_signature, rand_u256},
    };
    use serde::de::DeserializeOwned;
    use serde_json::value::RawValue;
    use std::fs::File;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_send_tx_out_of_gas() -> anyhow::Result<()> {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            FillProvider::new(
                ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .connect_mocked_client(asserter.clone()),
                MockFiller {},
            ),
            Address::default(),
        );

        // Used to mock all RPC responses of transaction sending operation
        let test_data_dir = test_data_dir();
        let estimate_gas: usize = parse_mock(&format!("{test_data_dir}/1_estimate_gas.json"))?;
        let nonce: String = parse_mock(&format!("{test_data_dir}/2_get_nonce.json"))?;
        let send_tx_sync: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_send_tx_sync.json"))?;
        let debug_trace_tx: GethTrace =
            parse_mock(&format!("{test_data_dir}/4_debug_trace_tx.json"))?;
        asserter.push_success(&estimate_gas);
        asserter.push_success(&nonce);
        asserter.push_success(&send_tx_sync);
        asserter.push_success(&debug_trace_tx);

        // Mock out of gas tx
        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            KMSGenerationInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                tx_retries: 1,
                trace_reverted_tx: true,
                gas_multiplier_percent: 105,
                ..Default::default()
            },
        );
        inner_sender
            .send_to_gateway(KmsResponseKind::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
            .await
            .unwrap_err();
        logs_contain("out of gas");
        Ok(())
    }

    #[tokio::test]
    async fn test_disable_reverted_tx_tracing() {
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            ProviderBuilder::new().connect_mocked_client(asserter.clone()),
            Address::default(),
        );
        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            KMSGenerationInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                trace_reverted_tx: false,
                ..Default::default()
            },
        );

        let test_data_dir = test_data_dir();
        let send_tx_sync: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_send_tx_sync.json")).unwrap();

        let result = inner_sender
            .get_revert_reason(&send_tx_sync)
            .await
            .unwrap_err()
            .to_string();
        assert!(result.contains("Reverted transaction tracing is disabled"));
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_error_decryption_not_requested() -> anyhow::Result<()> {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            FillProvider::new(
                ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .connect_mocked_client(asserter.clone()),
                MockFiller {},
            ),
            Address::default(),
        );

        // Used to mock all RPC responses of transaction sending operation
        let estimate_gas: usize = 21000;
        let send_tx_failure = ErrorPayload {
            code: 3,
            message: "execution reverted: custom error 0xd48af942: 77cb0955e69416cf320fcdf8186e8b3951fb40b84cb7f2a356d0e8af207b0046".into(),
            data: Some(RawValue::from_string(String::from(
                "\"0xd48af94277cb0955e69416cf320fcdf8186e8b3951fb40b84cb7f2a356d0e8af207b0046\"",
            ))?),
        };
        asserter.push_success(&estimate_gas);
        asserter.push_failure(send_tx_failure);

        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            KMSGenerationInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                tx_retries: 1,
                ..Default::default()
            },
        );
        let error = inner_sender
            .send_to_gateway(KmsResponseKind::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
            .await
            .unwrap_err();
        match error {
            Error::Irrecoverable(error_msg) => {
                assert!(error_msg.to_string().contains("DecryptionNotRequested"));
            }
            _ => panic!("Unexpected error type"),
        }
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_error_not_kms_tx_sender() -> anyhow::Result<()> {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            FillProvider::new(
                ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .connect_mocked_client(asserter.clone()),
                MockFiller {},
            ),
            Address::default(),
        );

        // Used to mock all RPC responses of transaction sending operation
        let estimate_gas: usize = 21000;
        let send_tx_failure = ErrorPayload {
            code: 3,
            message: "execution reverted: custom error 0xaee86323: 00000000000000000000000031de9c8ac5ecd5eaceddddee531e9bad8ac9c2a5".into(),
            data: Some(RawValue::from_string(String::from(
                "\"0xaee8632300000000000000000000000031de9c8ac5ecd5eaceddddee531e9bad8ac9c2a5\"",
            ))?),
        };
        asserter.push_success(&estimate_gas);
        asserter.push_failure(send_tx_failure);

        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            KMSGenerationInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                tx_retries: 1,
                ..Default::default()
            },
        );
        let error = inner_sender
            .send_to_gateway(KmsResponseKind::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
            .await
            .unwrap_err();
        match error {
            Error::Irrecoverable(error_msg) => {
                assert!(error_msg.to_string().contains("NotKmsTxSender"));
            }
            _ => panic!("Unexpected error type"),
        }
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_error_not_kms_signer() -> anyhow::Result<()> {
        // Create a mocked `alloy::Provider`
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            FillProvider::new(
                ProviderBuilder::new()
                    .disable_recommended_fillers()
                    .connect_mocked_client(asserter.clone()),
                Identity,
            ),
            Address::default(),
        );

        // Used to mock all RPC responses of transaction sending operation
        let estimate_gas: usize = 21000;
        let send_tx_failure = ErrorPayload {
            code: 3,
            message: "execution reverted: custom error 0x2a7c6ef6: 000000000000000000000000c5c5b98cb42800738f51b48b97b8d7998cfb3d68".into(),
            data: Some(RawValue::from_string(String::from(
                "\"0x2a7c6ef6000000000000000000000000c5c5b98cb42800738f51b48b97b8d7998cfb3d68\"",
            ))?),
        };
        asserter.push_success(&estimate_gas);
        asserter.push_failure(send_tx_failure);

        let inner_sender = TransactionSenderInner::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            KMSGenerationInstance::new(Address::default(), mock_provider),
            TransactionSenderInnerConfig {
                tx_retries: 1,
                ..Default::default()
            },
        );
        let error = inner_sender
            .send_to_gateway(KmsResponseKind::UserDecryption(UserDecryptionResponse {
                decryption_id: rand_u256(),
                user_decrypted_shares: vec![],
                signature: rand_signature(),
                extra_data: vec![],
            }))
            .await
            .unwrap_err();
        match error {
            Error::Irrecoverable(error_msg) => {
                assert!(error_msg.to_string().contains("NotKmsSigner"));
            }
            _ => panic!("Unexpected error type"),
        }
        Ok(())
    }

    fn parse_mock<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
        Ok(serde_json::from_reader::<_, T>(File::open(path)?)?)
    }

    fn test_data_dir() -> String {
        format!("{}/tests/data/tx_out_of_gas", env!("CARGO_MANIFEST_DIR"))
    }

    /// A filler that mocks gas estimation and signing of the transactions
    #[derive(Clone, Debug)]
    struct MockFiller;

    impl TxFiller<Ethereum> for MockFiller {
        type Fillable = ();

        fn status(&self, tx: &<Ethereum as Network>::TransactionRequest) -> FillerControlFlow {
            if tx.from().is_none() {
                return FillerControlFlow::Ready;
            }

            match tx.complete_preferred() {
                Ok(_) => FillerControlFlow::Ready,
                Err(e) => FillerControlFlow::Missing(vec![("Wallet", e)]),
            }
        }

        fn fill_sync(&self, _tx: &mut SendableTx<Ethereum>) {}

        async fn prepare<P>(
            &self,
            _provider: &P,
            _tx: &<Ethereum as Network>::TransactionRequest,
        ) -> TransportResult<Self::Fillable>
        where
            P: Provider<Ethereum>,
        {
            Ok(())
        }

        async fn fill(
            &self,
            _fillable: Self::Fillable,
            tx: SendableTx<Ethereum>,
        ) -> TransportResult<SendableTx<Ethereum>> {
            let mut builder = match tx {
                SendableTx::Builder(builder) => builder,
                _ => return Ok(tx),
            };

            let chain_id = 54321;
            let wallet = KmsWallet::from_private_key_str(
                "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774",
                Some(chain_id),
            )
            .unwrap()
            .into_wallet();
            builder.set_gas_limit(21000);
            builder.set_max_fee_per_gas(10);
            builder.set_max_priority_fee_per_gas(10);
            builder.set_chain_id(chain_id);
            builder.set_nonce(0);
            let envelope = builder
                .build(&wallet)
                .await
                .map_err(RpcError::local_usage)?;

            Ok(SendableTx::Envelope(envelope))
        }
    }
}
