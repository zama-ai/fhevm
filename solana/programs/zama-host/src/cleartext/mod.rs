//! Cleartext simulator build (feature `cleartext`): the production host, plus the plaintext of
//! every handle it produces, kept in chain state.
//!
//! `fhe_execute` runs unchanged, then this module evaluates the same steps on plaintexts and
//! writes each value next to its handle: in the `EncryptedStore` slot section for persistent
//! results and in the `TransientStore` tail for transaction-local ones. Because the values are
//! account data, snapshots, reverts and every client that sends a transaction stay in sync with
//! them, the property forge-fhevm-std gets from its cleartext host contracts.
//!
//! The build is for local tests only: `build-programs.sh` refuses it for an environment file and
//! the deployer refuses an artifact carrying [`layout::MAGIC`].

pub mod layout;

use anchor_lang::prelude::*;

use crate::{
    assert_binary_operand_types, assert_is_in_operand_types, assert_mul_div_operand_types,
    assert_sum_operand_types, assert_supported_fhe_type, assert_ternary_operand_types,
    assert_unary_operand_type, assert_valid_bounded_rand_upper_bound, errors::ZamaHostError,
    handle_fhe_type, FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteStep,
    FheTernaryOpCode, FheUnaryOpCode, MAX_INPUT_ATTESTATION_EXTRA_DATA,
};

/// Errors only the cleartext build returns. Numbered apart from [`ZamaHostError`], whose codes
/// are the production ABI.
#[error_code(offset = 7000)]
pub enum CleartextError {
    /// The account holds no plaintext for this handle: a production build wrote it.
    #[msg("cleartext build: no plaintext is recorded for this handle")]
    ValueUnknown,
    /// A verified input's `extra_data` does not carry one plaintext per attested handle.
    #[msg("cleartext build: input attestation extra_data does not carry its plaintexts")]
    InputMalformed,
}

/// A plaintext of a shipped FHE type (`0 | 2..=6`, at most 128 bits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Value {
    pub fhe_type: u8,
    pub bits: u128,
}

impl Value {
    /// Reduces `bits` to `fhe_type`: a bool is `bits != 0`, an integer wraps at its width.
    pub fn new(fhe_type: u8, bits: u128) -> Result<Self> {
        let bits = if fhe_type == 0 {
            u128::from(bits != 0)
        } else {
            bits & mask(fhe_type)?
        };
        Ok(Self { fhe_type, bits })
    }

    /// A big-endian 256-bit plaintext, as scalars and trivial encryptions carry it.
    pub fn from_be_bytes(fhe_type: u8, bytes: [u8; 32]) -> Result<Self> {
        if fhe_type == 0 {
            return Self::new(0, u128::from(bytes.iter().any(|byte| *byte != 0)));
        }
        Self::new(
            fhe_type,
            u128::from_be_bytes(bytes[16..].try_into().unwrap()),
        )
    }

    fn validation_handle(self) -> [u8; 32] {
        let mut handle = [0; 32];
        handle[30] = self.fhe_type;
        handle
    }
}

/// Width in bits of a shipped FHE type.
fn bit_width(fhe_type: u8) -> Result<u32> {
    match fhe_type {
        0 => Ok(1),
        2 => Ok(8),
        3 => Ok(16),
        4 => Ok(32),
        5 => Ok(64),
        6 => Ok(128),
        _ => err!(ZamaHostError::UnsupportedFheType),
    }
}

/// Bytes one value of `fhe_type` takes in an input attestation. At least two, so no encoding is
/// production's one-byte `extra_data` (`0x00`); 16 `euint128` inputs still fill 256 bytes.
fn value_len(fhe_type: u8) -> Result<usize> {
    Ok(bit_width(fhe_type)?.div_ceil(8).max(2) as usize)
}

fn mask(fhe_type: u8) -> Result<u128> {
    Ok(match bit_width(fhe_type)? {
        128 => u128::MAX,
        bits => (1u128 << bits) - 1,
    })
}

/// Encodes the plaintexts of an input attestation's handles as its `extra_data`: one big-endian
/// value per handle, in `ct_handles` order, each [`value_len`] bytes wide. Fails when they exceed
/// the attestation's `extra_data` limit, which the host would reject: 16 `euint128` inputs fit.
pub fn encode_input_values(values: &[Value]) -> Result<Vec<u8>> {
    let mut extra_data = Vec::new();
    for value in values {
        let len = value_len(value.fhe_type)?;
        extra_data.extend_from_slice(&value.bits.to_be_bytes()[16 - len..]);
    }
    require!(
        extra_data.len() <= MAX_INPUT_ATTESTATION_EXTRA_DATA,
        ZamaHostError::MalformedInputAttestation
    );
    Ok(extra_data)
}

