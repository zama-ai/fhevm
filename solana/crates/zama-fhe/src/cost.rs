//! Cost bounds for one app-to-host execution. State accounts are created separately;
//! execution may grow them and top up rent, but never creates per-result accounts.
//! Public API surface: application transaction planners inspect these bounds before composing CPIs.
//! Host heap use still depends on live State/history and is measured by runtime tests.

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
/// validated byte-for-byte against a counting allocator in `heap_budget/` — and `finish`
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

/// Lazy block-meter creation can transfer, allocate and assign.
pub const CPIS_PER_SQUAT_CREATE: usize = 3;

/// One app instruction, one host CPI, and the emitted host event CPIs.
pub fn instruction_trace_floor(random_event: bool, public_event: bool) -> usize {
    2 + usize::from(random_event) + usize::from(public_event)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FheExecutionCost {
    pub steps: usize,
    /// Conservative bound on State rent top-ups; multiple outputs may share one State.
    pub state_outputs: usize,
    pub emits_random_seeds_event: bool,
    /// Whether the host will emit the public-outputs event CPI (any `make_public` output).
    pub emits_public_outputs_event: bool,
    /// Exact serialized `fhe_execute` instruction data, discriminator included — what the CPI
    /// carries and what [`CPI_INSTRUCTION_DATA_LIMIT`] bounds.
    pub packet_bytes: usize,
    /// Every byte the build requested from the program heap, tallied at each allocation the
    /// builder performs and validated byte-for-byte against a counting allocator
    /// (`heap_budget/`). With [`packet_bytes`](Self::packet_bytes) and
    /// [`invoke_heap_bytes`](Self::invoke_heap_bytes) on top this is what
    /// [`BUILD_HEAP_BUDGET_BYTES`] bounds.
    pub build_heap_bytes: usize,
    /// Heap the crate's invoke path requests after the build: `resolve_accounts`'s three
    /// exact-sized vectors plus the CPI account meta/info tables, an exact function of the
    /// account counts (`invoke_table_heap_bytes`), validated byte-for-byte against a counting
    /// allocator (`heap_budget/`).
    pub invoke_heap_bytes: usize,
    /// Dynamic accounts the invocation appends after the fixed `fhe_execute` account list.
    pub remaining_accounts: usize,
    /// Remaining accounts the app must supply as dynamic accounts to `resolve_accounts`.
    pub dynamic_accounts: usize,
    /// Value-authority witnesses the app must supply to `resolve_accounts` (the fixed
    /// execution authority plus each State’s own authority that differs from it).
    pub value_authorities: usize,
}

impl FheExecutionCost {
    pub fn instruction_trace_floor(&self) -> usize {
        instruction_trace_floor(
            self.emits_random_seeds_event,
            self.emits_public_outputs_event,
        )
    }

    /// Includes a possible rent transfer per State output and lazy meter creation.
    /// App-level State/transient store creation, final close and other CPIs must be added by the caller.
    pub fn instruction_trace_worst_case(&self) -> usize {
        self.instruction_trace_floor() + self.state_outputs + CPIS_PER_SQUAT_CREATE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cost(state_outputs: usize, random: bool, public: bool) -> FheExecutionCost {
        FheExecutionCost {
            steps: 1,
            state_outputs,
            emits_random_seeds_event: random,
            emits_public_outputs_event: public,
            packet_bytes: 0,
            build_heap_bytes: 0,
            invoke_heap_bytes: 0,
            remaining_accounts: 0,
            dynamic_accounts: 0,
            value_authorities: 0,
        }
    }

    #[test]
    fn trace_floor_counts_only_the_wrapper_and_emitted_event_kinds() {
        assert_eq!(instruction_trace_floor(false, false), 2);
        assert_eq!(instruction_trace_floor(true, false), 3);
        assert_eq!(instruction_trace_floor(false, true), 3);
        assert_eq!(instruction_trace_floor(true, true), 4);
    }

    #[test]
    fn worst_case_charges_each_state_output_and_one_lazy_meter_creation() {
        let cost = cost(3, true, true);
        assert_eq!(cost.instruction_trace_floor(), 4);
        assert_eq!(
            cost.instruction_trace_worst_case(),
            4 + 3 + CPIS_PER_SQUAT_CREATE
        );
    }
}
