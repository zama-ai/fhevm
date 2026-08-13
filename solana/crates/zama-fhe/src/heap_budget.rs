//! Heap budget for building and invoking one `fhe_execute` instruction on-chain.
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
//! Measuring only the build is how the first version of this test reported a budget the runtime
//! does not have: the build alone fits comfortably where build plus packet does not. Both phases
//! are counted here, with a global allocator that models a never-freeing bump region — every
//! request tallied, every deallocation ignored.
//!
//! What is measured is a *matrix of buildable shapes*, not one worst case, because the builder's
//! typed ceilings shape what can exist at all: the instruction-trace check caps persistent
//! creates at twenty, and the CPI packet check caps attestation-heavy executions well below the
//! step cap. Every shape the builder admits must fit — that is the claim the single step ceiling
//! rests on — and the fit test below asserts it for each row of the matrix.
//!
//! Two smaller costs sit on top of the numbers this produces: `resolve_accounts`'s meta and info
//! vectors, and Anchor's own deserialization of the instruction's accounts before any of this
//! runs. They are why the fit asserted here keeps its multi-KB slack. The at-cap dep-chain
//! specimen (`runtime-tests/tests/dep_chain_mollusk.rs`) exercises them for real under SBF at
//! full depth.
//!
//! Counted on the host rather than under SBF because the quantity that regresses — bytes
//! requested per step — is the same in both places, and here it can be attributed to a phase.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use anchor_lang::prelude::Pubkey;

use zama_host::{CoprocessorInputAttestation, MAX_FHE_EXECUTION_STEPS};

use crate::builder::FheExecutionBuilder;
use crate::cost::{instruction_trace_floor, TRANSACTION_INSTRUCTION_TRACE_LIMIT};
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

/// A regression ceiling on any buildable execution's build plus packet, far above the measured
/// numbers so it only trips on a structural regression (a reintroduced per-step copy), not noise.
/// For scale, the clone-per-step rollback this replaced asked for 270 KB across a full execution
/// and exhausted the heap at the 10th step of the build alone.
const BUDGET_BYTES: usize = 64 * 1024;

/// The most persistent creates one execution can carry: the builder rejects the create whose
/// three host CPIs push the transaction's instruction trace past its limit.
const MAX_BUILDABLE_CREATES: usize = 20;

/// A boxed shape constructor, so the frontier can hold shapes of different closure types.
type ShapeBuilder = Box<dyn for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()>>;

/// One row of the app-side shape matrix: a named buildable shape, the bytes it requested as a
/// counting allocator saw them, and the builder's own tally of the same build.
struct MeasuredShape {
    name: String,
    steps: usize,
    build_bytes: usize,
    packet_bytes: usize,
    cost: crate::cost::FheExecutionCost,
}

impl MeasuredShape {
    fn total(&self) -> usize {
        self.build_bytes + self.packet_bytes
    }
}

fn balance_handle(tag: u8) -> [u8; 32] {
    let mut handle = [tag; 32];
    handle[30] = 5;
    handle
}

/// Runs one build-plus-packet measurement, or reports the typed error the builder rejected the
/// shape with. Everything the closure captures is built by the caller before the measurement
/// starts: ids, subject lists, previous values, and attestations are the app's own data, not
/// what the builder allocates on the app's behalf.
fn try_measure<F>(
    name: String,
    steps: usize,
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
        steps,
        build_bytes,
        packet_bytes,
        cost,
    })
}

fn measure<F>(name: &str, steps: usize, build: F) -> MeasuredShape
where
    F: for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()>,
{
    try_measure(name.to_string(), steps, build).expect("shape builds")
}

/// Whether a persist-heavy shape's outputs create their accounts or update existing ones.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PersistKind {
    Create,
    Update,
}

