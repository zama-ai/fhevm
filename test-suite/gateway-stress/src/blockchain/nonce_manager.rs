use alloy::{
    network::Network,
    primitives::Address,
    providers::{Provider, fillers::NonceManager},
    transports::TransportResult,
};
use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use std::collections::{BTreeSet, HashMap, hash_map::Entry};
use std::sync::Arc;
use tracing::trace;

/// A robust, in-memory nonce manager for a scalable transaction engine.
#[derive(Clone, Debug, Default)]
pub struct ZamaNonceManager {
    /// Nonce state for each account, shared across all tasks/threads using the nonce manager.
    accounts: Arc<Mutex<HashMap<Address, AccountState>>>,
}

/// Represents the complete nonce state for a single account.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccountState {
    /// The "high-water mark" nonce. Used only when no gaps are available.
    pub next_nonce: u64,
    /// Nonces that have been dispatched but not yet confirmed or rejected.
    pub locked_nonces: BTreeSet<u64>,
    /// Nonces that were previously locked but have been released, creating gaps.
    pub available_nonces: BTreeSet<u64>,
}

impl ZamaNonceManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// The primary logic for acquiring and locking the next valid nonce.
    ///
    /// The logic prioritizes filling gaps from `available_nonces` before
    /// incrementing the main `next_nonce` counter.
    pub async fn get_increase_and_lock_nonce<P, N>(
        &self,
        provider: &P,
        address: Address,
    ) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        let mut accounts_guard = self.accounts.lock().await;
        let account =
            Self::get_or_init_account_state(&mut accounts_guard, provider, address).await?;
        let nonce_to_use =
            if let Some(available_nonce) = account.available_nonces.iter().next().copied() {
                account.available_nonces.remove(&available_nonce);
                trace!(%address, nonce = available_nonce, "Reusing available nonce");
                available_nonce
            } else {
                let next = account.next_nonce;
                account.next_nonce += 1;
                trace!(%address, nonce = next, "Using next sequential nonce");
                next
            };

        account.locked_nonces.insert(nonce_to_use);
        Ok(nonce_to_use)
    }

    /// Releases a locked nonce, making it available for reuse.
    pub async fn release_nonce(&self, address: Address, nonce: u64) {
        let mut accounts = self.accounts.lock().await;
        if let Some(account) = accounts.get_mut(&address)
            && account.locked_nonces.remove(&nonce)
        {
            account.available_nonces.insert(nonce);
        }
    }

    /// Confirms a nonce has been used on-chain, removing it permanently.
    pub async fn confirm_nonce(&self, address: Address, nonce: u64) {
        let mut accounts = self.accounts.lock().await;
        if let Some(account) = accounts.get_mut(&address) {
            account.locked_nonces.remove(&nonce);
        }
    }

    /// Helper to retrieve or initialize the `AccountState` for an address.
    async fn get_or_init_account_state<'a, P, N>(
        accounts_guard: &'a mut MutexGuard<'_, HashMap<Address, AccountState>>,
        provider: &P,
        address: Address,
    ) -> TransportResult<&'a mut AccountState>
    where
        P: Provider<N>,
        N: Network,
    {
        let account = match accounts_guard.entry(address) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let initial_nonce = provider.get_transaction_count(address).await?;
                entry.insert(AccountState {
                    next_nonce: initial_nonce,
                    ..Default::default()
                })
            }
        };
        Ok(account)
    }
}

// Implements the `NonceManager` trait for seamless integration with Alloy's provider stack.
#[async_trait]
impl NonceManager for ZamaNonceManager {
    async fn get_next_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        self.get_increase_and_lock_nonce(provider, address).await
    }
}
