//! The `fhe_execute` capacity instrument: where each execution shape stops, and why.
//!
//! One shape table drives everything here — the boundary sweeps asserted against the cost
//! snapshot (`cost-snapshots/fhe_execute_boundary.json`), the per-shape wall pins that tie the
//! builder's model to the measured walls, and the `#[ignore]`d full-curve printer. Behavior
//! tests for `fhe_execute` live in `host_mollusk.rs`; this binary only measures capacity.
//!
//! A sweep probes one shape from its minimum legal step count to the host's step cap, asserts
//! success is a prefix of the range (so step count really is what governs the shape), and
//! records the largest passing count, the first failing one, and the wall it hit. The snapshot
//! pins all three; regenerate with `ZAMA_UPDATE_COST_SNAPSHOT=1` or
//! `bash scripts/update-cost-snapshots.sh`.

use anchor_lang::prelude::system_program;
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey};
use std::collections::BTreeMap;
use zama_host::encode::ExecutionDictionary;
use zama_host::{
    self as host, FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep, PreviousState,
};
use zama_solana_test_kit::{
    cost_snapshot, empty_system_account, encrypted_value_account, event_authority,
    funded_system_account, handle_for_chain, host_svm as mollusk, label,
    new_encrypted_value as new_encrypted_value_account, readonly, signing, system_program_account,
    u256_be, writable, HostConfigParams,
};

mod host_fixtures;
use host_fixtures::{
    fhe_execute_ix, host_config_account, persistent_creates_batch, CreatedPublicBatch,
};

/// Names the wall a failing probe hit, so a boundary sweep can never silently converge on the
/// wrong limit. `heap` is the SBF abort (`ProgramFailedToComplete`), which also covers panics:
/// Mollusk's `InstructionResult` carries no program logs, so the allocator's "memory allocation
/// failed" line cannot be read back here to separate them. A probe asserting `heap` must
/// therefore use a shape known to be panic-free at smaller sizes — which the sweep's monotonic
/// prefix requirement enforces in practice, since a panicking fixture fails at every size, not
/// past a boundary. Program errors (`other:*`) are never a capacity wall; `sweep_step_boundary`
/// panics on them instead of recording them.
fn failure_axis(result: &mollusk_svm::result::InstructionResult) -> String {
    use solana_sdk::instruction::InstructionError;
    match &result.raw_result {
        Ok(()) => "none".to_string(),
        Err(InstructionError::ProgramFailedToComplete) => "heap".to_string(),
        Err(InstructionError::ComputationalBudgetExceeded) => "compute_units".to_string(),
        Err(InstructionError::MaxInstructionTraceLengthExceeded) => "instruction_trace".to_string(),
        Err(other) => format!("other:{other:?}"),
    }
}

/// One probe of a boundary sweep: a complete instruction plus the accounts it runs against.
struct ProbeCase {
    instruction: Instruction,
    accounts: Vec<(Pubkey, Account)>,
}

impl From<CreatedPublicBatch> for ProbeCase {
    fn from(case: CreatedPublicBatch) -> Self {
        ProbeCase {
            instruction: case.instruction,
            accounts: case.accounts,
        }
    }
}

/// The result of sweeping one shape: the largest passing step count, the first failing one, the
/// wall that failure hit, and the passing run's evidence for the snapshot.
struct SweptBoundary {
    max_ok: usize,
    first_fail: usize,
    limited_by: String,
    instruction_at_max_ok: Instruction,
    result_at_max_ok: mollusk_svm::result::InstructionResult,
}

