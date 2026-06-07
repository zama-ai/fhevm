//! Redeems KMS-certified burned encrypted amounts from the SPL vault.

use super::*;

/// Accounts for redeeming a KMS-certified burned amount from the SPL vault.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmount<'info> {
    /// Token owner and redemption recipient.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose vault backs the redeemed burned amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account that produced the burned amount.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<Account<'info, SplMint>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,
    /// Owner's destination USDC token account.
    #[account(
        mut,
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Burned amount ACL record whose handle is redeemed.
    pub burned_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the burned handle.
    pub burned_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Account-backed redemption request witness.
    #[account(mut)]
    pub redemption_request: Box<Account<'info, BurnRedemptionRequest>>,
    /// Threshold verifier set whose quorum signs the redemption certificate.
    pub redemption_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
    /// Replay marker for this burned handle.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
    /// ZamaHost config bound into the request and certificate.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for the replay marker.
    pub system_program: Program<'info, System>,
}

/// Redeems a previously burned encrypted amount from the underlying-token vault.
pub fn redeem_burned_amount(
    ctx: Context<RedeemBurnedAmount>,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_host_config_allows_token_response(&ctx.accounts.host_config)?;
    let mint_key = ctx.accounts.mint.key();
    let token_account_key = ctx.accounts.token_account.key();
    require_keys_eq!(
        ctx.accounts.mint.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        ConfidentialTokenError::UnderlyingMintMismatch
    );
    assert_canonical_vault_token_account(
        ctx.accounts.vault_usdc.key(),
        ctx.accounts.vault_authority.key(),
        ctx.accounts.underlying_mint.key(),
    )?;
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
    assert_burned_amount_acl(
        &ctx.accounts.burned_amount_acl,
        burned_handle,
        mint_key,
        token_account_key,
        ctx.accounts.owner.key(),
        ctx.accounts.mint.compute_signer,
    )?;
    assert_material_commitment(
        &ctx.accounts.burned_material_commitment,
        ctx.accounts.burned_material_commitment.key(),
        &ctx.accounts.burned_amount_acl,
        burned_handle,
    )?;
    assert_public_decrypt_released(&ctx.accounts.burned_amount_acl)?;
    assert_active_verifier_set(
        &ctx.accounts.redemption_verifier_set,
        ctx.accounts.redemption_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_REDEMPTION,
        mint_key,
    )?;
    assert_burn_redemption_request_witness(
        &ctx.accounts.redemption_request,
        ctx.accounts.redemption_request.key(),
        mint_key,
        ctx.accounts.owner.key(),
        token_account_key,
        ctx.accounts.underlying_mint.key(),
        ctx.accounts.destination_usdc.owner,
        ctx.accounts.destination_usdc.key(),
        burned_handle,
        ctx.accounts.burned_amount_acl.key(),
        &ctx.accounts.burned_material_commitment,
        ctx.accounts.host_config.key(),
        &ctx.accounts.redemption_verifier_set,
    )?;
    let message = redemption_proof_message_v2(
        crate::ID,
        zama_host::ID,
        ctx.accounts.host_config.key(),
        ctx.accounts.redemption_request.chain_id,
        mint_key,
        ctx.accounts.redemption_verifier_set.key(),
        ctx.accounts.redemption_verifier_set.version,
        ctx.accounts.redemption_request.key(),
        ctx.accounts.redemption_request.request_hash,
        ctx.accounts.burned_amount_acl.key(),
        ctx.accounts
            .burned_material_commitment
            .material_commitment_hash,
        ctx.accounts.burned_material_commitment.key_id,
        burned_handle,
        cleartext_amount,
        ctx.accounts.owner.key(),
        token_account_key,
        ctx.accounts.underlying_mint.key(),
        ctx.accounts.destination_usdc.owner,
        ctx.accounts.destination_usdc.key(),
    );
    assert_threshold_verifier_signature(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        &ctx.accounts.redemption_verifier_set,
        &message,
    )?;

    let vault_authority_bump = [ctx.bumps.vault_authority];
    let vault_authority_seeds: &[&[u8]] =
        &[b"vault-authority", mint_key.as_ref(), &vault_authority_bump];
    spl_token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault_usdc.to_account_info(),
                mint: ctx.accounts.underlying_mint.to_account_info(),
                to: ctx.accounts.destination_usdc.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            &[vault_authority_seeds],
        ),
        cleartext_amount,
        ctx.accounts.mint.decimals,
    )?;

    let redemption = &mut ctx.accounts.redemption_record;
    redemption.mint = mint_key;
    redemption.owner = ctx.accounts.owner.key();
    redemption.token_account = token_account_key;
    redemption.burned_handle = burned_handle;
    redemption.burned_acl_record = ctx.accounts.burned_amount_acl.key();
    redemption.cleartext_amount = cleartext_amount;
    redemption.bump = ctx.bumps.redemption_record;
    ctx.accounts.redemption_request.status = REQUEST_STATUS_CONSUMED;

    emit_cpi!(BurnRedeemedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.owner.key(),
        token_account: token_account_key,
        burned_handle,
        burned_acl_record: ctx.accounts.burned_amount_acl.key(),
        destination_usdc: ctx.accounts.destination_usdc.key(),
        request: ctx.accounts.redemption_request.key(),
        request_hash: ctx.accounts.redemption_request.request_hash,
        cleartext_amount,
    });
    Ok(())
}
