//! Requests public disclosure for token-scoped encrypted amounts.

use super::*;

/// Accounts for requesting public disclosure of a token-scoped encrypted amount.
#[derive(Accounts)]
#[instruction(amount_handle: [u8; 32], request_nonce: [u8; 32], expires_slot: u64)]
#[event_cpi]
pub struct RequestDiscloseAmount<'info> {
    /// Requester that must have `ACL_ROLE_GRANT` on the amount lineage.
    #[account(mut)]
    pub requester: Signer<'info>,
    /// Confidential mint that scopes the encrypted amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount `EncryptedValue` lineage. Escalated with
    /// `ACL_ROLE_PUBLIC_DECRYPT` for the requester and appended a
    /// public-decrypt MMR leaf by this instruction's CPIs.
    #[account(mut)]
    pub amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
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
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// ZamaHost program used to update the amount lineage.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for request witness creation and lineage growth.
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
    assert_token_amount_encrypted_value(
        &ctx.accounts.amount_value,
        amount_handle,
        ctx.accounts.mint.key(),
        ctx.accounts.mint.compute_signer,
    )?;
    // Pin the request to the host's current KMS context; the response cert must verify against
    // this context (not a later rotated one) when the disclosure is consumed.
    let kms_context_id = ctx.accounts.host_config.current_kms_context_id;
    require!(
        kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );

    let encrypted_value = ctx.accounts.amount_value.key();
    let app_account = ctx.accounts.amount_value.app_account;
    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = disclosure_request_hash(
        crate::ID,
        request_key,
        ctx.accounts.mint.key(),
        ctx.accounts.requester.key(),
        Pubkey::default(),
        app_account,
        amount_handle,
        encrypted_value,
        ctx.accounts.host_config.key(),
        kms_context_id,
        request_nonce,
        ctx.accounts.host_config.chain_id,
        expires_slot,
        DISCLOSURE_REQUEST_MODE_AMOUNT,
    );

    // Roles cannot be granted at birth, so escalate the requester to public-decrypt here, then
    // append the public-decrypt MMR leaf for the current handle.
    fhe::allow_subjects(
        fhe::AllowSubjects {
            payer: &ctx.accounts.requester,
            authority: &ctx.accounts.requester,
            encrypted_value: ctx.accounts.amount_value.to_account_info(),
            host_config: &ctx.accounts.host_config,
            deny_subject_record: ctx
                .accounts
                .deny_subject_record
                .as_ref()
                .map(|account| account.to_account_info()),
            zama_program: &ctx.accounts.zama_program,
            system_program: &ctx.accounts.system_program,
        },
        vec![zama_host::instructions::EncryptedValueSubjectGrant {
            subject: ctx.accounts.requester.key(),
            role_flags: zama_host::ACL_ROLE_PUBLIC_DECRYPT,
        }],
    )?;
    fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
        authority: &ctx.accounts.requester,
        payer: &ctx.accounts.requester,
        encrypted_value: ctx.accounts.amount_value.to_account_info(),
        host_config: &ctx.accounts.host_config,
        deny_subject_record: ctx
            .accounts
            .deny_subject_record
            .as_ref()
            .map(|account| account.to_account_info()),
        zama_program: &ctx.accounts.zama_program,
        system_program: &ctx.accounts.system_program,
    })?;

    let request = &mut ctx.accounts.disclosure_request;
    request.mint = ctx.accounts.mint.key();
    request.requester = ctx.accounts.requester.key();
    request.token_account = Pubkey::default();
    request.app_account = app_account;
    request.handle = amount_handle;
    request.encrypted_value = encrypted_value;
    request.host_config = ctx.accounts.host_config.key();
    request.kms_context_id = kms_context_id;
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
        encrypted_value,
        request: request_key,
        request_hash,
        kms_context_id,
        expires_slot,
    });
    Ok(())
}