/// The plaintext of `ct_handles[index]` in an input attestation's `extra_data`. Every value must
/// fit its type, so no encoding reads as a different plaintext than the one it was built from.
pub fn decode_input_value(
    extra_data: &[u8],
    ct_handles: &[[u8; 32]],
    index: usize,
) -> Result<Value> {
    let malformed = || error!(CleartextError::InputMalformed);
    let mut rest = extra_data;
    let mut selected = None;
    for (position, handle) in ct_handles.iter().enumerate() {
        let fhe_type = handle_fhe_type(*handle);
        let len = value_len(fhe_type)?;
        require!(rest.len() >= len, CleartextError::InputMalformed);
        let (bytes, tail) = rest.split_at(len);
        let mut be = [0u8; 16];
        be[16 - len..].copy_from_slice(bytes);
        let bits = u128::from_be_bytes(be);
        require!(bits <= mask(fhe_type)?, CleartextError::InputMalformed);
        if position == index {
            selected = Some(Value::new(fhe_type, bits)?);
        }
        rest = tail;
    }
    require!(rest.is_empty(), CleartextError::InputMalformed);
    selected.ok_or_else(malformed)
}

/// The plaintext a rand step produces from its seed. Deterministic, so a replayed transaction
/// reproduces it; it is a mock value, not TFHE's oblivious PRG output.
fn rand_bits(seed: [u8; 16]) -> u128 {
    let digest = solana_keccak_hasher::hashv(&[b"ZAMA_CLEARTEXT_RAND_V1", &seed]).to_bytes();
    u128::from_be_bytes(digest[16..].try_into().unwrap())
}

