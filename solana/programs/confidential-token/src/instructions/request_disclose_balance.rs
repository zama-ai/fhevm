//! Requests public disclosure for confidential account balances.

use super::*;

/// Accounts for requesting public disclosure of the current balance handle.
#[derive(Accounts)]
#[instruction(request_nonce: [u8; 32], expires_slot: u64)]
#[event_cpi]
pub struct RequestDiscloseBalance<'info> {
    /// Token account owner and disclosure authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record. Updated by ZamaHost CPI.
    #[account(mut)]
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub balance_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Account-backed request witness consumed by the KMS response path.
    #[account(
        init,
        payer = owner,
        space = 8 + DisclosureRequest::SPACE,
        seeds = [
            b"disclosure-request",
            mint.key().as_ref(),
            owner.key().as_ref(),
            token_account.balance_handle.as_ref(),
            request_nonce.as_ref()
        ],
        bump
    )]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
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
    /// System program used for request witness creation.
    pub system_program: Program<'info, System>,
}

/// Requests public disclosure for the current confidential balance handle.
pub fn request_disclose_balance(
    ctx: Context<RequestDiscloseBalance>,
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
    assert_material_commitment(
        &ctx.accounts.balance_material_commitment,
        ctx.accounts.balance_material_commitment.key(),
        &ctx.accounts.balance_acl_record,
        handle,
    )?;
    // Pin the request to the host's current KMS context; the response cert must verify against
    // this context (not a later rotated one) when the disclosure is consumed.
    let kms_context_id = ctx.accounts.host_config.current_kms_context_id;
    require!(
        kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );

    let acl_record = ctx.accounts.balance_acl_record.key();
    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = disclosure_request_hash(
        crate::ID,
        request_key,
        ctx.accounts.mint.key(),
        ctx.accounts.owner.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.token_account.key(),
        handle,
        acl_record,
        ctx.accounts.balance_material_commitment.key(),
        ctx.accounts
            .balance_material_commitment
            .material_commitment_hash,
        ctx.accounts.balance_material_commitment.key_id,
        ctx.accounts.host_config.key(),
        kms_context_id,
        request_nonce,
        ctx.accounts.host_config.chain_id,
        expires_slot,
        DISCLOSURE_REQUEST_MODE_BALANCE,
    );
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
    let request = &mut ctx.accounts.disclosure_request;
    request.mint = ctx.accounts.mint.key();
    request.requester = ctx.accounts.owner.key();
    request.token_account = ctx.accounts.token_account.key();
    request.app_account = ctx.accounts.token_account.key();
    request.handle = handle;
    request.acl_record = acl_record;
    request.material_commitment = ctx.accounts.balance_material_commitment.key();
    request.material_commitment_hash = ctx
        .accounts
        .balance_material_commitment
        .material_commitment_hash;
    request.material_key_id = ctx.accounts.balance_material_commitment.key_id;
    request.host_config = ctx.accounts.host_config.key();
    request.kms_context_id = kms_context_id;
    request.request_nonce = request_nonce;
    request.request_hash = request_hash;
    request.chain_id = ctx.accounts.host_config.chain_id;
    request.expires_slot = expires_slot;
    request.mode = DISCLOSURE_REQUEST_MODE_BALANCE;
    request.status = REQUEST_STATUS_PENDING;
    request.bump = ctx.bumps.disclosure_request;
    emit_cpi!(BalanceDisclosureRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.owner.key(),
        token_account: ctx.accounts.token_account.key(),
        handle,
        acl_record,
        request: request_key,
        request_hash,
        kms_context_id,
        expires_slot,
    });
    Ok(())
}
