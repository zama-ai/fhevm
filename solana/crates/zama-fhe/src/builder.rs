//! `FheExecutionBuilder`'s admission machine: the step commit path and three typed resource ceilings.
//!
//! Public API surface: app programs. The named op methods live in `ops.rs` — this file is the
//! part that decides whether a step, and finally the execution, is admitted at all.
//!
//! Building on-chain: lowering interns into the builder's own tables and never copies them, so a
//! step costs a few hundred heap bytes, and the step-bounded tables reserve their bound up front
//! (a fixed ~10 KB every build pays) so growth never strands outgrown buffers on the
//! never-freeing bump region (DD-046: the heap is fixed at 32 KB). The instruction pays out of
//! that region three times — building, serializing the packet, and assembling the CPI account
//! tables in `FheExecution::invoke` — and the budget below charges all three.
//!
//! Three resource ceilings reject an oversized build before it reaches the app's CPI path.
//! The app must also budget its own allocations and the surrounding transaction:
//!
//! - **Steps** — the host's `MAX_FHE_EXECUTION_STEPS`, the one step ceiling
//!   ([`FheExecutionBuildError::TooManySteps`]), gated in [`FheExecutionBuilder::commit_step`].
//! - **CPI packet** — the serialized packet must fit the 10 KiB a CPI may carry, counted
//!   exactly at `finish` ([`FheExecutionBuildError::ExceedsCpiInstructionDataLimit`]).
//! - **Build heap** — the builder admits every byte it requests from the allocator against
//!   [`crate::BUILD_HEAP_BUDGET_BYTES`] before the allocator serves it ([`HeapBudget`]), leaving
//!   [`crate::APP_HEAP_RESERVE_BYTES`] for what it genuinely cannot see: Anchor's account
//!   deserialization and the app's own allocations
//!   ([`FheExecutionBuildError::ExceedsBuildHeapBudget`]). Intern tables grow through
//!   `TalliedVec::try_push` (there is no `DerefMut` to the `Vec`); exact-size `Vec` sites go
//!   through `try_with_capacity`; attestation embeds `admit` then clone. Packet and
//!   invoke terms land at `finish`, where they are first known. The tally is validated
//!   byte-for-byte against a counting allocator across the whole shape frontier in `heap_budget`,
//!   and the never-crosses claim has its own adversarial test there.
//!
//! The host's heap is not covered by these ceilings: its allocations depend on live State size,
//! MMR peaks and permissions per output. Boundary sweeps in `runtime-tests/tests/fhe_execute_boundary.rs`
//! measure those limits (invariant #61). State creation is separate; execution may grow existing
//! States and top up rent. [`FheExecution::cost`] reports packet bytes, trace bounds and tallied
//! app-side heap so callers can budget the surrounding transaction.

use zama_host::{
    AppScope, CoprocessorInputAttestation, FheExecuteArgs, FheExecuteEffect, FheExecuteOperand,
    FheExecuteStep, MAX_FHE_EXECUTION_STEPS,
};

use anchor_lang::prelude::Pubkey;

use crate::accounts::{ExecutionAccountMeta, ExecutionAccountPurpose};
use crate::execution::FheExecution;
use crate::heap_tally::{HeapBudget, TalliedVec};
use crate::lower::{lower_effect, lower_operand, StepTables};
use crate::operand::{BuilderIdentity, Operand, OperandKind};
use crate::validate::{validate_authority, validate_lowered_execution};
use crate::{FheExecutionBuildError, Result};

