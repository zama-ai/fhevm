use std::{future::IntoFuture, sync::Arc};

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, CachedNonceManager, ChainIdFiller, GasFiller, JoinFill, NonceManager,
        },
        DynProvider, PendingTransactionBuilder, Provider, ProviderBuilder, WsConnect,
    },
    rpc::types::TransactionRequest,
    transports::{RpcError, TransportErrorKind, TransportResult},
};
use futures_util::lock::Mutex;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::ConfigSettings;

pub type FillersWithoutNonceManagement =
    JoinFill<GasFiller, JoinFill<BlobGasFiller, ChainIdFiller>>;

/// A wrapper around an `alloy` provider that sends transactions with the correct nonce.
/// Note that the given provider by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself. Users can use the default `FillersWithoutNonceManagement` to create a provider.
#[derive(Clone)]
pub struct NonceManagedProvider {
    provider: Arc<RwLock<Option<DynProvider<Ethereum>>>>,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
    signer_address: Option<Address>,
    conf: ConfigSettings,
    wallet: EthereumWallet,
}

impl NonceManagedProvider {
    async fn create_provider(
        conf: &ConfigSettings,
        wallet: &EthereumWallet,
    ) -> TransportResult<DynProvider<Ethereum>> {
        match ProviderBuilder::default()
            .filler(FillersWithoutNonceManagement::default())
            .wallet(wallet.clone())
            .connect_ws(
                WsConnect::new(conf.gateway_url.clone())
                    .with_max_retries(conf.provider_max_retries)
                    .with_retry_interval(conf.provider_retry_interval),
            )
            .await
        {
            Ok(p) => {
                info!(
                    gateway_url = %conf.gateway_url,
                    "Connected to Gateway"
                );
                Ok(DynProvider::new(p))
            }
            Err(e) => {
                error!(
                    gateway_url = %conf.gateway_url,
                    error = %e,
                    retry_interval = ?conf.provider_retry_interval,
                    "Failed to connect to Gateway"
                );
                Err(e)
            }
        }
    }

    async fn get_or_try_create_provider(&self) -> TransportResult<DynProvider<Ethereum>> {
        // First try a read lock.
        if let Some(p) = &*self.provider.read().await {
            return Ok(p.clone());
        }

        // Then a write lock to create the provider.
        let mut write_lock = self.provider.write().await;
        let provider = Self::create_provider(&self.conf, &self.wallet).await;
        match provider {
            Ok(p) => {
                *write_lock = Some(p);
                Ok(write_lock.clone().unwrap())
            }
            Err(e) => Err(e),
        }
    }

    async fn with_provider_reset<T, F, Fut>(&self, f: F) -> TransportResult<T>
    where
        F: FnOnce(DynProvider<Ethereum>) -> Fut,
        Fut: IntoFuture<Output = TransportResult<T>>,
    {
        let provider = self.get_or_try_create_provider().await?;
        let res = f(provider).await;
        if let Err(RpcError::Transport(TransportErrorKind::BackendGone)) = res {
            *self.provider.write().await = None;
        }
        res
    }

    pub fn new(
        conf: &ConfigSettings,
        wallet: &EthereumWallet,
        signer_address: Option<Address>,
    ) -> Self {
        Self {
            provider: Arc::new(RwLock::new(None)),
            nonce_manager: Default::default(),
            signer_address,
            conf: conf.clone(),
            wallet: wallet.clone(),
        }
    }

    pub async fn send_transaction(
        &self,
        tx: impl Into<TransactionRequest>,
    ) -> TransportResult<PendingTransactionBuilder<Ethereum>> {
        let provider = self.get_or_try_create_provider().await?;
        let mut tx = tx.into();
        if let Some(signer_address) = self.signer_address {
            let nonce_manager = self.nonce_manager.lock().await;
            let nonce = nonce_manager
                .get_next_nonce(&provider, signer_address)
                .await?;
            tx.nonce = Some(nonce);
        }
        let res = self
            .with_provider_reset(|p| async move { p.send_transaction(tx).await })
            .await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    pub async fn get_chain_id(&self) -> TransportResult<u64> {
        self.with_provider_reset(|p| p.get_chain_id()).await
    }

    pub async fn get_block_number(&self) -> TransportResult<u64> {
        self.with_provider_reset(|p| p.get_block_number()).await
    }

    pub async fn get_transaction_count(&self, address: Address) -> TransportResult<u64> {
        self.with_provider_reset(|p| p.get_transaction_count(address))
            .await
    }

    pub async fn estimate_gas(&self, tx: impl Into<TransactionRequest>) -> TransportResult<u64> {
        self.with_provider_reset(|p| p.estimate_gas(tx.into()))
            .await
    }

    pub async fn inner(&self) -> TransportResult<DynProvider<Ethereum>> {
        self.get_or_try_create_provider().await
    }
}
