//! Counter state account and PDA/label derivations.
//!
//! Public API surface: off-chain callers deriving the counter PDAs — `runtime-tests`'
//! `counter_mollusk` fixtures.

use anchor_lang::prelude::*;
use zama_fhe::{Domain, EncryptedValueId, EncryptedValueLabel};

/// Seed of the per-owner counter state PDA.
pub const COUNTER_SEED: &[u8] = b"counter";
/// Seed of the counter's execution-signing authority PDA.
pub const COUNTER_AUTHORITY_SEED: &[u8] = b"counter-authority";

/// Fixed encrypted value label for the count.
pub fn encrypted_count_label() -> [u8; 32] {
    *b"count___________________________"
}

pub fn counter_address(owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_SEED, owner.as_ref()], &crate::id())
}

pub fn counter_authority_address(counter: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_AUTHORITY_SEED, counter.as_ref()], &crate::id())
}

/// The counter's encrypted value: domain is the counter account, authority its PDA.
pub fn count_encrypted_value_id(counter: Pubkey) -> EncryptedValueId {
    EncryptedValueId::new(
        Domain::new(counter),
        counter_authority_address(counter).0,
        EncryptedValueLabel::new(encrypted_count_label()),
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