/// Pubkey-oriented builder for `FheExecuteArgs`.
///
/// `'id` is this builder's identity: [`FheExecution::build`] hands every invocation a fresh invariant
/// lifetime, every transient value it returns carries it, and the op methods only accept values of
/// their own identity — so mixing two builders' values does not compile. That replaces a runtime tag
/// which SBF could not make unique (writable statics are forbidden on-chain, so every builder in a
/// program shared one scope number and the check found nothing). It is also why there is no public
/// constructor and no `Clone`: both would hand out a second builder wearing the same identity.
#[derive(Debug)]
pub struct FheExecutionBuilder<'id> {
    pub(crate) identity: BuilderIdentity<'id>,
    pub(crate) state: crate::StateId,
    pub(crate) steps: TalliedVec<FheExecuteStep>,
    pub(crate) effects: TalliedVec<FheExecuteEffect>,
    pub(crate) produced_types: TalliedVec<u8>,
    pub(crate) remaining_accounts: TalliedVec<ExecutionAccountMeta>,
    /// Interned 32-byte constant dictionary the lowered steps reference by `u8` index (operand
    /// handles, scalars, programs, authorities, scopes, labels, seeds, allowed keys, previous
    /// handles). The entries are deliberately untyped so one entry can serve several roles; see
    /// `FheExecuteArgs::dictionary` in zama-host for why typing them would cost packet bytes.
    pub(crate) dictionary: TalliedVec<[u8; 32]>,
    /// Coprocessor attestations backing `VerifiedInput` operands, referenced by index. Held here
    /// (rather than inline in the operand) so `Operand` stays `Copy`.
    pub(crate) verified_inputs: TalliedVec<CoprocessorInputAttestation>,
    /// Application of States controlled by the default authority, once referenced. The host
    /// meters and rand-seeds on it; States under additional signers may belong to other apps.
    pub(crate) app: AppScope,
    /// Committed State outputs; bounds possible rent top-ups in the instruction-trace estimate.
    pub(crate) state_outputs: usize,
    /// Whether any committed step is a rand step (the host emits one random-seeds event CPI).
    pub(crate) has_rand_step: bool,
    /// Whether any committed output is `make_public` (the host emits one public-outputs event CPI).
    pub(crate) has_public_output: bool,
    /// The one running total of every byte this build has admitted. Intern tables grow through
    /// it; exact-size sites charge it; `finish` tests packet and invoke against it.
    pub(crate) budget: HeapBudget,
}

/// One step's view of the builder — see [`FheExecutionBuilder::commit_step`].
pub(crate) struct StepLowering<'b> {
    steps_len: usize,
    authority: Pubkey,
    pub(crate) tables: StepTables<'b>,
    verified_inputs: &'b [CoprocessorInputAttestation],
    /// The execution's application as this step sees it; adopted by the builder on commit.
    app: AppScope,
}

impl StepLowering<'_> {
    pub(crate) fn operand(&mut self, operand: Operand) -> Result<FheExecuteOperand> {
        match operand.0 {
            OperandKind::StateSlot { state, .. } => self.fold_app(state.authority, state.app)?,
            OperandKind::Granted { consumer, .. } => {
                self.fold_app(consumer.authority, consumer.app)?
            }
            _ => {}
        }

        lower_operand(
            &mut self.tables,
            self.authority,
            self.steps_len,
            self.verified_inputs,
            operand,
        )
    }

    pub(crate) fn budget(&mut self) -> &mut HeapBudget {
        self.tables.budget()
    }

    /// Lowers a reduction's operand iterator into one exact-size table. Scalar checks and the
    /// operand cap live here so `sum` / `is_in` do not each carry a copy of the admission loop.
    /// `into_inner` happens at `FheExecuteStep` construction — the wire seam.
    pub(crate) fn reduction_operands(
        &mut self,
        operands: impl Iterator<Item = Operand>,
        max: usize,
    ) -> Result<TalliedVec<FheExecuteOperand>> {
        let (hinted, _) = operands.size_hint();
        let reserved = hinted.min(max);
        let mut lowered = TalliedVec::try_with_capacity(self.budget(), reserved)?;
        for operand in operands {
            if matches!(operand.0, OperandKind::Scalar(_)) {
                return Err(FheExecutionBuildError::ScalarEncryptedOperand);
            }
            if lowered.len() == max {
                return Err(FheExecutionBuildError::TooManyReductionOperands);
            }
            let lowered_op = self.operand(operand)?;
            lowered.try_push(self.budget(), lowered_op)?;
        }
        Ok(lowered)
    }

    /// Mirrors the host's one-application rule (`FheExecuteMixedScopes`): the first persistent
    /// value the default authority controls fixes the execution's application, every later one
    /// must match. A value under an additional signing authority belongs to that program.
    fn fold_app(&mut self, authority: Pubkey, app: AppScope) -> Result<()> {
        if authority != self.authority {
            return Ok(());
        }
        if self.app != app {
            return Err(FheExecutionBuildError::MixedScopes);
        }
        Ok(())
    }
}

