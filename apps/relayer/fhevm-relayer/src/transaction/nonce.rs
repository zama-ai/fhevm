use alloy::network::AnyNetwork;
use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::fillers::NonceManager;
use alloy::providers::Provider;
use alloy::transports::TransportResult;
use async_trait::async_trait;
use dashmap::DashMap;
use futures::lock::Mutex;
use std::sync::Arc;
use tracing::{debug, trace};

/// Cached nonce manager
///
/// This [`NonceManager`] implementation will fetch the transaction count for any new account it
/// sees, store it locally and increment the locally stored nonce as transactions are sent via
/// [`Provider::send_transaction`].
///
/// There is also an alternative implementation [`SimpleNonceManager`] that does not store the
/// transaction count locally.
#[derive(Clone, Debug, Default)]
pub struct CachedNonceManagerWithRefresh {
    nonces: Arc<DashMap<Address, Arc<Mutex<u64>>>>,
}

// TODO: make sure this is consistent!
impl CachedNonceManagerWithRefresh {
    pub async fn sync_nonce<P, N: Network>(
        &self,
        provider: &Arc<P>, //  + Send + Sync
        address: Address,
    ) -> TransportResult<u64>
    where
        P: Provider<N> + ?Sized,
    {
        // Nonce is a u64 but it's incremented whenever `get_next_nonce` is called
        const NONE: u64 = u64::MAX;
        let new_nonce: u64 = provider.get_transaction_count(address).pending().await?;

        let nonce = {
            let rm = self
                .nonces
                .entry(address)
                .or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        {
            let mut nonce = nonce.lock().await;
            *nonce = new_nonce;
        }

        Ok(new_nonce)
    }
}

/// Hack to be able to call this method with `dyn`s instead of generics.
pub trait DebugNonceManager {
    #![allow(async_fn_in_trait)]
    async fn increase_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<()>;

    async fn decrease_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<()>;

