//! Redeems a KMS-certified burned amount from the SPL vault through the stateless host verifier.
//!
//! This is the whole burn-redemption path after the burn-redemption request-witness lifecycle was
//! dissolved (fhevm-internal#1763, DD-040). It mirrors `disclose_secp`: the redeemer brings the KMS
//! `PublicDecryptVerification` certificate plus an MMR public-leaf inclusion proof in its own
//! transaction, CPIs the stateless `zama_host::verify_public_decrypt`, and asserts the handle the
//! host proved public equals the `burned_handle` it pinned and that the certified cleartext equals
//! the claimed `cleartext_amount`. There is no request witness, no request-time KMS context pin, and
//! no expiry: the certificate is verified against the live `KmsContext` its signed extra_data names
//! (any non-destroyed context, fhevm-internal#1765; `destroy_kms_context` is the revocation lever,
//! one layer down in the host verifier).
//!
//! ## Act-once IS enforced here
//!
//! Unlike disclosure, redemption moves real value, so it cannot be idempotent. Act-once is
//! **open-at-burn + close-at-redeem/cancel**: each token account may have one `PendingBurn`;
//! redeem closes it after paying the underlying tokens (rent to the owner). A second redemption
//! fails because the pending account is gone. Cancel (`cancel_pending_burn`) is the alternate close
//! path. This is the reference shape for the act-once rule stated at
//! `zama_host::instructions::verify_public_decrypt` (INVARIANTS #24).

use super::*;

/// Accounts for redeeming a KMS-certified burned amount via the stateless host verifier.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmount<'info> {
    /// Token owner, redemption recipient, and rent destination for the closed pending burn.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// Confidential mint whose vault backs the redeemed burned amount.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account that produced the burned amount.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Underlying SPL mint.
    pub underlying_mint: Box<InterfaceAccount<'info, SplMint>>,
    /// Program vault USDC token account.
    #[account(
        mut,
        constraint = vault_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = vault_usdc.owner == vault_authority.key() @ ConfidentialTokenError::VaultAuthorityMismatch
    )]
    pub vault_usdc: Box<InterfaceAccount<'info, TokenAccount>>,
    /// Signer's destination USDC token account (any SPL account of the right mint owned by the
    /// signer, not necessarily the ATA).
    #[account(
        mut,
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Burned amount `EncryptedValue` encrypted value account whose handle is redeemed. Bound to the mint/token
    /// account/owner by `assert_burned_amount_value_account`; its canonical PDA, layout, host ownership,
    /// and the exact-handle MMR inclusion proof are validated by the `verify_public_decrypt` CPI.
    pub burned_amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Pending-burn account opened at burn time; closed on successful redemption.
    #[account(
        mut,
        close = owner,
        seeds = [
            PENDING_BURN_SEED,
            mint.key().as_ref(),
            token_account.key().as_ref()
        ],
        bump = pending_burn.bump,
    )]
    pub pending_burn: Account<'info, PendingBurn>,
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context PDA for the id the certificate commits to (any live context; validated by the
    /// verifier CPI).
    pub kms_context: Box<Account<'info, zama_host::KmsContext>>,
    /// ZamaHost program used for the stateless verifier CPI.
    pub zama_program: Program<'info, ZamaHost>,
    /// Classic Token or Token-2022 program owning the underlying mint and token accounts.
    pub token_program: Interface<'info, TokenInterface>,
}

/// Redeems a previously burned encrypted amount from the underlying-token vault after the host
/// verifier certifies the burned handle's cleartext against the live KMS context the cert names.
pub fn redeem_burned_amount(
    ctx: Context<RedeemBurnedAmount>,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: zama_host::instructions::MmrInclusionProof,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_supported_underlying_mint(&ctx.accounts.underlying_mint, &ctx.accounts.token_program)?;
    assert_supported_underlying_token_account(
        &ctx.accounts.vault_usdc,
        &ctx.accounts.token_program,
    )?;
    assert_supported_underlying_token_account(
        &ctx.accounts.destination_usdc,
        &ctx.accounts.token_program,
    )?;
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
        ctx.accounts.token_program.key(),
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

    let pending = &ctx.accounts.pending_burn;
    require_keys_eq!(
        pending.owner,
        ctx.accounts.owner.key(),
        ConfidentialTokenError::PendingBurnMismatch
    );
    require_keys_eq!(
        pending.mint,
        mint_key,
        ConfidentialTokenError::PendingBurnMismatch
    );
    require_keys_eq!(
        pending.token_account,
        token_account_key,
        ConfidentialTokenError::PendingBurnMismatch
    );
    require!(
        pending.burned_handle == burned_handle,
        ConfidentialTokenError::PendingBurnMismatch
    );
    require_keys_eq!(
        pending.burned_encrypted_value,
        ctx.accounts.burned_amount_value.key(),
        ConfidentialTokenError::PendingBurnMismatch
    );

    require!(
        ctx.accounts.burned_amount_value.current_handle == burned_handle,
        ConfidentialTokenError::PendingBurnHandleNotCurrent
    );
    // The sequential pending-burn invariant makes the burned handle current until redeem or cancel.
    // The exact-handle public-decrypt proof is checked inside the verifier CPI.
    assert_burned_amount_value_account(
        &ctx.accounts.burned_amount_value,
        burned_handle,
        mint_key,
        token_account_key,
        ctx.accounts.owner.key(),
        ctx.accounts.mint.compute_signer,
    )?;

    // Verify the KMS certificate against the context the cert names (any live, non-destroyed
    // context, EVM-parity rotation grace) plus the exact-handle MMR proof. The wrapper asserts the
    // returned handle equals `burned_handle`; we additionally require the certified cleartext to
    // equal the claimed `cleartext_amount`.
    let certified_cleartext = fhe::verify_public_decrypt(fhe::VerifyPublicDecrypt {
        expected_handle: burned_handle,
        cleartext: kms_decrypted_result_bytes(cleartext_amount),
        signatures,
        extra_data,
        proof,
        encrypted_value: ctx.accounts.burned_amount_value.to_account_info(),
        host_config: &ctx.accounts.host_config,
        kms_context: ctx.accounts.kms_context.to_account_info(),
        zama_program: &ctx.accounts.zama_program,
    })?;
    require!(
        certified_cleartext == kms_decrypted_result_bytes(cleartext_amount),
        ConfidentialTokenError::VerifierReturnDataInvalid
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

    // The pending account closes via Anchor `close = owner`.

    emit_cpi!(BurnRedeemedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.owner.key(),
        token_account: token_account_key,
        burned_handle,
        burned_encrypted_value: ctx.accounts.burned_amount_value.key(),
        destination_usdc: ctx.accounts.destination_usdc.key(),
        cleartext_amount,
    });
    Ok(())
}
