//! Requests public disclosure for token-scoped encrypted amounts.

use super::*;

/// Accounts for requesting public disclosure of a token-scoped encrypted amount.
#[derive(Accounts)]
#[instruction(amount_handle: [u8; 32], request_nonce: [u8; 32], expires_slot: u64)]
#[event_cpi]
pub struct RequestDiscloseAmount<'info> {
    /// Requester that must have `ACL_ROLE_PUBLIC_DECRYPT` on the amount ACL.
    #[account(mut)]
    pub requester: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Account-backed request witness consumed by the KMS response path.
    #[account(
        init,
        payer = requester,
        space = 8 + DisclosureRequest::SPACE,
        seeds = [
            b"disclosure-request",
            mint.key().as_ref(),
            requester.key().as_ref(),
            amount_handle.as_ref(),
            request_nonce.as_ref()
        ],
        bump
    )]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
    /// Threshold verifier set expected to certify the disclosure response.
    pub disclosure_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
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
    /// System program used for request witness creation.
    pub system_program: Program<'info, System>,
}

/// Requests public disclosure for any token-scoped encrypted amount handle.
pub fn request_disclose_amount(
    ctx: Context<RequestDiscloseAmount>,
    amount_handle: [u8; 32],
    request_nonce: [u8; 32],
    expires_slot: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    let clock = Clock::get()?;
    require!(
        expires_slot >= clock.slot,
        ConfidentialTokenError::RequestWitnessUnavailable
    );
    assert_token_amount_acl(
        &ctx.accounts.amount_acl_record,
        amount_handle,
        ctx.accounts.mint.key(),
        ctx.accounts.mint.compute_signer,
    )?;
    assert_material_commitment(
        &ctx.accounts.amount_material_commitment,
        ctx.accounts.amount_material_commitment.key(),
        &ctx.accounts.amount_acl_record,
        amount_handle,
    )?;
    require_keys_eq!(
        ctx.accounts.mint.disclosure_verifier_set,
        ctx.accounts.disclosure_verifier_set.key(),
        ConfidentialTokenError::VerifierSetMismatch
    );
    assert_active_verifier_set(
        &ctx.accounts.disclosure_verifier_set,
        ctx.accounts.disclosure_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
        ctx.accounts.mint.key(),
    )?;

    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = disclosure_request_hash(
        crate::ID,
        request_key,
        ctx.accounts.mint.key(),
        ctx.accounts.requester.key(),
        Pubkey::default(),
        ctx.accounts.amount_acl_record.app_account,
        amount_handle,
        ctx.accounts.amount_acl_record.key(),
        ctx.accounts.amount_material_commitment.key(),
        ctx.accounts
            .amount_material_commitment
            .material_commitment_hash,
        ctx.accounts.amount_material_commitment.key_id,
        ctx.accounts.host_config.key(),
        ctx.accounts.disclosure_verifier_set.key(),
        ctx.accounts.disclosure_verifier_set.version,
        request_nonce,
        ctx.accounts.host_config.chain_id,
        expires_slot,
        DISCLOSURE_REQUEST_MODE_AMOUNT,
    );

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

    let request = &mut ctx.accounts.disclosure_request;
    request.mint = ctx.accounts.mint.key();
    request.requester = ctx.accounts.requester.key();
    request.token_account = Pubkey::default();
    request.app_account = ctx.accounts.amount_acl_record.app_account;
    request.handle = amount_handle;
    request.acl_record = ctx.accounts.amount_acl_record.key();
    request.material_commitment = ctx.accounts.amount_material_commitment.key();
    request.material_commitment_hash = ctx
        .accounts
        .amount_material_commitment
        .material_commitment_hash;
    request.material_key_id = ctx.accounts.amount_material_commitment.key_id;
    request.host_config = ctx.accounts.host_config.key();
    request.verifier_set = ctx.accounts.disclosure_verifier_set.key();
    request.verifier_set_version = ctx.accounts.disclosure_verifier_set.version;
    request.request_nonce = request_nonce;
    request.request_hash = request_hash;
    request.chain_id = ctx.accounts.host_config.chain_id;
    request.expires_slot = expires_slot;
    request.mode = DISCLOSURE_REQUEST_MODE_AMOUNT;
    request.status = REQUEST_STATUS_PENDING;
    request.bump = ctx.bumps.disclosure_request;

    emit_cpi!(AmountDisclosureRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        requester: ctx.accounts.requester.key(),
        handle: amount_handle,
        acl_record: ctx.accounts.amount_acl_record.key(),
        request: request_key,
        request_hash,
        verifier_set: ctx.accounts.disclosure_verifier_set.key(),
        verifier_set_version: ctx.accounts.disclosure_verifier_set.version,
        expires_slot,
    });
    Ok(())
}
