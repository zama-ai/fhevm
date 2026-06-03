//! Transfers encrypted balances through an active operator allowance.

use super::*;

/// Accounts for confidential operator transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransferFrom<'info> {
    /// Active operator and rent payer for output ACL records.
    #[account(mut)]
    pub operator: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account controlled by the operator row.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Operator authorization row for `(from_account, operator)`.
    #[account(
        seeds = [b"operator", from_account.key().as_ref(), operator.key().as_ref()],
        bump = operator_record.bump
    )]
    pub operator_record: Account<'info, ConfidentialOperator>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Sender current balance ACL record.
    pub from_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Recipient current balance ACL record.
    pub to_current_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transfer_success_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub debit_candidate_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub from_output_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub to_output_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialTransferFrom<'info> {
    pub(crate) const OPERATOR_ACCOUNT_INDEX: usize = 0;
    pub(crate) const MINT_ACCOUNT_INDEX: usize = 1;
    pub(crate) const FROM_ACCOUNT_INDEX: usize = 2;
    pub(crate) const TO_ACCOUNT_INDEX: usize = 3;
    pub(crate) const OPERATOR_RECORD_ACCOUNT_INDEX: usize = 4;
    pub(crate) const TRANSFERRED_AMOUNT_ACL_INDEX: usize = 12;

    pub(crate) fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.operator,
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            from_current_compute_acl: self.from_current_compute_acl.as_ref(),
            to_current_compute_acl: self.to_current_compute_acl.as_ref(),
            amount_compute_acl: &self.amount_compute_acl,
            transfer_success_acl: self.transfer_success_acl.to_account_info(),
            debit_candidate_acl: self.debit_candidate_acl.to_account_info(),
            from_output_acl: self.from_output_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_output_acl: self.to_output_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Transfers an encrypted amount from a holder through an active operator.
pub fn confidential_transfer_from(
    ctx: Context<ConfidentialTransferFrom>,
    amount_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_active_operator_record(
        &ctx.accounts.operator_record,
        &ctx.accounts.from_account,
        ctx.accounts.operator.key(),
    )?;
    let outcome = execute_transfer(
        ctx.accounts.as_transfer_accounts(),
        ctx.bumps.compute_signer,
        amount_handle,
    )?;
    if let Some(outcome) = outcome {
        emit_cpi!(ConfidentialTransferEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            from_owner: outcome.from_owner,
            from_token_account: outcome.from_token_account,
            to_owner: outcome.to_owner,
            to_token_account: outcome.to_token_account,
            transferred_handle: outcome.transferred_handle,
            transferred_acl_record: outcome.transferred_acl_record,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.from_owner,
            token_account: outcome.from_token_account,
            old_handle: outcome.old_from_handle,
            old_acl_record: outcome.old_from_acl_record,
            new_handle: outcome.new_from_handle,
            new_acl_record: outcome.new_from_acl_record,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            old_acl_record: outcome.old_to_acl_record,
            new_handle: outcome.new_to_handle,
            new_acl_record: outcome.new_to_acl_record,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}
