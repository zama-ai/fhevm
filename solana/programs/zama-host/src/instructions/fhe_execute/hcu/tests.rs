use super::*;
use crate::state::{
    CoprocessorInputAttestation, FheBinaryOpCode, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep, FheTernaryOpCode,
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

// ---- execution builders (handles are irrelevant to metering; only operand KIND matters) ----
fn trivial(fhe_type: u8) -> FheExecuteStep {
    FheExecuteStep::TrivialEncrypt {
        plaintext: [0u8; 32],
        fhe_type,
        output: FheExecuteOutput::Transient,
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
        output: FheExecuteOutput::Transient,
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
        output: FheExecuteOutput::Transient,
    }
}
fn add_persistent(ty: u8, lhs_producer: u8) -> FheExecuteStep {
    FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::EarlierStep {
            producer_index: lhs_producer,
        },
        rhs: FheExecuteOperand::StoredValue {
            handle_index: 0,
            encrypted_value_index: 0,
        },
        output_fhe_type: ty,
        output: FheExecuteOutput::Transient,
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
    handle[30] = ty;
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
                        let priced = if matches!(
                            op,
                            FheBinaryOpCode::Eq
                                | FheBinaryOpCode::Ne
                                | FheBinaryOpCode::Ge
                                | FheBinaryOpCode::Gt
                                | FheBinaryOpCode::Le
                                | FheBinaryOpCode::Lt
                        ) {
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
    use crate::state::{assert_supported_bounded_rand_type, assert_supported_rand_type};
    for ty in ALL_FHE_TYPE_PROBES {
        if assert_supported_rand_type(ty).is_ok() {
            assert!(
                rand_hcu(ty).is_ok(),
                "validated rand type {ty} has no cost row"
            );
        }
        // RandBounded meters through the same rand_hcu table.
        if assert_supported_bounded_rand_type(ty).is_ok() {
            assert!(
                rand_hcu(ty).is_ok(),
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
            output: FheExecuteOutput::Transient,
        },
    ];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let expected = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Ge, EU64, false).unwrap();
    assert_eq!(m.total, expected);
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
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let cost = trivial_encrypt_hcu(EU64).unwrap();
    assert_eq!(m.total, cost);
    assert_eq!(m.step_depths, vec![cost]);
}

#[test]
fn meter_chain_depth_accumulates_along_path() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(m.step_depths, vec![t, add + t, add + add + t]);
    assert_eq!(m.total, t + add + add);
}

#[test]
fn meter_total_sums_all_steps_depth_le_total() {
    let steps = vec![trivial(EU64), trivial(EU64), add_local(EU64, 0, 1)];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
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
        meter_execution(&steps, &[], total - 1, u64::MAX).unwrap_err(),
        error!(ZamaHostError::HcuTransactionLimitExceeded)
    );
}

#[test]
fn meter_total_within_limit_ok() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let total = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let m = meter_execution(&steps, &[], total, u64::MAX).unwrap();
    assert_eq!(m.total, total);
}

#[test]
fn meter_depth_exceeds_limit_independent_of_total() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let max_depth = add + t; // depth of step c (add+add+t) exceeds this
    assert_eq!(
        meter_execution(&steps, &[], u64::MAX, max_depth).unwrap_err(),
        error!(ZamaHostError::HcuTransactionDepthLimitExceeded)
    );
}

#[test]
fn meter_depth_within_limit_ok() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    let m = meter_execution(&steps, &[], u64::MAX, add + t).unwrap();
    assert_eq!(*m.step_depths.last().unwrap(), add + t);
}

#[test]
fn meter_unknown_cost_propagates() {
    // A Rand of type 7 has no cost row -> the walk surfaces HcuUnknownCost (fail-closed).
    let steps = vec![FheExecuteStep::Rand {
        fhe_type: 7,
        output: FheExecuteOutput::Transient,
    }];
    assert_eq!(
        meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap_err(),
        error!(ZamaHostError::HcuUnknownCost)
    );
}

// ---- leaf semantics ----

#[test]
fn meter_scalar_is_zero_leaf() {
    let steps = vec![trivial(EU64), add_scalar(EU64, 0)];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add_scalar_cost = binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap();
    assert_eq!(m.total, t + add_scalar_cost);
    assert_eq!(*m.step_depths.last().unwrap(), add_scalar_cost + t);
}

#[test]
fn meter_verified_input_is_zero_leaf() {
    let attestation = CoprocessorInputAttestation {
        input_handle: [9u8; 32],
        ct_handles: vec![[9u8; 32]],
        handle_index: 0,
        user_address: [0u8; 32],
        contract_address: [0u8; 32],
        contract_chain_id: 0,
        extra_data: vec![],
        signatures: vec![],
    };
    let steps = vec![FheExecuteStep::Binary {
        op: FheBinaryOpCode::Add,
        lhs: FheExecuteOperand::VerifiedInput {
            attestation: Box::new(attestation),
        },
        rhs: FheExecuteOperand::Scalar { value_index: 0 },
        output_fhe_type: EU64,
        output: FheExecuteOutput::Transient,
    }];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
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
        add_persistent(EU64, 2),
    ];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let expected = trivial_encrypt_hcu(EU64).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap()
        + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(m.total, expected);
}

#[test]
fn meter_persistent_input_is_zero_depth_leaf() {
    // A persistent operand contributes depth 0 (in-execution reset), so a
    // chain split across a persistent boundary resets depth there rather than carrying it forward.
    let steps = vec![trivial(EU64), add_persistent(EU64, 0)];
    let m = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let t = trivial_encrypt_hcu(EU64).unwrap();
    let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
    assert_eq!(*m.step_depths.last().unwrap(), add + t); // add + max(depth(a)=t, persistent=0)
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
    assert!(meter_execution(&steps, &[], u64::MAX, u64::MAX).is_ok());
}

// ---- determinism is the on-chain==off-chain parity basis ----

#[test]
fn meter_is_deterministic() {
    let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_scalar(EU64, 1)];
    let a = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    let b = meter_execution(&steps, &[], u64::MAX, u64::MAX).unwrap();
    assert_eq!(a.total, b.total);
    assert_eq!(a.step_depths, b.step_depths);
}

// ---- Documentation test for the deferred cross-execution gap (NOT an invariant guard) ----

#[test]
fn doc_cross_batch_total_not_metered() {
    // the total is per-execution. Two separate executions, each under the per-execution
    // total, BOTH succeed even though their combined cost exceeds the limit. A future reviewer
    // must not "fix" this into a false cross-execution coverage claim.
    let execution = vec![trivial(EU64), add_local(EU64, 0, 0)];
    let one = meter_execution(&execution, &[], u64::MAX, u64::MAX)
        .unwrap()
        .total;
    let limit = one + one / 2; // < 2 * one
    assert!(meter_execution(&execution, &[], limit, u64::MAX).is_ok()); // execution A
    assert!(meter_execution(&execution, &[], limit, u64::MAX).is_ok()); // execution B — combined exceeds `limit`
}
