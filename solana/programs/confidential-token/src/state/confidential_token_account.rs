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
    /// Current confidential balance handle.
    pub balance_handle: [u8; 32],
    /// Current ZamaHost ACL record for `balance_handle`.
    pub balance_acl_record: Pubkey,
    /// Next nonce sequence to use for a balance ACL record.
    pub next_balance_nonce_sequence: u64,
    /// Next nonce sequence to use for owner-scoped random amount ACL records.
    pub next_amount_nonce_sequence: u64,
    /// PDA bump for the token account.
    pub bump: u8,
}

impl ConfidentialTokenAccount {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 8 + 8 + 1;
}
