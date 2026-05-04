use crate::{
    core::{
        Config, DbKmsResponsePicker, EthereumTransactionSender, GatewayTransactionSender,
        KmsResponsePicker, ethereum::EthereumSenderConfig, gateway::GatewaySenderConfig,
    },
    monitoring::{health::State, metrics::register_response_forwarding_latency},
};
use alloy::{
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
    conn::{WalletProviderFillers, connect_to_db, connect_to_rpc_node_with_wallet},
    tasks::spawn_with_limit,
    types::{KmsResponse, KmsResponseKind},
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{self, DecryptionErrors},
    gateway_config::GatewayConfig::GatewayConfigErrors,
};
use fhevm_host_bindings::kms_generation::KMSGeneration::{self, KMSGenerationErrors};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Struct sending stored KMS Core's responses to the Gateway and Ethereum.
pub struct TransactionSender<L, F, P>
where
    F: TxFiller,
    P: Provider,
{
    /// The entity used to collect stored KMS Core's responses.
    response_picker: L,

    /// The entity responsible to send transactions to the Gateway.
    gw_sender: GatewayTransactionSender<F, P>,

    /// The entity responsible to send transactions to Ethereum.
    eth_sender: EthereumTransactionSender<F, P>,

    /// The database pool for where the KMS Core's responses are stored.
    db_pool: Pool<Postgres>,
}

