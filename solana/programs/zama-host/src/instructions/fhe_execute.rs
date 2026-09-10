//! Evaluates ordered instruction-local FHE executions.
//!
//! Two signers cover two roles, and they are only sometimes the same key: `payer` funds rent for
//! Store growth and lazy meter creation; `authority` is the default signer for the Stores the
//! execution reads and writes. Every Store an execution touches is admitted by its own authority's
//! signature, found among the default signer and the signing remaining accounts, and by nothing
//! else. An application program signs for its PDAs by CPI and forwards a user wallet as `payer`.

use anchor_lang::prelude::*;

use super::common::*;
use super::input_verification::verify_input_attestation;
use super::store_history::grow_account_if_needed;
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
mod store_output;
mod walk;

use account_table::ExecutionAccountTable;
use event_transport::{emit_execution_random_seeds, emit_public_outputs_produced};
use preflight::preflight_execution;
use walk::{walk_steps, ExecutionHandleContext, RandContext};

/// Accounts for one composed, instruction-local fhe_execute.
///
/// Persistent input and output `EncryptedStore` accounts are supplied in
/// `remaining_accounts` and referenced by index from [`FheExecuteArgs`].
#[derive(Accounts)]
#[event_cpi]
pub struct FheExecute<'info> {
    /// Pays rent for Store growth and lazy meter creation.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Default authority signer. Each Store read or written requires its stored authority to
    /// sign, either here or among the remaining accounts.
    pub authority: Signer<'info>,
    /// Singleton config PDA. Read-only: the cap is read from here, but the writable per-slot
    /// counter is the separate `hcu_block_meter`, never this singleton — so the hot path takes no
    /// write lock on the config.
    #[account(seeds = [HOST_CONFIG_SEED], bump = host_config.bump)]
    pub host_config: Account<'info, HostConfig>,
    /// System program used for rent top-ups and lazy meter creation.
    pub system_program: Program<'info, System>,
    /// Per-application HCU block meter (written once in the execution `charge`). The HCU PDAs
    /// (`hcu_block_meter`, `hcu_trusted_app_record`) key on the `(program, scope)` of the
    /// Stores controlled by the default authority. Store creation proves the authority is a PDA
    /// of `program`, so no caller can rotate a fresh *signer* to reach another
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
    /// Shared by every execution until the transaction's final CloseTransientStore.
    #[account(mut)]
    pub transient_store: AccountLoader<'info, TransientStore>,
    /// CHECK: authentic runtime transaction instructions, used to require final closure.
    #[account(address = solana_instructions_sysvar::ID)]
    pub instructions: UncheckedAccount<'info>,
}

/// Runs one ordered FHE execution, recording results in transaction transient store.
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
    validate_result_refs(&args)?;
    require!(
        args.effects.len() <= MAX_FHE_EXECUTION_EFFECTS,
        ZamaHostError::InvalidFheExecuteOperationCount
    );
    let rand_nonce = consume_rand_nonce(&mut ctx, &args)?;
    require!(
        ctx.accounts.transient_store.to_account_info().data_len() == TransientStore::SPACE,
        ZamaHostError::TransientAccountInvalid
    );
    let mut transient_store = ctx.accounts.transient_store.load_mut()?;
    transient_store.validate(ctx.accounts.transient_store.key())?;
    super::transient::assert_final_close(
        ctx.accounts.transient_store.key(),
        transient_store.payer,
        &ctx.accounts.instructions,
    )?;
    let call_start = transient_store.len();
    // The account table owns every remaining-accounts invariant for the execution:
    // duplicate rejection (at construction), the used-account bitmap (marked in
    // preflight), canonical Store validation, and cached Store/transient store writes.
    let mut account_table = ExecutionAccountTable::new(ctx.remaining_accounts)?;
    // Preflight also settles the execution's application identity: the one `(program, scope)`
    // every Store the default authority controls belongs to. Metering and rand seeds key on it;
    // the deny list gates every application the execution touches.
    let preflight = preflight_execution(&mut account_table, &ctx, &args)?;
    let app = preflight.app;
    let host_config = &ctx.accounts.host_config;
    for touched in preflight.touched_apps {
        let deny_record =
            account_table.deny_record(host_config.grant_deny_list_enabled, touched)?;
        check_scope_not_denied_info(host_config, touched, deny_record)?;
    }

    let starting_hcu = transient_store.total_hcu;

    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    let handle_context = ExecutionHandleContext {
        derivation: HandleDerivationContext {
            chain_id: ctx.accounts.host_config.chain_id,
            previous_bank_hash,
            unix_timestamp: clock.unix_timestamp,
        },
        rand: rand_nonce.map(|nonce| RandContext { nonce, app }),
    };
    let random_seeds = collect_execution_random_seeds(&args, &handle_context)?;
    // Execution is the single walk: it validates each step as it mutates. A failure mid-execution
    // leaves partial writes behind only until the runtime reverts the transaction, which discards
    // every account write — so no validate-only pre-pass is needed for atomicity. The event CPI
    // stays last so no event describes state that did not commit.
    let created_public_outputs = execute_steps(
        &mut account_table,
        &mut transient_store,
        call_start,
        &args,
        app,
        &handle_context,
        &ctx.accounts.host_config,
    )?;
    let execution_hcu = transient_store.total_hcu - starting_hcu;
    drop(transient_store);
    block_cap::charge(&ctx, app, execution_hcu, clock.slot)?;
    account_table.flush_states(
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;
    emit_execution_random_seeds(&ctx, random_seeds)?;
    emit_public_outputs_produced(&ctx, created_public_outputs)?;
    // Event CPIs may replace return data; restore the selected handles afterwards.
    return_execution_handles(
        &*ctx.accounts.transient_store.load()?,
        call_start,
        &args.returned_results,
    );
    Ok(())
}

