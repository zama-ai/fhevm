//! Evaluates ordered instruction-local FHE batches.
//!
//! Three signers cover three distinct authorities, and they are only sometimes the same key:
//! `payer` funds rent for persistent output ACL records; `compute_subject` is the identity that
//! must be allowed on every persistent encrypted input, and doubles as the HCU metering key;
//! `account_authority` is the app account that authorizes persistent output ACL metadata. A CPI
//! caller typically signs `compute_subject` and `account_authority` with its own PDAs while
//! forwarding a user wallet as `payer`.

use anchor_lang::prelude::*;

use super::common::*;
use super::encrypted_value::{
    append_public_decrypt_leaf, grow_account_if_needed, update_encrypted_value,
};
use super::input_verification::{verify_input_attestation, InputVerifierParams};
use crate::{
    errors::ZamaHostError,
    events::{
        FheExecuteRandomSeed, FheExecuteRandomSeedsEvent, ProducedPublicOutput,
        PublicOutputsProducedEvent,
    },
    state::*,
};

mod account_table;
mod block_cap;
mod event_transport;
mod hcu;
mod preflight;
mod walk;

use account_table::EvalAccountTable;
use event_transport::{emit_eval_random_seeds, emit_public_outputs_produced};
use preflight::preflight_eval_frame;
use walk::{walk_eval_frame, EvalHandleContext};

/// Accounts for a composed instruction-local fhe_execute batch.
///
/// Persistent input and output `EncryptedValue` accounts are supplied in
/// `remaining_accounts` and referenced by index from [`FheExecuteArgs`].
#[derive(Accounts)]
#[event_cpi]
pub struct FheExecute<'info> {
    /// Pays rent for any persistent output ACL records.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Compute subject that must be allowed on persistent encrypted inputs.
    pub compute_subject: Signer<'info>,
    /// App account signer authorizing any persistent output ACL metadata.
    pub account_authority: Signer<'info>,
    /// Singleton config PDA. Read-only: the cap is read from here, but the writable per-slot
    /// counter is the separate `hcu_block_meter`, never this singleton — so the hot path takes no
    /// write lock on the config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for persistent output ACL creation.
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

/// Executes an ordered FHE batch with instruction-local transient outputs.
pub fn fhe_execute<'info>(
    ctx: Context<'info, FheExecute<'info>>,
    args: FheExecuteArgs,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        !args.steps.is_empty() && args.steps.len() <= MAX_FHE_BATCH_OPS,
        ZamaHostError::InvalidFheExecuteOperationCount
    );
    require!(
        usize::from(args.account_count) == ctx.remaining_accounts.len(),
        ZamaHostError::FheExecuteAccountCountMismatch
    );
    // The account table owns every remaining-accounts invariant for the batch:
    // duplicate rejection (at construction), the used-account bitmap (marked in
    // preflight, asserted before execution mutates state), persistent-output
    // claims, and output-PDA derivation.
    let mut account_table = EvalAccountTable::new(ctx.remaining_accounts)?;
    preflight_eval_frame(&mut account_table, &ctx, &args)?;

    // HCU metering: one pure pass over the batch, enforcing the per-batch total + in-batch depth
    // caps against the canonical host_config limits (u64::MAX = unlimited). The same total then feeds the
    // block-cap charge — reused, never independently recomputed — so both caps trip before
    // execution burns CU or creates any ACL record.
    let host_config = &ctx.accounts.host_config;
    let batch = hcu::meter_batch(
        &args.steps,
        host_config.max_hcu_per_tx,
        host_config.max_hcu_depth_per_tx,
    )?;

    let subject = ctx.accounts.compute_subject.key();
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    let persistent_anchor_bytes = collect_persistent_anchor_bytes(&account_table, &args)?;
    let handle_context = EvalHandleContext {
        derivation: HandleDerivationContext {
            chain_id: ctx.accounts.host_config.chain_id,
            previous_bank_hash,
            unix_timestamp: clock.unix_timestamp,
        },
        compute_subject: subject,
        persistent_anchor_bytes: &persistent_anchor_bytes,
    };
    let random_seeds = collect_eval_random_seeds(&args, &handle_context);
    block_cap::charge(&ctx, batch.total, clock.slot)?;
    // Execution is the single walk: it validates each step as it mutates. A failure mid-batch
    // leaves partial writes behind only until the runtime reverts the transaction, which discards
    // every account write — so no validate-only pre-pass is needed for atomicity. The event CPI
    // stays last so no event describes state that did not commit.
    let created_public_outputs =
        execute_eval_frame(&mut account_table, &ctx, &args, subject, &handle_context)?;
    emit_eval_random_seeds(&ctx, random_seeds)?;
    emit_public_outputs_produced(&ctx, created_public_outputs)?;
    Ok(())
}

