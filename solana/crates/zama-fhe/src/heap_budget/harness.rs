//! Counting allocator and build+packet measurement.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use anchor_lang::prelude::Pubkey;

use crate::builder::FheExecutionBuilder;
use crate::{ExecutionEncryptedValueAccountAuthority, FheExecution};

thread_local! {
    /// Bytes this thread has requested so far. Thread-local so other tests in this binary — and the
    /// harness's own allocations — cannot leak into the measurement, and const-initialized so the
    /// counter itself never allocates.
    static REQUESTED_BYTES: Cell<usize> = const { Cell::new(0) };
}

fn count(bytes: usize) {
    let _ = REQUESTED_BYTES.try_with(|counter| counter.set(counter.get() + bytes));
}

pub(crate) fn counted_bytes() -> usize {
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

/// Shapes of [`super::frontier_shapes`] the builder currently admits, asserted exactly by the
/// keystone: a shape silently dropping out (a structural regression grew the tally) or joining
/// (the budget widened) both fail until the change that moved the frontier updates this
/// number and the documented tables with it.
pub(crate) const ADMITTED_FRONTIER_SHAPES: usize = 49;

/// The most persistent creates one execution can carry: the builder rejects the create whose
/// three host CPIs push the transaction's instruction trace past its limit.
pub(crate) const MAX_BUILDABLE_CREATES: usize = 20;

/// A boxed shape constructor, so the frontier can hold shapes of different closure types.
pub(crate) type ShapeBuilder =
    Box<dyn for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()>>;

/// One row of the app-side shape matrix: a named buildable shape, the bytes it requested as a
/// counting allocator saw them, and the builder's own tally of the same build.
pub(crate) struct MeasuredShape {
    pub(crate) name: String,
    pub(crate) build_bytes: usize,
    pub(crate) packet_bytes: usize,
    pub(crate) cost: crate::cost::FheExecutionCost,
}

impl MeasuredShape {
    /// Everything the admission check charged: the measured build and packet plus the modeled
    /// invoke-side account tables.
    pub(crate) fn total(&self) -> usize {
        self.build_bytes + self.packet_bytes + self.cost.invoke_heap_bytes
    }
}

pub(crate) fn balance_handle(tag: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = 5;
    handle
}

/// Runs one build-plus-packet measurement, or reports the typed error the builder rejected the
/// shape with. Everything the closure captures is built by the caller before the measurement
/// starts: ids, subject lists, previous values, and attestations are the app's own data, not
/// what the builder allocates on the app's behalf.
pub(crate) fn try_measure<F>(
    name: String,
    build: F,
) -> std::result::Result<MeasuredShape, crate::FheExecutionBuildError>
where
    F: for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()>,
{
    let authority = Pubkey::new_unique();
    let before_build = counted_bytes();
    let mut execution = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(authority),
        build,
    )?;
    let build_bytes = counted_bytes() - before_build;
    let cost = execution.cost();

    // Exactly what `invoke_execution_signed_resolved` does with the built execution: stamp the
    // final account count in place and serialize the packet once into a right-sized buffer.
    let before_packet = counted_bytes();
    execution.args.account_count =
        u8::try_from(execution.remaining_accounts.len()).expect("account count");
    let data = crate::execution::fhe_execute_instruction_data(&execution.args);
    let packet_bytes = counted_bytes() - before_packet;
    assert!(!data.is_empty(), "the packet is what gets submitted");

    Ok(MeasuredShape {
        name,
        build_bytes,
        packet_bytes,
        cost,
    })
}
