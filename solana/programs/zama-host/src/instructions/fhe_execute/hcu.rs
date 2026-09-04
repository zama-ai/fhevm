//! In-execution HCU (Homomorphic Compute Unit) cost model for [`super::fhe_execute`].
//!
//! Pure, account-independent metering over an `fhe_execute` execution. Everything needed is in
//! [`FheExecuteStep`] plus the interned dictionary (so comparisons can price on operand width).
//! A step's cost is a function of its op, FHE type, and scalar flag; its critical-path *depth*
//! is a function of the operand *kinds* (an `EarlierStep` reads the depth of the producer it
//! points at; persistent / verified / scalar operands are zero-depth leaves). No sysvars, no
//! accounts — so it runs once in [`super::fhe_execute`], before execution mutates any account,
//! and its total feeds the block-cap charge unchanged.
//!
//! **Fail-closed:** every op variant is enumerated explicitly (no `_ =>` arm over the op enums), so a
//! newly added op fails to compile until a cost decision is made; any `(op, fhe_type, scalar)`
//! combination without a ported EVM row returns [`ZamaHostError::HcuUnknownCost`].
//!
//! **Numbers are the EVM `HCULimit` tables**, hardcoded, through `euint128` (type 6). Types 7
//! (`euint160`) and 8 (`euint256`) have no rows. Caps stay `u64::MAX` (off).

use anchor_lang::prelude::*;

use crate::errors::ZamaHostError;
use crate::state::{
    handle_fhe_type, FheBinaryOpCode, FheExecuteOperand, FheExecuteStep, FheTernaryOpCode,
    FheUnaryOpCode,
};

fn is_comparison(op: FheBinaryOpCode) -> bool {
    matches!(
        op,
        FheBinaryOpCode::Eq
            | FheBinaryOpCode::Ne
            | FheBinaryOpCode::Ge
            | FheBinaryOpCode::Gt
            | FheBinaryOpCode::Le
            | FheBinaryOpCode::Lt
    )
}

/// Cost of a binary op. `fhe_type` is the result type except for comparisons, which price on
/// **operand width**.
pub(super) fn binary_op_hcu(op: FheBinaryOpCode, fhe_type: u8, scalar: bool) -> Result<u64> {
    match op {
        FheBinaryOpCode::Add => add_hcu(fhe_type, scalar),
        FheBinaryOpCode::Sub => sub_hcu(fhe_type, scalar),
        FheBinaryOpCode::Mul => mul_hcu(fhe_type, scalar),
        FheBinaryOpCode::Div => div_hcu(fhe_type, scalar),
        FheBinaryOpCode::Rem => rem_hcu(fhe_type, scalar),
        FheBinaryOpCode::And => bitand_hcu(fhe_type, scalar),
        FheBinaryOpCode::Or => bitor_hcu(fhe_type, scalar),
        FheBinaryOpCode::Xor => bitxor_hcu(fhe_type, scalar),
        FheBinaryOpCode::Shl => shl_hcu(fhe_type, scalar),
        FheBinaryOpCode::Shr => shr_hcu(fhe_type, scalar),
        FheBinaryOpCode::Rotl => rotl_hcu(fhe_type, scalar),
        FheBinaryOpCode::Rotr => rotr_hcu(fhe_type, scalar),
        FheBinaryOpCode::Eq => eq_hcu(fhe_type, scalar),
        FheBinaryOpCode::Ne => ne_hcu(fhe_type, scalar),
        FheBinaryOpCode::Ge => ge_hcu(fhe_type, scalar),
        FheBinaryOpCode::Gt => gt_hcu(fhe_type, scalar),
        FheBinaryOpCode::Le => le_hcu(fhe_type, scalar),
        FheBinaryOpCode::Lt => lt_hcu(fhe_type, scalar),
        FheBinaryOpCode::Min => min_hcu(fhe_type, scalar),
        FheBinaryOpCode::Max => max_hcu(fhe_type, scalar),
    }
}

pub(super) fn unary_op_hcu(op: FheUnaryOpCode, fhe_type: u8) -> Result<u64> {
    match op {
        FheUnaryOpCode::Neg => neg_hcu(fhe_type),
        FheUnaryOpCode::Not => not_hcu(fhe_type),
        FheUnaryOpCode::Cast => cast_hcu(fhe_type),
    }
}

