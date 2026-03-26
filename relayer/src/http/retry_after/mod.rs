//! Dynamic retry-after computation module.
//!
//! This module provides types and utilities for computing dynamic `Retry-After`
//! header values based on queue state, drain rate, and request processing stage.

pub mod queue_info;
pub mod state;

// Re-export key types for convenience
pub use queue_info::{DecryptQueueInfo, ReadinessQueueInfo, RequestQueueInfo, TxQueueInfo};
pub use state::{
    compute_readiness_queue_wait_ms, compute_tx_queue_wait_ms, RequestStateInfo, RetryAfterState,
};