/// Pre-built app data for a persist-heavy shape: the persistent input the chain starts from and
/// one ready persistent output per persisting step, each with `subjects_per_output` distinct
/// subjects so every subject interns its own dictionary entry — the heaviest legal audience.
fn persist_shape_data(
    kind: PersistKind,
    outputs: usize,
    subjects_per_output: usize,
) -> (Uint64Handle, Vec<PersistentOutput>) {
    let authority = Pubkey::new_unique();
    let domain = Domain::new(Pubkey::new_unique());
    let input = Uint64Handle::persistent(
        balance_handle(1),
        EncryptedValueId::new(domain, authority, EncryptedValueLabel::new([0xfe; 32])),
    )
    .expect("input handle");
    let outputs = (0..outputs)
        .map(|index| {
            let id = EncryptedValueId::new(
                domain,
                authority,
                EncryptedValueLabel::new([index as u8; 32]),
            );
            let subjects: Vec<Pubkey> = (0..subjects_per_output)
                .map(|subject| {
                    let mut key = [0u8; 32];
                    key[0] = 0x40 + index as u8;
                    key[1] = subject as u8 + 1;
                    key[31] = 1;
                    Pubkey::new_from_array(key)
                })
                .collect();
            match kind {
                PersistKind::Create => PersistentOutput::create(id, subjects),
                PersistKind::Update => {
                    let current = zama_host::EncryptedValue {
                        domain: domain.pubkey(),
                        encrypted_value_account_authority: authority,
                        label: [index as u8; 32],
                        current_handle: balance_handle(0xB0 + index as u8),
                        subjects: subjects.clone(),
                        leaf_count: 0,
                        peaks: Vec::new(),
                        bump: 0,
                    };
                    PersistentOutput::update(id, subjects, &current)
                }
            }
        })
        .collect();
    (input, outputs)
}

/// The chain shape at full depth with the first `outputs.len()` steps writing persistent
/// outputs and the rest staying transient, the value threading through all of them. With one
/// create this is the dep-chain / load-smoke shape; with `MAX_BUILDABLE_CREATES` creates it is
/// the heaviest create-load the builder admits.
fn chain_with_outputs(
    steps: usize,
    input: Uint64Handle,
    outputs: Vec<PersistentOutput>,
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    move |builder| {
        let mut value = Encrypted::from(input);
        let mut outputs = outputs.into_iter();
        for _ in 0..steps {
            let output = match outputs.next() {
                Some(persistent) => Output::persistent(persistent),
                None => Output::transient(),
            };
            value = builder.add(value, Scalar::<Uint<64>>::u64(1), output)?;
        }
        Ok(())
    }
}

/// A maximum-size coprocessor attestation: full handle coverage, full extra data, one
/// signature. The signature bytes are arbitrary — the builder checks only the handle type;
/// verification happens on the host.
fn max_size_attestation(tag: u8) -> CoprocessorInputAttestation {
    let handles: Vec<[u8; 32]> = (0..zama_host::MAX_INPUT_ATTESTATION_HANDLES)
        .map(|position| {
            let mut handle = balance_handle(tag);
            handle[21] = position as u8;
            handle
        })
        .collect();
    CoprocessorInputAttestation {
        input_handle: handles[0],
        ct_handles: handles,
        handle_index: 0,
        user_address: [0xAA; 32],
        contract_address: [0xBB; 32],
        contract_chain_id: 1,
        extra_data: vec![0xCC; zama_host::MAX_INPUT_ATTESTATION_EXTRA_DATA],
        signatures: vec![[0xDD; 65]],
    }
}

