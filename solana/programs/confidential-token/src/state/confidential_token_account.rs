//! On-chain account data for `ConfidentialTokenAccount`.

use anchor_lang::prelude::*;

/// Confidential token account state.
///
/// The balance's current handle and access encrypted value account live in the
/// `EncryptedValue` account at `balance_encrypted_value`, not here — addressing
/// is stable per token account, so no nonce/sequence bookkeeping is needed.
#[account]
#[derive(InitSpace)]
pub struct ConfidentialTokenAccount {
    /// Token account owner.
    pub owner: Pubkey,
    /// Confidential mint this account belongs to.
    pub mint: Pubkey,
    /// `EncryptedValue` encrypted value account PDA holding the current balance handle.
    pub balance_encrypted_value: Pubkey,
    /// PDA bump for the token account.
    pub bump: u8,
}

impl ConfidentialTokenAccount {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 1;
}
