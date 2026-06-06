//! Sample receiver hook program for the confidential token PoC.
//!
//! This program exists to keep the transfer-and-call return ABI honest: it is
//! not the token program, and it uses the receiver SDK helper exactly as an
//! external receiver would.

// Anchor macros generate framework-shaped code that trips rustc/Clippy checks.
#![allow(unexpected_cfgs)]
#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

use anchor_lang::prelude::*;
use confidential_token_receiver_sdk::{set_transfer_receiver_return_data, TransferReceiverReturn};

declare_id!("55pb9LouMGGfFMLkRWCLEkk6cZMP34oFgHhmsfSMAkpK");

#[program]
pub mod confidential_token_receiver {
    use super::*;

    /// Accepts a confidential transfer hook by returning the exact callback witness.
    pub fn accept_confidential_transfer(
        ctx: Context<AcceptConfidentialTransfer>,
        mint: Pubkey,
        from_token_account: Pubkey,
        to_token_account: Pubkey,
        sent_handle: [u8; 32],
        sent_acl_record: Pubkey,
        callback_success_handle: [u8; 32],
        callback_success_acl_record: Pubkey,
    ) -> Result<()> {
        require!(
            ctx.remaining_accounts.is_empty(),
            ReceiverError::UnexpectedRemainingAccounts
        );
        set_transfer_receiver_return_data(&TransferReceiverReturn {
            mint,
            from_token_account,
            to_token_account,
            sent_handle,
            sent_acl_record,
            callback_success_handle,
            callback_success_acl_record,
        });
        Ok(())
    }
}

/// Empty account set for the sample receiver hook.
#[derive(Accounts)]
pub struct AcceptConfidentialTransfer {}

#[error_code]
pub enum ReceiverError {
    /// The receiver hook was called with accounts not declared by this sample ABI.
    #[msg("receiver hook has unexpected remaining accounts")]
    UnexpectedRemainingAccounts,
}
