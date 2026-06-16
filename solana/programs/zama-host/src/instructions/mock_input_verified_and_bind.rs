//! Binds mock verified inputs behind authority-gated PoC controls.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{AclAllowedEvent, InputVerifiedEvent},
    state::*,
};

/// Accounts for the authority-gated mock encrypted-input bind path.
///
/// This is PoC glue for tests and future input-verifier integration. It is not
/// a generic production API because it stores a caller-supplied input handle.
#[derive(Accounts)]
#[instruction(
    input_handle: [u8; 32],
    user: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct MockInputVerifiedAndBind<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Input verifier signer used by the local mock path.
    pub input_verifier_authority: Signer<'info>,
    /// App account signer authorizing the ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA with `mock_input_enabled`.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Box<Account<'info, HostConfig>>,
    /// Canonical output ACL record created by this instruction.
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [ACL_RECORD_SEED, output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Box<Account<'info, AclRecord>>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Binds a mock verified input handle after config and verifier checks.
pub fn mock_input_verified_and_bind(
    ctx: Context<MockInputVerifiedAndBind>,
    input_handle: [u8; 32],
    user: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: Vec<AclSubjectEntry>,
    output_public_decrypt: bool,
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        ctx.accounts.host_config.mock_input_allowed(),
        ZamaHostError::MockInputDisabled
    );
    require_keys_eq!(
        ctx.accounts.input_verifier_authority.key(),
        ctx.accounts.host_config.input_verifier_authority,
        ZamaHostError::MockInputVerifierMismatch
    );
    assert_input_handle_for_chain(input_handle, ctx.accounts.host_config.chain_id)?;
    assert_output_acl_metadata(
        ctx.accounts.app_account_authority.key(),
        output_nonce_key,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        &output_subjects,
    )?;
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;

    write_acl_record(
        &mut ctx.accounts.output_acl_record,
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        input_handle,
        &output_subjects,
        output_public_decrypt,
        Clock::get()?.slot,
        ctx.bumps.output_acl_record,
    );

    emit_cpi!(InputVerifiedEvent {
        version: EVENT_VERSION,
        input_handle,
        result_handle: input_handle,
        user: user.to_bytes(),
        acl_domain_key: output_acl_domain_key.to_bytes(),
    });
    emit_record_bound(
        ctx.accounts.output_acl_record.key(),
        &ctx.accounts.output_acl_record,
    );
    for output_subject in output_subjects {
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle: input_handle,
            subject: output_subject.pubkey.to_bytes(),
        });
        emit_subject_event(
            ctx.accounts.output_acl_record.key(),
            input_handle,
            output_subject,
            Pubkey::default(),
        );
    }
    Ok(())
}
