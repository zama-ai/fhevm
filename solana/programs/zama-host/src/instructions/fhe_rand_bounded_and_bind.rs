//! Creates bounded-random ciphertext handles and binds initial ACL records.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    events::{AclAllowedEvent, FheRandBoundedEvent},
    state::*,
};

/// Accounts for bounded random ciphertext creation plus initial ACL record birth.
#[derive(Accounts)]
#[instruction(
    upper_bound: [u8; 32],
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct FheRandBoundedAndBind<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Subject associated with the emitted bounded-random event.
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

/// Creates a bounded random ciphertext handle and binds its first canonical ACL record.
pub fn fhe_rand_bounded_and_bind(
    ctx: Context<FheRandBoundedAndBind>,
    upper_bound: [u8; 32],
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
    assert_valid_bounded_rand_upper_bound(upper_bound, fhe_type)?;
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
    let seed = computed_rand_seed(
        ctx.accounts.host_config.chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    );
    let result = computed_rand_bounded_handle(
        upper_bound,
        seed,
        fhe_type,
        ctx.accounts.host_config.chain_id,
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
    emit_cpi!(FheRandBoundedEvent {
        version: EVENT_VERSION,
        subject: subject.to_bytes(),
        upper_bound,
        seed,
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
