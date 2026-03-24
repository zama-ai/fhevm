//! # Ethereum Mock Server
//!
//! A clean, modern Ethereum mock server for testing blockchain applications.
//! Supports EIP-1559 transactions and FHEVM (Fully Homomorphic Encryption VM) patterns.
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
//! let handle = server.start().await?;
//! // Run your tests...
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
//! - **FHEVM Patterns**: Built-in encrypted computation test patterns  
//! - **WebSocket Subscriptions**: Real-time event streaming
//! - **Flexible Patterns**: Match requests, return custom responses
//! - **Blockchain State**: Manage accounts, balances, storage
//! - **Clean Architecture**: Domain-focused organization

// Internal modules
mod blockchain;
mod mock_server;
mod pattern_matcher;

// Public API modules
pub mod fhevm;

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
pub use mock_server::Response;

/// Usage limiting for patterns
pub use pattern_matcher::UsageLimit;

/// Scheduled transaction support for delayed responses
pub use blockchain::ScheduledTransaction;

/// Subscription targeting for selective event emission
pub use mock_server::SubscriptionTarget;
