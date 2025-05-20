use std::{ops::Deref, sync::Arc};

use alloy::{
    network::Ethereum,
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, CachedNonceManager, ChainIdFiller, GasFiller, JoinFill, NonceManager,
        },
        PendingTransactionBuilder,
    },
    rpc::types::TransactionRequest,
    transports::TransportResult,
};
use futures_util::lock::Mutex;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A wrapper around an `alloy` provider that sends transactions with the correct nonce.
/// Note that the given provider by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself. Users can use the default `FillersWithoutNonceManagement` to create a provider.
#[derive(Clone)]
pub struct NonceManagedProvider<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    provider: P,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
    signer_address: Option<Address>,
}

impl<P: alloy::providers::Provider<Ethereum> + Clone + 'static> NonceManagedProvider<P> {
    pub fn new(provider: P, signer_address: Option<Address>) -> Self {
        Self {
            provider,
            nonce_manager: Default::default(),
            signer_address,
        }
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let mut tx = tx.into();
        if let Some(signer_address) = self.signer_address {
            let nonce_manager = self.nonce_manager.lock().await;
            let nonce = nonce_manager
                .get_next_nonce(&self.provider, signer_address)
                .await?;
            tx.nonce = Some(nonce);
        }
        let res = self.provider.send_transaction(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    pub async fn get_chain_id(&self) -> TransportResult<u64> {
        self.provider.get_chain_id().await
    }

    pub async fn get_transaction_count(&self, address: Address) -> TransportResult<u64> {
        self.provider.get_transaction_count(address).await
    }

    pub fn inner(&self) -> &P {
        &self.provider
    }
}

impl<P> Deref for NonceManagedProvider<P>
where
    P: alloy::providers::Provider<Ethereum> + Clone + 'static,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.provider
    }
}
