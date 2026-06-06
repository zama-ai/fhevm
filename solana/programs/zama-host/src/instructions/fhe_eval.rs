//! Evaluates ordered instruction-local FHE plans.

use anchor_lang::prelude::*;

use super::common::*;
use super::verify_input_and_bind::{assert_input_proof, assert_previous_ed25519_instruction};
use crate::{
    errors::ZamaHostError,
    events::{
        AclAllowedEvent, AclRecordBoundEvent, AclSubjectAllowedEvent, FheBinaryOpEvent,
        FheRandEvent, FheTernaryOpEvent, InputVerifiedEvent, TrivialEncryptEvent,
    },
    state::*,
};

mod admission;
mod event_budget;
mod event_transport;
mod handles;
mod preflight;

use admission::admit_eval_frame;
use event_budget::eval_event_capacity;
use event_transport::{emit_eval_events, EvalEvent};
use handles::{
    expected_binary_eval_result, expected_rand_eval_seed, expected_ternary_eval_result,
    expected_trivial_eval_result,
};
use preflight::{assert_eval_step_birth_policy, preflight_eval_frame};

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

/// Executes an ordered FHE plan with instruction-local transient outputs.
pub fn fhe_eval<'info>(ctx: Context<'info, FheEval<'info>>, args: FheEvalArgs) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        args.context_id != [0; 32],
        ZamaHostError::InvalidFheEvalContext
    );
    require!(
        !args.steps.is_empty() && args.steps.len() <= MAX_FHE_EVAL_OPS,
        ZamaHostError::InvalidFheEvalOperationCount
    );
    for step in &args.steps {
        assert_eval_step_birth_policy(step)?;
    }
    let instructions_sysvar = ctx
        .accounts
        .instructions_sysvar
        .as_ref()
        .map(|account| account.to_account_info());
    preflight_eval_frame(ctx.remaining_accounts, &args, instructions_sysvar.as_ref())?;

    let subject = ctx.accounts.compute_subject.key();
    let session_authority = ctx.accounts.app_account_authority.key();
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash_with_test_fallback(
        clock.slot,
        ctx.accounts.host_config.zero_birth_entropy_allowed(),
    )?;
    admit_eval_frame(
        &ctx,
        &args,
        subject,
        session_authority,
        previous_bank_hash,
        clock.slot,
        clock.unix_timestamp,
        instructions_sysvar.as_ref(),
    )?;
    let mut produced = Vec::with_capacity(args.steps.len());
    let mut events = Vec::with_capacity(eval_event_capacity(&args));
    let mut remaining_accounts_used = vec![false; ctx.remaining_accounts.len()];
    let mut instructions_sysvar_used = false;
    let instructions_sysvar = instructions_sysvar.as_ref();

    for (index, step) in args.steps.iter().enumerate() {
        match step {
            FheEvalStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                output,
            } => {
                let lhs = resolve_lhs_operand(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    &produced,
                    lhs,
                    subject,
                    session_authority,
                    ctx.accounts.host_config.chain_id,
                    clock.slot,
                    &mut instructions_sysvar_used,
                    instructions_sysvar,
                )?;
                let rhs = resolve_rhs_operand(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    &produced,
                    rhs,
                    subject,
                    session_authority,
                    ctx.accounts.host_config.chain_id,
                    clock.slot,
                    &mut instructions_sysvar_used,
                    instructions_sysvar,
                )?;
                assert_binary_operand_types(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                )?;
                let expected_result = expected_binary_eval_result(
                    *op,
                    lhs.handle,
                    rhs.handle,
                    rhs.scalar,
                    *output_fhe_type,
                    ctx.accounts.host_config.chain_id,
                    previous_bank_hash,
                    clock.unix_timestamp,
                    args.context_id,
                    index as u16,
                    output,
                );
                events.push(EvalEvent::Binary(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: *op,
                    subject: subject.to_bytes(),
                    lhs: lhs.handle,
                    rhs: rhs.handle,
                    scalar: rhs.scalar,
                    result: expected_result,
                }));
                accept_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    &mut produced,
                    &mut events,
                    expected_result,
                    output,
                    input_session_policies(&lhs, &rhs),
                    inputs_allow_public_decrypt(&lhs, &rhs),
                    true,
                    clock.slot,
                )?;
            }
            FheEvalStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                output,
            } => {
                let control = resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    &produced,
                    control,
                    subject,
                    session_authority,
                    ctx.accounts.host_config.chain_id,
                    clock.slot,
                    &mut instructions_sysvar_used,
                    instructions_sysvar,
                )?;
                let if_true = resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    &produced,
                    if_true,
                    subject,
                    session_authority,
                    ctx.accounts.host_config.chain_id,
                    clock.slot,
                    &mut instructions_sysvar_used,
                    instructions_sysvar,
                )?;
                let if_false = resolve_encrypted_operand(
                    ctx.remaining_accounts,
                    &mut remaining_accounts_used,
                    &produced,
                    if_false,
                    subject,
                    session_authority,
                    ctx.accounts.host_config.chain_id,
                    clock.slot,
                    &mut instructions_sysvar_used,
                    instructions_sysvar,
                )?;
                assert_ternary_operand_types(
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                )?;
                let expected_result = expected_ternary_eval_result(
                    *op,
                    control.handle,
                    if_true.handle,
                    if_false.handle,
                    *output_fhe_type,
                    ctx.accounts.host_config.chain_id,
                    previous_bank_hash,
                    clock.unix_timestamp,
                    args.context_id,
                    index as u16,
                    output,
                );
                events.push(EvalEvent::Ternary(FheTernaryOpEvent {
                    version: EVENT_VERSION,
                    op: *op,
                    subject: subject.to_bytes(),
                    control: control.handle,
                    if_true: if_true.handle,
                    if_false: if_false.handle,
                    result: expected_result,
                }));
                accept_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    &mut produced,
                    &mut events,
                    expected_result,
                    output,
                    input_session_policies3(&control, &if_true, &if_false),
                    inputs3_allow_public_decrypt(&control, &if_true, &if_false),
                    true,
                    clock.slot,
                )?;
            }
            FheEvalStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                output,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                let result = expected_trivial_eval_result(
                    *plaintext,
                    *fhe_type,
                    ctx.accounts.host_config.chain_id,
                    previous_bank_hash,
                    clock.unix_timestamp,
                    args.context_id,
                    index as u16,
                    output,
                );
                events.push(EvalEvent::Trivial(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                }));
                accept_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    &mut produced,
                    &mut events,
                    result,
                    output,
                    Vec::new(),
                    false,
                    false,
                    clock.slot,
                )?;
            }
            FheEvalStep::Rand { fhe_type, output } => {
                assert_supported_rand_type(*fhe_type)?;
                let seed = expected_rand_eval_seed(
                    ctx.accounts.host_config.chain_id,
                    previous_bank_hash,
                    clock.unix_timestamp,
                    args.context_id,
                    index as u16,
                    output,
                );
                let result =
                    computed_rand_handle(seed, *fhe_type, ctx.accounts.host_config.chain_id);
                events.push(EvalEvent::Rand(FheRandEvent {
                    version: EVENT_VERSION,
                    subject: subject.to_bytes(),
                    seed,
                    fhe_type: *fhe_type,
                    result,
                }));
                accept_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    &mut produced,
                    &mut events,
                    result,
                    output,
                    Vec::new(),
                    false,
                    false,
                    clock.slot,
                )?;
            }
            FheEvalStep::Input {
                input_handle,
                proof,
                output,
            } => {
                let durable = durable_output(output)?;
                assert_input_proof(
                    proof,
                    *input_handle,
                    ctx.accounts.host_config.chain_id,
                    durable.output_acl_domain_key,
                    durable.output_app_account,
                )?;
                let bind_intent = SolanaInputBindIntent {
                    output_nonce_key: durable.output_nonce_key,
                    output_nonce_sequence: durable.output_nonce_sequence,
                    output_acl_domain_key: durable.output_acl_domain_key,
                    output_app_account: durable.output_app_account,
                    output_encrypted_value_label: durable.output_encrypted_value_label,
                    output_subjects: durable.output_subjects.to_vec(),
                    output_public_decrypt: durable.output_public_decrypt,
                };
                let proof_message = input_proof_message(
                    proof,
                    &bind_intent,
                    crate::ID,
                    ctx.accounts.host_config.chain_id,
                );
                let instructions_sysvar = ctx
                    .accounts
                    .instructions_sysvar
                    .as_ref()
                    .ok_or(ZamaHostError::InputProofSignatureMissing)?
                    .to_account_info();
                instructions_sysvar_used = true;
                assert_previous_ed25519_instruction(
                    &instructions_sysvar,
                    ctx.accounts.host_config.input_verifier_authority,
                    &proof_message,
                )?;
                let output_public_decrypt_allowed =
                    output_subjects_grant_public_decrypt(durable.output_subjects);
                events.push(EvalEvent::Input(InputVerifiedEvent {
                    version: EVENT_VERSION,
                    input_handle: *input_handle,
                    result_handle: *input_handle,
                    user: proof.user.to_bytes(),
                    acl_domain_key: durable.output_acl_domain_key.to_bytes(),
                }));
                accept_eval_output(
                    &ctx,
                    &mut remaining_accounts_used,
                    &mut produced,
                    &mut events,
                    *input_handle,
                    output,
                    Vec::new(),
                    output_public_decrypt_allowed,
                    false,
                    clock.slot,
                )?;
            }
        }
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

    emit_eval_events(&ctx, events)?;
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

