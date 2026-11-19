use crate::core::tx_sender::{Error, get_revert_reason, overprovision_gas};
use alloy::{
    hex,
    providers::Provider,
    rpc::types::{TransactionReceipt, TransactionRequest},
    sol_types::SolValue,
};
use anyhow::anyhow;
use connector_utils::types::{KmsResponseKind, extra_data::extra_data_v2_payload};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::KMSGenerationInstance,
    protocol_config::{
        IProtocolConfig::{EpochCrsResult, EpochKeyResult},
        ProtocolConfig::ProtocolConfigInstance,
    },
};
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// The struct used to send keygen transactions to Ethereum.
pub struct EthereumTransactionSender<P>
where
    P: Provider,
{
    provider: P,
    kms_generation_contract: KMSGenerationInstance<P>,
    protocol_config_contract: ProtocolConfigInstance<P>,
    config: EthereumSenderConfig,
}

#[derive(Clone, Default)]
pub struct EthereumSenderConfig {
    pub tx_retries: u8,
    pub tx_retry_interval: Duration,
    pub trace_reverted_tx: bool,
    pub gas_multiplier_percent: usize,
    pub tx_required_confirmations: u64,
    pub get_receipt_timeout: Duration,
}

impl From<&super::Config> for EthereumSenderConfig {
    fn from(config: &super::Config) -> Self {
        Self {
            tx_retries: config.tx_retries,
            tx_retry_interval: config.tx_retry_interval,
            trace_reverted_tx: config.trace_reverted_tx,
            gas_multiplier_percent: config.gas_multiplier_percent,
            tx_required_confirmations: config.ethereum_tx_required_confirmations,
            get_receipt_timeout: config.ethereum_tx_get_receipt_timeout,
        }
    }
}

