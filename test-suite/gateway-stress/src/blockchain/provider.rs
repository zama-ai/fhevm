use crate::blockchain::nonce_manager::ZamaNonceManager;
use alloy::{
    network::{Network, TransactionBuilder},
    primitives::{Address, Bytes, U64},
    providers::{
        EthCall, PendingTransactionBuilder, Provider, RootProvider, SendableTx,
        fillers::{BlobGasFiller, GasFiller, JoinFill, NonceManager},
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
    nonce_manager: Arc<Mutex<ZamaNonceManager>>,
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
