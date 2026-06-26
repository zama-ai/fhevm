//! On-chain account data for `ConfidentialTokenAccount`.

use anchor_lang::prelude::*;

/// Confidential token account state.
#[account]
#[derive(InitSpace)]
pub struct ConfidentialTokenAccount {
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential mint this account belongs to.
    pub mint: Pubkey,
    /// Current confidential balance handle. Its durable ACL is the balance
    /// encrypted-value ACL lineage derived from `(mint, token_account)` — not
    /// stored here.
    pub balance_handle: [u8; 32],
    /// Next nonce sequence to use for one-shot amount ACL records (transfer,
    /// random, burn/refund witnesses).
    pub next_amount_nonce_sequence: u64,
    /// PDA bump for the token account.
    pub bump: u8,
}

impl ConfidentialTokenAccount {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 1;
}