struct DurableOutputRef<'a> {
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: &'a [AclSubjectEntry],
    output_public_decrypt: bool,
}

fn durable_output(output: &FheEvalOutput) -> Result<DurableOutputRef<'_>> {
    match output {
        FheEvalOutput::Durable {
            output_nonce_key,
            output_nonce_sequence,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            output_public_decrypt,
            ..
        } => Ok(DurableOutputRef {
            output_nonce_key: *output_nonce_key,
            output_nonce_sequence: *output_nonce_sequence,
            output_acl_domain_key: *output_acl_domain_key,
            output_app_account: *output_app_account,
            output_encrypted_value_label: *output_encrypted_value_label,
            output_subjects,
            output_public_decrypt: *output_public_decrypt,
        }),
        FheEvalOutput::Transient | FheEvalOutput::TransientSession { .. } => {
            Err(error!(ZamaHostError::InvalidFheEvalAccount))
        }
    }
}

fn assert_ternary_operand_types(
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_fhe_type(output_fhe_type)?;
    require!(
        handle_fhe_type(control) == 0
            && handle_fhe_type(if_true) == output_fhe_type
            && handle_fhe_type(if_false) == output_fhe_type,
        ZamaHostError::InvalidInputHandleType
    );
    Ok(())
}

fn accept_eval_output<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    remaining_accounts_used: &mut [bool],
    produced: &mut Vec<ProducedValue>,
    events: &mut Vec<EvalEvent>,
    result: [u8; 32],
    output: &FheEvalOutput,
    output_policies: Vec<SessionPolicy>,
    output_public_decrypt_allowed: bool,
    enforce_public_decrypt_role_propagation: bool,
    current_slot: u64,
) -> Result<()> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheEvalDuplicateHandle
    );

    match output {
        FheEvalOutput::Transient => {}
        FheEvalOutput::TransientSession {
            session_index,
            capability,
        } => {
            assert_session_policies_allow_transient_grant(&output_policies, *capability)?;
            let session_info = remaining_account(
                ctx.remaining_accounts,
                remaining_accounts_used,
                *session_index,
            )?;
            append_transient_capability(
                session_info,
                ctx.accounts.app_account_authority.key(),
                current_slot,
                result,
                *capability,
            )?;
        }
        FheEvalOutput::Durable {
            output_acl_record_index,
            output_app_account_authority_index,
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
            if enforce_public_decrypt_role_propagation {
                assert_public_decrypt_roles_allowed(
                    output_subjects,
                    output_public_decrypt_allowed,
                )?;
            }
            let app_account_authority = durable_output_authority(
                ctx,
                remaining_accounts_used,
                *output_app_account_authority_index,
                *output_app_account,
            )?;
            bind_eval_output(
                ctx,
                remaining_accounts_used,
                events,
                *output_acl_record_index,
                result,
                app_account_authority,
                *output_nonce_key,
                *output_nonce_sequence,
                *output_acl_domain_key,
                *output_app_account,
                *output_encrypted_value_label,
                output_subjects,
                *output_public_decrypt,
                current_slot,
            )?
        }
    };

    produced.push(ProducedValue {
        handle: result,
        public_decrypt_allowed: output_public_decrypt_allowed,
        session_policies: output_policies,
    });
    Ok(())
}

