//! Requests KMS certification for redeeming a burned encrypted amount.

use super::*;

/// Accounts for creating a burn-redemption request witness.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], request_nonce: [u8; 32], expires_slot: u64)]
#[event_cpi]
pub struct RequestBurnRedemption<'info> {
    /// Token owner and redemption authority.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose vault backs the redeemed burned amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account that produced the burned amount.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Owner's destination USDC token account.
    #[account(
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<Account<'info, TokenAccount>>,
    /// Stable burned-amount lineage whose handle will be redeemed. Escalated
    /// with `ACL_ROLE_PUBLIC_DECRYPT` for the owner and appended a
    /// public-decrypt MMR leaf by this instruction's CPIs.
    #[account(mut)]
    pub burned_amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Account-backed request witness consumed by the redemption path.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemptionRequest::SPACE,
        seeds = [
            b"burn-redemption-request",
            mint.key().as_ref(),
            owner.key().as_ref(),
            burned_handle.as_ref(),
            request_nonce.as_ref()
        ],
        bump
    )]
    pub redemption_request: Box<Account<'info, BurnRedemptionRequest>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// ZamaHost program used to update the burned-amount lineage.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for request witness creation and lineage growth.
    pub system_program: Program<'info, System>,
}

/// Creates a finalized-account witness for a future burn-redemption certificate.
pub fn request_burn_redemption(
    ctx: Context<RequestBurnRedemption>,
    burned_handle: [u8; 32],
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
    let mint_key = ctx.accounts.mint.key();
    let token_account_key = ctx.accounts.token_account.key();
    require_keys_eq!(
        ctx.accounts.mint.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::OwnerMismatch
    );
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        mint_key,
        ctx.accounts.owner.key(),
    )?;
    assert_burned_amount_encrypted_value(
        &ctx.accounts.burned_amount_value,
        burned_handle,
        mint_key,
        token_account_key,
        ctx.accounts.owner.key(),
        ctx.accounts.mint.compute_signer,
    )?;
    // Pin the request to the host's current KMS context; the redemption cert must verify against
    // this context (not a later rotated one) when the redemption is consumed.
    let kms_context_id = ctx.accounts.host_config.current_kms_context_id;
    require!(
        kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );

    let burned_encrypted_value = ctx.accounts.burned_amount_value.key();
    let request_key = ctx.accounts.redemption_request.key();
    let request_hash = burn_redemption_request_hash(
        crate::ID,
        request_key,
        mint_key,
        ctx.accounts.owner.key(),
        token_account_key,
        ctx.accounts.underlying_mint.key(),
        ctx.accounts.destination_usdc.owner,
        ctx.accounts.destination_usdc.key(),
        burned_handle,
        burned_encrypted_value,
        ctx.accounts.host_config.key(),
        kms_context_id,
        request_nonce,
        ctx.accounts.host_config.chain_id,
        expires_slot,
    );

    // Roles cannot be granted at birth, so escalate the owner to public-decrypt here, then
    // append the public-decrypt MMR leaf for the current handle.
    fhe::allow_subjects(
        fhe::AllowSubjects {
            payer: &ctx.accounts.owner,
            authority: &ctx.accounts.owner,
            encrypted_value: ctx.accounts.burned_amount_value.to_account_info(),
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
            subject: ctx.accounts.owner.key(),
            role_flags: zama_host::ACL_ROLE_PUBLIC_DECRYPT,
        }],
    )?;
    fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
        authority: &ctx.accounts.owner,
        payer: &ctx.accounts.owner,
        encrypted_value: ctx.accounts.burned_amount_value.to_account_info(),
        host_config: &ctx.accounts.host_config,
        deny_subject_record: ctx
            .accounts
            .deny_subject_record
            .as_ref()
            .map(|account| account.to_account_info()),
        zama_program: &ctx.accounts.zama_program,
        system_program: &ctx.accounts.system_program,
    })?;

    let request = &mut ctx.accounts.redemption_request;
    request.mint = mint_key;
    request.owner = ctx.accounts.owner.key();
    request.token_account = token_account_key;
    request.underlying_mint = ctx.accounts.underlying_mint.key();
    request.destination_owner = ctx.accounts.destination_usdc.owner;
    request.destination_account = ctx.accounts.destination_usdc.key();
    request.burned_handle = burned_handle;
    request.burned_encrypted_value = burned_encrypted_value;
    request.host_config = ctx.accounts.host_config.key();
    request.kms_context_id = kms_context_id;
    request.request_nonce = request_nonce;
    request.request_hash = request_hash;
    request.chain_id = ctx.accounts.host_config.chain_id;
    request.expires_slot = expires_slot;
    request.status = REQUEST_STATUS_PENDING;
    request.bump = ctx.bumps.redemption_request;

    emit_cpi!(BurnRedemptionRequestedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.owner.key(),
        token_account: token_account_key,
        burned_handle,
        burned_encrypted_value,
        destination_owner: ctx.accounts.destination_usdc.owner,
        destination_account: ctx.accounts.destination_usdc.key(),
        request: request_key,
        request_hash,
        kms_context_id,
        expires_slot,
    });
    Ok(())
}
