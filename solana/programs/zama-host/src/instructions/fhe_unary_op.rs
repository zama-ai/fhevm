//! Emits unary FHE operations without creating durable output ACL state.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{errors::ZamaHostError, events::FheUnaryOpEvent, state::*};

/// Accounts for a unary FHE operation without durable output ACL birth.
#[derive(Accounts)]
#[instruction(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    output_fhe_type: u8
)]
#[event_cpi]
pub struct FheUnaryOp<'info> {
    /// Compute subject that must be allowed on the encrypted operand.
    pub compute_subject: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// Canonical ACL record for the operand.
    pub operand_acl_record: Account<'info, AclRecord>,
    /// Optional overflow permission witness when `compute_subject` is not inline on the operand record.
    pub operand_permission_record: Option<UncheckedAccount<'info>>,
}

/// Verifies operand ACL, checks the caller-supplied result handle, and emits a compute event.
pub fn fhe_unary_op(
    ctx: Context<FheUnaryOp>,
    op: FheUnaryOpCode,
    operand: [u8; 32],
    output_fhe_type: u8,
    result: [u8; 32],
) -> Result<()> {
    assert_no_remaining_accounts(ctx.remaining_accounts)?;
    assert_not_paused(&ctx.accounts.host_config)?;
    assert_unary_operand_type(op, operand, output_fhe_type)?;
    let subject = ctx.accounts.compute_subject.key();

    assert_canonical_acl_record(
        &ctx.accounts.operand_acl_record.to_account_info(),
        &ctx.accounts.operand_acl_record,
    )?;
    assert_acl_record_handle_for_chain(
        &ctx.accounts.operand_acl_record,
        ctx.accounts.host_config.chain_id,
    )?;
    assert_record_subject_role(
        &ctx.accounts.operand_acl_record,
        ctx.accounts.operand_acl_record.key(),
        operand,
        subject,
        ACL_ROLE_USE,
        ctx.accounts
            .operand_permission_record
            .as_ref()
            .map(|account| account.to_account_info())
            .as_ref(),
    )?;

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    let expected_result = computed_unary_handle(
        op,
        operand,
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
    emit_cpi!(FheUnaryOpEvent {
        version: EVENT_VERSION,
        op,
        subject: subject.to_bytes(),
        operand,
        result,
    });
    Ok(())
}
