use std::sync::Arc;

use alloy::{
    eips::Encodable2718,
    network::{AnyTransactionReceipt, Ethereum, Network},
    primitives::{Address, Bytes},
    providers::{
        fillers::{BlobGasFiller, FillProvider, GasFiller, JoinFill, TxFiller},
        Provider,
    },
    rpc::types::TransactionRequest,
    transports::{TransportError, TransportResult},
};

use crate::gateway::arbitrum::transaction::nonce_manager::NonceManagerNonOptimistic;

pub type FillersWithoutNonceManagement = JoinFill<GasFiller, BlobGasFiller>;

#[derive(Debug)]
pub struct NonceManagedProvider<F, P, N = Ethereum>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N>,
{
    pub inner: FillProvider<F, P, N>,
    signer_address: Address,
    pub nonce_manager: Arc<NonceManagerNonOptimistic>,
}

impl<F, P> NonceManagedProvider<F, P>
where
    F: TxFiller<Ethereum>,
    P: Provider<Ethereum>,
{
    pub fn new(
        provider: FillProvider<F, P, Ethereum>,
        signer_address: Address,
        nonce_manager: Arc<NonceManagerNonOptimistic>,
    ) -> Self {
        Self {
            inner: provider,
            signer_address,
            nonce_manager,
        }
    }

    pub async fn send_raw_transaction_sync(
        &self,
        tx: TransactionRequest,
    ) -> TransportResult<AnyTransactionReceipt> {
        let mut tx_bytes = Vec::new();
        // TODO: Catch this error.
        self.inner
            .fill(tx)
            .await?
            .try_into_envelope()
            .map_err(|e| TransportError::LocalUsageError(Box::new(e)))?
            .encode_2718(&mut tx_bytes);

        self.inner
            .client()
            .request("eth_sendRawTransactionSync", (Bytes::from(tx_bytes),))
            .await
    }
}

impl<F, P, N> Clone for NonceManagedProvider<F, P, N>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            signer_address: self.signer_address,
            nonce_manager: self.nonce_manager.clone(),
        }
    }
}
