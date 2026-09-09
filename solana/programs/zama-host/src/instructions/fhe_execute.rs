//! Evaluates ordered instruction-local FHE executions.
//!
//! Two signers cover two authorities, and they are only sometimes the same key: `payer` funds rent
//! for persistent output accounts; `authority` is the default authority
//! that signs for persistent values read and written. Every persistent value an execution touches
//! is admitted by its own authority's signature — found among the default signer and the signing
//! remaining accounts — and nothing else: an application program signs for its PDAs by CPI and
//! forwards a user wallet as `payer`.

use anchor_lang::prelude::*;

use super::common::*;
use super::input_verification::verify_input_attestation;
use super::state_history::grow_account_if_needed;
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
mod state_output;
mod walk;

use account_table::ExecutionAccountTable;
use event_transport::{emit_execution_random_seeds, emit_public_outputs_produced};
use preflight::preflight_execution;
use walk::{walk_steps, ExecutionHandleContext, RandContext};

/// Accounts for one composed, instruction-local fhe_execute.
///
/// Persistent input and output `EncryptedState` accounts are supplied in
/// `remaining_accounts` and referenced by index from [`FheExecuteArgs`].
#[derive(Accounts)]
#[event_cpi]
pub struct FheExecute<'info> {
    /// Pays rent for any persistent output accounts.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Default authority signer: admits every persistent value read or written that does not name
    /// an authority of its own. An output that sets `authority_index` points at a remaining account
    /// instead, and that account must sign and must equal the authority the output declares; an
    /// operand's authority may likewise be any signing remaining account.
    pub authority: Signer<'info>,
    /// Singleton config PDA. Read-only: the cap is read from here, but the writable per-slot
    /// counter is the separate `hcu_block_meter`, never this singleton — so the hot path takes no
    /// write lock on the config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for persistent output creation.
    pub system_program: Program<'info, System>,
    /// Per-application HCU block meter (written once in the execution `charge`). The HCU PDAs
    /// (`hcu_block_meter`, `hcu_trusted_app_record`) key on the `(program, scope)` of the
    /// persistent values the execution touches. Only the `program` half is proved (from the
    /// output authority, on create), so no caller can rotate a fresh *signer* to reach another
    /// program's meter — but a program declares its own `scope` freely, and a fresh scope is a
    /// fresh meter (INVARIANTS #41). Untrusted applications in the metering band MUST supply
    /// this meter; trusted applications and the unrestricted default omit it. An
    /// `UncheckedAccount` because it may be uninitialized (lazy-created) and is validated
    /// manually.
    #[account(mut)]
    pub hcu_block_meter: Option<UncheckedAccount<'info>>,
    /// Trust witness (read-only), keyed on `(program, scope)`. Present + program-owned +
    /// `trusted == true` ⇒ bypass the cap; absent (`None`) ⇒ untrusted, fall through to the meter;
    /// present-but-malformed ⇒ reject.
    pub hcu_trusted_app_record: Option<UncheckedAccount<'info>>,
    /// The host's rand nonce, consumed and incremented by an execution that contains a rand step.
    /// Required exactly then, and refused otherwise so no execution write-locks it for nothing.
    #[account(mut, seeds = [RAND_NONCE_SEED], bump = rand_nonce.bump)]
    pub rand_nonce: Option<Account<'info, RandNonce>>,
}