pub(super) fn ternary_op_hcu(op: FheTernaryOpCode, fhe_type: u8) -> Result<u64> {
    match op {
        FheTernaryOpCode::IfThenElse => select_hcu(fhe_type),
    }
}

pub(super) fn trivial_encrypt_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 | 2 | 3 | 4 | 5 | 6 => Ok(32),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

pub(super) fn rand_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 => Ok(19_000),
        2 => Ok(23_000),
        3 => Ok(23_000),
        4 => Ok(24_000),
        5 => Ok(24_000),
        6 => Ok(25_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn add_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 84_000,
        (3, true) => 93_000,
        (4, true) => 95_000,
        (5, true) => 133_000,
        (6, true) => 172_000,
        (2, false) => 88_000,
        (3, false) => 93_000,
        (4, false) => 125_000,
        (5, false) => 162_000,
        (6, false) => 259_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn sub_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 84_000,
        (3, true) => 93_000,
        (4, true) => 95_000,
        (5, true) => 133_000,
        (6, true) => 172_000,
        (2, false) => 91_000,
        (3, false) => 93_000,
        (4, false) => 125_000,
        (5, false) => 162_000,
        (6, false) => 260_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn mul_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 122_000,
        (3, true) => 193_000,
        (4, true) => 265_000,
        (5, true) => 365_000,
        (6, true) => 696_000,
        (2, false) => 150_000,
        (3, false) => 222_000,
        (4, false) => 328_000,
        (5, false) => 596_000,
        (6, false) => 1_686_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn div_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    if !scalar {
        return Err(error!(ZamaHostError::HcuUnknownCost));
    }
    match fhe_type {
        2 => Ok(210_000),
        3 => Ok(302_000),
        4 => Ok(438_000),
        5 => Ok(715_000),
        6 => Ok(1_225_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn rem_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    if !scalar {
        return Err(error!(ZamaHostError::HcuUnknownCost));
    }
    match fhe_type {
        2 => Ok(440_000),
        3 => Ok(580_000),
        4 => Ok(792_000),
        5 => Ok(1_153_000),
        6 => Ok(1_943_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn bitand_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (0, true) => 22_000,
        (2, true) => 31_000,
        (3, true) => 31_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (0, false) => 25_000,
        (2, false) => 31_000,
        (3, false) => 31_000,
        (4, false) => 32_000,
        (5, false) => 34_000,
        (6, false) => 37_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn bitor_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (0, true) => 22_000,
        (2, true) => 30_000,
        (3, true) => 30_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (0, false) => 24_000,
        (2, false) => 30_000,
        (3, false) => 31_000,
        (4, false) => 32_000,
        (5, false) => 34_000,
        (6, false) => 37_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn bitxor_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (0, true) => 22_000,
        (2, true) => 31_000,
        (3, true) => 31_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (0, false) => 22_000,
        (2, false) => 31_000,
        (3, false) => 31_000,
        (4, false) => 32_000,
        (5, false) => 34_000,
        (6, false) => 37_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn shl_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 32_000,
        (3, true) => 32_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (2, false) => 92_000,
        (3, false) => 125_000,
        (4, false) => 162_000,
        (5, false) => 208_000,
        (6, false) => 272_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn shr_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 32_000,
        (3, true) => 32_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (2, false) => 91_000,
        (3, false) => 123_000,
        (4, false) => 163_000,
        (5, false) => 209_000,
        (6, false) => 272_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn rotl_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 31_000,
        (3, true) => 31_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (2, false) => 91_000,
        (3, false) => 125_000,
        (4, false) => 163_000,
        (5, false) => 209_000,
        (6, false) => 278_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn rotr_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 31_000,
        (3, true) => 31_000,
        (4, true) => 32_000,
        (5, true) => 34_000,
        (6, true) => 37_000,
        (2, false) => 93_000,
        (3, false) => 125_000,
        (4, false) => 160_000,
        (5, false) => 209_000,
        (6, false) => 283_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn eq_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (0, true) => 25_000,
        (2, true) => 55_000,
        (3, true) => 55_000,
        (4, true) => 82_000,
        (5, true) => 83_000,
        (6, true) => 117_000,
        (0, false) => 26_000,
        (2, false) => 55_000,
        (3, false) => 83_000,
        (4, false) => 86_000,
        (5, false) => 120_000,
        (6, false) => 122_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn ne_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (0, true) => 23_000,
        (2, true) => 55_000,
        (3, true) => 55_000,
        (4, true) => 83_000,
        (5, true) => 84_000,
        (6, true) => 117_000,
        (0, false) => 23_000,
        (2, false) => 55_000,
        (3, false) => 83_000,
        (4, false) => 85_000,
        (5, false) => 118_000,
        (6, false) => 122_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn ge_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 52_000,
        (3, true) => 55_000,
        (4, true) => 84_000,
        (5, true) => 116_000,
        (6, true) => 149_000,
        (2, false) => 63_000,
        (3, false) => 84_000,
        (4, false) => 118_000,
        (5, false) => 152_000,
        (6, false) => 210_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn gt_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 52_000,
        (3, true) => 55_000,
        (4, true) => 84_000,
        (5, true) => 117_000,
        (6, true) => 150_000,
        (2, false) => 59_000,
        (3, false) => 84_000,
        (4, false) => 118_000,
        (5, false) => 152_000,
        (6, false) => 218_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn le_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 58_000,
        (3, true) => 58_000,
        (4, true) => 84_000,
        (5, true) => 119_000,
        (6, true) => 150_000,
        (2, false) => 58_000,
        (3, false) => 83_000,
        (4, false) => 117_000,
        (5, false) => 149_000,
        (6, false) => 218_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn lt_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 52_000,
        (3, true) => 58_000,
        (4, true) => 83_000,
        (5, true) => 118_000,
        (6, true) => 149_000,
        (2, false) => 59_000,
        (3, false) => 84_000,
        (4, false) => 117_000,
        (5, false) => 146_000,
        (6, false) => 215_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn min_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 84_000,
        (3, true) => 88_000,
        (4, true) => 117_000,
        (5, true) => 150_000,
        (6, true) => 186_000,
        (2, false) => 119_000,
        (3, false) => 146_000,
        (4, false) => 182_000,
        (5, false) => 219_000,
        (6, false) => 289_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn max_hcu(fhe_type: u8, scalar: bool) -> Result<u64> {
    Ok(match (fhe_type, scalar) {
        (2, true) => 89_000,
        (3, true) => 89_000,
        (4, true) => 117_000,
        (5, true) => 149_000,
        (6, true) => 180_000,
        (2, false) => 121_000,
        (3, false) => 145_000,
        (4, false) => 180_000,
        (5, false) => 218_000,
        (6, false) => 290_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    })
}

fn select_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 => Ok(55_000),
        2 => Ok(55_000),
        3 => Ok(55_000),
        4 => Ok(55_000),
        5 => Ok(55_000),
        6 => Ok(57_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn mul_div_hcu(fhe_type: u8, factor2_scalar: bool) -> Result<u64> {
    let cost = match (fhe_type, factor2_scalar) {
        (2, true) => 495_000,
        (3, true) => 703_000,
        (4, true) => 1_080_000,
        (5, true) => 1_921_000,
        (2, false) => 524_000,
        (3, false) => 766_000,
        (4, false) => 1_311_000,
        (5, false) => 2_911_000,
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(cost)
}

fn neg_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        2 => Ok(79_000),
        3 => Ok(93_000),
        4 => Ok(95_000),
        5 => Ok(131_000),
        6 => Ok(168_000),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn not_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 => Ok(2),
        2 => Ok(9),
        3 => Ok(16),
        4 => Ok(32),
        5 => Ok(63),
        6 => Ok(130),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn cast_hcu(fhe_type: u8) -> Result<u64> {
    match fhe_type {
        0 | 2 | 3 | 4 | 5 | 6 => Ok(32),
        _ => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

fn sum_hcu(fhe_type: u8, operand_count: usize) -> Result<u64> {
    let n = operand_count;
    let cost = match fhe_type {
        2 => {
            if n <= 10 {
                90_900
            } else if n <= 30 {
                127_000
            } else if n <= 60 {
                148_000
            } else {
                159_000
            }
        }
        3 => {
            if n <= 10 {
                95_000
            } else if n <= 30 {
                136_000
            } else if n <= 60 {
                162_000
            } else {
                184_000
            }
        }
        4 => {
            if n <= 10 {
                116_000
            } else if n <= 30 {
                164_000
            } else if n <= 60 {
                205_000
            } else {
                281_000
            }
        }
        5 => {
            if n <= 10 {
                139_000
            } else if n <= 30 {
                216_000
            } else {
                306_000
            }
        }
        6 => {
            if n <= 10 {
                219_000
            } else if n <= 30 {
                355_000
            } else {
                552_000
            }
        }
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(cost)
}

fn is_in_hcu(fhe_type: u8, set_len: usize) -> Result<u64> {
    let n = set_len;
    let cost = match fhe_type {
        2 => {
            if n <= 10 {
                71_300
            } else if n <= 30 {
                148_000
            } else if n <= 60 {
                247_000
            } else {
                374_000
            }
        }
        3 => {
            if n <= 10 {
                103_000
            } else if n <= 30 {
                218_000
            } else if n <= 60 {
                378_000
            } else {
                605_000
            }
        }
        4 => {
            if n <= 10 {
                137_000
            } else if n <= 30 {
                300_000
            } else if n <= 60 {
                531_000
            } else {
                827_000
            }
        }
        5 => {
            if n <= 10 {
                218_000
            } else if n <= 30 {
                492_000
            } else {
                879_000
            }
        }
        6 => {
            if n <= 10 {
                256_000
            } else if n <= 30 {
                535_000
            } else {
                921_000
            }
        }
        _ => return Err(error!(ZamaHostError::HcuUnknownCost)),
    };
    Ok(cost)
}

pub(super) fn enforce_le(used: u64, limit: u64, err: ZamaHostError) -> Result<()> {
    if limit != u64::MAX && used > limit {
        return Err(error!(err));
    }
    Ok(())
}

pub(super) fn accumulate_total(running: u64, step_hcu: u64) -> Result<u64> {
    running
        .checked_add(step_hcu)
        .ok_or_else(|| error!(ZamaHostError::HcuTransactionLimitExceeded))
}

pub(super) fn step_depth(step_hcu: u64, max_input_depth: u64) -> Result<u64> {
    step_hcu
        .checked_add(max_input_depth)
        .ok_or_else(|| error!(ZamaHostError::HcuTransactionDepthLimitExceeded))
}

#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub(super) struct ExecutionMeter {
    pub total: u64,
    pub step_depths: Vec<u64>,
}

fn operand_depth(operand: &FheExecuteOperand, step_depths: &[u64]) -> u64 {
    match operand {
        FheExecuteOperand::EarlierStep { producer_index } => step_depths
            .get(*producer_index as usize)
            .copied()
            .unwrap_or(0),
        FheExecuteOperand::StoredValue { .. } => 0,
        FheExecuteOperand::VerifiedInput { .. } => 0,
        FheExecuteOperand::Scalar { .. } => 0,
    }
}

fn operand_pricing_type(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced_types: &[u8],
) -> Result<u8> {
    match operand {
        FheExecuteOperand::EarlierStep { producer_index } => produced_types
            .get(*producer_index as usize)
            .copied()
            .ok_or_else(|| error!(ZamaHostError::FheExecuteEarlierStepMissing)),
        FheExecuteOperand::StoredValue { handle_index, .. } => {
            let handle = dictionary
                .get(*handle_index as usize)
                .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))?;
            Ok(handle_fhe_type(*handle))
        }
        FheExecuteOperand::VerifiedInput { attestation } => {
            Ok(handle_fhe_type(attestation.input_handle))
        }
        FheExecuteOperand::Scalar { .. } => Err(error!(ZamaHostError::HcuUnknownCost)),
    }
}

/// Meters an execution: sums per-step costs into an execution total and computes each step's
/// critical-path depth, enforcing both caps after every step (`u64::MAX = unlimited`). Comparisons
/// price on operand width, resolved from the dictionary / attestation handle / earlier produced
/// types.
pub(super) fn meter_execution(
    steps: &[FheExecuteStep],
    dictionary: &[[u8; 32]],
    max_hcu_per_tx: u64,
    max_hcu_depth_per_tx: u64,
) -> Result<ExecutionMeter> {
    let mut total: u64 = 0;
    let mut step_depths: Vec<u64> = Vec::with_capacity(steps.len());
    let mut produced_types: Vec<u8> = Vec::with_capacity(steps.len());

    for step in steps {
        let (op_hcu, max_input_depth, produced_type) = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let pricing_type = if is_comparison(*op) {
                    operand_pricing_type(lhs, dictionary, &produced_types)?
                } else {
                    *output_fhe_type
                };
                let cost = binary_op_hcu(
                    *op,
                    pricing_type,
                    matches!(rhs, FheExecuteOperand::Scalar { .. }),
                )?;
                let depth = operand_depth(lhs, &step_depths).max(operand_depth(rhs, &step_depths));
                (cost, depth, *output_fhe_type)
            }
            FheExecuteStep::Ternary {
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
                (cost, depth, *output_fhe_type)
            }
            FheExecuteStep::TrivialEncrypt { fhe_type, .. } => {
                (trivial_encrypt_hcu(*fhe_type)?, 0, *fhe_type)
            }
            FheExecuteStep::Rand { fhe_type, .. } => (rand_hcu(*fhe_type)?, 0, *fhe_type),
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                ..
            } => {
                let cost = unary_op_hcu(*op, *output_fhe_type)?;
                let depth = operand_depth(operand, &step_depths);
                (cost, depth, *output_fhe_type)
            }
            FheExecuteStep::RandBounded { fhe_type, .. } => (rand_hcu(*fhe_type)?, 0, *fhe_type),
            FheExecuteStep::Sum {
                operands, fhe_type, ..
            } => {
                let cost = sum_hcu(*fhe_type, operands.len())?;
                let depth = operands
                    .iter()
                    .map(|operand| operand_depth(operand, &step_depths))
                    .max()
                    .unwrap_or(0);
                (cost, depth, *fhe_type)
            }
            FheExecuteStep::IsIn {
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
                (cost, depth, 0)
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                output_fhe_type,
                ..
            } => {
                let cost = mul_div_hcu(
                    *output_fhe_type,
                    matches!(factor2, FheExecuteOperand::Scalar { .. }),
                )?;
                let depth =
                    operand_depth(factor1, &step_depths).max(operand_depth(factor2, &step_depths));
                (cost, depth, *output_fhe_type)
            }
        };

        total = accumulate_total(total, op_hcu)?;
        enforce_le(
            total,
            max_hcu_per_tx,
            ZamaHostError::HcuTransactionLimitExceeded,
        )?;

        let depth = step_depth(op_hcu, max_input_depth)?;
        enforce_le(
            depth,
            max_hcu_depth_per_tx,
            ZamaHostError::HcuTransactionDepthLimitExceeded,
        )?;

        step_depths.push(depth);
        produced_types.push(produced_type);
    }

    Ok(ExecutionMeter { total, step_depths })
}

#[cfg(test)]
mod tests {
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
                            let dropped = ty == 7
                                || ty == 8
                                || operand_ty == 7
                                || operand_ty == 8;
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
                            if dropped {
                                continue;
                            }
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
        use super::super::assert_ternary_operand_types;
        for ty in ALL_FHE_TYPE_PROBES {
            let validated =
                assert_ternary_operand_types(handle_of(0), handle_of(ty), handle_of(ty), ty)
                    .is_ok();
            if validated && ty != 7 && ty != 8 {
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
            if assert_supported_fhe_type(ty).is_ok() && ty != 7 && ty != 8 {
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
            if assert_supported_rand_type(ty).is_ok() && ty != 8 {
                assert!(
                    rand_hcu(ty).is_ok(),
                    "validated rand type {ty} has no cost row"
                );
            }
            // RandBounded meters through the same rand_hcu table.
            if assert_supported_bounded_rand_type(ty).is_ok() && ty != 8 {
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
                if assert_sum_operand_types(&handles, ty).is_ok() && ty != 7 && ty != 8 {
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
                if assert_is_in_operand_types(handle_of(ty), &handles, ty).is_ok()
                    && ty != 7
                    && ty != 8
                {
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
                if validated && ty != 7 && ty != 8 {
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
}
