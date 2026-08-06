//! Consumes a KMS public-decrypt certificate through the stateless host verifier.
//!
//! This is the whole disclosure "consume" path after the `DisclosureRequest` lifecycle was dissolved
//! (fhevm-internal#1704, DD-040). It replaces both `disclose_balance_secp` and `disclose_amount_secp`
//! with one generic thin instruction: the app brings the KMS `PublicDecryptVerification` certificate
//! plus an MMR public-leaf inclusion proof in its own transaction, CPIs the stateless
//! `zama_host::verify_public_decrypt`, asserts the handle the host proved public equals the handle it
//! pinned, validates the exact token state field, and emits a token-scoped
//! [`HandleDisclosedEvent`].
//!
//! The request side has no request account: a token owner calls
//! `make_token_account_handle_public`, or a mint authority calls
//! `make_total_supply_handle_public`; the wrapper validates the exact state field and CPIs the host
//! while signing as the encrypted value account authority. There is no per-request PDA, no `kms_context_id` pin,
//! and no `expires_slot` — the certificate is verified against the `KmsContext` the cert names (any
//! live, non-destroyed context, EVM-parity rotation grace; `destroy_kms_context` is the revocation
//! lever, one layer down in the host verifier), and any deadline the app wants lives in the app's own
//! state machine.
//!
//! ## Act-once is intentionally NOT enforced here
//!
//! Public disclosure is idempotent information release: once a handle's cleartext is KMS-certified
//! and its public-decrypt leaf is sealed, the value is public forever, so re-running this instruction
//! only re-emits the same event with the same cleartext — it reveals nothing new and moves no funds.
//! There is therefore no replay marker by design (contrast `redeem_burned_amount`, which closes a
//! `PendingBurn` account). An app that needs consume-once semantics (e.g. gating a one-time
//! state transition on the reveal) tracks a settled flag in its own account, exactly as an EVM app
//! tracks its decryption callback. The rule this instruction is applying is stated once, at
//! `zama_host::instructions::verify_public_decrypt` (INVARIANTS #24).
//!
//! ## Indexer note (fhevm-internal#1862 #14)
//!
//! `HandleDisclosedEvent` carries the validated token state kind, authority, and label. Consumers do
//! not need to infer whether a mint-domain value was a balance, transfer amount, burn amount, or
//! total supply from an untrusted account choice.

use super::*;

/// Accounts for consuming a KMS public-decrypt certificate via the stateless host verifier.
#[derive(Accounts)]
#[event_cpi]
pub struct DiscloseSecp<'info> {
    /// Confidential mint whose ACL domain scopes the disclosed encrypted value account and event.
    pub mint: Box<Account<'info, ConfidentialMint>>,
    /// Confidential token account for account-scoped kinds. Must be absent for total supply.
    pub token_account: Option<Box<Account<'info, ConfidentialTokenAccount>>>,
    /// The `EncryptedValue` encrypted value account the disclosed handle belongs to.
    /// CHECK: canonical PDA, layout, and host ownership are validated by the `verify_public_decrypt`
    /// CPI; this handler additionally binds its `domain` to `mint`.
    pub encrypted_value: UncheckedAccount<'info>,
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
    kind: DisclosedValueKind,
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

    // Bind the encrypted value account to one exact token state field. Domain-only binding would let
    // an arbitrary mint-domain value emit an event that indexers could mistake for a balance or
    // supply disclosure.
    let value = fhe::read_encrypted_value(&ctx.accounts.encrypted_value.to_account_info())?;
    require_keys_eq!(
        value.domain,
        mint_key,
        ConfidentialTokenError::DomainMismatch
    );
    let (expected_authority, expected_label, expected_value) = match kind {
        DisclosedValueKind::TotalSupply => {
            require!(
                ctx.accounts.token_account.is_none(),
                ConfidentialTokenError::DisclosedValueBindingMismatch
            );
            (
                total_supply_authority_address(mint_key).0,
                encrypted_total_supply_label(),
                ctx.accounts.mint.total_supply_encrypted_value,
            )
        }
        DisclosedValueKind::Balance
        | DisclosedValueKind::TransferredAmount
        | DisclosedValueKind::BurnedAmount => {
            let token_account = ctx
                .accounts
                .token_account
                .as_ref()
                .ok_or(ConfidentialTokenError::DisclosedValueBindingMismatch)?;
            require_keys_eq!(
                token_account.mint,
                mint_key,
                ConfidentialTokenError::DisclosedValueBindingMismatch
            );
            assert_confidential_token_account_shape(token_account, mint_key, token_account.owner)?;
            let label = match kind {
                DisclosedValueKind::Balance => encrypted_balance_label(),
                DisclosedValueKind::TransferredAmount => encrypted_transferred_amount_label(),
                DisclosedValueKind::BurnedAmount => encrypted_burned_amount_label(),
                DisclosedValueKind::TotalSupply => unreachable!(),
            };
            (
                token_account.key(),
                label,
                encrypted_value_address(mint_key, token_account.key(), label).0,
            )
        }
    };
    require_keys_eq!(
        value.encrypted_value_account_authority,
        expected_authority,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    require!(
        value.label == expected_label,
        ConfidentialTokenError::DisclosedValueBindingMismatch
    );
    require_keys_eq!(
        ctx.accounts.encrypted_value.key(),
        expected_value,
        ConfidentialTokenError::DisclosedValueBindingMismatch
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

    // Token encrypted value accounts are euint64 today, so the certified uint256 cleartext must fit in 64 bits: the
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
        encrypted_value: ctx.accounts.encrypted_value.key(),
        kind,
        encrypted_value_account_authority: expected_authority,
        encrypted_value_label: expected_label,
        cleartext_amount: u64::from_be_bytes(
            certified_cleartext[24..]
                .try_into()
                .expect("cleartext is 32 bytes"),
        ),
    });
    Ok(())
}
