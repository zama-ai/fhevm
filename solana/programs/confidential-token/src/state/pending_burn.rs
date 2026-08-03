//! Per-burn pending lane opened at burn time (fhevm-internal#1862 Wave 2).
//!
//! Replaces the forever `BurnRedemption` replay marker. A lane is opened when the burn
//! completes (seeds include `token_account` + client-supplied `burn_id`), then closed by either:
//! - **claim** (`redeem_burned_amount`): KMS cert + MMR proof → USDC payout
//! - **recover** (`recover_pending_burn`): FHE re-credit to confidential balance while the
//!   burned handle is still the shared `burned_amount` EV's `current_handle`
//!
//! Concurrent burns open concurrent lanes (distinct `burn_id`s). Historical burns — ones a later
//! burn has updated the `burned_amount` handle past — cannot recover without KMS; claim remains
//! the path for those handles.
//!
//! Seeds use `burn_id` rather than `burned_handle` so the PDA can appear in transaction account
//! metas before `fhe_execute` derives the burned handle.

use anchor_lang::prelude::*;

/// Open burn lane awaiting claim (USDC) or recover (confidential).
#[account]
#[derive(InitSpace)]
pub struct PendingBurn {
    /// Confidential mint whose vault backs a claim.
    pub mint: Pubkey,
    /// Token account owner (rent destination on close).
    pub owner: Pubkey,
    /// Token account that produced the burned amount.
    pub token_account: Pubkey,
    /// Client-supplied unique id (also in PDA seeds); known before the burn tx.
    pub burn_id: [u8; 32],
    /// Burned amount handle produced by the burn eval (stored for claim matching).
    pub burned_handle: [u8; 32],
    /// Shared `burned_amount` EncryptedValue account for this token account.
    pub burned_encrypted_value: Pubkey,
    /// PDA bump for `(mint, token_account, burn_id)`.
    pub bump: u8,
}

impl PendingBurn {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 32 + 32 + 1;
}
