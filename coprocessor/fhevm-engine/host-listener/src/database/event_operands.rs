//! Pure application event metadata, shared by ingestion and application-only fuzz targets.
use crate::contracts::TfheContract;
use crate::contracts::TfheContract::TfheContractEvents;
use alloy_primitives::FixedBytes;

type Handle = FixedBytes<32>;
type ScalarByte = FixedBytes<1>;
type OperandBoundaryMask = [u8; 32];
const OPERAND_BOUNDARY_MASK_BYTES: usize = 32;

/// Successful executor events use the same scalar modes admitted by the
/// contract: binary flags are Boolean, division/remainder require a scalar
/// divisor, and MulDiv uses bitmasks 0x01 or 0x03. Refuse ABI-valid but
/// semantically impossible flags before writing any block state.
pub fn validate_scalar_mode(
    op: &TfheContractEvents,
) -> Result<(), &'static str> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::FheDiv(C::FheDiv { scalarByte, .. })
        | E::FheRem(C::FheRem { scalarByte, .. }) => {
            if scalarByte.0[0] == 1 {
                Ok(())
            } else {
                Err("division/remainder event requires scalarByte 0x01")
            }
        }
        E::FheMulDiv(C::FheMulDiv { scalarByte, .. }) => {
            if matches!(scalarByte.0[0], 1 | 3) {
                Ok(())
            } else {
                Err("MulDiv event requires scalarByte 0x01 or 0x03")
            }
        }
        E::FheAdd(C::FheAdd { scalarByte, .. })
        | E::FheSub(C::FheSub { scalarByte, .. })
        | E::FheMul(C::FheMul { scalarByte, .. })
        | E::FheBitAnd(C::FheBitAnd { scalarByte, .. })
        | E::FheBitOr(C::FheBitOr { scalarByte, .. })
        | E::FheBitXor(C::FheBitXor { scalarByte, .. })
        | E::FheShl(C::FheShl { scalarByte, .. })
        | E::FheShr(C::FheShr { scalarByte, .. })
        | E::FheRotl(C::FheRotl { scalarByte, .. })
        | E::FheRotr(C::FheRotr { scalarByte, .. })
        | E::FheEq(C::FheEq { scalarByte, .. })
        | E::FheNe(C::FheNe { scalarByte, .. })
        | E::FheGe(C::FheGe { scalarByte, .. })
        | E::FheGt(C::FheGt { scalarByte, .. })
        | E::FheLe(C::FheLe { scalarByte, .. })
        | E::FheLt(C::FheLt { scalarByte, .. })
        | E::FheMin(C::FheMin { scalarByte, .. })
        | E::FheMax(C::FheMax { scalarByte, .. }) => {
            if scalarByte.0[0] <= 1 {
                Ok(())
            } else {
                Err("binary event scalarByte is not Boolean")
            }
        }
        _ => Ok(()),
    }
}

pub fn tfhe_inputs_handle(op: &TfheContractEvents) -> Vec<Handle> {
    use TfheContract as C;
    use TfheContractEvents as E;
    match op {
        E::Cast(C::Cast { ct, .. })
        | E::FheNeg(C::FheNeg { ct, .. })
        | E::FheNot(C::FheNot { ct, .. }) => vec![*ct],

        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => {
            if scalarByte.const_is_zero() {
                vec![*lhs, *rhs]
            } else {
                vec![*lhs]
            }
        }

        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            ..
        }) => {
            vec![*control, *ifTrue, *ifFalse]
        }

        E::FheRand(_) | E::FheRandBounded(_) | E::TrivialEncrypt(_) => vec![],

        E::FheSum(C::FheSum { values, .. }) => values.clone(),

        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let mut handles = vec![*value];
            handles.extend(values.iter().copied());
            handles
        }

        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            ..
        }) => {
            if fhe_mul_div_factor2_is_scalar(scalarByte) {
                vec![*factor1]
            } else {
                vec![*factor1, *factor2]
            }
        }

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}

