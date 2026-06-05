//! Publishes KMS-certified cleartexts for current confidential balances, verifying
//! the KMS `PublicDecryptVerification` EIP-712 certificate on-chain via
//! `secp256k1_recover` (the gateway-compatible path, #1494 Phase 3 cert-secp).
//!
//! Mirrors `disclose_balance` but trusts a gateway-level KMS signer configured on
//! `HostConfig` (EVM secp256k1) instead of the per-mint Ed25519 verifier. Added
//! alongside the Ed25519 path; the legacy path stays until the secp path is adopted.

use super::*;

/// Encodes a u64 cleartext as the 32-byte big-endian (abi `uint256`) decrypted result
/// the KMS signs over in the `PublicDecryptVerification` certificate.
fn decrypted_result_bytes(cleartext_amount: u64) -> [u8; 32] {
    let mut decrypted = [0u8; 32];
    decrypted[24..].copy_from_slice(&cleartext_amount.to_be_bytes());
    decrypted
}

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
    /// Host config carrying the gateway KMS verifier (signer + decryption contract).
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
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

    let config = &ctx.accounts.host_config;
    require!(
        config.kms_signer != [0u8; 20] && config.decryption_contract != [0u8; 20],
        ConfidentialTokenError::GatewayVerifierConfigUnset
    );
    let verifier = zama_host::eip712::Eip712VerifierConfig {
        gateway_chain_id: config.gateway_chain_id,
        verifying_contract: config.decryption_contract,
        signers: std::slice::from_ref(&config.kms_signer),
        threshold: 1,
    };
    require!(
        zama_host::eip712::verify_kms_public_decrypt(
            &verifier,
            &[handle],
            &decrypted_result_bytes(cleartext_amount),
            &extra_data,
            &signatures,
        ),
        ConfidentialTokenError::InvalidKmsCertificate
    );

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
