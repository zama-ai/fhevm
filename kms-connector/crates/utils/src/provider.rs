use alloy::{
    network::{Ethereum, Network},
    primitives::Address,
    providers::{
        PendingTransactionBuilder, Provider,
        fillers::{
            BlobGasFiller, CachedNonceManager, ChainIdFiller, GasFiller, JoinFill, NonceManager,
        },
    },
    transports::TransportResult,
};
use futures::lock::Mutex;
use std::sync::Arc;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A wrapper around an `alloy` provider that recover its nonce manager on error.
///
/// Note that the provider given by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
/// Users can use the default `FillersWithoutNonceManagement` to create a provider.
pub struct NonceManagedProvider<P> {
    inner: P,
    signer_address: Address,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
}

#[async_trait::async_trait]
impl<P> Provider for NonceManagedProvider<P>
where
    P: Provider,
{
    fn root(&self) -> &alloy::providers::RootProvider<Ethereum> {
        self.inner.root()
    }

    async fn send_transaction(
        &self,
        mut tx: <Ethereum as Network>::TransactionRequest,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let nonce = self
            .nonce_manager
            .lock()
            .await
            .get_next_nonce(&self.inner, self.signer_address)
            .await?;
        tx.nonce = Some(nonce);
        let res = self.inner.send_transaction(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }
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
