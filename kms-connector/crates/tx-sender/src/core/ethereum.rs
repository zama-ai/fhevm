use crate::core::tx_sender::{Error, get_revert_reason, overprovision_gas};
use alloy::{
    hex,
    providers::{Provider, fillers::TxFiller},
    rpc::types::{TransactionReceipt, TransactionRequest},
};
use anyhow::anyhow;
use connector_utils::{
    provider::NonceManagedProvider,
    types::{CrsgenResponse, KeygenResponse, KmsResponseKind, PrepKeygenResponse},
};
use fhevm_host_bindings::kms_generation::KMSGeneration::KMSGenerationInstance;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// The internal struct used to send keygen transactions to Ethereum.
pub struct EthereumTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider,
{
    provider: NonceManagedProvider<F, P>,
    kms_generation_contract: KMSGenerationInstance<NonceManagedProvider<F, P>>,
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

impl<F, P> EthereumTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider,
{
    pub fn new(
        provider: NonceManagedProvider<F, P>,
        kms_generation_contract: KMSGenerationInstance<NonceManagedProvider<F, P>>,
        inner_config: EthereumSenderConfig,
    ) -> Self {
        Self {
            provider,
            kms_generation_contract,
            config: inner_config,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn send_to_ethereum(&self, response: KmsResponseKind) -> Result<(), Error> {
        info!("Sending response to Ethereum: {response:?}");
        let tx_result = match response {
            KmsResponseKind::PrepKeygen(response) => self.send_prep_keygen_response(response).await,
            KmsResponseKind::Keygen(response) => self.send_keygen_response(response).await,
            KmsResponseKind::Crsgen(response) => self.send_crsgen_response(response).await,
            _ => unreachable!("Only keygen responses should be sent to Ethereum"),
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

    pub async fn send_prep_keygen_response(
        &self,
        response: PrepKeygenResponse,
    ) -> Result<TransactionReceipt, Error> {
        let call_builder = self
            .kms_generation_contract
            .prepKeygenResponse(response.prep_keygen_id, response.signature.into());
        debug!("Calldata length {}", call_builder.calldata().len());

        let call = call_builder.into_transaction_request();
        self.send_tx_with_retry(call).await
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
        self.send_tx_with_retry(call).await
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
        self.send_tx_with_retry(call).await
    }

    /// Sends the requested transaction with retries.
    ///
    /// The `gas_limit` is increased at each attempts.
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
        info!("Tx sent to RPC node. Waiting for finalized receipt (~64 confirmations)...");
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

impl<F, P> Clone for EthereumTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider + Clone,
{
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            kms_generation_contract: self.kms_generation_contract.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::{Ethereum, IntoWallet, Network, TransactionBuilder},
        primitives::Address,
        providers::{
            ProviderBuilder, SendableTx,
            fillers::{FillProvider, FillerControlFlow},
            mock::Asserter,
        },
        rpc::types::trace::geth::GethTrace,
        transports::{RpcError, TransportResult},
    };
    use connector_utils::{
        config::KmsWallet,
        tests::rand::{rand_signature, rand_u256},
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
        //    since ETHEREUM_FINALIZED_CONFIRMATIONS == 1 in test, returns PendingTransaction::ready)
        asserter.push_success(&receipt);
        // 5. eth_getTransactionReceipt (from get_receipt loop body)
        asserter.push_success(&receipt);
        // 6. debug_traceTransaction (from get_revert_reason)
        asserter.push_success(&debug_trace_tx);

        // Mock out of gas tx
        let tx_sender = EthereumTransactionSender::new(
            mock_provider.clone(),
            KMSGenerationInstance::new(Address::default(), mock_provider.clone()),
            EthereumSenderConfig {
                tx_retries: 1,
                trace_reverted_tx: true,
                gas_multiplier_percent: 105,
                ..Default::default()
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
