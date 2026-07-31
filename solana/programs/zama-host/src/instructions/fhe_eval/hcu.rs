//! In-frame HCU (Homomorphic Compute Unit) cost model for [`super::fhe_eval`].
//!
//! Pure, account-independent metering over an `fhe_eval` plan. Everything needed is in
//! [`FheEvalStep`]: a step's cost is a function of its op, result FHE type, and scalar flag; its
//! critical-path *depth* is a function of the operand *kinds* (an `AllowedLocal` reads the depth of
//! the producer it points at; persistent / verified / scalar operands are zero-depth leaves). No
//! sysvars, no accounts — so it runs once in [`super::fhe_eval`], before execution mutates any
//! account, and its total feeds the block-cap charge unchanged.
//!
//! **Fail-closed:** every op variant is enumerated explicitly (no `_ =>` arm over the op enums), so a
//! newly added op fails to compile until a cost decision is made; any `(op, fhe_type, scalar)`
//! combination without a ported row returns [`ZamaHostError::HcuUnknownCost`].
//!
//! **Numbers are representative, EVM-ordered placeholders** (scalar ≤ ciphertext; monotonic in width;
//! select ≥ comparison). Solana-fleet calibration is deferred; because limits ship at `0` (disabled),
//! placeholder costs cannot reject anything pre-calibration.

use anchor_lang::prelude::*;

use crate::errors::ZamaHostError;
use crate::state::{
    FheBinaryOpCode, FheEvalOperand, FheEvalStep, FheTernaryOpCode, FheUnaryOpCode,
};

/// Cost of a binary op producing `fhe_type`. `scalar` is true when the RHS is a plaintext scalar.
pub(super) fn binary_op_hcu(op: FheBinaryOpCode, fhe_type: u8, scalar: bool) -> Result<u64> {
    // No `_ =>` arm: a new FheBinaryOpCode variant must break the build here.
    match op {
        FheBinaryOpCode::Add | FheBinaryOpCode::Sub => arithmetic_hcu(fhe_type, scalar),
        FheBinaryOpCode::Mul | FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            mul_div_rem_hcu(fhe_type, scalar)
        }
        FheBinaryOpCode::And | FheBinaryOpCode::Or | FheBinaryOpCode::Xor => {
            bitwise_hcu(fhe_type, scalar)
        }
        FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr => shift_hcu(fhe_type, scalar),
        // Comparisons produce an `ebool` (fhe_type 0).
        FheBinaryOpCode::Eq
        | FheBinaryOpCode::Ne
        | FheBinaryOpCode::Ge
        | FheBinaryOpCode::Gt
        | FheBinaryOpCode::Le
        | FheBinaryOpCode::Lt => comparison_hcu(fhe_type, scalar),
        // Min/Max: a comparison plus a select.
        FheBinaryOpCode::Min | FheBinaryOpCode::Max => select_hcu(fhe_type),
    }
}

/// Cost of a unary op producing `fhe_type`.
pub(super) fn unary_op_hcu(op: FheUnaryOpCode, fhe_type: u8) -> Result<u64> {
    // No `_ =>` arm: a new FheUnaryOpCode variant must break the build here.
    match op {
        FheUnaryOpCode::Neg | FheUnaryOpCode::Not => unary_transform_hcu(fhe_type),
        FheUnaryOpCode::Cast => cast_hcu(fhe_type),
    }
}

/// Cost of a ternary op producing `fhe_type`.
pub(super) fn ternary_op_hcu(op: FheTernaryOpCode, fhe_type: u8) -> Result<u64> {
    // No `_ =>` arm: a new FheTernaryOpCode variant must break the build here.
    match op {
        FheTernaryOpCode::IfThenElse => select_hcu(fhe_type),
    }
}

