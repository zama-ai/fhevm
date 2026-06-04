use anchor_lang::prelude::*;

/// Data stored inside the PDA.
#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    /// The authority allowed to write to this PDA.
    pub authority: Pubkey,
    /// An arbitrary numeric value.
    pub value: u64,
    /// A short message.
    #[max_len(64)]
    pub message: String,
    /// PDA bump, cached for cheap re-derivation.
    pub bump: u8,
}
