//! RPC method implementations and utilities
//!
//! This module contains the actual implementation logic for Ethereum JSON-RPC methods,
//! along with utilities for formatting responses and building complex data structures.

use crate::{
    blockchain::BlockchainState,
    mock_server::{
        rpc::EthRpcApiServer,
        rpc_types::{CallParams, Response, ResponseData, TxParams},
        MockConfig,
    },
    pattern_matcher::PatternMatcher,
};
use alloy::primitives::{Log as InnerLog, B256};
use alloy::rpc::types::eth::Log as RpcLog;
use anyhow::Context;
use jsonrpsee::{
    core::RpcResult,
    types::{ErrorObject, SubscriptionId},
    PendingSubscriptionSink, SubscriptionMessage, SubscriptionSink,
};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock as AsyncRwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

/// Default gas price (20 gwei in hex)
const DEFAULT_GAS_PRICE: &str = "0x4a817c800";

/// Parse block number from hex string using alloy's FromStr
fn parse_block_number(block_str: &str) -> Result<u64, String> {
    use alloy::primitives::U256;
    use std::str::FromStr;

    if block_str.starts_with("0x") {
        U256::from_str(block_str)
            .map(|u| u.to::<u64>())
            .map_err(|_| "Invalid block number format".to_string())
    } else {
        Err("Block number must be hex with 0x prefix".to_string())
    }
}

/// Container for mock RPC implementation that manages all dependencies
pub struct MockRpcHandler {
    pub config: Arc<MockConfig>,
    pub pattern_matcher: Arc<PatternMatcher>,
    pub blockchain_state: Arc<BlockchainState>,
    pub log_subscriptions: Arc<AsyncRwLock<HashMap<SubscriptionId<'static>, SubscriptionSink>>>,
    pub head_subscriptions: Arc<AsyncRwLock<HashMap<SubscriptionId<'static>, SubscriptionSink>>>,
    #[allow(dead_code)] // Reserved for future graceful shutdown implementation
    pub shutdown_token: CancellationToken,
}

#[async_trait::async_trait]
impl EthRpcApiServer for MockRpcHandler {
    async fn chain_id(&self) -> RpcResult<String> {
        Ok(format!("0x{:x}", self.config.chain_id))
    }

    async fn block_number(&self) -> RpcResult<String> {
        let current_block = self.blockchain_state.get_current_block();
        Ok(format!("0x{:x}", current_block))
    }

    async fn get_balance(&self, address: String, _block: Option<String>) -> RpcResult<String> {
        let address_parsed = address.parse().unwrap_or_default();
        let balance = self.blockchain_state.get_balance(address_parsed);
        Ok(format!("{:#x}", balance))
    }

    async fn get_transaction_count(
        &self,
        address: String,
        _block: Option<String>,
    ) -> RpcResult<String> {
        let address_parsed = address.parse().unwrap_or_default();
        let nonce = self.blockchain_state.get_nonce(address_parsed);
        Ok(format!("0x{:x}", nonce))
    }

    async fn get_code(&self, address: String, _block: Option<String>) -> RpcResult<String> {
        let address_parsed = address.parse().unwrap_or_default();
        let code = self.blockchain_state.get_code(address_parsed);
        Ok(format!("0x{}", hex::encode(&code)))
    }

    async fn get_storage_at(
        &self,
        address: String,
        position: String,
        _block: Option<String>,
    ) -> RpcResult<String> {
        let address_parsed = address.parse().unwrap_or_default();
        let slot = position.parse().unwrap_or_default();
        let value = self.blockchain_state.get_storage(address_parsed, slot);
        Ok(format!("0x{:064x}", value))
    }

    async fn get_transaction_receipt(&self, hash: String) -> RpcResult<Option<Value>> {
        let hash_parsed = hash.parse().unwrap_or_default();
        if let Some(receipt) = self.blockchain_state.get_transaction_receipt(hash_parsed) {
            let inner_logs: Vec<InnerLog> = receipt
                .logs()
                .iter()
                .map(|rpc_log| rpc_log.inner.clone())
                .collect();
            let mut receipt_json = self
                .blockchain_state
                .build_receipt_json(&inner_logs, receipt.transaction_hash);

            // Update fields from actual receipt
            if let Some(receipt_obj) = receipt_json.as_object_mut() {
                receipt_obj.insert(
                    "to".to_string(),
                    receipt
                        .to
                        .map(|addr| serde_json::Value::String(format!("{:#x}", addr)))
                        .unwrap_or(serde_json::Value::Null),
                );
                receipt_obj.insert(
                    "gasUsed".to_string(),
                    serde_json::Value::String(format!("0x{:x}", receipt.gas_used)),
                );
                receipt_obj.insert(
                    "status".to_string(),
                    serde_json::Value::String(if receipt.status() {
                        "0x1".to_string()
                    } else {
                        "0x0".to_string()
                    }),
                );
                receipt_obj.insert(
                    "type".to_string(),
                    serde_json::Value::String(format!("0x{:x}", receipt.transaction_type() as u64)),
                );
                receipt_obj.insert(
                    "effectiveGasPrice".to_string(),
                    serde_json::Value::String(format!(
                        "0x{:x}",
                        receipt.effective_gas_price as u64
                    )),
                );
            }

            return Ok(Some(receipt_json));
        }
        Ok(None)
    }

