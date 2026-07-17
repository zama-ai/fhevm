use alloy::{
    network::{Network, TransactionBuilder},
    primitives::{Address, Bytes},
    providers::{
        PendingTransactionBuilder, Provider, RootProvider, SendableTx,
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

// Only the transaction sending/signing methods are overridden, as the `TxFiller`s of the inner
// provider only act on these operations.
// All other methods intentionally use the trait defaults, which perform their RPC requests via
// `self.root()`, i.e. the same client the inner provider forwards to. If a layer intercepting
// other operations is ever added to the inner provider, forward these operations here as well.
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
}
