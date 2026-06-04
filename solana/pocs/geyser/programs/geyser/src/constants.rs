use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

/// Seed prefix for the per-authority data PDA.
#[constant]
pub const DATA_SEED: &[u8] = b"data";
