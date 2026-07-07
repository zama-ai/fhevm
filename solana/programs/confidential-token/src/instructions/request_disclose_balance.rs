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
    /// Stable balance lineage. The owner must be allowed so this instruction
    /// can append a public-decrypt MMR leaf.
    #[account(mut, address = token_account.balance_encrypted_value)]
    pub balance_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Account-backed request witness consumed by the KMS response path.
    #[account(
        init,
        payer = owner,
        space = 8 + DisclosureRequest::SPACE,
        seeds = [
            b"disclosure-request",
            mint.key().as_ref(),
            owner.key().as_ref(),
            balance_value.current_handle.as_ref(),
            request_nonce.as_ref()
        ],
        bump
    )]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
    /// CHECK: optional deny-list witness when host deny-lists are enabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// ZamaHost program used to update the balance lineage.
    pub zama_program: Program<'info, ZamaHost>,
    /// ZamaHost config used for pause and deny-list checks.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// System program used for request witness creation and lineage growth.
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
    assert_current_balance_encrypted_value(
        &ctx.accounts.balance_value,
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
    )?;
    let handle = ctx.accounts.balance_value.current_handle;
    // Pin the request to the host's current KMS context; the response cert must verify against
    // this context (not a later rotated one) when the disclosure is consumed.
    let kms_context_id = ctx.accounts.host_config.current_kms_context_id;
    require!(
        kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );

    let encrypted_value = ctx.accounts.balance_value.key();
    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = disclosure_request_hash(
        crate::ID,
        request_key,
        ctx.accounts.mint.key(),
        ctx.accounts.owner.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.token_account.key(),
        handle,
        encrypted_value,
        ctx.accounts.host_config.key(),
        kms_context_id,
        request_nonce,
        ctx.accounts.host_config.chain_id,
        expires_slot,
        DISCLOSURE_REQUEST_MODE_BALANCE,
    );

    // Re-add the owner idempotently, then append the public-decrypt MMR leaf for
    // the current handle.
    fhe::allow_subjects(
        fhe::AllowSubjects {
            payer: &ctx.accounts.owner,
            authority: &ctx.accounts.owner,
            encrypted_value: ctx.accounts.balance_value.to_account_info(),
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
        }],
    )?;
    fhe::allow_public_decrypt(fhe::AllowPublicDecrypt {
        authority: &ctx.accounts.owner,
        payer: &ctx.accounts.owner,
        handle,
        encrypted_value: ctx.accounts.balance_value.to_account_info(),
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
    request.requester = ctx.accounts.owner.key();
    request.token_account = ctx.accounts.token_account.key();
    request.app_account = ctx.accounts.token_account.key();
    request.handle = handle;
    request.encrypted_value = encrypted_value;
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
        encrypted_value,
        request: request_key,
        request_hash,
        kms_context_id,
        expires_slot,
    });
    Ok(())
}
