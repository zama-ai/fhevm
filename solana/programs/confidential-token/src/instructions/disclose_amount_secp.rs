//! Publishes KMS-certified cleartexts for token-scoped encrypted amounts, verifying
//! the KMS `PublicDecryptVerification` EIP-712 certificate on-chain via
//! `secp256k1_recover` (the gateway-compatible path, #1494 Phase 3 cert-secp).
//!
//! Consumes a `DisclosureRequest` witness created by `request_disclose_amount`: the cert is
//! verified against the KMS context the witness was pinned to (not the current context), the
//! witness must still be PENDING and unexpired, and it is flipped to CONSUMED here so a single
//! request authorizes exactly one disclosure.

use super::*;

/// Accounts for disclosing a KMS-certified token-scoped amount via secp256k1 EIP-712.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmountSecp<'info> {
    /// Confidential mint the disclosed amount belongs to.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount `EncryptedValue` lineage for the disclosed handle.
    pub amount_value: Box<Account<'info, zama_host::EncryptedValue>>,
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

/// Emits a KMS-certified cleartext for a token-scoped amount after on-chain
/// secp256k1 verification of the KMS `PublicDecryptVerification` certificate.
pub fn disclose_amount_secp(
    ctx: Context<DiscloseAmountSecp>,
    amount_handle: [u8; 32],
    cleartext_amount: u64,
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_host_config_allows_token_response(&ctx.accounts.host_config)?;
    let mint_key = ctx.accounts.mint.key();
    assert_token_amount_encrypted_value(
        &ctx.accounts.amount_value,
        amount_handle,
        mint_key,
        ctx.accounts.mint.compute_signer,
    )?;

    // Bind to the request witness: same mode/handle/accounts/host config; PENDING and unexpired;
    // recomputed request_hash matches.
    assert_disclosure_request_witness(
        &ctx.accounts.disclosure_request,
        ctx.accounts.disclosure_request.key(),
        DISCLOSURE_REQUEST_MODE_AMOUNT,
        mint_key,
        Pubkey::default(),
        ctx.accounts.amount_value.app_account,
        amount_handle,
        ctx.accounts.amount_value.key(),
        ctx.accounts.host_config.key(),
    )?;
    // Verify the cert against the witness-pinned context, closing rotation reuse.
    assert_kms_public_decrypt_cert_for_request(
        &ctx.accounts.host_config,
        &ctx.accounts.kms_context,
        ctx.accounts.disclosure_request.kms_context_id,
        amount_handle,
        cleartext_amount,
        &signatures,
        &extra_data,
    )?;

    let request_key = ctx.accounts.disclosure_request.key();
    let request_hash = ctx.accounts.disclosure_request.request_hash;
    ctx.accounts.disclosure_request.status = REQUEST_STATUS_CONSUMED;

    emit_cpi!(AmountDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        handle: amount_handle,
        request: request_key,
        request_hash,
        cleartext_amount,
    });
    Ok(())
}
