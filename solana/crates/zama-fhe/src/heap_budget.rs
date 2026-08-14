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
//!    args in place, serializes the whole packet once into a right-sized buffer, resolves the
//!    dynamic accounts, and assembles the CPI account tables.
//!
//! Measuring only the build is how the first version of this test reported a budget the runtime
//! does not have: the build alone fits comfortably where the whole instruction does not. Every
//! phase is counted here, with a global allocator that models a never-freeing bump region —
//! every request tallied, every deallocation ignored — and `finish` charges all of it against
//! the budget: the build tally, the exact packet, and the invoke-side table model
//! (`invoke_table_heap_bytes`), each proven against the counting allocator by its own test
//! below. The only cost left to the [`crate::cost::APP_HEAP_RESERVE_BYTES`] reserve is what the
//! builder genuinely cannot see: Anchor's deserialization of the instruction's accounts before
//! any of this runs, and the app's own allocations. The at-cap dep-chain specimen
//! (`runtime-tests/tests/dep_chain_mollusk.rs`) exercises those for real under SBF at full
//! depth.
//!
//! What is measured is a *matrix of buildable shapes*, not one worst case, because the builder's
//! typed ceilings shape what can exist at all: the instruction-trace check caps persistent
//! creates at twenty, and the CPI packet check caps attestation-heavy executions well below the
//! step cap. Every shape the builder admits must fit — that is the claim the single step ceiling
//! rests on — and the fit test below asserts it for each row of the matrix.
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

