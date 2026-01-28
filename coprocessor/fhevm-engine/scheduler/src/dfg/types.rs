//! # Scheduler Type Definitions
//!
//! This module defines the core types used throughout the scheduler for representing
//! task inputs, outputs, and error conditions.
//!
//! ## Key Types
//!
//! - [`TaskResult`]: Result of executing a single FHE operation
//! - [`DFGTaskInput`]: Input to an individual task within a transaction
//! - [`DFGTxResult`]: Transaction-level result with handle and compressed ciphertext
//! - [`DFGTxInput`]: Input to a transaction (decompressed or compressed)
//! - [`SchedulerError`]: Error types that can occur during scheduling

use anyhow::Result;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};

/// Result of executing a single FHE operation.
///
/// Contains the computed ciphertext along with metadata about whether it should
/// be persisted and which transaction it belongs to.
///
/// # Fields
///
/// * `ct` - The computed ciphertext in decompressed form
/// * `compressed_ct` - Optional compressed representation for persistence;
///   `Some((type_id, bytes))` for allowed outputs, `None` for intermediates
/// * `is_allowed` - Whether this result should be persisted to the database
/// * `transaction_id` - The ID of the transaction this result belongs to
pub struct TaskResult {
    pub ct: SupportedFheCiphertexts,
    pub compressed_ct: Option<(i16, Vec<u8>)>,
    pub is_allowed: bool,
    pub transaction_id: Handle,
}

/// Transaction-level result for a single output handle.
///
/// This is the final output format returned by the scheduler after all
/// operations are complete. It contains the compressed ciphertext ready
/// for storage or an error if computation failed.
///
/// # Fields
///
/// * `handle` - The unique handle identifying this output
/// * `transaction_id` - The ID of the transaction that produced this result
/// * `compressed_ct` - Either `Ok((type_id, bytes))` with the compressed ciphertext,
///   or `Err` if the computation failed
pub struct DFGTxResult {
    pub handle: Handle,
    pub transaction_id: Handle,
    pub compressed_ct: Result<(i16, Vec<u8>)>,
}
impl std::fmt::Debug for DFGTxResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(
            f,
            "Result: [{:?}] - tid [{:?}]",
            self.handle, self.transaction_id
        );
        if self.compressed_ct.is_err() {
            let _ = write!(f, "\t ERROR");
        } else {
            let _ = write!(f, "\t OK");
        }
        writeln!(f)
    }
}

/// Input to a transaction component.
///
/// Represents an input ciphertext that a transaction needs for its computations.
/// The input can be either already decompressed (ready for use) or still in
/// compressed form (needs decompression before use).
///
/// # Variants
///
/// * `Value` - A decompressed ciphertext ready for FHE operations.
///   Contains `(ciphertext, is_allowed)` where `is_allowed` indicates if the
///   ciphertext has been verified/authorized for use.
/// * `Compressed` - A compressed ciphertext that needs decompression.
///   Contains `((type_id, bytes), is_allowed)`.
#[derive(Clone)]
pub enum DFGTxInput {
    /// Decompressed ciphertext with its allowed status.
    Value((SupportedFheCiphertexts, bool)),
    /// Compressed ciphertext with type ID, bytes, and allowed status.
    Compressed(((i16, Vec<u8>), bool)),
}
impl std::fmt::Debug for DFGTxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(_) => write!(f, "DecCT"),
            Self::Compressed(_) => write!(f, "ComCT"),
        }
    }
}

/// Input to an individual FHE operation within a transaction.
///
/// This enum represents the three possible states of an operation input:
/// 1. Already available as a decompressed value
/// 2. Available but compressed (needs decompression)
/// 3. Not yet available (depends on another operation's output)
///
/// # Variants
///
/// * `Value` - A decompressed ciphertext ready for immediate use
/// * `Compressed` - A compressed ciphertext `(type_id, bytes)` that needs decompression
/// * `Dependence` - A handle referring to another operation's output that must complete first
#[derive(Clone)]
pub enum DFGTaskInput {
    /// Decompressed ciphertext ready for use.
    Value(SupportedFheCiphertexts),
    /// Compressed ciphertext awaiting decompression.
    Compressed((i16, Vec<u8>)),
    /// Handle to a dependence that must be resolved before this input is available.
    Dependence(Handle),
}
impl std::fmt::Debug for DFGTaskInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(_) => write!(f, "DecCT"),
            Self::Compressed(_) => write!(f, "ComCT"),
            Self::Dependence(_) => write!(f, "DepHL"),
        }
    }
}

/// Error types that can occur during FHE operation scheduling.
///
/// These errors represent various failure modes in the scheduling process,
/// from graph construction issues to runtime execution failures.
#[derive(Debug, Copy, Clone)]
pub enum SchedulerError {
    /// A cyclic dependence was detected in the dataflow graph.
    /// This indicates that operations form a dependence loop, making
    /// execution impossible.
    CyclicDependence,

    /// An inconsistency was detected in the dataflow graph structure.
    /// This typically indicates a bug in graph construction or an
    /// invalid node/edge reference.
    DataflowGraphError,

    /// Required inputs for an operation are not available.
    /// This can happen if a dependence failed or if external inputs
    /// were not provided.
    MissingInputs,

    /// Re-randomisation of input ciphertexts failed.
    /// This cryptographic operation is required for security and
    /// cannot be skipped.
    ReRandomisationError,

    /// A generic scheduler error that doesn't fit other categories.
    /// Used for unexpected runtime conditions.
    SchedulerError,
}

impl std::error::Error for SchedulerError {}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CyclicDependence => {
                write!(f, "Dependence cycle in dataflow graph")
            }
            Self::DataflowGraphError => {
                write!(f, "Inconsistent dataflow graph error")
            }
            Self::MissingInputs => {
                write!(f, "Missing inputs")
            }
            Self::ReRandomisationError => {
                write!(f, "Re-randomisation error")
            }
            Self::SchedulerError => {
                write!(f, "Generic scheduler error")
            }
        }
    }
}
