//! Decoded op records: the per-step compute shapes off-chain decoders
//! reconstruct by replaying `fhe_execute` instruction data through the same
//! handle derivation the program runs (see the host-listener's
//! `solana_reconstruct`).
//!
//! Nothing here is emitted on-chain — these are plain value types, not Anchor
//! events. The only evented compute data is what an indexer cannot recompute
//! from instruction data alone (`FheExecuteRandomSeedsEvent`,
//! `PublicOutputsProducedEvent` in `events.rs`).

use crate::state::{FheBinaryOpCode, FheTernaryOpCode, FheUnaryOpCode};

/// Decoded record of a binary FHE operation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheBinaryOp {
    /// Record schema version.
    pub version: u8,
    /// Binary operator.
    pub op: FheBinaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Left-hand operand handle.
    pub lhs: [u8; 32],
    /// Right-hand operand handle or scalar bytes.
    pub rhs: [u8; 32],
    /// Whether `rhs` is plaintext scalar bytes.
    pub scalar: bool,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of a ternary FHE operation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheTernaryOp {
    /// Record schema version.
    pub version: u8,
    /// Ternary operator.
    pub op: FheTernaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Encrypted control handle.
    pub control: [u8; 32],
    /// Handle selected when `control` is true.
    pub if_true: [u8; 32],
    /// Handle selected when `control` is false.
    pub if_false: [u8; 32],
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of a trivial encryption accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrivialEncrypt {
    /// Record schema version.
    pub version: u8,
    /// Subject associated with the created handle.
    pub subject: [u8; 32],
    /// Plaintext encoded into the handle.
    pub plaintext: [u8; 32],
    /// FHE type byte embedded in the handle.
    pub fhe_type: u8,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of a random ciphertext creation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheRand {
    /// Record schema version.
    pub version: u8,
    /// Subject associated with the random handle.
    pub subject: [u8; 32],
    /// Host-derived random seed for this step.
    pub seed: [u8; 16],
    /// FHE type byte.
    pub fhe_type: u8,
    /// Output handle.
    pub result: [u8; 32],
}

/// Decoded record of a bounded random ciphertext creation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheRandBounded {
    /// Record schema version.
    pub version: u8,
    /// Subject associated with the random handle.
    pub subject: [u8; 32],
    /// Exclusive upper bound encoded as a 256-bit big-endian integer.
    pub upper_bound: [u8; 32],
    /// Host-derived random seed for this step.
    pub seed: [u8; 16],
    /// FHE type byte.
    pub fhe_type: u8,
    /// Output handle.
    pub result: [u8; 32],
}

/// Decoded record of a unary FHE operation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheUnaryOp {
    /// Record schema version.
    pub version: u8,
    /// Unary operator.
    pub op: FheUnaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Operand handle.
    pub operand: [u8; 32],
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of an FHE sum operation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheSum {
    /// Record schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Input operand handles.
    pub operands: Vec<[u8; 32]>,
    /// FHE type of all operands and the output.
    pub fhe_type: u8,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of an FHE is-in test accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheIsIn {
    /// Record schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Value handle being tested.
    pub value: [u8; 32],
    /// Set of handles to test against.
    pub set: Vec<[u8; 32]>,
    /// FHE type of value and set elements.
    pub fhe_type: u8,
    /// Output handle (always ebool) verified by the host formula.
    pub result: [u8; 32],
}

/// Decoded record of an FHE multiply-divide operation accepted by the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FheMulDiv {
    /// Record schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// First factor handle.
    pub factor1: [u8; 32],
    /// Second factor handle or scalar bytes.
    pub factor2: [u8; 32],
    /// Divisor plaintext scalar bytes.
    pub divisor: [u8; 32],
    /// Whether `factor2` is plaintext scalar bytes.
    pub scalar: bool,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}
