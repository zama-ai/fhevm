//! Blockchain state management for the mock server
//!
//! This module provides thread-safe blockchain state management including:
//! - Account management (balance, nonce, code, storage)
//! - Transaction storage and retrieval
//! - Block progression for subscription events
//! - Transaction validation and decoding
//! - Receipt generation and storage

use crate::mock_server::SubscriptionTarget;
use alloy::consensus::{Header as ConsensusHeader, Receipt, ReceiptEnvelope, ReceiptWithBloom};
use alloy::primitives::{Address, Bytes, Log as InnerLog, B256, U256};
use indexmap::IndexMap;

/// Mock-specific constants for deterministic testing
mod mock_constants {
    /// Standard gas limit for mock blocks (15M gas)
    pub const MOCK_GAS_LIMIT: u64 = 15_000_000;

    /// Standard base fee for mock transactions (20 Gwei)
    pub const MOCK_BASE_FEE_PER_GAS: u64 = 20_000_000_000;

    /// Standard gas used for simple mock transactions (21k gas)
    pub const MOCK_STANDARD_GAS_USED: u64 = 21_000;

    /// Standard effective gas price for mock transactions (20 Gwei)
    pub const MOCK_EFFECTIVE_GAS_PRICE: u128 = 20_000_000_000;
}
use alloy::rpc::types::eth::{Block, Header, Log as RpcLog, TransactionReceipt};
use anyhow::{Context, Result as AnyhowResult};
use jsonrpsee::types::SubscriptionId;
use jsonrpsee::{SubscriptionMessage, SubscriptionSink};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::RwLock as AsyncRwLock;
use tracing::{debug, error, info};

/// Account state representation
#[derive(Debug, Clone)]
pub struct Account {
    /// Account balance in wei
    pub balance: U256,
    /// Transaction nonce
    pub nonce: u64,
    /// Contract bytecode (empty for EOAs)
    pub code: Bytes,
    /// Contract storage mapping
    pub storage: HashMap<U256, U256>,
}

impl Account {
    /// Create a new account with specified balance
    pub fn new(balance: U256) -> Self {
        Self {
            balance,
            nonce: 0,
            code: Bytes::new(),
            storage: HashMap::new(),
        }
    }

    /// Check if this is a contract account
    pub fn is_contract(&self) -> bool {
        !self.code.is_empty()
    }
}

impl Default for Account {
    fn default() -> Self {
        Self {
            balance: U256::ZERO,
            nonce: 0,
            code: Bytes::new(),
            storage: HashMap::new(),
        }
    }
}

/// Represents a transaction that should be executed at a later time
#[derive(Debug, Clone)]
pub struct ScheduledTransaction {
    /// Target contract address (None for contract creation)
    pub target_address: Option<Address>,
    /// Response events to emit with their individual delays and targets
    pub response_events: Vec<(Duration, InnerLog, SubscriptionTarget)>,
}

impl ScheduledTransaction {
    /// Create scheduled transaction with single event (most common case)
    pub fn with_single_event(
        delay: Duration,
        target_address: Option<Address>,
        event: InnerLog,
    ) -> Self {
        Self {
            target_address,
            response_events: vec![(delay, event, SubscriptionTarget::All)],
        }
    }

    /// Create scheduled transaction with multiple events (new multi-response capability)
    pub fn with_multiple_events(
        target_address: Option<Address>,
        events: Vec<(Duration, InnerLog)>,
    ) -> Self {
        debug_assert!(
            !events.is_empty(),
            "ScheduledTransaction requires at least one event"
        );
        Self {
            target_address,
            response_events: events
                .into_iter()
                .map(|(d, e)| (d, e, SubscriptionTarget::All))
                .collect(),
        }
    }
}

/// Internal runtime state for the blockchain
#[derive(Debug)]
struct RuntimeState {
    /// Current block number
    current_block: u64,
    /// Account storage with addresses mapped to account state
    accounts: HashMap<Address, Account>,
    /// All logs for log filtering
    all_logs: Vec<InnerLog>,
    /// Transaction receipts storage
    transaction_receipts: HashMap<B256, TransactionReceipt>,
}

impl RuntimeState {
    fn new(initial_accounts: HashMap<Address, Account>) -> Self {
        Self {
            current_block: 1,
            accounts: initial_accounts,
            all_logs: Vec::new(),
            transaction_receipts: HashMap::new(),
        }
    }
}

/// Thread-safe blockchain state management
#[derive(Debug, Clone)]
pub struct BlockchainState {
    state: Arc<RwLock<RuntimeState>>,
}

impl BlockchainState {
    pub fn new(initial_accounts: HashMap<Address, Account>) -> Self {
        Self {
            state: Arc::new(RwLock::new(RuntimeState::new(initial_accounts))),
        }
    }