/// The step's declared output, shared by preflight rules and anchor collection.
pub(in crate::instructions) fn step_output(step: &FheExecuteStep) -> &FheExecuteOutput {
    match step {
        FheExecuteStep::Binary { output, .. }
        | FheExecuteStep::Ternary { output, .. }
        | FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::Unary { output, .. }
        | FheExecuteStep::RandBounded { output, .. }
        | FheExecuteStep::Sum { output, .. }
        | FheExecuteStep::IsIn { output, .. }
        | FheExecuteStep::MulDiv { output, .. } => output,
    }
}

/// Flattens the batch's persistent-write anchor from live account state. Each entry is
/// `(account key, create/update tag, current handle, leaf count)` in wire order.
/// `leaf_count` advances whenever an outgoing handle is sealed, so returning to an
/// earlier content-addressed handle cannot replay a previous random seed.
fn collect_persistent_anchor_bytes(
    table: &EvalAccountTable<'_, '_>,
    args: &FheExecuteArgs,
) -> Result<Vec<u8>> {
    let mut anchor_bytes = Vec::with_capacity(args.steps.len() * 73);
    for step in &args.steps {
        if let FheExecuteOutput::AllowedPersistent {
            output_encrypted_value_index,
            ..
        } = step_output(step)
        {
            let account = table.account(u16::from(*output_encrypted_value_index))?;
            anchor_bytes.extend_from_slice(account.key().as_ref());
            if account.owner == &crate::ID {
                let value = read_canonical_encrypted_value(account)?;
                anchor_bytes.push(1);
                anchor_bytes.extend_from_slice(&value.current_handle);
                anchor_bytes.extend_from_slice(&value.leaf_count.to_le_bytes());
            } else {
                anchor_bytes.push(0);
                anchor_bytes.extend_from_slice(&[0; 32]);
                anchor_bytes.extend_from_slice(&0u64.to_le_bytes());
            }
        }
    }
    Ok(anchor_bytes)
}

fn collect_eval_random_seeds(
    args: &FheExecuteArgs,
    handle_context: &EvalHandleContext<'_>,
) -> Vec<FheExecuteRandomSeed> {
    args.steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            matches!(
                step,
                FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
            )
        })
        .map(|(index, _)| FheExecuteRandomSeed {
            step_index: index as u16,
            seed: handle_context.rand_seed(index as u16),
        })
        .collect()
}

#[inline(never)]
fn execute_eval_frame<'a, 'info>(
    table: &mut EvalAccountTable<'a, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    args: &FheExecuteArgs,
    subject: Pubkey,
    handle_context: &EvalHandleContext<'_>,
) -> Result<Vec<ProducedPublicOutput>> {
    let mut execution = EvalExecutionState {
        table,
        dictionary: &args.dictionary,
        produced: Vec::with_capacity(args.steps.len()),
        created_public_outputs: Vec::new(),
        subject,
        chain_id: handle_context.derivation.chain_id,
        verifier_params: InputVerifierParams::from_config(&ctx.accounts.host_config),
    };
    walk_eval_frame(&mut execution, ctx, args, handle_context)?;
    Ok(execution.created_public_outputs)
}