    async fn estimate_gas(&self, _tx: Value, _block: Option<String>) -> RpcResult<String> {
        Ok(format!("0x{:x}", self.config.gas_limit))
    }

    async fn gas_price(&self) -> RpcResult<String> {
        Ok(format!("0x{:x}", self.config.gas_price))
    }

    async fn max_priority_fee_per_gas(&self) -> RpcResult<String> {
        Ok(format!("0x{:x}", 2_000_000_000u64)) // 2 gwei
    }

    async fn fee_history(
        &self,
        _block_count: String,
        _newest_block: String,
        _reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<Value> {
        Ok(serde_json::json!({
            "oldestBlock": "0x1",
            "baseFeePerGas": [DEFAULT_GAS_PRICE],
            "gasUsedRatio": [0.5],
            "reward": [[DEFAULT_GAS_PRICE]]
        }))
    }

    async fn get_block_by_number(
        &self,
        block: String,
        include_txs: bool,
    ) -> RpcResult<Option<Value>> {
        debug!(
            "RpcHandlers: handling eth_getBlockByNumber with block: {}, include_txs: {}",
            block, include_txs
        );

        let current_block = self.blockchain_state.get_current_block();

        // Parse block number - handle "latest", "pending", "earliest", or hex number
        let requested_block = match block.as_str() {
            "latest" => current_block,
            "pending" => current_block + 1,
            "earliest" => 0,
            hex_str if hex_str.starts_with("0x") => match parse_block_number(hex_str) {
                Ok(num) => num,
                Err(msg) => {
                    return Err(ErrorObject::owned(-32602, msg, None::<()>));
                }
            },
            _ => {
                return Err(ErrorObject::owned(
                    -32602,
                    "Invalid block parameter",
                    None::<()>,
                ));
            }
        };

        debug!(
            "Requested block: {}, current block: {}",
            requested_block, current_block
        );

        // Handle future blocks (beyond current tip)
        if requested_block > current_block + 1 {
            return Ok(None);
        }

        // Create block data using BlockchainState
        let block_data = self.blockchain_state.create_block_json(requested_block);

        Ok(Some(block_data))
    }

    async fn get_logs(&self, _filter: Value) -> RpcResult<Vec<Value>> {
        debug!(
            "RpcHandlers: handling eth_getLogs with filter: {:?}",
            _filter
        );

        // Get all logs from blockchain state (no filtering applied for simplicity)
        let all_logs = self.blockchain_state.get_all_logs();
        let structured_logs = self.format_logs(&all_logs, None);

        // Convert alloy RpcLog structs to JSON Values
        let formatted_logs: Vec<Value> = structured_logs
            .into_iter()
            .map(|log| serde_json::to_value(log).unwrap_or(serde_json::Value::Null))
            .collect();

        info!(
            "RpcHandlers: Returning {} total logs for eth_getLogs (no filtering applied)",
            formatted_logs.len()
        );

        Ok(formatted_logs)
    }

    async fn call(&self, tx: Value, _block: Option<String>) -> RpcResult<String> {
        // Parse call parameters using idiomatic conversion trait
        let call_params = match CallParams::try_from(&tx) {
            Ok(params) => params,
            Err(e) => {
                warn!("Failed to parse call parameters: {}", e);
                return Ok("0x".to_string());
            }
        };

        // Try to find a matching pattern using PatternMatcher
        if let Some(response) = self.pattern_matcher.find_call_match(&call_params) {
            // Handle call response directly (simplified)
            match response {
                Response::Success {
                    data: ResponseData::Bytes(bytes),
                    ..
                } => {
                    return Ok(format!("0x{}", hex::encode(&bytes)));
                }
                Response::Success {
                    data: ResponseData::Logs(_),
                    ..
                } => {
                    // This shouldn't happen for call responses, but return empty bytes
                    return Ok("0x".to_string());
                }
                Response::Revert { reason, .. } => {
                    let error_message = reason.as_deref().unwrap_or("Call reverted");
                    return Err(ErrorObject::owned(-32000, error_message, None::<()>));
                }
                Response::Error(message) => {
                    return Err(ErrorObject::owned(-32603, message, None::<()>));
                }
            }
        }

        // Default behavior: return empty bytes
        Ok("0x".to_string())
    }

    async fn send_raw_transaction(&self, tx: String) -> RpcResult<String> {
        // Parse transaction parameters using idiomatic conversion trait
        let tx_params = match TxParams::try_from(tx.as_str()) {
            Ok(params) => params,
            Err(e) => {
                error!("Transaction parsing failed: {}", e);
                return Err(ErrorObject::owned(
                    -32602,
                    format!("Invalid transaction: {}", e),
                    None::<()>,
                ));
            }
        };

        // Try to find a matching pattern using PatternMatcher
        let response =
            if let Some(tx_response) = self.pattern_matcher.find_transaction_match(&tx_params) {
                tx_response
            } else {
                // Default behavior: accept transaction with random hash
                Response::Success {
                    hash: Some(BlockchainState::generate_random_hash()),
                    data: ResponseData::Logs(vec![]),
                    scheduled_transaction: None,
                }
            };

        // Simplified transaction flow (no validation, no state mutations)
        match response {
            Response::Success {
                hash,
                data: ResponseData::Logs(logs),
                scheduled_transaction,
            } => {
                // Create and store receipt using BlockchainState
                let _receipt = self.blockchain_state.create_and_store_receipt(
                    hash.unwrap_or_default(),
                    &logs,
                    tx_params.to,
                );

                // Emit logs to WebSocket subscribers if any
                if !logs.is_empty() {
                    for log in &logs {
                        if let Err(e) = self.emit_to_log_subscribers(log.clone()).await {
                            warn!("Failed to emit log event: {}", e);
                        }
                    }
                }

                // Schedule delayed transaction if present
                if let Some(scheduled_tx) = scheduled_transaction {
                    debug!(
                        delay_ms = scheduled_tx.delay.as_millis(),
                        "Scheduling follow-up transaction after transaction execution"
                    );
                    self.blockchain_state
                        .schedule_delayed_transaction(scheduled_tx, self.log_subscriptions.clone());
                }

                // Increment block number and emit new head event
                self.blockchain_state.increment_block();
                if let Err(e) = self.emit_new_head_event().await {
                    warn!("Failed to emit new head event: {}", e);
                }

                Ok(format!("{:#x}", hash.unwrap_or_default()))
            }
            Response::Success {
                data: ResponseData::Bytes(_),
                hash,
                ..
            } => {
                // This shouldn't happen for transaction responses, but handle it gracefully
                warn!(
                    "Received Bytes data for transaction response, treating as empty transaction"
                );
                Ok(format!("{:#x}", hash.unwrap_or_default()))
            }
            Response::Revert {
                hash: _,
                reason,
                scheduled_transaction,
            } => {
                // Schedule delayed transaction if present even for reverts
                if let Some(scheduled_tx) = scheduled_transaction {
                    debug!(
                        delay_ms = scheduled_tx.delay.as_millis(),
                        "Scheduling follow-up transaction after transaction revert"
                    );
                    self.blockchain_state
                        .schedule_delayed_transaction(scheduled_tx, self.log_subscriptions.clone());
                }

                let error_message = reason.as_deref().unwrap_or("Transaction reverted");
                warn!("Transaction reverted: {}", error_message);
                Err(ErrorObject::owned(-32000, error_message, None::<()>))
            }
            Response::Error(message) => {
                error!("Transaction error: {}", message);
                Err(ErrorObject::owned(-32603, message, None::<()>))
            }
        }
    }

    fn subscribe(
        &self,
        sink: PendingSubscriptionSink,
        subscription_type: String,
        _filter: Option<Value>,
    ) {
        debug!(
            component = "rpc_handlers",
            subscription_type = %subscription_type,
            "New subscription request received"
        );

        // Check subscription type and reject invalid ones immediately
        match subscription_type.as_str() {
            "logs" | "newHeads" => {
                // Accept the subscription asynchronously
                let log_subs = self.log_subscriptions.clone();
                let head_subs = self.head_subscriptions.clone();

                tokio::spawn(async move {
                    let subscription = match sink.accept().await {
                        Ok(subscription) => subscription,
                        Err(err) => {
                            error!(
                                component = "rpc_handlers",
                                error = %err,
                                subscription_type = %subscription_type,
                                "Failed to accept subscription"
                            );
                            return;
                        }
                    };

                    let subscription_id = subscription.subscription_id();

                    info!(
                        component = "rpc_handlers",
                        subscription_type = %subscription_type,
                        subscription_id = ?subscription_id,
                        "Subscription accepted and registered"
                    );

                    // Store the subscription in the appropriate map
                    match subscription_type.as_str() {
                        "logs" => {
                            let mut subs = log_subs.write().await;
                            subs.insert(subscription_id, subscription);

                            info!(
                                component = "rpc_handlers",
                                active_log_subscriptions = subs.len(),
                                "Log subscription registered"
                            );
                        }
                        "newHeads" => {
                            let mut subs = head_subs.write().await;
                            subs.insert(subscription_id, subscription);

                            info!(
                                component = "rpc_handlers",
                                active_head_subscriptions = subs.len(),
                                "Head subscription registered"
                            );
                        }
                        _ => unreachable!("Already validated subscription type"),
                    }
                });
            }
            _ => {
                error!(
                    component = "rpc_handlers",
                    subscription_type = %subscription_type,
                    "Invalid subscription type requested"
                );
                // For invalid subscription types, just reject the sink
                std::mem::drop(sink.reject(jsonrpsee::types::error::ErrorCode::InvalidParams));
            }
        }
    }
}

impl MockRpcHandler {
    /// Format logs for JSON-RPC response
    fn format_logs(&self, logs: &[InnerLog], tx_hash: Option<B256>) -> Vec<RpcLog> {
        logs.iter()
            .enumerate()
            .map(|(log_index, log)| {
                RpcLog {
                    inner: log.clone(),
                    block_hash: Some(self.blockchain_state.generate_hash()),
                    block_number: Some(self.blockchain_state.get_current_block()),
                    block_timestamp: None,
                    transaction_hash: tx_hash,
                    transaction_index: Some(0u64), // DEFAULT_TX_INDEX equivalent
                    log_index: Some(log_index as u64),
                    removed: false,
                }
            })
            .collect()
    }