    pub fn get_balance(&self, address: Address) -> U256 {
        let state = self.state.read().unwrap();
        state
            .accounts
            .get(&address)
            .map(|acc| acc.balance)
            .unwrap_or(U256::ZERO)
    }

    pub fn set_balance(&self, address: Address, amount: U256) {
        let mut state = self.state.write().unwrap();
        state.accounts.entry(address).or_default().balance = amount;
    }

    pub fn get_nonce(&self, address: Address) -> u64 {
        let state = self.state.read().unwrap();
        state
            .accounts
            .get(&address)
            .map(|acc| acc.nonce)
            .unwrap_or(0)
    }

    pub fn set_nonce(&self, address: Address, nonce: u64) {
        let mut state = self.state.write().unwrap();
        state.accounts.entry(address).or_default().nonce = nonce;
    }

    pub fn get_code(&self, address: Address) -> Bytes {
        let state = self.state.read().unwrap();
        state
            .accounts
            .get(&address)
            .map(|acc| acc.code.clone())
            .unwrap_or_default()
    }

    pub fn set_code(&self, address: Address, code: Bytes) {
        let mut state = self.state.write().unwrap();
        state.accounts.entry(address).or_default().code = code;
    }

    pub fn get_storage(&self, address: Address, slot: U256) -> U256 {
        let state = self.state.read().unwrap();
        state
            .accounts
            .get(&address)
            .and_then(|acc| acc.storage.get(&slot))
            .copied()
            .unwrap_or(U256::ZERO)
    }

    pub fn set_storage(&self, address: Address, slot: U256, value: U256) {
        let mut state = self.state.write().unwrap();
        state
            .accounts
            .entry(address)
            .or_default()
            .storage
            .insert(slot, value);
    }

    pub fn get_current_block(&self) -> u64 {
        let state = self.state.read().unwrap();
        state.current_block
    }

    pub fn increment_block(&self) -> u64 {
        let mut state = self.state.write().unwrap();
        state.current_block += 1;
        state.current_block
    }

    pub fn store_transaction_receipt(&self, hash: B256, receipt: TransactionReceipt) {
        let mut state = self.state.write().unwrap();
        state.transaction_receipts.insert(hash, receipt);
    }

    pub fn get_transaction_receipt(&self, hash: B256) -> Option<TransactionReceipt> {
        let state = self.state.read().unwrap();
        state.transaction_receipts.get(&hash).cloned()
    }

    /// Get all stored logs
    pub fn get_all_logs(&self) -> Vec<InnerLog> {
        let state = self.state.read().unwrap();
        state.all_logs.clone()
    }

    pub fn reset(&self, initial_accounts: HashMap<Address, Account>) {
        let mut state = self.state.write().unwrap();
        *state = RuntimeState::new(initial_accounts);
    }

    /// Generate a random hash for blocks
    pub fn generate_hash(&self) -> B256 {
        Self::generate_random_hash()
    }

    /// Generate a random hash (static method for use across modules)
    pub fn generate_random_hash() -> B256 {
        use rand::RngExt;
        B256::from(rand::rng().random::<[u8; 32]>())
    }

    /// Convert inner log to RPC log with proper metadata for WebSocket subscribers
    pub fn convert_inner_log_to_rpc(&self, inner_log: InnerLog) -> RpcLog {
        RpcLog {
            inner: inner_log,
            block_hash: Some(Self::generate_random_hash()),
            block_number: Some(self.get_current_block()),
            block_timestamp: None,
            transaction_hash: Some(Self::generate_random_hash()),
            transaction_index: Some(0),
            log_index: Some(0),
            removed: false,
        }
    }

    /// Schedule a delayed transaction for later execution
    ///
    /// This method preserves the exact async pattern from TransactionScheduler for safety.
    /// It spawns one tokio task per event with individual delays, then emits the response
    /// event and increments the block number.
    pub fn schedule_delayed_transaction(
        &self,
        transaction: ScheduledTransaction,
        log_subscriptions: Arc<AsyncRwLock<IndexMap<SubscriptionId<'static>, SubscriptionSink>>>,
    ) {
        let num_events = transaction.response_events.len();
        let blockchain_state = self.clone(); // Clone Arc wrapper (expert recommendation)

        debug!(
            num_response_events = num_events,
            "Scheduling delayed transaction via BlockchainState"
        );

        // Spawn one tokio task per event with individual delays.
        // Each task runs independently - subscription targeting is evaluated at emission time.
        for (delay, event, target) in transaction.response_events {
            let log_subscriptions = log_subscriptions.clone();
            let blockchain_state = blockchain_state.clone();

            debug!(
                delay_ms = delay.as_millis(),
                "Scheduling individual event with delay"
            );

            // Preserve exact same async task pattern from TransactionScheduler
            tokio::spawn(async move {
                tokio::time::sleep(delay).await;

                debug!("Emitting scheduled transaction event from BlockchainState");
                if let Err(e) = Self::emit_to_log_subscribers_static(
                    &log_subscriptions,
                    &blockchain_state,
                    event,
                    target,
                )
                .await
                {
                    error!(error = %e, "Failed to emit scheduled event from BlockchainState");
                }

                blockchain_state.increment_block();
                info!("Executed scheduled transaction event via BlockchainState");
            });
        }
    }

