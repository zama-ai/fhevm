//! Publishes a KMS-certified handle and its cleartext on-chain, as ERC-7984
//! `discloseEncryptedAmount` does. Disclosure is informational and replayable; redemption
//! separately consumes PendingBurn.

use super::*;

/// Accounts for consuming a KMS public-decrypt certificate via the stateless host verifier.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseSecp<'info> {
    /// Host config carrying the current KMS context id and gateway EIP-712 domain.
    #[account(seeds = [zama_host::HOST_CONFIG_SEED], bump = host_config.bump, seeds::program = zama_host::ID)]
    pub host_config: Box<Account<'info, zama_host::HostConfig>>,
    /// KMS context PDA for the id the certificate commits to (any live context; validated by the
    /// verifier CPI).
    pub kms_context: Box<Account<'info, zama_host::KmsContext>>,
    /// ZamaHost program used for the stateless verifier CPI.
    pub zama_program: Program<'info, ZamaHost>,
}

/// Verifies a KMS public-decrypt certificate through the host verifier and emits the certified
/// cleartext for `handle`. Idempotent by design — see the module doc comment.
pub fn disclose_secp(
    ctx: Context<DiscloseSecp>,
    handle: [u8; 32],
    cleartext: [u8; 32],
    signatures: Vec<[u8; 65]>,
    extra_data: Vec<u8>,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;

    let certified_cleartext = fhe::verify_public_decrypt(fhe::VerifyPublicDecrypt {
        expected_handle: handle,
        cleartext,
        signatures,
        extra_data,
        host_config: &ctx.accounts.host_config,
        kms_context: ctx.accounts.kms_context.to_account_info(),
        zama_program: &ctx.accounts.zama_program,
    })?;

    // `HandleDisclosedEvent.cleartext_amount` is a u64, so the certified uint256 cleartext must fit
    // in 64 bits: the high 24 bytes must be zero for the truncation below to be lossless. Reject
    // anything wider rather than silently discarding high bits.
    require!(
        certified_cleartext[..24].iter().all(|byte| *byte == 0),
        ConfidentialTokenError::CleartextExceedsEuint64
    );

    emit_cpi!(HandleDisclosedEvent {
        version: APP_EVENT_VERSION,
        handle,
        cleartext_amount: u64::from_be_bytes(
            certified_cleartext[24..]
                .try_into()
                .expect("cleartext is 32 bytes"),
        ),
    });
    Ok(())
}