    /// Emit new head event to WebSocket subscribers
    async fn emit_new_head_event(&self) -> anyhow::Result<()> {
        let current_block = self.blockchain_state.get_current_block();

        // Create block header using BlockchainState
        let block = self.blockchain_state.create_block_header(current_block);

        debug!("Emitting new head event for block {}", current_block);

        self.emit_to_head_subscribers(block).await
    }

    /// Emit a log event to all log subscribers
    async fn emit_to_log_subscribers(&self, log: InnerLog) -> anyhow::Result<()> {
        // Convert inner log to RPC log with proper metadata for WebSocket subscribers
        let rpc_log = self.blockchain_state.convert_inner_log_to_rpc(log);

        // Send the RPC log object - JsonRPSee will wrap it with subscription ID
        let log_value = serde_json::to_value(rpc_log).context("Failed to serialize log to JSON")?;

        self.broadcast_to_subscribers(&self.log_subscriptions, &log_value, "log")
            .await
            .context("Failed to broadcast log event")
    }

    /// Broadcast block header to all subscribers, removing dead sinks
    async fn emit_to_head_subscribers(
        &self,
        block: alloy::rpc::types::eth::Block,
    ) -> anyhow::Result<()> {
        // Send just the block header - JsonRPSee will wrap it with subscription ID
        let block_value = serde_json::to_value(block.header)
            .context("Failed to serialize block header to JSON")?;

        self.broadcast_to_subscribers(&self.head_subscriptions, &block_value, "head")
            .await
            .context("Failed to broadcast block header event")
    }

