use alloy::{
    network::{Network, TransactionBuilder},
    primitives::{Address, Bytes, U64},
    providers::{
        EthCall, PendingTransactionBuilder, Provider, RootProvider, SendableTx,
        fillers::{BlobGasFiller, CachedNonceManager, GasFiller, JoinFill, NonceManager},
    },
    transports::TransportResult,
};
use futures::lock::Mutex;
use std::sync::Arc;

pub type FillersWithoutNonceManagement = JoinFill<GasFiller, BlobGasFiller>;

/// A wrapper around an `alloy` provider that recovers its nonce manager on error.
///
/// Note that the provider given by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
/// Users can use the default `FillersWithoutNonceManagement` to create a provider.
pub struct NonceManagedProvider<P> {
    inner: P,
    signer_address: Address,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
}

impl<P> NonceManagedProvider<P> {
    pub fn new(provider: P, signer_address: Address) -> Self {
        Self {
            inner: provider,
            signer_address,
            nonce_manager: Default::default(),
        }
    }
}

impl<P: Clone> Clone for NonceManagedProvider<P> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            signer_address: self.signer_address,
            nonce_manager: self.nonce_manager.clone(),
        }
    }
}

// We forward to `self.inner` exactly the methods that rely on the wallet: the `send_*`/
// `sign_transaction` methods (which build and sign the transaction) and `call`/`estimate_gas`
// (which need the `from` field set to the signer address). Any other method left to the trait
// default reaches the same underlying node and behaves identically, so it doesn't need forwarding.
//
// If a wallet-dependent method were missing here, it would fall back to the trait default, which
// skips the fillers: `estimate_gas`, for example, would then run with `from = 0x0` instead of the
// signer address, and could revert on contracts that restrict callers. This list is complete for
// the current `alloy` version, and the unit tests below guard the methods our services rely on.
#[async_trait::async_trait]
impl<N, P> Provider<N> for NonceManagedProvider<P>
where
    N: Network,
    P: Provider<N>,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }

    async fn send_transaction_sync(
        &self,
        mut tx: N::TransactionRequest,
    ) -> TransportResult<N::ReceiptResponse> {
        let nonce = self
            .nonce_manager
            .lock()
            .await
            .get_next_nonce(&self.inner, self.signer_address)
            .await?;
        tx.set_nonce(nonce);
        let res = self.inner.send_transaction_sync(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    async fn send_transaction(
        &self,
        mut tx: N::TransactionRequest,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        let nonce = self
            .nonce_manager
            .lock()
            .await
            .get_next_nonce(&self.inner, self.signer_address)
            .await?;
        tx.set_nonce(nonce);
        let res = self.inner.send_transaction(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    // Not used but overridden for consistency with other send_transaction methods.
    async fn send_transaction_sync_internal(
        &self,
        tx: SendableTx<N>,
    ) -> TransportResult<N::ReceiptResponse> {
        let tx = match tx {
            SendableTx::Builder(mut tx) => {
                let nonce = self
                    .nonce_manager
                    .lock()
                    .await
                    .get_next_nonce(&self.inner, self.signer_address)
                    .await?;
                tx.set_nonce(nonce);
                SendableTx::Builder(tx)
            }
            // An envelope is already signed, with its nonce baked in.
            tx => tx,
        };
        let res = self.inner.send_transaction_sync_internal(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    // Not used but overridden for consistency with other send_transaction methods.
    async fn send_transaction_internal(
        &self,
        tx: SendableTx<N>,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        let tx = match tx {
            SendableTx::Builder(mut tx) => {
                let nonce = self
                    .nonce_manager
                    .lock()
                    .await
                    .get_next_nonce(&self.inner, self.signer_address)
                    .await?;
                tx.set_nonce(nonce);
                SendableTx::Builder(tx)
            }
            // An envelope is already signed, with its nonce baked in.
            tx => tx,
        };
        let res = self.inner.send_transaction_internal(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    async fn sign_transaction(&self, tx: N::TransactionRequest) -> TransportResult<Bytes> {
        self.inner.sign_transaction(tx).await
    }

    fn call(&self, tx: N::TransactionRequest) -> EthCall<N, Bytes> {
        self.inner.call(tx)
    }

    fn estimate_gas(&self, tx: N::TransactionRequest) -> EthCall<N, U64, u64> {
        self.inner.estimate_gas(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::KmsWallet, conn::WalletProvider};
    use alloy::{
        providers::ProviderBuilder,
        rpc::{client::RpcClient, json_rpc::RequestPacket, types::TransactionRequest},
        transports::mock::{Asserter, MockTransport},
    };
    use std::sync::Mutex as StdMutex;

    /// The JSON-RPC requests received by the mocked node, serialized, in order.
    type RecordedRequests = Arc<StdMutex<Vec<String>>>;

    // Builds the exact provider stack used in production on top of a mocked transport that
    // records every JSON-RPC request it receives.
    fn wallet_provider(asserter: Asserter) -> (WalletProvider, Address, RecordedRequests) {
        let wallet = KmsWallet::from_private_key_str(
            "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774",
            None,
        )
        .unwrap();
        let signer_address = wallet.address();

        let requests = RecordedRequests::default();
        let requests_recorder = requests.clone();
        let transport = tower::ServiceBuilder::new()
            .map_request(move |request: RequestPacket| {
                if let RequestPacket::Single(single) = &request {
                    let serialized = serde_json::to_string(single).unwrap();
                    requests_recorder.lock().unwrap().push(serialized);
                }
                request
            })
            .service(MockTransport::new(asserter));

        let inner_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .with_chain_id(1)
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet)
            .connect_client(RpcClient::new(transport, true));
        let provider = NonceManagedProvider::new(inner_provider, signer_address);
        (provider, signer_address, requests)
    }

    // Extracts the `from` field of the single `expected_method` request the node received.
    fn from_field_received_by_node(
        requests: &RecordedRequests,
        expected_method: &str,
    ) -> Option<Address> {
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let request: serde_json::Value = serde_json::from_str(&requests[0]).unwrap();
        assert_eq!(request["method"], expected_method);
        request["params"][0]["from"]
            .as_str()
            .map(|from| from.parse().unwrap())
    }

    // `estimate_gas` must delegate to the inner provider so its wallet filler populates `from`.
    // If it fell back to the `Provider` trait default (which routes through the fillerless
    // `RootProvider`), the node would run the estimation with `from = 0x0` instead of the signer
    // address, which could revert on contracts that restrict callers.
    #[tokio::test]
    async fn estimate_gas_goes_through_wallet_filler() {
        let asserter = Asserter::new();
        let (provider, signer_address, requests) = wallet_provider(asserter.clone());
        asserter.push_success(&U64::from(21_000));

        // A request with no `from`, as produced by `CallBuilder::into_transaction_request()`.
        provider
            .estimate_gas(TransactionRequest::default())
            .await
            .unwrap();

        assert_eq!(
            from_field_received_by_node(&requests, "eth_estimateGas"),
            Some(signer_address)
        );
    }

    // Same guarantee for `call`, which the tx-sender relies on for caller-gated contract reads.
    #[tokio::test]
    async fn call_goes_through_wallet_filler() {
        let asserter = Asserter::new();
        let (provider, signer_address, requests) = wallet_provider(asserter.clone());
        asserter.push_success(&Bytes::new());

        provider.call(TransactionRequest::default()).await.unwrap();

        assert_eq!(
            from_field_received_by_node(&requests, "eth_call"),
            Some(signer_address)
        );
    }
}