    async fn current_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<u64>;
}

// TODO: define behavior when nonce isn't set
// TODO: add tests for it
impl DebugNonceManager for CachedNonceManagerWithRefresh {
    async fn current_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<u64> {
        // Use `u64::MAX` as a sentinel value to indicate that the nonce has not been fetched yet.
        const NONE: u64 = u64::MAX;

        // Locks dashmap internally for a short duration to clone the `Arc`.
        // We also don't want to hold the dashmap lock through the await point below.
        let nonce = {
            let rm = self
                .nonces
                .entry(address)
                .or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        let nonce_guard = nonce.lock().await;
        let current_nonce = if *nonce_guard == NONE {
            // Initialize the nonce if we haven't seen this account before.
            debug!(%address, "fetching nonce");
            provider.get_transaction_count(address).await?
        } else {
            debug!(%address, current_nonce = *nonce_guard, "incrementing nonce");
            *nonce_guard
        };
        Ok(current_nonce)
    }

    async fn increase_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<()> {
        // Use `u64::MAX` as a sentinel value to indicate that the nonce has not been fetched yet.
        const NONE: u64 = u64::MAX;

        // Locks dashmap internally for a short duration to clone the `Arc`.
        // We also don't want to hold the dashmap lock through the await point below.
        let nonce = {
            let rm = self
                .nonces
                .entry(address)
                .or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        let mut nonce = nonce.lock().await;
        let new_nonce = if *nonce == NONE {
            // Initialize the nonce if we haven't seen this account before.
            trace!(%address, "fetching nonce");
            provider.get_transaction_count(address).await?
        } else {
            trace!(%address, current_nonce = *nonce, "incrementing nonce");
            *nonce
        };
        *nonce = new_nonce + 1;
        Ok(())
    }

    async fn decrease_nonce(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        address: Address,
    ) -> TransportResult<()> {
        // Use `u64::MAX` as a sentinel value to indicate that the nonce has not been fetched yet.
        const NONE: u64 = u64::MAX;

        // Locks dashmap internally for a short duration to clone the `Arc`.
        // We also don't want to hold the dashmap lock through the await point below.
        let nonce = {
            let rm = self
                .nonces
                .entry(address)
                .or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        let mut nonce = nonce.lock().await;
        let new_nonce = if *nonce == NONE {
            // Initialize the nonce if we haven't seen this account before.
            trace!(%address, "fetching nonce");
            provider.get_transaction_count(address).await?
        } else {
            trace!(%address, current_nonce = *nonce, "incrementing nonce");
            *nonce
        };
        *nonce = new_nonce - 1;
        Ok(())
    }
}

// NOTE: in oposition to [`CachedNonceManager`] we store the nonce to use and not the previous one,
// mainly to avoid having to store a -1 nonce in the case no transactions were made (the
// implementation of [`CachedNonceManager::get_next_nonce`] increasing the value if it's already in
// the nonce map
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl NonceManager for CachedNonceManagerWithRefresh {
    async fn get_next_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        // Use `u64::MAX` as a sentinel value to indicate that the nonce has not been fetched yet.
        const NONE: u64 = u64::MAX;

        // Locks dashmap internally for a short duration to clone the `Arc`.
        // We also don't want to hold the dashmap lock through the await point below.
        let nonce_mutex = {
            let rm = self
                .nonces
                .entry(address)
                .or_insert_with(|| Arc::new(Mutex::new(NONE)));
            Arc::clone(rm.value())
        };

        let mut nonce_guard = nonce_mutex.lock().await;
        let new_nonce = if *nonce_guard == NONE {
            // Initialize the nonce if we haven't seen this account before.
            debug!(%address, "fetching nonce");
            provider.get_transaction_count(address).await?
        } else {
            debug!(%address, current_nonce = *nonce_guard, "incrementing nonce");
            *nonce_guard
        };
        // Increment the value for future tx
        *nonce_guard = new_nonce + 1;
        Ok(new_nonce)
    }
}

#[cfg(not(feature = "ci"))]
#[cfg(test)]
mod tests {
    use crate::transaction::nonce::CachedNonceManagerWithRefresh;
    use alloy::consensus::Transaction;
    use alloy::network::Ethereum;
    use alloy::primitives::address;
    use alloy::primitives::Address;
    use alloy::primitives::U256;
    use alloy::providers::fillers::NonceFiller;
    use alloy::providers::fillers::NonceManager;
    use alloy::providers::Provider;
    use alloy::providers::ProviderBuilder;
    use alloy::providers::WalletProvider;
    use alloy::rpc::types::TransactionRequest;
    use reqwest::Url;
    use std::sync::Arc;

    #[test]
    fn test() {
        let val = true;
        assert!(val);
    }

    // TODO: Remove usage of `connect_anvil_with_wallet` and `connect_anvil` that relies on
    // `anvil` being available on the test machine.
    #[tokio::test]
    async fn increments_nonce() {
        let cnm1 = CachedNonceManagerWithRefresh::default();
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .filler(NonceFiller::new(cnm1))
            .connect_anvil_with_wallet();

        let from = provider.default_signer_address();
        let tx = TransactionRequest {
            from: Some(from),
            value: Some(U256::from(100)),
            to: Some(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into()),
            gas_price: Some(20e9 as u128),
            gas: Some(21000),
            ..Default::default()
        };

        let pending = provider.send_transaction(tx.clone()).await.unwrap();
        let tx_hash = pending.watch().await.unwrap();
        let mined_tx = provider
            .get_transaction_by_hash(tx_hash)
            .await
            .expect("failed to fetch tx")
            .expect("tx not included");
        assert_eq!(mined_tx.nonce(), 0);

        let pending = provider.send_transaction(tx).await.unwrap();
        let tx_hash = pending.watch().await.unwrap();
        let mined_tx = provider
            .get_transaction_by_hash(tx_hash)
            .await
            .expect("fail to fetch tx")
            .expect("tx didn't finalize");
        assert_eq!(mined_tx.nonce(), 1);
    }

    #[tokio::test]
    async fn cloned_managers() {
        let cnm1 = CachedNonceManagerWithRefresh::default();
        let cnm2 = cnm1.clone();

        let provider =
            ProviderBuilder::new().connect_http(Url::parse("http://localhost:8756").unwrap());
        let address = Address::ZERO;

        assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 0);
        assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 1);
        assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 2);
        assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 3);

        let arc_provider: Arc<dyn Provider<Ethereum>> = Arc::new(Box::new(provider.clone()));
        let _ = cnm1.sync_nonce(&arc_provider, address).await;

        assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 0);
        assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 1);
        assert_eq!(cnm1.get_next_nonce(&provider, address).await.unwrap(), 2);
        assert_eq!(cnm2.get_next_nonce(&provider, address).await.unwrap(), 3);
    }
}
