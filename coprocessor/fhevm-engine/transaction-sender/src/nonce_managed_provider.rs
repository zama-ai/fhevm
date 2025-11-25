use std::sync::Arc;

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
    transports::{RpcError, TransportResult},
};
use futures_util::lock::Mutex;
use tracing::warn;

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
    retry_immediately_on_nonce_issue: u64,
}

pub fn is_nonce_error(err: &RpcError<impl std::fmt::Debug>) -> bool {
    if let RpcError::ErrorResp(err) = err {
        // server returned an error response: error code -32003: Nonce too high err
        if err.code == -32003 && (err.message.contains("Nonce") || err.message.contains("nonce")) {
            return true;
        }
    }
    false
}

impl<P: alloy::providers::Provider<Ethereum> + Clone + 'static> NonceManagedProvider<P> {
    pub fn new(provider: P, signer_address: Option<Address>) -> Self {
        Self {
            provider,
            nonce_manager: Default::default(),
            signer_address,
            retry_immediately_on_nonce_issue: 0,
        }
    }

    pub fn new_with_nonce_retry(
        provider: P,
        signer_address: Option<Address>,
        retry_immediately_on_nonce_issue: u64,
    ) -> Self {
        Self {
            provider,
            nonce_manager: Default::default(),
            signer_address,
            retry_immediately_on_nonce_issue,
        }
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let tx_req = tx.into();
        let Some(signer_address) = self.signer_address else {
            // no signer address provided, just send the transaction as is
            return self.provider.send_transaction(tx_req.clone()).await;
        };
        let mut retry = self.retry_immediately_on_nonce_issue as i64;
        loop {
            let mut tx: TransactionRequest = tx_req.clone();
            tx.nonce = {
                let nonce_manager = self.nonce_manager.lock().await;
                let nonce = nonce_manager
                    .get_next_nonce(&self.provider, signer_address)
                    .await?;
                Some(nonce)
            };
            let returned = self.provider.send_transaction(tx).await;
            if let Err(err) = &returned {
                // Reset the nonce manager if the transaction sending failed.
                *self.nonce_manager.lock().await = Default::default();
                if retry > 0 && is_nonce_error(err) {
                    retry -= 1;
                    let msg = err.to_string();
                    warn!(msg, "Transaction failed due to nonce, resetting nonce manager and retrying immediately");
                    continue;
                }
                warn!(err = ?err, "Transaction failed, resetting nonce manager");
            }
            return returned;
        }
    }

    pub async fn get_chain_id(&self) -> TransportResult<u64> {
        self.provider.get_chain_id().await
    }

    pub async fn get_transaction_count(&self, address: Address) -> TransportResult<u64> {
        self.provider.get_transaction_count(address).await
    }

    pub async fn get_block_number(&self) -> TransportResult<u64> {
        self.provider.get_block_number().await
    }

    pub fn inner(&self) -> &P {
        &self.provider
    }
}
