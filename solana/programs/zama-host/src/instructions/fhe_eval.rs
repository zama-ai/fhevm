//! Evaluates ordered instruction-local FHE plans.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{AclAllowedEvent, FheBinaryOpEvent},
    state::*,
};

/// Accounts for composed instruction-local FHE evaluation.
///
/// Durable input ACL records and durable output ACL records are supplied in
/// `remaining_accounts` and referenced by index from [`FheEvalArgs`].
#[derive(Accounts)]
#[event_cpi]
pub struct FheEval<'info> {
    /// Pays rent for any durable output ACL records.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Compute subject that must be allowed on durable encrypted inputs.
    pub compute_subject: Signer<'info>,
    /// App account signer authorizing any durable output ACL metadata.
    pub app_account_authority: Signer<'info>,
    /// Singleton config PDA.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for durable output ACL creation.
    pub system_program: Program<'info, System>,
    /// Instructions sysvar used to bind transient capabilities to the top-level receiver program.
    pub instructions_sysvar: Option<UncheckedAccount<'info>>,
}

/// Executes an ordered binary-op plan with instruction-local transient outputs.
pub fn fhe_eval<'info>(ctx: Context<'info, FheEval<'info>>, args: FheEvalArgs) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        args.context_id != [0; 32],
        ZamaHostError::InvalidFheEvalContext
    );
    require!(
        !args.ops.is_empty() && args.ops.len() <= MAX_FHE_EVAL_OPS,
        ZamaHostError::InvalidFheEvalOperationCount
    );

    let subject = ctx.accounts.compute_subject.key();
    let session_authority = ctx.accounts.app_account_authority.key();
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    let mut produced = Vec::with_capacity(args.ops.len());
    let mut binary_events = Vec::with_capacity(args.ops.len());
    let mut remaining_accounts_used = vec![false; ctx.remaining_accounts.len()];
    let mut instructions_sysvar_used = false;

    for (index, op) in args.ops.iter().enumerate() {
        let lhs = resolve_lhs_operand(
            ctx.remaining_accounts,
            &mut remaining_accounts_used,
            &produced,
            &op.lhs,
            subject,
            session_authority,
            ctx.accounts.host_config.chain_id,
            clock.slot,
            &mut instructions_sysvar_used,
            ctx.accounts
                .instructions_sysvar
                .as_ref()
                .map(|account| account.to_account_info())
                .as_ref(),
        )?;
        let rhs = resolve_rhs_operand(
            ctx.remaining_accounts,
            &mut remaining_accounts_used,
            &produced,
            &op.rhs,
            subject,
            session_authority,
            ctx.accounts.host_config.chain_id,
            clock.slot,
            &mut instructions_sysvar_used,
            ctx.accounts
                .instructions_sysvar
                .as_ref()
                .map(|account| account.to_account_info())
                .as_ref(),
        )?;
        assert_binary_operand_types(
            op.op,
            lhs.handle,
            rhs.handle,
            rhs.scalar,
            op.output_fhe_type,
        )?;
        let expected_result = match &op.output {
            FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => {
                computed_eval_handle(
                    op.op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    op.output_fhe_type,
                    ctx.accounts.host_config.chain_id,
                    previous_bank_hash,
                    clock.unix_timestamp,
                    args.context_id,
                    index as u16,
                )
            }
            FheEvalOutput::Durable {
                output_nonce_key,
                output_nonce_sequence,
                ..
            } => computed_bound_eval_handle(
                op.op,
                lhs.handle,
                rhs.handle,
                rhs.scalar,
                op.output_fhe_type,
                ctx.accounts.host_config.chain_id,
                previous_bank_hash,
                clock.unix_timestamp,
                args.context_id,
                index as u16,
                *output_nonce_key,
                *output_nonce_sequence,
            ),
        };
        require!(
            op.result == expected_result,
            ZamaHostError::ComputedHandleMismatch
        );
        require!(
            !produced.iter().any(|value| value.handle == op.result),
            ZamaHostError::FheEvalDuplicateHandle
        );

        binary_events.push(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: op.op,
            subject: subject.to_bytes(),
            lhs: lhs.handle,
            rhs: rhs.handle,
            scalar: rhs.scalar,
            result: op.result,
        });

        let output_policies = input_session_policies(&lhs, &rhs);
        let output_public_decrypt_allowed = inputs_allow_public_decrypt(&lhs, &rhs);

        match &op.output {
            FheEvalOutput::Transient => {}
            FheEvalOutput::TransientSession {
                session_index,
                capability,
            } => {
                assert_session_policies_allow_transient_grant(&output_policies, *capability)?;
                let session_info = remaining_account(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    *session_index,
                )?;
                append_transient_capability(
                    session_info,
                    session_authority,
                    clock.slot,
                    op.result,
                    *capability,
                )?;
            }
            FheEvalOutput::Durable {
                output_acl_record_index,
                output_nonce_key,
                output_nonce_sequence,
                output_acl_domain_key,
                output_app_account,
                output_encrypted_value_label,
                output_subjects,
                output_public_decrypt,
            } => {
                assert_session_policies_allow_output(
                    &output_policies,
                    *output_acl_domain_key,
                    *output_app_account,
                    output_subjects,
                    *output_public_decrypt,
                )?;
                bind_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    *output_acl_record_index,
                    op.result,
                    *output_nonce_key,
                    *output_nonce_sequence,
                    *output_acl_domain_key,
                    *output_app_account,
                    *output_encrypted_value_label,
                    output_subjects,
                    *output_public_decrypt,
                )?;
            }
        }

        produced.push(ProducedValue {
            handle: op.result,
            public_decrypt_allowed: output_public_decrypt_allowed,
            session_policies: output_policies,
        });
    }

    require!(
        remaining_accounts_used.iter().all(|used| *used),
        ZamaHostError::InvalidFheEvalAccount
    );
    if !instructions_sysvar_used {
        require!(
            ctx.accounts.instructions_sysvar.is_none(),
            ZamaHostError::InvalidFheEvalAccount
        );
    }

    for event in binary_events {
        emit_cpi!(event);
    }

    Ok(())
}

