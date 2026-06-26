//! Prepares receiver-callback settlement and computes encrypted refund state.

use super::*;

/// Accounts for preparing receiver callback settlement and debiting any refund.
#[derive(Accounts)]
#[event_cpi]
#[instruction(sent_handle: [u8; 32])]
pub struct ConfidentialPrepareTransferCallback<'info> {
    /// Rent payer for callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: receiver-side authority key that produced the encrypted callback result.
    ///
    /// This authority is already enforced by the callback-success ACL record; it
    /// does not sign settlement so a failed callback can be refunded without
    /// recipient cooperation after the hook.
    pub callback_authority: UncheckedAccount<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account.
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; pays any best-effort refund in this prepare step.
    #[account(mut)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// ACL record for the encrypted callback success bit.
    pub callback_success_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Verified receiver-hook invocation for this sent amount.
    #[account(
        seeds = [b"transfer-hook", mint.key().as_ref(), sent_handle.as_ref()],
        bump = hook_record.bump
    )]
    pub hook_record: Account<'info, TransferReceiverHookCall>,
    /// Replay marker for this callback settlement.
    #[account(
        init,
        payer = payer,
        space = 8 + TransferCallbackSettlement::SPACE,
        seeds = [b"transfer-callback", mint.key().as_ref(), sent_handle.as_ref()],
        bump
    )]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// CHECK: recipient balance encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub to_balance_value_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub refund_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL and replay-marker account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialPrepareTransferCallback<'info> {
    pub(crate) fn as_prepare_callback_accounts(
        &mut self,
    ) -> PrepareTransferCallbackAccounts<'_, 'info> {
        PrepareTransferCallbackAccounts {
            payer: &self.payer,
            callback_authority: &self.callback_authority,
            mint: &self.mint,
            from_account: self.from_account.as_ref(),
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            sent_amount_acl: &self.sent_amount_acl,
            callback_success_acl: &self.callback_success_acl,
            hook_record: &self.hook_record,
            settlement_record: &mut self.settlement_record,
            to_balance_value_acl: self.to_balance_value_acl.to_account_info(),
            refund_amount_acl: self.refund_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Prepares receiver callback settlement by computing the encrypted refund.
pub fn confidential_prepare_transfer_callback(
    ctx: Context<ConfidentialPrepareTransferCallback>,
    sent_handle: [u8; 32],
    callback_success_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let outcome = prepare_transfer_callback_settlement(
        ctx.accounts.as_prepare_callback_accounts(),
        ctx.bumps.compute_signer,
        ctx.bumps.settlement_record,
        sent_handle,
        callback_success_handle,
    )?;
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.to_owner,
        token_account: outcome.to_token_account,
        old_handle: outcome.old_to_handle,
        new_handle: outcome.new_to_handle,
        reason: BalanceHandleUpdateReason::TransferCallbackRefundDebit,
    });
    Ok(())
}
