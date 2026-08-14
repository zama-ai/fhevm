//! `FheExecutionBuilder`'s admission machine: the step commit path and the four typed ceilings.
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
//! An execution `build` returns is one whose *app-side* instruction fits: four ceilings make
//! every wall the builder can see a typed rejection instead of a runtime abort, each an exact
//! function of the shape:
//!
//! - **Steps** — the host's `MAX_FHE_EXECUTION_STEPS`, the one step ceiling
//!   ([`FheExecutionBuildError::TooManySteps`]), gated in [`FheExecutionBuilder::commit_step`].
//! - **Instruction trace** — three system CPIs per created output plus the event CPIs, checked
//!   per step against the transaction's 64-instruction trace; at most 20 creates fit
//!   ([`FheExecutionBuildError::ExceedsInstructionTraceLimit`]).
//! - **CPI packet** — the serialized packet must fit the 10 KiB a CPI may carry, counted
//!   exactly at `finish` ([`FheExecutionBuildError::ExceedsCpiInstructionDataLimit`]).
//! - **Build heap** — the builder tallies every byte it requests from the allocator and holds
//!   build + packet + the invoke-side account tables under
//!   [`crate::BUILD_HEAP_BUDGET_BYTES`], leaving [`crate::APP_HEAP_RESERVE_BYTES`] for what it
//!   genuinely cannot see: Anchor's account deserialization and the app's own allocations
//!   ([`FheExecutionBuildError::ExceedsBuildHeapBudget`]). The build side is checked per
//!   committed step — so a shape that could exhaust the real region mid-build is rejected at
//!   the step that crosses the budget, like the other per-step ceilings — and the packet and
//!   invoke terms land at `finish`, where they are first known. The tally is validated
//!   byte-for-byte against a counting allocator across the whole shape frontier in
//!   `heap_budget.rs`, so it cannot silently drift from what the code allocates.
//!
//! One wall is deliberately *not* typed, because no app-side number can see it: the host's own
//! CPI frame grows with created outputs times subjects per output, and for shared-audience
//! `make_public` creates it aborts before the ceilings above do (15 eight-subject public
//! creates land, the 16th dies host-side, while the app-side ceilings admit 20). That
//! wall — like the host's heap cost for MMR-mature updates — is pinned by the boundary sweeps
//! in `runtime-tests/tests/fhe_execute_boundary.rs` and documented in invariant #54;
//! see `crate::cost`'s module doc for the mechanism. `FheExecution::cost`
//! reports the exact packet bytes, trace floor, and tallied heap so an app composing a larger
//! transaction can budget the rest.

use zama_host::{
    CoprocessorInputAttestation, FheExecuteArgs, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep, MAX_FHE_EXECUTION_STEPS,
};

use crate::accounts::{ExecutionAccountMeta, ExecutionEncryptedValueAccountAuthority};
use crate::acl::Output;
use crate::execution::FheExecution;
use crate::heap_tally::TalliedVec;
use crate::lower::{lower_operand, lower_output, StepTables};
use crate::operand::{BuilderIdentity, Operand};
use crate::validate::{
    validate_encrypted_value_account_authority, validate_lowered_execution,
    validate_rand_steps_anchor_persistent_output,
};
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
    pub(crate) encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    pub(crate) steps: TalliedVec<FheExecuteStep>,
    pub(crate) produced_types: TalliedVec<u8>,
    /// Persistent accounts this execution has already written. A later persistent-shaped reference
    /// to one of them is rejected with `PersistentOperandWrittenEarlier`: the app must feed the
    /// earlier step's transient value instead, which is the only spelling the host accepts.
    pub(crate) persistent_producers: TalliedVec<anchor_lang::prelude::Pubkey>,
    pub(crate) remaining_accounts: TalliedVec<ExecutionAccountMeta>,
    /// Interned 32-byte constant dictionary the lowered steps reference by `u8` index (operand
    /// handles, scalars, ACL domain keys, encrypted value account authorities, labels, subjects).
    /// The entries are deliberately untyped so one entry can serve several roles; see
    /// `FheExecuteArgs::dictionary` in zama-host for why typing them would cost packet bytes.
    pub(crate) dictionary: TalliedVec<[u8; 32]>,
    /// Coprocessor attestations backing `VerifiedInput` operands, referenced by index. Held here
    /// (rather than inline in the operand) so `Operand` stays `Copy`.
    pub(crate) verified_inputs: TalliedVec<CoprocessorInputAttestation>,
    /// Persistent outputs committed so far that create their account — three system CPIs each on
    /// the host, which is what [`crate::cost::instruction_trace_floor`] charges against the
    /// transaction's instruction trace.
    pub(crate) persistent_creates: usize,
    /// Persistent outputs committed so far that update an existing account.
    pub(crate) persistent_updates: usize,
    /// Whether any committed step is a rand step (the host emits one random-seeds event CPI).
    pub(crate) has_rand_step: bool,
    /// Whether any committed output is `make_public` (the host emits one public-outputs event CPI).
    pub(crate) has_public_output: bool,
    /// Requested bytes that are not a [`TalliedVec`]'s own growth: the exact-size allocations
    /// (attestation embeds, subject index lists, purpose tables, `finish`'s validation bitmaps)
    /// and the harvested step-local tables. The tables carry their own requests;
    /// [`requested_heap_bytes`](Self::requested_heap_bytes) is the sum of both, and
    /// `heap_budget.rs` asserts that sum matches a counting allocator byte-for-byte, so an
    /// allocation that forgets to pay fails the build there.
    pub(crate) explicit_heap_bytes: usize,
}

