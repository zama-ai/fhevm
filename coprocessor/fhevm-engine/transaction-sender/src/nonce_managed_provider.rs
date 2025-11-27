use std::{sync::Arc, time::Duration};

use alloy::{
    network::{Ethereum, TransactionBuilder},
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, CachedNonceManager, ChainIdFiller, GasFiller, JoinFill, NonceManager,
        },
        PendingTransactionBuilder, Provider,
    },
    rpc::types::{TransactionReceipt, TransactionRequest},
    transports::{TransportErrorKind, TransportResult},
};
use futures_util::lock::Mutex;
use tracing::{debug, warn};

use crate::config::DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A wrapper around an `alloy` provider that sends transactions with the correct nonce.
/// Note that the given provider by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
#[derive(Clone)]
pub struct NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
    provider: P,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
    signer_address: Option<Address>,
}

impl<P> NonceManagedProvider<P>
where
    P: Provider<Ethereum>,
{
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

    pub async fn send_transaction_sync(
        &self,
        tx: impl Into<TransactionRequest>,
        timeout: Duration,
    ) -> TransportResult<TransactionReceipt> {
        let mut tx = tx.into();
        if let Some(signer_address) = self.signer_address {
            let nonce_manager = self.nonce_manager.lock().await;
            let nonce = nonce_manager
                .get_next_nonce(&self.provider, signer_address)
                .await?;
            tx.nonce = Some(nonce);
        }
        let res = tokio::time::timeout(timeout, self.provider.send_transaction_sync(tx))
            .await
            .map_err(|_| TransportErrorKind::custom_str("eth_sendRawTransactionSync timeout"))
            .flatten();
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    /// If `txn_request.gas` is set, overprovision it by the given percent.
    /// If `txn_request.gas` is not set, estimate the gas limit and then overprovision it by the given percent.
    /// If the percent is less than 100, DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT is used.
    pub async fn overprovision_gas_limit(
        &self,
        txn_request: impl Into<TransactionRequest>,
        percent: u32,
    ) -> TransportResult<TransactionRequest> {
        let percent = if percent < 100 {
            warn!(
                gas_limit_overprovision_percent = percent,
                default_gas_limit_overprovision_percent = DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT,
                "Overprovision percent is less than 100, using default value instead"
            );
            DEFAULT_GAS_LIMIT_OVERPROVISION_PERCENT
        } else {
            percent
        };

        let overprovision = |gas: u64| (gas as u128 * percent as u128 / 100) as u64;

        let mut txn: TransactionRequest = txn_request.into();

        let new_gas = match txn.gas {
            Some(existing_gas) => Some(existing_gas),
            None => Some(self.provider.estimate_gas(txn.clone()).await?),
        }
        .map(overprovision);

        if let Some(gas) = new_gas {
            debug!(
                gas_limit = gas,
                gas_limit_overprovision_percent = percent,
                "Overprovisioned gas limit"
            );
            txn.set_gas_limit(gas);
        }

        Ok(txn)
    }

    // Ensure that if gas estimation fails due to a revert, the transaction is not sent and no nonce is consumed.
    pub async fn send_sync_with_overprovision(
        &self,
        txn_request: impl Into<TransactionRequest>,
        percent: u32,
        send_sync_timeout: Duration,
    ) -> TransportResult<alloy::rpc::types::TransactionReceipt> {
        let overprovisioned_txn = self.overprovision_gas_limit(txn_request, percent).await?;
        self.send_transaction_sync(overprovisioned_txn, send_sync_timeout)
            .await
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
