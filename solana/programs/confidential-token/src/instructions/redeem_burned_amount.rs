use super::*;

/// Redeems a previously burned encrypted amount from the underlying-token vault.
pub fn redeem_burned_amount(
    ctx: Context<RedeemBurnedAmount>,
    burned_handle: [u8; 32],
    cleartext_amount: u64,
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
    assert_disclosure_signature(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        ctx.accounts.mint.kms_verifier_authority,
        mint_key,
        burned_handle,
        cleartext_amount,
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
