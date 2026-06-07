//! Publishes KMS-certified cleartexts for token-scoped encrypted amounts.

use super::*;

/// Accounts for disclosing a KMS-certified token-scoped amount cleartext.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmount<'info> {
    /// Confidential mint carrying the disclosure verifier-set pointer.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record for the disclosed handle.
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Account-backed disclosure request witness.
    #[account(mut)]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
    /// Threshold verifier set whose quorum signs the response certificate.
    pub disclosure_verifier_set: Box<Account<'info, zama_host::VerifierSet>>,
    /// ZamaHost config bound into the request and certificate.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
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
    assert_host_config_allows_token_response(&ctx.accounts.host_config)?;
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
    assert_active_verifier_set(
        &ctx.accounts.disclosure_verifier_set,
        ctx.accounts.disclosure_verifier_set.key(),
        zama_host::VERIFIER_SET_KIND_TOKEN_DISCLOSURE,
        ctx.accounts.mint.key(),
    )?;
    assert_disclosure_request_witness(
        &ctx.accounts.disclosure_request,
        ctx.accounts.disclosure_request.key(),
        DISCLOSURE_REQUEST_MODE_AMOUNT,
        ctx.accounts.mint.key(),
        Pubkey::default(),
        ctx.accounts.amount_acl_record.app_account,
        amount_handle,
        ctx.accounts.amount_acl_record.key(),
        &ctx.accounts.amount_material_commitment,
        ctx.accounts.host_config.key(),
        &ctx.accounts.disclosure_verifier_set,
    )?;
    let message = disclosure_proof_message_v2(
        crate::ID,
        zama_host::ID,
        ctx.accounts.host_config.key(),
        ctx.accounts.disclosure_request.chain_id,
        ctx.accounts.mint.key(),
        DISCLOSURE_REQUEST_MODE_AMOUNT,
        ctx.accounts.disclosure_verifier_set.key(),
        ctx.accounts.disclosure_verifier_set.version,
        ctx.accounts.disclosure_request.key(),
        ctx.accounts.disclosure_request.request_hash,
        ctx.accounts.amount_acl_record.key(),
        ctx.accounts
            .amount_material_commitment
            .material_commitment_hash,
        ctx.accounts.amount_material_commitment.key_id,
        amount_handle,
        cleartext_amount,
    );
    assert_threshold_verifier_signature(
        &ctx.accounts.instructions_sysvar.to_account_info(),
        &ctx.accounts.disclosure_verifier_set,
        &message,
    )?;
    ctx.accounts.disclosure_request.status = REQUEST_STATUS_CONSUMED;

    emit_cpi!(AmountDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        handle: amount_handle,
        request: ctx.accounts.disclosure_request.key(),
        request_hash: ctx.accounts.disclosure_request.request_hash,
        cleartext_amount,
    });
    Ok(())
}
