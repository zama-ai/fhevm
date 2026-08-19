//! What one `fhe_execute` invocation costs against the transaction ceilings it must fit.
//!
//! Public API surface: app programs. An app composing a transaction with more than the minimal
//! wrapper reads [`FheExecutionCost`]'s numbers — and the ceiling constants — to budget the
//! rest of the transaction around the execution.
//!
//! Two of Solana's per-transaction ceilings are exact functions of an execution's shape, so the
//! builder enforces them with typed errors instead of letting the transaction abort at runtime:
//!
//! - **Instruction trace.** A transaction may execute at most
//!   [`TRANSACTION_INSTRUCTION_TRACE_LIMIT`] instructions, top-level and CPI together. The host
//!   issues exactly three system-program CPIs per persistent output it creates (transfer,
//!   allocate, assign), plus one event CPI per event kind the execution emits. The floor below
//!   is what the transaction is guaranteed to spend even in the minimal production wrapper —
//!   one app instruction invoking `fhe_execute` once; an execution whose floor exceeds the
//!   limit cannot land in any transaction, and the builder rejects the step that crosses it
//!   with [`FheExecutionBuildError::ExceedsInstructionTraceLimit`].
//! - **CPI packet size.** The runtime rejects any CPI whose instruction data exceeds
//!   [`CPI_INSTRUCTION_DATA_LIMIT`] bytes, and an `fhe_execute` packet always travels by CPI —
//!   a transaction itself carries at most 1,232 bytes, so no full-size packet can be submitted
//!   top-level. `finish` counts the exact packet and rejects an oversized one with
//!   [`FheExecutionBuildError::ExceedsCpiInstructionDataLimit`].
//!
//! What is deliberately *not* enforced here: costs that depend on on-chain state rather than
//! the execution's shape. A persistent update may add one rent top-up transfer if the account
//! must grow, the host lazily creates its per-app block meter once under a finite block cap
//! (three CPIs, first execution only), and the host's heap consumption for updates grows with
//! the stored value's MMR peak count — none of which the builder can see. Those are measured
//! per shape by the boundary sweeps in `runtime-tests` and surface through
//! [`FheExecutionCost::instruction_trace_worst_case`] and the cost-snapshot data instead.
//!
//! One shape-dependent cost also stays measured rather than typed: the host's own **CPI frame**.
//! The host CPI runs in a fresh 32 KB heap region of its own, and what it allocates
//! there scales with created outputs times subjects per output — each created account's
//! subject table, plus the public-outputs event payload for `make_public` creates. The builder
//! cannot price that from its side of the CPI boundary: its packet cost interns a shared
//! audience once in the dictionary, while the host materializes it per created account. The
//! `subject_heavy_public_creates` boundary sweep in `runtime-tests` pins the measured wall
//! (15 eight-subject public creates land, 16 abort in the host's CPI frame) where the
//! builder's typed ceilings alone would admit 20.
//!
//! Both ceilings are runtime facts of the pinned agave 4.x toolchain, asserted against
//! measurement by `runtime-tests/tests/fhe_execute_boundary.rs` (`cost_snapshot_solana_ceilings`
//! and the boundary sweeps).

/// Instructions one transaction may execute, top-level and CPI together — agave's
/// `MAX_INSTRUCTION_TRACE_LENGTH` (`solana-transaction-context`); exceeding it aborts the
/// transaction with `MaxInstructionTraceLengthExceeded`. Not extendable.
pub const TRANSACTION_INSTRUCTION_TRACE_LIMIT: usize = 64;

/// The heap region the SBF entrypoint's default bump allocator serves one program invocation
/// from. Fixed: the allocator's region length is a compile-time constant, so a granted
/// `RequestHeapFrame` is not usable and the region never frees (DD-046).
pub const PROGRAM_HEAP_BYTES: usize = 32 * 1024;

/// Heap bytes the builder leaves untouched for what it genuinely cannot see: Anchor's account
/// deserialization before the app's instruction body runs, and the app's own allocations. The
/// at-cap dep-chain specimen (`runtime-tests/tests/dep_chain_mollusk.rs`) exercises those real
/// costs under SBF at full chain depth. An app that allocates more than this reserve in the
/// same instruction must stay correspondingly further below [`BUILD_HEAP_BUDGET_BYTES`].
pub const APP_HEAP_RESERVE_BYTES: usize = 8 * 1024;

/// What one build, its serialized packet, and the invoke-side account tables together may
/// request from the program heap: the builder tallies every byte it asks the allocator for —
/// validated byte-for-byte against a counting allocator in `heap_budget.rs` — and `finish`
/// rejects an execution over this budget with
/// [`FheExecutionBuildError::ExceedsBuildHeapBudget`](crate::FheExecutionBuildError::ExceedsBuildHeapBudget),
/// because on the never-freeing bump region an over-budget build aborts the instruction with
/// no error at all.
///
/// The tallied bytes are what the code *requests*; the entrypoint's bump allocator additionally
/// consumes up to `align - 1` padding bytes per allocation plus its own position word, a
/// drift of at most a few hundred bytes on the widest shapes, absorbed by the reserve.
pub const BUILD_HEAP_BUDGET_BYTES: usize = PROGRAM_HEAP_BYTES - APP_HEAP_RESERVE_BYTES;

/// Bytes of instruction data one CPI may carry — agave's `MAX_INSTRUCTION_DATA_LEN`, checked on
/// every invoke (`solana-program-runtime`'s `check_instruction_size`). Not extendable.
pub const CPI_INSTRUCTION_DATA_LIMIT: usize = 10 * 1024;