    async fn broadcast_to_subscribers(
        &self,
        subscribers: &Arc<AsyncRwLock<HashMap<SubscriptionId<'static>, SubscriptionSink>>>,
        event_value: &serde_json::Value,
        event_type: &str,
    ) -> anyhow::Result<()> {
        // Create the subscription message once
        let message = SubscriptionMessage::from_json(event_value)
            .context("Failed to create subscription message from JSON")?;

        // Use async write lock to send immediately within the lock
        let mut subs = subscribers.write().await;

        debug!(
            "Broadcasting {} event to {} subscribers",
            event_type,
            subs.len()
        );

        let mut dead_subscription_ids = Vec::new();

        // Send to all subscribers within the lock
        for (subscription_id, sink) in subs.iter() {
            if let Err(err) = sink.send(message.clone()).await {
                debug!(
                    "Failed to send {} event to subscriber {:?}: {}. Will remove dead sink.",
                    event_type, subscription_id, err
                );
                dead_subscription_ids.push(subscription_id.clone());
            }
        }

        // Clean up dead subscriptions by removing from HashMap
        for subscription_id in dead_subscription_ids.iter() {
            subs.remove(subscription_id);
            debug!(
                "Removed dead {} subscriber with ID {:?}",
                event_type, subscription_id
            );
        }

        if !dead_subscription_ids.is_empty() {
            debug!(
                "Cleaned up {} dead {} subscribers. Remaining: {}",
                dead_subscription_ids.len(),
                event_type,
                subs.len()
            );
        }

        Ok(())
    }
}
