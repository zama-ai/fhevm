use alloy::{
    network::Network,
    primitives::Address,
    providers::{fillers::NonceManager, Provider},
    transports::TransportResult,
};
use async_trait::async_trait;
use dashmap::DashMap;
use futures::lock::Mutex;
use std::collections::BTreeSet;
use std::fmt;
use std::sync::Arc;
use tracing::{debug, info};

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

impl fmt::Display for AccountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccountState {{ next_nonce: {}, locked_nonces: {:?}, available_nonces: {:?} }}",
            self.next_nonce, self.locked_nonces, self.available_nonces
        )
    }
}

/// A robust, in-memory nonce manager for a scalable transaction engine.
#[derive(Debug, Default, Clone)]
pub struct NonceManagerNonOptimistic {
    accounts: DashMap<Address, Arc<Mutex<AccountState>>>,
}

impl NonceManagerNonOptimistic {
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
        let account_mutex = self.get_or_init_account_state(provider, address).await?;
        let mut state = account_mutex.lock().await;

        let nonce_to_use =
            if let Some(available_nonce) = state.available_nonces.iter().next().cloned() {
                state.available_nonces.remove(&available_nonce);
                debug!(%address, nonce = available_nonce, "Reusing available nonce");
                available_nonce
            } else {
                let next = state.next_nonce;
                state.next_nonce += 1;
                debug!(%address, nonce = next, "Using next sequential nonce");
                next
            };

        state.locked_nonces.insert(nonce_to_use);
        debug!(
            address = address.to_string(),
            nonce = nonce_to_use.to_string(),
            "Nonce assigned"
        );
        Ok(nonce_to_use)
    }

    /// Releases a locked nonce, making it available for reuse.
    pub async fn release_nonce(&self, address: Address, nonce: u64) {
        debug!(
            nonce = nonce.to_string(),
            address = address.to_string(),
            "Releasing nonce"
        );
        if let Some(account_mutex) = self.accounts.get(&address) {
            let mut state = account_mutex.lock().await;
            if state.locked_nonces.remove(&nonce) {
                state.available_nonces.insert(nonce);
            }
        }
    }

    /// Confirms a nonce has been used on-chain, removing it permanently.
    pub async fn confirm_nonce(&self, address: Address, nonce: u64) {
        debug!(
            nonce = nonce.to_string(),
            address = address.to_string(),
            "Confirming nonce"
        );
        if let Some(account_mutex) = self.accounts.get(&address) {
            account_mutex.lock().await.locked_nonces.remove(&nonce);
        }
    }

    /// Returns a snapshot of the current nonce state for a given address.
    pub async fn get_account_details(&self, address: Address) -> Option<AccountState> {
        if let Some(entry) = self.accounts.get(&address) {
            Some(entry.value().lock().await.clone())
        } else {
            None
        }
    }

    /// Helper to retrieve or initialize the `AccountState` for an address.
    /// This implementation is ATOMIC and solves the concurrency race condition.
    async fn get_or_init_account_state<P, N>(
        &self,
        provider: &P,
        address: Address,
    ) -> TransportResult<Arc<Mutex<AccountState>>>
    where
        P: Provider<N>,
        N: Network,
    {
        // Use the atomic `entry` API from DashMap.
        if let Some(entry) = self.accounts.get(&address) {
            return Ok(Arc::clone(entry.value()));
        }

        let initial_nonce = provider.get_transaction_count(address).await?;

        // Use `entry` again, which provides an atomic way to get or insert.
        let account_mutex = self.accounts.entry(address).or_insert_with(|| {
            info!(
                address = address.to_string(),
                nonce = initial_nonce.to_string(),
                "Initial nonce"
            );
            let initial_state = AccountState {
                next_nonce: initial_nonce,
                ..Default::default()
            };
            Arc::new(Mutex::new(initial_state))
        });

        // Return a clone of the Arc. If another task inserted it in the meantime,
        // `or_insert_with` does nothing and just returns the existing value.
        Ok(Arc::clone(account_mutex.value()))
    }
}

