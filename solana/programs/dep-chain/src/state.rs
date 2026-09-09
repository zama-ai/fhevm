//! Chain state account and PDA/label derivations.
//!
//! Public API surface: off-chain callers deriving the chain PDAs — `runtime-tests`'
//! `dep_chain_mollusk` fixtures.

use anchor_lang::prelude::*;
use zama_fhe::StateId;

/// Seed of the per-owner chain state PDA.
pub const CHAIN_SEED: &[u8] = b"dep-chain";
/// Seed of the chain's execution-signing authority PDA.
pub const CHAIN_AUTHORITY_SEED: &[u8] = b"chain-authority";

/// Fixed encrypted value label for the chain tail.
pub fn encrypted_tail_label() -> [u8; 32] {
    *b"tail____________________________"
}

pub fn chain_address(owner: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CHAIN_SEED, owner.as_ref()], &crate::id())
}

pub fn chain_authority_address(chain: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CHAIN_AUTHORITY_SEED, chain.as_ref()], &crate::id())
}

/// The chain's encrypted tail value: the chain's application, authority its PDA.
pub fn chain_state_id(chain: Pubkey) -> StateId {
    StateId::new(
        crate::id(),
        chain_authority_address(chain).0,
        chain.to_bytes(),
    )
}

/// One owner's chain. The owner is bound by the PDA seeds; the stored bumps let instructions
/// skip the bump search.
#[account]
pub struct Chain {
    pub bump: u8,
    pub authority_bump: u8,
}

impl Chain {
    pub const SPACE: usize = 1 + 1;
}
