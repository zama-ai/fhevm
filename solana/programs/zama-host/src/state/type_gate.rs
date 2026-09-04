//! Type and operator legality for `fhe_execute`.
//!
//! One module owns “is this type/op legal”: supported FHE types (Bool + Uint8..Uint128,
//! `0 | 2..=6`), per-operator operand/output gates, rand, reductions, and ternary.

use super::{handle_fhe_type, FheBinaryOpCode, FheUnaryOpCode};
use crate::errors::ZamaHostError;
use anchor_lang::prelude::*;

pub fn assert_supported_fhe_type(fhe_type: u8) -> Result<()> {
    require!(
        is_supported_fhe_type(fhe_type),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

/// Checks that a binary operation's declared result type matches the shipped operator.
fn assert_supported_binary_output_type(op: FheBinaryOpCode, fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)?;
    let valid = match op {
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::Div
        | FheBinaryOpCode::Rem
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => matches!(fhe_type, 2..=6),
        // Bitwise: Bool + Uint8..Uint128. Solana host max is euint128.
        FheBinaryOpCode::And | FheBinaryOpCode::Or | FheBinaryOpCode::Xor => {
            matches!(fhe_type, 0 | 2..=6)
        }
        // Shifts/rotations: Uint8..Uint128. Solana host max is euint128.
        FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr => matches!(fhe_type, 2..=6),
        FheBinaryOpCode::Eq
        | FheBinaryOpCode::Ne
        | FheBinaryOpCode::Ge
        | FheBinaryOpCode::Gt
        | FheBinaryOpCode::Le
        | FheBinaryOpCode::Lt => fhe_type == 0,
    };
    require!(valid, ZamaHostError::UnsupportedFheType);
    Ok(())
}

/// Checks binary operand metadata against the EVM executor's type discipline.
pub fn assert_binary_operand_types(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_binary_output_type(op, output_fhe_type)?;
    let lhs_type = handle_fhe_type(lhs);
    match op {
        // Comparisons produce `ebool`, so the operand type is gated here: Eq/Ne accept Bool and
        // Uint8..Uint128; ordered comparisons accept Uint8..Uint128. Solana host max is euint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            require!(
                matches!(lhs_type, 0 | 2..=6),
                ZamaHostError::UnsupportedFheType
            );
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            require!(matches!(lhs_type, 2..=6), ZamaHostError::UnsupportedFheType);
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            require!(
                lhs_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
            require!(scalar, ZamaHostError::DivisorMustBeScalar);
            require!(
                !scalar_is_zero_for_type(rhs, lhs_type),
                ZamaHostError::DivisionByZero
            );
        }
        // Remaining ops: the operand type must equal the (op-gated) output type.
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::And
        | FheBinaryOpCode::Or
        | FheBinaryOpCode::Xor
        | FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => {
            require!(
                lhs_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
    }
    if !scalar {
        require!(
            handle_fhe_type(rhs) == lhs_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

pub fn assert_supported_rand_type(fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)
}

pub(crate) fn assert_supported_bounded_rand_type(fhe_type: u8) -> Result<()> {
    require!(
        bounded_rand_type_bits(fhe_type).is_some(),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

pub fn assert_valid_bounded_rand_upper_bound(upper_bound: [u8; 32], fhe_type: u8) -> Result<()> {
    assert_supported_bounded_rand_type(fhe_type)?;
    let bit_index =
        power_of_two_bit_index(upper_bound).ok_or(ZamaHostError::InvalidRandomUpperBound)?;
    if let Some(max_bits) = bounded_rand_type_bits(fhe_type).flatten() {
        require!(
            bit_index <= max_bits,
            ZamaHostError::InvalidRandomUpperBound
        );
    }
    Ok(())
}

fn assert_supported_unary_output_type(op: FheUnaryOpCode, fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)?;
    let valid = match op {
        FheUnaryOpCode::Neg => matches!(fhe_type, 2..=6),
        FheUnaryOpCode::Not => matches!(fhe_type, 0 | 2..=6),
        // Cast output set: Uint8..Uint128 (no ebool, no eaddress/Uint160). Solana host max is euint128.
        FheUnaryOpCode::Cast => matches!(fhe_type, 2..=6),
    };
    require!(valid, ZamaHostError::UnsupportedFheType);
    Ok(())
}

pub fn assert_unary_operand_type(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_unary_output_type(op, output_fhe_type)?;
    let operand_type = handle_fhe_type(operand);
    require!(
        is_supported_fhe_type(operand_type),
        ZamaHostError::UnsupportedFheType
    );
    match op {
        FheUnaryOpCode::Neg => {
            require!(
                matches!(operand_type, 2..=6),
                ZamaHostError::UnsupportedFheType
            );
            require!(
                operand_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
        FheUnaryOpCode::Not => {
            require!(
                matches!(operand_type, 0 | 2..=6),
                ZamaHostError::UnsupportedFheType
            );
            require!(
                operand_type == output_fhe_type,
                ZamaHostError::BinaryOperandTypeMismatch
            );
        }
        FheUnaryOpCode::Cast => {
            // Cast input set: Bool | Uint8..Uint128 (no eaddress/Uint160). Solana host max is euint128.
            require!(
                matches!(operand_type, 0 | 2..=6),
                ZamaHostError::UnsupportedFheType
            );
            // Cast reinterprets to a different type; a same-type cast is rejected (EVM InvalidType).
            require!(
                operand_type != output_fhe_type,
                ZamaHostError::UnsupportedFheType
            );
        }
    }
    Ok(())
}

/// Requires every operand's resolved handle type to equal the declared uint type (2..=6). Like EVM
/// `fheSum` and the coprocessor, only the maximum operand count is bounded — a zero/single-operand
/// sum is valid (EVM enforces no minimum).
pub fn assert_sum_operand_types(operand_handles: &[[u8; 32]], fhe_type: u8) -> Result<()> {
    require!(matches!(fhe_type, 2..=6), ZamaHostError::UnsupportedFheType);
    // Cap the operand count at the coprocessor's FheSum limit (transient operands use no accounts).
    assert_reduction_count(operand_handles.len(), fhe_type)?;
    for handle in operand_handles {
        require!(
            handle_fhe_type(*handle) == fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

/// Requires the value and every set member to share the declared uint type (Uint8..Uint128, 2..=6) —
/// Solana host max is euint128; `ebool` is excluded. Like EVM, only the maximum set size is
/// bounded — an empty set is valid (membership is trivially false).
pub fn assert_is_in_operand_types(
    value_handle: [u8; 32],
    set_handles: &[[u8; 32]],
    fhe_type: u8,
) -> Result<()> {
    require!(matches!(fhe_type, 2..=6), ZamaHostError::UnsupportedFheType);
    // Cap the set size at the coprocessor's FheIsIn limit (its `set_size` bound excludes the value).
    assert_reduction_count(set_handles.len(), fhe_type)?;
    require!(
        handle_fhe_type(value_handle) == fhe_type,
        ZamaHostError::BinaryOperandTypeMismatch
    );
    for handle in set_handles {
        require!(
            handle_fhe_type(*handle) == fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

/// MulDiv: factor1 is an encrypted uint8..uint64 (EVM + coprocessor cap at Uint64); factor2 is
/// either an encrypted operand of the same type or a plaintext scalar; divisor is an always-scalar
/// plaintext that must be non-zero (EVM DivisionByZero parity).
pub fn assert_mul_div_operand_types(
    factor1: [u8; 32],
    factor2: [u8; 32],
    factor2_scalar: bool,
    divisor: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    require!(
        matches!(output_fhe_type, 2..=5),
        ZamaHostError::UnsupportedFheType
    );
    require!(
        handle_fhe_type(factor1) == output_fhe_type,
        ZamaHostError::BinaryOperandTypeMismatch
    );
    if !factor2_scalar {
        require!(
            handle_fhe_type(factor2) == output_fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    // Divisor must be non-zero once truncated to the operand type (EVM parity).
    require!(
        !scalar_is_zero_for_type(divisor, output_fhe_type),
        ZamaHostError::MulDivDivisorZero
    );
    Ok(())
}

/// Checks ternary operand metadata against the declared result type.
pub fn assert_ternary_operand_types(
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_fhe_type(output_fhe_type)?;
    require!(
        handle_fhe_type(control) == 0
            && handle_fhe_type(if_true) == output_fhe_type
            && handle_fhe_type(if_false) == output_fhe_type,
        ZamaHostError::InvalidInputHandleType
    );
    Ok(())
}

pub(crate) fn is_supported_fhe_type(fhe_type: u8) -> bool {
    matches!(fhe_type, 0 | 2..=6)
}

/// Whether a big-endian scalar is zero once truncated to `fhe_type`'s width (EVM `_isScalarZeroForType`).
fn scalar_is_zero_for_type(scalar: [u8; 32], fhe_type: u8) -> bool {
    let width = match fhe_type {
        2 => 1,  // Uint8
        3 => 2,  // Uint16
        4 => 4,  // Uint32
        5 => 8,  // Uint64
        6 => 16, // Uint128
        _ => 32, // unsupported for division: fall back to the whole buffer (fail closed)
    };
    scalar[32 - width..].iter().all(|byte| *byte == 0)
}

/// Coprocessor FheSum/FheIsIn max operand count: 100 for narrow types (Uint8..Uint32), 60 for wider.
pub fn max_reduction_operands(fhe_type: u8) -> usize {
    match fhe_type {
        2..=4 => 100,
        _ => 60,
    }
}

pub(crate) fn assert_reduction_count(n: usize, ty: u8) -> Result<()> {
    require!(
        n <= max_reduction_operands(ty),
        ZamaHostError::InvalidFheExecuteAccount
    );
    Ok(())
}

fn bounded_rand_type_bits(fhe_type: u8) -> Option<Option<u16>> {
    match fhe_type {
        2 => Some(Some(8)),
        3 => Some(Some(16)),
        4 => Some(Some(32)),
        5 => Some(Some(64)),
        6 => Some(Some(128)),
        _ => None,
    }
}

fn power_of_two_bit_index(value: [u8; 32]) -> Option<u16> {
    let mut bit_index = None;
    for (byte_index, byte) in value.iter().enumerate() {
        if *byte == 0 {
            continue;
        }
        if byte.count_ones() != 1 || bit_index.is_some() {
            return None;
        }
        let bit_in_byte = 7 - byte.leading_zeros() as u16;
        bit_index = Some(((31 - byte_index) as u16 * 8) + bit_in_byte);
    }
    bit_index
}