    /// Static helper for event emission in async tasks (preserves exact behavior)
    async fn emit_to_log_subscribers_static(
        log_subscriptions: &Arc<AsyncRwLock<IndexMap<SubscriptionId<'static>, SubscriptionSink>>>,
        blockchain_state: &BlockchainState,
        log: InnerLog,
        target: SubscriptionTarget,
    ) -> AnyhowResult<()> {
        // Convert inner log to RPC log with proper metadata
        let rpc_log = blockchain_state.convert_inner_log_to_rpc(log);

        // Send the RPC log object - JsonRPSee will wrap it with subscription ID
        let log_value = serde_json::to_value(rpc_log).context("Failed to serialize log to JSON")?;

        // Use async write lock to send immediately within the lock
        let mut subs = log_subscriptions.write().await;

        // Determine which indices to send to
        let indices_to_send: Vec<usize> = match target {
            SubscriptionTarget::All => (0..subs.len()).collect(),
            SubscriptionTarget::Only(indices) => indices,
        };

        debug!(
            "Broadcasting scheduled log event to {} of {} subscribers via BlockchainState",
            indices_to_send.len(),
            subs.len()
        );

        let mut dead_subscription_ids = Vec::new();

        // Send to targeted subscribers within the lock.
        // Note: get_index returns None for out-of-bounds indices (safe handling).
        for index in indices_to_send {
            if let Some((subscription_id, sink)) = subs.get_index(index) {
                let message = SubscriptionMessage::new(
                    "eth_subscription",
                    subscription_id.clone(),
                    &log_value,
                )
                .context("Failed to create subscription message")?;
                if let Err(err) = sink.send(message).await {
                    debug!(
                        "Failed to send scheduled log event to subscriber {:?}: {}. Will remove dead sink.",
                        subscription_id, err
                    );
                    dead_subscription_ids.push(subscription_id.clone());
                }
            }
        }

        // Clean up dead subscriptions (happens AFTER all sends complete, so indices were stable above)
        for subscription_id in dead_subscription_ids.iter() {
            // shift_remove maintains insertion order (unlike swap_remove).
            // Note: indices of subsequent subscriptions will shift, but this is expected -
            // future emissions will target "whatever subscription is at index N at that time".
            subs.shift_remove(subscription_id);
            debug!("Removed dead log subscriber with ID {:?}", subscription_id);
        }

        if !dead_subscription_ids.is_empty() {
            debug!(
                "Cleaned up {} dead log subscribers. Remaining: {}",
                dead_subscription_ids.len(),
                subs.len()
            );
        }

        Ok(())
    }

    /// Build a structured transaction receipt from logs and hash
    pub fn build_receipt(&self, logs: &[InnerLog], tx_hash: B256) -> TransactionReceipt {
        let rpc_logs = self.convert_inner_logs_to_rpc(logs, tx_hash);

        // Create the inner receipt envelope
        let receipt = Receipt {
            status: true.into(), // Mock-specific: Always success for testing
            cumulative_gas_used: self.calculate_cumulative_gas(rpc_logs.len()),
            logs: rpc_logs.clone(),
        };

        let receipt_with_bloom = ReceiptWithBloom {
            receipt,
            logs_bloom: Default::default(), // Empty logs bloom
        };

        TransactionReceipt {
            // Essential mock fields for proper functionality
            inner: ReceiptEnvelope::Eip1559(receipt_with_bloom),
            transaction_hash: tx_hash,
            block_hash: Some(Self::generate_random_hash()),
            block_number: Some(self.get_current_block()),
            gas_used: mock_constants::MOCK_STANDARD_GAS_USED, // Mock-specific: Standard transfer gas for predictable testing
            effective_gas_price: mock_constants::MOCK_EFFECTIVE_GAS_PRICE, // Mock-specific: 20 Gwei for predictable testing
            // Use sensible defaults for other fields
            transaction_index: Some(0),
            from: Address::ZERO,
            to: None,
            contract_address: None,
            blob_gas_price: None,
            blob_gas_used: None,
        }
    }

    /// Build a JSON transaction receipt from logs and hash
    pub fn build_receipt_json(&self, logs: &[InnerLog], tx_hash: B256) -> Value {
        let receipt = self.build_receipt(logs, tx_hash);
        serde_json::to_value(receipt).unwrap_or(serde_json::Value::Null)
    }

