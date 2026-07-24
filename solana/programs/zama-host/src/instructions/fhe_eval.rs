//! Evaluates ordered instruction-local FHE plans.

use anchor_lang::prelude::*;

use super::common::*;
use super::encrypted_value::{
    append_public_decrypt_leaf, grow_account_if_needed, supersede_current_handle,
};
use super::input_verification::{verify_input_attestation, InputVerifierParams};
use crate::{
    errors::ZamaHostError,
    events::{ProducedPublicOutput, PublicOutputsProducedEvent},
    state::*,
};

mod admission;
mod block_cap;
mod event_transport;
mod handles;
mod hcu;
mod preflight;
mod walk;

use admission::admit_eval_frame;
use event_transport::emit_public_outputs_produced;
use handles::EvalHandleContext;
use preflight::preflight_eval_frame;
use walk::{walk_eval_frame, EvalStepVisitor};

/// Accounts for composed instruction-local FHE evaluation.
///
/// Durable input and output `EncryptedValue` accounts are supplied in
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
    /// Singleton config PDA. Read-only: the cap is read from here, but the writable per-slot
    /// counter is the separate `hcu_block_meter`, never this singleton — so the hot path takes no
    /// write lock on the config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for durable output ACL creation.
    pub system_program: Program<'info, System>,
    /// Per-`compute_subject` HCU block meter (written once in the execution `charge`). The HCU PDAs
    /// (`hcu_block_meter`, `hcu_trusted_app_record`) key on `compute_subject` — the mandatory signed
    /// caller identity already used for ACL admission — so no caller can rotate a fresh signer to
    /// mint a fresh per-slot meter. Untrusted subjects in the metering band MUST supply this meter;
    /// trusted subjects and the unrestricted default omit it. An `UncheckedAccount` because it may
    /// be uninitialized (lazy-created) and is validated manually.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// Trust witness (read-only), keyed on `compute_subject`. Present + program-owned +
    /// `trusted == true` ⇒ bypass the cap; absent (`None`) ⇒ untrusted, fall through to the meter;
    /// present-but-malformed ⇒ reject.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
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
    preflight_eval_frame(&ctx, &args)?;

    let subject = ctx.accounts.compute_subject.key();
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    let handle_context = EvalHandleContext {
        chain_id: ctx.accounts.host_config.chain_id,
        previous_bank_hash: &previous_bank_hash,
        unix_timestamp: clock.unix_timestamp,
        context_id: &args.context_id,
    };
    let current_slot = clock.slot;
    // Admission (walk #1) computes the frame's HCU total; the read-only block-cap check then trips
    // an over-budget frame before execution burns CU or creates any ACL record.
    let frame_total = admit_eval_frame(&ctx, &args, subject, &handle_context)?;
    block_cap::check(&ctx, frame_total, current_slot)?;
    let born_public_outputs =
        execute_eval_frame(&ctx, &args, subject, current_slot, &handle_context)?;
    emit_public_outputs_produced(&ctx, born_public_outputs)?;
    Ok(())
}

#[inline(never)]
fn execute_eval_frame<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    args: &FheEvalArgs,
    subject: Pubkey,
    current_slot: u64,
    handle_context: &EvalHandleContext<'_>,
) -> Result<Vec<ProducedPublicOutput>> {
    let mut execution = EvalExecutionState::new(
        ctx.remaining_accounts,
        args.steps.len(),
        subject,
        handle_context.chain_id,
        InputVerifierParams::from_config(&ctx.accounts.host_config),
    );
    // Execution (walk #2) recomputes the same frame total; the block-cap charge is the single meter
    // write — lazy-create/reset, checked accumulate, cap assert, write once.
    let frame_total = walk_eval_frame(&mut execution, ctx, args, handle_context)?;
    block_cap::charge(ctx, frame_total, current_slot)?;
    execution.finish()
}

/// Execution phase: resolves operands while marking the dynamic accounts used,
/// creates durable output ACL records, and buffers produced-public lifecycle records.
struct EvalExecutionState<'a, 'info> {
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: Vec<bool>,
    produced: Vec<ProducedValue>,
    born_public_outputs: Vec<ProducedPublicOutput>,
    subject: Pubkey,
    chain_id: u64,
    verifier_params: InputVerifierParams,
    /// Handles superseded by this frame's own output bindings, keyed by
    /// encrypted value account. A later operand may still reference one (EVM parity:
    /// a handle stays usable as a value within the transaction that rotated
    /// it); admission already authorized it against frame-entry state.
    superseded_in_frame: Vec<(Pubkey, [u8; 32])>,
}

