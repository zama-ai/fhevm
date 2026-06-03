//! On-chain account data for `DenySubjectRecord`.

use super::*;

/// Optional deny-list record used to fail persistent grants closed.
#[account]
pub struct DenySubjectRecord {
    /// Subject controlled by this deny-list record.
    pub subject: Pubkey,
    /// Whether `subject` is currently denied for grant-authority use.
    pub denied: bool,
    /// PDA bump for this deny-list account.
    pub bump: u8,
}

impl DenySubjectRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 1 + 1;
}