/// Every step consumes its own maximum-size verified input. The CPI packet limit is what caps
/// this shape — each attestation serializes to roughly a kilobyte — so the heaviest buildable
/// attestation count is found by asking the builder, not assumed. The attestations are built
/// before the measurement starts: they are the app's own data; what the builder pays for is
/// registering them and embedding one copy per consuming step.
fn attestation_shape(
    count: usize,
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let attestations: Vec<CoprocessorInputAttestation> = (0..count)
        .map(|tag| max_size_attestation(0x90 + tag as u8))
        .collect();
    move |builder| {
        for attestation in attestations {
            let input = builder.verified_input::<Uint<64>>(attestation)?;
            builder.add(input, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        }
        Ok(())
    }
}

/// The largest attestation-per-step execution the builder admits — one past it must be rejected
/// by the packet check, which the fit test asserts.
fn max_buildable_attestation_count() -> usize {
    (1..=MAX_FHE_EXECUTION_STEPS)
        .take_while(|count| {
            FheExecution::build(
                ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
                attestation_shape(*count),
            )
            .is_ok()
        })
        .last()
        .expect("at least one attestation fits")
}

fn persist_shape(
    kind: PersistKind,
    steps: usize,
    outputs: usize,
    subjects: usize,
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let (input, outputs) = persist_shape_data(kind, outputs, subjects);
    chain_with_outputs(steps, input, outputs)
}

/// The app-side shape matrix: every named shape the fit test asserts and the table prints.
fn measured_shapes() -> Vec<MeasuredShape> {
    let full = MAX_FHE_EXECUTION_STEPS;
    let attestation_count = max_buildable_attestation_count();
    vec![
        measure(
            "small_typical (4 steps, 1 create)",
            4,
            persist_shape(PersistKind::Create, 4, 1, 1),
        ),
        measure(
            "full_chain (1 create)",
            full,
            persist_shape(PersistKind::Create, full, 1, 1),
        ),
        measure(
            "max_creates (20, 1 subject)",
            full,
            persist_shape(PersistKind::Create, full, MAX_BUILDABLE_CREATES, 1),
        ),
        measure(
            "near_budget_creates (20 x 4)",
            full,
            persist_shape(PersistKind::Create, full, MAX_BUILDABLE_CREATES, 4),
        ),
        measure(
            "max_attestations (packet-capped)",
            attestation_count,
            attestation_shape(attestation_count),
        ),
    ]
}

/// The full app-side frontier, persist kind x output count x subject width, printed with the
/// typed rejection where the builder refuses the shape. This is the exploration companion to
/// the host-side boundary sweeps in `runtime-tests/tests/host_mollusk.rs`.
#[test]
#[ignore = "frontier grid, run explicitly with --nocapture"]
fn print_build_frontier_grid() {
    for (kind, kind_name) in [
        (PersistKind::Create, "create"),
        (PersistKind::Update, "update"),
    ] {
        for subjects in [1, 2, 4, 6, 8] {
            for outputs in [4, 8, 12, 16, 20, 24, 28, MAX_FHE_EXECUTION_STEPS] {
                let name = format!("{kind_name} x{outputs:2} subjects={subjects}");
                match try_measure(
                    name.clone(),
                    MAX_FHE_EXECUTION_STEPS,
                    persist_shape(kind, MAX_FHE_EXECUTION_STEPS, outputs, subjects),
                ) {
                    Ok(shape) => println!(
                        "{name:28} build={:6} packet={:6} total={:6} {}",
                        shape.build_bytes,
                        shape.packet_bytes,
                        shape.total(),
                        if shape.total() + crate::cost::APP_HEAP_RESERVE_BYTES
                            <= crate::cost::PROGRAM_HEAP_BYTES
                        {
                            "fits"
                        } else {
                            "OVER"
                        }
                    ),
                    Err(error) => println!("{name:28} rejected: {error:?}"),
                }
            }
        }
    }
}

/// Every shape on the exploration frontier, admitted or not: persist kind x output count x
/// subject width, plus the attestation ladder.
fn frontier_shapes() -> Vec<(String, ShapeBuilder)> {
    let mut shapes: Vec<(String, ShapeBuilder)> = Vec::new();
    for (kind, kind_name) in [
        (PersistKind::Create, "create"),
        (PersistKind::Update, "update"),
    ] {
        for subjects in [1, 2, 4, 6, 8] {
            for outputs in [4, 8, 12, 16, 20, 24, 28, MAX_FHE_EXECUTION_STEPS] {
                shapes.push((
                    format!("{kind_name} x{outputs:2} subjects={subjects}"),
                    Box::new(persist_shape(
                        kind,
                        MAX_FHE_EXECUTION_STEPS,
                        outputs,
                        subjects,
                    )),
                ));
            }
        }
    }
    for count in 1..=MAX_FHE_EXECUTION_STEPS {
        shapes.push((
            format!("attestations x{count:2}"),
            Box::new(attestation_shape(count)),
        ));
    }
    shapes
}

/// The keystone of the build-heap budget: for every shape the builder admits, its own
/// allocation tally and packet count equal what a counting allocator measured, byte for byte.
/// A new allocation in the builder that forgets to tally fails here, which is what lets
/// `ExceedsBuildHeapBudget` fire exactly when a build cannot survive on-chain.
#[test]
fn the_heap_tally_matches_a_counting_allocator_for_every_admitted_shape() {
    // The ceilings that define "admitted" hold where this file assumes they do.
    assert_eq!(
        instruction_trace_floor(MAX_BUILDABLE_CREATES, true, true),
        TRANSACTION_INSTRUCTION_TRACE_LIMIT,
        "MAX_BUILDABLE_CREATES is stale against the trace model"
    );
    let mut admitted = 0;
    for (name, build) in frontier_shapes() {
        let Ok(shape) = try_measure(name, MAX_FHE_EXECUTION_STEPS, build) else {
            continue;
        };
        admitted += 1;
        assert_eq!(
            shape.cost.build_heap_bytes, shape.build_bytes,
            "{}: the builder's tally disagrees with the counting allocator — an allocation \
             is missing from the tally (or tallied twice)",
            shape.name,
        );
        assert_eq!(
            shape.cost.packet_bytes, shape.packet_bytes,
            "{}: the counted packet size disagrees with the bytes the packet requested",
            shape.name,
        );
        // What the budget check guarantees, asserted independently: an admitted shape fits
        // the program heap with the documented reserve to spare, and is nowhere near the
        // structural regression budget.
        let total = shape.total();
        assert!(
            total + crate::cost::APP_HEAP_RESERVE_BYTES <= crate::cost::PROGRAM_HEAP_BYTES,
            "{}: admitted at {total} bytes, over the build budget",
            shape.name,
        );
        assert!(
            total < BUDGET_BYTES,
            "{}: a per-step copy is back",
            shape.name
        );
    }
    assert!(
        admitted >= 50,
        "the frontier admitted only {admitted} shapes — the ceilings tightened unexpectedly"
    );
}

#[test]
fn the_shapes_past_each_ceiling_are_rejected_with_their_own_error() {
    let build = |shape: ShapeBuilder| {
        FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
            shape,
        )
        .unwrap_err()
    };
    // The 21st create's three host CPIs cannot fit any transaction's instruction trace.
    assert_eq!(
        build(Box::new(persist_shape(
            PersistKind::Create,
            MAX_FHE_EXECUTION_STEPS,
            MAX_BUILDABLE_CREATES + 1,
            1,
        ))),
        crate::FheExecutionBuildError::ExceedsInstructionTraceLimit,
    );
    // Twenty wide-audience creates build a packet that fits a CPI but a build that cannot
    // survive the program heap.
    assert_eq!(
        build(Box::new(persist_shape(
            PersistKind::Create,
            MAX_FHE_EXECUTION_STEPS,
            MAX_BUILDABLE_CREATES,
            zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS,
        ))),
        crate::FheExecutionBuildError::ExceedsBuildHeapBudget,
    );
    // A full-depth all-update execution with wide audiences serializes past what a CPI may
    // carry (updates ship their previous state inline).
    assert_eq!(
        build(Box::new(persist_shape(
            PersistKind::Update,
            MAX_FHE_EXECUTION_STEPS,
            MAX_FHE_EXECUTION_STEPS,
            6,
        ))),
        crate::FheExecutionBuildError::ExceedsCpiInstructionDataLimit,
    );
    // One attestation past the buildable maximum trips a typed admission error, not a runtime
    // abort.
    let count = max_buildable_attestation_count();
    let over = build(Box::new(attestation_shape(count + 1)));
    assert!(
        matches!(
            over,
            crate::FheExecutionBuildError::ExceedsBuildHeapBudget
                | crate::FheExecutionBuildError::ExceedsCpiInstructionDataLimit
        ),
        "attestations x{}: expected a typed admission error, got {over:?}",
        count + 1,
    );
}