/// Returns every encrypted executor operand together with its position in the
/// `dependencies` vector stored for the computation. The position is also the
/// bit position used by `FHEVMExecutor`'s `boundaryBits` preimage. Plaintext
/// scalar positions are intentionally absent, and therefore retain their
/// required zero bit.
///
/// Keep this in lock-step with both `insert_tfhe_event` and the executor's
/// `_oneOperandBoundaryBit` callers. In particular, `FheMulDiv` always has a
/// scalar divisor at position 2, and binary scalar operations have a scalar
/// right operand at position 1.
pub fn tfhe_encrypted_operand_positions(
    op: &TfheContractEvents,
) -> Vec<(usize, Handle)> {
    use TfheContract as C;
    use TfheContractEvents as E;

    match op {
        E::Cast(C::Cast { ct, .. })
        | E::FheNeg(C::FheNeg { ct, .. })
        | E::FheNot(C::FheNot { ct, .. }) => vec![(0, *ct)],

        E::FheAdd(C::FheAdd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitAnd(C::FheBitAnd {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitOr(C::FheBitOr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheBitXor(C::FheBitXor {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheDiv(C::FheDiv {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMax(C::FheMax {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMin(C::FheMin {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheMul(C::FheMul {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRem(C::FheRem {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotl(C::FheRotl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheRotr(C::FheRotr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShl(C::FheShl {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheShr(C::FheShr {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheSub(C::FheSub {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheEq(C::FheEq {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGe(C::FheGe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheGt(C::FheGt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLe(C::FheLe {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheLt(C::FheLt {
            lhs,
            rhs,
            scalarByte,
            ..
        })
        | E::FheNe(C::FheNe {
            lhs,
            rhs,
            scalarByte,
            ..
        }) => {
            let mut operands = vec![(0, *lhs)];
            if scalarByte.const_is_zero() {
                operands.push((1, *rhs));
            }
            operands
        }

        E::FheIfThenElse(C::FheIfThenElse {
            control,
            ifTrue,
            ifFalse,
            ..
        }) => vec![(0, *control), (1, *ifTrue), (2, *ifFalse)],

        E::FheRand(_) | E::FheRandBounded(_) | E::TrivialEncrypt(_) => {
            vec![]
        }

        E::FheSum(C::FheSum { values, .. }) => {
            values.iter().copied().enumerate().collect()
        }

        E::FheIsIn(C::FheIsIn { value, values, .. }) => {
            let mut operands =
                Vec::with_capacity(values.len().saturating_add(1));
            operands.push((0, *value));
            operands.extend(
                values
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, value)| (i.saturating_add(1), value)),
            );
            operands
        }

        E::FheMulDiv(C::FheMulDiv {
            factor1,
            factor2,
            scalarByte,
            ..
        }) => {
            let mut operands = vec![(0, *factor1)];
            if !fhe_mul_div_factor2_is_scalar(scalarByte) {
                operands.push((1, *factor2));
            }
            operands
        }

        E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => vec![],
    }
}

/// Reconstructs the exact `uint256 boundaryBits` value committed by the
/// executor for one operation. `was_minted` must represent successful prior
/// executor operations in this transaction, in log order.
///
/// The executor only has 256 bits of provenance. Current collection events
/// are bounded below that limit; reject a larger event rather than silently
/// inventing a sourcing rule that its handle does not commit to.
pub fn operand_boundary_mask_from_minted<F>(
    op: &TfheContractEvents,
    mut was_minted: F,
) -> Result<OperandBoundaryMask, String>
where
    F: FnMut(&Handle) -> bool,
{
    let mut mask = [0_u8; OPERAND_BOUNDARY_MASK_BYTES];
    for (position, handle) in tfhe_encrypted_operand_positions(op) {
        if position >= OPERAND_BOUNDARY_MASK_BYTES * 8 {
            return Err(format!(
                "operation has encrypted operand at position {position}, beyond executor boundaryBits"
            ));
        }
        if !was_minted(&handle) {
            // Match `uint256` ABI byte order: Solidity bit 0 is the least
            // significant bit of the final byte.
            let byte_index = OPERAND_BOUNDARY_MASK_BYTES - 1 - position / 8;
            mask[byte_index] |= 1 << (position % 8);
        }
    }
    Ok(mask)
}

/// `fheMulDiv` `scalarByte` bit 1 — factor2 is a plaintext scalar (bit 0 is the
/// always-scalar divisor).
pub(crate) fn fhe_mul_div_factor2_is_scalar(scalar_byte: &ScalarByte) -> bool {
    scalar_byte.0[0] & 0b10 != 0
}

#[cfg(test)]
mod scalar_mode_tests {
    use super::*;
    use alloy_primitives::Address;
    #[test]
    fn executor_scalar_flags_match_contract_modes() {
        use TfheContract as C;
        use TfheContractEvents as E;
        for flag in [0, 1, 2, 3, 128, 255] {
            let caller = Address::ZERO;
            let lhs = Handle::ZERO;
            let rhs = Handle::ZERO;
            let result = Handle::ZERO;
            let scalar_byte = FixedBytes([flag]);
            let add = E::FheAdd(C::FheAdd { caller, lhs, rhs, result, scalarByte: scalar_byte });
            let div = E::FheDiv(C::FheDiv { caller, lhs, rhs, result, scalarByte: scalar_byte });
            let rem = E::FheRem(C::FheRem { caller, lhs, rhs, result, scalarByte: scalar_byte });
            let mul_div = E::FheMulDiv(C::FheMulDiv { caller, factor1: lhs, factor2: rhs,
                divisor: rhs, result, scalarByte: scalar_byte });
            assert_eq!(validate_scalar_mode(&add).is_ok(), flag <= 1);
            assert_eq!(validate_scalar_mode(&div).is_ok(), flag == 1);
            assert_eq!(validate_scalar_mode(&rem).is_ok(), flag == 1);
            assert_eq!(validate_scalar_mode(&mul_div).is_ok(), flag == 1 || flag == 3);
        }
    }
}