/// Cost of trivially encrypting a plaintext of `fhe_type`.
pub(super) fn trivial_encrypt_hcu(fhe_type: u8) -> Result<u64> {
    // Supported trivial-encrypt types mirror `state::assert_supported_fhe_type` (0 | 2..=8).
    match fhe_type {
        0 => Ok(100),   // ebool
        2 => Ok(200),   // euint8
        3 => Ok(300),   // euint16
        4 => Ok(500),   // euint32
        5 => Ok(900),   // euint64
        6 => Ok(1_700), // euint128
        7 => Ok(2_100), // euint160
        8 => Ok(3_300), // euint256
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Cost of generating a random ciphertext of `fhe_type`.
pub(super) fn rand_hcu(fhe_type: u8) -> Result<u64> {
    // Supported rand types mirror `state::assert_supported_rand_type` (0 | 2..=6 | 8 — note: not 7).
    match fhe_type {
        0 => Ok(40_000),
        2 => Ok(42_000),
        3 => Ok(44_000),
        4 => Ok(47_000),
        5 => Ok(52_000),
        6 => Ok(60_000),
        8 => Ok(80_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Add/Sub: monotonic in width; scalar form is cheaper. Same table for Add and Sub (EVM parity).
fn arithmetic_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    let base: u64 = match fhe_type {
        2 => 28_000, // euint8
        3 => 31_000, // euint16
        4 => 34_000, // euint32
        5 => 38_000, // euint64
        6 => 45_000, // euint128
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    // Scalar operations skip ciphertext-ciphertext work: 12.5% cheaper (keeps width-monotonicity).
    Ok(if scalar { base - base / 8 } else { base })
}

/// Ge: result is `ebool` (type 0); operand width is not encoded in the result type, so the placeholder
/// is a single comparison cost (see tests-stage note on this representativeness limitation).
fn comparison_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    match fhe_type {
        0 => Ok(if scalar { 18_000 } else { 21_000 }),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// IfThenElse (select): at least as expensive as a comparison, monotonic in width.
fn select_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 => Ok(30_000),
        2 => Ok(32_000),
        3 => Ok(35_000),
        4 => Ok(38_000),
        5 => Ok(45_000),
        6 => Ok(55_000),
        7 => Ok(60_000),
        8 => Ok(75_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Mul/Div/Rem: heaviest binary ops; monotonic in width; scalar form is cheaper.
fn mul_div_rem_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    let base: u64 = match fhe_type {
        2 => 150_000, // euint8
        3 => 180_000, // euint16
        4 => 220_000, // euint32
        5 => 290_000, // euint64
        6 => 480_000, // euint128
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(if scalar { base - base / 8 } else { base })
}

/// fheMulDiv: a dedicated cost mirroring EVM `HCULimit.checkHCUForFheMulDiv` — a single calibrated
/// opHCU per result type and `factor2` scalar-ness (NOT a mul+div sum), since the coprocessor runs
/// the fused op as an unoptimized 2N-wide chained mul+div. EVM caps mulDiv at Uint64.
fn mul_div_hcu(fhe_type: u8, factor2_scalar: bool) -> Result<u64> {
    let cost = match (fhe_type, factor2_scalar) {
        (2, true) => 495_000,    // euint8, scalar factor2
        (3, true) => 703_000,    // euint16
        (4, true) => 1_080_000,  // euint32
        (5, true) => 1_921_000,  // euint64
        (2, false) => 524_000,   // euint8, encrypted factor2
        (3, false) => 766_000,   // euint16
        (4, false) => 1_311_000, // euint32
        (5, false) => 2_911_000, // euint64
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(cost)
}

/// And/Or/Xor: bitwise, cheaper than arithmetic (no carry propagation); scalar form is cheaper.
fn bitwise_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    let base: u64 = match fhe_type {
        0 => 16_000, // ebool
        2 => 20_000, // euint8
        3 => 22_000, // euint16
        4 => 24_000, // euint32
        5 => 27_000, // euint64
        6 => 32_000, // euint128
        8 => 40_000, // euint256
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(if scalar { base - base / 8 } else { base })
}

/// Shl/Shr/Rotl/Rotr: shift/rotate, between bitwise and arithmetic; scalar form is cheaper.
fn shift_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    let base: u64 = match fhe_type {
        2 => 25_000, // euint8
        3 => 28_000, // euint16
        4 => 31_000, // euint32
        5 => 35_000, // euint64
        6 => 42_000, // euint128
        8 => 55_000, // euint256
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(if scalar { base - base / 8 } else { base })
}

/// Neg/Not: single-operand transform; cheaper than a binary op. `Not` also applies to `ebool` (0).
fn unary_transform_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 => Ok(15_000), // ebool (Not)
        2 => Ok(18_000), // euint8
        3 => Ok(20_000), // euint16
        4 => Ok(23_000), // euint32
        5 => Ok(27_000), // euint64
        6 => Ok(33_000), // euint128
        8 => Ok(42_000), // euint256
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Cast: reinterpret/resize to `fhe_type`; cheap relative to arithmetic. Covers every type Cast's
/// output validation accepts (`is_supported_fhe_type`: 0 | 2..=8), including Address (7) and
/// Uint256 (8), so metering never rejects a cast the on-chain/client validation admitted.
fn cast_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => Ok(5_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Sum of `operand_count` ciphertexts ≈ `(operand_count - 1)` additions of `fhe_type`.
fn sum_hcu(fhe_type: u8, operand_count: usize) -> Result<u64> {
    let per_add = arithmetic_hcu(fhe_type, false)?;
    let adds = operand_count.max(1) as u64 - 1;
    per_add
        .checked_mul(adds)
        .ok_or_else(|| error!(ZamaHostError::HcuUnknownCost))
}

/// IsIn: one equality per set member (select-cost proxy, spanning the full Uint8..Uint256 range), OR-reduced to `ebool`.
fn is_in_hcu(fhe_type: u8, set_len: usize) -> Result<u64> {
    let per_member = select_hcu(fhe_type)?;
    per_member
        .checked_mul(set_len.max(1) as u64)
        .ok_or_else(|| error!(ZamaHostError::HcuUnknownCost))
}

/// `u64::MAX = unlimited`: a no-op when `limit == u64::MAX`, otherwise `used <= limit` or `err`.
pub(super) fn enforce_le(used: u64, limit: u64, err: ZamaHostError) -> Result<()> {
    if limit != u64::MAX && used > limit {
        return Err(error!(err));
    }
    Ok(())
}

/// Running frame total with checked arithmetic; overflow fails closed (never wraps).
pub(super) fn accumulate_total(running: u64, step_hcu: u64) -> Result<u64> {
    running
        .checked_add(step_hcu)
        .ok_or_else(|| error!(ZamaHostError::HcuTransactionLimitExceeded))
}

/// Per-step cumulative depth = op cost + max input depth; overflow fails closed.
pub(super) fn step_depth(step_hcu: u64, max_input_depth: u64) -> Result<u64> {
    step_hcu
        .checked_add(max_input_depth)
        .ok_or_else(|| error!(ZamaHostError::HcuTransactionDepthLimitExceeded))
}

/// The result of metering one frame: the summed total and the cumulative depth of each produced step.
///
/// Production only needs the enforcement side-effect of [`meter_eval_plan`]; the returned values exist
/// for unit tests, so the fields are read only under `#[cfg(test)]`.
#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub(super) struct FrameMeter {
    pub total: u64,
    pub step_depths: Vec<u64>,
}

/// Depth a resolved operand contributes: an `AllowedLocal` carries its producer's cumulative depth;
/// every other operand kind is a zero-depth leaf (persistent resets in-frame; verified input &
/// scalar are intrinsic zero leaves). An out-of-range producer index is treated as `0` here;
/// the walk's operand resolver rejects it with the precise `FheEvalAllowedLocalMissing`.
fn operand_depth(operand: &FheEvalOperand, step_depths: &[u64]) -> u64 {
    // No `_ =>` arm: a new FheEvalOperand variant must break the build here.
    match operand {
        FheEvalOperand::AllowedLocal { producer_index } => step_depths
            .get(*producer_index as usize)
            .copied()
            .unwrap_or(0),
        FheEvalOperand::AllowedPersistent { .. } => 0,
        FheEvalOperand::VerifiedInput { .. } => 0,
        FheEvalOperand::Scalar { .. } => 0,
    }
}

/// Meters a plan: sums per-step costs into a frame total and computes each step's critical-path depth,
/// enforcing both caps after every step (`u64::MAX = unlimited`). Pure over `steps` + the two limits, so
/// off-chain planners compute exactly what on-chain enforcement trips.
pub(super) fn meter_eval_plan(
    steps: &[FheEvalStep],
    max_hcu_per_tx: u64,
    max_hcu_depth_per_tx: u64,
) -> Result<FrameMeter> {
    let mut total: u64 = 0;
    let mut step_depths: Vec<u64> = Vec::with_capacity(steps.len());

    for step in steps {
        let (op_hcu, max_input_depth) = match step {
            FheEvalStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let cost = binary_op_hcu(
                    *op,
                    *output_fhe_type,
                    matches!(rhs, FheEvalOperand::Scalar { .. }),
                )?;
                let depth = operand_depth(lhs, &step_depths).max(operand_depth(rhs, &step_depths));
                (cost, depth)
            }
            FheEvalStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                let cost = ternary_op_hcu(*op, *output_fhe_type)?;
                let depth = operand_depth(control, &step_depths)
                    .max(operand_depth(if_true, &step_depths))
                    .max(operand_depth(if_false, &step_depths));
                (cost, depth)
            }
            FheEvalStep::TrivialEncrypt { fhe_type, .. } => (trivial_encrypt_hcu(*fhe_type)?, 0),
            FheEvalStep::Rand { fhe_type, .. } => (rand_hcu(*fhe_type)?, 0),
            FheEvalStep::Unary {
                op,
                operand,
                output_fhe_type,
                ..
            } => {
                let cost = unary_op_hcu(*op, *output_fhe_type)?;
                let depth = operand_depth(operand, &step_depths);
                (cost, depth)
            }
            // Bounded randomness is a fresh creation (no operands), like Rand.
            FheEvalStep::RandBounded { fhe_type, .. } => (rand_hcu(*fhe_type)?, 0),
            FheEvalStep::Sum {
                operands, fhe_type, ..
            } => {
                let cost = sum_hcu(*fhe_type, operands.len())?;
                let depth = operands
                    .iter()
                    .map(|operand| operand_depth(operand, &step_depths))
                    .max()
                    .unwrap_or(0);
                (cost, depth)
            }
            FheEvalStep::IsIn {
                value,
                set,
                fhe_type,
                ..
            } => {
                let cost = is_in_hcu(*fhe_type, set.len())?;
                let depth = operand_depth(value, &step_depths).max(
                    set.iter()
                        .map(|operand| operand_depth(operand, &step_depths))
                        .max()
                        .unwrap_or(0),
                );
                (cost, depth)
            }
            FheEvalStep::MulDiv {
                factor1,
                factor2,
                output_fhe_type,
                ..
            } => {
                let cost = mul_div_hcu(
                    *output_fhe_type,
                    matches!(factor2, FheEvalOperand::Scalar { .. }),
                )?;
                let depth =
                    operand_depth(factor1, &step_depths).max(operand_depth(factor2, &step_depths));
                (cost, depth)
            }
        };

        // Total: running sum, capped.
        total = accumulate_total(total, op_hcu)?;
        enforce_le(
            total,
            max_hcu_per_tx,
            ZamaHostError::HcuTransactionLimitExceeded,
        )?;

        // Depth: critical path, capped independently of total.
        let depth = step_depth(op_hcu, max_input_depth)?;
        enforce_le(
            depth,
            max_hcu_depth_per_tx,
            ZamaHostError::HcuTransactionDepthLimitExceeded,
        )?;

        step_depths.push(depth);
    }

    Ok(FrameMeter { total, step_depths })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CoprocessorInputAttestation, FheBinaryOpCode, FheEvalOperand, FheEvalOutput, FheEvalStep,
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

    // ---- plan builders (handles are irrelevant to metering; only operand KIND matters) ----
    fn trivial(fhe_type: u8) -> FheEvalStep {
        FheEvalStep::TrivialEncrypt {
            plaintext: [0u8; 32],
            fhe_type,
            output: FheEvalOutput::AllowedLocal,
        }
    }
    fn add_local(ty: u8, lhs_producer: u8, rhs_producer: u8) -> FheEvalStep {
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedLocal {
                producer_index: lhs_producer,
            },
            rhs: FheEvalOperand::AllowedLocal {
                producer_index: rhs_producer,
            },
            output_fhe_type: ty,
            output: FheEvalOutput::AllowedLocal,
        }
    }
    fn add_scalar(ty: u8, lhs_producer: u8) -> FheEvalStep {
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedLocal {
                producer_index: lhs_producer,
            },
            rhs: FheEvalOperand::Scalar { value_index: 0 },
            output_fhe_type: ty,
            output: FheEvalOutput::AllowedLocal,
        }
    }
    fn add_persistent(ty: u8, lhs_producer: u8) -> FheEvalStep {
        FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedLocal {
                producer_index: lhs_producer,
            },
            rhs: FheEvalOperand::AllowedPersistent {
                handle_index: 0,
                encrypted_value_index: 0,
            },
            output_fhe_type: ty,
            output: FheEvalOutput::AllowedLocal,
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
        assert!(binary_op_hcu(FheBinaryOpCode::Ge, EBOOL, false).unwrap() > 0);
        assert!(binary_op_hcu(FheBinaryOpCode::Ge, EBOOL, true).unwrap() > 0);
    }

    #[test]
    fn unary_op_hcu_covers_every_validated_output_type() {
        // Metering must define a cost for every output type the unary validation accepts, or a
        // validated op fails mid-frame with HcuUnknownCost. Cast accepts the full supported set
        // (is_supported_fhe_type: 0 | 2..=8, including Address=7 and Uint256=8).
        for ty in [EBOOL, EU8, 3, 4, EU64, EU128, 7, 8] {
            assert!(unary_op_hcu(FheUnaryOpCode::Cast, ty).unwrap() > 0);
        }
        // Neg accepts 2..=6 | 8; Not additionally accepts ebool.
        for ty in [EU8, 3, 4, EU64, EU128, 8] {
            assert!(unary_op_hcu(FheUnaryOpCode::Neg, ty).unwrap() > 0);
        }
        for ty in [EBOOL, EU8, 3, 4, EU64, EU128, 8] {
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
                            assert!(
                                binary_op_hcu(op, ty, scalar).is_ok(),
                                "validated binary op {op:?} output type {ty} scalar {scalar} \
                                 has no cost row"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn ternary_op_hcu_covers_every_validated_output_type() {
        use super::super::assert_ternary_operand_types;
        for ty in ALL_FHE_TYPE_PROBES {
            let validated =
                assert_ternary_operand_types(handle_of(0), handle_of(ty), handle_of(ty), ty)
                    .is_ok();
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
        for ty in [0u8, 2, 3, 4, 5, 6, 8] {
            assert!(rand_hcu(ty).unwrap() > 0);
        }
    }

    #[test]
    fn rand_hcu_unknown_fails_closed() {
        // type 7 is a supported FHE type generally but not a supported rand type.
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
        assert!(
            ternary_op_hcu(FheTernaryOpCode::IfThenElse, EU64).unwrap()
                >= binary_op_hcu(FheBinaryOpCode::Ge, EBOOL, false).unwrap()
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
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let cost = trivial_encrypt_hcu(EU64).unwrap();
        assert_eq!(m.total, cost);
        assert_eq!(m.step_depths, vec![cost]);
    }

    #[test]
    fn meter_chain_depth_accumulates_along_path() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let t = trivial_encrypt_hcu(EU64).unwrap();
        let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        assert_eq!(m.step_depths, vec![t, add + t, add + add + t]);
        assert_eq!(m.total, t + add + add);
    }

    #[test]
    fn meter_total_sums_all_steps_depth_le_total() {
        let steps = vec![trivial(EU64), trivial(EU64), add_local(EU64, 0, 1)];
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let t = trivial_encrypt_hcu(EU64).unwrap();
        let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        assert_eq!(m.total, t + t + add);
        for d in &m.step_depths {
            assert!(*d <= m.total, "per-value depth never exceeds frame total");
        }
        assert_eq!(*m.step_depths.last().unwrap(), add + t);
    }

    #[test]
    fn meter_total_exceeds_limit_errors() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
        let total = trivial_encrypt_hcu(EU64).unwrap()
            + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        assert_eq!(
            meter_eval_plan(&steps, total - 1, u64::MAX).unwrap_err(),
            error!(ZamaHostError::HcuTransactionLimitExceeded)
        );
    }

    #[test]
    fn meter_total_within_limit_ok() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
        let total = trivial_encrypt_hcu(EU64).unwrap()
            + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        let m = meter_eval_plan(&steps, total, u64::MAX).unwrap();
        assert_eq!(m.total, total);
    }

    #[test]
    fn meter_depth_exceeds_limit_independent_of_total() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_local(EU64, 1, 1)];
        let t = trivial_encrypt_hcu(EU64).unwrap();
        let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        let max_depth = add + t; // depth of step c (add+add+t) exceeds this
        assert_eq!(
            meter_eval_plan(&steps, u64::MAX, max_depth).unwrap_err(),
            error!(ZamaHostError::HcuTransactionDepthLimitExceeded)
        );
    }

    #[test]
    fn meter_depth_within_limit_ok() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0)];
        let t = trivial_encrypt_hcu(EU64).unwrap();
        let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        let m = meter_eval_plan(&steps, u64::MAX, add + t).unwrap();
        assert_eq!(*m.step_depths.last().unwrap(), add + t);
    }

    #[test]
    fn meter_unknown_cost_propagates() {
        // A Rand of type 7 has no cost row -> the walk surfaces HcuUnknownCost (fail-closed).
        let steps = vec![FheEvalStep::Rand {
            fhe_type: 7,
            output: FheEvalOutput::AllowedLocal,
        }];
        assert_eq!(
            meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap_err(),
            error!(ZamaHostError::HcuUnknownCost)
        );
    }

    // ---- leaf semantics ----

    #[test]
    fn meter_scalar_is_zero_leaf() {
        let steps = vec![trivial(EU64), add_scalar(EU64, 0)];
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
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
        let steps = vec![FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::VerifiedInput {
                attestation: Box::new(attestation),
            },
            rhs: FheEvalOperand::Scalar { value_index: 0 },
            output_fhe_type: EU64,
            output: FheEvalOutput::AllowedLocal,
        }];
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
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
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let expected = trivial_encrypt_hcu(EU64).unwrap()
            + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap()
            + binary_op_hcu(FheBinaryOpCode::Add, EU64, true).unwrap()
            + binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        assert_eq!(m.total, expected);
    }

    #[test]
    fn meter_persistent_input_is_zero_depth_leaf() {
        // A persistent operand contributes depth 0 (in-frame reset), so a
        // chain split across a persistent boundary resets depth there rather than carrying it forward.
        let steps = vec![trivial(EU64), add_persistent(EU64, 0)];
        let m = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let t = trivial_encrypt_hcu(EU64).unwrap();
        let add = binary_op_hcu(FheBinaryOpCode::Add, EU64, false).unwrap();
        assert_eq!(*m.step_depths.last().unwrap(), add + t); // add + max(depth(a)=t, persistent=0)
    }

    // ---- disabled at deploy ----

    #[test]
    fn meter_disabled_limits_accept_costliest_plan() {
        // MAX_FHE_EVAL_OPS chained EU128 adds with limits off.
        let cap = u8::try_from(crate::state::MAX_FHE_EVAL_OPS)
            .expect("MAX_FHE_EVAL_OPS must fit producer indices");
        let mut steps = vec![trivial(EU128)];
        for i in 1..cap {
            steps.push(add_local(EU128, i - 1, i - 1));
        }
        assert_eq!(steps.len(), crate::state::MAX_FHE_EVAL_OPS);
        assert!(meter_eval_plan(&steps, u64::MAX, u64::MAX).is_ok());
    }

    // ---- determinism is the on-chain==off-chain parity basis ----

    #[test]
    fn meter_is_deterministic() {
        let steps = vec![trivial(EU64), add_local(EU64, 0, 0), add_scalar(EU64, 1)];
        let a = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        let b = meter_eval_plan(&steps, u64::MAX, u64::MAX).unwrap();
        assert_eq!(a.total, b.total);
        assert_eq!(a.step_depths, b.step_depths);
    }

    // ---- Documentation test for the deferred cross-frame gap (NOT an invariant guard) ----

    #[test]
    fn doc_cross_frame_total_not_metered() {
        // the total is per-frame. Two separate frames, each under the per-frame
        // total, BOTH succeed even though their combined cost exceeds the limit. A future reviewer
        // must not "fix" this into a false cross-frame coverage claim.
        let frame = vec![trivial(EU64), add_local(EU64, 0, 0)];
        let one = meter_eval_plan(&frame, u64::MAX, u64::MAX).unwrap().total;
        let limit = one + one / 2; // < 2 * one
        assert!(meter_eval_plan(&frame, limit, u64::MAX).is_ok()); // frame A
        assert!(meter_eval_plan(&frame, limit, u64::MAX).is_ok()); // frame B — combined exceeds `limit`
    }
}