impl<'a, 'info> EvalExecutionState<'a, 'info> {
    fn new(
        remaining_accounts: &'a [AccountInfo<'info>],
        step_count: usize,
        subject: Pubkey,
        chain_id: u64,
        verifier_params: InputVerifierParams,
    ) -> Self {
        Self {
            remaining_accounts,
            remaining_accounts_used: vec![false; remaining_accounts.len()],
            produced: Vec::with_capacity(step_count),
            born_public_outputs: Vec::new(),
            subject,
            chain_id,
            verifier_params,
            superseded_in_frame: Vec::new(),
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

    fn finish(self) -> Result<Vec<ProducedPublicOutput>> {
        require!(
            self.remaining_accounts_used.iter().all(|used| *used),
            ZamaHostError::InvalidFheEvalAccount
        );
        Ok(self.born_public_outputs)
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
        encrypted_value_index: u16,
    ) -> Result<ResolvedOperand> {
        let value_info = self.remaining_account(encrypted_value_index)?;
        if self
            .superseded_in_frame
            .iter()
            .any(|(key, superseded)| *key == value_info.key() && *superseded == handle)
        {
            // The frame itself rotated this encrypted value account past `handle`; the operand
            // was authorized by admission against frame-entry state, and
            // supersession never edits membership, so only the current-handle
            // equality is exempted here.
            let value = read_canonical_encrypted_value(value_info)?;
            require!(
                value.has_subject(self.subject),
                ZamaHostError::SubjectNotAllowed
            );
            return Ok(ResolvedOperand::encrypted(handle, false));
        }
        assert_encrypted_value_subject_allowed(value_info, handle, self.chain_id, self.subject)?;
        Ok(ResolvedOperand::encrypted(handle, false))
    }

    #[inline(never)]
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Authoritative in-frame verification: re-run the coprocessor attestation. No account, no
        // PDA — the "allow" exists only for this instruction's execution (the EVM
        // `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is enforced in
        // `resolve_encrypted_operand`; derived outputs are then unconstrained, exactly like EVM.
        // public_decrypt propagates like a public scalar (the app controls decryptability of
        // results via an explicit allow_for_decryption; it is not blocked by the input itself).
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
        Ok(ResolvedOperand::encrypted(attestation.input_handle, true))
    }

    #[inline(never)]
    fn accept_output<'info>(
        &mut self,
        ctx: &Context<'info, FheEval<'info>>,
        op_index: u16,
        result: [u8; 32],
        output: &FheEvalOutput,
        output_public_decrypt_allowed: bool,
    ) -> Result<()> {
        let born_public_output = accept_eval_output(
            ctx,
            &mut self.remaining_accounts_used,
            &mut self.produced,
            result,
            output,
            output_public_decrypt_allowed,
            op_index,
        )?;
        if let Some(record) = born_public_output {
            self.born_public_outputs.push(record);
        }
        if let FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            previous_handle: Some(previous_handle),
            ..
        } = output
        {
            let key = self.remaining_account(*output_encrypted_value_index)?.key();
            self.superseded_in_frame.push((key, *previous_handle));
        }
        Ok(())
    }
}

/// Checks ternary operand metadata against the declared result type.
pub fn assert_ternary_operand_types(
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
    result: [u8; 32],
    output: &FheEvalOutput,
    output_public_decrypt_allowed: bool,
    op_index: u16,
) -> Result<Option<ProducedPublicOutput>> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheEvalDuplicateHandle
    );

    let born_public_output = match output {
        FheEvalOutput::AllowedLocal => None,
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index,
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            output_subjects,
            previous_handle,
            previous_subjects,
            make_public,
        } => {
            let app_account_authority = durable_output_authority(
                ctx,
                remaining_accounts_used,
                *output_app_account_authority_index,
                *output_app_account,
            )?;
            let encrypted_value = bind_eval_output(
                ctx,
                remaining_accounts_used,
                *output_encrypted_value_index,
                result,
                app_account_authority.key(),
                *output_acl_domain_key,
                *output_app_account,
                *output_encrypted_value_label,
                output_subjects,
                previous_handle,
                previous_subjects,
                *make_public,
            )?;
            make_public.then(|| ProducedPublicOutput {
                step_index: op_index,
                encrypted_value,
                output_handle: result,
            })
        }
    };

    produced.push(ProducedValue {
        handle: result,
        public_decrypt_allowed: output_public_decrypt_allowed,
    });
    Ok(born_public_output)
}