/// Sweeps a shape from `min_steps` to the host's step cap. The sweep is linear rather than a
/// binary search so it can assert the boundary premise itself: success must be a prefix of the
/// range. A success after a failure would mean step count is not what governs this shape, and a
/// recorded boundary would be meaningless.
fn sweep_step_boundary(min_steps: usize, build: impl Fn(usize) -> ProbeCase) -> SweptBoundary {
    let mut last_ok: Option<(usize, Instruction, mollusk_svm::result::InstructionResult)> = None;
    let mut first_failure: Option<(usize, String)> = None;
    for steps in min_steps..=host::MAX_FHE_EXECUTION_STEPS {
        let case = build(steps);
        let result = mollusk().process_instruction(&case.instruction, &case.accounts);
        let axis = failure_axis(&result);
        assert!(
            !axis.starts_with("other:"),
            "probe at {steps} steps failed on a program error, not a capacity wall: {axis} — \
             fix the fixture instead of recording a boundary"
        );
        if axis == "none" {
            if let Some((failed_steps, failed_axis)) = &first_failure {
                panic!(
                    "non-monotonic boundary: {steps} steps succeeded after {failed_steps} \
                     failed on {failed_axis}; step count does not govern this shape"
                );
            }
            last_ok = Some((steps, case.instruction, result));
        } else if first_failure.is_none() {
            first_failure = Some((steps, axis));
        }
    }
    let (max_ok, instruction_at_max_ok, result_at_max_ok) =
        last_ok.expect("the smallest probe of the sweep must pass");
    let (first_fail, limited_by) = first_failure.unwrap_or((
        // Every legal size passes: the policy cap is the wall, and no runtime wall was
        // observed below it.
        host::MAX_FHE_EXECUTION_STEPS + 1,
        "MAX_FHE_EXECUTION_STEPS".to_string(),
    ));
    SweptBoundary {
        max_ok,
        first_fail,
        limited_by,
        instruction_at_max_ok,
        result_at_max_ok,
    }
}

fn assert_swept_boundary(profile: &str, sweep: &SweptBoundary) {
    cost_snapshot::assert_boundary_snapshot(
        "fhe_execute_boundary",
        profile,
        &cost_snapshot::Boundary {
            max_ok: sweep.max_ok as u64,
            first_fail: sweep.first_fail as u64,
            limited_by: sweep.limited_by.clone(),
        },
        &sweep.instruction_at_max_ok,
        &sweep.result_at_max_ok,
    );
}

/// A dependent transient chain closing in one persistent create: every step adds a scalar to the
/// previous step's transient result. This is the transient-heavy shape `dep-chain` and the
/// load-smoke scenario drive, and the lightest per-step shape in the matrix.
fn dependent_chain_case(steps: usize, authority: Pubkey) -> ProbeCase {
    assert!(
        steps >= 2,
        "the chain shape needs a first read and a final persist"
    );
    let (host_config, host_config_account) = host_config_account(authority);
    let balance_handle = handle_for_chain(0x61, 5);
    let (balance_address, balance_value) = new_encrypted_value_account(
        authority,
        authority,
        label("boundary-chain-balance"),
        balance_handle,
        &[authority],
    );
    let output_label = label("boundary-chain-output");
    let output_id = zama_solana_acl::derive_encrypted_value_id(
        authority.to_bytes(),
        authority.to_bytes(),
        output_label,
    );
    let output_address = host::encrypted_value_address(output_id).0;

    let mut dictionary = ExecutionDictionary::default();
    let one = dictionary.intern(u256_be(1));
    let balance_handle_index = dictionary.intern(balance_handle);
    let mut chain = Vec::with_capacity(steps);
    chain.push(FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::StoredValue {
            handle_index: balance_handle_index,
            encrypted_value_index: 0,
        },
        rhs: FheExecuteOperand::Scalar { value_index: one },
        output_fhe_type: 5,
        output: FheExecuteOutput::Transient,
    });
    for index in 1..steps {
        let output = if index == steps - 1 {
            FheExecuteOutput::StoredValue {
                output_encrypted_value_index: 1,
                output_authority_index: None,
                output_domain_index: dictionary.intern_key(authority),
                output_account_index: dictionary.intern_key(authority),
                output_label_index: dictionary.intern(output_label),
                output_subject_indexes: dictionary.intern_subjects([authority]),
                previous_state: None,
                make_public: false,
            }
        } else {
            FheExecuteOutput::Transient
        };
        chain.push(FheExecuteStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheExecuteOperand::EarlierStep {
                producer_index: (index - 1) as u8,
            },
            rhs: FheExecuteOperand::Scalar { value_index: one },
            output_fhe_type: 5,
            output,
        });
    }
    let instruction = fhe_execute_ix(
        authority,
        authority,
        authority,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps: chain,
        },
        vec![readonly(balance_address), writable(output_address)],
    );
    ProbeCase {
        instruction,
        accounts: vec![
            (system_program::ID, system_program_account()),
            (authority, funded_system_account()),
            (host_config, host_config_account),
            (event_authority(host::id()), Account::default()),
            (balance_address, encrypted_value_account(&balance_value)),
            (output_address, empty_system_account()),
        ],
    }
}

