//! Publishes KMS-certified cleartexts for current confidential balances, verifying
//! the KMS `PublicDecryptVerification` EIP-712 certificate on-chain via
//! `secp256k1_recover` (the gateway-compatible path, #1494 Phase 3 cert-secp).
//!
//! Consumes a `DisclosureRequest` witness created by `request_disclose_balance`: the cert is
//! verified against the KMS context the witness was pinned to (not the current context), the
//! witness must still be PENDING and unexpired, and it is flipped to CONSUMED here so a single
//! request authorizes exactly one disclosure.

use super::*;

/// Accounts for disclosing a KMS-certified current balance via secp256k1 EIP-712.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseBalanceSecp<'info> {
    /// Confidential mint the disclosed balance belongs to.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account whose current balance is disclosed.
    pub token_account: Box<Account<'info, ConfidentialTokenAccount>>,
    /// Current balance ACL record for the disclosed handle.
    pub balance_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub balance_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
    /// Account-backed disclosure request witness consumed by this instruction.
    #[account(mut)]
    pub disclosure_request: Box<Account<'info, DisclosureRequest>>,
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
}

/// Emits a KMS-certified cleartext after on-chain secp256k1 verification of the KMS
/// `PublicDecryptVerification` EIP-712 certificate.
pub fn disclose_balance_secp(
    ctx: Context<DiscloseBalanceSecp>,
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_host_config_allows_token_response(&ctx.accounts.host_config)?;
    let mint_key = ctx.accounts.mint.key();
    require_keys_eq!(
        ctx.accounts.token_account.mint,
        mint_key,
        ConfidentialTokenError::MintMismatch
    );
    assert_confidential_token_account_shape(
        &ctx.accounts.token_account,
        mint_key,
        ctx.accounts.token_account.owner,
    )?;
    assert_current_balance_acl(
        &ctx.accounts.balance_acl_record,
        ctx.accounts.balance_acl_record.key(),
        &ctx.accounts.token_account,
        mint_key,
    )?;
    let handle = ctx.accounts.token_account.balance_handle;
    assert_material_commitment(
        &ctx.accounts.balance_material_commitment,
        ctx.accounts.balance_material_commitment.key(),
        &ctx.accounts.balance_acl_record,
        handle,
    )?;
    assert_public_decrypt_released(&ctx.accounts.balance_acl_record)?;

    // Bind to the request witness: same mode/handle/accounts/material/host config; PENDING and
    // unexpired; recomputed request_hash matches.
    let token_account_key = ctx.accounts.token_account.key();
    assert_disclosure_request_witness(
        &ctx.accounts.disclosure_request,
        ctx.accounts.disclosure_request.key(),
        DISCLOSURE_REQUEST_MODE_BALANCE,
        mint_key,
        token_account_key,
        token_account_key,
        handle,
        ctx.accounts.balance_acl_record.key(),
        &ctx.accounts.balance_material_commitment,
        ctx.accounts.host_config.key(),
    )?;
    // Verify the cert against the witness-pinned context, closing rotation reuse.
    assert_kms_public_decrypt_cert_for_request(
        &ctx.accounts.host_config,
        &ctx.accounts.kms_context,
        ctx.accounts.disclosure_request.kms_context_id,
        handle,
        cleartext_amount,
        &signatures,
        &extra_data,
    )?;

    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = ctx.accounts.disclosure_request.request_hash;
    ctx.accounts.disclosure_request.status = REQUEST_STATUS_CONSUMED;

    emit_cpi!(BalanceDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        owner: ctx.accounts.token_account.owner,
        token_account: token_account_key,
        handle,
        request: request_key,
        request_hash,
        cleartext_amount,
    });
    Ok(())
}
