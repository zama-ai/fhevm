//! Chain state account and PDA/label derivations.
//!
//! Public API surface: off-chain callers deriving the chain PDAs — `runtime-tests`'
//! `dep_chain_mollusk` fixtures.

use anchor_lang::prelude::*;
use zama_fhe::{AppScope, EncryptedValueId, EncryptedValueLabel};

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

/// The application one chain is to the host: this program, scoped to the chain account.
/// HCU metering and the deny list key on it.
pub fn chain_app(chain: Pubkey) -> AppScope {
    AppScope {
        program: crate::id(),
        scope: chain.to_bytes(),
    }
}

/// The chain's encrypted tail value: the chain's application, authority its PDA.
pub fn tail_encrypted_value_id(chain: Pubkey) -> EncryptedValueId {
    EncryptedValueId::new(
        chain_app(chain),
        chain_authority_address(chain).0,
        EncryptedValueLabel::new(encrypted_tail_label()),
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