/// Shapes of [`frontier_shapes`] the builder currently admits, asserted exactly by the
/// keystone: a shape silently dropping out (a structural regression grew the tally) or joining
/// (the budget widened) both fail until the change that moved the frontier updates this
/// number and the documented tables with it.
const ADMITTED_FRONTIER_SHAPES: usize = 43;

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
    /// Everything the admission check charged: the measured build and packet plus the modeled
    /// invoke-side account tables.
    fn total(&self) -> usize {
        self.build_bytes + self.packet_bytes + self.cost.invoke_heap_bytes
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

/// A full-depth all-update execution whose outputs share one audience. The shared subjects
/// intern once in the dictionary, so the build stays cheap — but every update ships its
/// previous state (handle plus the full subject list) inline in the packet, which is what
/// makes this the shape that outgrows the CPI data limit before it outgrows the heap budget.
fn shared_audience_update_shape(
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
    let audience: Vec<Pubkey> = (0..subjects_per_output)
        .map(|subject| {
            let mut key = [0u8; 32];
            key[0] = 0x70;
            key[1] = subject as u8 + 1;
            key[31] = 1;
            Pubkey::new_from_array(key)
        })
        .collect();
    let outputs = (0..outputs)
        .map(|index| {
            let id = EncryptedValueId::new(
                domain,
                authority,
                EncryptedValueLabel::new([index as u8; 32]),
            );
            let current = zama_host::EncryptedValue {
                domain: domain.pubkey(),
                encrypted_value_account_authority: authority,
                label: [index as u8; 32],
                current_handle: balance_handle(0xB0 + index as u8),
                subjects: audience.clone(),
                leaf_count: 0,
                peaks: Vec::new(),
                bump: 0,
            };
            PersistentOutput::update(id, audience.clone(), &current)
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

/// Every step consumes its own maximum-size verified input. The build-heap budget is what caps
/// this shape — each attestation embeds at over a kilobyte of handles, extra data, and
/// signature — so the heaviest buildable attestation count is found by asking the builder, not
/// assumed. The attestations are built before the measurement starts: they are the app's own
/// data; what the builder pays for is registering them and embedding one copy per consuming
/// step.
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
/// by a typed ceiling (today the build-heap budget), which the fit test asserts.
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

/// The invariant #61 counterexample shape: `creates` public outputs that all grant the same
/// eight-subject audience. The shared subjects intern once in the dictionary, so the app-side
/// ceilings price this shape like a narrow one — while the host materializes the audience per
/// created account in its own CPI frame.
fn shared_audience_public_creates_shape(
    creates: usize,
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let authority = Pubkey::new_unique();
    let domain = Domain::new(Pubkey::new_unique());
    let input = Uint64Handle::persistent(
        balance_handle(1),
        EncryptedValueId::new(domain, authority, EncryptedValueLabel::new([0xfd; 32])),
    )
    .expect("input handle");
    let audience: Vec<Pubkey> = (0..zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS)
        .map(|subject| Pubkey::new_from_array([0x60 + subject as u8; 32]))
        .collect();
    let outputs = (0..creates)
        .map(|index| {
            let id = EncryptedValueId::new(
                domain,
                authority,
                EncryptedValueLabel::new([index as u8; 32]),
            );
            PersistentOutput::create(id, audience.clone()).with_make_public(true)
        })
        .collect();
    chain_with_outputs(MAX_FHE_EXECUTION_STEPS, input, outputs)
}

/// Invariant #61's builder-side pin: `build()` admits the shared-audience eight-subject
/// `make_public` shape all the way to the trace-capped twenty creates — including the 16–20
/// band the host's own CPI frame cannot hold (the measured wall is 15, pinned by
/// `fhe_execute_boundary/subject_heavy_public_creates` in `runtime-tests`). If the app-side
/// model ever learns to reject this shape, this test fails and both #61 and the sweep's
/// documentation must move together; until then the 16–20 band is the documented gap between
/// `build`'s admission and the host's survival — fhevm-internal#1872.
#[test]
fn the_builder_admits_what_the_host_heap_cannot_hold() {
    for creates in [15, 16, MAX_BUILDABLE_CREATES] {
        FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
            shared_audience_public_creates_shape(creates),
        )
        .unwrap_or_else(|error| {
            panic!("{creates} shared-audience public creates should build: {error:?}")
        });
    }
    // One past the trace cap is where the app-side ceilings finally stop the shape — the gap
    // is exactly the 16–20 band, not open-ended.
    assert_eq!(
        FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
            shared_audience_public_creates_shape(MAX_BUILDABLE_CREATES + 1),
        )
        .unwrap_err(),
        crate::FheExecutionBuildError::ExceedsInstructionTraceLimit,
    );
}

/// The pre-emptive heap protocol, proven adversarially: drive the heaviest per-step
/// allocators into their typed rejection with a probe after every single call, and assert the
/// builder's tally never crosses the budget — not even transiently, not even on the rejected
/// call, not even when the caller ignores the rejection and keeps going. Before the protocol
/// admitted allocations up front, the per-step gate ran only *after* lowering had allocated,
/// so a near-budget build could request past the 32 KB region itself (eleven 8-operand set
/// memberships followed by one 60-operand sum requested 33.5 KB) and abort with no error —
/// the exact failure mode `build()` promises away.
#[test]
fn the_tally_never_crosses_the_budget_even_transiently() {
    fn probe(builder: &FheExecutionBuilder<'_>, probes: &Cell<usize>) {
        probes.set(probes.get() + 1);
        let tally = builder.requested_heap_bytes();
        assert!(
            tally <= crate::cost::BUILD_HEAP_BUDGET_BYTES,
            "the tally reached {tally} bytes, past the {} budget — an allocation site is \
             missing its headroom admission",
            crate::cost::BUILD_HEAP_BUDGET_BYTES,
        );
    }
    let probes = Cell::new(0usize);
    let rejections = Cell::new(0usize);

    // The review's counterexample: 8-operand set memberships to the brink, then a 60-operand
    // sum whose operand table alone must be refused before it is allocated.
    let authority = Pubkey::new_unique();
    let domain = Domain::new(Pubkey::new_unique());
    let input = Uint64Handle::persistent(
        balance_handle(3),
        EncryptedValueId::new(domain, authority, EncryptedValueLabel::new([0xfc; 32])),
    )
    .expect("input handle");
    let _ = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
        |builder| {
            let value = Encrypted::from(input);
            for _ in 0..MAX_FHE_EXECUTION_STEPS {
                let result = builder.is_in(value, (0..8).map(|_| value), Output::transient());
                probe(builder, &probes);
                if let Err(error) = result {
                    assert_eq!(error, crate::FheExecutionBuildError::ExceedsBuildHeapBudget);
                    rejections.set(rejections.get() + 1);
                    break;
                }
            }
            let oversized = builder.sum((0..60).map(|_| value), Output::transient());
            probe(builder, &probes);
            assert_eq!(
                oversized.unwrap_err(),
                crate::FheExecutionBuildError::ExceedsBuildHeapBudget,
            );
            Ok(())
        },
    );

    // Subject-heavy creates: every output interns eight fresh subjects, driving the
    // dictionary and account tables through their doublings — and the rejections are ignored,
    // as a buggy app would, so the ratchet past the first rejection is probed too.
    let (input, outputs) = persist_shape_data(PersistKind::Create, MAX_BUILDABLE_CREATES, 8);
    let _ = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
        |builder| {
            let mut value = Encrypted::from(input);
            for output in outputs {
                let result = builder.add(
                    value,
                    Scalar::<Uint<64>>::u64(1),
                    Output::persistent(output),
                );
                probe(builder, &probes);
                match result {
                    Ok(next) => value = next,
                    Err(error) => {
                        assert_eq!(error, crate::FheExecutionBuildError::ExceedsBuildHeapBudget);
                        rejections.set(rejections.get() + 1);
                    }
                }
            }
            Ok(())
        },
    );

    // Maximum-size attestations: the embeds go through the explicit counter rather than a
    // table, so this drives `tally_bytes`' admission into rejection.
    let _ = FheExecution::build(
        ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
        |builder| {
            for tag in 0..MAX_FHE_EXECUTION_STEPS {
                let attested =
                    builder.verified_input::<Uint<64>>(max_size_attestation(0x20 + tag as u8));
                probe(builder, &probes);
                let result = match attested {
                    Ok(attested_input) => builder
                        .add(
                            attested_input,
                            Scalar::<Uint<64>>::u64(1),
                            Output::transient(),
                        )
                        .map(|_| ()),
                    Err(error) => Err(error),
                };
                probe(builder, &probes);
                if let Err(error) = result {
                    assert_eq!(error, crate::FheExecutionBuildError::ExceedsBuildHeapBudget);
                    rejections.set(rejections.get() + 1);
                    break;
                }
            }
            Ok(())
        },
    );

    assert!(
        rejections.get() >= 3,
        "every adversarial shape must reach its heap rejection (got {})",
        rejections.get(),
    );
    assert!(probes.get() > 20, "the probes must actually run");
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
            "near_budget_creates (20 x 2)",
            full,
            persist_shape(PersistKind::Create, full, MAX_BUILDABLE_CREATES, 2),
        ),
        measure(
            "max_attestations (budget-capped)",
            attestation_count,
            attestation_shape(attestation_count),
        ),
    ]
}

