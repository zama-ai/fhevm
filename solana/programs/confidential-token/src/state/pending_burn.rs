//! Single pending burn opened at burn time (fhevm-internal#1862 Wave 2).
//!
//! Replaces the forever `BurnRedemption` replay marker. Exactly one account may exist for a
//! confidential token account. It is opened when a burn completes and closed by either:
//! - **redeem** (`redeem_burned_amount`): certificate + proof -> underlying-token payout
//! - **cancel** (`cancel_pending_burn`): FHE re-credit to confidential balance and total supply
//!
//! A new burn cannot begin until the previous pending burn has been settled through one of those
//! terminal operations. This keeps the shared burned-amount encrypted value account current and
//! removes the need for multiple pending burns per token account.

use anchor_lang::prelude::*;

/// Open burn awaiting redeem (underlying tokens) or cancel (confidential tokens).
#[account]
#[derive(InitSpace)]
pub struct PendingBurn {
    /// Confidential mint whose underlying-token vault funds a redemption payout.
    pub mint: Pubkey,
    /// Token account owner (rent destination on close).
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Burned amount handle produced by the burn execution (stored for redeem matching).
    pub burned_handle: [u8; 32],
    /// Shared `burned_amount` EncryptedValue account for this token account.
    pub burned_encrypted_value: Pubkey,
    /// PDA bump for `(mint, token_account)`.
    pub bump: u8,
}

impl PendingBurn {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 1;
}