/// Runs one ordered FHE execution, with instruction-local transient outputs.
pub fn fhe_execute<'info>(
    mut ctx: Context<'info, FheExecute<'info>>,
    args: FheExecuteArgs,
) -> Result<()> {
    assert_not_paused(&ctx.accounts.host_config)?;
    require!(
        !args.steps.is_empty() && args.steps.len() <= MAX_FHE_EXECUTION_STEPS,
        ZamaHostError::InvalidFheExecuteOperationCount
    );
    require!(
        usize::from(args.account_count) == ctx.remaining_accounts.len(),
        ZamaHostError::FheExecuteAccountCountMismatch
    );
    validate_returned_results(&args)?;
    let rand_nonce = consume_rand_nonce(&mut ctx, &args)?;
    // The account table owns every remaining-accounts invariant for the execution:
    // duplicate rejection (at construction), the used-account bitmap (marked in
    // preflight, asserted before execution mutates state), persistent-output
    // claims, and output-PDA derivation.
    let mut account_table = ExecutionAccountTable::new(ctx.remaining_accounts)?;
    // Preflight also settles the execution's application identity: the one `(program, scope)`
    // every persistent value the default authority controls belongs to. Metering and rand seeds
    // key on it; the deny list gates every application the execution touches.
    let preflight = preflight_execution(&mut account_table, &ctx, &args)?;
    let app = preflight.app;
    let host_config = &ctx.accounts.host_config;
    for touched in preflight.touched_apps {
        let deny_record =
            account_table.deny_record(host_config.grant_deny_list_enabled, touched)?;
        check_scope_not_denied_info(host_config, touched, deny_record)?;
    }

    // HCU metering: one pure pass over the execution, enforcing the per-execution total + in-execution depth
    // caps against the canonical host_config limits (u64::MAX = unlimited). The same total then feeds the
    // block-cap charge — reused, never independently recomputed — so both caps trip before
    // execution burns CU or creates any ACL record.
    let execution = hcu::meter_execution(
        &args.steps,
        &args.dictionary,
        host_config.max_hcu_per_tx,
        host_config.max_hcu_depth_per_tx,
    )?;

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    let handle_context = ExecutionHandleContext {
        derivation: HandleDerivationContext {
            chain_id: ctx.accounts.host_config.chain_id,
            previous_bank_hash,
            unix_timestamp: clock.unix_timestamp,
        },
        rand: rand_nonce.map(|nonce| RandContext {
            nonce,
            app: app.unwrap_or_default(),
        }),
    };
    let random_seeds = collect_execution_random_seeds(&args, &handle_context)?;
    block_cap::charge(&ctx, app, execution.total, clock.slot)?;
    // Execution is the single walk: it validates each step as it mutates. A failure mid-execution
    // leaves partial writes behind only until the runtime reverts the transaction, which discards
    // every account write — so no validate-only pre-pass is needed for atomicity. The event CPI
    // stays last so no event describes state that did not commit.
    let (created_public_outputs, produced) = execute_steps(
        &mut account_table,
        &args,
        app,
        &handle_context,
        &ctx.accounts.host_config,
    )?;
    account_table.flush_states(
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;
    emit_execution_random_seeds(&ctx, random_seeds)?;
    emit_public_outputs_produced(&ctx, created_public_outputs)?;
    return_execution_handles(&produced, &args.returned_results);
    Ok(())
}

fn validate_returned_results(args: &FheExecuteArgs) -> Result<()> {
    require!(
        args.returned_results.len() <= crate::MAX_RETURNED_HANDLES,
        ZamaHostError::InvalidReturnSelection
    );
    for result in &args.returned_results {
        require!(
            usize::from(result.step_index) < args.steps.len() && result.output_index == 0,
            ZamaHostError::InvalidReturnSelection
        );
    }
    Ok(())
}

#[inline(never)]
fn return_execution_handles(produced: &[ProducedValue], selected: &[crate::ExecutionResultRef]) {
    let mut bytes = [0u8; crate::MAX_RETURNED_HANDLES * 32];
    for (chunk, result) in bytes.chunks_exact_mut(32).zip(selected) {
        chunk.copy_from_slice(&produced[usize::from(result.step_index)].handle);
    }
    // Set even an empty result after event CPIs, so their return data cannot leak to callers.
    anchor_lang::solana_program::program::set_return_data(&bytes[..selected.len() * 32]);
}

/// Takes the host's rand nonce for this execution and advances it, when the execution has a rand
/// step. The account is required exactly then: a rand execution without it cannot derive a fresh
/// seed, and a non-rand execution that passes it would serialize on it for nothing.
fn consume_rand_nonce(
    ctx: &mut Context<'_, FheExecute<'_>>,
    args: &FheExecuteArgs,
) -> Result<Option<u64>> {
    let has_rand = args.steps.iter().any(|step| {
        matches!(
            step,
            FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
        )
    });
    match (ctx.accounts.rand_nonce.as_mut(), has_rand) {
        (Some(rand_nonce), true) => {
            let nonce = rand_nonce.nonce;
            rand_nonce.nonce = nonce
                .checked_add(1)
                .ok_or(ZamaHostError::InvalidFheExecuteAccount)?;
            Ok(Some(nonce))
        }
        (None, true) => Err(error!(ZamaHostError::FheExecuteRandNonceMissing)),
        (Some(_), false) => Err(error!(ZamaHostError::InvalidFheExecuteAccount)),
        (None, false) => Ok(None),
    }
}

fn collect_execution_random_seeds(
    args: &FheExecuteArgs,
    handle_context: &ExecutionHandleContext,
) -> Result<Vec<FheExecuteRandomSeed>> {
    args.steps
        .iter()
        .enumerate()
        .filter(|(_, step)| {
            matches!(
                step,
                FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
            )
        })
        .map(|(index, _)| {
            Ok(FheExecuteRandomSeed {
                step_index: index as u16,
                seed: handle_context.rand_seed(index as u16)?,
            })
        })
        .collect()
}

#[inline(never)]
fn execute_steps<'a, 'info>(
    table: &mut ExecutionAccountTable<'a, 'info>,
    args: &FheExecuteArgs,
    app: Option<AppScope>,
    handle_context: &ExecutionHandleContext,
    host_config: &HostConfig,
) -> Result<(Vec<ProducedPublicOutput>, Vec<ProducedValue>)> {
    let mut execution = ExecutionState {
        table,
        dictionary: &args.dictionary,
        produced: Vec::with_capacity(args.steps.len()),
        created_public_outputs: Vec::new(),
        app,
        chain_id: handle_context.derivation.chain_id,
        host_config,
    };
    walk_steps(&mut execution, args, handle_context)?;
    Ok((execution.created_public_outputs, execution.produced))
}

/// The single walk's state: resolves operands through the shared account table
/// (which preflight already validated for coverage and authority signatures),
/// validates and creates or updates persistent outputs, and buffers
/// produced-public lifecycle records. The operand resolvers driving these
/// methods live with the step match in [`walk`].
struct ExecutionState<'t, 'a, 'info> {
    table: &'t mut ExecutionAccountTable<'a, 'info>,
    /// The execution's interned constant dictionary ([`FheExecuteArgs::dictionary`]).
    dictionary: &'t [[u8; 32]],
    produced: Vec<ProducedValue>,
    created_public_outputs: Vec<ProducedPublicOutput>,
    /// The application every persistent value of the execution belongs to; `None` when the
    /// execution touches none.
    app: Option<AppScope>,
    chain_id: u64,
    host_config: &'t HostConfig,
}