/// Evaluates every step of an execution on plaintexts, in step order.
///
/// `resolve` answers the operands whose value lives outside the execution (a store slot, a
/// transient result or a verified input); it also receives the values produced so far. `rand_seed`
/// returns the seed of the rand step at an index. Operand and output types go through the host's
/// own gates, so an execution the host would refuse is refused here too.
pub fn evaluate_steps(
    args: &FheExecuteArgs,
    mut resolve: impl FnMut(&FheExecuteOperand, &[Value]) -> Result<Value>,
    rand_seed: impl Fn(u16) -> Result<[u8; 16]>,
) -> Result<Vec<Value>> {
    let mut produced = Vec::with_capacity(args.steps.len());
    // One buffer for every Sum and IsIn step, because the host heap never frees.
    let mut operand_handles: Vec<[u8; 32]> = Vec::new();
    for (index, step) in args.steps.iter().enumerate() {
        let mut encrypted = |operand: &FheExecuteOperand, produced: &[Value]| match operand {
            FheExecuteOperand::EarlierStep { producer_index } => produced
                .get(usize::from(*producer_index))
                .copied()
                .ok_or_else(|| error!(ZamaHostError::FheExecuteEarlierStepMissing)),
            FheExecuteOperand::Scalar { .. } => err!(ZamaHostError::InvalidFheExecuteAccount),
            _ => resolve(operand, produced),
        };
        let value = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
            } => {
                let lhs = encrypted(lhs, &produced)?;
                let (rhs, rhs_handle, scalar) = match rhs {
                    FheExecuteOperand::Scalar { value_index } => {
                        let bytes = args.dictionary_bytes(*value_index)?;
                        (Value::from_be_bytes(lhs.fhe_type, bytes)?, bytes, true)
                    }
                    operand => {
                        let rhs = encrypted(operand, &produced)?;
                        (rhs, rhs.validation_handle(), false)
                    }
                };
                assert_binary_operand_types(
                    *op,
                    lhs.validation_handle(),
                    rhs_handle,
                    scalar,
                    *output_fhe_type,
                )?;
                binary(*op, lhs, rhs, *output_fhe_type)?
            }
            FheExecuteStep::Ternary {
                op: FheTernaryOpCode::IfThenElse,
                control,
                if_true,
                if_false,
                output_fhe_type,
            } => {
                let control = encrypted(control, &produced)?;
                let if_true = encrypted(if_true, &produced)?;
                let if_false = encrypted(if_false, &produced)?;
                assert_ternary_operand_types(
                    control.validation_handle(),
                    if_true.validation_handle(),
                    if_false.validation_handle(),
                    *output_fhe_type,
                )?;
                if control.bits != 0 {
                    if_true
                } else {
                    if_false
                }
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
            } => {
                assert_supported_fhe_type(*fhe_type)?;
                // A trivial bool reads only the last byte, as the coprocessor's
                // `trivial_encrypt_be_bytes` does.
                if *fhe_type == 0 {
                    Value::new(0, u128::from(plaintext[31]))?
                } else {
                    Value::from_be_bytes(*fhe_type, *plaintext)?
                }
            }
            FheExecuteStep::Rand { fhe_type } => {
                assert_supported_fhe_type(*fhe_type)?;
                // Truncation, not `!= 0`, so a random bool is a fair bit.
                Value::new(
                    *fhe_type,
                    rand_bits(rand_seed(index as u16)?) & mask(*fhe_type)?,
                )?
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
            } => {
                assert_valid_bounded_rand_upper_bound(*upper_bound, *fhe_type)?;
                let bits = rand_bits(rand_seed(index as u16)?);
                // A power of two no wider than the type: the value is the low bits below it.
                let bound = u128::from_be_bytes(upper_bound[16..].try_into().unwrap());
                let bits = if upper_bound[..16].iter().any(|byte| *byte != 0) || bound == 0 {
                    bits
                } else {
                    bits & (bound - 1)
                };
                Value::new(*fhe_type, bits)?
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
            } => {
                let operand = encrypted(operand, &produced)?;
                assert_unary_operand_type(*op, operand.validation_handle(), *output_fhe_type)?;
                let bits = match op {
                    FheUnaryOpCode::Neg => operand.bits.wrapping_neg(),
                    FheUnaryOpCode::Not => operand.bits ^ mask(*output_fhe_type)?,
                    FheUnaryOpCode::Cast => operand.bits,
                };
                Value::new(*output_fhe_type, bits)?
            }
            FheExecuteStep::Sum { operands, fhe_type } => {
                operand_handles.clear();
                operand_handles.reserve(operands.len());
                let mut sum = 0u128;
                for operand in operands {
                    let value = encrypted(operand, &produced)?;
                    operand_handles.push(value.validation_handle());
                    sum = sum.wrapping_add(value.bits);
                }
                assert_sum_operand_types(&operand_handles, *fhe_type)?;
                Value::new(*fhe_type, sum)?
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
            } => {
                let value = encrypted(value, &produced)?;
                operand_handles.clear();
                operand_handles.reserve(set.len());
                let mut found = false;
                for operand in set {
                    let item = encrypted(operand, &produced)?;
                    operand_handles.push(item.validation_handle());
                    found |= item.bits == value.bits;
                }
                assert_is_in_operand_types(value.validation_handle(), &operand_handles, *fhe_type)?;
                Value::new(0, u128::from(found))?
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
            } => {
                let factor1 = encrypted(factor1, &produced)?;
                let (factor2, factor2_handle, scalar) = match factor2 {
                    FheExecuteOperand::Scalar { value_index } => {
                        let bytes = args.dictionary_bytes(*value_index)?;
                        (Value::from_be_bytes(factor1.fhe_type, bytes)?, bytes, true)
                    }
                    operand => {
                        let factor2 = encrypted(operand, &produced)?;
                        (factor2, factor2.validation_handle(), false)
                    }
                };
                assert_mul_div_operand_types(
                    factor1.validation_handle(),
                    factor2_handle,
                    scalar,
                    *divisor,
                    *output_fhe_type,
                )?;
                // Output types stop at euint64, so the product fits 128 bits.
                let divisor = Value::from_be_bytes(*output_fhe_type, *divisor)?;
                Value::new(*output_fhe_type, factor1.bits * factor2.bits / divisor.bits)?
            }
        };
        produced.push(value);
    }
    Ok(produced)
}

