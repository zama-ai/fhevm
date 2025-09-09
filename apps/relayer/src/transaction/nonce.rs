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
    pub async fn sync_nonce<P, N>(
        &self,
        provider: &P, //  + Send + Sync
        address: Address,
    ) -> TransportResult<u64>
    where
        N: Network,
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
    async fn increase_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<()>
    where
        N: Network,
        P: Provider<N> + ?Sized;

    async fn decrease_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<()>
    where
        N: Network,
        P: Provider<N> + ?Sized;

    async fn current_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        N: Network,
        P: Provider<N> + ?Sized;
}

// TODO: define behavior when nonce isn't set
// TODO: add tests for it
impl DebugNonceManager for CachedNonceManagerWithRefresh {
    async fn current_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        N: Network,
        P: Provider<N> + ?Sized,
    {
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

    async fn increase_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<()>
    where
        N: Network,
        P: Provider<N> + ?Sized,
    {
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

    async fn decrease_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<()>
    where
        N: Network,
        P: Provider<N> + ?Sized,
    {
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