fn validate_result_refs(args: &FheExecuteArgs) -> Result<()> {
    require!(
        args.returned_results.len() <= crate::MAX_RETURNED_HANDLES,
        ZamaHostError::InvalidReturnSelection
    );
    for result in args
        .returned_results
        .iter()
        .chain(args.effects.iter().map(|effect| &effect.result))
    {
        require!(
            usize::from(result.step_index) < args.steps.len() && result.output_index == 0,
            ZamaHostError::InvalidReturnSelection
        );
    }
    Ok(())
}

#[inline(never)]
fn return_execution_handles(
    transient_store: &TransientStore,
    call_start: usize,
    selected: &[crate::ExecutionResultRef],
) {
    let mut bytes = [0u8; crate::MAX_RETURNED_HANDLES * 32];
    for (chunk, result) in bytes.chunks_exact_mut(32).zip(selected) {
        chunk.copy_from_slice(
            &transient_store
                .result(call_start + usize::from(result.step_index))
                .expect("validated execution result")
                .handle,
        );
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
    transient_store: &mut TransientStore,
    call_start: usize,
    args: &FheExecuteArgs,
    app: AppScope,
    handle_context: &ExecutionHandleContext,
    host_config: &HostConfig,
) -> Result<Vec<ProducedPublicOutput>> {
    let producer_store = table.account(args.execution_store_index.into())?.key();
    let mut execution = ExecutionState {
        table,
        transient_store,
        call_start,
        producer_store,
        dictionary: &args.dictionary,
        app,
        chain_id: handle_context.derivation.chain_id,
        host_config,
    };
    walk_steps(&mut execution, args, handle_context)?;
    let mut public_outputs = Vec::new();
    for effect in &args.effects {
        let handle = execution
            .transient_store
            .result(call_start + usize::from(effect.result.step_index))
            .ok_or(ZamaHostError::InvalidReturnSelection)?
            .handle;
        let state = store_output::accept_store_output(
            execution.table,
            &args.dictionary,
            execution.transient_store,
            effect.store_index,
            effect.previous_leaf_count,
            &effect.slot,
            &effect.allow_indexes,
            effect.make_public,
            &effect.grants,
            handle,
        )?;
        if effect.make_public {
            public_outputs.push(ProducedPublicOutput {
                step_index: u16::from(effect.result.step_index),
                encrypted_store: state,
                output_handle: handle,
            });
        }
    }
    Ok(public_outputs)
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
    transient_store: &'t mut TransientStore,
    call_start: usize,
    producer_store: Pubkey,
    /// The application every persistent value of the execution belongs to; `None` when the
    /// execution touches none.
    app: AppScope,
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
        Ok(self.encrypted_operand(attestation.input_handle))
    }

    #[inline(never)]
    fn accept_output(
        &mut self,
        result: [u8; 32],
        cost: u64,
        operands: &[ResolvedOperand],
    ) -> Result<()> {
        let total = hcu::accumulate_total(self.transient_store.total_hcu, cost)?;
        hcu::enforce_le(
            total,
            self.host_config.max_hcu_per_tx,
            ZamaHostError::HcuTransactionLimitExceeded,
        )?;
        let input_depth = operands
            .iter()
            .map(|operand| operand.depth)
            .max()
            .unwrap_or(0);
        let depth = hcu::step_depth(cost, input_depth)?;
        hcu::enforce_le(
            depth,
            self.host_config.max_hcu_depth_per_tx,
            ZamaHostError::HcuTransactionDepthLimitExceeded,
        )?;
        self.transient_store
            .record(result, self.producer_store, depth)?;
        self.transient_store.total_hcu = total;
        Ok(())
    }
}

fn resolve_dictionary_keys(dictionary: &[[u8; 32]], indexes: &[u8]) -> Result<Vec<Pubkey>> {
    indexes
        .iter()
        .map(|index| dictionary_key(dictionary, *index))
        .collect()
}

#[derive(Clone, Copy)]
pub(super) struct ResolvedOperand {
    pub(super) handle: [u8; 32],
    pub(super) scalar: bool,
    boundary: bool,
    depth: u64,
}

impl ResolvedOperand {
    fn scalar(handle: [u8; 32]) -> Self {
        Self {
            handle,
            scalar: true,
            boundary: false,
            depth: 0,
        }
    }
}

impl ExecutionState<'_, '_, '_> {
    fn encrypted_operand(&self, handle: [u8; 32]) -> ResolvedOperand {
        let depth = self.transient_store.origin_depth(handle);
        ResolvedOperand {
            handle,
            scalar: false,
            boundary: depth.is_none(),
            depth: depth.unwrap_or(0),
        }
    }
}

fn boundary_mask(operands: &[ResolvedOperand]) -> Result<[u8; 32]> {
    operand_boundary_mask(operands.iter().map(|operand| operand.boundary))
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