fn durable_output_authority<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    remaining_accounts_used: &mut [bool],
    authority_index: Option<u16>,
    output_app_account: Pubkey,
) -> Result<AccountInfo<'info>> {
    let authority = match authority_index {
        Some(index) => {
            let authority =
                remaining_account(ctx.remaining_accounts, remaining_accounts_used, index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheEvalAccount);
            require_keys_eq!(
                authority.key(),
                output_app_account,
                ZamaHostError::AppAccountAuthorityMismatch
            );
            authority.clone()
        }
        None => ctx.accounts.app_account_authority.to_account_info(),
    };
    let deny_record = deny_subject_record_for(
        &ctx.accounts.host_config,
        ctx.remaining_accounts,
        Some(remaining_accounts_used),
        authority.key(),
    )?;
    check_grant_not_denied_info(&ctx.accounts.host_config, authority.key(), deny_record)?;
    Ok(authority)
}

fn deny_subject_record_for<'a, 'info>(
    host_config: &HostConfig,
    remaining_accounts: &'a [AccountInfo<'info>],
    remaining_accounts_used: Option<&mut [bool]>,
    subject: Pubkey,
) -> Result<Option<&'a AccountInfo<'info>>> {
    if !host_config.grant_deny_list_enabled {
        return Ok(None);
    }
    let (expected, _) = deny_subject_address(subject);
    let Some((index, record)) = remaining_accounts
        .iter()
        .enumerate()
        .find(|(_, account)| account.key() == expected)
    else {
        return Err(error!(ZamaHostError::AclDenyRecordMissing));
    };
    if let Some(used) = remaining_accounts_used {
        used[index] = true;
    }
    Ok(Some(record))
}

#[derive(Clone)]
pub(super) struct ProducedValue {
    handle: [u8; 32],
    public_decrypt_allowed: bool,
}

#[derive(Clone)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
    pub(super) public_decrypt_allowed: bool,
}

