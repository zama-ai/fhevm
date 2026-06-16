//! On-chain account data for `TransferReceiverHookCall`.

use anchor_lang::prelude::*;

/// One-shot marker for a receiver hook call.
#[account]
#[derive(InitSpace)]
pub struct TransferReceiverHookCall {
    /// Confidential mint whose transfer reached the receiver hook.
    pub mint: Pubkey,
    /// Original sender token account.
    pub from_token_account: Pubkey,
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
    /// Receiver hook program that returned the callback witness.
    pub receiver_program: Pubkey,
    /// Sender that invoked the hook.
    pub caller: Pubkey,
    /// PDA bump for `(mint, sent_handle)`.
    pub bump: u8,
}

impl TransferReceiverHookCall {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = (32 * 9) + 1;
}
