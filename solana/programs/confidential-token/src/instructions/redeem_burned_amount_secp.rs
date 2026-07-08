//! Redeems KMS-certified burned encrypted amounts from the SPL vault, verifying the
//! KMS `PublicDecryptVerification` EIP-712 certificate on-chain via `secp256k1_recover`
//! (the gateway-compatible path, #1494 Phase 3 cert-secp).
//!
//! Mirrors `redeem_burned_amount` but trusts the gateway-level KMS context signer set
//! (EVM secp256k1 EIP-712) instead of the per-mint Ed25519 verifier — the same cert the
//! `disclose_*_secp` instructions verify. This is the secp256k1-parity counterpart of the
//! disclose path; the legacy Ed25519 redeem stays until the secp path is adopted.

use super::*;

/// Accounts for redeeming a KMS-certified burned amount via secp256k1 EIP-712.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmountSecp<'info> {
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
    /// Burned amount `EncryptedValue` lineage whose handle is redeemed.
    pub burned_amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Account-backed redemption request witness pinned to a KMS context id.
    #[account(mut)]
    pub redemption_request: Box<Account<'info, BurnRedemptionRequest>>,
    /// Replay marker for this burned handle.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// Host config carrying the gateway KMS verifier params.
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context the request was pinned to. Verified in-handler against the witness's
    /// `kms_context_id` (not the current context), so a cert minted under one context cannot be
    /// presented against a request pinned to another after rotation.
    pub kms_context: Box<Account<'info, zama_host::KmsContext>>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for the replay marker.
    pub system_program: Program<'info, System>,
}

/// Redeems a previously burned encrypted amount from the underlying-token vault after
/// on-chain secp256k1 verification of the KMS `PublicDecryptVerification` certificate.
pub fn redeem_burned_amount_secp(
    ctx: Context<RedeemBurnedAmountSecp>,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: MmrInclusionProof,
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
    // Authorize the pinned burned handle by MMR public-decrypt proof rather than
    // requiring it to still be the live handle, so a redemption survives a later
    // burn superseding the shared burned-amount lineage.
    let proof = zama_solana_acl::MmrProof::from(proof);
    authorize_burned_amount_redeem(
        &ctx.accounts.burned_amount_value,
        ctx.accounts.burned_amount_value.key(),
        burned_handle,
        &proof,
        mint_key,
        token_account_key,
        ctx.accounts.owner.key(),
        ctx.accounts.mint.compute_signer,
    )?;

    // Bind the redemption to a previously created request witness: same handle, accounts, host
    // config; still PENDING and not expired; recomputed request_hash matches.
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
        ctx.accounts.burned_amount_value.key(),
        ctx.accounts.host_config.key(),
    )?;
    // Verify the KMS PublicDecryptVerification secp256k1 cert against the context the witness was
    // pinned to at request time (not the current context), closing rotation reuse.
    assert_kms_public_decrypt_cert_for_request(
        &ctx.accounts.host_config,
        &ctx.accounts.kms_context,
        ctx.accounts.redemption_request.kms_context_id,
        burned_handle,
        cleartext_amount,
        &signatures,
        &extra_data,
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
    redemption.burned_encrypted_value = ctx.accounts.burned_amount_value.key();
    redemption.cleartext_amount = cleartext_amount;
    redemption.bump = ctx.bumps.redemption_record;
    ctx.accounts.redemption_request.status = REQUEST_STATUS_CONSUMED;

    emit_cpi!(BurnRedeemedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.owner.key(),
        token_account: token_account_key,
        burned_handle,
        burned_encrypted_value: ctx.accounts.burned_amount_value.key(),
        destination_usdc: ctx.accounts.destination_usdc.key(),
        request: ctx.accounts.redemption_request.key(),
        request_hash: ctx.accounts.redemption_request.request_hash,
        cleartext_amount,
    });
    Ok(())
}
