//! Transfers encrypted balances between confidential token accounts.

use super::*;

/// Accounts for confidential balance transfer.
#[derive(Accounts)]
#[event_cpi]
pub struct ConfidentialTransfer<'info> {
    /// Sender and transfer authority.
    pub owner: Signer<'info>,
    /// Pays rent for output ACL records.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Sender token account.
    #[account(mut)]
    pub from_account: Box<Account<'info, ConfidentialTokenAccount>>,
    // Anchor 1 rejects duplicate mutable Account<T> values unless the account opts in.
    // A self-transfer is a supported no-op, so from_account and to_account may be equal.
    #[account(mut, dup)]
    pub to_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// CHECK: Program-controlled compute signer PDA.
    #[account(seeds = [b"fhe-compute", mint.key().as_ref()], bump)]
    pub compute_signer: UncheckedAccount<'info>,
    /// Encrypted amount ACL record.
    pub amount_compute_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: sender balance encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub from_balance_value_acl: UncheckedAccount<'info>,
    /// CHECK: initialized and validated by the Zama host program CPI.
    #[account(mut)]
    pub transferred_amount_acl: UncheckedAccount<'info>,
    /// CHECK: recipient balance encrypted-value ACL lineage; rotated via the Zama host CPI.
    #[account(mut)]
    pub to_balance_value_acl: UncheckedAccount<'info>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used for FHE operations.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for handle derivation.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

impl<'info> ConfidentialTransfer<'info> {
    pub(crate) const OWNER_ACCOUNT_INDEX: usize = 0;
    pub(crate) const MINT_ACCOUNT_INDEX: usize = 2;
    pub(crate) const FROM_ACCOUNT_INDEX: usize = 3;
    pub(crate) const TO_ACCOUNT_INDEX: usize = 4;
    pub(crate) const TRANSFERRED_AMOUNT_ACL_INDEX: usize = 8;

    pub(crate) fn as_transfer_accounts(&mut self) -> TransferAccounts<'_, 'info> {
        TransferAccounts {
            payer: &self.payer,
            transfer_authority: self.owner.key(),
            mint: &self.mint,
            from_account: &mut self.from_account,
            to_account: &mut self.to_account,
            compute_signer: &self.compute_signer,
            amount_compute_acl: &self.amount_compute_acl,
            from_balance_value_acl: self.from_balance_value_acl.to_account_info(),
            transferred_amount_acl: self.transferred_amount_acl.to_account_info(),
            to_balance_value_acl: self.to_balance_value_acl.to_account_info(),
            zama_event_authority: &self.zama_event_authority,
            zama_program: &self.zama_program,
            host_config: &self.host_config,
            system_program: &self.system_program,
        }
    }
}

/// Transfers an encrypted amount by rotating the sender and recipient balance handles.
pub fn confidential_transfer(
    ctx: Context<ConfidentialTransfer>,
    amount_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    require_keys_eq!(
        ctx.accounts.from_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
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
            new_handle: outcome.new_from_handle,
            reason: BalanceHandleUpdateReason::TransferDebit,
        });
        emit_cpi!(BalanceHandleUpdatedEvent {
            version: APP_EVENT_VERSION,
            mint: outcome.mint,
            owner: outcome.to_owner,
            token_account: outcome.to_token_account,
            old_handle: outcome.old_to_handle,
            new_handle: outcome.new_to_handle,
            reason: BalanceHandleUpdateReason::TransferCredit,
        });
    }
    Ok(())
}
