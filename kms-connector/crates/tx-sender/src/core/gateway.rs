use std::time::Duration;

use crate::{
    core::tx_sender::{Error, get_revert_reason, overprovision_gas},
    monitoring::metrics::{GATEWAY_TX_SENT_COUNTER, GATEWAY_TX_SENT_ERRORS},
};
use alloy::{
    hex,
    providers::{Provider, fillers::TxFiller},
    rpc::types::{TransactionReceipt, TransactionRequest},
};
use anyhow::anyhow;
use connector_utils::{
    provider::NonceManagedProvider,
    types::{KmsResponseKind, PublicDecryptionResponse, UserDecryptionResponse},
};
use fhevm_gateway_bindings::decryption::Decryption::DecryptionInstance;
use tracing::{debug, error, info, warn};

/// The struct used to send decryption transactions to the Gateway.
pub struct GatewayTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider,
{
    provider: NonceManagedProvider<F, P>,
    decryption_contract: DecryptionInstance<NonceManagedProvider<F, P>>,
    config: GatewaySenderConfig,
}

#[derive(Clone, Default)]
pub struct GatewaySenderConfig {
    pub tx_retries: u8,
    pub tx_retry_interval: Duration,
    pub trace_reverted_tx: bool,
    pub gas_multiplier_percent: usize,
}

impl From<&super::Config> for GatewaySenderConfig {
    fn from(config: &super::Config) -> Self {
        Self {
            tx_retries: config.tx_retries,
            tx_retry_interval: config.tx_retry_interval,
            trace_reverted_tx: config.trace_reverted_tx,
            gas_multiplier_percent: config.gas_multiplier_percent,
        }
    }
}

impl<F, P> GatewayTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider,
{
    pub fn new(
        provider: NonceManagedProvider<F, P>,
        decryption_contract: DecryptionInstance<NonceManagedProvider<F, P>>,
        inner_config: GatewaySenderConfig,
    ) -> Self {
        Self {
            provider,
            decryption_contract,
            config: inner_config,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn send_to_gateway(&self, response: KmsResponseKind) -> Result<(), Error> {
        info!("Sending response to the Gateway: {response:?}");
        let response_str = response.as_str();
        let tx_result = match response {
            KmsResponseKind::PublicDecryption(response) => {
                self.send_public_decryption_response(response).await
            }
            KmsResponseKind::UserDecryption(response) => {
                self.send_user_decryption_response(response).await
            }
            _ => unreachable!("Only decryption responses should be sent to the Gateway"),
        };

        let receipt = tx_result.inspect_err(|e| {
            GATEWAY_TX_SENT_ERRORS
                .with_label_values(&[response_str])
                .inc();
            error!("Failed to send response to the Gateway: {e}");
        })?;

        debug!("Transaction receipt: {:?}", receipt);
        GATEWAY_TX_SENT_COUNTER
            .with_label_values(&[response_str])
            .inc();
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

    /// Sends the requested transaction with retries.
    ///
    /// The gas_limit is re-estimated and (likely) increased at each attempt.
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
        overprovision_gas(
            self.decryption_contract.provider(),
            self.config.gas_multiplier_percent,
            &mut call,
        )
        .await?;

        let receipt = self.provider.send_transaction_sync(call).await?;
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

impl<F, P> Clone for GatewayTransactionSender<F, P>
where
    F: TxFiller,
    P: Provider + Clone,
{
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
        let send_tx_sync: TransactionReceipt =
            parse_mock(&format!("{test_data_dir}/3_send_tx_sync.json"))?;
        let debug_trace_tx: GethTrace =
            parse_mock(&format!("{test_data_dir}/4_debug_trace_tx.json"))?;
        asserter.push_success(&estimate_gas);
        asserter.push_success(&nonce);
        asserter.push_success(&send_tx_sync);
        asserter.push_success(&debug_trace_tx);

        // Mock out of gas tx
        let inner_sender = GatewayTransactionSender::new(
            mock_provider.clone(),
            DecryptionInstance::new(Address::default(), mock_provider.clone()),
            GatewaySenderConfig {
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
