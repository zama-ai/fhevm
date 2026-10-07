//! Program-specific errors returned by dep-chain instructions.

use anchor_lang::prelude::*;

/// Errors returned by the dependency chain.
#[error_code]
pub enum DepChainError {
    #[msg("tail encrypted store mismatch")]
    TailValueInvalid,
    #[msg("chain length must be between 1 and 32 steps")]
    InvalidChainLength,
}
