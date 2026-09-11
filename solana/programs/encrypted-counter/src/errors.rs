//! Program-specific errors returned by encrypted-counter instructions.
//!
//! Append new variants at the tail only; error codes are part of the app ABI.

use anchor_lang::prelude::*;

/// Errors returned by the encrypted counter.
#[error_code]
pub enum CounterError {
    #[msg("FHE execution failed to build or resolve")]
    InvalidFheExecution,
    #[msg("counter encrypted store address mismatch")]
    CountValueInvalid,
}
