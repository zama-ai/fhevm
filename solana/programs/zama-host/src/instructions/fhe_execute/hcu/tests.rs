mod evm_parity;

use super::*;
use crate::state::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteOperand, FheExecuteStep,
    FheTernaryOpCode,
};

// FHE type ids (handle byte 30): 0 = ebool, 2..=6 = euint8..euint128.
const EBOOL: u8 = 0;
const EU8: u8 = 2;
const EU64: u8 = 5;
const EU128: u8 = 6;

/// Every FHE type byte the ABI can carry, plus out-of-range probes. The inverse
/// conformance tests sweep this whole space and consult the `state` validation
/// functions to decide which combinations metering must price.
const ALL_FHE_TYPE_PROBES: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 200];

const ALL_BINARY_OPS: [FheBinaryOpCode; 20] = [
    FheBinaryOpCode::Add,
    FheBinaryOpCode::Sub,
    FheBinaryOpCode::Mul,
    FheBinaryOpCode::Div,
    FheBinaryOpCode::Rem,
    FheBinaryOpCode::And,
    FheBinaryOpCode::Or,
    FheBinaryOpCode::Xor,
    FheBinaryOpCode::Shl,
    FheBinaryOpCode::Shr,
    FheBinaryOpCode::Rotl,
    FheBinaryOpCode::Rotr,
    FheBinaryOpCode::Eq,
    FheBinaryOpCode::Ne,
    FheBinaryOpCode::Ge,
    FheBinaryOpCode::Gt,
    FheBinaryOpCode::Le,
    FheBinaryOpCode::Lt,
    FheBinaryOpCode::Min,
    FheBinaryOpCode::Max,
];

// These fixtures run the production execution walk, including handle validation and derivation.
fn trivial(fhe_type: u8) -> FheExecuteStep {
    FheExecuteStep::TrivialEncrypt {
        plaintext: [0u8; 32],
        fhe_type,
    }
}
fn add_local(ty: u8, lhs_producer: u8, rhs_producer: u8) -> FheExecuteStep {
    FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep {
            producer_index: lhs_producer,
        },
        rhs: FheExecuteOperand::EarlierStep {
            producer_index: rhs_producer,
        },
        output_fhe_type: ty,
    }
}
fn add_scalar(ty: u8, lhs_producer: u8) -> FheExecuteStep {
    FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep {
            producer_index: lhs_producer,
        },
        rhs: FheExecuteOperand::Scalar { value_index: 0 },
        output_fhe_type: ty,
    }
}
fn add_state_slot(ty: u8, lhs_producer: u8) -> FheExecuteStep {
    FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep {
            producer_index: lhs_producer,
        },
        rhs: FheExecuteOperand::StateSlot {
            handle_index: 1,
            state_index: 0,
            key_index: 2,
        },
        output_fhe_type: ty,
    }
}

// ---- cost table is fail-closed + pure ----

#[test]
fn binary_op_hcu_returns_cost_for_shipping_combos() {
    for ty in [EU8, 3, 4, EU64, EU128] {
        for scalar in [false, true] {
            assert!(binary_op_hcu(FheBinaryOpCode::Add, ty, scalar).unwrap() > 0);
            assert!(binary_op_hcu(FheBinaryOpCode::Sub, ty, scalar).unwrap() > 0);
        }
    }
    assert!(binary_op_hcu(FheBinaryOpCode::Ge, EU8, false).unwrap() > 0);
    assert!(binary_op_hcu(FheBinaryOpCode::Ge, EU8, true).unwrap() > 0);
    assert!(binary_op_hcu(FheBinaryOpCode::Eq, EBOOL, false).unwrap() > 0);
}

#[test]
fn unary_op_hcu_covers_every_validated_output_type() {
    // Cast / Not / Neg price through euint128 only (types 7 and 8 have no HCU row).
    for ty in [EBOOL, EU8, 3, 4, EU64, EU128] {
        assert!(unary_op_hcu(FheUnaryOpCode::Cast, ty).unwrap() > 0);
    }
    for ty in [EU8, 3, 4, EU64, EU128] {
        assert!(unary_op_hcu(FheUnaryOpCode::Neg, ty).unwrap() > 0);
    }
    for ty in [EBOOL, EU8, 3, 4, EU64, EU128] {
        assert!(unary_op_hcu(FheUnaryOpCode::Not, ty).unwrap() > 0);
    }
}