fn resolve_lhs_operand<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    remaining_accounts_used: &mut [bool],
    produced: &[ProducedValue],
    operand: &FheEvalOperand,
    subject: Pubkey,
    session_authority: Pubkey,
    chain_id: u64,
    current_slot: u64,
    instructions_sysvar_used: &mut bool,
    instructions_sysvar: Option<&AccountInfo>,
) -> Result<ResolvedOperand> {
    match operand {
        FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
        _ => resolve_encrypted_operand(
            remaining_accounts,
            remaining_accounts_used,
            produced,
            operand,
            subject,
            session_authority,
            chain_id,
            current_slot,
            instructions_sysvar_used,
            instructions_sysvar,
        ),
    }
}

fn resolve_rhs_operand<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    remaining_accounts_used: &mut [bool],
    produced: &[ProducedValue],
    operand: &FheEvalOperand,
    subject: Pubkey,
    session_authority: Pubkey,
    chain_id: u64,
    current_slot: u64,
    instructions_sysvar_used: &mut bool,
    instructions_sysvar: Option<&AccountInfo>,
) -> Result<ResolvedOperand> {
    match operand {
        FheEvalOperand::Scalar(value) => Ok(ResolvedOperand::scalar(*value)),
        _ => resolve_encrypted_operand(
            remaining_accounts,
            remaining_accounts_used,
            produced,
            operand,
            subject,
            session_authority,
            chain_id,
            current_slot,
            instructions_sysvar_used,
            instructions_sysvar,
        ),
    }
}

