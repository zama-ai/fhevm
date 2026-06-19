//! Evaluates ordered instruction-local FHE plans.

use anchor_lang::prelude::*;

use super::common::*;
use super::verify_coprocessor_input::{verify_input_attestation, InputVerifierParams};
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
    preflight_eval_frame(ctx.remaining_accounts, &args)?;

    let subject = ctx.accounts.compute_subject.key();
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
    admit_eval_frame(&ctx, &args, subject, &handle_context)?;
    let events = execute_eval_frame(&ctx, &args, subject, current_slot, &handle_context)?;
    emit_eval_events(&ctx, events)?;
    Ok(())
}

#[inline(never)]
fn execute_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    current_slot: u64,
    handle_context: &EvalHandleContext<'_>,
) -> Result<Vec<EvalEvent>> {
    let mut execution = EvalExecutionState::new(
        ctx.remaining_accounts,
        args.steps.len(),
        eval_event_capacity(args),
        subject,
        handle_context.chain_id,
        current_slot,
        InputVerifierParams::from_config(&ctx.accounts.host_config),
    );
    walk_eval_frame(&mut execution, ctx, args, handle_context)?;
    execution.finish()
}

/// Execution phase: resolves operands while marking the dynamic accounts used,
/// creates durable output ACL records, and buffers the events for transport.
struct EvalExecutionState<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: Vec<bool>,
    produced: Vec<ProducedValue>,
    events: Vec<EvalEvent>,
    subject: Pubkey,
    chain_id: u64,
    current_slot: u64,
    verifier_params: InputVerifierParams,
}

impl<'a, 'info> EvalExecutionState<'a, 'info> {
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        step_count: usize,
        event_capacity: usize,
        subject: Pubkey,
        chain_id: u64,
        current_slot: u64,
        verifier_params: InputVerifierParams,
    ) -> Self {
        Self {
            remaining_accounts,
            remaining_accounts_used: vec![false; remaining_accounts.len()],
            produced: Vec::with_capacity(step_count),
            events: Vec::with_capacity(event_capacity),
            subject,
            chain_id,
            current_slot,
            verifier_params,
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

    fn finish(self) -> Result<Vec<EvalEvent>> {
        require!(
            self.remaining_accounts_used.iter().all(|used| *used),
            ZamaHostError::InvalidFheEvalAccount
        );
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
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Authoritative in-frame verification: re-run the coprocessor attestation. No account, no
        // PDA — the "allow" exists only for this instruction's execution. public_decrypt is NOT
        // implied by a verified input; the durable output gets an explicit allow_for_decryption.
        verify_input_attestation(
            &self.verifier_params,
            attestation.input_handle,
            &attestation.ct_handles,
            attestation.handle_index,
            &attestation.user_address,
            &attestation.contract_address,
            attestation.contract_chain_id,
            &attestation.extra_data,
            &attestation.signatures,
        )?;
        Ok(ResolvedOperand::verified_input(
            attestation.input_handle,
            VerifiedInputBinding {
                user_address: Pubkey::new_from_array(attestation.user_address),
                contract_address: Pubkey::new_from_array(attestation.contract_address),
            },
        ))
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
        output_public_decrypt_allowed: bool,
        enforce_public_decrypt_role_propagation: bool,
        verified_input: Option<VerifiedInputBinding>,
    ) -> Result<()> {
        accept_eval_output(
            ctx,
            &mut self.remaining_accounts_used,
            &mut self.produced,
            &mut self.events,
            result,
            output,
            output_public_decrypt_allowed,
            enforce_public_decrypt_role_propagation,
            verified_input,
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
    output_public_decrypt_allowed: bool,
    enforce_public_decrypt_role_propagation: bool,
    verified_input: Option<VerifiedInputBinding>,
    current_slot: u64,
) -> Result<()> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheEvalDuplicateHandle
    );

    match output {
        FheEvalOutput::AllowedLocal => {}
        FheEvalOutput::AllowedDurable {
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
            if let Some(binding) = verified_input {
                assert_verified_input_output_binding(
                    &binding,
                    *output_acl_domain_key,
                    *output_app_account,
                    output_subjects,
                )?;
            }
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
        verified_input,
    });
    Ok(())
}

/// A durable output derived from a verified external input must bind the attested identities: the
/// output domain and app account must both be the attested `contract_address` (so the attested
/// domain owns the output — and since the output app account must itself sign, the attested domain
/// must sign, the EVM `contractAddress == msg.sender` invariant), and the attested `user_address`
/// must be one of the output subjects. This stops a copied (public) attestation from being turned
/// into a decryptable derived value by any signer other than the attested domain.
fn assert_verified_input_output_binding(
    binding: &VerifiedInputBinding,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_subjects: &[AclSubjectEntry],
) -> Result<()> {
    require_keys_eq!(
        output_acl_domain_key,
        binding.contract_address,
        ZamaHostError::AclDomainKeyMismatch
    );
    require_keys_eq!(
        output_app_account,
        binding.contract_address,
        ZamaHostError::InputBindContractMismatch
    );
    require!(
        output_subjects
            .iter()
            .any(|subject| subject.pubkey == binding.user_address),
        ZamaHostError::InputBindUserNotSubject
    );
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
    verified_input: Option<VerifiedInputBinding>,
}

/// The coprocessor-attested identities a verified external input carries forward. A durable output
/// derived from it must bind these: `output_acl_domain_key` and `output_app_account` must equal the
/// attested `contract_address` (so the attested domain — which `app_account` must sign for — owns
/// the output, the EVM `contractAddress == msg.sender` invariant), and the attested `user_address`
/// must be one of the output subjects. This makes a copied (public) attestation unusable by any
/// signer other than the attested domain.
#[derive(Clone, Copy)]
pub(super) struct VerifiedInputBinding {
    pub(super) user_address: Pubkey,
    pub(super) contract_address: Pubkey,
}

#[derive(Clone)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
    pub(super) public_decrypt_allowed: bool,
    /// Set when this value traces back to a verified external input: the attested identities its
    /// derived durable outputs must bind to (see [`VerifiedInputBinding`]).
    pub(super) verified_input: Option<VerifiedInputBinding>,
}

