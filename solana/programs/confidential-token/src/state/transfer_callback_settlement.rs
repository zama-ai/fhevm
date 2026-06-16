//! On-chain account data for `TransferCallbackSettlement`.

use anchor_lang::prelude::*;

/// Replay marker for a transfer callback settlement.
#[account]
#[derive(InitSpace)]
pub struct TransferCallbackSettlement {
    /// Confidential mint whose transfer was settled.
    pub mint: Pubkey,
    /// Original sender owner.
    pub from_owner: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
    /// Original recipient owner.
    pub to_owner: Pubkey,
    /// Original recipient token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Encrypted receiver callback success bit.
    pub callback_success_handle: [u8; 32],
    /// ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
    /// Encrypted amount actually refunded.
    pub refund_handle: [u8; 32],
    /// ACL record for `refund_handle`.
    pub refund_acl_record: Pubkey,
    /// Recipient balance handle after the prepared refund debit.
    pub to_balance_handle: [u8; 32],
    /// ACL record for `to_balance_handle`.
    pub to_balance_acl_record: Pubkey,
    /// Sender balance handle after the finalized refund credit.
    pub from_balance_handle: [u8; 32],
    /// ACL record for `from_balance_handle`.
    pub from_balance_acl_record: Pubkey,
    /// Encrypted amount that remains transferred after refund.
    pub transferred_handle: [u8; 32],
    /// ACL record for `transferred_handle`.
    pub transferred_acl_record: Pubkey,
    /// Settlement lifecycle status.
    pub status: u8,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferCallbackSettlement {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 17) + 2;
}
