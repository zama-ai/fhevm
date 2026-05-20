//! # Ethereum Mock Server
//!
//! A deterministic Ethereum JSON-RPC mock server for testing blockchain
//! applications.
//!
//! The mock provides RPC simulation: configured `eth_call` responses, mocked
//! transaction receipts, blockchain state, and HTTP/WebSocket endpoints.
//! Component-specific contract setup should live with the component tests that
//! own those bindings.
//!
//! ## Quick Start
//!
//! Create a `MockServer`, configure responses, and start testing:
//!
//! ```rust,no_run
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use ethereum_rpc_mock::{MockServer, MockConfig};
//!
//! let server = MockServer::new(MockConfig::new());
//! let handle = server.clone().start().await?;
//! // Point your component's provider at `handle.url()`, then run your tests.
//! handle.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! See `tests/` for detailed examples.
//!
//! ## Features
//!
//! - **EIP-1559 Support**: Modern Ethereum transaction mocking
//! - **WebSocket Subscriptions**: Real-time event streaming
//! - **Flexible Patterns**: Match requests, return custom responses
//! - **Blockchain State**: Manage accounts, balances, storage

// Internal modules
mod blockchain;
mod mock_server;
mod pattern_matcher;

// Test utilities module
pub mod test_utils;

// Primary API Exports

/// The main mock server - primary entry point for all functionality
pub use mock_server::MockServer;

/// Server handle for lifecycle management
pub use mock_server::MockServerHandle;

/// Configuration for server setup
pub use mock_server::MockConfig;

/// Response types for mocking
pub use mock_server::{CallParams, Response, ResponseData, TxParams};

/// Usage limiting for patterns
pub use pattern_matcher::UsageLimit;

/// Scheduled transaction support for delayed responses
pub use blockchain::ScheduledTransaction;

/// Subscription targeting for selective event emission
pub use mock_server::SubscriptionTarget;