/// The single walk's state: resolves operands through the shared account table
/// (which preflight already validated for coverage), validates and creates or
/// updates persistent outputs, and buffers produced-public lifecycle records.
/// The operand resolvers driving these methods live with the step match in
/// [`walk`].
struct EvalExecutionState<'t, 'a, 'info> {
    table: &'t mut EvalAccountTable<'a, 'info>,
    /// The batch's interned constant dictionary ([`FheExecuteArgs::dictionary`]).
    dictionary: &'t [[u8; 32]],
    produced: Vec<ProducedValue>,
    created_public_outputs: Vec<ProducedPublicOutput>,
    subject: Pubkey,
    chain_id: u64,
    verifier_params: InputVerifierParams,
}

impl<'info> EvalExecutionState<'_, '_, 'info> {
    fn dictionary_bytes(&self, index: u8) -> Result<[u8; 32]> {
        self.dictionary
            .get(index as usize)
            .copied()
            .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))
    }

    #[inline(never)]
    fn resolve_persistent_operand(
        &mut self,
        handle: [u8; 32],
        encrypted_value_index: u16,
    ) -> Result<ResolvedOperand> {
        let value_info = self.table.account(encrypted_value_index)?;
        assert_encrypted_value_subject_allowed(value_info, handle, self.chain_id, self.subject)?;
        Ok(ResolvedOperand::encrypted(handle, false))
    }

    #[inline(never)]
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Authoritative in-batch verification of the coprocessor attestation. No account, no
        // PDA — the "allow" exists only for this instruction's execution (the EVM
        // `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is enforced in
        // `resolve_encrypted_operand`; derived outputs are then unconstrained, exactly like EVM.
        // public_decrypt propagates like a public scalar (the app controls decryptability of
        // results via an explicit allow_for_decryption; it is not blocked by the input itself).
        verify_input_attestation(&self.verifier_params, attestation)?;
        Ok(ResolvedOperand::encrypted(attestation.input_handle, true))
    }

    #[inline(never)]
    fn accept_output(
        &mut self,
        ctx: &Context<'info, FheExecute<'info>>,
        op_index: u16,
        result: [u8; 32],
        output: &FheExecuteOutput,
        output_public_decrypt_allowed: bool,
    ) -> Result<()> {
        let created_public_output = accept_eval_output(
            ctx,
            self.table,
            self.dictionary,
            &mut self.produced,
            result,
            output,
            output_public_decrypt_allowed,
            op_index,
        )?;
        if let Some(record) = created_public_output {
            self.created_public_outputs.push(record);
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
    ctx: &Context<'info, FheExecute<'info>>,
    table: &mut EvalAccountTable<'_, 'info>,
    dictionary: &[[u8; 32]],
    produced: &mut Vec<ProducedValue>,
    result: [u8; 32],
    output: &FheExecuteOutput,
    output_public_decrypt_allowed: bool,
    op_index: u16,
) -> Result<Option<ProducedPublicOutput>> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheExecuteDuplicateHandle
    );

    let created_public_output = match output {
        FheExecuteOutput::AllowedLocal => None,
        FheExecuteOutput::AllowedPersistent {
            output_encrypted_value_index,
            output_account_authority_index,
            output_domain_index,
            output_account_index,
            output_label_index,
            output_subject_indexes,
            previous_state,
            make_public,
        } => {
            let output_domain = dictionary_key(dictionary, *output_domain_index)?;
            let output_account = dictionary_key(dictionary, *output_account_index)?;
            let output_label = dictionary_bytes(dictionary, *output_label_index)?;
            let output_subjects = resolve_dictionary_subjects(dictionary, output_subject_indexes)?;
            let account_authority = persistent_output_authority(
                table,
                ctx,
                output_account_authority_index.map(u16::from),
                output_account,
            )?;
            let encrypted_value = bind_eval_output(
                ctx,
                table,
                u16::from(*output_encrypted_value_index),
                result,
                account_authority.key(),
                output_domain,
                output_account,
                output_label,
                &output_subjects,
                previous_state,
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
    Ok(created_public_output)
}

fn dictionary_bytes(dictionary: &[[u8; 32]], index: u8) -> Result<[u8; 32]> {
    dictionary
        .get(index as usize)
        .copied()
        .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))
}