/// Every step updates its own mature `EncryptedValue` carrying `peak_count` MMR peaks and the
/// full subject list, so each account decode allocates what the given maturity forces. The leaf
/// count keeps eight trailing zero bits so the eight seal-appends per account stay cheap — the
/// shape stresses decode allocation, not MMR merge hashing. Peak count is `popcount(leaf_count)`
/// — pure on-chain state the builder cannot see, which is why this axis is swept per maturity
/// instead of enforced at build time.
fn mature_updates_case(steps: usize, peak_count: u32, authority: Pubkey) -> ProbeCase {
    let (host_config, host_config_account) = host_config_account(authority);
    let subjects: Vec<Pubkey> = (1..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS as u8)
        .map(|index| Pubkey::new_from_array([0x70 + index; 32]))
        .collect();
    // `peak_count` one-bits, 8 trailing zeros (cheap appends).
    assert!(
        (1..=55).contains(&peak_count),
        "peak_count must keep 8 trailing zeros free"
    );
    let leaf_count: u64 = (((1u128 << peak_count) - 1) as u64) << 8;
    let peaks: Vec<[u8; 32]> = (0..leaf_count.count_ones())
        .map(|peak| [peak as u8 + 1; 32])
        .collect();

    let mut dictionary = ExecutionDictionary::default();
    let domain_index = dictionary.intern_key(authority);
    let account_index = dictionary.intern_key(authority);
    let subject_indexes = dictionary.intern_subjects(subjects.iter().copied());
    let mut update_steps = Vec::with_capacity(steps);
    let mut metas = Vec::with_capacity(steps);
    let mut accounts = vec![
        (system_program::ID, system_program_account()),
        (authority, funded_system_account()),
        (host_config, host_config_account),
        (event_authority(host::id()), Account::default()),
    ];
    for step_index in 0..steps {
        let value_label = label(&format!("boundary-mature-{step_index}"));
        let handle = handle_for_chain(0x80 + step_index as u8, 5);
        let (address, mut value) =
            new_encrypted_value_account(authority, authority, value_label, handle, &subjects);
        value.leaf_count = leaf_count;
        value.peaks = peaks.clone();
        metas.push(writable(address));
        accounts.push((address, encrypted_value_account(&value)));
        update_steps.push(FheExecuteStep::TrivialEncrypt {
            plaintext: [step_index as u8 + 1; 32],
            fhe_type: 5,
            output: FheExecuteOutput::StoredValue {
                output_encrypted_value_index: step_index as u8,
                output_authority_index: None,
                output_domain_index: domain_index,
                output_account_index: account_index,
                output_label_index: dictionary.intern(value_label),
                output_subject_indexes: subject_indexes.clone(),
                previous_state: Some(PreviousState {
                    handle,
                    subjects: subjects.clone(),
                }),
                make_public: false,
            },
        });
    }
    let instruction = fhe_execute_ix(
        authority,
        authority,
        authority,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps: update_steps,
        },
        metas,
    );
    ProbeCase {
        instruction,
        accounts,
    }
}

/// Every step consumes its own maximum-size verified input: an inline attestation covering
/// `MAX_INPUT_ATTESTATION_HANDLES` handles with `MAX_INPUT_ATTESTATION_EXTRA_DATA` bytes of extra
/// data, signed at the fixture threshold of one. Each attestation is decoded and re-verified
/// (one secp256k1 recovery per signature) in-execution.
fn attestation_per_step_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let signer_key = signing::coprocessor_signing_key();
    let (host_config, host_config_account) =
        zama_solana_test_kit::host_config_account(&HostConfigParams {
            coprocessor_signers: vec![signing::secp_evm_address(&signer_key)],
            coprocessor_threshold: 1,
            ..HostConfigParams::new(authority)
        });
    let mut dictionary = ExecutionDictionary::default();
    let one = dictionary.intern(u256_be(1));
    let attestation_steps = (0..steps)
        .map(|step_index| {
            let ct_handles: Vec<[u8; 32]> = (0..host::MAX_INPUT_ATTESTATION_HANDLES)
                .map(|position| {
                    let mut handle = handle_for_chain(0x90 + step_index as u8, 5);
                    // The host requires each covered handle to carry its own position.
                    handle[21] = position as u8;
                    handle
                })
                .collect();
            let attestation = signing::attestation_signed_by(
                ct_handles[0],
                ct_handles,
                0,
                authority,
                authority,
                vec![0xAB; host::MAX_INPUT_ATTESTATION_EXTRA_DATA],
                std::slice::from_ref(&signer_key),
            );
            FheExecuteStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheExecuteOperand::VerifiedInput {
                    attestation: Box::new(attestation),
                },
                rhs: FheExecuteOperand::Scalar { value_index: one },
                output_fhe_type: 5,
                output: FheExecuteOutput::Transient,
            }
        })
        .collect();
    let instruction = fhe_execute_ix(
        authority,
        authority,
        authority,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: dictionary.into_entries(),
            steps: attestation_steps,
        },
        Vec::new(),
    );
    ProbeCase {
        instruction,
        accounts: vec![
            (system_program::ID, system_program_account()),
            (authority, funded_system_account()),
            (host_config, host_config_account),
            (event_authority(host::id()), Account::default()),
        ],
    }
}

