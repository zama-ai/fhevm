//! Program-specific errors returned by dep-chain instructions.
//!
//! Append new variants at the tail only; error codes are part of the app ABI.

use anchor_lang::prelude::*;

/// Errors returned by the dependency chain.
#[error_code]
pub enum DepChainError {
    #[msg("FHE execution failed to build or resolve")]
    InvalidFheExecution,
    #[msg("tail encrypted value account mismatch")]
    TailValueInvalid,
    #[msg("chain length must be between 1 and 32 steps")]
    InvalidChainLength,
}