fn durable_output_authority<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    remaining_accounts_used: &mut [bool],
    authority_index: Option<u16>,
    output_app_account: Pubkey,
) -> Result<Pubkey> {
    match authority_index {
        Some(index) => {
            let authority =
                remaining_account(ctx.remaining_accounts, remaining_accounts_used, index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheEvalAccount);
            require_keys_eq!(
                authority.key(),
                output_app_account,
                ZamaHostError::AppAccountAuthorityMismatch
            );
            Ok(authority.key())
        }
        None => Ok(ctx.accounts.app_account_authority.key()),
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

fn input_session_policies3(
    first: &ResolvedOperand,
    second: &ResolvedOperand,
    third: &ResolvedOperand,
) -> Vec<SessionPolicy> {
    let mut policies = Vec::with_capacity(
        first.session_policies.len() + second.session_policies.len() + third.session_policies.len(),
    );
    policies.extend_from_slice(&first.session_policies);
    policies.extend_from_slice(&second.session_policies);
    policies.extend_from_slice(&third.session_policies);
    policies
}

fn inputs_allow_public_decrypt(lhs: &ResolvedOperand, rhs: &ResolvedOperand) -> bool {
    lhs.public_decrypt_allowed && rhs.public_decrypt_allowed
}

fn inputs3_allow_public_decrypt(
    first: &ResolvedOperand,
    second: &ResolvedOperand,
    third: &ResolvedOperand,
) -> bool {
    first.public_decrypt_allowed && second.public_decrypt_allowed && third.public_decrypt_allowed
}

fn assert_public_decrypt_roles_allowed(
    output_subjects: &[AclSubjectEntry],
    output_public_decrypt_allowed: bool,
) -> Result<()> {
    require!(
        !output_subjects_grant_public_decrypt(output_subjects) || output_public_decrypt_allowed,
        ZamaHostError::TransientCapabilityPublicDecryptDenied
    );
    Ok(())
}

fn output_subjects_grant_public_decrypt(output_subjects: &[AclSubjectEntry]) -> bool {
    output_subjects
        .iter()
        .any(|subject| subject_has_role(subject.role_flags, ACL_ROLE_PUBLIC_DECRYPT))
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
    events: &mut Vec<EvalEvent>,
    output_acl_record_index: u16,
    result: [u8; 32],
    app_account_authority: Pubkey,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: &[AclSubjectEntry],
    output_public_decrypt: bool,
    current_slot: u64,
) -> Result<()> {
    assert_output_acl_metadata(
        app_account_authority,
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
        created_slot: current_slot,
        bump,
    };
    write_account(output_info, &record)?;

    let record_key = output_info.key();
    events.push(EvalEvent::AclRecordBound(AclRecordBoundEvent {
        version: EVENT_VERSION,
        acl_record: record_key,
        handle: record.handle,
        nonce_key: record.nonce_key,
        nonce_sequence: record.nonce_sequence,
        acl_domain_key: record.acl_domain_key,
        app_account: record.app_account,
        encrypted_value_label: record.encrypted_value_label,
        subject_count: record.subject_count,
        public_decrypt: record.public_decrypt,
        created_slot: record.created_slot,
    }));
    for output_subject in output_subjects.iter().copied() {
        events.push(EvalEvent::AclAllowed(AclAllowedEvent {
            version: EVENT_VERSION,
            handle: result,
            subject: output_subject.pubkey.to_bytes(),
        }));
        events.push(EvalEvent::AclSubjectAllowed(AclSubjectAllowedEvent {
            version: EVENT_VERSION,
            acl_record: record_key,
            handle: result,
            authority_subject: Pubkey::default(),
            subject: output_subject.pubkey.to_bytes(),
            role_flags: output_subject.role_flags,
            overflow_permission_record: Pubkey::default(),
            inline_index: u8::MAX,
            updated_slot: current_slot,
        }));
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
