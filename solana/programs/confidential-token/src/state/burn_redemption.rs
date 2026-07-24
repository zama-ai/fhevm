//! On-chain account data for `BurnRedemption`.

use anchor_lang::prelude::*;

/// Replay marker for a redeemed burned amount handle.
#[account]
#[derive(InitSpace)]
pub struct BurnRedemption {
    /// Confidential mint whose vault paid the redemption.
    pub mint: Pubkey,
    /// Token owner that redeemed the burned amount.
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle proven by KMS.
    pub burned_handle: [u8; 32],
    /// `EncryptedValue` encrypted value account for `burned_handle`.
    pub burned_encrypted_value: Pubkey,
    /// KMS-certified cleartext amount released from the vault.
    pub cleartext_amount: u64,
    /// PDA bump for `(mint, burned_handle)`.
    pub bump: u8,
}

impl BurnRedemption {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 8 + 1;
}
