//! Publishes KMS-certified cleartexts for current confidential balances.

use super::*;

/// Accounts for disclosing a KMS-certified current balance cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseBalance<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record for the disclosed handle.
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub balance_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Emits a KMS-certified cleartext for the current balance handle.
pub fn disclose_balance(ctx: Context<DiscloseBalance>, cleartext_amount: u64) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        ctx.accounts.mint.key(),
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        ctx.accounts.mint.key(),
        ctx.accounts.token_account.owner,
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
    assert_public_decrypt_released(&ctx.accounts.balance_acl_record)?;
    assert_disclosure_signature(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        ctx.accounts.mint.kms_verifier_authority,
        ctx.accounts.mint.key(),
        handle,
        cleartext_amount,
    )?;

    emit_cpi!(BalanceDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        owner: ctx.accounts.token_account.owner,
        token_account: ctx.accounts.token_account.key(),
        handle,
        cleartext_amount,
    });
    Ok(())
}