fn all_created_public_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let all: Vec<usize> = (0..steps).collect();
    persistent_creates_batch(
        steps,
        &all,
        authority,
        true,
        std::slice::from_ref(&authority),
    )
    .into()
}

/// Every step writes a plain persistent create (no `make_public`) — the exact shape
/// `zama-fhe`'s `heap_budget.rs` measures on the app side, so its host-side wall is on record
/// next to the app-side byte count that motivates the single step ceiling.
fn all_private_creates_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let all: Vec<usize> = (0..steps).collect();
    persistent_creates_batch(
        steps,
        &all,
        authority,
        false,
        std::slice::from_ref(&authority),
    )
    .into()
}

/// Every fourth step persists a created-public output, the rest stay transient — a mid-weight
/// composite between the chain and all-created-public extremes.
fn mixed_chain_creates_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let creates: Vec<usize> = (0..steps).step_by(4).collect();
    persistent_creates_batch(
        steps,
        &creates,
        authority,
        true,
        std::slice::from_ref(&authority),
    )
    .into()
}

/// Every step creates a persistent output with the widest legal audience: eight distinct
/// subjects, each interning its own dictionary entry and each resolved and stored by the host.
/// The subject-width axis of the create frontier — the builder's build-heap budget stops this
/// shape well below the host's own wall.
fn subject_heavy_creates_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let all: Vec<usize> = (0..steps).collect();
    let subjects: Vec<Pubkey> = (1..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS as u8)
        .map(|index| Pubkey::new_from_array([0x50 + index; 32]))
        .collect();
    persistent_creates_batch(steps, &all, authority, false, &subjects).into()
}

/// [`subject_heavy_creates_case`] with every output also made public — the corner where the
/// widest audience meets the public-outputs event. The one-subject public sweep sits exactly on
/// the trace-heap boundary with zero margin, so this axis pair is measured rather than
/// interpolated from the two single-axis sweeps.
fn subject_heavy_public_creates_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let all: Vec<usize> = (0..steps).collect();
    let subjects: Vec<Pubkey> = (1..=zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS as u8)
        .map(|index| Pubkey::new_from_array([0x60 + index; 32]))
        .collect();
    persistent_creates_batch(steps, &all, authority, true, &subjects).into()
}

/// Every step past the first is a `Sum` over the coprocessor's maximum euint64 operand count,
/// all referencing the previous step's transient (distinct operands keep the derived handles
/// distinct) — the operand-width axis: per step the host allocates its operand tables
/// proportionally and meters every operand's HCU.
fn reduction_heavy_case(steps: usize, authority: Pubkey) -> ProbeCase {
    let (host_config, host_config_account) = host_config_account(authority);
    let mut steps_vec = Vec::with_capacity(steps);
    steps_vec.push(FheExecuteStep::TrivialEncrypt {
        plaintext: [1; 32],
        fhe_type: 5,
        output: FheExecuteOutput::Transient,
    });
    for step_index in 1..steps {
        steps_vec.push(FheExecuteStep::Sum {
            operands: vec![
                FheExecuteOperand::EarlierStep {
                    producer_index: (step_index - 1) as u8
                };
                60
            ],
            fhe_type: 5,
            output: FheExecuteOutput::Transient,
        });
    }
    let instruction = fhe_execute_ix(
        authority,
        authority,
        authority,
        host_config,
        FheExecuteArgs {
            account_count: 0,
            dictionary: ExecutionDictionary::default().into_entries(),
            steps: steps_vec,
        },
        Vec::new(),
    );
    ProbeCase {
        instruction,
        accounts: vec![
            (system_program::ID, system_program_account()),
            (authority, funded_system_account()),
            (host_config, host_config_account),
            (event_authority(host::id()), Account::default()),
        ],
    }
}

