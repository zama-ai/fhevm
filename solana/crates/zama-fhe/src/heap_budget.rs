//! Heap budget for one on-chain `fhe_execute` instruction.
//!
//! An app program pays for its execution on the SBF entrypoint's default allocator
//! (`solana-program-entrypoint`, not Anchor): a bump allocator over a fixed 32 KB region that
//! never frees, so what decides whether an instruction fits is the *total* number of bytes it
//! requests, not its peak live set. Two phases share that one region and neither gives
//! anything back:
//!
//! 1. **Building** the execution — lowering interns into the builder's own tables.
//! 2. **Invoking** it — `invoke_execution_signed_resolved` stamps the final account count into the
//!    args in place and serializes the whole packet once, into a right-sized buffer.
//!
//! Measuring only the build is how the first version of this test reported a budget the runtime does
//! not have: at 24 steps the build alone fits comfortably and the packet that follows it does not.
//! Both phases are counted here, with a global allocator that models a never-freeing bump region —
//! every request tallied, every deallocation ignored.
//!
//! Two smaller costs sit on top of the number this produces: `resolve_accounts`'s meta and info
//! vectors, and Anchor's own deserialization of the instruction's accounts before any of this
//! runs. They are why the fit asserted here keeps its ~8 KB of slack, and the at-cap dep-chain
//! specimen (`runtime-tests/tests/dep_chain_mollusk.rs`) is what exercises them for real under SBF.
//!
//! Counted on the host rather than under SBF because the quantity that regresses — bytes requested
//! per step — is the same in both places, and here it can be attributed to a phase.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use anchor_lang::prelude::Pubkey;

use zama_host::MAX_FHE_EXECUTION_STEPS;

use crate::{
    Domain, Encrypted, EncryptedValueId, EncryptedValueLabel,
    ExecutionEncryptedValueAccountAuthority, FheExecution, Output, PersistentOutput, Scalar, Uint,
    Uint64Handle,
};

thread_local! {
    /// Bytes this thread has requested so far. Thread-local so other tests in this binary — and the
    /// harness's own allocations — cannot leak into the measurement, and const-initialized so the
    /// counter itself never allocates.
    static REQUESTED_BYTES: Cell<usize> = const { Cell::new(0) };
}

fn count(bytes: usize) {
    let _ = REQUESTED_BYTES.try_with(|counter| counter.set(counter.get() + bytes));
}

fn counted_bytes() -> usize {
    REQUESTED_BYTES.with(Cell::get)
}

/// Delegates to the system allocator and tallies every request the way a bump allocator that never
/// reuses freed memory experiences them. Growing a `Vec` is a fresh allocation there, so `realloc`
/// counts its new size in full.
struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        count(new_size);
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// The region the entrypoint's default bump allocator serves one instruction from.
const DEFAULT_HEAP_BYTES: usize = 32 * 1024;

/// A regression ceiling on a full-size execution's build plus packet, far above the measured
/// number so it only trips on a structural regression (a reintroduced per-step copy), not noise.
/// For scale, the clone-per-step rollback this replaced asked for 270 KB across a full execution
/// and exhausted the heap at the 10th step of the build alone.
const BUDGET_BYTES: usize = 64 * 1024;

fn balance_handle(tag: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = 5;
    handle
}

/// Builds the heaviest legal execution of `steps` steps — every step also writes a persistent output,
/// which is the most interning an execution can ask for — and returns the bytes its build requested and
/// the bytes its instruction packet requested.
fn measure(steps: usize) -> (usize, usize) {
    let authority = Pubkey::new_unique();
    let domain = Domain::new(Pubkey::new_unique());
    let input = Uint64Handle::persistent(
        balance_handle(1),
        EncryptedValueId::new(domain, authority, EncryptedValueLabel::new([0xfe; 32])),
    )
    .expect("input handle");
    // Built before the measurement starts: the ids and subject lists are the app's own data, not
    // what the builder allocates on its behalf.
    let outputs: Vec<EncryptedValueId> = (0..steps)
        .map(|index| {
            EncryptedValueId::new(
                domain,
                authority,
                EncryptedValueLabel::new([index as u8; 32]),
            )
        })
        .collect();
    let subjects: Vec<Vec<Pubkey>> = (0..steps).map(|_| vec![authority]).collect();

    let before_build = counted_bytes();
    let mut execution = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(authority),
        |builder| {
            // The first step reads the persistent input; every later step chains on the previous step's
            // transient, which is what makes this the heaviest legal execution.
            let mut value = Encrypted::from(input);
            for (output, subjects) in outputs.into_iter().zip(subjects) {
                value = builder.add(
                    value,
                    Scalar::<Uint<64>>::u64(1),
                    Output::persistent(PersistentOutput::create(output, subjects)),
                )?;
            }
            Ok(())
        },
    )
    .expect("execution builds");
    let build_bytes = counted_bytes() - before_build;

    // Exactly what `invoke_execution_signed_resolved` does with the built execution: stamp the
    // final account count in place and serialize the packet once into a right-sized buffer.
    let before_packet = counted_bytes();
    execution.args.account_count =
        u8::try_from(execution.remaining_accounts.len()).expect("account count");
    let data = crate::execution::fhe_execute_instruction_data(&execution.args);
    let packet_bytes = counted_bytes() - before_packet;
    assert!(!data.is_empty(), "the packet is what gets submitted");

    (build_bytes, packet_bytes)
}

#[test]
fn the_largest_legal_execution_fits_the_default_heap() {
    let (build_bytes, packet_bytes) = measure(MAX_FHE_EXECUTION_STEPS);
    let total = build_bytes + packet_bytes;
    assert!(
        total < BUDGET_BYTES,
        "a full {MAX_FHE_EXECUTION_STEPS}-step execution requested {total} bytes ({build_bytes} building, \
         {packet_bytes} for the packet), over the {BUDGET_BYTES}-byte budget — a per-step copy is back"
    );
    // The claim the single step ceiling rests on: since the builder reserves its tables up front
    // (no doubling strands) and the invoke path stopped cloning, a full-size execution requests
    // ~24.6 KB of the 32 KB region. The ~8 KB left is what this measurement cannot count — account
    // resolution and Anchor's own account deserialization — which the at-cap specimen exercises
    // for real under SBF. If this fails, either win the bytes back or reintroduce a smaller
    // on-chain budget (builder docs and INVARIANTS #54 are the claims to update with it).
    assert!(
        total <= DEFAULT_HEAP_BYTES,
        "the full-size execution no longer fits the default heap ({total} bytes)"
    );
}

#[test]
#[ignore = "measurement table, run explicitly with --nocapture"]
fn print_measurement_table() {
    for steps in [4, 8, 12, 16, 20, 24, 28, MAX_FHE_EXECUTION_STEPS] {
        let (build, packet) = measure(steps);
        println!(
            "steps={steps:2} build={build:6} packet={packet:6} total={:6} {}",
            build + packet,
            if build + packet <= DEFAULT_HEAP_BYTES {
                "fits"
            } else {
                "OVER"
            }
        );
    }
}
