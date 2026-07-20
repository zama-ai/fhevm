//! Redeems a KMS-certified burned amount from the SPL vault through the stateless host verifier.
//!
//! This is the whole burn-redemption path after the burn-redemption request-witness lifecycle was
//! dissolved (fhevm-internal#1763, DD-040). It mirrors `disclose_secp`: the redeemer brings the KMS
//! `PublicDecryptVerification` certificate plus an MMR public-leaf inclusion proof in its own
//! transaction, CPIs the stateless `zama_host::verify_public_decrypt`, and asserts the handle the
//! host proved public equals the `burned_handle` it pinned and that the certified cleartext equals
//! the claimed `cleartext_amount`. There is no request witness, no request-time KMS context pin, and
//! no expiry: the certificate is verified against the host's CURRENT `KmsContext` (context rotation
//! fails closed one layer down in the host verifier).
//!
//! ## Act-once IS enforced here
//!
//! Unlike disclosure, redemption moves real value, so it cannot be idempotent: the per-handle
//! write-once, never-closed `burn-redemption` marker PDA is the single durable "paid out" bit. A
//! second redeem of the same burned handle fails when Anchor tries to `init` the already-initialized
//! marker.
//!
//! ## Deny check at payout
//!
//! The denied-subject check is explicit here (fhevm-internal#1763): a denied signer cannot cash out.
//! When the host grant deny-list is enabled the redeemer must pass the canonical `deny_subject_record`
//! for the signer; a record marking the signer denied rejects the redemption. This replaces the
//! request-time no-op `allow_subjects` CPI the dissolved witness relied on.

use super::*;

/// Accounts for redeeming a KMS-certified burned amount via the stateless host verifier.
#[derive(Accounts)]
#[instruction(burned_handle: [u8; 32], cleartext_amount: u64)]
#[event_cpi]
pub struct RedeemBurnedAmount<'info> {
    /// Token owner, redemption recipient, and payer for the replay marker.
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
    /// Signer's destination USDC token account (any SPL account of the right mint owned by the
    /// signer, not necessarily the ATA).
    #[account(
        mut,
        constraint = destination_usdc.mint == underlying_mint.key() @ ConfidentialTokenError::UnderlyingMintMismatch,
        constraint = destination_usdc.owner == owner.key() @ ConfidentialTokenError::OwnerMismatch
    )]
    pub destination_usdc: Box<Account<'info, TokenAccount>>,
    /// CHECK: PDA authority for the underlying-token vault.
    #[account(seeds = [b"vault-authority", mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    /// Burned amount `EncryptedValue` lineage whose handle is redeemed. Bound to the mint/token
    /// account/owner by `assert_burned_amount_lineage`; its canonical PDA, layout, host ownership,
    /// and the exact-handle MMR inclusion proof are validated by the `verify_public_decrypt` CPI.
    pub burned_amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
    /// Replay marker for this burned handle: write-once, never closed, the sole durable "paid out"
    /// bit for `(mint, burned_handle)`.
    #[account(
        init,
        payer = owner,
        space = 8 + BurnRedemption::SPACE,
        seeds = [b"burn-redemption", mint.key().as_ref(), burned_handle.as_ref()],
        bump
    )]
    pub redemption_record: Account<'info, BurnRedemption>,
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context PDA for the host's current context id (validated by the verifier CPI).
    pub kms_context: Box<Account<'info, zama_host::KmsContext>>,
    /// CHECK: canonical deny-list record for the signer when the host grant deny-list is enabled;
    /// consulted read-only by `assert_redeem_subject_not_denied`. Absent (must be `None`) when the
    /// deny-list is disabled.
    pub deny_subject_record: Option<UncheckedAccount<'info>>,
    /// ZamaHost program used for the stateless verifier CPI.
    pub zama_program: Program<'info, ZamaHost>,
    /// SPL token program.
    pub token_program: Program<'info, Token>,
    /// System program used for the replay marker.
    pub system_program: Program<'info, System>,
}

/// Redeems a previously burned encrypted amount from the underlying-token vault after the host
/// verifier certifies the burned handle's cleartext against the current KMS context.
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

    // Lineage binding: the burned handle need not be current. The burn already made it publicly
    // decryptable (DD-036 / Vector 2), so a historical handle superseded by a later burn stays
    // redeemable; the exact-handle MMR public-decrypt proof is checked inside the verifier CPI.
    assert_burned_amount_lineage(
        &ctx.accounts.burned_amount_value,
        burned_handle,
        mint_key,
        token_account_key,
        ctx.accounts.owner.key(),
        ctx.accounts.mint.compute_signer,
    )?;

    // Explicit deny-list check at payout: a denied signer cannot cash out.
    assert_redeem_subject_not_denied(
        &ctx.accounts.host_config,
        ctx.accounts.owner.key(),
        ctx.accounts.deny_subject_record.as_ref(),
    )?;

    // Verify the KMS certificate against the CURRENT KMS context plus the exact-handle MMR proof.
    // The wrapper asserts the returned handle equals `burned_handle`; we additionally require the
    // certified cleartext to equal the claimed `cleartext_amount`.
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

    let redemption = &mut ctx.accounts.redemption_record;
    redemption.mint = mint_key;
    redemption.owner = ctx.accounts.owner.key();
    redemption.token_account = token_account_key;
    redemption.burned_handle = burned_handle;
    redemption.burned_encrypted_value = ctx.accounts.burned_amount_value.key();
    redemption.cleartext_amount = cleartext_amount;
    redemption.bump = ctx.bumps.redemption_record;

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