/// The largest create count `zama-fhe`'s instruction-trace model admits: the greatest count
/// whose floor — including the app wrapper instruction the builder budgets for — still fits the
/// transaction's trace.
fn max_creates_the_builder_admits(make_public: bool) -> usize {
    (0..=host::MAX_FHE_EXECUTION_STEPS)
        .take_while(|creates| {
            zama_fhe::instruction_trace_floor(*creates, false, make_public)
                <= zama_fhe::TRANSACTION_INSTRUCTION_TRACE_LIMIT
        })
        .last()
        .expect("zero creates always fit")
}

/// What a shape's sweep must prove beyond matching the snapshot: the per-shape tie between the
/// builder's admission model and the measured wall.
enum WallPin {
    /// No app-side model speaks about this shape's wall; the snapshot alone pins it.
    SnapshotOnly,
    /// The builder never admits a create count the host measured as failing: builder cap
    /// `<= max_ok`. For the one-subject public shape the heap and trace walls sit within one
    /// step of each other, so the builder's trace cap covers both.
    BuilderCapCoversWall,
    /// The tie between the builder's trace model and the measured wall: Mollusk probes the host
    /// top-level, one instruction short of the app wrapper the builder's floor budgets, so the
    /// host must run exactly one create past the builder's cap and no further.
    TraceOneUnderBuilderCap,
    /// Invariant #61's measured half. On this axis pair the wall is the HOST's own CPI frame —
    /// the created accounts' subject tables and the public-outputs event payload grow with
    /// creates x subjects-per-output — and it sits well below the trace cap, which the
    /// one-subject sweeps could not see. The builder models only the app's CPI frame
    /// (build + packet + invoke tables), and no app-side number can see this driver: with a
    /// shared audience the dictionary interns eight subjects once, so the builder admits the
    /// trace-capped twenty such creates (`the_builder_admits_what_the_host_heap_cannot_hold`
    /// in zama-fhe's `heap_budget.rs` pins `build()` Ok at 15, 16, and 20) while the host
    /// survives fifteen. Until the host's side gets its own typed ceiling (or a policy cap on
    /// the public payload), this wall is measured, not typed: the snapshot pins it, and the
    /// boundary matrix is the guidance — fhevm-internal#1872.
    HostHeapGap,
}

/// One row of the shape table: the snapshot profile it records, its minimum legal step count,
/// its case builder, and the wall pin its sweep must prove.
struct BoundaryShape {
    profile: &'static str,
    min_steps: usize,
    build: Box<dyn Fn(usize) -> ProbeCase>,
    pin: WallPin,
}

/// The whole frontier as data: every swept shape, its builder, and its pin. The sweeps, the
/// snapshot, and the printer all walk this one table, so a new axis is one new row.
fn boundary_shapes() -> Vec<BoundaryShape> {
    let shape = |profile: &'static str,
                 min_steps: usize,
                 pin: WallPin,
                 build: Box<dyn Fn(usize) -> ProbeCase>| BoundaryShape {
        profile,
        min_steps,
        build,
        pin,
    };
    vec![
        shape(
            "fhe_execute_boundary/all_created_public",
            1,
            WallPin::BuilderCapCoversWall,
            Box::new(|steps| all_created_public_case(steps, Pubkey::new_from_array([0x31; 32]))),
        ),
        shape(
            "fhe_execute_boundary/all_private_creates",
            1,
            WallPin::TraceOneUnderBuilderCap,
            Box::new(|steps| all_private_creates_case(steps, Pubkey::new_from_array([0x36; 32]))),
        ),
        shape(
            "fhe_execute_boundary/subject_heavy_creates",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| subject_heavy_creates_case(steps, Pubkey::new_from_array([0x37; 32]))),
        ),
        shape(
            "fhe_execute_boundary/subject_heavy_public_creates",
            1,
            WallPin::HostHeapGap,
            Box::new(|steps| {
                subject_heavy_public_creates_case(steps, Pubkey::new_from_array([0x3B; 32]))
            }),
        ),
        shape(
            "fhe_execute_boundary/reduction_heavy",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| reduction_heavy_case(steps, Pubkey::new_from_array([0x38; 32]))),
        ),
        shape(
            "fhe_execute_boundary/dependent_chain",
            2,
            WallPin::SnapshotOnly,
            Box::new(|steps| dependent_chain_case(steps, Pubkey::new_from_array([0x32; 32]))),
        ),
        shape(
            "fhe_execute_boundary/mature_updates",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| mature_updates_case(steps, 55, Pubkey::new_from_array([0x33; 32]))),
        ),
        shape(
            "fhe_execute_boundary/mature_updates_peaks_8",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| mature_updates_case(steps, 8, Pubkey::new_from_array([0x39; 32]))),
        ),
        shape(
            "fhe_execute_boundary/mature_updates_peaks_32",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| mature_updates_case(steps, 32, Pubkey::new_from_array([0x3A; 32]))),
        ),
        shape(
            "fhe_execute_boundary/attestation_per_step",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| attestation_per_step_case(steps, Pubkey::new_from_array([0x34; 32]))),
        ),
        shape(
            "fhe_execute_boundary/mixed_chain_creates",
            1,
            WallPin::SnapshotOnly,
            Box::new(|steps| mixed_chain_creates_case(steps, Pubkey::new_from_array([0x35; 32]))),
        ),
    ]
}

