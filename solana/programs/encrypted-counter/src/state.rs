//! Counter state and address derivations.
//!
//! Public API surface: off-chain callers deriving the counter PDAs — `runtime-tests`'
//! `counter_mollusk` fixtures.

use anchor_lang::prelude::*;
use zama_fhe::StateId;

/// Seed of the per-owner counter state PDA.
pub const COUNTER_SEED: &[u8] = b"counter";
/// Seed of the counter's execution-signing authority PDA.
pub const COUNTER_AUTHORITY_SEED: &[u8] = b"counter-authority";

/// Dictionary key for the encrypted count.
pub fn count_key() -> [u8; 32] {
    *b"count___________________________"
}

pub fn counter_address(owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_SEED, owner.as_ref()], &crate::id())
}

pub fn counter_authority_address(counter: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_AUTHORITY_SEED, counter.as_ref()], &crate::id())
}

/// The host dictionary controlled by the counter's authority PDA.
pub fn counter_state_id(counter: Pubkey) -> StateId {
    StateId::new(
        crate::id(),
        counter_authority_address(counter).0,
        counter.to_bytes(),
    )
}

/// One owner's counter. The owner is bound by the PDA seeds; the stored bumps let instructions
/// skip the bump search.
#[account]
pub struct Counter {
    pub bump: u8,
    pub authority_bump: u8,
}

impl Counter {
    pub const SPACE: usize = 1 + 1;
}
