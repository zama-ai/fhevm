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
//! (`euint160`) and 8 (`euint256`) have no rows. Caps stay `u64::MAX` (off). A cell of `0` is the
//! unknown-cost sentinel — no shipped cost is 0 (Not/ebool is 2; trivial/cast is 32).

use anchor_lang::prelude::*;

use crate::errors::ZamaHostError;
use crate::state::{
    handle_fhe_type, FheBinaryOpCode, FheExecuteOperand, FheExecuteStep, FheTernaryOpCode,
    FheUnaryOpCode,
};

#[cfg(test)]
mod tests;

/// `fhe_type` 0..=6. Index 1 is unused (no euint4).
const N: usize = 7;

struct BinaryCosts {
    scalar: [u32; N],
    ciphertext: [u32; N],
}

struct ReductionCosts {
    n10: [u32; N],
    n30: [u32; N],
    n60: [u32; N],
    rest: [u32; N],
}

fn lookup(row: &[u32; N], ty: u8) -> Result<u64> {
    let cost = *row
        .get(ty as usize)
        .ok_or_else(|| error!(ZamaHostError::HcuUnknownCost))?;
    if cost == 0 {
        return Err(error!(ZamaHostError::HcuUnknownCost));
    }
    Ok(u64::from(cost))
}

fn binary_lookup(costs: &BinaryCosts, ty: u8, scalar: bool) -> Result<u64> {
    lookup(
        if scalar {
            &costs.scalar
        } else {
            &costs.ciphertext
        },
        ty,
    )
}

fn reduction_lookup(costs: &ReductionCosts, ty: u8, n: usize) -> Result<u64> {
    let row = if n <= 10 {
        &costs.n10
    } else if n <= 30 {
        &costs.n30
    } else if n <= 60 {
        &costs.n60
    } else {
        &costs.rest
    };
    lookup(row, ty)
}

// Rows indexed by fhe_type 0..=6. Zero cells are unknown (not a shipped cost).
const ADD: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 84_000, 93_000, 95_000, 133_000, 172_000],
    ciphertext: [0, 0, 88_000, 93_000, 125_000, 162_000, 259_000],
};
const SUB: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 84_000, 93_000, 95_000, 133_000, 172_000],
    ciphertext: [0, 0, 91_000, 93_000, 125_000, 162_000, 260_000],
};
const MUL: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 122_000, 193_000, 265_000, 365_000, 696_000],
    ciphertext: [0, 0, 150_000, 222_000, 328_000, 596_000, 1_686_000],
};
const DIV: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 210_000, 302_000, 438_000, 715_000, 1_225_000],
    ciphertext: [0, 0, 0, 0, 0, 0, 0],
};
const REM: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 440_000, 580_000, 792_000, 1_153_000, 1_943_000],
    ciphertext: [0, 0, 0, 0, 0, 0, 0],
};
const AND: BinaryCosts = BinaryCosts {
    scalar: [22_000, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
    ciphertext: [25_000, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
};
const OR: BinaryCosts = BinaryCosts {
    scalar: [22_000, 0, 30_000, 30_000, 32_000, 34_000, 37_000],
    ciphertext: [24_000, 0, 30_000, 31_000, 32_000, 34_000, 37_000],
};
const XOR: BinaryCosts = BinaryCosts {
    scalar: [22_000, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
    ciphertext: [22_000, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
};
const SHL: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 32_000, 32_000, 32_000, 34_000, 37_000],
    ciphertext: [0, 0, 92_000, 125_000, 162_000, 208_000, 272_000],
};
const SHR: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 32_000, 32_000, 32_000, 34_000, 37_000],
    ciphertext: [0, 0, 91_000, 123_000, 163_000, 209_000, 272_000],
};
const ROTL: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
    ciphertext: [0, 0, 91_000, 125_000, 163_000, 209_000, 278_000],
};
const ROTR: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 31_000, 31_000, 32_000, 34_000, 37_000],
    ciphertext: [0, 0, 93_000, 125_000, 160_000, 209_000, 283_000],
};
const EQ: BinaryCosts = BinaryCosts {
    scalar: [25_000, 0, 55_000, 55_000, 82_000, 83_000, 117_000],
    ciphertext: [26_000, 0, 55_000, 83_000, 86_000, 120_000, 122_000],
};
const NE: BinaryCosts = BinaryCosts {
    scalar: [23_000, 0, 55_000, 55_000, 83_000, 84_000, 117_000],
    ciphertext: [23_000, 0, 55_000, 83_000, 85_000, 118_000, 122_000],
};
const GE: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 52_000, 55_000, 84_000, 116_000, 149_000],
    ciphertext: [0, 0, 63_000, 84_000, 118_000, 152_000, 210_000],
};
const GT: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 52_000, 55_000, 84_000, 117_000, 150_000],
    ciphertext: [0, 0, 59_000, 84_000, 118_000, 152_000, 218_000],
};
const LE: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 58_000, 58_000, 84_000, 119_000, 150_000],
    ciphertext: [0, 0, 58_000, 83_000, 117_000, 149_000, 218_000],
};
const LT: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 52_000, 58_000, 83_000, 118_000, 149_000],
    ciphertext: [0, 0, 59_000, 84_000, 117_000, 146_000, 215_000],
};
const MIN: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 84_000, 88_000, 117_000, 150_000, 186_000],
    ciphertext: [0, 0, 119_000, 146_000, 182_000, 219_000, 289_000],
};
const MAX: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 89_000, 89_000, 117_000, 149_000, 180_000],
    ciphertext: [0, 0, 121_000, 145_000, 180_000, 218_000, 290_000],
};
const MUL_DIV: BinaryCosts = BinaryCosts {
    scalar: [0, 0, 495_000, 703_000, 1_080_000, 1_921_000, 0],
    ciphertext: [0, 0, 524_000, 766_000, 1_311_000, 2_911_000, 0],
};

