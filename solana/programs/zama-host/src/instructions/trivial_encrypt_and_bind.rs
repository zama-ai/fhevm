//! Creates trivial-encrypt handles and binds initial ACL records.

use anchor_lang::prelude::*;

use super::common::*;
#[cfg(feature = "emit-events")]
use crate::events::{AclAllowedEvent, TrivialEncryptEvent};
use crate::state::*;

/// Accounts for trivial encryption plus initial ACL record birth.
#[derive(Accounts)]
#[instruction(
    plaintext: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct TrivialEncryptAndBind<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Subject associated with the emitted trivial-encrypt event.
    pub compute_subject: Signer<'info>,
    /// App account signer authorizing the new ACL record metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Canonical output ACL record created by this instruction.
    #[account(
        init,
        payer = payer,
        space = 8 + AclRecord::SPACE,
        seeds = [ACL_RECORD_SEED, output_nonce_key.as_ref(), &output_nonce_sequence.to_le_bytes()],
        bump
    )]
    pub output_acl_record: Account<'info, AclRecord>,
    /// System program used for ACL account creation.
    pub system_program: Program<'info, System>,
}

/// Creates a trivial-encrypt handle and its first canonical ACL record.
pub fn trivial_encrypt_and_bind(
    ctx: Context<TrivialEncryptAndBind>,
    plaintext: [u8; 32],
    fhe_type: u8,
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
    assert_supported_fhe_type(fhe_type)?;
    // Only used by the gated TrivialEncryptEvent emit below.
    #[cfg(feature = "emit-events")]
    let subject = ctx.accounts.compute_subject.key();
    assert_output_acl_metadata(
        ctx.accounts.app_account_authority.key(),
        output_nonce_key,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        &output_subjects,
    )?;
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    let result = computed_trivial_handle(
        plaintext,
        fhe_type,
        ctx.accounts.host_config.chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    );

    write_acl_record(
        &mut ctx.accounts.output_acl_record,
        output_nonce_key,
        output_nonce_sequence,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        result,
        &output_subjects,
        output_public_decrypt,
        clock.slot,
        ctx.bumps.output_acl_record,
    );

    #[cfg(feature = "emit-events")]
    emit_cpi!(TrivialEncryptEvent {
        version: EVENT_VERSION,
        subject: subject.to_bytes(),
        plaintext,
        fhe_type,
        result,
    });
    emit_record_bound(
        ctx.accounts.output_acl_record.key(),
        &ctx.accounts.output_acl_record,
    );
    for output_subject in output_subjects {
        #[cfg(feature = "emit-events")]
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle: result,
            subject: output_subject.pubkey.to_bytes(),
        });
        emit_subject_event(
            ctx.accounts.output_acl_record.key(),
            result,
            output_subject,
            Pubkey::default(),
        );
    }
    Ok(())
}