impl<'id> FheExecutionBuilder<'id> {
    /// The single mutation path for appending a step. Every op method validates first, then lowers
    /// through this: lowering interns into the builder's own tables, and dropping an uncommitted
    /// [`StepTables`] undoes what the step wrote, so a failed step leaves the builder exactly as
    /// it was. The tables are never copied per step — an app program builds its execution on the
    /// entrypoint's fixed 32 KB bump heap, which is never freed, so a clone-and-swap rollback would
    /// make the heap cost of an execution grow with the square of its step count.
    ///
    /// This is also the canonical [`TooManySteps`](FheExecutionBuildError::TooManySteps) gate:
    /// steps only ever grow through here, so the op methods and `finish` do not re-check it.
    /// Heap admission lives on [`HeapBudget`]: intern tables grow through `try_push`, exact-size
    /// `Vec` sites through `try_with_capacity`. Packet and invoke land at `finish`.
    ///
    pub(crate) fn commit_step(
        &mut self,
        produced_type: u8,
        lower: impl FnOnce(&mut StepLowering<'_>) -> Result<FheExecuteStep>,
    ) -> Result<u8> {
        // Checked before the step interns anything, so a build stopped here leaves the tables
        // exactly at the host's cap.
        if self.steps.len() >= MAX_FHE_EXECUTION_STEPS {
            return Err(FheExecutionBuildError::TooManySteps);
        }
        let op_index =
            u8::try_from(self.steps.len()).map_err(|_| FheExecutionBuildError::TooManySteps)?;
        let Self {
            state,
            steps,
            effects: _,
            produced_types,
            remaining_accounts,
            dictionary,
            verified_inputs,
            app,
            state_outputs: _,
            has_rand_step,
            has_public_output: _,
            budget,
            identity: _,
        } = self;
        let mut lowering = StepLowering {
            steps_len: steps.len(),
            authority: state.authority(),
            tables: StepTables::open(remaining_accounts, dictionary, budget),
            verified_inputs,
            app: *app,
        };
        match lower(&mut lowering) {
            Ok(step) => {
                let is_rand = matches!(
                    step,
                    FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
                );
                // Push while Drop still rolls intern tables. `steps` / `produced_types` are
                // not borrowed by lowering; the budget is.
                steps.try_push(lowering.budget(), step)?;
                produced_types.try_push(lowering.budget(), produced_type)?;
                *has_rand_step |= is_rand;
                *app = lowering.app;
                lowering.tables.commit();
                Ok(op_index)
            }
            Err(error) => Err(error),
        }
    }

    /// Applies a State write or permission to a produced result after the arithmetic.
    /// Reads in this execution keep seeing their initial State snapshot.
    pub fn output<T: crate::FheTyped>(
        &mut self,
        value: crate::Encrypted<'id, T>,
        output: crate::StateOutput,
    ) -> Result<()> {
        let OperandKind::Transient { producer_index } = value.operand().0 else {
            return Err(FheExecutionBuildError::ResultNotProduced);
        };
        if usize::from(producer_index) >= self.steps.len() {
            return Err(FheExecutionBuildError::InvalidTransientReference);
        }
        if self.effects.len() == zama_host::MAX_FHE_EXECUTION_EFFECTS {
            return Err(FheExecutionBuildError::TooManyEffects);
        }
        let mut lowering = StepLowering {
            steps_len: self.steps.len(),
            authority: self.state.authority(),
            tables: StepTables::open(
                &mut self.remaining_accounts,
                &mut self.dictionary,
                &mut self.budget,
            ),
            verified_inputs: &self.verified_inputs,
            app: self.app,
        };
        lowering.fold_app(output.state.authority, output.state.app)?;
        let result = zama_host::ExecutionResultRef {
            step_index: producer_index,
            output_index: 0,
        };
        let mut effect =
            lower_effect(&mut lowering.tables, self.state.authority(), result, output)?;
        if let Some(slot) = &effect.slot {
            if self.effects.iter().any(|earlier| {
                earlier.state_index == effect.state_index
                    && earlier
                        .slot
                        .as_ref()
                        .is_some_and(|previous| previous.key_index == slot.key_index)
            }) {
                return Err(FheExecutionBuildError::DuplicateSlotWrite);
            }
        }
        advance_state_history(&mut effect, &self.effects)?;
        let makes_public = effect.make_public;
        self.effects.try_push(lowering.budget(), effect)?;
        lowering.tables.commit();
        self.app = lowering.app;
        self.state_outputs += 1;
        self.has_public_output |= makes_public;
        Ok(())
    }

    /// Crate-internal: a public constructor would let two builders share one identity, which is the
    /// mixing hazard the identity exists to remove. App code gets a builder from [`FheExecution::build`].
    pub(crate) fn new(state: crate::StateId) -> Self {
        // Growth by doubling strands every outgrown buffer on the entrypoint's never-freeing
        // bump heap, so the step-bounded tables reserve their per-execution bound up front —
        // each reservation is charged to the budget. `verified_inputs` stays empty: most
        // executions carry no attestations.
        let mut budget = HeapBudget::new();
        fn reserved<T>(budget: &mut HeapBudget, capacity: usize) -> TalliedVec<T> {
            TalliedVec::try_with_capacity(budget, capacity)
                .expect("step-table reservation fits BUILD_HEAP_BUDGET_BYTES")
        }
        let mut builder = Self {
            identity: std::marker::PhantomData,
            state,
            steps: reserved(&mut budget, MAX_FHE_EXECUTION_STEPS),
            effects: reserved(&mut budget, zama_host::MAX_FHE_EXECUTION_EFFECTS),
            produced_types: reserved(&mut budget, MAX_FHE_EXECUTION_STEPS),
            remaining_accounts: reserved(&mut budget, MAX_FHE_EXECUTION_STEPS),
            dictionary: reserved(&mut budget, 2 * MAX_FHE_EXECUTION_STEPS),
            verified_inputs: TalliedVec::new(),
            app: state.app(),
            state_outputs: 0,
            has_rand_step: false,
            has_public_output: false,
            budget,
        };
        builder
            .remaining_accounts
            .try_push(
                &mut builder.budget,
                ExecutionAccountMeta::readonly(
                    state.address(),
                    ExecutionAccountPurpose::StateInput,
                ),
            )
            .expect("initial State fits the reserved account table");
        builder
    }

    /// Every byte this build has admitted against the heap budget. On the entrypoint's
    /// never-freeing bump region the total requested is what decides whether the instruction
    /// survives. Test-only: production code goes through `finish`'s `fits_with` / `admit`.
    #[cfg(test)]
    pub(crate) fn requested_heap_bytes(&self) -> usize {
        self.budget.total()
    }

    /// Validates the accumulated execution and lowers it to an [`FheExecution`].
    ///
    /// Mirrors the host preflight checks (non-empty steps, one application per execution) so a
    /// malformed execution fails locally instead of on-chain; the step cap needs no re-check
    /// because [`commit_step`](Self::commit_step) is the only way steps grow.
    ///
    pub(crate) fn finish(self) -> Result<FheExecution> {
        self.finish_returning(Vec::new())
    }

    pub(crate) fn finish_returning(
        self,
        returned_results: Vec<zama_host::ExecutionResultRef>,
    ) -> Result<FheExecution> {
        validate_authority(self.state.authority())?;
        if self.steps.is_empty() {
            return Err(FheExecutionBuildError::EmptySteps);
        }
        let dynamic_accounts = self
            .remaining_accounts
            .iter()
            .filter(|meta| meta.requires_dynamic_account())
            .count();
        let value_authorities = 1 + self
            .remaining_accounts
            .iter()
            .filter(|meta| meta.requires_state_authority())
            .count();
        let return_allocation =
            returned_results.capacity() * std::mem::size_of::<zama_host::ExecutionResultRef>();
        let invoke_heap_bytes = returned_results.len() * 32
            + crate::heap_tally::invoke_table_heap_bytes(
                self.remaining_accounts.len(),
                dynamic_accounts,
                value_authorities,
            );
        let account_count = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        let args = FheExecuteArgs {
            execution_state_index: 0,
            returned_results,
            account_count,
            dictionary: self.dictionary.into_inner(),
            steps: self.steps.into_inner(),
            effects: self.effects.into_inner(),
        };
        // An fhe_execute packet always travels by CPI — a transaction itself carries at most
        // 1,232 bytes, so no full-size packet can be submitted top-level — and the runtime
        // rejects any CPI over the data limit. Counted here (allocating nothing) so an
        // undeliverable execution fails with a typed error instead of aborting the invoke.
        let packet_bytes = crate::execution::packet_byte_count(&args);
        if packet_bytes > crate::cost::CPI_INSTRUCTION_DATA_LIMIT {
            return Err(FheExecutionBuildError::ExceedsCpiInstructionDataLimit);
        }
        // Packet and invoke tables are everything still uncharged. Finish's used-entry
        // bitmaps live on the stack: wire indexes are `u8`.
        if !self
            .budget
            .fits_with(return_allocation + packet_bytes + invoke_heap_bytes)
        {
            return Err(FheExecutionBuildError::ExceedsBuildHeapBudget);
        }
        let mut used_accounts = [false; 256];
        let mut used_dictionary = [false; 256];
        used_accounts[usize::from(args.execution_state_index)] = true;
        validate_lowered_execution(
            &args.steps,
            &args.effects,
            &self.remaining_accounts,
            &args.dictionary,
            &mut used_accounts[..self.remaining_accounts.len()],
            &mut used_dictionary[..args.dictionary.len()],
        )?;
        let build_heap_bytes = self.budget.total() + return_allocation;
        let cost = crate::cost::FheExecutionCost {
            steps: args.steps.len(),
            state_outputs: self.state_outputs,
            emits_random_seeds_event: self.has_rand_step,
            emits_public_outputs_event: self.has_public_output,
            packet_bytes,
            build_heap_bytes,
            invoke_heap_bytes,
            remaining_accounts: self.remaining_accounts.len(),
            dynamic_accounts,
            value_authorities,
        };
        Ok(FheExecution {
            state: self.state,
            has_rand_step: self.has_rand_step,
            args,
            remaining_accounts: self.remaining_accounts.into_inner(),
            cost,
        })
    }
}

fn advance_state_history(
    effect: &mut FheExecuteEffect,
    previous: &[FheExecuteEffect],
) -> Result<()> {
    let mut expected = effect.previous_leaf_count;
    for earlier in previous
        .iter()
        .filter(|earlier| earlier.state_index == effect.state_index)
    {
        if earlier.previous_leaf_count != expected {
            return Err(FheExecutionBuildError::StateHistoryMismatch);
        }
        expected = expected
            .checked_add(earlier.allow_indexes.len() as u64 + u64::from(earlier.make_public))
            .ok_or(FheExecutionBuildError::StateHistoryMismatch)?;
    }
    effect.previous_leaf_count = expected;
    Ok(())
}
