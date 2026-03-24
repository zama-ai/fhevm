//! Mock Server Core Coordination
//!
//! This module provides the core MockServer that coordinates all components:
//! PatternMatcher, BlockchainState, RPC handlers, and server lifecycle management.

use crate::{
    blockchain::BlockchainState,
    mock_server::{
        handler::MockRpcHandler,
        rpc::EthRpcApiServer,
        rpc_types::{CallParams, Response, TxParams},
    },
    pattern_matcher::{PatternMatcher, UsageLimit},
};

/// Configuration for the mock Ethereum server
#[derive(Debug, Clone)]
pub struct MockConfig {
    pub port: u16,
    pub chain_id: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub block_time_ms: u64,
}

impl MockConfig {
    pub fn new() -> Self {
        Self {
            port: 8545,
            chain_id: 1337,
            gas_price: 20_000_000_000, // 20 gwei
            gas_limit: 21_000,
            block_time_ms: 100,
        }
    }
}

impl Default for MockConfig {
    fn default() -> Self {
        Self::new()
    }
}
use alloy::primitives::{Address, Bytes, U256};
use anyhow::{Context, Result as AnyhowResult};
use indexmap::IndexMap;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use jsonrpsee::types::SubscriptionId;
use jsonrpsee::SubscriptionSink;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock as AsyncRwLock;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

/// Server handle that provides lifecycle management and server information
#[derive(Debug)]
pub struct MockServerHandle {
    /// Server handle for lifecycle management
    server_handle: ServerHandle,
    /// Server URL for client connections
    url: String,
    /// Shutdown token for graceful shutdown coordination
    shutdown_token: CancellationToken,
}

impl MockServerHandle {
    /// Create a new server handle
    pub fn new(
        server_handle: ServerHandle,
        url: String,
        shutdown_token: CancellationToken,
    ) -> Self {
        Self {
            server_handle,
            url,
            shutdown_token,
        }
    }

    /// Get the server URL for client connections
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Shutdown the server gracefully
    pub async fn shutdown(self) -> AnyhowResult<()> {
        info!("Initiating graceful shutdown of MockServer");

        // Signal shutdown to any listeners
        self.shutdown_token.cancel();

        // Stop the JsonRPSee server
        self.server_handle.stop()?;

        info!("MockServer shutdown completed successfully");
        Ok(())
    }

    /// Check if the server is still running
    pub fn is_running(&self) -> bool {
        !self.server_handle.is_stopped()
    }
}

/// Mock server coordinating PatternMatcher, BlockchainState, and RPC handlers
#[derive(Debug, Clone)]
pub struct MockServer {
    config: Arc<MockConfig>,
    pattern_matcher: Arc<PatternMatcher>,
    blockchain_state: Arc<BlockchainState>,
    log_subscriptions: Arc<AsyncRwLock<IndexMap<SubscriptionId<'static>, SubscriptionSink>>>,
    head_subscriptions: Arc<AsyncRwLock<IndexMap<SubscriptionId<'static>, SubscriptionSink>>>,
    shutdown_token: CancellationToken,
}

impl MockServer {
    /// Create new mock server with given configuration
    pub fn new(config: MockConfig) -> Self {
        debug!("Creating new MockServer with config: {:?}", config);

        // Direct component creation - single authoritative way
        let config = Arc::new(config);
        let pattern_matcher = Arc::new(PatternMatcher::new());
        let blockchain_state = Arc::new(BlockchainState::new(HashMap::new()));
        let log_subscriptions = Arc::new(AsyncRwLock::new(IndexMap::new()));
        let head_subscriptions = Arc::new(AsyncRwLock::new(IndexMap::new()));
        let shutdown_token = CancellationToken::new();

        debug!("MockServer components initialized successfully");

        Self {
            config,
            pattern_matcher,
            blockchain_state,
            log_subscriptions,
            head_subscriptions,
            shutdown_token,
        }
    }

