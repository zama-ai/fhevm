//! Requests public disclosure for confidential account balances.

use super::*;

/// Accounts for requesting public disclosure of the current balance handle.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseBalance<'info> {
    /// Token account owner and disclosure authority.
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the owner authority.
    pub authority_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// CHECK: Anchor event CPI authority for the Zama host program.
    pub zama_event_authority: UncheckedAccount<'info>,
    /// ZamaHost program used to update the ACL record.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
}

/// Requests public disclosure for the current confidential balance handle.
pub fn request_disclose_balance(ctx: Context<RequestDiscloseBalance>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require_keys_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        ctx.accounts.mint.key(),
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
        ctx.accounts.owner.key(),
    )?;
    assert_current_balance_acl(
        &ctx.accounts.balance_acl_record,
        ctx.accounts.balance_acl_record.key(),
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
    )?;

    let handle = ctx.accounts.token_account.balance_handle;
    let acl_record = ctx.accounts.balance_acl_record.key();
    fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
        authority: &ctx.accounts.owner,
        authority_permission_record: ctx
            .accounts
            .authority_permission_record
            .as_ref()
            .map(|account| account.to_account_info()),
        acl_record: ctx.accounts.balance_acl_record.to_account_info(),
        host_config: &ctx.accounts.host_config,
        deny_subject_record: ctx
            .accounts
            .deny_subject_record
            .as_ref()
            .map(|account| account.to_account_info()),
        event_authority: &ctx.accounts.zama_event_authority,
        zama_program: &ctx.accounts.zama_program,
        handle,
    })?;
    emit_cpi!(BalanceDisclosureRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: ctx.accounts.token_account.key(),
        handle,
        acl_record,
    });
    Ok(())
}