#[test]
#[ignore = "debug breakdown"]
fn debug_tally_breakdown() {
    let authority = Pubkey::new_unique();
    let (input, outputs) = persist_shape_data(PersistKind::Create, 1, 1);
    let before = counted_bytes();
    let mut builder =
        FheExecutionBuilder::new(ExecutionEncryptedValueAccountAuthority::new(authority));
    println!(
        "new: measured={} tally={}",
        counted_bytes() - before,
        builder.requested_heap_bytes
    );
    let (m0, t0) = (counted_bytes(), builder.requested_heap_bytes);
    let mut value = Encrypted::from(input);
    value = builder
        .add(value, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    println!(
        "transient add: measured={} tally={}",
        counted_bytes() - m0,
        builder.requested_heap_bytes - t0
    );
    let (m1, t1) = (counted_bytes(), builder.requested_heap_bytes);
    let mut outputs = outputs.into_iter();
    builder
        .add(
            value,
            Scalar::<Uint<64>>::u64(1),
            Output::persistent(outputs.next().unwrap()),
        )
        .unwrap();
    println!(
        "create add: measured={} tally={}",
        counted_bytes() - m1,
        builder.requested_heap_bytes - t1
    );
    let (m2, t2) = (counted_bytes(), builder.requested_heap_bytes);
    let execution = builder.finish().unwrap();
    println!(
        "finish: measured={} tally={}",
        counted_bytes() - m2,
        execution.cost().build_heap_bytes - t2
    );
}

#[test]
#[ignore = "measurement table, run explicitly with --nocapture"]
fn print_measurement_table() {
    for shape in measured_shapes() {
        println!(
            "{:40} steps={:2} build={:6} packet={:6} total={:6} {}",
            shape.name,
            shape.steps,
            shape.build_bytes,
            shape.packet_bytes,
            shape.total(),
            if shape.total() + crate::cost::APP_HEAP_RESERVE_BYTES
                <= crate::cost::PROGRAM_HEAP_BYTES
            {
                "fits"
            } else {
                "OVER"
            }
        );
    }
}
