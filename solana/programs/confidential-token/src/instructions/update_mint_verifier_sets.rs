//! Rotates token-scoped verifier sets for an existing confidential mint.

use super::*;

/// Accounts for updating the active verifier sets used by new token KMS requests.
#[derive(Accounts)]
#[event_cpi]
pub struct UpdateMintVerifierSets<'info> {
    /// Mint authority.
    pub authority: Signer<'info>,
    /// Confidential mint whose verifier-set pointers are updated.
    #[account(mut)]
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Active threshold verifier set for future disclosure requests.
    pub disclosure_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
    /// Active threshold verifier set for future burn-redemption requests.
    pub redemption_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
}

/// Updates the mint's active verifier-set pointers for future request witnesses.
///
/// Existing pending requests remain bound to the verifier set stored in their
/// request witness. Disabling an old verifier set still makes old requests fail
/// closed because response instructions require the request-bound set to be active.
pub fn update_mint_verifier_sets(ctx: Context<UpdateMintVerifierSets>) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require_keys_eq!(
        ctx.accounts.mint.authority,
        ctx.accounts.authority.key(),
        ConfidentialTokenError::MintAuthorityMismatch
    );
    let mint_key = ctx.accounts.mint.key();
    assert_active_verifier_set(
        &ctx.accounts.disclosure_verifier_set,
        ctx.accounts.disclosure_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
        mint_key,
    )?;
    assert_active_verifier_set(
        &ctx.accounts.redemption_verifier_set,
        ctx.accounts.redemption_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_REDEMPTION,
        mint_key,
    )?;

    let old_disclosure_verifier_set = ctx.accounts.mint.disclosure_verifier_set;
    let old_redemption_verifier_set = ctx.accounts.mint.redemption_verifier_set;
    ctx.accounts.mint.disclosure_verifier_set = ctx.accounts.disclosure_verifier_set.key();
    ctx.accounts.mint.redemption_verifier_set = ctx.accounts.redemption_verifier_set.key();
    let updated_slot = Clock::get()?.slot;

    emit_cpi!(MintVerifierSetsUpdatedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        authority: ctx.accounts.authority.key(),
        old_disclosure_verifier_set,
        new_disclosure_verifier_set: ctx.accounts.disclosure_verifier_set.key(),
        old_redemption_verifier_set,
        new_redemption_verifier_set: ctx.accounts.redemption_verifier_set.key(),
        disclosure_verifier_set_version: ctx.accounts.disclosure_verifier_set.version,
        redemption_verifier_set_version: ctx.accounts.redemption_verifier_set.version,
        updated_slot,
    });
    Ok(())
}