fn binary(op: FheBinaryOpCode, lhs: Value, rhs: Value, output_fhe_type: u8) -> Result<Value> {
    let width = bit_width(lhs.fhe_type)?;
    // Shift and rotate amounts wrap at the operand width, as tfhe-rs 1.7 does.
    let amount = || (rhs.bits % u128::from(width)) as u32;
    let bits = match op {
        FheBinaryOpCode::Add => lhs.bits.wrapping_add(rhs.bits),
        FheBinaryOpCode::Sub => lhs.bits.wrapping_sub(rhs.bits),
        FheBinaryOpCode::Mul => lhs.bits.wrapping_mul(rhs.bits),
        FheBinaryOpCode::Div => lhs.bits / rhs.bits,
        FheBinaryOpCode::Rem => lhs.bits % rhs.bits,
        FheBinaryOpCode::And => lhs.bits & rhs.bits,
        FheBinaryOpCode::Or => lhs.bits | rhs.bits,
        FheBinaryOpCode::Xor => lhs.bits ^ rhs.bits,
        FheBinaryOpCode::Shl => lhs.bits.checked_shl(amount()).unwrap_or(0),
        FheBinaryOpCode::Shr => lhs.bits >> amount(),
        FheBinaryOpCode::Rotl | FheBinaryOpCode::Rotr => {
            let amount = amount();
            let left = matches!(op, FheBinaryOpCode::Rotl);
            let (up, down) = if left {
                (amount, (width - amount) % width)
            } else {
                ((width - amount) % width, amount)
            };
            let value = lhs.bits & mask(lhs.fhe_type)?;
            match amount {
                0 => value,
                _ => (value << up) | (value >> down),
            }
        }
        FheBinaryOpCode::Eq => u128::from(lhs.bits == rhs.bits),
        FheBinaryOpCode::Ne => u128::from(lhs.bits != rhs.bits),
        FheBinaryOpCode::Ge => u128::from(lhs.bits >= rhs.bits),
        FheBinaryOpCode::Gt => u128::from(lhs.bits > rhs.bits),
        FheBinaryOpCode::Le => u128::from(lhs.bits <= rhs.bits),
        FheBinaryOpCode::Lt => u128::from(lhs.bits < rhs.bits),
        FheBinaryOpCode::Min => lhs.bits.min(rhs.bits),
        FheBinaryOpCode::Max => lhs.bits.max(rhs.bits),
    };
    Value::new(output_fhe_type, bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(fhe_type: u8, bits: u128) -> Value {
        Value::new(fhe_type, bits).unwrap()
    }

    #[test]
    fn rotations_wrap_at_the_operand_width() {
        let rotl =
            |bits, amount| binary(FheBinaryOpCode::Rotl, value(2, bits), value(2, amount), 2);
        assert_eq!(rotl(0b1000_0001, 1).unwrap().bits, 0b0000_0011);
        assert_eq!(rotl(0b1000_0001, 9).unwrap().bits, 0b0000_0011);
        let rotr = binary(FheBinaryOpCode::Rotr, value(6, 1), value(6, 1), 6).unwrap();
        assert_eq!(rotr.bits, 1u128 << 127);
    }

    #[test]
    fn input_values_round_trip_and_refuse_malformed_encodings() {
        let handle = |fhe_type| {
            let mut handle = [0u8; 32];
            handle[30] = fhe_type;
            handle
        };
        let handles = [handle(0), handle(5), handle(6)];
        let values = [value(0, 1), value(5, 7), value(6, u128::MAX)];
        let extra_data = encode_input_values(&values).unwrap();
        assert_eq!(extra_data.len(), 2 + 8 + 16);
        for (index, expected) in values.iter().enumerate() {
            assert_eq!(
                decode_input_value(&extra_data, &handles, index).unwrap(),
                *expected
            );
        }
        assert!(decode_input_value(&extra_data[..extra_data.len() - 1], &handles, 0).is_err());
        assert!(decode_input_value(&[extra_data.as_slice(), &[0]].concat(), &handles, 0).is_err());
        // Production's one-byte `0x00` is no input, not even a lone bool.
        assert!(decode_input_value(&[0x00], &handles[..1], 0).is_err());
        // A value wider than its type is refused wherever it sits, not truncated.
        assert!(decode_input_value(&[0x00, 0x02], &handles[..1], 0).is_err());
        let mut too_wide = extra_data.clone();
        too_wide[0] = 1;
        assert!(decode_input_value(&too_wide, &handles, 2).is_err());
    }

    #[test]
    fn input_values_fit_the_attestation_extra_data_limit() {
        let euint128 = |count| vec![value(6, u128::MAX); count];
        assert_eq!(encode_input_values(&euint128(16)).unwrap().len(), 256);
        assert!(encode_input_values(&euint128(17)).is_err());
    }
}