/// System-program CPIs the host issues to create one persistent output account: transfer,
/// allocate, assign (`create_pda_strict`). Two suffice when the address is already funded to
/// rent exemption, so counting three never under-counts.
pub const CPIS_PER_PERSISTENT_CREATE: usize = 3;

/// The instructions a transaction is guaranteed to execute for one `fhe_execute` invocation in
/// the minimal production wrapper: the app instruction, its `fhe_execute` CPI, three
/// system-program CPIs per created persistent output, and one event CPI per event kind emitted
/// (random seeds, public outputs). Everything an app adds on top — its own CPIs, other
/// instructions in the transaction — comes out of what the limit leaves over this floor.
pub fn instruction_trace_floor(
    persistent_creates: usize,
    emits_random_seeds_event: bool,
    emits_public_outputs_event: bool,
) -> usize {
    2 + CPIS_PER_PERSISTENT_CREATE * persistent_creates
        + usize::from(emits_random_seeds_event)
        + usize::from(emits_public_outputs_event)
}

/// Shape-derived cost of one built execution, computed by `finish` and carried on
/// [`FheExecution`](crate::FheExecution) so an app composing a larger transaction can budget
/// against the ceilings before submitting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FheExecutionCost {
    /// Steps in the execution.
    pub steps: usize,
    /// Persistent outputs this execution creates (three system CPIs each).
    pub persistent_creates: usize,
    /// Persistent outputs this execution updates (no CPI, unless the account needs a rent
    /// top-up to grow — see [`instruction_trace_worst_case`](Self::instruction_trace_worst_case)).
    pub persistent_updates: usize,
    /// Whether the host will emit the random-seeds event CPI (any rand step).
    pub emits_random_seeds_event: bool,
    /// Whether the host will emit the public-outputs event CPI (any `make_public` output).
    pub emits_public_outputs_event: bool,
    /// Exact serialized `fhe_execute` instruction data, discriminator included — what the CPI
    /// carries and what [`CPI_INSTRUCTION_DATA_LIMIT`] bounds.
    pub packet_bytes: usize,
    /// Every byte the build requested from the program heap, tallied at each allocation the
    /// builder performs and validated byte-for-byte against a counting allocator
    /// (`heap_budget.rs`). With [`packet_bytes`](Self::packet_bytes) and
    /// [`invoke_heap_bytes`](Self::invoke_heap_bytes) on top this is what
    /// [`BUILD_HEAP_BUDGET_BYTES`] bounds.
    pub build_heap_bytes: usize,
    /// Heap the crate's invoke path requests after the build: `resolve_accounts`'s three
    /// exact-sized vectors plus the CPI account meta/info tables, an exact function of the
    /// account counts (`invoke_table_heap_bytes`), validated byte-for-byte against a counting
    /// allocator (`heap_budget.rs`).
    pub invoke_heap_bytes: usize,
    /// Dynamic accounts the invocation appends after the fixed `fhe_execute` account list.
    pub remaining_accounts: usize,
    /// Remaining accounts the app must supply as dynamic accounts to `resolve_accounts`.
    pub dynamic_accounts: usize,
    /// Output-authority witnesses the app must supply to `resolve_accounts` (the fixed
    /// execution authority plus each per-output authority).
    pub output_authorities: usize,
}

impl FheExecutionCost {
    /// See [`instruction_trace_floor`].
    pub fn instruction_trace_floor(&self) -> usize {
        instruction_trace_floor(
            self.persistent_creates,
            self.emits_random_seeds_event,
            self.emits_public_outputs_event,
        )
    }

    /// The floor plus every state-dependent instruction this execution *can* add: one rent
    /// top-up transfer per persistent update whose account must grow, and the three CPIs that
    /// lazily create the per-app block meter on an app's first execution under a finite block
    /// cap. A transaction budgeted against this number lands regardless of on-chain state.
    pub fn instruction_trace_worst_case(&self) -> usize {
        self.instruction_trace_floor() + self.persistent_updates + CPIS_PER_PERSISTENT_CREATE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_floor_counts_the_wrapper_the_creates_and_the_events() {
        // The minimal wrapper alone: app instruction + fhe_execute CPI.
        assert_eq!(instruction_trace_floor(0, false, false), 2);
        // Each create is three system CPIs; each event kind is one CPI.
        assert_eq!(instruction_trace_floor(1, false, false), 5);
        assert_eq!(instruction_trace_floor(20, true, true), 64);
        // The first create past twenty cannot land in any transaction.
        assert_eq!(instruction_trace_floor(21, false, false), 65);
    }

    #[test]
    fn the_worst_case_adds_the_state_dependent_instructions() {
        let cost = FheExecutionCost {
            steps: 8,
            persistent_creates: 2,
            persistent_updates: 3,
            emits_random_seeds_event: true,
            emits_public_outputs_event: false,
            packet_bytes: 0,
            build_heap_bytes: 0,
            invoke_heap_bytes: 0,
            remaining_accounts: 0,
            dynamic_accounts: 0,
            output_authorities: 0,
        };
        // The floor: wrapper (2) + three CPIs per create (6) + the rand event (1).
        assert_eq!(cost.instruction_trace_floor(), 9);
        // The worst case adds one rent top-up transfer per update and the one-time three-CPI
        // block-meter creation.
        assert_eq!(
            cost.instruction_trace_worst_case(),
            9 + cost.persistent_updates + CPIS_PER_PERSISTENT_CREATE
        );
    }
}
