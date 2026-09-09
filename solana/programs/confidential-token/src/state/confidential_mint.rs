//! On-chain account data for `ConfidentialMint`.

use anchor_lang::prelude::*;

/// Confidential mint state for the token PoC.
///
/// The total supply's current handle and access encrypted State live in the
/// `EncryptedState` account at `total_supply_encrypted_state`.
#[account]
#[derive(InitSpace)]
pub struct ConfidentialMint {
    /// Admin/authority that created the mint.
    pub authority: Pubkey,
    /// Underlying SPL mint wrapped by this confidential mint.
    pub underlying_mint: Pubkey,
    /// Decimal precision inherited from the underlying mint.
    pub decimals: u8,
}

impl ConfidentialMint {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 1;
}