/// The full app-side frontier, persist kind x output count x subject width, printed with the
/// typed rejection where the builder refuses the shape. This is the exploration companion to
/// the host-side boundary sweeps in `runtime-tests/tests/fhe_execute_boundary.rs`.
#[test]
#[ignore = "frontier grid, run explicitly with --nocapture"]
fn print_build_frontier_grid() {
    for (name, build) in frontier_shapes() {
        match try_measure(name.clone(), MAX_FHE_EXECUTION_STEPS, build) {
            Ok(shape) => println!(
                "{name:36} build={:6} packet={:6} invoke={:6} total={:6} fits",
                shape.build_bytes,
                shape.packet_bytes,
                shape.cost.invoke_heap_bytes,
                shape.total(),
            ),
            Err(error) => println!("{name:36} rejected: {error:?}"),
        }
    }
}

/// Which reduction op a reduction-heavy shape drives — the two ops whose operand tables carry
/// hand-written tallies in `builder.rs`, which is exactly why the frontier must exercise them.
#[derive(Clone, Copy)]
enum ReductionKind {
    Sum,
    IsIn,
}

/// `steps` reduction steps of `operands` operands each, chained so each step consumes the
/// previous one's result. The operand tables are what this shape stresses: reserved from the
/// size hint and tallied per push.
fn reduction_shape(
    kind: ReductionKind,
    steps: usize,
    operands: usize,
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let authority = Pubkey::new_unique();
    let domain = Domain::new(Pubkey::new_unique());
    let input = Uint64Handle::persistent(
        balance_handle(2),
        EncryptedValueId::new(domain, authority, EncryptedValueLabel::new([0xfd; 32])),
    )
    .expect("input handle");
    move |builder| {
        let mut value = Encrypted::from(input);
        for _ in 0..steps {
            match kind {
                ReductionKind::Sum => {
                    value = builder.sum((0..operands).map(|_| value), Output::transient())?;
                }
                ReductionKind::IsIn => {
                    builder.is_in(value, (0..operands).map(|_| value), Output::transient())?;
                }
            }
        }
        Ok(())
    }
}

/// A chain that mixes every tallied table in one build: adds, a mid-chain sum and set
/// membership, and one persistent create at the end.
fn mixed_ops_shape() -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let (input, outputs) = persist_shape_data(PersistKind::Create, 1, 2);
    move |builder| {
        let mut value = Encrypted::from(input);
        for step in 0..(MAX_FHE_EXECUTION_STEPS - 2) {
            value = if step % 4 == 3 {
                builder.sum((0..8).map(|_| value), Output::transient())?
            } else {
                builder.add(value, Scalar::<Uint<64>>::u64(1), Output::transient())?
            };
        }
        builder.is_in(value, (0..8).map(|_| value), Output::transient())?;
        let output = outputs.into_iter().next().expect("one create");
        builder.add(
            value,
            Scalar::<Uint<64>>::u64(1),
            Output::persistent(output),
        )?;
        Ok(())
    }
}

