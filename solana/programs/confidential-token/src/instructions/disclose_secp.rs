//! Verifies public disclosure of an exact historical handle under a token or mint state.
//! Disclosure is informational and replayable; redemption separately consumes PendingBurn.

use super::*;

/// Accounts for consuming a KMS public-decrypt certificate via the stateless host verifier.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseSecp<'info> {
    /// Confidential mint whose application scopes the disclosed encrypted State and event.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Token account whose State contains the handle. Absent for the mint total-supply State.
    pub token_account: Option<Box<Account<'info, ConfidentialTokenAccount>>>,
    /// The State whose history contains a public permission for the disclosed handle.
    /// The handler binds its program, mint scope and token-account or total-supply authority.
    /// Disclosure authenticates the handle and cleartext, without a token-specific field label.
    /// CHECK: canonical PDA, layout and host ownership are validated by the verifier CPI.
    pub encrypted_state: UncheckedAccount<'info>,
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context PDA for the id the certificate commits to (any live context; validated by the
    /// verifier CPI).
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

    let value = fhe::read_state(&ctx.accounts.encrypted_state.to_account_info())?;
    let expected_authority = if let Some(token_account) = &ctx.accounts.token_account {
        assert_confidential_token_account_shape(token_account, mint_key, token_account.owner)?;
        token_account.key()
    } else {
        total_supply_authority_address(mint_key).0
    };
    require!(
        value.program == crate::ID
            && value.scope == mint_key.to_bytes()
            && value.authority == expected_authority,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    require_keys_eq!(
        ctx.accounts.encrypted_state.key(),
        encrypted_state_address(mint_key, expected_authority).0,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );

    let certified_cleartext = fhe::verify_public_decrypt(fhe::VerifyPublicDecrypt {
        expected_handle: handle,
        cleartext,
        signatures,
        extra_data,
        proof,
        encrypted_state: ctx.accounts.encrypted_state.to_account_info(),
        host_config: &ctx.accounts.host_config,
        kms_context: ctx.accounts.kms_context.to_account_info(),
        zama_program: &ctx.accounts.zama_program,
    })?;

    // Token encrypted States are euint64 today, so the certified uint256 cleartext must fit in 64 bits: the
    // high 24 bytes must be zero for the low-64-bit truncation below to be lossless. Reject anything
    // wider rather than silently discarding high bits.
    require!(
        certified_cleartext[..24].iter().all(|byte| *byte == 0),
        ConfidentialTokenError::CleartextExceedsEuint64
    );

    emit_cpi!(HandleDisclosedEvent {
        version: APP_EVENT_VERSION,
        mint: mint_key,
        handle,
        encrypted_state: ctx.accounts.encrypted_state.key(),
        authority: expected_authority,
        cleartext_amount: u64::from_be_bytes(
            certified_cleartext[24..]
                .try_into()
                .expect("cleartext is 32 bytes"),
        ),
    });
    Ok(())
}