fn resolve_encrypted_operand<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    remaining_accounts_used: &mut [bool],
    produced: &[ProducedValue],
    operand: &FheEvalOperand,
    subject: Pubkey,
    session_authority: Pubkey,
    chain_id: u64,
    current_slot: u64,
    instructions_sysvar_used: &mut bool,
    instructions_sysvar: Option<&AccountInfo>,
) -> Result<ResolvedOperand> {
    match operand {
        FheEvalOperand::Durable {
            handle,
            acl_record_index,
            permission_index,
        } => {
            let record_info = remaining_account(
                remaining_accounts,
                remaining_accounts_used,
                *acl_record_index,
            )?;
            let permission_info = permission_index
                .map(|index| remaining_account(remaining_accounts, remaining_accounts_used, index))
                .transpose()?;
            assert_unchecked_acl_record_subject_role(
                record_info,
                *handle,
                chain_id,
                subject,
                ACL_ROLE_USE,
                permission_info,
            )?;
            let public_decrypt_allowed = unchecked_acl_record_subject_has_role(
                record_info,
                *handle,
                subject,
                ACL_ROLE_PUBLIC_DECRYPT,
                permission_info,
            )?;
            Ok(ResolvedOperand::encrypted(*handle, public_decrypt_allowed))
        }
        FheEvalOperand::Transient { producer_index } => produced
            .get(*producer_index as usize)
            .map(ResolvedOperand::from_produced)
            .ok_or_else(|| error!(ZamaHostError::FheEvalTransientMissing)),
        FheEvalOperand::TransientSession {
            handle,
            session_index,
            capability_index,
        } => {
            let session_info =
                remaining_account(remaining_accounts, remaining_accounts_used, *session_index)?;
            *instructions_sysvar_used = true;
            let capability = consume_transient_capability(
                session_info,
                session_authority,
                current_slot,
                *handle,
                subject,
                ACL_ROLE_USE,
                *capability_index,
                instructions_sysvar,
            )?;
            Ok(ResolvedOperand {
                handle: *handle,
                scalar: false,
                public_decrypt_allowed: capability.grant.public_decrypt_allowed,
                session_policies: vec![SessionPolicy {
                    subject: capability.grant.subject,
                    receiver_program: capability.grant.receiver_program,
                    role_flags: capability.grant.role_flags,
                    max_uses: capability.grant.max_uses,
                    durable_output_allowed: capability.grant.durable_output_allowed,
                    public_decrypt_allowed: capability.grant.public_decrypt_allowed,
                    acl_domain_key: capability.grant.acl_domain_key,
                    app_account: capability.grant.app_account,
                }],
            })
        }
        FheEvalOperand::Scalar(_) => Err(error!(ZamaHostError::InvalidFheEvalAccount)),
    }
}

#[derive(Clone)]
struct ProducedValue {
    handle: [u8; 32],
    public_decrypt_allowed: bool,
    session_policies: Vec<SessionPolicy>,
}

#[derive(Clone)]
struct ResolvedOperand {
    handle: [u8; 32],
    scalar: bool,
    public_decrypt_allowed: bool,
    session_policies: Vec<SessionPolicy>,
}

impl ResolvedOperand {
    fn encrypted(handle: [u8; 32], public_decrypt_allowed: bool) -> Self {
        Self {
            handle,
            scalar: false,
            public_decrypt_allowed,
            session_policies: Vec::new(),
        }
    }

    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
            public_decrypt_allowed: true,
            session_policies: Vec::new(),
        }
    }

    fn from_produced(value: &ProducedValue) -> Self {
        Self {
            handle: value.handle,
            scalar: false,
            public_decrypt_allowed: value.public_decrypt_allowed,
            session_policies: value.session_policies.clone(),
        }
    }
}

#[derive(Clone, Copy)]
struct SessionPolicy {
    subject: Pubkey,
    receiver_program: Pubkey,
    role_flags: u8,
    max_uses: u8,
    durable_output_allowed: bool,
    public_decrypt_allowed: bool,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
}

fn input_session_policies(lhs: &ResolvedOperand, rhs: &ResolvedOperand) -> Vec<SessionPolicy> {
    let mut policies = Vec::with_capacity(lhs.session_policies.len() + rhs.session_policies.len());
    policies.extend_from_slice(&lhs.session_policies);
    policies.extend_from_slice(&rhs.session_policies);
    policies
}

fn inputs_allow_public_decrypt(lhs: &ResolvedOperand, rhs: &ResolvedOperand) -> bool {
    lhs.public_decrypt_allowed && rhs.public_decrypt_allowed
}

fn assert_session_policies_allow_output(
    policies: &[SessionPolicy],
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_subjects: &[AclSubjectEntry],
    output_public_decrypt: bool,
) -> Result<()> {
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;
    for policy in policies {
        require!(
            policy.durable_output_allowed,
            ZamaHostError::TransientCapabilityOutputDenied
        );
        require_keys_eq!(
            policy.acl_domain_key,
            output_acl_domain_key,
            ZamaHostError::TransientCapabilityOutputDenied
        );
        require_keys_eq!(
            policy.app_account,
            output_app_account,
            ZamaHostError::TransientCapabilityOutputDenied
        );
        for output_subject in output_subjects {
            require_keys_eq!(
                output_subject.pubkey,
                policy.subject,
                ZamaHostError::TransientCapabilityOutputDenied
            );
            require!(
                output_subject.role_flags & !policy.role_flags == 0,
                ZamaHostError::TransientCapabilityOutputDenied
            );
        }
    }
    Ok(())
}