impl<L, F, P> TransactionSender<L, F, P>
where
    L: KmsResponsePicker,
    F: TxFiller + 'static,
    P: Provider + Clone + 'static,
{
    /// Creates a new `TransactionSender` instance.
    pub fn new(
        response_picker: L,
        gw_sender: GatewayTransactionSender<F, P>,
        eth_sender: EthereumTransactionSender<F, P>,
        db_pool: Pool<Postgres>,
    ) -> Self {
        Self {
            response_picker,
            gw_sender,
            eth_sender,
            db_pool,
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

    /// Spawns a new task to forward each response to the appropriate chain.
    async fn spawn_response_forwarding_tasks(
        &self,
        responses: Vec<KmsResponse>,
        cancel_token: &CancellationToken,
    ) {
        for response in responses {
            let gw_sender = self.gw_sender.clone();
            let eth_sender = self.eth_sender.clone();
            let db_pool = self.db_pool.clone();
            let cloned_cancel_token = cancel_token.clone();
            spawn_with_limit(async move {
                Self::forward_response(
                    gw_sender,
                    eth_sender,
                    db_pool,
                    response,
                    cloned_cancel_token,
                )
                .await
            })
            .await;
        }
    }

    /// Handles a response coming from the KMS Core.
    ///
    /// Routes decryption responses to the Gateway and keygen responses to Ethereum.
    #[tracing::instrument(skip(gw_sender, eth_sender, db_pool, cancel_token), fields(response = %response.kind))]
    async fn forward_response(
        gw_sender: GatewayTransactionSender<F, P>,
        eth_sender: EthereumTransactionSender<F, P>,
        db_pool: Pool<Postgres>,
        response: KmsResponse,
        cancel_token: CancellationToken,
    ) {
        tracing::Span::current().set_parent(response.otlp_context.extract());

        let result = match &response.kind {
            KmsResponseKind::PublicDecryption(_) | KmsResponseKind::UserDecryption(_) => {
                gw_sender.send_to_gateway(response.kind.clone()).await
            }
            _ => eth_sender.send_to_ethereum(response.kind.clone()).await,
        };

        match result {
            Err(Error::Recoverable(_)) => response.mark_as_pending(&db_pool).await,
            Err(Error::Irrecoverable(_)) => response.mark_as_failed(&db_pool).await,
            Err(Error::AlloyBackendGone) => {
                response.mark_as_pending(&db_pool).await;
                cancel_token.cancel();
            }
            Ok(()) => {
                response.mark_as_completed(&db_pool).await;
                register_response_forwarding_latency(&response);
            }
        }
    }
}

impl TransactionSender<DbKmsResponsePicker, WalletProviderFillers, RootProvider> {
    /// Creates a new `TransactionSender` instance from a valid `Config`.
    pub async fn from_config(config: Config) -> anyhow::Result<(Self, State)> {
        let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
        let response_picker = DbKmsResponsePicker::connect(db_pool.clone(), &config).await?;

        // Gateway provider + Decryption contract
        let gw_sender_config = GatewaySenderConfig::from(&config);
        let gw_wallet = config.build_wallet(config.gateway_chain_id).await?;
        let gw_provider = connect_to_rpc_node_with_wallet(
            config.gateway_url.clone(),
            config.gateway_chain_id,
            gw_wallet,
        )
        .await?;
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, gw_provider.clone());
        let gw_sender = GatewayTransactionSender::new(
            gw_provider.clone(),
            decryption_contract,
            gw_sender_config,
        );

        // Ethereum provider + KMSGeneration contract
        let eth_sender_config = EthereumSenderConfig::from(&config);
        let eth_wallet = config.build_wallet(config.ethereum_chain_id).await?;
        let eth_provider = connect_to_rpc_node_with_wallet(
            config.ethereum_url.clone(),
            config.ethereum_chain_id,
            eth_wallet,
        )
        .await?;
        let kms_generation_contract =
            KMSGeneration::new(config.kms_generation_contract.address, eth_provider.clone());
        let eth_sender = EthereumTransactionSender::new(
            eth_provider.clone(),
            kms_generation_contract,
            eth_sender_config,
        );

        let state = State::new(
            db_pool.clone(),
            gw_provider,
            eth_provider,
            config.healthcheck_timeout,
        );
        let tx_sender = TransactionSender::new(response_picker, gw_sender, eth_sender, db_pool);
        Ok((tx_sender, state))
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

/// Increases the `gas_limit` for the upcoming transaction.
pub async fn overprovision_gas<P: Provider>(
    provider: &P,
    gas_multiplier_percent: usize,
    call: &mut TransactionRequest,
) -> Result<(), Error> {
    let current_gas = match call.gas {
        Some(gas) => gas,
        None => provider
            .estimate_gas(call.clone())
            .await
            .map_err(Error::from)?,
    };
    let new_gas = (current_gas as u128 * gas_multiplier_percent as u128 / 100) as u64;
    call.gas = Some(new_gas);
    info!("Initial gas estimation for the tx: {current_gas}. Increased to {new_gas}");
    Ok(())
}

/// Tries to use the `debug_trace_transaction` RPC call to find the cause of a reverted tx.
pub async fn get_revert_reason<P: Provider>(
    provider: &P,
    receipt: &TransactionReceipt,
    trace_enabled: bool,
) -> anyhow::Result<String> {
    if !trace_enabled {
        return Err(anyhow!(
            "Reverted transaction tracing is disabled. See configuration documentation to enable it."
        ));
    }

    let trace = provider
        .debug_trace_transaction(
            receipt.transaction_hash,
            GethDebugTracingOptions::call_tracer(CallConfig::default()),
        )
        .await
        .map_err(|e| anyhow!("Unable to use `debug_trace_transaction` to get revert reason: {e}"))?
        .try_into_call_frame()
        .map_err(|e| anyhow!("Unable to retrieve revert reason: {e}"))?;

    debug!("`debug_trace_transaction` result: {trace:?}");
    trace
        .clone()
        .revert_reason
        .or_else(|| trace.calls.iter().find_map(|c| c.error.clone()))
        .ok_or_else(|| anyhow!("Unable to find revert reason in trace: {trace:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::Address,
        providers::{ProviderBuilder, mock::Asserter},
        rpc::json_rpc::ErrorPayload,
        transports::{RpcError, TransportErrorKind},
    };
    use connector_utils::provider::NonceManagedProvider;
    use serde::de::DeserializeOwned;
    use serde_json::value::RawValue;
    use std::fs::File;

    #[tokio::test]
    async fn test_disable_reverted_tx_tracing() {
        let asserter = Asserter::new();
        let mock_provider = NonceManagedProvider::new(
            ProviderBuilder::new().connect_mocked_client(asserter.clone()),
            Address::default(),
        );
        let test_data_dir = test_data_dir();
        let tx_receipt: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_send_tx_sync.json")).unwrap();

        let result = get_revert_reason(&mock_provider, &tx_receipt, false)
            .await
            .unwrap_err()
            .to_string();
        assert!(result.contains("Reverted transaction tracing is disabled"));
    }

    #[test]
    fn test_decryption_error_is_irrecoverable() {
        // DecryptionNotRequested(bytes32) has selector 0xd48af942
        let error_payload = ErrorPayload {
            code: 3,
            message: "execution reverted: custom error 0xd48af942: 77cb0955e69416cf320fcdf8186e8b3951fb40b84cb7f2a356d0e8af207b0046".into(),
            data: Some(RawValue::from_string(String::from(
                "\"0xd48af94277cb0955e69416cf320fcdf8186e8b3951fb40b84cb7f2a356d0e8af207b0046\"",
            )).unwrap()),
        };
        let rpc_error: RpcError<TransportErrorKind> = RpcError::ErrorResp(error_payload);
        let error = Error::from(rpc_error);
        match error {
            Error::Irrecoverable(msg) => {
                assert!(msg.to_string().contains("DecryptionNotRequested"));
            }
            _ => panic!("Expected Irrecoverable, got {error:?}"),
        }
    }

    fn parse_mock<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
        Ok(serde_json::from_reader::<_, T>(File::open(path)?)?)
    }

    fn test_data_dir() -> String {
        format!("{}/tests/data/tx_out_of_gas", env!("CARGO_MANIFEST_DIR"))
    }
}