/// One step's view of the builder — see [`FheExecutionBuilder::commit_step`].
pub(crate) struct StepLowering<'b> {
    steps_len: usize,
    encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    pub(crate) tables: StepTables<'b>,
    verified_inputs: &'b [CoprocessorInputAttestation],
}

impl StepLowering<'_> {
    pub(crate) fn operand(&mut self, operand: Operand) -> Result<FheExecuteOperand> {
        lower_operand(
            &mut self.tables,
            self.steps_len,
            self.verified_inputs,
            operand,
        )
    }

    /// Pays step-local allocation bytes into the builder's tally.
    pub(crate) fn tally_bytes(&mut self, bytes: usize) {
        self.tables.tally_bytes(bytes);
    }

    pub(crate) fn output(&mut self, output: Output) -> Result<FheExecuteOutput> {
        lower_output(
            &mut self.tables,
            self.encrypted_value_account_authority,
            output,
        )
    }
}

impl<'id> FheExecutionBuilder<'id> {
    /// The single mutation path for appending a step. Every op method validates first, then lowers
    /// through this: lowering interns into the builder's own tables and, when any part of the step
    /// fails, [`StepTables::rollback`] undoes what it wrote, so a failed step leaves the builder
    /// exactly as it was. The tables are never copied per step — an app program builds its execution on
    /// the entrypoint's fixed 32 KB bump heap, which is never freed, so a clone-and-swap rollback would
    /// make the heap cost of an execution grow with the square of its step count.
    ///
    /// This is also the canonical [`TooManySteps`](FheExecutionBuildError::TooManySteps) gate:
    /// steps only ever grow through here, so the op methods and `finish` do not re-check it.
    ///
    /// Ordering dependency inside a step: `operand()` reads `persistent_producers`, which still
    /// holds the pre-step state only because every op lowers its operands before its output. An op
    /// that lowered its output first would silently change what `PersistentOperandWrittenEarlier`
    /// sees for its own operands — keep operands first.
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
            encrypted_value_account_authority,
            steps,
            produced_types,
            persistent_producers,
            remaining_accounts,
            dictionary,
            verified_inputs,
            persistent_creates,
            persistent_updates,
            has_rand_step,
            has_public_output,
            explicit_heap_bytes,
            identity: _,
        } = self;
        // These three tables only mutate through `lowering` below, so their pre-step requests can
        // be captured here for the per-step budget check.
        let stable_table_bytes = steps.requested_bytes()
            + produced_types.requested_bytes()
            + verified_inputs.requested_bytes();
        let mut lowering = StepLowering {
            steps_len: steps.len(),
            encrypted_value_account_authority: *encrypted_value_account_authority,
            tables: StepTables::open(
                remaining_accounts,
                dictionary,
                persistent_producers,
                explicit_heap_bytes,
            ),
            verified_inputs,
        };
        match lower(&mut lowering) {
            Ok(step) => {
                let output = crate::execution::fhe_execute_step_output(&step);
                let creates_account = matches!(
                    output,
                    FheExecuteOutput::StoredValue {
                        previous_state: None,
                        ..
                    }
                );
                let updates_account = matches!(
                    output,
                    FheExecuteOutput::StoredValue {
                        previous_state: Some(_),
                        ..
                    }
                );
                let makes_public = matches!(
                    output,
                    FheExecuteOutput::StoredValue {
                        make_public: true,
                        ..
                    }
                );
                let is_rand = matches!(
                    step,
                    FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
                );
                // Every created output costs the transaction three CPIs on the host, so the
                // instruction trace runs out before the step cap does on create-heavy shapes.
                // Checked per step against the floor — the count the transaction pays even in
                // the minimal wrapper — so the step that can never land is the one rejected.
                let floor = crate::cost::instruction_trace_floor(
                    *persistent_creates + usize::from(creates_account),
                    *has_rand_step || is_rand,
                    *has_public_output || makes_public,
                );
                if floor > crate::cost::TRANSACTION_INSTRUCTION_TRACE_LIMIT {
                    lowering.tables.rollback();
                    return Err(FheExecutionBuildError::ExceedsInstructionTraceLimit);
                }
                // The build side of the heap budget, checked at the step that crosses it — the
                // packet and invoke-table terms land at `finish`, where they are first known.
                // Without this, a shape whose *build alone* outgrows the budget would keep
                // allocating on the real, never-freeing region all the way to `finish`; with it,
                // the running request stays under the budget after every admitted step, and no
                // single step's allocations exceed the reserve — so the region itself can never
                // be exhausted mid-build. The rolled-back tables stay truncated but their
                // requests stay counted: on a bump region a rejected step's bytes are spent.
                if stable_table_bytes + lowering.tables.requested_bytes()
                    > crate::cost::BUILD_HEAP_BUDGET_BYTES
                {
                    lowering.tables.rollback();
                    return Err(FheExecutionBuildError::ExceedsBuildHeapBudget);
                }
                *persistent_creates += usize::from(creates_account);
                *persistent_updates += usize::from(updates_account);
                *has_rand_step |= is_rand;
                *has_public_output |= makes_public;
                drop(lowering);
                // Both stay within their up-front reservation (the step cap was checked above);
                // the tallied push keeps the total honest if that ever changes.
                steps.push(step);
                produced_types.push(produced_type);
                Ok(op_index)
            }
            Err(error) => {
                lowering.tables.rollback();
                Err(error)
            }
        }
    }

    /// Crate-internal: a public constructor would let two builders share one identity, which is the
    /// mixing hazard the identity exists to remove. App code gets a builder from [`FheExecution::build`].
    pub(crate) fn new(
        encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    ) -> Self {
        // Growth by doubling strands every outgrown buffer on the entrypoint's never-freeing
        // bump heap, so the step-bounded tables reserve their per-execution bound up front —
        // each `TalliedVec` records its own reservation as requested bytes.
        // `verified_inputs` stays empty: most executions carry no attestations.
        Self {
            identity: std::marker::PhantomData,
            encrypted_value_account_authority,
            steps: TalliedVec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            produced_types: TalliedVec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            persistent_producers: TalliedVec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            remaining_accounts: TalliedVec::with_capacity(MAX_FHE_EXECUTION_STEPS),
            dictionary: TalliedVec::with_capacity(2 * MAX_FHE_EXECUTION_STEPS),
            verified_inputs: TalliedVec::new(),
            persistent_creates: 0,
            persistent_updates: 0,
            has_rand_step: false,
            has_public_output: false,
            explicit_heap_bytes: 0,
        }
    }

    /// Every byte this build has requested from the allocator: each table's reservation and
    /// growth, plus the exact-size allocations on the explicit counter. On the entrypoint's
    /// never-freeing bump region the total requested is what decides whether the instruction
    /// survives.
    pub(crate) fn requested_heap_bytes(&self) -> usize {
        self.explicit_heap_bytes
            + self.steps.requested_bytes()
            + self.produced_types.requested_bytes()
            + self.persistent_producers.requested_bytes()
            + self.remaining_accounts.requested_bytes()
            + self.dictionary.requested_bytes()
            + self.verified_inputs.requested_bytes()
    }

    /// Validates the accumulated execution and lowers it to an [`FheExecution`].
    ///
    /// Mirrors the host preflight checks (non-empty steps, rand steps anchored by a persistent
    /// output) so a malformed execution fails locally instead of on-chain; the step cap needs no
    /// re-check because [`commit_step`](Self::commit_step) is the only way steps grow.
    ///
    /// Not mirrored (it depends on the deployed `hcu_block_cap_per_app`, unknown here): under a
    /// finite block cap the host rejects a persist-nothing execution — one binding no persistent input, no
    /// verified input, and no persistent output — with `FheExecuteUnanchoredUnderBlockCap`
    /// (fhevm-internal#1744). Give such an execution a persistent output (the bootstrap/mint path) or a
    /// verified input if it must run under a finite cap.
    pub(crate) fn finish(mut self) -> Result<FheExecution> {
        validate_encrypted_value_account_authority(self.encrypted_value_account_authority)?;
        if self.steps.is_empty() {
            return Err(FheExecutionBuildError::EmptySteps);
        }
        // The validation below allocates its two one-byte-per-entry usage bitmaps.
        self.explicit_heap_bytes += self.remaining_accounts.len() + self.dictionary.len();
        validate_lowered_execution(&self.steps, &self.remaining_accounts, &self.dictionary)?;
        validate_rand_steps_anchor_persistent_output(&self.steps)?;
        let account_count = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        let build_heap_bytes = self.requested_heap_bytes();
        let args = FheExecuteArgs {
            account_count,
            dictionary: self.dictionary.into_inner(),
            steps: self.steps.into_inner(),
        };
        // An fhe_execute packet always travels by CPI — a transaction itself carries at most
        // 1,232 bytes, so no full-size packet can be submitted top-level — and the runtime
        // rejects any CPI over the data limit. Counted here (allocating nothing) so an
        // undeliverable execution fails with a typed error instead of aborting the invoke.
        let packet_bytes = crate::execution::packet_byte_count(&args);
        if packet_bytes > crate::cost::CPI_INSTRUCTION_DATA_LIMIT {
            return Err(FheExecutionBuildError::ExceedsCpiInstructionDataLimit);
        }
        // Build, packet, and the invoke-side account tables are everything this crate will
        // request from the program heap for this execution — the tables are an exact function
        // of the account counts known here (`invoke_table_heap_bytes`). The build term alone is
        // already gated per step in `commit_step`; this is where the two finish-only terms can
        // first be charged. Over budget the instruction would abort with no error at all, so it
        // is rejected here where the app can still shrink the shape.
        let dynamic_accounts = self
            .remaining_accounts
            .iter()
            .filter(|meta| meta.requires_dynamic_account())
            .count();
        let output_authorities = 1 + self
            .remaining_accounts
            .iter()
            .filter(|meta| meta.requires_output_authority())
            .count();
        let invoke_heap_bytes = crate::heap_tally::invoke_table_heap_bytes(
            self.remaining_accounts.len(),
            dynamic_accounts,
            output_authorities,
        );
        if build_heap_bytes + packet_bytes + invoke_heap_bytes
            > crate::cost::BUILD_HEAP_BUDGET_BYTES
        {
            return Err(FheExecutionBuildError::ExceedsBuildHeapBudget);
        }
        let cost = crate::cost::FheExecutionCost {
            steps: args.steps.len(),
            persistent_creates: self.persistent_creates,
            persistent_updates: self.persistent_updates,
            emits_random_seeds_event: self.has_rand_step,
            emits_public_outputs_event: self.has_public_output,
            packet_bytes,
            build_heap_bytes,
            invoke_heap_bytes,
            remaining_accounts: self.remaining_accounts.len(),
            dynamic_accounts,
            output_authorities,
        };
        Ok(FheExecution {
            encrypted_value_account_authority: self.encrypted_value_account_authority,
            args,
            remaining_accounts: self.remaining_accounts.into_inner(),
            cost,
        })
    }
}
