//! On-chain account data for `ConfidentialOperator`.

use anchor_lang::prelude::*;

/// Operator authorization for one confidential token account.
#[account]
pub struct ConfidentialOperator {
    /// Token account whose balance may be transferred by the operator.
    pub token_account: Pubkey,
    /// Token account owner that created the authorization.
    pub owner: Pubkey,
    /// Operator signer allowed until `expiration_slot`.
    pub operator: Pubkey,
    /// Last slot in which the operator remains active. Zero revokes the row.
    pub expiration_slot: u64,
    /// PDA bump for `(token_account, operator)`.
    pub bump: u8,
}

impl ConfidentialOperator {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 1;
}
