//! Emits binary FHE operations and binds durable output ACL state.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{AclAllowedEvent, FheBinaryOpEvent},
    state::*,
};

/// Accounts for a binary FHE operation that also births durable output ACL state.
#[derive(Accounts)]
#[instruction(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    result: [u8; 32],
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64
)]
#[event_cpi]
pub struct FheBinaryOpAndBindOutput<'info> {
    /// Pays rent for the output ACL record.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Compute subject that must be allowed on encrypted operands.
    pub compute_subject: Signer<'info>,
    /// App account signer authorizing the output ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Canonical ACL record for the left-hand operand.
    pub lhs_acl_record: Account<'info, AclRecord>,
    /// Optional overflow permission witness when `compute_subject` is not inline on the LHS record.
    pub lhs_permission_record: Option<UncheckedAccount<'info>>,
    /// CHECK: encrypted RHS operands are deserialized and ACL-checked in the
    /// instruction body; scalar RHS operands deliberately skip this account.
    pub rhs_acl_record: UncheckedAccount<'info>,
    /// Optional overflow permission witness when `compute_subject` is not inline on the RHS record.
    pub rhs_permission_record: Option<UncheckedAccount<'info>>,
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

/// Verifies operand ACLs, checks a nonce-bound result handle, emits compute,
/// and creates the output ACL record in the same instruction.
pub fn fhe_binary_op_and_bind_output(
    ctx: Context<FheBinaryOpAndBindOutput>,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    result: [u8; 32],
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
    assert_binary_operand_types(op, lhs, rhs, scalar, output_fhe_type)?;
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

    assert_canonical_acl_record(
        &ctx.accounts.lhs_acl_record.to_account_info(),
        &ctx.accounts.lhs_acl_record,
    )?;
    assert_acl_record_handle_for_chain(
        &ctx.accounts.lhs_acl_record,
        ctx.accounts.host_config.chain_id,
    )?;
    assert_record_subject_role(
        &ctx.accounts.lhs_acl_record,
        ctx.accounts.lhs_acl_record.key(),
        lhs,
        subject,
        ACL_ROLE_USE,
        ctx.accounts
            .lhs_permission_record
            .as_ref()
            .map(|account| account.to_account_info())
            .as_ref(),
    )?;
    let lhs_public_decrypt_allowed = unchecked_acl_record_subject_has_role(
        &ctx.accounts.lhs_acl_record.to_account_info(),
        lhs,
        subject,
        ACL_ROLE_PUBLIC_DECRYPT,
        ctx.accounts
            .lhs_permission_record
            .as_ref()
            .map(|account| account.to_account_info())
            .as_ref(),
    )?;
    if scalar {
        require!(
            ctx.accounts.rhs_permission_record.is_none(),
            ZamaHostError::AclPermissionMismatch
        );
    } else {
        assert_unchecked_acl_record_subject_role(
            &ctx.accounts.rhs_acl_record.to_account_info(),
            rhs,
            ctx.accounts.host_config.chain_id,
            subject,
            ACL_ROLE_USE,
            ctx.accounts
                .rhs_permission_record
                .as_ref()
                .map(|account| account.to_account_info())
                .as_ref(),
        )?;
    }
    let rhs_public_decrypt_allowed = if scalar {
        true
    } else {
        unchecked_acl_record_subject_has_role(
            &ctx.accounts.rhs_acl_record.to_account_info(),
            rhs,
            subject,
            ACL_ROLE_PUBLIC_DECRYPT,
            ctx.accounts
                .rhs_permission_record
                .as_ref()
                .map(|account| account.to_account_info())
                .as_ref(),
        )?
    };
    assert_derived_public_decrypt_roles_allowed(
        &output_subjects,
        lhs_public_decrypt_allowed && rhs_public_decrypt_allowed,
        &ctx.accounts.app_account_authority.to_account_info(),
    )?;
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    let expected_result = computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        output_fhe_type,
        ctx.accounts.host_config.chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    );
    require!(
        result == expected_result,
        ZamaHostError::ComputedHandleMismatch
    );
    #[cfg(feature = "emit-events")]
    emit_cpi!(FheBinaryOpEvent {
        version: EVENT_VERSION,
        op,
        subject: subject.to_bytes(),
        lhs,
        rhs,
        scalar,
        result,
    });

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