/// Proves what a shape's [`WallPin`] claims about its swept boundary.
fn assert_wall_pin(profile: &str, pin: &WallPin, sweep: &SweptBoundary) {
    match pin {
        WallPin::SnapshotOnly => {}
        WallPin::BuilderCapCoversWall => {
            assert!(
                max_creates_the_builder_admits(true) <= sweep.max_ok,
                "{profile}: the builder admits {} created-public outputs but the host wall is {}",
                max_creates_the_builder_admits(true),
                sweep.max_ok,
            );
        }
        WallPin::TraceOneUnderBuilderCap => {
            assert_eq!(sweep.limited_by, "instruction_trace", "{profile}");
            assert_eq!(
                max_creates_the_builder_admits(false) + 1,
                sweep.max_ok,
                "{profile}: the measured trace wall no longer sits one wrapper instruction \
                 past the builder's cap",
            );
        }
        WallPin::HostHeapGap => {
            assert_eq!(
                sweep.limited_by, "heap",
                "{profile}: the wide-audience public wall moved off the host heap — re-read \
                 the WallPin::HostHeapGap doc",
            );
            // The gap's direction, asserted so it cannot drift silently: the host wall sits
            // strictly below the trace-capped count the builder admits. If this ever fails the
            // gap has closed — retire invariant #61, the builder-side pin, and this arm
            // together.
            assert!(
                sweep.max_ok < max_creates_the_builder_admits(true),
                "{profile}: the host's CPI frame now holds every builder-admitted \
                 shared-audience public create ({} measured vs {} admitted) — invariant #61's \
                 gap has closed",
                sweep.max_ok,
                max_creates_the_builder_admits(true),
            );
        }
    }
}

/// Sweeps every shape in the table, asserts its pinned wall, and records it in the snapshot.
/// The `cost_snapshot_` prefix keeps it inside `scripts/update-cost-snapshots.sh`'s test filter.
///
/// Every shape is swept before any failure is reported: a change that moves several
/// boundaries surfaces as one complete report instead of a fix-and-rerun cycle per shape, and
/// an update run (`ZAMA_UPDATE_COST_SNAPSHOT=1`) records every profile even when a wall pin
/// fails mid-table — the snapshot on disk is never left partially rewritten.
#[test]
fn cost_snapshot_boundary_sweeps() {
    let mut failures = Vec::new();
    for shape in boundary_shapes() {
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let sweep = sweep_step_boundary(shape.min_steps, &shape.build);
            assert_swept_boundary(shape.profile, &sweep);
            assert_wall_pin(shape.profile, &shape.pin, &sweep);
        }));
        if let Err(panic) = outcome {
            let message = panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied())
                .unwrap_or("non-string panic");
            failures.push(format!("{}: {message}", shape.profile));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} boundary sweeps failed:\n\n{}",
        failures.len(),
        boundary_shapes().len(),
        failures.join("\n\n"),
    );
}

