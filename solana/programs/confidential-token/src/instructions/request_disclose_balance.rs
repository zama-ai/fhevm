use super::*;

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
