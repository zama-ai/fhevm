use super::*;

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
