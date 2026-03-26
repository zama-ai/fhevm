//! Mock server components
//!
//! This module contains the main mock server implementation and its supporting components.

pub mod handler;
pub mod rpc;
pub mod rpc_types;
pub mod server;

// Re-export main types for convenience
pub use rpc_types::{CallParams, Response, ResponseData, TxParams};
pub use server::{MockConfig, MockServer, MockServerHandle};

/// Target for subscription event emission.
///
/// Controls which subscriptions receive an event based on their index in the subscription list.
///
/// # Index Semantics
///
/// - Indices are **point-in-time** references evaluated at emission time
/// - Out-of-bounds indices are safely ignored (no panic)
/// - Duplicate indices will send the event multiple times to the same subscription
///
/// # Examples
///
/// ```text
/// SubscriptionTarget::All               → sends to all subscriptions [0, 1, 2]
/// SubscriptionTarget::Only(vec![0, 2])  → sends only to subscriptions at indices 0 and 2
/// SubscriptionTarget::Only(vec![5])     → out-of-bounds index 5 is ignored (no send)
/// SubscriptionTarget::Only(vec![0, 0])  → subscription 0 receives event twice
/// ```
#[derive(Debug, Clone, Default)]
pub enum SubscriptionTarget {
    /// Emit to all subscriptions (default behavior)
    #[default]
    All,
    /// Emit only to subscriptions at these indices
    Only(Vec<usize>),
}

impl SubscriptionTarget {
    pub fn all() -> Self {
        Self::All
    }

    pub fn only(indices: Vec<usize>) -> Self {
        Self::Only(indices)
    }
}
