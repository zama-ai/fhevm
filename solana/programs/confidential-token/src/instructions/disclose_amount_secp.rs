//! Publishes KMS-certified cleartexts for token-scoped encrypted amounts, verifying
//! the KMS `PublicDecryptVerification` EIP-712 certificate on-chain via
//! `secp256k1_recover` (the gateway-compatible path, #1494 Phase 3 cert-secp).
//!
//! Amount-handle counterpart of `disclose_balance_secp`; trusts the gateway-level KMS
//! signer on `HostConfig`. Added alongside the Ed25519 `disclose_amount`.

use super::*;

/// Accounts for disclosing a KMS-certified token-scoped amount via secp256k1 EIP-712.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseAmountSecp<'info> {
    /// Confidential mint the disclosed amount belongs to.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token-scoped amount ACL record for the disclosed handle.
    pub amount_acl_record: Box<Account<'info, zama_host::AclRecord>>,
    /// Material commitment witness for the disclosed handle.
    pub amount_material_commitment: Box<Account<'info, zama_host::HandleMaterialCommitment>>,
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
            &[amount_handle],
            &kms_decrypted_result_bytes(cleartext_amount),
            &extra_data,
            &signatures,
        ),
        ConfidentialTokenError::InvalidKmsCertificate
    );

    emit_cpi!(AmountDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: ctx.accounts.mint.key(),
        handle: amount_handle,
        cleartext_amount,
    });
    Ok(())
}