    /// Convert inner logs to RPC logs with proper metadata
    pub fn convert_inner_logs_to_rpc(&self, inner_logs: &[InnerLog], tx_hash: B256) -> Vec<RpcLog> {
        inner_logs
            .iter()
            .enumerate()
            .map(|(log_index, log)| RpcLog {
                // Use alloy defaults, override only mock-essential metadata
                inner: log.clone(),
                block_hash: Some(Self::generate_random_hash()),
                block_number: Some(self.get_current_block()),
                transaction_hash: Some(tx_hash),
                log_index: Some(log_index as u64),
                ..Default::default() // Alloy defaults for other RpcLog fields
            })
            .collect()
    }

    /// Create and store a complete TransactionReceipt
    pub fn create_and_store_receipt(
        &self,
        tx_hash: B256,
        logs: &[InnerLog],
        to_address: Option<Address>,
    ) -> TransactionReceipt {
        let rpc_logs = self.convert_inner_logs_to_rpc(logs, tx_hash);
        let inner_receipt = Receipt {
            status: true.into(),
            cumulative_gas_used: 21000,
            logs: rpc_logs,
        };

        let receipt_with_bloom = ReceiptWithBloom {
            receipt: inner_receipt,
            logs_bloom: Default::default(),
        };

        let receipt = TransactionReceipt {
            inner: ReceiptEnvelope::Eip1559(receipt_with_bloom),
            transaction_hash: tx_hash,
            transaction_index: Some(0),
            block_hash: Some(self.generate_hash()),
            block_number: Some(self.get_current_block()),
            from: Address::ZERO, // Use dummy address for simplified mock
            to: to_address,
            gas_used: 21000,
            contract_address: None,
            effective_gas_price: 20_000_000_000u128,
            blob_gas_price: None,
            blob_gas_used: None,
        };

        // Store receipt for eth_getTransactionReceipt
        self.store_transaction_receipt(tx_hash, receipt.clone());

        receipt
    }

    /// Calculate cumulative gas used based on number of logs
    fn calculate_cumulative_gas(&self, num_logs: usize) -> u64 {
        mock_constants::MOCK_STANDARD_GAS_USED + (num_logs as u64 * 375) // Base gas + log gas
    }

    /// Create block JSON data for get_block_by_number RPC method
    /// Create a structured block using alloy's Block type
    pub fn create_block(&self, block_number: u64) -> Block {
        let consensus_header = ConsensusHeader {
            // Use alloy defaults for most fields, override only mock-essential ones
            parent_hash: self.generate_hash(),
            number: block_number,
            timestamp: self.current_timestamp(),
            gas_limit: mock_constants::MOCK_GAS_LIMIT, // Mock-specific: 15M gas limit for predictable testing
            base_fee_per_gas: Some(mock_constants::MOCK_BASE_FEE_PER_GAS), // Mock-specific: 20 Gwei for predictable testing
            ..Default::default() // Let alloy provide sensible defaults for all other fields
        };

        let header = Header {
            inner: consensus_header,
            hash: self.generate_hash(),
            size: Some(U256::from(544)), // 0x220 in decimal
            total_difficulty: Some(U256::ZERO),
        };

        Block {
            header,
            transactions: alloy::rpc::types::eth::BlockTransactions::Hashes(vec![]), // Empty for simplicity
            uncles: vec![],
            withdrawals: None,
        }
    }

    /// Create block JSON
    pub fn create_block_json(&self, block_number: u64) -> Value {
        let block = self.create_block(block_number);
        serde_json::to_value(block).unwrap_or(serde_json::Value::Null)
    }

    /// Create a block header for WebSocket head subscriptions
    pub fn create_block_header(&self, block_number: u64) -> Block {
        let consensus_header = self.create_mock_consensus_header(block_number);

        let header = Header {
            inner: consensus_header,
            hash: self.generate_hash(),
            size: Some(U256::from(1000)),
            total_difficulty: Some(U256::from(1000)),
        };

        Block {
            header,
            transactions: alloy::rpc::types::eth::BlockTransactions::Hashes(vec![]),
            uncles: vec![],
            withdrawals: None,
        }
    }

    /// Create a mock consensus header with essential fields filled
    fn create_mock_consensus_header(&self, block_number: u64) -> ConsensusHeader {
        ConsensusHeader {
            // Use alloy defaults, override only mock-essential fields
            parent_hash: self.generate_hash(),
            number: block_number,
            timestamp: self.current_timestamp(),
            gas_limit: mock_constants::MOCK_GAS_LIMIT, // Mock-specific gas limit
            base_fee_per_gas: Some(mock_constants::MOCK_BASE_FEE_PER_GAS), // Mock-specific base fee
            ..Default::default() // Alloy defaults for all other standard fields
        }
    }

    /// Get current timestamp as seconds since Unix epoch
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}
