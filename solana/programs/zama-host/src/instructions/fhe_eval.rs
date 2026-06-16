//! Evaluates ordered instruction-local FHE plans.

use anchor_lang::prelude::*;

use super::common::*;
use crate::{
    errors::ZamaHostError,
    events::{
        AclAllowedEvent, AclRecordBoundEvent, AclSubjectAllowedEvent, FheBinaryOpEvent,
        FheRandEvent, FheTernaryOpEvent, TrivialEncryptEvent,
    },
    state::*,
};

mod admission;
mod event_budget;
mod event_transport;
mod handles;
mod preflight;
mod walk;

use admission::admit_eval_frame;
use event_budget::eval_event_capacity;
use event_transport::{emit_eval_events, EvalEvent};
use handles::EvalHandleContext;
use preflight::{assert_eval_step_birth_policy, preflight_eval_frame};
use walk::{walk_eval_frame, EvalStepVisitor};

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
    let current_slot = clock.slot;
    let handle_context = EvalHandleContext {
        chain_id: ctx.accounts.host_config.chain_id,
        previous_bank_hash: &previous_bank_hash,
        unix_timestamp: clock.unix_timestamp,
        context_id: &args.context_id,
    };
    admit_eval_frame(
        &ctx,
        &args,
        subject,
        session_authority,
        current_slot,
        &handle_context,
        instructions_sysvar.as_ref(),
    )?;
    let events = execute_eval_frame(
        &ctx,
        &args,
        subject,
        session_authority,
        current_slot,
        &handle_context,
        instructions_sysvar.as_ref(),
    )?;
    emit_eval_events(&ctx, events)?;
    Ok(())
}

#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn execute_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    session_authority: Pubkey,
    current_slot: u64,
    handle_context: &EvalHandleContext<'_>,
    instructions_sysvar: Option<&AccountInfo<'info>>,
) -> Result<Vec<EvalEvent>> {
    let mut execution = EvalExecutionState::new(
        ctx.remaining_accounts,
        args.steps.len(),
        eval_event_capacity(args),
        subject,
        session_authority,
        handle_context.chain_id,
        current_slot,
        instructions_sysvar,
    );
    walk_eval_frame(&mut execution, ctx, args, handle_context)?;
    execution.finish(ctx)
}

/// Execution phase: resolves operands while marking the dynamic accounts used,
/// mutates transient sessions and creates durable output ACL records, and
/// buffers the events for transport.
struct EvalExecutionState<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: Vec<bool>,
    produced: Vec<ProducedValue>,
    events: Vec<EvalEvent>,
    instructions_sysvar_used: bool,
    subject: Pubkey,
    session_authority: Pubkey,
    chain_id: u64,
    current_slot: u64,
    instructions_sysvar: Option<&'a AccountInfo<'info>>,
}

impl<'a, 'info> EvalExecutionState<'a, 'info> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        step_count: usize,
        event_capacity: usize,
        subject: Pubkey,
        session_authority: Pubkey,
        chain_id: u64,
        current_slot: u64,
        instructions_sysvar: Option<&'a AccountInfo<'info>>,
    ) -> Self {
        Self {
            remaining_accounts,
            remaining_accounts_used: vec![false; remaining_accounts.len()],
            produced: Vec::with_capacity(step_count),
            events: Vec::with_capacity(event_capacity),
            instructions_sysvar_used: false,
            subject,
            session_authority,
            chain_id,
            current_slot,
            instructions_sysvar,
        }
    }

    fn remaining_account(&mut self, index: u16) -> Result<&'a AccountInfo<'info>> {
        let account_index = index as usize;
        let account = self
            .remaining_accounts
            .get(account_index)
            .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))?;
        self.remaining_accounts_used[account_index] = true;
        Ok(account)
    }

    fn finish(self, ctx: &Context<'info, FheEval<'info>>) -> Result<Vec<EvalEvent>> {
        require!(
            self.remaining_accounts_used.iter().all(|used| *used),
            ZamaHostError::InvalidFheEvalAccount
        );
        if !self.instructions_sysvar_used {
            require!(
                ctx.accounts.instructions_sysvar.is_none(),
                ZamaHostError::InvalidFheEvalAccount
            );
        }
        Ok(self.events)
    }
}

impl EvalStepVisitor for EvalExecutionState<'_, '_> {
    fn subject(&self) -> Pubkey {
        self.subject
    }

