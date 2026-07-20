//! Consumes a KMS public-decrypt certificate through the stateless host verifier.
//!
//! This is the whole disclosure "consume" path after the `DisclosureRequest` lifecycle was dissolved
//! (fhevm-internal#1704, DD-040). It replaces both `disclose_balance_secp` and `disclose_amount_secp`
//! with one generic thin instruction: the app brings the KMS `PublicDecryptVerification` certificate
//! plus an MMR public-leaf inclusion proof in its own transaction, CPIs the stateless
//! `zama_host::verify_public_decrypt`, asserts the handle the host proved public equals the handle it
//! pinned, and emits a token-scoped [`HandleDisclosedEvent`].
//!
//! The request side is no longer a token instruction: an allowed subject (the balance owner, or any
//! subject on a token amount lineage) seals the public-decrypt leaf by calling the host
//! `make_handle_public` instruction directly. There is no per-request PDA, no `kms_context_id` pin,
//! and no `expires_slot` — the certificate is verified against the host's CURRENT `KmsContext`
//! (context rotation fails closed, one layer down in the host verifier), and any deadline the app
//! wants lives in the app's own state machine.
//!
//! ## Act-once is intentionally NOT enforced here
//!
//! Public disclosure is idempotent information release: once a handle's cleartext is KMS-certified
//! and its public-decrypt leaf is sealed, the value is public forever, so re-running this instruction
//! only re-emits the same event with the same cleartext — it reveals nothing new and moves no funds.
//! There is therefore no replay marker by design (contrast `redeem_burned_amount`, which guards
//! a vault transfer with a per-handle `burn-redemption` marker PDA). An app that needs consume-once
//! semantics (e.g. gating a one-time state transition on the reveal) tracks a settled flag in its own
//! account, exactly as an EVM app tracks its decryption callback.

use super::*;

/// Accounts for consuming a KMS public-decrypt certificate via the stateless host verifier.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseSecp<'info> {
    /// Confidential mint whose ACL domain scopes the disclosed lineage and event.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// The `EncryptedValue` lineage the disclosed handle belongs to.
    /// CHECK: canonical PDA, layout, and host ownership are validated by the `verify_public_decrypt`
    /// CPI; this handler additionally binds its `acl_domain_key` to `mint`.
    pub encrypted_value: UncheckedAccount<'info>,
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    #[account(
        seeds = [zama_host::HOST_CONFIG_SEED],
        seeds::program = zama_host::ID,
        bump = host_config.bump,
    )]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context PDA for the host's current context id (validated by the verifier CPI).
    pub kms_context: Box<Account<'info, zama_host::KmsContext>>,
    /// ZamaHost program used for the stateless verifier CPI.
    pub zama_program: Program<'info, ZamaHost>,
}

/// Verifies a KMS public-decrypt certificate through the host verifier and emits the disclosed
/// cleartext for a token-scoped handle. Idempotent by design — see the module doc comment.
pub fn disclose_secp(
    ctx: Context<DiscloseSecp>,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
    proof: zama_host::instructions::MmrInclusionProof,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_confidential_mint_shape(&ctx.accounts.mint)?;
    assert_host_config_allows_token_response(&ctx.accounts.host_config)?;
    let mint_key = ctx.accounts.mint.key();

    // Bind the disclosed lineage to this mint's ACL domain so the emitted event is genuinely
    // token-scoped. `read_encrypted_value` also asserts the account is a host-owned `EncryptedValue`;
    // the verifier CPI re-reads it and enforces the canonical PDA and inclusion proof.
    let value = fhe::read_encrypted_value(&ctx.accounts.encrypted_value.to_account_info())?;
    require_keys_eq!(
        value.acl_domain_key,
        mint_key,
        ConfidentialTokenError::AmountAclMismatch
    );

    let certified_cleartext = fhe::verify_public_decrypt(fhe::VerifyPublicDecrypt {
        expected_handle: handle,
        cleartext,
        signatures,
        extra_data,
        proof,
        encrypted_value: ctx.accounts.encrypted_value.to_account_info(),
        host_config: &ctx.accounts.host_config,
        kms_context: ctx.accounts.kms_context.to_account_info(),
        zama_program: &ctx.accounts.zama_program,
    })?;

    // Token lineages are euint64 today, so the certified uint256 cleartext must fit in 64 bits: the
    // high 24 bytes must be zero for the low-64-bit truncation below to be lossless. Reject anything
    // wider rather than silently discarding high bits.
    require!(
        certified_cleartext[..24].iter().all(|byte| *byte == 0),
        ConfidentialTokenError::VerifierReturnDataInvalid
    );

    emit_cpi!(HandleDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        handle,
        encrypted_value: ctx.accounts.encrypted_value.key(),
        cleartext_amount: u64::from_be_bytes(
            certified_cleartext[24..]
                .try_into()
                .expect("cleartext is 32 bytes"),
        ),
    });
    Ok(())
}