impl ResolvedOperand {
    fn encrypted(handle: [u8; 32], public_decrypt_allowed: bool) -> Self {
        Self {
            handle,
            scalar: false,
            public_decrypt_allowed,
            verified_input: None,
        }
    }

    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
            public_decrypt_allowed: true,
            verified_input: None,
        }
    }

    /// Builds an operand from an in-frame verified external input, carrying the attested identities
    /// its derived durable outputs must bind to. The input is authorized by its provider for that
    /// domain, so it propagates public-decrypt like a public scalar (EVM `fromExternal` parity: the
    /// app that received the input controls whether results are made publicly decryptable, via an
    /// explicit allow_for_decryption — it is not blocked by the input itself).
    fn verified_input(handle: [u8; 32], binding: VerifiedInputBinding) -> Self {
        Self {
            handle,
            scalar: false,
            public_decrypt_allowed: true,
            verified_input: Some(binding),
        }
    }

    fn from_produced(value: &ProducedValue) -> Self {
        Self {
            handle: value.handle,
            scalar: false,
            public_decrypt_allowed: value.public_decrypt_allowed,
            verified_input: value.verified_input,
        }
    }
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

/// The verified-input binding a derived output must satisfy. Folds the operands' taints: at most one
/// attested input may flow into a single op (mixing two distinct attestations would be ambiguous),
/// and a durable output derived from it must bind the attested domain/user (see [`accept_eval_output`]).
fn combine_verified_input_binding(
    operands: &[&ResolvedOperand],
) -> Result<Option<VerifiedInputBinding>> {
    let mut binding: Option<VerifiedInputBinding> = None;
    for operand in operands {
        if let Some(input) = operand.verified_input {
            match binding {
                None => binding = Some(input),
                Some(existing) => {
                    require_keys_eq!(
                        existing.contract_address,
                        input.contract_address,
                        ZamaHostError::AclDomainKeyMismatch
                    );
                    require_keys_eq!(
                        existing.user_address,
                        input.user_address,
                        ZamaHostError::InputBindUserNotSubject
                    );
                }
            }
        }
    }
    Ok(binding)
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