impl<P> EthereumTransactionSender<P>
where
    P: Provider,
{
    pub fn new(
        provider: P,
        kms_generation_contract: KMSGenerationInstance<P>,
        protocol_config_contract: ProtocolConfigInstance<P>,
        inner_config: EthereumSenderConfig,
    ) -> Self {
        Self {
            provider,
            kms_generation_contract,
            protocol_config_contract,
            config: inner_config,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn send_to_ethereum(&self, response: KmsResponseKind) -> Result<(), Error> {
        info!("Sending response to Ethereum: {response:?}");
        let tx_result = match self.build_call(response) {
            Ok(call) => self.send_tx_with_retry(call).await,
            Err(e) => Err(e),
        };

        let receipt = tx_result.inspect_err(|e| {
            error!("Failed to send response to Ethereum: {e}");
        })?;

        debug!("Transaction receipt: {:?}", receipt);
        info!(
            tx_hash = hex::encode(receipt.transaction_hash),
            block_hash = receipt.block_hash.map(hex::encode),
            "Response successfully sent to Ethereum!"
        );
        Ok(())
    }

    /// Builds the transaction request matching the given response.
    fn build_call(&self, response: KmsResponseKind) -> Result<TransactionRequest, Error> {
        let call = match response {
            KmsResponseKind::PrepKeygen(r) => self
                .kms_generation_contract
                .prepKeygenResponse(r.prep_keygen_id, r.signature.into())
                .into_transaction_request(),
            KmsResponseKind::Keygen(r) => self
                .kms_generation_contract
                .keygenResponse(
                    r.key_id,
                    r.key_digests.into_iter().map(|k| k.into()).collect(),
                    r.signature.into(),
                )
                .into_transaction_request(),
            KmsResponseKind::Crsgen(r) => self
                .kms_generation_contract
                .crsgenResponse(r.crs_id, r.crs_digest.into(), r.signature.into())
                .into_transaction_request(),
            KmsResponseKind::NewKmsContext(r) => self
                .protocol_config_contract
                .confirmKmsContextCreation(r.context_id, r.signature.into(), r.extra_data.into())
                .into_transaction_request(),
            KmsResponseKind::ContextDestruction(r) => self
                .protocol_config_contract
                .confirmKmsContextDestruction(r.context_id, r.signature.into(), r.extra_data.into())
                .into_transaction_request(),
            KmsResponseKind::EpochDestruction(r) => self
                .protocol_config_contract
                .confirmKmsEpochDestruction(r.epoch_id, r.signature.into(), r.extra_data.into())
                .into_transaction_request(),
            KmsResponseKind::EpochResult(r) => {
                let keys = <Vec<EpochKeyResult> as SolValue>::abi_decode(&r.keys).map_err(|e| {
                    Error::Irrecoverable(anyhow!("Failed to decode epoch keys: {e}"))
                })?;
                let crs_list =
                    <Vec<EpochCrsResult> as SolValue>::abi_decode(&r.crs_list).map_err(|e| {
                        Error::Irrecoverable(anyhow!("Failed to decode epoch crs_list: {e}"))
                    })?;
                self.protocol_config_contract
                    .confirmEpochActivation(
                        r.epoch_id,
                        keys,
                        crs_list,
                        extra_data_v2_payload(r.context_id, r.epoch_id).into(),
                        r.signature.into(),
                        r.extra_data.into(),
                    )
                    .into_transaction_request()
            }
            _ => {
                unreachable!("Only keygen and ProtocolConfig responses should be sent to Ethereum")
            }
        };
        Ok(call)
    }

    /// Sends the requested transaction with retries.
    ///
    /// The gas_limit is re-estimated and (likely) increased at each attempt.
    async fn send_tx_with_retry(
        &self,
        call: TransactionRequest,
    ) -> Result<TransactionReceipt, Error> {
        for i in 1..=self.config.tx_retries {
            match self.send_tx_with_increased_gas_limit(call.clone()).await {
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

    async fn send_tx_with_increased_gas_limit(
        &self,
        mut call: TransactionRequest,
    ) -> Result<TransactionReceipt, Error> {
        // Force a fresh gas estimation on each attempt to account for state drift
        call.gas = None;
        overprovision_gas(
            &self.provider,
            self.config.gas_multiplier_percent,
            &mut call,
        )
        .await?;

        let tx = self.provider.send_transaction(call).await?;
        info!(
            "Tx sent to RPC node. Waiting for receipt ({} confirmations)...",
            self.config.tx_required_confirmations
        );
        let receipt = tx
            .with_required_confirmations(self.config.tx_required_confirmations)
            .with_timeout(Some(self.config.get_receipt_timeout))
            .get_receipt()
            .await?;

        if !receipt.status() {
            let revert_reason =
                get_revert_reason(&self.provider, &receipt, self.config.trace_reverted_tx)
                    .await
                    .unwrap_or_else(|e| e.to_string());

            return Err(Error::Recoverable(anyhow!(
                "{revert_reason}. Tx hash {}",
                hex::encode(receipt.transaction_hash)
            )));
        }
        Ok(receipt)
    }
}

impl<P> Clone for EthereumTransactionSender<P>
where
    P: Provider + Clone,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            kms_generation_contract: self.kms_generation_contract.clone(),
            protocol_config_contract: self.protocol_config_contract.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::{Ethereum, IntoWallet, Network, NetworkTransactionBuilder, TransactionBuilder},
        primitives::Address,
        providers::{
            ProviderBuilder, SendableTx,
            fillers::{FillProvider, FillerControlFlow, TxFiller},
            mock::Asserter,
        },
        rpc::types::trace::geth::GethTrace,
        transports::{RpcError, TransportResult},
    };
    use connector_utils::{
        config::KmsWallet,
        provider::NonceManagedProvider,
        tests::rand::{rand_signature, rand_u256},
        types::PrepKeygenResponse,
    };
    use serde::de::DeserializeOwned;
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
        let receipt: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_send_tx_sync.json"))?;
        let debug_trace_tx: GethTrace =
            parse_mock(&format!("{test_data_dir}/4_debug_trace_tx.json"))?;

        // Mock sequence for the two-step send_transaction + get_receipt path:
        // 1. eth_estimateGas (from overprovision_gas)
        asserter.push_success(&estimate_gas);
        // 2. eth_getTransactionCount (from NonceManagedProvider)
        asserter.push_success(&nonce);
        // 3. eth_sendRawTransaction (returns tx hash)
        asserter.push_success(&receipt.transaction_hash);
        // 4. eth_getTransactionReceipt (from watch_pending_transaction initial check;
        asserter.push_success(&receipt);
        // 5. eth_getTransactionReceipt (from get_receipt loop body)
        asserter.push_success(&receipt);
        // 6. debug_traceTransaction (from get_revert_reason)
        asserter.push_success(&debug_trace_tx);

        // Mock out of gas tx
        let tx_sender = EthereumTransactionSender::new(
            mock_provider.clone(),
            KMSGenerationInstance::new(Address::default(), mock_provider.clone()),
            ProtocolConfigInstance::new(Address::default(), mock_provider.clone()),
            EthereumSenderConfig {
                tx_retries: 1,
                trace_reverted_tx: true,
                gas_multiplier_percent: 105,
                tx_required_confirmations: 1,
                get_receipt_timeout: Duration::from_secs(10),
                tx_retry_interval: Duration::from_millis(100),
            },
        );
        tx_sender
            .send_to_ethereum(KmsResponseKind::PrepKeygen(PrepKeygenResponse {
                prep_keygen_id: rand_u256(),
                signature: rand_signature(),
            }))
            .await
            .unwrap_err();
        logs_contain("out of gas");
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
