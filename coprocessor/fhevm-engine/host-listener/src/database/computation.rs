//! The listener's computation payload, shared by native chain decoders,
//! dependency analysis and database encoding.
use alloy_primitives::B256 as Handle;

use crate::contracts::{
    TfheContract as C, TfheContract::TfheContractEvents as E,
};
use fhevm_engine_common::types::{
    is_valid_multi_output_arity, FheOperationType, SupportedFheOperations,
    MAX_MULTI_OUTPUT_ARITY,
};

/// Big-endian executor boundaryBits; operand i is bit i, including clear positions.
pub type OperandBoundaryMask = [u8; 32];
pub const OPERAND_BOUNDARY_MASK_BYTES: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Encrypted(Handle),
    Clear(Vec<u8>),
}

impl Operand {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Encrypted(handle) => handle.to_vec(),
            Self::Clear(bytes) => bytes.clone(),
        }
    }

    pub fn encrypted(handle: [u8; 32]) -> Self {
        Self::Encrypted(handle.into())
    }

    pub fn binary_rhs(value: Handle, scalar: bool) -> Self {
        if scalar {
            Self::Clear(value.to_vec())
        } else {
            Self::Encrypted(value)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Computation {
    operation: SupportedFheOperations,
    operands: Vec<Operand>,
    /// Ordered outputs; their first handle identifies a multi-output group.
    outputs: Vec<Handle>,
}

impl Computation {
    /// Validates the listener's wire representation. Ciphertext types and
    /// operation semantics remain the executor/worker's responsibility.
    pub fn new(
        operation: SupportedFheOperations,
        operands: Vec<Operand>,
        outputs: Vec<Handle>,
    ) -> Result<Self, String> {
        use Operand::{Clear as P, Encrypted as H};
        use SupportedFheOperations as O;
        let word = |operand: &Operand| match operand {
            H(_) => true,
            P(bytes) => bytes.len() == 32,
        };
        let valid = match (operation.op_type(), operands.as_slice()) {
            (FheOperationType::Binary, [H(_), rhs]) => word(rhs),
            (FheOperationType::Unary, [H(_)]) => true,
            (FheOperationType::Other, inputs) => match (operation, inputs) {
                (O::FheCast, [H(_), P(kind)]) => kind.len() == 1,
                (O::FheTrivialEncrypt, [P(value), P(kind)]) => {
                    value.len() == 32 && kind.len() == 1
                }
                (O::FheIfThenElse, [H(_), H(_), H(_)]) => true,
                (O::FheRand, [P(seed), P(kind)]) => {
                    seed.len() == 16 && kind.len() == 1
                }
                (O::FheRandBounded, [P(seed), P(bound), P(kind)]) => {
                    seed.len() == 16 && bound.len() == 32 && kind.len() == 1
                }
                (O::FheSum | O::FheIsIn, inputs) => {
                    (operation == O::FheSum || !inputs.is_empty())
                        && inputs.len() <= OPERAND_BOUNDARY_MASK_BYTES * 8
                        && inputs.iter().all(|operand| matches!(operand, H(_)))
                }
                (O::FheMulDiv, [H(_), rhs, P(divisor)]) => {
                    word(rhs) && divisor.len() == 32
                }
                _ => false,
            },
            _ => false,
        };
        if !valid {
            return Err(format!(
                "invalid operand shape or byte widths for {operation:?}"
            ));
        }
        if !is_valid_multi_output_arity(outputs.len()) {
            return Err(format!(
                "unsupported computation output arity {} (maximum {MAX_MULTI_OUTPUT_ARITY})",
                outputs.len(),
            ));
        }
        Ok(Self {
            operation,
            operands,
            outputs,
        })
    }

    pub fn operation(&self) -> SupportedFheOperations {
        self.operation
    }

    pub fn operands(&self) -> &[Operand] {
        &self.operands
    }

    pub fn outputs(&self) -> &[Handle] {
        &self.outputs
    }

    pub fn trivial(plaintext: [u8; 32], fhe_type: u8, result: Handle) -> Self {
        Self::single(
            SupportedFheOperations::FheTrivialEncrypt,
            vec![
                Operand::Clear(plaintext.to_vec()),
                Operand::Clear(vec![fhe_type]),
            ],
            result,
        )
        .expect("fixed-width trivial-encryption operands")
    }

    pub fn single(
        operation: SupportedFheOperations,
        operands: Vec<Operand>,
        result: Handle,
    ) -> Result<Self, String> {
        Self::new(operation, operands, vec![result])
    }

    pub fn encrypted_operands(
        &self,
    ) -> impl Iterator<Item = (usize, Handle)> + '_ {
        self.operands
            .iter()
            .enumerate()
            .filter_map(|(position, operand)| match operand {
                Operand::Encrypted(handle) => Some((position, *handle)),
                Operand::Clear(_) => None,
            })
    }

    pub fn inputs(&self) -> Vec<Handle> {
        self.encrypted_operands()
            .map(|(_, handle)| handle)
            .collect()
    }

    /// The worker's legacy flag describes factor2 for MulDiv; its divisor
    /// is always clear regardless of this flag.
    pub fn is_scalar(&self) -> bool {
        if self.operation == SupportedFheOperations::FheMulDiv {
            matches!(self.operands.get(1), Some(Operand::Clear(_)))
        } else {
            self.operands
                .iter()
                .any(|operand| matches!(operand, Operand::Clear(_)))
        }
    }

    pub fn boundary_mask(
        &self,
        mut was_minted: impl FnMut(&Handle) -> bool,
    ) -> Result<OperandBoundaryMask, String> {
        let mut mask = [0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        for (position, handle) in self.encrypted_operands() {
            if position >= OPERAND_BOUNDARY_MASK_BYTES * 8 {
                return Err(format!("operation has encrypted operand at position {position}, beyond executor boundaryBits"));
            }
            if !was_minted(&handle) {
                mask[OPERAND_BOUNDARY_MASK_BYTES - 1 - position / 8] |=
                    1 << (position % 8);
            }
        }
        Ok(mask)
    }

    /// Decode computation semantics at the EVM boundary. The caller retains
    /// chain/transaction/log provenance; administrative events and input
    /// verification do not enqueue computations.
    pub fn from_evm(event: &E) -> Result<Option<Self>, String> {
        use Operand::{Clear as P, Encrypted as H};
        use SupportedFheOperations as O;
        let (operation, operands, result) = match event {
            E::FheAdd(C::FheAdd {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheAdd,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheSub(C::FheSub {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheSub,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheMul(C::FheMul {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheMul,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheDiv(C::FheDiv {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheDiv,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheRem(C::FheRem {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheRem,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheBitAnd(C::FheBitAnd {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheBitAnd,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheBitOr(C::FheBitOr {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheBitOr,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheBitXor(C::FheBitXor {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheBitXor,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheShl(C::FheShl {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheShl,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheShr(C::FheShr {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheShr,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheRotl(C::FheRotl {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheRotl,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheRotr(C::FheRotr {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheRotr,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheEq(C::FheEq {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheEq,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheNe(C::FheNe {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheNe,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheGe(C::FheGe {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheGe,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheGt(C::FheGt {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheGt,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheLe(C::FheLe {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheLe,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheLt(C::FheLt {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheLt,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheMin(C::FheMin {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheMin,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::FheMax(C::FheMax {
                lhs,
                rhs,
                scalarByte,
                result,
                ..
            }) => (
                O::FheMax,
                vec![H(*lhs), Operand::binary_rhs(*rhs, !scalarByte.is_zero())],
                *result,
            ),
            E::Cast(C::Cast {
                ct, toType, result, ..
            }) => (O::FheCast, vec![H(*ct), P(vec![*toType])], *result),
            E::FheNeg(C::FheNeg { ct, result, .. }) => {
                (O::FheNeg, vec![H(*ct)], *result)
            }
            E::FheNot(C::FheNot { ct, result, .. }) => {
                (O::FheNot, vec![H(*ct)], *result)
            }
            E::FheIfThenElse(C::FheIfThenElse {
                control,
                ifTrue,
                ifFalse,
                result,
                ..
            }) => (
                O::FheIfThenElse,
                vec![H(*control), H(*ifTrue), H(*ifFalse)],
                *result,
            ),
            E::FheRand(C::FheRand {
                seed,
                randType,
                result,
                ..
            }) => (
                O::FheRand,
                vec![P(seed.to_vec()), P(vec![*randType])],
                *result,
            ),
            E::FheRandBounded(C::FheRandBounded {
                seed,
                upperBound,
                randType,
                result,
                ..
            }) => (
                O::FheRandBounded,
                vec![
                    P(seed.to_vec()),
                    P(upperBound.to_be_bytes_vec()),
                    P(vec![*randType]),
                ],
                *result,
            ),
            E::TrivialEncrypt(C::TrivialEncrypt {
                pt, toType, result, ..
            }) => {
                return Ok(Some(Self::trivial(
                    pt.to_be_bytes(),
                    *toType,
                    *result,
                )))
            }
            E::FheSum(C::FheSum { values, result, .. }) => {
                (O::FheSum, values.iter().copied().map(H).collect(), *result)
            }
            E::FheIsIn(C::FheIsIn {
                value,
                values,
                result,
                ..
            }) => (
                O::FheIsIn,
                std::iter::once(*value)
                    .chain(values.iter().copied())
                    .map(H)
                    .collect(),
                *result,
            ),
            E::FheMulDiv(C::FheMulDiv {
                factor1,
                factor2,
                divisor,
                scalarByte,
                result,
                ..
            }) => (
                O::FheMulDiv,
                vec![
                    H(*factor1),
                    Operand::binary_rhs(*factor2, scalarByte.0[0] & 0b10 != 0),
                    P(divisor.to_vec()),
                ],
                *result,
            ),
            E::Initialized(_) | E::Upgraded(_) | E::VerifyInput(_) => {
                return Ok(None)
            }
        };
        Self::single(operation, operands, result).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, FixedBytes, U256};

    #[test]
    fn construction_rejects_invalid_operand_shapes_and_widths() {
        use Operand::{Clear as P, Encrypted as H};
        use SupportedFheOperations as O;
        let handle = Handle::repeat_byte(1);
        for (operation, operands) in [
            (O::FheAdd, vec![]),
            (O::FheAdd, vec![H(handle)]),
            (O::FheAdd, vec![P(vec![0; 32]), H(handle)]),
            (O::FheAdd, vec![H(handle), P(vec![0; 31])]),
            (O::FheNeg, vec![H(handle), H(handle)]),
            (O::FheCast, vec![H(handle), P(vec![5; 32])]),
            (O::FheCast, vec![H(handle), H(handle)]),
            (O::FheTrivialEncrypt, vec![P(vec![0; 31]), P(vec![5])]),
            (O::FheIfThenElse, vec![H(handle), H(handle), P(vec![0; 32])]),
            (O::FheRand, vec![P(vec![0; 32]), P(vec![5])]),
            (
                O::FheRandBounded,
                vec![P(vec![0; 16]), P(vec![0; 31]), P(vec![5])],
            ),
            (O::FheMulDiv, vec![H(handle), H(handle), H(handle)]),
            (O::FheSum, vec![P(vec![0; 32])]),
            (O::FheSum, vec![H(handle); 257]),
            (O::FheIsIn, vec![]),
            (O::FheGetInputCiphertext, vec![H(handle)]),
        ] {
            assert!(
                Computation::single(operation, operands, handle).is_err(),
                "{operation:?}"
            );
        }
        // Empty Sum and an empty IsIn set retain their native event encoding.
        assert!(Computation::single(O::FheSum, vec![], handle).is_ok());
        assert!(
            Computation::single(O::FheIsIn, vec![H(handle)], handle).is_ok()
        );
    }

    #[test]
    fn construction_enforces_output_group_bounds() {
        for count in [0, 1, MAX_MULTI_OUTPUT_ARITY, MAX_MULTI_OUTPUT_ARITY + 1]
        {
            let result = Computation::new(
                SupportedFheOperations::FheNeg,
                vec![Operand::Encrypted(Handle::ZERO)],
                vec![Handle::ZERO; count],
            );
            assert_eq!(
                result.is_ok(),
                (1..=MAX_MULTI_OUTPUT_ARITY).contains(&count)
            );
        }
    }

    #[test]
    fn malformed_evm_collection_is_an_error_instead_of_a_skipped_event() {
        assert!(Computation::from_evm(&E::FheSum(C::FheSum {
            caller: Address::ZERO,
            values: vec![Handle::ZERO; 257],
            result: Handle::ZERO,
        }))
        .is_err());
    }

    #[test]
    fn administrative_and_input_verification_events_do_not_create_computations()
    {
        for event in [
            E::Initialized(C::Initialized { version: 1 }),
            E::Upgraded(C::Upgraded {
                implementation: Address::ZERO,
            }),
            E::VerifyInput(C::VerifyInput {
                caller: Address::ZERO,
                inputHandle: Handle::repeat_byte(1),
                userAddress: Address::ZERO,
                inputProof: Default::default(),
                inputType: 5,
                result: Handle::repeat_byte(2),
            }),
        ] {
            assert_eq!(Computation::from_evm(&event).unwrap(), None);
        }
    }

    #[cfg(feature = "solana-reconstruct")]
    #[test]
    fn boundary_bits_match_the_host_at_every_position() {
        let computation = Computation::single(
            SupportedFheOperations::FheSum,
            (0..=u8::MAX)
                .map(|index| Operand::Encrypted(Handle::repeat_byte(index)))
                .collect(),
            Handle::ZERO,
        )
        .unwrap();
        for boundary in 0..256 {
            assert_eq!(
                computation
                    .boundary_mask(|handle| usize::from(handle[0]) != boundary)
                    .unwrap(),
                zama_host::operand_boundary_mask(
                    (0..256).map(|index| index == boundary)
                )
                .unwrap(),
            );
        }
        let mixed = Computation::single(
            SupportedFheOperations::FheMulDiv,
            vec![
                Operand::encrypted([1; 32]),
                Operand::Clear(vec![2; 32]),
                Operand::Clear(vec![3; 32]),
            ],
            Handle::ZERO,
        )
        .unwrap();
        assert_eq!(
            mixed.boundary_mask(|_| false).unwrap(),
            zama_host::operand_boundary_mask([true, false, false]).unwrap(),
            "clear operands retain their positions without setting a boundary bit",
        );
    }

    #[test]
    fn clear_operands_keep_the_worker_byte_widths() {
        let caller = Address::ZERO;
        let result = Handle::repeat_byte(8);
        let cases = [
            (
                E::Cast(C::Cast {
                    caller,
                    ct: Handle::repeat_byte(1),
                    toType: 5,
                    result,
                }),
                vec![vec![1; 32], vec![5]],
            ),
            (
                E::TrivialEncrypt(C::TrivialEncrypt {
                    caller,
                    pt: U256::from(7),
                    toType: 5,
                    result,
                }),
                vec![U256::from(7).to_be_bytes_vec(), vec![5]],
            ),
            (
                E::FheRand(C::FheRand {
                    caller,
                    randType: 5,
                    seed: FixedBytes::repeat_byte(2),
                    result,
                }),
                vec![vec![2; 16], vec![5]],
            ),
            (
                E::FheRandBounded(C::FheRandBounded {
                    caller,
                    randType: 5,
                    seed: FixedBytes::repeat_byte(2),
                    upperBound: U256::from(100),
                    result,
                }),
                vec![vec![2; 16], U256::from(100).to_be_bytes_vec(), vec![5]],
            ),
        ];
        for (event, bytes) in cases {
            let computation = Computation::from_evm(&event).unwrap().unwrap();
            assert_eq!(
                computation
                    .operands
                    .iter()
                    .map(Operand::bytes)
                    .collect::<Vec<_>>(),
                bytes
            );
            assert!(computation.is_scalar());
        }
    }

    #[test]
    fn mul_div_divisor_is_clear_but_flag_only_describes_factor2() {
        let factor1 = Handle::repeat_byte(1);
        let factor2 = Handle::repeat_byte(2);
        let divisor = Handle::repeat_byte(3);
        for scalar in [false, true] {
            let event = E::FheMulDiv(C::FheMulDiv {
                caller: Address::ZERO,
                factor1,
                factor2,
                divisor,
                scalarByte: FixedBytes::from([1 | (u8::from(scalar) << 1)]),
                result: Handle::repeat_byte(4),
            });
            let computation = Computation::from_evm(&event).unwrap().unwrap();
            assert_eq!(computation.is_scalar(), scalar);
            assert_eq!(
                computation
                    .operands
                    .iter()
                    .map(Operand::bytes)
                    .collect::<Vec<_>>(),
                vec![factor1.to_vec(), factor2.to_vec(), divisor.to_vec()]
            );
            assert_eq!(
                computation.inputs(),
                if scalar {
                    vec![factor1]
                } else {
                    vec![factor1, factor2]
                }
            );
            assert_eq!(
                computation.boundary_mask(|h| *h == factor1).unwrap()[31],
                if scalar { 0 } else { 2 }
            );
        }
    }
}