fn dictionary_key(dictionary: &[[u8; 32]], index: u8) -> Result<Pubkey> {
    Ok(Pubkey::new_from_array(dictionary_bytes(dictionary, index)?))
}

fn resolve_dictionary_subjects(dictionary: &[[u8; 32]], indexes: &[u8]) -> Result<Vec<Pubkey>> {
    indexes
        .iter()
        .map(|index| dictionary_key(dictionary, *index))
        .collect()
}

fn persistent_output_authority<'info>(
    table: &EvalAccountTable<'_, 'info>,
    ctx: &Context<'info, FheExecute<'info>>,
    authority_index: Option<u16>,
    output_account: Pubkey,
) -> Result<AccountInfo<'info>> {
    let authority = match authority_index {
        Some(index) => {
            let authority = table.account(index)?;
            require!(authority.is_signer, ZamaHostError::InvalidFheExecuteAccount);
            require_keys_eq!(
                authority.key(),
                output_account,
                ZamaHostError::AppAccountAuthorityMismatch
            );
            authority.clone()
        }
        None => ctx.accounts.account_authority.to_account_info(),
    };
    let deny_record = table.deny_record(
        ctx.accounts.host_config.grant_deny_list_enabled,
        authority.key(),
    )?;
    check_grant_not_denied_info(&ctx.accounts.host_config, authority.key(), deny_record)?;
    Ok(authority)
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
    ctx: &Context<'info, FheExecute<'info>>,
    table: &mut EvalAccountTable<'_, 'info>,
    output_encrypted_value_index: u16,
    result: [u8; 32],
    account_authority: Pubkey,
    output_domain: Pubkey,
    output_account: Pubkey,
    output_label: [u8; 32],
    output_subjects: &[Pubkey],
    previous_state: &Option<PreviousState>,
    make_public: bool,
) -> Result<Pubkey> {
    assert_output_acl_metadata(account_authority, output_account, output_subjects)?;

    let output_info = table.account(output_encrypted_value_index)?;
    let output_pda = table.expected_output_pda(output_domain, output_account, output_label);
    require_keys_eq!(
        output_info.key(),
        output_pda.key,
        ZamaHostError::EncryptedValuePdaMismatch
    );
    // One write per account per batch — load-bearing for the rand seed anchor (#1853 W4).
    table.claim_persistent_output(output_info.key())?;
    // Explicit on the update path; `create_pda_strict` enforces it on create.
    require!(
        output_info.is_writable,
        ZamaHostError::InvalidFheExecuteAccount
    );

    if output_info.owner == &crate::ID {
        // Update: the batch's declared previous state must match the stored
        // state exactly, so indexers can reconstruct the appended MMR leaves
        // from instruction data alone. `output_subjects` may replace the audience.
        let mut value = read_canonical_encrypted_value(output_info)?;
        validate_persistent_output_previous_state(&value, previous_state)?;
        check_new_grants_not_denied(
            &ctx.accounts.host_config,
            table,
            &value.subjects,
            output_subjects,
        )?;
        update_encrypted_value(output_info, &mut value, result)?;
        // Seal the outgoing audience into historical leaves first (above), then replace
        // to the new set — every added subject cleared the deny-list check above.
        value.subjects = output_subjects.to_vec();
        // Created-public opt-in: after the outgoing handle's historical leaves, seal a
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
        // not created public-decryptable; `make_public` is the documented opt-in relaxation
        // (DD-036), sealing a public-decrypt leaf for the new handle at leaf index 0.
        require!(
            previous_state.is_none(),
            ZamaHostError::PreviousStateMismatch
        );
        check_new_grants_not_denied(&ctx.accounts.host_config, table, &[], output_subjects)?;
        let mut value = EncryptedValue {
            domain: output_domain,
            account: output_account,
            label: output_label,
            current_handle: result,
            subjects: output_subjects.to_vec(),
            leaf_count: 0,
            peaks: Vec::new(),
            bump: output_pda.bump,
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
                output_pda.encrypted_value_id.as_ref(),
                &[output_pda.bump],
            ],
        )?;
        write_account(output_info, &value)?;
    }
    Ok(output_info.key())
}