const SELECT: [u32; N] = [55_000, 0, 55_000, 55_000, 55_000, 55_000, 57_000];
const NEG: [u32; N] = [0, 0, 79_000, 93_000, 95_000, 131_000, 168_000];
const NOT: [u32; N] = [2, 0, 9, 16, 32, 63, 130];
const CAST: [u32; N] = [32, 0, 32, 32, 32, 32, 32];
const TRIVIAL: [u32; N] = [32, 0, 32, 32, 32, 32, 32];
const RAND: [u32; N] = [19_000, 0, 23_000, 23_000, 24_000, 24_000, 25_000];
// `checkHCUForFheRandBounded` has no ebool row.
const RAND_BOUNDED: [u32; N] = [0, 0, 23_000, 23_000, 24_000, 24_000, 25_000];

// Types 5 and 6 share the last two buckets (n>30 is one cost; no distinct n<=60 vs else).
const SUM: ReductionCosts = ReductionCosts {
    n10: [0, 0, 90_900, 95_000, 116_000, 139_000, 219_000],
    n30: [0, 0, 127_000, 136_000, 164_000, 216_000, 355_000],
    n60: [0, 0, 148_000, 162_000, 205_000, 306_000, 552_000],
    rest: [0, 0, 159_000, 184_000, 281_000, 306_000, 552_000],
};
const IS_IN: ReductionCosts = ReductionCosts {
    n10: [0, 0, 71_300, 103_000, 137_000, 218_000, 256_000],
    n30: [0, 0, 148_000, 218_000, 300_000, 492_000, 535_000],
    n60: [0, 0, 247_000, 378_000, 531_000, 879_000, 921_000],
    rest: [0, 0, 374_000, 605_000, 827_000, 879_000, 921_000],
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
    let costs = match op {
        FheBinaryOpCode::Add => &ADD,
        FheBinaryOpCode::Sub => &SUB,
        FheBinaryOpCode::Mul => &MUL,
        FheBinaryOpCode::Div => &DIV,
        FheBinaryOpCode::Rem => &REM,
        FheBinaryOpCode::And => &AND,
        FheBinaryOpCode::Or => &OR,
        FheBinaryOpCode::Xor => &XOR,
        FheBinaryOpCode::Shl => &SHL,
        FheBinaryOpCode::Shr => &SHR,
        FheBinaryOpCode::Rotl => &ROTL,
        FheBinaryOpCode::Rotr => &ROTR,
        FheBinaryOpCode::Eq => &EQ,
        FheBinaryOpCode::Ne => &NE,
        FheBinaryOpCode::Ge => &GE,
        FheBinaryOpCode::Gt => &GT,
        FheBinaryOpCode::Le => &LE,
        FheBinaryOpCode::Lt => &LT,
        FheBinaryOpCode::Min => &MIN,
        FheBinaryOpCode::Max => &MAX,
    };
    binary_lookup(costs, fhe_type, scalar)
}

pub(super) fn unary_op_hcu(op: FheUnaryOpCode, fhe_type: u8) -> Result<u64> {
    match op {
        FheUnaryOpCode::Neg => lookup(&NEG, fhe_type),
        FheUnaryOpCode::Not => lookup(&NOT, fhe_type),
        FheUnaryOpCode::Cast => lookup(&CAST, fhe_type),
    }
}

pub(super) fn ternary_op_hcu(op: FheTernaryOpCode, fhe_type: u8) -> Result<u64> {
    match op {
        FheTernaryOpCode::IfThenElse => lookup(&SELECT, fhe_type),
    }
}

pub(super) fn trivial_encrypt_hcu(fhe_type: u8) -> Result<u64> {
    lookup(&TRIVIAL, fhe_type)
}

pub(super) fn rand_hcu(fhe_type: u8) -> Result<u64> {
    lookup(&RAND, fhe_type)
}

pub(super) fn rand_bounded_hcu(fhe_type: u8) -> Result<u64> {
    lookup(&RAND_BOUNDED, fhe_type)
}

pub(super) fn mul_div_hcu(fhe_type: u8, factor2_scalar: bool) -> Result<u64> {
    binary_lookup(&MUL_DIV, fhe_type, factor2_scalar)
}

pub(super) fn sum_hcu(fhe_type: u8, operand_count: usize) -> Result<u64> {
    reduction_lookup(&SUM, fhe_type, operand_count)
}

pub(super) fn is_in_hcu(fhe_type: u8, set_len: usize) -> Result<u64> {
    reduction_lookup(&IS_IN, fhe_type, set_len)
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
            FheExecuteStep::RandBounded { fhe_type, .. } => {
                (rand_bounded_hcu(*fhe_type)?, 0, *fhe_type)
            }
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