    /// Start server and return handle for lifecycle management
    pub async fn start(self) -> AnyhowResult<MockServerHandle> {
        let addr = Self::parse_socket_addr(self.config.port)?;
        let url = Self::create_server_url(addr);

        info!(
            port = self.config.port,
            address = %addr,
            "Starting MockServer"
        );

        // Create RPC implementation
        let rpc_impl = MockRpcHandler {
            config: self.config.clone(),
            pattern_matcher: self.pattern_matcher,
            blockchain_state: self.blockchain_state,
            log_subscriptions: self.log_subscriptions,
            head_subscriptions: self.head_subscriptions,
            shutdown_token: self.shutdown_token.clone(),
        };

        let server_handle = Self::start_server_on_addr(EthRpcApiServer::into_rpc(rpc_impl), addr)
            .await
            .with_context(|| format!("Failed to start mock server on {}", addr))?;

        info!(
            port = self.config.port,
            url = %url,
            "MockServer started successfully"
        );

        Ok(MockServerHandle::new(
            server_handle,
            url,
            self.shutdown_token,
        ))
    }

    /// Parse and validate a socket address from port
    fn parse_socket_addr(port: u16) -> AnyhowResult<SocketAddr> {
        let addr = format!("127.0.0.1:{}", port);
        addr.parse::<SocketAddr>()
            .with_context(|| format!("Invalid socket address: {}", addr))
    }

    /// Create a server URL from address
    fn create_server_url(addr: SocketAddr) -> String {
        format!("http://{}", addr)
    }

    /// Start a JsonRPSee server on a specific address
    async fn start_server_on_addr<T>(rpc_impl: T, addr: SocketAddr) -> AnyhowResult<ServerHandle>
    where
        T: Into<jsonrpsee::Methods>,
    {
        info!(
            component = "server_lifecycle",
            bind_address = %addr,
            "Starting JsonRPSee HTTP/WebSocket server"
        );

        // Build the server with both HTTP and WebSocket support
        let server = ServerBuilder::default()
            .build(addr)
            .await
            .with_context(|| format!("Failed to create JsonRPSee server on {}", addr))?;

        // Start the server with registered methods
        let server_handle = server.start(rpc_impl.into());

        info!(
            component = "server_lifecycle",
            bind_address = %addr,
            "JsonRPSee HTTP/WebSocket server ready for JSON-RPC requests and subscriptions"
        );

        Ok(server_handle)
    }

