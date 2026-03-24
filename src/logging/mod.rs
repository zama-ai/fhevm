//! Logging primitives for the relayer
//!
//! This module provides standardized enums for structured logging with two levels:
//!
//! ## L1: Request Steps (First Responder)
//! Track job progress through the system. Always include `int_job_id`.
//! - **INFO**: Happy path milestones
//! - **WARN**: Degraded states (retrying, bounced)
//! - **ERROR**: Failures at boundaries
//!
//! Enums: `PublicDecryptStep`, `UserDecryptStep`, `InputProofStep`, `ListenerStep`
//!
//! ## L2: Worker Steps (Deep Debug)
//! Track infrastructure operations. May or may not have `int_job_id`.
//! - **DEBUG**: Normal operations
//! - **WARN**: Infrastructure problems
//!
//! Enums: `WorkerStep`, `ThrottlerStep`, `TxEngineStep`
//!
//! ## Usage
//!
//! ```rust,ignore
//! use fhevm_relayer::logging::{InputProofStep, ThrottlerStep, TxEngineStep};
//!
//! // L1: Request step (INFO with int_job_id)
//! info!(
//!     int_job_id = %job_id,
//!     step = %InputProofStep::TxConfirmed,
//!     tx_hash = %hash,
//!     "Transaction confirmed"
//! );
//!
//! // L1: Degraded step (WARN with int_job_id)
//! warn!(
//!     int_job_id = %job_id,
//!     step = %InputProofStep::Bounced,
//!     reason = "queue_full",
//!     "Request bounced"
//! );
//!
//! // L2: Worker step (DEBUG, may or may not have int_job_id)
//! debug!(
//!     step = %ThrottlerStep::TaskDequeued,
//!     queue_size = queue.len(),
//!     "Task dequeued"
//! );
//!
//! // L2: Transaction engine step (DEBUG/WARN)
//! debug!(
//!     step = %TxEngineStep::NonceAcquired,
//!     nonce = nonce,
//!     "Nonce acquired"
//! );
//! ```
//!
//! See `LOGGING_POLICY.md` for full guidelines.

mod steps;

// Re-export all public types
pub use steps::{
    InputProofStep, ListenerStep, PublicDecryptStep, ThrottlerStep, TxEngineStep, UserDecryptStep,
    WorkerStep,
};