/// Update batch validation against an existing encrypted value account. The batch's
/// declared `previous_state` must equal the stored state exactly, so indexers
/// reconstruct the appended MMR leaves from instruction data alone. The
/// audience (`output_subjects`) is NOT constrained to the stored set: an update
/// may explicitly replace it — the outgoing audience is sealed into historical
/// leaves before the new set replaces it, and every added subject passes the
/// grant deny-list via [`check_new_grants_not_denied`].
pub(super) fn validate_persistent_output_previous_state(
    value: &EncryptedValue,
    previous_state: &Option<PreviousState>,
) -> Result<()> {
    let Some(previous) = previous_state else {
        return Err(error!(ZamaHostError::PreviousStateMismatch));
    };
    require!(
        previous.handle == value.current_handle,
        ZamaHostError::PreviousStateMismatch
    );
    require!(
        previous.subjects.as_slice() == value.subjects.as_slice(),
        ZamaHostError::PreviousStateMismatch
    );
    Ok(())
}

/// Deny-list gate for persistent-output subject grants: every subject present in
/// `output_subjects` but absent from `stored_subjects` is a new grant and must
/// clear the grant deny-list (pass `&[]` on the create path, where every output
/// subject is new). Respects `grant_deny_list_enabled`; the deny record for each
/// added subject is located by canonical derived address through the table.
fn check_new_grants_not_denied(
    host_config: &HostConfig,
    table: &EvalAccountTable<'_, '_>,
    stored_subjects: &[Pubkey],
    output_subjects: &[Pubkey],
) -> Result<()> {
    if !host_config.grant_deny_list_enabled {
        return Ok(());
    }
    for subject in output_subjects {
        if stored_subjects.contains(subject) {
            continue;
        }
        let deny_record = table.deny_record(host_config.grant_deny_list_enabled, *subject)?;
        check_grant_not_denied_info(host_config, *subject, deny_record)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::AccountSerialize;

    fn encrypted_value_account(handle: [u8; 32], subjects: &[Pubkey]) -> EncryptedValue {
        EncryptedValue {
            domain: Pubkey::default(),
            account: Pubkey::default(),
            label: [0; 32],
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
            max_hcu_per_tx: u64::MAX,
            max_hcu_depth_per_tx: u64::MAX,
            hcu_block_cap_per_app: u64::MAX,
            updated_slot: 0,
            bump: 0,
        }
    }

    fn grants(subjects: &[Pubkey]) -> Vec<Pubkey> {
        subjects.to_vec()
    }

    fn persistent_anchor_at_leaf_count(leaf_count: u64) -> Vec<u8> {
        let mut value = encrypted_value_account([9; 32], &[Pubkey::new_from_array([1; 32])]);
        value.domain = Pubkey::new_from_array([2; 32]);
        value.account = Pubkey::new_from_array([3; 32]);
        value.label = [7; 32];
        value.leaf_count = leaf_count;
        let (key, bump) = encrypted_value_address(value.encrypted_value_id());
        value.bump = bump;

        let mut lamports = 1_000_000;
        let mut data = vec![0; 8 + EncryptedValue::space(value.subjects.len(), 0)];
        value.try_serialize(&mut &mut data[..]).unwrap();
        let owner = crate::ID;
        let account = AccountInfo::new(&key, false, true, &mut lamports, &mut data, &owner, false);
        let accounts = [account];
        let table = EvalAccountTable::new(&accounts).unwrap();
        let args = FheExecuteArgs {
            account_count: 1,
            dictionary: Vec::new(),
            steps: vec![FheExecuteStep::Rand {
                fhe_type: 5,
                output: FheExecuteOutput::AllowedPersistent {
                    output_encrypted_value_index: 0,
                    output_account_authority_index: None,
                    output_domain_index: 0,
                    output_account_index: 0,
                    output_label_index: 0,
                    output_subject_indexes: Vec::new(),
                    previous_state: Some(PreviousState {
                        handle: value.current_handle,
                        subjects: value.subjects,
                    }),
                    make_public: false,
                },
            }],
        };
        collect_persistent_anchor_bytes(&table, &args).unwrap()
    }

    #[test]
    fn persistent_anchor_changes_when_handle_cycles_to_a_later_leaf_count() {
        assert_ne!(
            persistent_anchor_at_leaf_count(1),
            persistent_anchor_at_leaf_count(3)
        );
    }

    #[test]
    fn persistent_output_previous_state_accepts_exact_previous_match() {
        let subjects = vec![Pubkey::new_unique(), Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        assert!(validate_persistent_output_previous_state(
            &value,
            &Some(PreviousState {
                handle: [9; 32],
                subjects,
            }),
        )
        .is_ok());
    }

    #[test]
    fn persistent_output_previous_state_rejects_previous_mismatch() {
        let subjects = vec![Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        // Wrong previous handle.
        assert!(validate_persistent_output_previous_state(
            &value,
            &Some(PreviousState {
                handle: [8; 32],
                subjects: subjects.clone(),
            }),
        )
        .is_err());
        // Wrong previous subjects.
        assert!(validate_persistent_output_previous_state(
            &value,
            &Some(PreviousState {
                handle: [9; 32],
                subjects: vec![Pubkey::new_unique()],
            }),
        )
        .is_err());
        // Missing previous state on an existing encrypted value account (create shape on update).
        assert!(validate_persistent_output_previous_state(&value, &None).is_err());
    }

    #[test]
    fn persistent_output_previous_state_ignores_output_audience() {
        // Validation pins only the outgoing state (`previous_state`); it no
        // longer constrains the new audience, so an update may replace `output_subjects`.
        let subjects = vec![Pubkey::new_unique()];
        let value = encrypted_value_account([9; 32], &subjects);
        assert!(validate_persistent_output_previous_state(
            &value,
            &Some(PreviousState {
                handle: [9; 32],
                subjects,
            }),
        )
        .is_ok());
    }

    #[test]
    fn assert_output_acl_metadata_rejects_empty_and_over_cap_replacements() {
        let account = Pubkey::new_unique();
        // An empty replacement set is rejected, mirroring remove_subject's last-subject rule.
        assert!(assert_output_acl_metadata(account, account, &[]).is_err());
        // A replacement set above MAX_ENCRYPTED_VALUE_SUBJECTS (8) is rejected.
        let over_cap = grants(
            &(0..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
                .map(|_| Pubkey::new_unique())
                .collect::<Vec<_>>(),
        );
        assert!(assert_output_acl_metadata(account, account, &over_cap).is_err());
    }

    #[test]
    fn update_added_denied_subject_is_rejected() {
        let stored = vec![Pubkey::new_unique()];
        let added = Pubkey::new_unique();
        let replacement = grants(&[stored[0], added]);
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
        let table = EvalAccountTable::new(&remaining).unwrap();

        let config = deny_enabled_config();

        // A stored subject that stays put needs no record; only `added` is checked, and it is denied.
        assert_eq!(
            check_new_grants_not_denied(&config, &table, &stored, &replacement).unwrap_err(),
            error!(ZamaHostError::SubjectDenied)
        );
    }
}