// ---- inverse conformance: every combination validation admits has a cost row ----
// (fhevm-internal#1853 W9). Metering runs before the walk's type validation, so a
// validated combination that reached execution must never die with HcuUnknownCost.
// Each test sweeps the full type space and consults the corresponding `state`
// validation function to decide what metering must price.

fn handle_of(ty: u8) -> [u8; 32] {
    let mut handle = [0u8; 32];
    handle[22..30].copy_from_slice(&crate::SOLANA_POC_CHAIN_ID.to_be_bytes());
    handle[30] = ty;
    handle[31] = crate::HANDLE_VERSION;
    handle
}

#[test]
fn binary_op_hcu_covers_every_validated_combination() {
    use crate::state::assert_binary_operand_types;
    for op in ALL_BINARY_OPS {
        for ty in ALL_FHE_TYPE_PROBES {
            for scalar in [false, true] {
                // Comparisons take same-typed operands and produce ebool; other ops take
                // operands of the output type. Sweep the operand type independently so the
                // (operand, output) pairs validation admits are exactly the ones probed.
                for operand_ty in ALL_FHE_TYPE_PROBES {
                    let validated = assert_binary_operand_types(
                        op,
                        handle_of(operand_ty),
                        handle_of(if scalar { 0 } else { operand_ty }),
                        scalar,
                        ty,
                    )
                    .is_ok();
                    if validated {
                        let priced = if is_comparison(op) {
                            binary_op_hcu(op, operand_ty, scalar)
                        } else {
                            binary_op_hcu(op, ty, scalar)
                        };
                        assert!(
                            priced.is_ok(),
                            "validated binary op {op:?} output type {ty} operand {operand_ty} \
                             scalar {scalar} has no cost row"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn ternary_op_hcu_covers_every_validated_output_type() {
    use crate::state::assert_ternary_operand_types;
    for ty in ALL_FHE_TYPE_PROBES {
        let validated =
            assert_ternary_operand_types(handle_of(0), handle_of(ty), handle_of(ty), ty).is_ok();
        if validated {
            assert!(
                ternary_op_hcu(FheTernaryOpCode::IfThenElse, ty).is_ok(),
                "validated ternary output type {ty} has no cost row"
            );
        }
    }
}

#[test]
fn trivial_encrypt_hcu_covers_every_validated_type() {
    use crate::state::assert_supported_fhe_type;
    for ty in ALL_FHE_TYPE_PROBES {
        if assert_supported_fhe_type(ty).is_ok() {
            assert!(
                trivial_encrypt_hcu(ty).is_ok(),
                "validated trivial-encrypt type {ty} has no cost row"
            );
        }
    }
}

#[test]
fn rand_hcu_covers_every_validated_rand_and_bounded_rand_type() {
    use crate::state::{assert_supported_fhe_type, assert_valid_bounded_rand_upper_bound};
    // 2 is a power of two inside every bounded-rand width, so only the type decides.
    let mut two = [0u8; 32];
    two[31] = 2;
    for ty in ALL_FHE_TYPE_PROBES {
        if assert_supported_fhe_type(ty).is_ok() {
            assert!(
                rand_hcu(ty).is_ok(),
                "validated rand type {ty} has no cost row"
            );
        }
        if assert_valid_bounded_rand_upper_bound(two, ty).is_ok() {
            assert!(
                rand_bounded_hcu(ty).is_ok(),
                "validated bounded-rand type {ty} has no cost row"
            );
        }
    }
}

#[test]
fn sum_hcu_covers_every_validated_type_and_operand_count() {
    use crate::state::assert_sum_operand_types;
    for ty in ALL_FHE_TYPE_PROBES {
        // 100 operands is the widest count any type admits; validation gates per type.
        for count in [0usize, 1, 2, 60, 100] {
            let handles = vec![handle_of(ty); count];
            if assert_sum_operand_types(&handles, ty).is_ok() {
                assert!(
                    sum_hcu(ty, count).is_ok(),
                    "validated sum type {ty} x{count} has no cost row"
                );
            }
        }
    }
}

#[test]
fn is_in_hcu_covers_every_validated_type_and_set_size() {
    use crate::state::assert_is_in_operand_types;
    for ty in ALL_FHE_TYPE_PROBES {
        for count in [0usize, 1, 2, 60, 100] {
            let handles = vec![handle_of(ty); count];
            if assert_is_in_operand_types(handle_of(ty), &handles, ty).is_ok() {
                assert!(
                    is_in_hcu(ty, count).is_ok(),
                    "validated is-in type {ty} x{count} has no cost row"
                );
            }
        }
    }
}

#[test]
fn mul_div_hcu_covers_every_validated_combination() {
    use crate::state::assert_mul_div_operand_types;
    for ty in ALL_FHE_TYPE_PROBES {
        for scalar in [false, true] {
            let validated = assert_mul_div_operand_types(
                handle_of(ty),
                handle_of(if scalar { 0 } else { ty }),
                scalar,
                [0xFF; 32], // non-zero at every truncation width
                ty,
            )
            .is_ok();
            if validated {
                assert!(
                    mul_div_hcu(ty, scalar).is_ok(),
                    "validated mul-div output type {ty} scalar {scalar} has no cost row"
                );
            }
        }
    }
}

#[test]
fn binary_op_hcu_unknown_combo_fails_closed() {
    assert_eq!(
        binary_op_hcu(FheBinaryOpCode::Add, EBOOL, false).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
    assert_eq!(
        binary_op_hcu(FheBinaryOpCode::Add, 200, false).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
}

#[test]
fn ternary_op_hcu_returns_cost() {
    for ty in [EBOOL, EU8, EU64, EU128] {
        assert!(ternary_op_hcu(FheTernaryOpCode::IfThenElse, ty).unwrap() > 0);
    }
}

#[test]
fn ternary_op_hcu_unknown_fails_closed() {
    assert_eq!(
        ternary_op_hcu(FheTernaryOpCode::IfThenElse, 200).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
}

#[test]
fn trivial_encrypt_hcu_returns_cost() {
    for ty in [EBOOL, EU8, EU64, EU128] {
        assert!(trivial_encrypt_hcu(ty).unwrap() > 0);
    }
}

#[test]
fn trivial_encrypt_hcu_unknown_fails_closed() {
    assert_eq!(
        trivial_encrypt_hcu(200).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
}

#[test]
fn rand_hcu_returns_cost() {
    for ty in [0u8, 2, 3, 4, 5, 6] {
        assert!(rand_hcu(ty).unwrap() > 0);
    }
}

#[test]
fn rand_hcu_unknown_fails_closed() {
    // Types 7 and 8 are outside the host type gate and have no rand cost row.
    assert_eq!(
        rand_hcu(7).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
    assert_eq!(
        rand_hcu(200).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
}

#[test]
fn cost_rows_are_representative_and_evm_ordered() {
    // Assert RELATIONSHIPS, not magnitudes, so calibration can change numbers freely.
    assert_eq!(
        binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap(),
        binary_op_hcu(FheBinaryOpCode::Sub, EU64, false).unwrap()
    );
    assert!(
        binary_op_hcu(FheBinaryOpCode::Add, EU8, false).unwrap()
            <= binary_op_hcu(FheBinaryOpCode::Add, EU128, false).unwrap()
    );
    assert!(
        binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap()
            <= binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap()
    );
    assert!(sum_hcu(EU64, 1).unwrap() > 0);
}

#[test]
fn meter_comparison_prices_operand_width_not_ebool() {
    let steps = vec![
        trivial(EU64),
        FheExecuteStep::Binary {
            op: FheBinaryOpCode::Ge,
            lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
            rhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
            output_fhe_type: EBOOL,
        },
    ];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let expected = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Ge, EU64, false).unwrap();
    assert_eq!(m.total, expected);
}

#[test]
fn meter_comparison_prices_dictionary_and_verified_input_width() {
    let handle = handle_of(EU64);
    let stored = FheExecuteStep::Binary {
        op: FheBinaryOpCode::Ge,
        lhs: FheExecuteOperand::StateSlot {
            handle_index: 1,
            state_index: 0,
            key_index: 2,
        },
        rhs: FheExecuteOperand::StateSlot {
            handle_index: 1,
            state_index: 0,
            key_index: 2,
        },
        output_fhe_type: EBOOL,
    };
    let stored_meter = meter_execution(&[stored], &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    assert_eq!(
        stored_meter.total,
        binary_op_hcu(FheBinaryOpCode::Ge, EU64, false).unwrap()
    );

    let attestation = input_attestation(handle);
    let verified = FheExecuteStep::Binary {
        op: FheBinaryOpCode::Ge,
        lhs: FheExecuteOperand::VerifiedInput {
            attestation: Box::new(attestation.clone()),
        },
        rhs: FheExecuteOperand::VerifiedInput {
            attestation: Box::new(attestation),
        },
        output_fhe_type: EBOOL,
    };
    let verified_meter =
        meter_execution(&[verified], &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    assert_eq!(
        verified_meter.total,
        binary_op_hcu(FheBinaryOpCode::Ge, EU64, false).unwrap()
    );
}

#[test]
fn cost_accessors_are_deterministic() {
    assert_eq!(
        binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap(),
        binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap()
    );
    assert_eq!(
        trivial_encrypt_hcu(EU64).unwrap(),
        trivial_encrypt_hcu(EU64).unwrap()
    );
}

// ---- u64::MAX = unlimited ----

#[test]
fn enforce_le_zero_limit_is_noop() {
    assert!(enforce_le(
        u64::MAX - 1,
        u64::MAX,
        ZamaHostError::HcuTransactionLimitExceeded
    )
    .is_ok());
}

#[test]
fn enforce_le_at_boundary_ok() {
    assert!(enforce_le(100, 100, ZamaHostError::HcuTransactionLimitExceeded).is_ok());
    assert!(enforce_le(0, 100, ZamaHostError::HcuTransactionLimitExceeded).is_ok());
}

#[test]
fn enforce_le_over_limit_errors() {
    assert_eq!(
        enforce_le(101, 100, ZamaHostError::HcuTransactionLimitExceeded).unwrap_err(),
        error!(ZamaHostError::HcuTransactionLimitExceeded)
    );
}

// ---- checked arithmetic, fail-closed on overflow ----

#[test]
fn accumulate_total_sums() {
    assert_eq!(accumulate_total(10, 5).unwrap(), 15);
    assert_eq!(accumulate_total(0, 0).unwrap(), 0);
}

#[test]
fn accumulate_total_overflow_fails_closed() {
    assert_eq!(
        accumulate_total(u64::MAX, 1).unwrap_err(),
        error!(ZamaHostError::HcuTransactionLimitExceeded)
    );
}

#[test]
fn step_depth_adds() {
    assert_eq!(step_depth(7, 3).unwrap(), 10);
    assert_eq!(step_depth(7, 0).unwrap(), 7);
}

#[test]
fn step_depth_overflow_fails_closed() {
    assert_eq!(
        step_depth(u64::MAX, 1).unwrap_err(),
        error!(ZamaHostError::HcuTransactionDepthLimitExceeded)
    );
}

// ---- the metering pass ----

#[test]
fn meter_single_step_total_and_depth() {
    let steps = vec![trivial(EU64)];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let cost = trivial_encrypt_hcu(EU64).unwrap();
    assert_eq!(m.total, cost);
    assert_eq!(m.step_depths, vec![cost]);
}

#[test]
fn meter_chain_depth_accumulates_along_path() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(m.step_depths, vec![t, add + t, add + add + t]);
    assert_eq!(m.total, t + add + add);
}

#[test]
fn meter_total_sums_all_steps_depth_le_total() {
    let steps = vec![trivial(EU64), trivial(EU64), add_local(EU64, 0, 1)];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(m.total, t + t + add);
    for d in &m.step_depths {
        assert!(
            *d <= m.total,
            "per-value depth never exceeds execution total"
        );
    }
    assert_eq!(*m.step_depths.last().unwrap(), add + t);
}

#[test]
fn meter_total_exceeds_limit_errors() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let total = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(
        meter_execution(&steps, &walk_dictionary(), total - 1, u64::MAX).unwrap_err(),
        error!(ZamaHostError::HcuTransactionLimitExceeded)
    );
}

#[test]
fn meter_total_within_limit_ok() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let total = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let m = meter_execution(&steps, &walk_dictionary(), total, u64::MAX).unwrap();
    assert_eq!(m.total, total);
}

#[test]
fn meter_depth_exceeds_limit_independent_of_total() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let max_depth = add + t; // depth of step c (add+add+t) exceeds this
    assert_eq!(
        meter_execution(&steps, &walk_dictionary(), u64::MAX, max_depth).unwrap_err(),
        error!(ZamaHostError::HcuTransactionDepthLimitExceeded)
    );
}

#[test]
fn meter_depth_within_limit_ok() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, add + t).unwrap();
    assert_eq!(*m.step_depths.last().unwrap(), add + t);
}

#[test]
fn meter_unknown_type_fails_before_charging() {
    // Unsupported types never reach charging; missing cost rows are covered above.
    let steps = vec![FheExecuteStep::Rand { fhe_type: 7 }];
    assert_eq!(
        meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap_err(),
        error!(ZamaHostError::UnsupportedFheType)
    );
}

// ---- leaf semantics ----

#[test]
fn meter_scalar_is_zero_leaf() {
    let steps = vec![trivial(EU64), add_scalar(EU64, 0)];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add_scalar_cost = binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap();
    assert_eq!(m.total, t + add_scalar_cost);
    assert_eq!(*m.step_depths.last().unwrap(), add_scalar_cost + t);
}

#[test]
fn meter_verified_input_is_zero_leaf() {
    let attestation = input_attestation(handle_of(EU64));
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::VerifiedInput {
            attestation: Box::new(attestation),
        },
        rhs: FheExecuteOperand::Scalar { value_index: 0 },
        output_fhe_type: EU64,
    }];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let add_scalar_cost = binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap();
    assert_eq!(m.total, add_scalar_cost);
    assert_eq!(m.step_depths, vec![add_scalar_cost]);
}

#[test]
fn meter_operands_never_add_to_total() {
    let steps = vec![
        trivial(EU64),
        add_local(EU64, 0, 0),
        add_scalar(EU64, 1),
        add_state_slot(EU64, 2),
    ];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let expected = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(m.total, expected);
}

#[test]
fn meter_state_slot_input_is_zero_depth_leaf() {
    // A State-slot operand contributes depth 0 (in-execution reset), so a
    // chain split across a State boundary resets depth there rather than carrying it forward.
    let steps = vec![trivial(EU64), add_state_slot(EU64, 0)];
    let m = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(*m.step_depths.last().unwrap(), add + t); // add + max(depth(a)=t, State slot=0)
}

// ---- disabled at deploy ----

#[test]
fn meter_disabled_limits_accept_costliest_plan() {
    // MAX_FHE_EXECUTION_STEPS chained EU128 adds with limits off.
    let cap = u8::try_from(crate::state::MAX_FHE_EXECUTION_STEPS)
        .expect("MAX_FHE_EXECUTION_STEPS must fit producer indices");
    let mut steps = vec![trivial(EU128)];
    for i in 1..cap {
        steps.push(add_local(EU128, i - 1, i - 1));
    }
    assert_eq!(steps.len(), crate::state::MAX_FHE_EXECUTION_STEPS);
    assert!(meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).is_ok());
}

// ---- determinism is the on-chain==off-chain parity basis ----

#[test]
fn meter_is_deterministic() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_scalar(EU64, 1)];
    let a = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    let b = meter_execution(&steps, &walk_dictionary(), u64::MAX, u64::MAX).unwrap();
    assert_eq!(a.total, b.total);
    assert_eq!(a.step_depths, b.step_depths);
}

#[test]
fn cross_execution_total_is_metered_in_one_journal() {
    let execution = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let one = meter_execution(&execution, &walk_dictionary(), u64::MAX, u64::MAX)
        .unwrap()
        .total;
    let mut transient_store = Box::new(<crate::TransientStore as bytemuck::Zeroable>::zeroed());
    run_walk(
        &execution,
        &walk_dictionary(),
        one + one / 2,
        u64::MAX,
        &mut transient_store,
    )
    .unwrap();
    assert_eq!(
        run_walk(
            &execution,
            &walk_dictionary(),
            one + one / 2,
            u64::MAX,
            &mut transient_store
        )
        .unwrap_err(),
        error!(ZamaHostError::HcuTransactionLimitExceeded)
    );
}

fn walk_dictionary() -> [[u8; 32]; 3] {
    [[0; 32], handle_of(EU64), [42; 32]]
}

fn signing_key() -> k256::ecdsa::SigningKey {
    k256::ecdsa::SigningKey::from_bytes(&[0x44; 32].into()).unwrap()
}

fn input_attestation(handle: [u8; 32]) -> CoprocessorInputAttestation {
    let user_address = [1; 32];
    let contract_address = [2; 32];
    let hash = crate::eip712::ciphertext_verification_struct_hash(
        &[handle],
        &user_address,
        &contract_address,
        crate::SOLANA_POC_CHAIN_ID,
        &[],
    );
    let domain = crate::eip712::domain_separator(b"InputVerification", b"1", 31337, &[0xCD; 20]);
    let digest = crate::eip712::typed_data_digest(&domain, &hash);
    let (signature, recovery_id) = signing_key().sign_prehash_recoverable(&digest).unwrap();
    let mut bytes = [0; 65];
    bytes[..64].copy_from_slice(&signature.to_bytes());
    bytes[64] = recovery_id.to_byte() + 27;
    CoprocessorInputAttestation {
        input_handle: handle,
        ct_handles: vec![handle],
        handle_index: 0,
        user_address,
        contract_address,
        contract_chain_id: crate::SOLANA_POC_CHAIN_ID,
        extra_data: vec![],
        signatures: vec![bytes],
    }
}

#[derive(Debug)]
struct Metered {
    total: u64,
    step_depths: Vec<u64>,
}

fn meter_execution(
    steps: &[FheExecuteStep],
    dictionary: &[[u8; 32]],
    total: u64,
    depth: u64,
) -> Result<Metered> {
    let mut transient_store = Box::new(<crate::TransientStore as bytemuck::Zeroable>::zeroed());
    run_walk(steps, dictionary, total, depth, &mut transient_store)
}

// Directly exercises the same walk used by fhe_execute. Signature admission and
// transaction lifetime are covered separately by preflight and runtime tests.
fn run_walk(
    steps: &[FheExecuteStep],
    dictionary: &[[u8; 32]],
    total: u64,
    depth: u64,
    transient_store: &mut crate::TransientStore,
) -> Result<Metered> {
    use crate::{AppScope, EncryptedSlot, EncryptedState, FheExecuteArgs, HostConfig};
    let app = AppScope {
        program: Pubkey::new_from_array([2; 32]),
        scope: [3; 32],
    };
    let authority = Pubkey::new_from_array([4; 32]);
    let mut state = EncryptedState {
        program: app.program,
        scope: app.scope,
        authority,
        slots: vec![EncryptedSlot {
            key: [42; 32],
            handle: handle_of(EU64),
        }],
        leaf_count: 0,
        peaks: vec![],
        bump: 0,
    };
    let (address, bump) = state.canonical_address();
    state.bump = bump;
    let mut data = Vec::new();
    state.try_serialize(&mut data).unwrap();
    let mut lamports = 0;
    let account = AccountInfo::new(
        &address,
        false,
        true,
        &mut lamports,
        &mut data,
        &crate::ID,
        false,
    );
    let accounts = [account];
    let mut table = super::super::account_table::ExecutionAccountTable::new(&accounts)?;
    let public = signing_key().verifying_key().to_encoded_point(false);
    let hash = solana_keccak_hasher::hash(&public.as_bytes()[1..]).to_bytes();
    let signer: [u8; 20] = hash[12..].try_into().unwrap();
    let config = HostConfig {
        admin: authority,
        chain_id: crate::SOLANA_POC_CHAIN_ID,
        gateway_chain_id: 31337,
        input_verification_contract: [0xCD; 20],
        coprocessor_signers: crate::pack_coprocessor_signers(&[signer]),
        coprocessor_signer_count: 1,
        coprocessor_threshold: 1,
        decryption_contract: [1; 20],
        current_kms_context_id: [0; 32],
        paused: false,
        grant_deny_list_enabled: false,
        max_hcu_per_tx: total,
        max_hcu_depth_per_tx: depth,
        hcu_block_cap_per_app: u64::MAX,
        updated_slot: 0,
        bump: 0,
    };
    let context = super::super::walk::ExecutionHandleContext {
        derivation: crate::HandleDerivationContext {
            chain_id: config.chain_id,
            previous_bank_hash: [1; 32],
            unix_timestamp: 42,
        },
        rand: Some(super::super::walk::RandContext { nonce: 0, app }),
    };
    let args = FheExecuteArgs {
        execution_state_index: 0,
        account_count: 1,
        dictionary: dictionary.to_vec(),
        steps: steps.to_vec(),
        effects: vec![],
        returned_results: vec![],
    };
    let start = transient_store.len();
    super::super::execute_steps(
        &mut table,
        transient_store,
        start,
        &args,
        app,
        &context,
        &config,
    )?;
    Ok(Metered {
        total: transient_store.total_hcu,
        step_depths: (start..transient_store.len())
            .map(|i| transient_store.result(i).unwrap().depth)
            .collect(),
    })
}
