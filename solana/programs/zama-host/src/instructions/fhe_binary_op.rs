//! Emits binary FHE operations without creating durable output ACL state.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, events::FheBinaryOpEvent, state::*};

/// Accounts for a binary FHE operation without durable output ACL birth.
#[derive(Accounts)]
#[instruction(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8
)]
#[event_cpi]
pub struct FheBinaryOp<'info> {
    /// Compute subject that must be allowed on encrypted operands.
    pub compute_subject: Signer<'info>,
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
}

/// Verifies operand ACLs, checks the caller-supplied result handle, and emits a compute event.
pub fn fhe_binary_op(
    ctx: Context<FheBinaryOp>,
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    result: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_binary_operand_types(op, lhs, rhs, scalar, output_fhe_type)?;
    let subject = ctx.accounts.compute_subject.key();

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

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    let expected_result = computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        output_fhe_type,
        ctx.accounts.host_config.chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
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
    Ok(())
}
