//! Publishes KMS-certified cleartexts for token-scoped encrypted amounts.

use super::*;

/// Accounts for disclosing a KMS-certified token-scoped amount cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmount<'info> {
    /// Confidential mint carrying the KMS verifier authority.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record for the disclosed handle.
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// CHECK: Solana instructions sysvar; handler verifies its address and previous Ed25519 ix.
    pub instructions_sysvar: UncheckedAccount<'info>,
}

/// Emits a KMS-certified cleartext for any token-scoped encrypted amount.
pub fn disclose_amount(
    ctx: Context<DiscloseAmount>,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_token_amount_acl(
        &ctx.accounts.amount_acl_record,
        amount_handle,
        ctx.accounts.mint.key(),
        ctx.accounts.mint.compute_signer,
    )?;
    assert_material_commitment(
        &ctx.accounts.amount_material_commitment,
        ctx.accounts.amount_material_commitment.key(),
        &ctx.accounts.amount_acl_record,
        amount_handle,
    )?;
    assert_public_decrypt_released(&ctx.accounts.amount_acl_record)?;
    assert_disclosure_signature(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        ctx.accounts.mint.kms_verifier_authority,
        ctx.accounts.mint.key(),
        amount_handle,
        cleartext_amount,
    )?;

    emit_cpi!(AmountDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        handle: amount_handle,
        cleartext_amount,
    });
    Ok(())
}
