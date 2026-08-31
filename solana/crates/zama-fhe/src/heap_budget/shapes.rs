//! Specimen constructors for the app-side frontier.

use anchor_lang::prelude::Pubkey;

use zama_host::{CoprocessorInputAttestation, MAX_FHE_EXECUTION_STEPS};

use crate::builder::FheExecutionBuilder;
use crate::{
    Domain, Encrypted, EncryptedValueId, EncryptedValueLabel,
    ExecutionEncryptedValueAccountAuthority, FheExecution, Output, PersistentOutput, Scalar, Uint,
    Uint64Handle,
};

use super::harness::balance_handle;

/// Whether a persist-heavy shape's outputs create their accounts or update existing ones.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PersistKind {
    Create,
    Update,
}

/// Pre-built app data for a persist-heavy shape: the persistent input the chain starts from and
/// one ready persistent output per persisting step, each with `subjects_per_output` distinct
/// subjects so every subject interns its own dictionary entry — the heaviest legal audience.
pub(crate) fn persist_shape_data(
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
pub(crate) fn shared_audience_update_shape(
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
/// create this is the dep-chain / load-smoke shape; with `MAX_PERSISTENT_CREATES` creates it is
/// the heaviest create-load the builder admits.
pub(crate) fn chain_with_outputs(
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
pub(crate) fn max_size_attestation(tag: u8) -> CoprocessorInputAttestation {
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
pub(crate) fn attestation_shape(
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
pub(crate) fn max_buildable_attestation_count() -> usize {
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

pub(crate) fn persist_shape(
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
pub(crate) fn shared_audience_public_creates_shape(
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

/// Which reduction op a reduction-heavy shape drives — the only caller-sized operand
/// tables, admitted through `reduction_operands`.
#[derive(Clone, Copy)]
pub(crate) enum ReductionKind {
    Sum,
    IsIn,
}

/// `steps` reduction steps of `operands` operands each, chained so each step consumes the
/// previous one's result. The operand tables are what this shape stresses: reserved from the
/// size hint and tallied per push.
pub(crate) fn reduction_shape(
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
pub(crate) fn mixed_ops_shape(
) -> impl for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> crate::Result<()> {
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
pub(crate) fn reused_attestation_shape(
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