    fn produced(&self) -> &[ProducedValue] {
        &self.produced
    }

    #[inline(never)]
    fn resolve_durable_operand(
        &mut self,
        handle: [u8; 32],
        acl_record_index: u16,
        permission_index: Option<u16>,
    ) -> Result<ResolvedOperand> {
        let record_info = self.remaining_account(acl_record_index)?;
        let permission_info = permission_index
            .map(|index| self.remaining_account(index))
            .transpose()?;
        assert_unchecked_acl_record_subject_role(
            record_info,
            handle,
            self.chain_id,
            self.subject,
            ACL_ROLE_USE,
            permission_info,
        )?;
        let public_decrypt_allowed = unchecked_acl_record_subject_has_role(
            record_info,
            handle,
            self.subject,
            ACL_ROLE_PUBLIC_DECRYPT,
            permission_info,
        )?;
        Ok(ResolvedOperand::encrypted(handle, public_decrypt_allowed))
    }

    #[inline(never)]
    fn resolve_transient_session_operand(
        &mut self,
        handle: [u8; 32],
        session_index: u16,
        capability_index: u16,
    ) -> Result<ResolvedOperand> {
        let session_info = self.remaining_account(session_index)?;
        self.instructions_sysvar_used = true;
        let capability = consume_transient_capability(
            session_info,
            self.session_authority,
            self.current_slot,
            handle,
            self.subject,
            ACL_ROLE_USE,
            capability_index,
            self.instructions_sysvar,
        )?;
        Ok(ResolvedOperand::transient_session(handle, capability.grant))
    }

    fn record_op_event(&mut self, event: EvalEvent) {
        self.events.push(event);
    }

    #[inline(never)]
    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_policies: Vec<SessionPolicy>,
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
    ) -> Result<()> {
        accept_eval_output(
            ctx,
            &mut self.remaining_accounts_used,
            &mut self.produced,
            &mut self.events,
            result,
            output,
            output_policies,
            output_public_decrypt_allowed,
            enforce_public_decrypt_role_propagation,
            self.current_slot,
        )
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

#[inline(never)]
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
            let app_account_authority = durable_output_authority(
                ctx,
                remaining_accounts_used,
                *output_app_account_authority_index,
                *output_app_account,
            )?;
            if enforce_public_decrypt_role_propagation {
                assert_derived_public_decrypt_roles_allowed(
                    output_subjects,
                    output_public_decrypt_allowed,
                    &app_account_authority,
                )?;
            }
            bind_eval_output(
                ctx,
                remaining_accounts_used,
                events,
                *output_acl_record_index,
                result,
                app_account_authority.key(),
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
) -> Result<AccountInfo<'info>> {
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
            Ok(authority.clone())
        }
        None => Ok(ctx.accounts.app_account_authority.to_account_info()),
    }
}

#[derive(Clone)]
pub(super) struct ProducedValue {
    handle: [u8; 32],
    public_decrypt_allowed: bool,
    session_policies: Vec<SessionPolicy>,
}

#[derive(Clone)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
    pub(super) public_decrypt_allowed: bool,
    pub(super) session_policies: Vec<SessionPolicy>,
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

    /// Builds an operand from a transient-session capability grant, carrying the
    /// grant's policy forward so any output it produces stays within the grant.
    fn transient_session(handle: [u8; 32], grant: TransientCapabilityGrant) -> Self {
        Self {
            handle,
            scalar: false,
            public_decrypt_allowed: grant.public_decrypt_allowed,
            session_policies: vec![SessionPolicy {
                subject: grant.subject,
                receiver_program: grant.receiver_program,
                role_flags: grant.role_flags,
                max_uses: grant.max_uses,
                durable_output_allowed: grant.durable_output_allowed,
                public_decrypt_allowed: grant.public_decrypt_allowed,
                acl_domain_key: grant.acl_domain_key,
                app_account: grant.app_account,
            }],
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct SessionPolicy {
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

#[inline(never)]
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

fn remaining_account<'a, 'info>(
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: &mut [bool],
    index: u16,
) -> Result<&'a AccountInfo<'info>> {
    let account_index = index as usize;
    let account = remaining_accounts
        .get(account_index)
        .ok_or_else(|| error!(ZamaHostError::InvalidFheEvalAccount))?;
    remaining_accounts_used[account_index] = true;
    Ok(account)
}