/// Prints every shape's full curve — CU, packet bytes, CPIs, and the failure axis per step
/// count — from the same table the sweeps assert.
#[test]
#[ignore = "full boundary curves, run explicitly with --nocapture"]
fn print_boundary_matrix() {
    for shape in boundary_shapes() {
        let name = shape
            .profile
            .rsplit('/')
            .next()
            .expect("profile names end in the shape name");
        for steps in shape.min_steps..=host::MAX_FHE_EXECUTION_STEPS {
            let case = (shape.build)(steps);
            let result = mollusk().process_instruction(&case.instruction, &case.accounts);
            println!(
                "{name:28} steps={steps:2} cu={:7} ix_bytes={:5} cpis={:2} axis={}",
                result.compute_units_consumed,
                case.instruction.data.len(),
                result.inner_instructions.len(),
                failure_axis(&result),
            );
        }
    }
}

/// The limits every measurement in this file runs against, as data: what each is, whether a
/// transaction can push it, and how. Solana values are runtime facts of the pinned toolchain;
/// zama values are this repo's policy, asserted from the constants so the snapshot cannot drift
/// from the code.
#[test]
fn cost_snapshot_solana_ceilings() {
    let ceiling = |value: u64, extendable: bool, note: &str| cost_snapshot::Ceiling {
        value,
        extendable,
        note: note.to_string(),
    };
    let ceilings = BTreeMap::from([
        (
            "solana/heap_frame_bytes".to_string(),
            ceiling(
                32 * 1024,
                false,
                "fixed-length bump allocator installed by solana-program-entrypoint; every \
                 invocation (each top-level instruction and each CPI frame) gets a fresh region. \
                 RequestHeapFrame grants up to 256 KiB per frame, but the default allocator's \
                 length is a compile-time constant, so the grant is unusable without a custom \
                 allocator (fhevm-internal#1872: we do not ship one)",
            ),
        ),
        (
            "solana/compute_units_per_instruction_default".to_string(),
            ceiling(
                200_000,
                true,
                "SetComputeUnitLimit raises it toward the transaction cap; the priority fee is \
                 charged on the requested limit, not on usage",
            ),
        ),
        (
            "solana/compute_units_per_transaction_max".to_string(),
            ceiling(
                1_400_000,
                false,
                "hard runtime cap; no instruction raises it",
            ),
        ),
        (
            "solana/transaction_bytes".to_string(),
            ceiling(
                1_232,
                false,
                "IPv6-MTU packet limit; address lookup tables compress account keys, not \
                 instruction data",
            ),
        ),
        (
            "solana/instruction_trace_length".to_string(),
            ceiling(
                64,
                false,
                "every instruction executed in a transaction, top-level plus each CPI; sits \
                 within one step of the all-created-public shape's heap wall (each created \
                 output issues ~3 CPIs), and it is shared with the app's own CPIs in the same \
                 transaction",
            ),
        ),
        (
            "solana/cpi_instruction_data_bytes".to_string(),
            ceiling(10_240, false, "per CPI call"),
        ),
        (
            "solana/cpi_call_depth".to_string(),
            ceiling(5, false, "maximum invoke nesting"),
        ),
        (
            "solana/stack_frame_bytes".to_string(),
            ceiling(4_096, false, "per stack frame"),
        ),
        (
            "solana/return_data_bytes".to_string(),
            ceiling(1_024, false, "per set_return_data"),
        ),
        (
            "zama/max_fhe_execution_steps".to_string(),
            ceiling(
                host::MAX_FHE_EXECUTION_STEPS as u64,
                true,
                "policy cap owned by this repo; raising it is a code change plus a re-run of \
                 the boundary sweeps above (fhevm-internal#1872)",
            ),
        ),
        (
            "zama/max_input_attestation_handles".to_string(),
            ceiling(
                host::MAX_INPUT_ATTESTATION_HANDLES as u64,
                true,
                "policy: covered handles per inline attestation",
            ),
        ),
        (
            "zama/max_input_attestation_extra_data_bytes".to_string(),
            ceiling(
                host::MAX_INPUT_ATTESTATION_EXTRA_DATA as u64,
                true,
                "policy: extra-data bytes per inline attestation",
            ),
        ),
        (
            "zama/max_mmr_peaks".to_string(),
            ceiling(
                zama_solana_acl::MAX_MMR_PEAKS as u64,
                false,
                "structural: a u64 leaf count can never hold more than 64 peaks",
            ),
        ),
        (
            "zama/max_encrypted_value_subjects".to_string(),
            ceiling(
                zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS as u64,
                true,
                "policy: subjects per encrypted value",
            ),
        ),
    ]);
    cost_snapshot::assert_ceilings_snapshot("fhe_execute_boundary", &ceilings);
}
