//! Counting-allocator proofs and typed-ceiling pins.

use std::cell::Cell;

use anchor_lang::prelude::Pubkey;

use zama_host::MAX_FHE_EXECUTION_STEPS;

use crate::builder::FheExecutionBuilder;
use crate::cost::{instruction_trace_floor, TRANSACTION_INSTRUCTION_TRACE_LIMIT};
use crate::{
    Domain, Encrypted, EncryptedValueId, EncryptedValueLabel,
    ExecutionEncryptedValueAccountAuthority, FheExecution, Output, Scalar, Uint, Uint64Handle,
};

use super::frontier::frontier_shapes;
use super::harness::{
    balance_handle, counted_bytes, try_measure, ShapeBuilder, ADMITTED_FRONTIER_SHAPES,
    MAX_BUILDABLE_CREATES,
};
use super::shapes::*;

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

    // The reduction and subject-heavy shapes must still hit the budget. Max-size
    // attestations may fit after purpose lists moved off the heap; they still run so
    // embed admission stays probed, but they are not required to reject.
    assert!(
        rejections.get() >= 2,
        "the heavy per-step allocators must reach a heap rejection (got {})",
        rejections.get(),
    );
    assert!(probes.get() > 20, "the probes must actually run");
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
        let Ok(shape) = try_measure(name, build) else {
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