fn assert_session_policies_allow_transient_grant(
    policies: &[SessionPolicy],
    grant: TransientCapabilityGrant,
) -> Result<()> {
    for policy in policies {
        require_keys_eq!(
            policy.subject,
            grant.subject,
            ZamaHostError::TransientCapabilityUnauthorized
        );
        require_keys_eq!(
            policy.receiver_program,
            grant.receiver_program,
            ZamaHostError::TransientCapabilityUnauthorized
        );
        require!(
            grant.role_flags & !policy.role_flags == 0,
            ZamaHostError::TransientCapabilityUnauthorized
        );
        require!(
            grant.max_uses <= policy.max_uses,
            ZamaHostError::TransientCapabilityConsumed
        );
        if grant.durable_output_allowed {
            require!(
                policy.durable_output_allowed,
                ZamaHostError::TransientCapabilityOutputDenied
            );
            require_keys_eq!(
                policy.acl_domain_key,
                grant.acl_domain_key,
                ZamaHostError::TransientCapabilityOutputDenied
            );
            require_keys_eq!(
                policy.app_account,
                grant.app_account,
                ZamaHostError::TransientCapabilityOutputDenied
            );
        }
        if grant.public_decrypt_allowed {
            require!(
                policy.public_decrypt_allowed,
                ZamaHostError::TransientCapabilityPublicDecryptDenied
            );
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn bind_eval_output<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    remaining_accounts_used: &mut [bool],
    output_acl_record_index: u16,
    result: [u8; 32],
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: &[AclSubjectEntry],
    output_public_decrypt: bool,
) -> Result<()> {
    assert_output_acl_metadata(
        ctx.accounts.app_account_authority.key(),
        output_nonce_key,
        output_acl_domain_key,
        output_app_account,
        output_encrypted_value_label,
        output_subjects,
    )?;
    assert_public_decrypt_not_set_at_birth(output_public_decrypt)?;

    let output_info = remaining_account(
        ctx.remaining_accounts,
        remaining_accounts_used,
        output_acl_record_index,
    )?;
    let (expected, bump) = acl_record_address(output_nonce_key, output_nonce_sequence);
    require_keys_eq!(
        output_info.key(),
        expected,
        ZamaHostError::AclRecordPdaMismatch
    );
    let sequence_bytes = output_nonce_sequence.to_le_bytes();
    create_pda_strict(
        &ctx.accounts.payer.to_account_info(),
        output_info,
        &ctx.accounts.system_program.to_account_info(),
        8 + AclRecord::SPACE,
        &[
            ACL_RECORD_SEED,
            output_nonce_key.as_ref(),
            &sequence_bytes,
            &[bump],
        ],
    )?;

    let mut subjects = [Pubkey::default(); MAX_ACL_SUBJECTS];
    let mut subject_roles = [0; MAX_ACL_SUBJECTS];
    for (index, subject) in output_subjects.iter().enumerate() {
        subjects[index] = subject.pubkey;
        subject_roles[index] = subject.role_flags;
    }
    let record = AclRecord {
        handle: result,
        nonce_key: output_nonce_key,
        nonce_sequence: output_nonce_sequence,
        acl_domain_key: output_acl_domain_key,
        app_account: output_app_account,
        encrypted_value_label: output_encrypted_value_label,
        subjects,
        subject_roles,
        subject_count: output_subjects.len() as u8,
        overflow_subject_count: 0,
        public_decrypt: output_public_decrypt,
        material_commitment: Pubkey::default(),
        material_commitment_hash: [0; 32],
        material_key_id: [0; 32],
        created_slot: Clock::get()?.slot,
        bump,
    };
    write_account(output_info, &record)?;

    emit_record_bound(output_info.key(), &record);
    for output_subject in output_subjects.iter().copied() {
        emit_cpi!(AclAllowedEvent {
            version: EVENT_VERSION,
            handle: result,
            subject: output_subject.pubkey.to_bytes(),
        });
        emit_subject_event(output_info.key(), result, output_subject, Pubkey::default());
    }
    Ok(())
}

fn remaining_account<'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
    remaining_accounts_used: &mut [bool],
    index: u16,
) -> Result<&'info AccountInfo<'info>> {
    let account_index = index as usize;
    let account = remaining_accounts
        .get(account_index)
        .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))?;
    remaining_accounts_used[account_index] = true;
    Ok(account)
}