impl<'info> ExecutionState<'_, '_, 'info> {
    fn dictionary_bytes(&self, index: u8) -> Result<[u8; 32]> {
        crate::state::dictionary_bytes(self.dictionary, index)
    }

    #[inline(never)]
    fn resolve_verified_input_operand(
        &mut self,
        attestation: &CoprocessorInputAttestation,
    ) -> Result<ResolvedOperand> {
        // Authoritative in-execution verification of the coprocessor attestation. No account, no
        // PDA — the "allow" exists only for this instruction's execution (the EVM
        // `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is enforced in
        // `resolve_encrypted_operand`; derived outputs are then unconstrained, exactly like EVM.
        verify_input_attestation(self.host_config, attestation)?;
        Ok(ResolvedOperand::encrypted(attestation.input_handle))
    }

    #[inline(never)]
    fn accept_output(
        &mut self,
        op_index: u16,
        result: [u8; 32],
        output: &FheExecuteOutput,
    ) -> Result<()> {
        let created_public_output = accept_execution_output(
            self.table,
            self.dictionary,
            &mut self.produced,
            result,
            output,
            op_index,
        )?;
        if let Some(record) = created_public_output {
            self.created_public_outputs.push(record);
        }
        Ok(())
    }
}

#[inline(never)]
fn accept_execution_output<'info>(
    table: &mut ExecutionAccountTable<'_, 'info>,
    dictionary: &[[u8; 32]],
    produced: &mut Vec<ProducedValue>,
    result: [u8; 32],
    output: &FheExecuteOutput,
    op_index: u16,
) -> Result<Option<ProducedPublicOutput>> {
    require!(
        !produced.iter().any(|value| value.handle == result),
        ZamaHostError::FheExecuteDuplicateHandle
    );

    let created_public_output = match output {
        FheExecuteOutput::State {
            state_index,
            previous_leaf_count,
            slot,
            allow_indexes,
            make_public,
            grants,
        } => {
            let state = state_output::accept_state_output(
                table,
                dictionary,
                *state_index,
                *previous_leaf_count,
                slot,
                allow_indexes,
                *make_public,
                grants,
                result,
            )?;
            make_public.then_some(ProducedPublicOutput {
                step_index: op_index,
                encrypted_state: state,
                output_handle: result,
            })
        }
        FheExecuteOutput::Transient => None,
    };

    produced.push(ProducedValue { handle: result });
    Ok(created_public_output)
}

fn resolve_dictionary_keys(dictionary: &[[u8; 32]], indexes: &[u8]) -> Result<Vec<Pubkey>> {
    indexes
        .iter()
        .map(|index| dictionary_key(dictionary, *index))
        .collect()
}

#[derive(Clone)]
pub(super) struct ProducedValue {
    handle: [u8; 32],
}

#[derive(Clone)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
}

impl ResolvedOperand {
    fn encrypted(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: false,
        }
    }

    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
        }
    }

    fn from_produced(value: &ProducedValue) -> Self {
        Self {
            handle: value.handle,
            scalar: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doc-sync guard (the `resource_bounds_match_liveness_doc` pattern): EVM_PARITY.md's
    /// FHEVMExecutor row quotes `MAX_FHE_EXECUTION_STEPS=32`; a change here must update that row in
    /// the same PR.
    #[test]
    fn batch_ops_bound_matches_evm_parity_doc() {
        assert_eq!(
            MAX_FHE_EXECUTION_STEPS, 32,
            "EVM_PARITY.md FHEVMExecutor row"
        );
    }
}