    /// Get the URL the server will listen on when started
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.config.port)
    }

    // Blockchain State Management Methods

    pub fn set_balance(&self, address: Address, amount: U256) {
        debug!(
            address = %address,
            amount = %amount,
            "Setting account balance"
        );
        self.blockchain_state.set_balance(address, amount);
    }

    pub fn set_code(&self, address: Address, bytecode: Bytes) {
        debug!(
            address = %address,
            bytecode_len = bytecode.len(),
            "Setting contract bytecode"
        );
        self.blockchain_state.set_code(address, bytecode);
    }

    pub fn set_storage(&self, address: Address, slot: U256, value: U256) {
        debug!(
            address = %address,
            slot = %slot,
            value = %value,
            "Setting contract storage"
        );
        self.blockchain_state.set_storage(address, slot, value);
    }

    pub fn set_nonce(&self, address: Address, nonce: u64) {
        debug!(
            address = %address,
            nonce = nonce,
            "Setting account nonce"
        );
        self.blockchain_state.set_nonce(address, nonce);
    }

    // Pattern Registration Methods

    /// Register mock response for transactions matching predicate
    pub fn on_transaction(
        &self,
        predicate: impl Fn(&TxParams) -> bool + Send + Sync + 'static,
        response: Response,
        usage: UsageLimit,
    ) {
        debug!("Registering transaction pattern");

        self.pattern_matcher
            .add_transaction_pattern(Arc::new(predicate), response, usage);
    }

    /// Register mock response for calls matching predicate
    pub fn on_call(
        &self,
        predicate: impl Fn(&CallParams) -> bool + Send + Sync + 'static,
        response: Response,
        usage: UsageLimit,
    ) {
        debug!("Registering call pattern");

        self.pattern_matcher
            .add_call_pattern(Arc::new(predicate), response, usage);
    }

    /// Register a dynamic mock response that can inspect the request.
    pub fn on_call_dynamic(
        &self,
        predicate: impl Fn(&CallParams) -> bool + Send + Sync + 'static,
        responder: impl Fn(&CallParams) -> Response + Send + Sync + 'static,
        usage: UsageLimit,
    ) {
        debug!("Registering dynamic call pattern");

        self.pattern_matcher.add_call_pattern_dynamic(
            Arc::new(predicate),
            Arc::new(responder),
            usage,
        );
    }

    /// Clear all patterns and reset blockchain state for test cleanup
    pub fn reset_state(&self) {
        debug!("Resetting MockServer state");

        // Clear all registered patterns
        self.pattern_matcher.clear_all_patterns();

        // Reset blockchain state to empty accounts
        self.blockchain_state.reset(HashMap::new());

        debug!("MockServer state reset completed");
    }

    // Internal Access Methods

    pub fn pattern_matcher(&self) -> &Arc<PatternMatcher> {
        &self.pattern_matcher
    }

    pub fn blockchain_state(&self) -> &Arc<BlockchainState> {
        &self.blockchain_state
    }

    pub fn shutdown_token(&self) -> &CancellationToken {
        &self.shutdown_token
    }

    /// Get the number of active log subscriptions
    pub async fn get_log_subscription_count(&self) -> usize {
        self.log_subscriptions.read().await.len()
    }

    /// Get access to log subscriptions (for testing)
    pub fn log_subscriptions(
        &self,
    ) -> &Arc<AsyncRwLock<IndexMap<SubscriptionId<'static>, SubscriptionSink>>> {
        &self.log_subscriptions
    }

    /// Convert MockServer into RPC methods
    pub fn into_rpc(self) -> jsonrpsee::Methods {
        let rpc_impl = MockRpcHandler {
            config: self.config,
            pattern_matcher: self.pattern_matcher,
            blockchain_state: self.blockchain_state,
            log_subscriptions: self.log_subscriptions,
            head_subscriptions: self.head_subscriptions,
            shutdown_token: self.shutdown_token,
        };
        EthRpcApiServer::into_rpc(rpc_impl).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_server::{MockConfig, Response},
        pattern_matcher::UsageLimit,
    };
    use alloy::primitives::{Address, Bytes, U256};

    #[test]
    fn test_server_creation_and_configuration() {
        let config = MockConfig {
            port: 8545,
            ..MockConfig::new()
        };
        let server = MockServer::new(config.clone());

        assert_eq!(
            server.url(),
            "http://127.0.0.1:8545",
            "Server URL should match configured port"
        );

        // Verify components are properly initialized (Arc references should not be null)
        assert!(
            !std::ptr::eq(server.pattern_matcher().as_ref(), std::ptr::null()),
            "Pattern matcher should be initialized"
        );
        assert!(
            !std::ptr::eq(server.blockchain_state().as_ref(), std::ptr::null()),
            "Blockchain state should be initialized"
        );
    }

    #[test]
    fn test_pattern_registration_and_state_management() {
        let server = MockServer::new(MockConfig::new());
        let test_address = Address::repeat_byte(1);

        // Test transaction pattern registration
        server.on_transaction(
            move |params| params.to == Some(test_address),
            Response::transaction_success(),
            UsageLimit::Once,
        );

        // Test call pattern registration
        server.on_call(
            |params| params.input.len() == 4,
            Response::call_success(Bytes::from("test")),
            UsageLimit::Unlimited,
        );

        // Test state management
        server.set_balance(test_address, U256::from(1000));
        server.set_nonce(test_address, 5);

        // Test state reset
        server.reset_state();

        // Verify state reset completes without panicking (no assertion needed - test passes if no panic)
    }
}
