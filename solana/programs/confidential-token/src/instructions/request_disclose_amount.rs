//! Requests public disclosure for token-scoped encrypted amounts.

use super::*;

/// Accounts for requesting public disclosure of a token-scoped encrypted amount.
#[derive(Accounts)]
#[event_cpi]
pub struct RequestDiscloseAmount<'info> {
    /// Requester that must have `ACL_ROLE_PUBLIC_DECRYPT` on the amount ACL.
    pub requester: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// CHECK: optional overflow permission witness for the requester authority.
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

/// Requests public disclosure for any token-scoped encrypted amount handle.
pub fn request_disclose_amount(
    ctx: Context<RequestDiscloseAmount>,
    amount_handle: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_token_amount_acl(
        &ctx.accounts.amount_acl_record,
        amount_handle,
        ctx.accounts.mint.key(),
        ctx.accounts.mint.compute_signer,
    )?;

    fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
        authority: &ctx.accounts.requester,
        authority_permission_record: ctx
            .accounts
            .authority_permission_record
            .as_ref()
            .map(|account| account.to_account_info()),
        acl_record: ctx.accounts.amount_acl_record.to_account_info(),
        host_config: &ctx.accounts.host_config,
        deny_subject_record: ctx
            .accounts
            .deny_subject_record
            .as_ref()
            .map(|account| account.to_account_info()),
        event_authority: &ctx.accounts.zama_event_authority,
        zama_program: &ctx.accounts.zama_program,
        handle: amount_handle,
    })?;

    emit_cpi!(AmountDisclosureRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        requester: ctx.accounts.requester.key(),
        handle: amount_handle,
        acl_record: ctx.accounts.amount_acl_record.key(),
    });
    Ok(())
}