/// One maximum-size attestation consumed once, then reused by reference for the rest of the
/// chain — the embed is paid once, not per consuming step.
fn reused_attestation_shape(
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
    let attestation = max_size_attestation(0x8F);
    move |builder| {
        let mut value = builder.verified_input::<Uint<64>>(attestation)?;
        for _ in 0..(MAX_FHE_EXECUTION_STEPS - 1) {
            value = builder.add(value, Scalar::<Uint<64>>::u64(1), Output::transient())?;
        }
        Ok(())
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
    // The reduction ops carry the only hand-written operand-table tallies in `builder.rs`, so
    // the frontier exercises both, thin-and-deep and wide-and-shallow.
    for (kind, kind_name) in [(ReductionKind::Sum, "sum"), (ReductionKind::IsIn, "is_in")] {
        for (steps, operands) in [(1, 60), (2, 60), (8, 8), (MAX_FHE_EXECUTION_STEPS, 8)] {
            shapes.push((
                format!("{kind_name} x{steps:2} operands={operands}"),
                Box::new(reduction_shape(kind, steps, operands)),
            ));
        }
    }
    shapes.push((
        "mixed ops (add/sum/is_in, 1 create)".to_string(),
        Box::new(mixed_ops_shape()),
    ));
    shapes.push((
        "attestation reused across the chain".to_string(),
        Box::new(reused_attestation_shape()),
    ));
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
    }
    // Exact on purpose: a structural regression (a reintroduced per-step copy, a heavier
    // table) shows up as shapes silently dropping out of the admitted set, and a widened
    // budget as shapes silently joining it. Update the number together with the change that
    // deliberately moves the frontier.
    assert_eq!(
        admitted, ADMITTED_FRONTIER_SHAPES,
        "the admitted frontier moved — deliberate changes update ADMITTED_FRONTIER_SHAPES"
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
    // A full-depth all-update execution with wide audiences outgrows the budget during the
    // build itself — its dictionary doubles past the up-front reservation — so the per-step
    // gate rejects it at the step that crosses, before `finish` would also find its packet
    // oversized.
    assert_eq!(
        build(Box::new(persist_shape(
            PersistKind::Update,
            MAX_FHE_EXECUTION_STEPS,
            MAX_FHE_EXECUTION_STEPS,
            6,
        ))),
        crate::FheExecutionBuildError::ExceedsBuildHeapBudget,
    );
    // A full-depth all-update execution whose outputs share one wide audience keeps the build
    // cheap (the audience interns once) but ships every output's previous state inline — the
    // packet ceiling, first known at `finish`, is the one that fires.
    let (input, outputs) = shared_audience_update_shape(
        MAX_FHE_EXECUTION_STEPS,
        zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS,
    );
    assert_eq!(
        build(Box::new(chain_with_outputs(
            MAX_FHE_EXECUTION_STEPS,
            input,
            outputs,
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
        builder.requested_heap_bytes()
    );
    let (m0, t0) = (counted_bytes(), builder.requested_heap_bytes());
    let mut value = Encrypted::from(input);
    value = builder
        .add(value, Scalar::<Uint<64>>::u64(1), Output::transient())
        .unwrap();
    println!(
        "transient add: measured={} tally={}",
        counted_bytes() - m0,
        builder.requested_heap_bytes() - t0
    );
    let (m1, t1) = (counted_bytes(), builder.requested_heap_bytes());
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
        builder.requested_heap_bytes() - t1
    );
    let (m2, t2) = (counted_bytes(), builder.requested_heap_bytes());
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
            "{:40} steps={:2} build={:6} packet={:6} invoke={:6} total={:6} fits",
            shape.name,
            shape.steps,
            shape.build_bytes,
            shape.packet_bytes,
            shape.cost.invoke_heap_bytes,
            shape.total(),
        );
    }
}

/// The invoke-side model measured against the real code: for every admitted frontier shape,
/// `resolve_accounts` plus the CPI account-table assembly request exactly the bytes
/// [`crate::heap_tally::invoke_table_heap_bytes`] charged at `build()`. Runs under the `cpi` feature
/// (workspace builds unify it in); Anchor codegen changing its allocation pattern, a new
/// allocation in resolution, or a host account-list change all fail here.
#[cfg(feature = "cpi")]
#[test]
fn the_invoke_model_matches_a_counting_allocator_for_every_admitted_shape() {
    use anchor_lang::prelude::AccountInfo;

    fn arena_infos<'a>(
        keys: &'a [Pubkey],
        lamports: &'a mut [u64],
        data: &'a mut [Vec<u8>],
        owner: &'a Pubkey,
    ) -> Vec<AccountInfo<'a>> {
        keys.iter()
            .zip(lamports.iter_mut())
            .zip(data.iter_mut())
            .map(|((key, lamports), data)| {
                AccountInfo::new(
                    key,
                    false,
                    true,
                    lamports,
                    data.as_mut_slice(),
                    owner,
                    false,
                )
            })
            .collect()
    }

    let owner = Pubkey::new_unique();
    let mut checked = 0;
    for (name, build) in frontier_shapes() {
        let Ok(execution) = FheExecution::build(
            ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique()),
            build,
        ) else {
            continue;
        };
        checked += 1;
        let cost = execution.cost();

        // Every AccountInfo the invoke needs, allocated before the measurement window: the
        // accounts are the app's (Anchor deserialized them long before the build).
        let dynamic_keys: Vec<Pubkey> = execution
            .remaining_accounts
            .iter()
            .filter(|meta| meta.requires_dynamic_account())
            .map(|meta| meta.pubkey)
            .collect();
        let authority_keys: Vec<Pubkey> = execution.output_authorities().collect();
        let fixed_keys: Vec<Pubkey> = (0..crate::heap_tally::FHE_EXECUTE_FIXED_CPI_ACCOUNTS)
            .map(|_| Pubkey::new_unique())
            .collect();
        assert_eq!(dynamic_keys.len(), cost.dynamic_accounts, "{name}");
        assert_eq!(authority_keys.len(), cost.output_authorities, "{name}");
        let mut dynamic_lamports = vec![0u64; dynamic_keys.len()];
        let mut dynamic_data = vec![Vec::new(); dynamic_keys.len()];
        let dynamic_infos = arena_infos(
            &dynamic_keys,
            &mut dynamic_lamports,
            &mut dynamic_data,
            &owner,
        );
        let mut authority_lamports = vec![0u64; authority_keys.len()];
        let mut authority_data = vec![Vec::new(); authority_keys.len()];
        let authority_infos = arena_infos(
            &authority_keys,
            &mut authority_lamports,
            &mut authority_data,
            &owner,
        );
        let mut fixed_lamports = vec![0u64; fixed_keys.len()];
        let mut fixed_data = vec![Vec::new(); fixed_keys.len()];
        let fixed_infos = arena_infos(&fixed_keys, &mut fixed_lamports, &mut fixed_data, &owner);
        // Both optional HCU witnesses present — the maximum the model charges; an absent one
        // only shrinks the real cost. A host account-list change breaks this literal, which is
        // the pin behind `FHE_EXECUTE_FIXED_CPI_ACCOUNTS`.
        let fixed = zama_host::cpi::accounts::FheExecute {
            payer: fixed_infos[0].clone(),
            compute_subject: fixed_infos[1].clone(),
            encrypted_value_account_authority: fixed_infos[2].clone(),
            host_config: fixed_infos[3].clone(),
            system_program: fixed_infos[4].clone(),
            hcu_block_meter: Some(fixed_infos[5].clone()),
            hcu_trusted_app_record: Some(fixed_infos[6].clone()),
            event_authority: fixed_infos[7].clone(),
            program: fixed_infos[8].clone(),
        };

        let before = counted_bytes();
        let resolved = execution
            .resolve_accounts(
                dynamic_infos.iter().cloned(),
                authority_infos.iter().cloned(),
            )
            .unwrap_or_else(|error| panic!("{name}: resolves: {error:?}"));
        let tables = crate::cpi::fhe_execute_account_tables(&fixed, &execution, &resolved, &[])
            .expect("assembles");
        let measured = counted_bytes() - before;
        assert_eq!(
            measured, cost.invoke_heap_bytes,
            "{name}: the invoke-side model disagrees with the counting allocator — an \
             allocation is missing from the model (or modeled twice)",
        );
        drop(tables);
    }
    assert_eq!(
        checked, ADMITTED_FRONTIER_SHAPES,
        "the invoke measurement covered a different admitted set than the keystone"
    );
}