// ================================================================================= //
//                            ALLOY NONCE MANAGER TRAIT                              //
// ================================================================================= //

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl NonceManager for NonceManagerNonOptimistic {
    /// Implements the `NonceManager` trait for seamless integration with Alloy's provider stack.
    /// This method directly calls our core `get_increase_and_lock_nonce` logic.
    async fn get_next_nonce<P, N>(&self, provider: &P, address: Address) -> TransportResult<u64>
    where
        P: Provider<N>,
        N: Network,
    {
        self.get_increase_and_lock_nonce(provider, address).await
    }
}

// ================================================================================= //
//                                    TEST SUITE                                     //
// ================================================================================= //

// TODO: Rework unit test for getting the rpc url correctly.
// Commenting for now passing node rpc url cloudflare failure.
/*
#[cfg(test)]
mod tests {
    // use crate::config::RPC_URL;

    use super::*;
    use alloy::{network::AnyNetwork, providers::ProviderBuilder};
    use std::{collections::HashSet, str::FromStr};
    use tokio::time::{sleep, Duration};

    // --- Test Setup ---

    // This helper connects to your running Anvil or Geth dev node.
    // Ensure the URL is set in your environment (e.g., in a .env file loaded by your test runner).
    async fn get_live_provider() -> impl Provider<AnyNetwork> {
        // let url = RPC_URL.to_string();
        let url = "https://ethereum-sepolia-rpc.publicnode.com".to_string();
        ProviderBuilder::new()
            .network::<AnyNetwork>()
            .connect_http(url.parse().unwrap())
    }

    // Use a pre-funded account with already made transactions.
    fn get_test_address() -> Address {
        Address::from_str("0x65AEa1cb6feAa8d3a1939f612CBEEEe9baad5fB8").unwrap()
    }

    // --- Test Scenarios ---

    #[tokio::test]
    async fn test_initialization_matches_live_chain_nonce() {
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        let address = get_test_address();

        let on_chain_nonce = provider.get_transaction_count(address).await.unwrap();

        // Trigger initialization by getting the first nonce
        let first_nonce = manager.get_next_nonce(&provider, address).await.unwrap();

        assert_eq!(
            first_nonce, on_chain_nonce,
            "First fetched nonce should match the on-chain nonce"
        );

        let details = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details.next_nonce,
            on_chain_nonce + 1,
            "High-water mark should be incremented"
        );
        assert!(details.locked_nonces.contains(&on_chain_nonce));
    }

    #[tokio::test]
    async fn test_sequential_nonces_are_dispensed_correctly() {
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        // let address = get_test_address();

        // This test requires a clean state, so we use a fresh, unfunded address.
        // The nonce should always start at 0.
        let fresh_address =
            Address::from_str("0x1234567890123456789012345678901234567890").unwrap();

        let nonce0 = manager
            .get_next_nonce(&provider, fresh_address)
            .await
            .unwrap();
        assert_eq!(nonce0, 0);

        let nonce1 = manager
            .get_next_nonce(&provider, fresh_address)
            .await
            .unwrap();
        assert_eq!(nonce1, 1);

        let nonce2 = manager
            .get_next_nonce(&provider, fresh_address)
            .await
            .unwrap();
        assert_eq!(nonce2, 2);

        let details = manager.get_account_details(fresh_address).await.unwrap();
        assert_eq!(details.next_nonce, 3);
        assert_eq!(details.locked_nonces, BTreeSet::from([0, 1, 2]));
        assert!(details.available_nonces.is_empty());
    }

    #[tokio::test]
    async fn test_get_next_nonce_prioritizes_available_gaps_over_sequential() {
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        let address = get_test_address();

        // Manually set up a state with a high `next_nonce` and some available gaps.
        let initial_state = AccountState {
            next_nonce: 100,
            locked_nonces: BTreeSet::new(),
            available_nonces: BTreeSet::from([5, 2, 8]), // Intentionally unsorted
        };
        manager
            .accounts
            .insert(address, Arc::new(Mutex::new(initial_state)));

        // The manager should dispense the LOWEST available nonces first.
        assert_eq!(manager.get_next_nonce(&provider, address).await.unwrap(), 2);
        assert_eq!(manager.get_next_nonce(&provider, address).await.unwrap(), 5);
        assert_eq!(manager.get_next_nonce(&provider, address).await.unwrap(), 8);

        // Now that the available pool is empty, it should use the high-water mark.
        assert_eq!(
            manager.get_next_nonce(&provider, address).await.unwrap(),
            100
        );

        let details = manager.get_account_details(address).await.unwrap();
        assert!(details.available_nonces.is_empty());
        assert_eq!(details.locked_nonces, BTreeSet::from([2, 5, 8, 100]));
        assert_eq!(details.next_nonce, 101);
    }

    // ADDITIONAL SCENARIOS:

    #[tokio::test]
    async fn scenario_initialization_and_sequential_dispatch() {
        // This test simulates the "happy path": a new account comes online
        // and requests several nonces in a row without any failures.
        println!("SCENARIO: Initialization and Sequential Dispatch");

        // 1. Arrange
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        // Use a fresh address to guarantee the nonce starts from 0.
        let address = Address::from_str("0x1111111111111111111111111111111111111111").unwrap();

        // 2. Act & Assert
        // First request should initialize by calling the provider, getting nonce 0.
        let nonce0 = manager.get_next_nonce(&provider, address).await.unwrap();
        assert_eq!(nonce0, 0, "First nonce should be 0 for a new account");

        // Subsequent requests should use the internal high-water mark.
        let nonce1 = manager.get_next_nonce(&provider, address).await.unwrap();
        assert_eq!(nonce1, 1, "Second nonce should be 1");

        let nonce2 = manager.get_next_nonce(&provider, address).await.unwrap();
        assert_eq!(nonce2, 2, "Third nonce should be 2");

        // 3. Final State Verification
        let details = manager.get_account_details(address).await.unwrap();
        assert_eq!(details.next_nonce, 3, "High-water mark should now be 3");
        assert_eq!(
            details.locked_nonces,
            BTreeSet::from([0, 1, 2]),
            "All dispatched nonces should be locked"
        );
        assert!(
            details.available_nonces.is_empty(),
            "Available pool should be empty"
        );
        // Validating transaction:
        manager.confirm_nonce(address, 0).await;
        manager.confirm_nonce(address, 1).await;
        let details1 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details1.locked_nonces,
            BTreeSet::from([2]),
            "Only the transaction with nonce 2 is still pending."
        );

        manager.confirm_nonce(address, 2).await;
        let details2 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details2.locked_nonces,
            BTreeSet::from([]),
            "All locked nonce should be released."
        );
        assert_eq!(
            details2.next_nonce, 3,
            "High-water mark should now still be 3"
        );
    }

    #[tokio::test]
    async fn scenario_stuck_transaction_is_released_and_reused() {
        // This simulates a core relayer function: a transaction gets stuck,
        // the engine releases its nonce, and a new transaction reuses that nonce to fill the gap.
        println!("SCENARIO: Stuck Transaction is Released and Reused");

        // 1. Arrange: Dispatch 3 nonces (0, 1, 2)
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        let address = Address::from_str("0x2222222222222222222222222222222222222222").unwrap();
        manager.get_next_nonce(&provider, address).await.unwrap(); // Nonce 0
        manager.get_next_nonce(&provider, address).await.unwrap(); // Nonce 1
        manager.get_next_nonce(&provider, address).await.unwrap(); // Nonce 2

        // 2. Act: Simulate nonce 1 getting stuck and being released.
        println!("Releasing stuck nonce 1...");
        manager.release_nonce(address, 1).await;

        // 3. Assert Intermediate State
        let details1 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details1.locked_nonces,
            BTreeSet::from([0, 2]),
            "Nonces 0 and 2 should remain locked"
        );
        assert_eq!(
            details1.available_nonces,
            BTreeSet::from([1]),
            "Nonce 1 should now be available"
        );

        // 4. Act: Request a new nonce.
        println!("Requesting a new nonce, expecting it to be the released one...");
        let reused_nonce = manager.get_next_nonce(&provider, address).await.unwrap();

        // 5. Assert Final State
        assert_eq!(
            reused_nonce, 1,
            "The manager MUST reuse the released nonce 1 to fill the gap"
        );
        let details2 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details2.locked_nonces,
            BTreeSet::from([0, 1, 2]),
            "All nonces are now locked again"
        );
        assert!(
            details2.available_nonces.is_empty(),
            "Available pool should be empty after reuse"
        );
    }

    #[tokio::test]
    async fn scenario_concurrent_requests_for_same_address_are_safe() {
        // This is the most critical test. It proves that the manager is thread-safe
        // by simulating many concurrent requests for nonces for the SAME address.
        // It must dispense unique, sequential nonces with no duplicates.
        println!("SCENARIO: Concurrent Requests are Safe");

        // 1. Arrange
        let provider = Arc::new(get_live_provider().await);
        let manager = Arc::new(NonceManagerNonOptimistic::new());
        let address = Address::from_str("0x3333333333333333333333333333333333333333").unwrap();

        let initial_nonce = provider.get_transaction_count(address).await.unwrap();
        let num_tasks = 20;
        let mut tasks = Vec::new();

        // 2. Act: Spawn N tasks that all request a nonce at the same time.
        for _ in 0..num_tasks {
            let manager_clone = Arc::clone(&manager);
            let provider_clone = Arc::clone(&provider);
            tasks.push(tokio::spawn(async move {
                manager_clone
                    .get_next_nonce(&*provider_clone, address)
                    .await
            }));
        }

        let results = futures::future::join_all(tasks).await;

        // 3. Assert
        let mut received_nonces = HashSet::new();
        for res in results {
            let nonce_result = res.unwrap(); // Panics on task failure
            assert!(nonce_result.is_ok());
            let nonce = nonce_result.unwrap();
            // The core assertion: was this nonce already given to another task?
            assert!(
                received_nonces.insert(nonce),
                "FATAL: Duplicate nonce {} dispensed under contention!",
                nonce
            );
        }

        assert_eq!(
            received_nonces.len() as u64,
            num_tasks,
            "Should have received exactly {} unique nonces",
            num_tasks
        );
        println!("✅ All {} dispensed nonces were unique.", num_tasks);

        let final_details = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            final_details.next_nonce,
            initial_nonce + num_tasks,
            "High-water mark should be advanced by the number of tasks"
        );
    }

    #[tokio::test]
    async fn scenario_mixed_lifecycle_operations() {
        // This test simulates a complex, realistic sequence of events.
        println!("SCENARIO: Mixed Lifecycle Operations");

        // 1. Arrange: Get 5 nonces (0-4)
        let provider = get_live_provider().await;
        let manager = NonceManagerNonOptimistic::new();
        let address = Address::from_str("0x4444444444444444444444444444444444444444").unwrap();
        for i in 0..5 {
            assert_eq!(manager.get_next_nonce(&provider, address).await.unwrap(), i);
        }

        let details1 = manager.get_account_details(address).await.unwrap();
        assert_eq!(details1.locked_nonces, BTreeSet::from([0, 1, 2, 3, 4]));

        // 2. Act: A series of events happens out of order.
        println!("Locked nonce 2, Releasing nonce 1, Confirming nonce 0...");
        manager.release_nonce(address, 1).await; // Got stuck
        manager.confirm_nonce(address, 0).await; // Mined successfully

        // 3. Assert Intermediate State
        let details2 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            details2.locked_nonces,
            BTreeSet::from([2, 3, 4]),
            "Only nonces 2, 3 and 4 should be locked"
        );
        assert_eq!(
            details2.available_nonces,
            BTreeSet::from([1]),
            "Only nonce 1 should be available"
        );

        // 4. Act: Get two more nonces.
        println!("Requesting two more nonces...");
        let nonce_a = manager.get_next_nonce(&provider, address).await.unwrap();
        let nonce_b = manager.get_next_nonce(&provider, address).await.unwrap();

        // 5. Assert Final State
        assert_eq!(nonce_a, 1, "Should have reused the available nonce 1 first");
        assert_eq!(
            nonce_b, 5,
            "Should have used the high-water mark nonce 5 after filling the gap"
        );

        let details3 = manager.get_account_details(address).await.unwrap();
        assert_eq!(details3.locked_nonces, BTreeSet::from([1, 2, 3, 4, 5]));
        assert_eq!(details3.next_nonce, 6);

        // Then txs 1 and 2 passes ! And transaction 4 is dropped by the mempool !
        manager.confirm_nonce(address, 1).await; // Mined successfully
        manager.confirm_nonce(address, 2).await; // Mined successfully
        manager.release_nonce(address, 4).await;

        let details4 = manager.get_account_details(address).await.unwrap();

        assert_eq!(details4.locked_nonces, BTreeSet::from([3, 5]));

        let released = manager.get_next_nonce(&provider, address).await.unwrap();
        let details5 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            released, 4,
            "Should have reused the available nonce 1 first"
        );
        assert_eq!(details5.locked_nonces, BTreeSet::from([3, 4, 5]));
        assert_eq!(details5.next_nonce, 6);
    }

    #[tokio::test]
    async fn scenario_concurrent_requests_with_mixed_lifecycle() {
        println!("SCENARIO: Concurrent Requests with Mixed Lifecycle ('Chaos Test')");

        // 1. Arrange
        let provider = Arc::new(get_live_provider().await);
        let manager = Arc::new(NonceManagerNonOptimistic::new());
        // Use a fresh address for a predictable state.
        let address = Address::from_str("0x5555555555555555555555555555555555555555").unwrap();

        // Pre-warm the manager: get the first 10 nonces (0-9) and lock them.
        for i in 0..10 {
            assert_eq!(
                manager.get_next_nonce(&*provider, address).await.unwrap(),
                i
            );
        }

        let mut tasks = Vec::new();

        // - Release nonce 2 and 5 (simulating stuck txs)
        let manager_clone = Arc::clone(&manager);
        tasks.push(tokio::spawn(async move {
            println!("Task A: Releasing nonces 2 and 5");
            manager_clone.release_nonce(address, 2).await;
            sleep(Duration::from_millis(10)).await; // small delay to allow interleaving
            manager_clone.release_nonce(address, 5).await;
        }));

        // - Confirm nonce 3 and 7 (simulating mined txs)
        let manager_clone = Arc::clone(&manager);
        tasks.push(tokio::spawn(async move {
            println!("Task B: Confirming nonces 3 and 7");
            manager_clone.confirm_nonce(address, 3).await;
            sleep(Duration::from_millis(5)).await;
            manager_clone.confirm_nonce(address, 7).await;
        }));

        // - Request 5 new nonces
        let mut nonce_requester_tasks = Vec::new();
        for i in 0..5 {
            let manager_clone = Arc::clone(&manager);
            let provider_clone = Arc::clone(&provider);
            nonce_requester_tasks.push(tokio::spawn(async move {
                let nonce = manager_clone
                    .get_next_nonce(&*provider_clone, address)
                    .await
                    .unwrap();
                println!("Task C.{}: Requested and received nonce {}", i, nonce);
                nonce
            }));
        }

        // Wait for the release/confirm tasks to finish first.
        futures::future::join_all(tasks).await;
        // Then get the results from the nonce requesters.
        futures::future::join_all(nonce_requester_tasks).await;
        let state1 = manager.get_account_details(address).await.unwrap();
        assert_eq!(
            state1.locked_nonces,
            BTreeSet::from([0, 1, 2, 4, 6, 8, 9, 10, 11, 12, 13])
        );
        // Because of milliseconds waiting before release.
        assert_eq!(state1.available_nonces, BTreeSet::from([5]));
        assert_eq!(state1.next_nonce, 14);
        manager.get_next_nonce(&provider, address).await.unwrap();
        let state2 = manager.get_account_details(address).await.unwrap();

        assert_eq!(
            state2.locked_nonces,
            BTreeSet::from([0, 1, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13])
        );
        // Because of milliseconds waiting before release.
        assert_eq!(state2.available_nonces, BTreeSet::from([]));
        assert_eq!(state2.next_nonce, 14);
    }
}
*/
