//! Ethereum JSON-RPC API trait definition
//!
//! This module defines the core Ethereum JSON-RPC API interface using JsonRPSee
//! proc macros. The trait provides clean separation between the API contract
//! and its implementation.

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde_json::Value;

/// Ethereum JSON-RPC API trait using JsonRPSee proc macros
///
/// This trait defines the complete Ethereum JSON-RPC interface with proper
/// JsonRPSee annotations for automatic request/response handling. All methods
/// follow the standard Ethereum JSON-RPC specification.
#[rpc(server)]
pub trait EthRpcApi {
    /// Returns the chain ID of the current network
    #[method(name = "eth_chainId")]
    async fn chain_id(&self) -> RpcResult<String>;

    /// Returns the current block number
    #[method(name = "eth_blockNumber")]
    async fn block_number(&self) -> RpcResult<String>;

    /// Returns the balance of the account at the given address
    #[method(name = "eth_getBalance")]
    async fn get_balance(&self, address: String, block: Option<String>) -> RpcResult<String>;

    /// Returns the transaction count (nonce) for the given address
    #[method(name = "eth_getTransactionCount")]
    async fn get_transaction_count(
        &self,
        address: String,
        block: Option<String>,
    ) -> RpcResult<String>;

    /// Returns the code at the given address
    #[method(name = "eth_getCode")]
    async fn get_code(&self, address: String, block: Option<String>) -> RpcResult<String>;

    /// Returns the storage value at the given address and position
    #[method(name = "eth_getStorageAt")]
    async fn get_storage_at(
        &self,
        address: String,
        position: String,
        block: Option<String>,
    ) -> RpcResult<String>;

    /// Returns the transaction receipt for the given transaction hash
    #[method(name = "eth_getTransactionReceipt")]
    async fn get_transaction_receipt(&self, hash: String) -> RpcResult<Option<Value>>;

    /// Estimates the gas required for a transaction
    #[method(name = "eth_estimateGas")]
    async fn estimate_gas(&self, tx: Value, block: Option<String>) -> RpcResult<String>;

    /// Returns the current gas price
    #[method(name = "eth_gasPrice")]
    async fn gas_price(&self) -> RpcResult<String>;

    /// Returns the current max priority fee per gas
    #[method(name = "eth_maxPriorityFeePerGas")]
    async fn max_priority_fee_per_gas(&self) -> RpcResult<String>;

    /// Returns fee history for the given block range
    #[method(name = "eth_feeHistory")]
    async fn fee_history(
        &self,
        block_count: String,
        newest_block: String,
        reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<Value>;

    /// Returns a block by its number
    #[method(name = "eth_getBlockByNumber")]
    async fn get_block_by_number(
        &self,
        block: String,
        include_txs: bool,
    ) -> RpcResult<Option<Value>>;

    /// Returns logs matching the given filter
    /// Mock ignores any filters
    #[method(name = "eth_getLogs")]
    async fn get_logs(&self, filter: Value) -> RpcResult<Vec<Value>>;

    /// Executes a message call transaction
    #[method(name = "eth_call")]
    async fn call(&self, tx: Value, block: Option<String>) -> RpcResult<String>;

    /// Submits a raw transaction to the network
    #[method(name = "eth_sendRawTransaction")]
    async fn send_raw_transaction(&self, tx: String) -> RpcResult<String>;

    /// Submits a raw transaction to the network and returns the receipt synchronously
    #[method(name = "eth_sendRawTransactionSync")]
    async fn send_raw_transaction_sync(&self, tx: String) -> RpcResult<Value>;

    /// Subscribe to events using WebSocket
    #[subscription(name = "eth_subscribe", unsubscribe = "eth_unsubscribe", item = Value)]
    fn subscribe(&self, subscription_type: String, filter: Option<Value>);
}
