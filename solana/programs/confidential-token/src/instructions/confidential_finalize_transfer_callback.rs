//! Finalizes receiver-callback settlement and applies any encrypted refund.

use super::*;

/// Accounts for finalizing a prepared callback settlement and crediting any refund.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialFinalizeTransferCallback<'info> {
    /// Rent payer for final callback-settlement output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Original sender token account; receives any best-effort refund in this finalize step.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Original recipient token account; must match the prepared settlement.
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// ACL record for the prior transfer's all-or-zero sent amount.
    pub sent_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Prepared callback settlement.
    #[account(mut)]
    pub settlement_record: Account<'info, TransferCallbackSettlement>,
    /// ACL record for the prepared refund amount.
    pub refund_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: sender balance encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub from_balance_value_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialFinalizeTransferCallback<'info> {
    pub(crate) fn as_finalize_callback_accounts(
        &mut self,
    ) -> FinalizeTransferCallbackAccounts<'_, 'info> {
        FinalizeTransferCallbackAccounts {
            payer: &self.payer,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: self.to_account.as_ref(),
            compute_signer: &self.compute_signer,
            sent_amount_acl: &self.sent_amount_acl,
            settlement_record: &mut self.settlement_record,
            refund_amount_acl: &self.refund_amount_acl,
            from_balance_value_acl: self.from_balance_value_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Finalizes a prepared callback settlement by crediting refund and recording final transfer.
pub fn confidential_finalize_transfer_callback(
    ctx: Context<ConfidentialFinalizeTransferCallback>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    let outcome = finalize_transfer_callback_settlement(
        ctx.accounts.as_finalize_callback_accounts(),
        ctx.bumps.compute_signer,
    )?;
    emit_cpi!(ConfidentialTransferEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        from_owner: outcome.to_owner,
        from_token_account: outcome.to_token_account,
        to_owner: outcome.from_owner,
        to_token_account: outcome.from_token_account,
        transferred_handle: outcome.refund_handle,
        transferred_acl_record: outcome.refund_acl_record,
    });
    emit_cpi!(BalanceHandleUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: outcome.mint,
        owner: outcome.from_owner,
        token_account: outcome.from_token_account,
        old_handle: outcome.old_from_handle,
        new_handle: outcome.new_from_handle,
        reason: BalanceHandleUpdateReason::TransferCallbackRefundCredit,
    });
    Ok(())
}
