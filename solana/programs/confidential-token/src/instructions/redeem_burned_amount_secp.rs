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
    /// Burned amount ACL record whose handle is redeemed.
    pub burned_amount_acl: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the burned handle.
    pub burned_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Replay marker for this burned handle.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// Host config carrying the gateway KMS verifier params + active context id.
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context the certificate is bound to, resolved from `extra_data` (EVM `_extractContextId`
    /// parity) and verified in-handler against the canonical PDA — not assumed to be the current
    /// context, so a cert minted under one context cannot be verified under another after rotation.
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
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
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

    // Gateway-compatible KMS cert check: verify the KMS PublicDecryptVerification EIP-712
    // certificate on-chain via secp256k1 against the active KMS context signer set.
    let config = &ctx.accounts.host_config;
    let kms_context = &ctx.accounts.kms_context;
    require!(
        config.decryption_contract != [0u8; 20] && config.current_kms_context_id != 0,
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );
    require!(
        !kms_context.destroyed,
        ConfidentialTokenError::InvalidKmsContext
    );
    // Resolve the context the certificate is bound to from extra_data (which the KMS signs over)
    // and verify the passed kms_context is the canonical PDA for THAT context, not the current one.
    // This stops a cert minted under context N from being verified under a rotated context N+1.
    let expected_context_id =
        zama_host::eip712::extract_kms_context_id(&extra_data, config.current_kms_context_id)
            .ok_or(ConfidentialTokenError::InvalidKmsContext)?;
    require!(
        kms_context.context_id == expected_context_id
            && kms_context.key() == zama_host::kms_context_address(expected_context_id).0,
        ConfidentialTokenError::InvalidKmsContext
    );
    let verifier = zama_host::eip712::Eip712VerifierConfig {
        gateway_chain_id: config.gateway_chain_id,
        verifying_contract: config.decryption_contract,
        signers: &kms_context.signers,
        threshold: kms_context.thresholds.public_decryption,
    };
    require!(
        zama_host::eip712::verify_kms_public_decrypt(
            &verifier,
            &[burned_handle],
            &kms_decrypted_result_bytes(cleartext_amount),
            &extra_data,
            &signatures,
        ),
        ConfidentialTokenError::InvalidKmsCertificate
    );

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

    emit_cpi!(BurnRedeemedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.owner.key(),
        token_account: token_account_key,
        burned_handle,
        burned_acl_record: ctx.accounts.burned_amount_acl.key(),
        destination_usdc: ctx.accounts.destination_usdc.key(),
        cleartext_amount,
    });
    Ok(())
}