impl ResolvedOperand {
    fn encrypted(handle: [u8; 32], public_decrypt_allowed: bool) -> Self {
        Self {
            handle,
            scalar: false,
            public_decrypt_allowed,
        }
    }

    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
            public_decrypt_allowed: true,
        }
    }

    fn from_produced(value: &ProducedValue) -> Self {
        Self {
            handle: value.handle,
            scalar: false,
            public_decrypt_allowed: value.public_decrypt_allowed,
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

#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn bind_eval_output<'info>(
    ctx: &Context<'info, FheEval<'info>>,
    remaining_accounts_used: &mut [bool],
    output_encrypted_value_index: u16,
    result: [u8; 32],
    app_account_authority: Pubkey,
    output_acl_domain_key: Pubkey,
    output_app_account: Pubkey,
    output_encrypted_value_label: [u8; 32],
    output_subjects: &[AclSubjectEntry],
    previous_handle: &Option<[u8; 32]>,
    previous_subjects: &Option<Vec<Pubkey>>,
    make_public: bool,
) -> Result<Pubkey> {
    assert_output_acl_metadata(app_account_authority, output_app_account, output_subjects)?;

    let output_info = remaining_account(
        ctx.remaining_accounts,
        remaining_accounts_used,
        output_encrypted_value_index,
    )?;
    let value_key = zama_solana_acl::derive_value_key(
        output_acl_domain_key.to_bytes(),
        output_app_account.to_bytes(),
        output_encrypted_value_label,
    );
    let (expected, bump) = encrypted_value_address(value_key);
    require_keys_eq!(
        output_info.key(),
        expected,
        ZamaHostError::EncryptedValuePdaMismatch
    );

    if output_info.owner == &crate::ID {
        // Supersede: the plan's previous_* fields must match the stored state
        // exactly, so indexers can reconstruct the appended MMR leaves from
        // instruction data alone. `output_subjects` may rotate the audience.
        let mut value = read_canonical_encrypted_value(output_info)?;
        validate_durable_output_previous_state(&value, previous_handle, previous_subjects)?;
        check_rotation_grants_not_denied(
            &ctx.accounts.host_config,
            ctx.remaining_accounts,
            Some(remaining_accounts_used),
            &value.subjects,
            output_subjects,
        )?;
        supersede_current_handle(output_info, &mut value, result)?;
        // Seal the outgoing audience into historical leaves first (above), then rotate
        // to the new set — every added subject cleared the deny-list check above.
        value.subjects = output_subjects.iter().map(|entry| entry.pubkey).collect();
        // Born-public opt-in: after the outgoing handle's historical leaves, seal a
        // public-decrypt leaf for the NEW current handle (leaf order: historical(old)
        // per subject FIRST, then public(new) LAST). Same commitment as
        // `make_handle_public`; the single realloc below covers the extra peak.
        if make_public {
            append_public_decrypt_leaf(output_info, &mut value, result)?;
        }
        let space = 8 + EncryptedValue::space(value.subjects.len(), value.peaks.len());
        grow_account_if_needed(
            &ctx.accounts.payer.to_account_info(),
            output_info,
            &ctx.accounts.system_program.to_account_info(),
            space,
        )?;
        write_account(output_info, &value)?;
    } else {
        // Create: a fresh encrypted value account has no previous state to reconstruct. It is normally
        // not born public-decryptable; `make_public` is the documented opt-in relaxation
        // (DD-036), sealing a public-decrypt leaf for the new handle at leaf index 0.
        require!(
            previous_handle.is_none() && previous_subjects.is_none(),
            ZamaHostError::PreviousStateMismatch
        );
        let mut value = EncryptedValue {
            acl_domain_key: output_acl_domain_key,
            app_account: output_app_account,
            encrypted_value_label: output_encrypted_value_label,
            current_handle: result,
            subjects: output_subjects.iter().map(|s| s.pubkey).collect(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump,
        };
        if make_public {
            append_public_decrypt_leaf(output_info, &mut value, result)?;
        }
        create_pda_strict(
            &ctx.accounts.payer.to_account_info(),
            output_info,
            &ctx.accounts.system_program.to_account_info(),
            8 + EncryptedValue::space(value.subjects.len(), value.peaks.len()),
            &[
                zama_solana_acl::ENCRYPTED_VALUE_SEED,
                value_key.as_ref(),
                &[bump],
            ],
        )?;
        write_account(output_info, &value)?;
    }
    Ok(output_info.key())
}

/// Supersede plan validation against an existing encrypted value account. The plan's
/// `previous_handle`/`previous_subjects` must equal the stored state exactly, so
/// indexers reconstruct the appended MMR leaves from instruction data alone. The
/// audience (`output_subjects`) is NOT constrained to the stored set: a supersede
/// may explicitly rotate it — the outgoing audience is sealed into historical
/// leaves before the new set replaces it, and every added subject passes the
/// grant deny-list via [`check_rotation_grants_not_denied`].
pub(super) fn validate_durable_output_previous_state(
    value: &EncryptedValue,
    previous_handle: &Option<[u8; 32]>,
    previous_subjects: &Option<Vec<Pubkey>>,
) -> Result<()> {
    require!(
        *previous_handle == Some(value.current_handle),
        ZamaHostError::PreviousStateMismatch
    );
    require!(
        previous_subjects.as_deref() == Some(value.subjects.as_slice()),
        ZamaHostError::PreviousStateMismatch
    );
    Ok(())
}

/// Deny-list gate for a supersede that rotates the audience. Every subject present
/// in `output_subjects` but absent from the stored set is a new grant and must
/// clear the grant deny-list exactly as `allow_subjects` gates added subjects, so
/// rotation cannot bypass the deny list. Respects `grant_deny_list_enabled` and
/// (via the shared helpers) pause-independent deny semantics; the deny record for
/// each added subject is located in `remaining_accounts` by canonical address.
pub(super) fn check_rotation_grants_not_denied<'info>(
    host_config: &HostConfig,
    remaining_accounts: &[AccountInfo<'info>],
    mut remaining_accounts_used: Option<&mut [bool]>,
    stored_subjects: &[Pubkey],
    output_subjects: &[AclSubjectEntry],
) -> Result<()> {
    if !host_config.grant_deny_list_enabled {
        return Ok(());
    }
    for entry in output_subjects {
        if stored_subjects.contains(&entry.pubkey) {
            continue;
        }
        let deny_record = deny_subject_record_for(
            host_config,
            remaining_accounts,
            remaining_accounts_used.as_deref_mut(),
            entry.pubkey,
        )?;
        check_grant_not_denied_info(host_config, entry.pubkey, deny_record)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn encrypted_value_account(handle: [u8; 32], subjects: &[Pubkey]) -> EncryptedValue {
        EncryptedValue {
            acl_domain_key: Pubkey::default(),
            app_account: Pubkey::default(),
            encrypted_value_label: [0; 32],
            current_handle: handle,
            subjects: subjects.to_vec(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump: 0,
        }
    }

    fn deny_enabled_config() -> HostConfig {
        HostConfig {
            admin: Pubkey::default(),
            chain_id: 0,
            gateway_chain_id: 0,
            input_verification_contract: [0; 20],
            coprocessor_signers: [[0; 20]; HostConfig::MAX_COPROCESSOR_SIGNERS],
            coprocessor_signer_count: 0,
            coprocessor_threshold: 0,
            decryption_contract: [0; 20],
            current_kms_context_id: 0,
            paused: false,
            grant_deny_list_enabled: true,
            max_hcu_per_tx: 0,
            max_hcu_depth_per_tx: 0,
            hcu_block_cap_per_app: u64::MAX,
            updated_slot: 0,
            bump: 0,
        }
    }

    fn grants(subjects: &[Pubkey]) -> Vec<AclSubjectEntry> {
        subjects
            .iter()
            .map(|subject| AclSubjectEntry { pubkey: *subject })
            .collect()
    }

    #[test]
    fn durable_output_previous_state_accepts_exact_previous_match() {
        let subjects = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        assert!(
            validate_durable_output_previous_state(&value, &Some([9; 32]), &Some(subjects),)
                .is_ok()
        );
    }

    #[test]
    fn durable_output_previous_state_rejects_previous_mismatch() {
        let subjects = vec![Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        // Wrong previous handle.
        assert!(validate_durable_output_previous_state(
            &value,
            &Some([8; 32]),
            &Some(subjects.clone()),
        )
        .is_err());
        // Wrong previous subjects.
        assert!(validate_durable_output_previous_state(
            &value,
            &Some([9; 32]),
            &Some(vec![Pubkey::new_unique()]),
        )
        .is_err());
        // Missing previous_* on an existing encrypted value account (create shape on supersede).
        assert!(validate_durable_output_previous_state(&value, &None, &None).is_err());
    }

    #[test]
    fn durable_output_previous_state_ignores_output_audience() {
        // Validation pins only the outgoing state (previous_handle/previous_subjects); it no
        // longer constrains the new audience, so a supersede may rotate `output_subjects`.
        let subjects = vec![Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        assert!(
            validate_durable_output_previous_state(&value, &Some([9; 32]), &Some(subjects)).is_ok()
        );
    }

    #[test]
    fn assert_output_acl_metadata_rejects_empty_and_over_cap_rotations() {
        let app_account = Pubkey::new_unique();
        // Empty rotated set is rejected, mirroring remove_subject's last-subject rule.
        assert!(assert_output_acl_metadata(app_account, app_account, &[]).is_err());
        // A rotated set above MAX_ACL_SUBJECTS (8) is rejected.
        let over_cap = grants(
            &(0..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
                .map(|_| Pubkey::new_unique())
                .collect::<Vec<_>>(),
        );
        assert!(assert_output_acl_metadata(app_account, app_account, &over_cap).is_err());
    }

    #[test]
    fn rotation_added_denied_subject_is_rejected() {
        let stored = vec![Pubkey::new_unique()];
        let added = Pubkey::new_unique();
        let rotated = grants(&[stored[0], added]);
        let (record_key, bump) = deny_subject_address(added);

        let mut lamports = 1_000_000u64;
        let mut data = vec![0u8; 8 + crate::state::DenySubjectRecord::SPACE];
        crate::state::DenySubjectRecord {
            subject: added,
            denied: true,
            bump,
        }
        .try_serialize(&mut &mut data[..])
        .unwrap();
        let owner = crate::ID;
        let record = AccountInfo::new(
            &record_key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
        );
        let remaining = [record];

        let config = deny_enabled_config();

        // A stored subject that stays put needs no record; only `added` is checked, and it is denied.
        assert_eq!(
            check_rotation_grants_not_denied(&config, &remaining, None, &stored, &rotated)
                .unwrap_err(),
            error!(ZamaHostError::AclSubjectDenied)
        );
    }
}
