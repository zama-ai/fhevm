//! Heap budget for building a batch on-chain.
//!
//! An app program lowers its batch on Anchor's default allocator: a bump allocator over a fixed
//! 32 KB region that never frees, so what decides whether a build fits is the *total* number of
//! bytes it requests, not its peak live set. This test counts those bytes with a global allocator
//! that models that behaviour — every request tallied, every deallocation ignored — while building
//! the most expensive batch the host accepts: `MAX_FHE_BATCH_OPS` steps that each also write a
//! persistent output, which is the most interning a legal batch can ask for.
//!
//! Counted on the host rather than under SBF because no program in this repo builds a batch anywhere
//! near the cap on-chain (the largest is five steps), so an SBF harness would need a fixture program
//! written only for this measurement, while the quantity that regresses — bytes requested per step —
//! is the same in both places.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use anchor_lang::prelude::Pubkey;
use zama_fhe::{
    Batch, BatchAppAuthority, Encrypted, EncryptedValueId, Output, PersistentLabel,
    PersistentOutput, Scalar, Uint, Uint64Handle,
};
use zama_host::MAX_FHE_BATCH_OPS;

thread_local! {
    /// Bytes this thread has requested so far. Thread-local so the test harness's own allocations
    /// on other threads cannot leak into the measurement, and const-initialized so the counter
    /// itself never allocates.
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

/// The region Anchor's default bump allocator serves one instruction from.
const DEFAULT_HEAP_BYTES: usize = 32 * 1024;

/// Steps an app program can count on lowering inside that region. Measured at 31 — the 32nd step's
/// `Vec` growth is what crosses the line — and asserted well below it so ordinary growth-pattern
/// noise does not fail the build. Kept high enough that the clone-per-step rollback this replaced,
/// which exhausted the heap at the 10th step and asked for 270 KB across the full batch, could not
/// pass this test.
const RELIABLE_STEPS: usize = 24;

/// A regression ceiling on the whole build, for the same reason.
const BUDGET_BYTES: usize = 48 * 1024;

fn balance_handle(tag: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = 5;
    handle
}

#[test]
fn building_the_largest_legal_batch_stays_within_the_heap_budget() {
    let authority = Pubkey::new_unique();
    let domain = Pubkey::new_unique();
    let input = Uint64Handle::persistent(
        balance_handle(1),
        EncryptedValueId::new(domain, authority, PersistentLabel::new([0xfe; 32])),
    )
    .expect("input handle");
    // Built before the measurement starts: the ids and subject lists are the app's own data, not
    // what the builder allocates on its behalf.
    let outputs: Vec<EncryptedValueId> = (0..MAX_FHE_BATCH_OPS)
        .map(|index| {
            EncryptedValueId::new(domain, authority, PersistentLabel::new([index as u8; 32]))
        })
        .collect();
    let subjects: Vec<Vec<Pubkey>> = (0..MAX_FHE_BATCH_OPS).map(|_| vec![authority]).collect();

    let before = counted_bytes();
    let mut steps_within_default_heap = 0;
    let batch = Batch::build(BatchAppAuthority::new(authority), |builder| {
        // The first step reads the persistent input; every later step chains on the previous step's
        // transient, which is what makes this the heaviest legal batch.
        let mut value = Encrypted::from(input);
        for (index, (output, subjects)) in outputs.into_iter().zip(subjects).enumerate() {
            value = builder.add(
                value,
                Scalar::<Uint<64>>::u64(index as u64),
                Output::persistent(PersistentOutput::create(output, subjects)),
            )?;
            if counted_bytes() - before <= DEFAULT_HEAP_BYTES {
                steps_within_default_heap = index + 1;
            }
        }
        Ok(())
    })
    .expect("batch builds");
    let requested = counted_bytes() - before;

    assert_eq!(
        batch.dynamic_account_requirements().len(),
        MAX_FHE_BATCH_OPS + 1,
        "the measured batch is the full-size one: one input ACL plus one output ACL per step"
    );
    assert!(
        steps_within_default_heap >= RELIABLE_STEPS,
        "only {steps_within_default_heap} steps lowered inside Anchor's {DEFAULT_HEAP_BYTES}-byte \
         default heap, fewer than the {RELIABLE_STEPS} an app program is told to expect"
    );
    assert!(
        requested < BUDGET_BYTES,
        "building the largest legal batch requested {requested} bytes, over the \
         {BUDGET_BYTES}-byte budget"
    );
}
