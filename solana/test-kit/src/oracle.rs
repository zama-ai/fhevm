//! The cleartext oracle: evaluates `fhe_execute` step programs without Solana or TFHE, and
//! replays the `fhe_execute` CPIs an instruction issued so encrypted state can be asserted in
//! cleartext.
//!
//! This is the same move the EVM test suite's mock tier makes — real cryptography is covered by
//! the coprocessor's own tests and the live tier; here the signal is orchestration, accounting,
//! and access control. Arithmetic follows the canonical host/worker width and type rules, and
//! evaluation calls `zama-host`'s own validators, so the oracle cannot drift from the program's
//! admission rules.

use std::collections::HashMap;

use mollusk_svm::result::InstructionResult;
use num_bigint::BigUint;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use solana_sdk::pubkey::Pubkey;
use zama_host::{
    assert_binary_operand_types, assert_is_in_operand_types, assert_mul_div_operand_types,
    assert_sum_operand_types, assert_supported_fhe_type, assert_supported_rand_type,
    assert_unary_operand_type, assert_valid_bounded_rand_upper_bound, handle_fhe_type,
    FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteStep, FheTernaryOpCode,
    FheUnaryOpCode,
};

use crate::{
    decode_fhe_execute_args, execution_step_output, read_encrypted_value, Ctx, BALANCE_FHE_TYPE,
};

pub type Handle = [u8; 32];
pub type ClearInputs = HashMap<Handle, TypedClearValue>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedClearValue {
    pub fhe_type: u8,
    pub value: [u8; 32],
}

impl TypedClearValue {
    pub fn from_u64(fhe_type: u8, value: u64) -> Self {
        Self {
            fhe_type,
            value: crate::u256_be(value),
        }
    }

    pub fn from_be_bytes(fhe_type: u8, value: [u8; 32]) -> Self {
        Self { fhe_type, value }
    }
}

#[derive(Clone, Debug)]
struct ClearValue {
    fhe_type: u8,
    value: BigUint,
}

impl ClearValue {
    fn from_typed(value: TypedClearValue) -> Result<Self, String> {
        Self::new(value.fhe_type, BigUint::from_bytes_be(&value.value))
    }

    fn new(fhe_type: u8, value: BigUint) -> Result<Self, String> {
        let value = normalize(value, fhe_type)?;
        Ok(Self { fhe_type, value })
    }

    fn typed(&self) -> TypedClearValue {
        let bytes = self.value.to_bytes_be();
        let mut value = [0; 32];
        value[32 - bytes.len()..].copy_from_slice(&bytes);
        TypedClearValue {
            fhe_type: self.fhe_type,
            value,
        }
    }

    fn validation_handle(&self) -> Handle {
        validation_handle(self.fhe_type)
    }
}

/// Evaluates step-level cleartext compute from canonical `FheExecuteArgs` without Solana or TFHE.
///
/// Arithmetic follows the canonical host/worker width and type rules. Random steps deliberately
/// use a deterministic local PRNG: they are mock values, not predictions of TFHE's oblivious PRG.
/// The returned values are ordered by step index, matching `EarlierStep::producer_index`.
/// This is not host preflight: output descriptors, account indices, attestations, and ACL checks
/// are intentionally ignored.
pub fn evaluate(
    args: &FheExecuteArgs,
    inputs: &ClearInputs,
) -> Result<Vec<TypedClearValue>, String> {
    let mut produced = Vec::<ClearValue>::with_capacity(args.steps.len());
    let mut random = StdRng::from_seed([7; 32]);

    for step in &args.steps {
        let value = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => evaluate_binary(
                &args.dictionary,
                *op,
                resolve_encrypted(lhs, &args.dictionary, inputs, &produced)?,
                rhs,
                *output_fhe_type,
                inputs,
                &produced,
            )?,
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                canonical(
                    assert_supported_fhe_type(*output_fhe_type),
                    "ternary operation",
                )?;
                let control = resolve_encrypted(control, &args.dictionary, inputs, &produced)?;
                let if_true = resolve_encrypted(if_true, &args.dictionary, inputs, &produced)?;
                let if_false = resolve_encrypted(if_false, &args.dictionary, inputs, &produced)?;
                // Mirrors `fhe_execute::assert_ternary_operand_types`; keep the malformed-ternary
                // cases in `operator_conformance.rs::rejected::ternary` aligned with it.
                if control.fhe_type != 0
                    || if_true.fhe_type != *output_fhe_type
                    || if_false.fhe_type != *output_fhe_type
                {
                    return Err("invalid ternary operand types".into());
                }
                match op {
                    FheTernaryOpCode::IfThenElse => {
                        if control.value != BigUint::from(0u8) {
                            if_true
                        } else {
                            if_false
                        }
                    }
                }
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => {
                canonical(assert_supported_fhe_type(*fhe_type), "trivial encrypt")?;
                let value = if *fhe_type == 0 {
                    BigUint::from(u8::from(plaintext[31] != 0))
                } else {
                    BigUint::from_bytes_be(plaintext)
                };
                ClearValue::new(*fhe_type, value)?
            }
            FheExecuteStep::Rand { fhe_type, .. } => {
                canonical(assert_supported_rand_type(*fhe_type), "rand")?;
                let mut value = random_biguint(&mut random);
                if *fhe_type == 0 {
                    value &= BigUint::from(1u8);
                }
                ClearValue::new(*fhe_type, value)?
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                ..
            } => evaluate_unary(
                *op,
                resolve_encrypted(operand, &args.dictionary, inputs, &produced)?,
                *output_fhe_type,
            )?,
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
                ..
            } => {
                canonical(
                    assert_valid_bounded_rand_upper_bound(*upper_bound, *fhe_type),
                    "bounded rand",
                )?;
                let bound = BigUint::from_bytes_be(upper_bound);
                ClearValue::new(*fhe_type, random_biguint(&mut random) % bound)?
            }
            FheExecuteStep::Sum {
                operands, fhe_type, ..
            } => {
                let values = operands
                    .iter()
                    .map(|operand| resolve_encrypted(operand, &args.dictionary, inputs, &produced))
                    .collect::<Result<Vec<_>, _>>()?;
                let handles = values
                    .iter()
                    .map(ClearValue::validation_handle)
                    .collect::<Vec<_>>();
                canonical(assert_sum_operand_types(&handles, *fhe_type), "sum")?;
                ClearValue::new(*fhe_type, values.into_iter().map(|value| value.value).sum())?
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
                ..
            } => {
                let value = resolve_encrypted(value, &args.dictionary, inputs, &produced)?;
                let set = set
                    .iter()
                    .map(|operand| resolve_encrypted(operand, &args.dictionary, inputs, &produced))
                    .collect::<Result<Vec<_>, _>>()?;
                let handles = set
                    .iter()
                    .map(ClearValue::validation_handle)
                    .collect::<Vec<_>>();
                canonical(
                    assert_is_in_operand_types(value.validation_handle(), &handles, *fhe_type),
                    "is-in",
                )?;
                ClearValue::new(
                    0,
                    BigUint::from(u8::from(set.iter().any(|item| item.value == value.value))),
                )?
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                ..
            } => evaluate_mul_div(
                &args.dictionary,
                resolve_encrypted(factor1, &args.dictionary, inputs, &produced)?,
                factor2,
                *divisor,
                *output_fhe_type,
                inputs,
                &produced,
            )?,
        };
        produced.push(value);
    }

    Ok(produced.iter().map(ClearValue::typed).collect())
}

fn evaluate_binary(
    dictionary: &[[u8; 32]],
    op: FheBinaryOpCode,
    lhs: ClearValue,
    rhs_operand: &FheExecuteOperand,
    output_fhe_type: u8,
    inputs: &ClearInputs,
    produced: &[ClearValue],
) -> Result<ClearValue, String> {
    let (rhs, rhs_handle, scalar) =
        resolve_rhs(rhs_operand, dictionary, lhs.fhe_type, inputs, produced)?;
    canonical(
        assert_binary_operand_types(
            op,
            lhs.validation_handle(),
            rhs_handle,
            scalar,
            output_fhe_type,
        ),
        "binary operation",
    )?;

    let result = match op {
        FheBinaryOpCode::Add => lhs.value + rhs.value,
        FheBinaryOpCode::Sub => wrapping_sub(lhs.value, rhs.value, lhs.fhe_type)?,
        FheBinaryOpCode::Mul => lhs.value * rhs.value,
        FheBinaryOpCode::Div => lhs.value / rhs.value,
        FheBinaryOpCode::Rem => lhs.value % rhs.value,
        FheBinaryOpCode::And => lhs.value & rhs.value,
        FheBinaryOpCode::Or => lhs.value | rhs.value,
        FheBinaryOpCode::Xor => lhs.value ^ rhs.value,
        FheBinaryOpCode::Shl => shift_left(lhs.value, &rhs.value, lhs.fhe_type)?,
        FheBinaryOpCode::Shr => shift_right(lhs.value, &rhs.value, lhs.fhe_type)?,
        FheBinaryOpCode::Rotl => rotate(lhs.value, &rhs.value, lhs.fhe_type, true)?,
        FheBinaryOpCode::Rotr => rotate(lhs.value, &rhs.value, lhs.fhe_type, false)?,
        FheBinaryOpCode::Eq => BigUint::from(u8::from(lhs.value == rhs.value)),
        FheBinaryOpCode::Ne => BigUint::from(u8::from(lhs.value != rhs.value)),
        FheBinaryOpCode::Ge => BigUint::from(u8::from(lhs.value >= rhs.value)),
        FheBinaryOpCode::Gt => BigUint::from(u8::from(lhs.value > rhs.value)),
        FheBinaryOpCode::Le => BigUint::from(u8::from(lhs.value <= rhs.value)),
        FheBinaryOpCode::Lt => BigUint::from(u8::from(lhs.value < rhs.value)),
        FheBinaryOpCode::Min => lhs.value.min(rhs.value),
        FheBinaryOpCode::Max => lhs.value.max(rhs.value),
    };
    ClearValue::new(output_fhe_type, result)
}

fn evaluate_unary(
    op: FheUnaryOpCode,
    operand: ClearValue,
    output_fhe_type: u8,
) -> Result<ClearValue, String> {
    canonical(
        assert_unary_operand_type(op, operand.validation_handle(), output_fhe_type),
        "unary operation",
    )?;
    let result = match op {
        FheUnaryOpCode::Neg => wrapping_sub(BigUint::from(0u8), operand.value, output_fhe_type)?,
        FheUnaryOpCode::Not => mask(output_fhe_type)? ^ operand.value,
        FheUnaryOpCode::Cast => operand.value,
    };
    ClearValue::new(output_fhe_type, result)
}

fn evaluate_mul_div(
    dictionary: &[[u8; 32]],
    factor1: ClearValue,
    factor2_operand: &FheExecuteOperand,
    divisor: [u8; 32],
    output_fhe_type: u8,
    inputs: &ClearInputs,
    produced: &[ClearValue],
) -> Result<ClearValue, String> {
    let (factor2, factor2_handle, scalar) = resolve_rhs(
        factor2_operand,
        dictionary,
        factor1.fhe_type,
        inputs,
        produced,
    )?;
    canonical(
        assert_mul_div_operand_types(
            factor1.validation_handle(),
            factor2_handle,
            scalar,
            divisor,
            output_fhe_type,
        ),
        "mul-div",
    )?;
    let divisor = normalize(BigUint::from_bytes_be(&divisor), output_fhe_type)?;
    ClearValue::new(output_fhe_type, (factor1.value * factor2.value) / divisor)
}

fn resolve_pool_bytes(dictionary: &[[u8; 32]], index: u8) -> Result<[u8; 32], String> {
    dictionary
        .get(usize::from(index))
        .copied()
        .ok_or_else(|| format!("dictionary index {index} out of bounds"))
}

fn resolve_rhs(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    fhe_type: u8,
    inputs: &ClearInputs,
    produced: &[ClearValue],
) -> Result<(ClearValue, Handle, bool), String> {
    match operand {
        FheExecuteOperand::Scalar { value_index } => {
            let bytes = resolve_pool_bytes(dictionary, *value_index)?;
            Ok((
                ClearValue::new(fhe_type, BigUint::from_bytes_be(&bytes))?,
                bytes,
                true,
            ))
        }
        _ => {
            let value = resolve_encrypted(operand, dictionary, inputs, produced)?;
            let handle = value.validation_handle();
            Ok((value, handle, false))
        }
    }
}

fn resolve_encrypted(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    inputs: &ClearInputs,
    produced: &[ClearValue],
) -> Result<ClearValue, String> {
    let (handle, value) = match operand {
        FheExecuteOperand::StoredValue { handle_index, .. } => {
            let handle = resolve_pool_bytes(dictionary, *handle_index)?;
            (handle, inputs.get(&handle))
        }
        FheExecuteOperand::VerifiedInput { attestation } => {
            let handle = attestation.input_handle;
            (handle, inputs.get(&handle))
        }
        FheExecuteOperand::EarlierStep { producer_index } => {
            return produced
                .get(*producer_index as usize)
                .cloned()
                .ok_or_else(|| format!("missing earlier local output {producer_index}"));
        }
        FheExecuteOperand::Scalar { .. } => {
            return Err("scalar is not valid in this operand position".into())
        }
    };
    let value = value
        .copied()
        .ok_or_else(|| format!("missing cleartext input for handle {handle:?}"))?;
    if handle_fhe_type(handle) != value.fhe_type {
        return Err(format!(
            "handle type {} does not match cleartext type {}",
            handle_fhe_type(handle),
            value.fhe_type
        ));
    }
    ClearValue::from_typed(value)
}

fn canonical(result: anchor_lang::Result<()>, context: &str) -> Result<(), String> {
    result.map_err(|error| format!("invalid {context}: {error}"))
}

fn type_bits(fhe_type: u8) -> Result<usize, String> {
    match fhe_type {
        0 => Ok(1),
        2 => Ok(8),
        3 => Ok(16),
        4 => Ok(32),
        5 => Ok(64),
        6 => Ok(128),
        7 => Ok(160),
        8 => Ok(256),
        _ => Err(format!("unsupported FHE type {fhe_type}")),
    }
}

fn modulus(fhe_type: u8) -> Result<BigUint, String> {
    Ok(BigUint::from(1u8) << type_bits(fhe_type)?)
}

fn mask(fhe_type: u8) -> Result<BigUint, String> {
    Ok(modulus(fhe_type)? - BigUint::from(1u8))
}

fn normalize(value: BigUint, fhe_type: u8) -> Result<BigUint, String> {
    if fhe_type == 0 {
        return Ok(BigUint::from(u8::from(value != BigUint::from(0u8))));
    }
    Ok(value % modulus(fhe_type)?)
}

fn wrapping_sub(lhs: BigUint, rhs: BigUint, fhe_type: u8) -> Result<BigUint, String> {
    let modulus = modulus(fhe_type)?;
    Ok((lhs + &modulus - rhs) % modulus)
}

fn shift_amount(value: &BigUint, bits: usize) -> usize {
    let bytes = (value % BigUint::from(bits)).to_bytes_be();
    bytes
        .iter()
        .fold(0usize, |amount, byte| (amount << 8) | *byte as usize)
}

fn shift_left(value: BigUint, rhs: &BigUint, fhe_type: u8) -> Result<BigUint, String> {
    let bits = type_bits(fhe_type)?;
    Ok((value << shift_amount(rhs, bits)) & mask(fhe_type)?)
}

fn shift_right(value: BigUint, rhs: &BigUint, fhe_type: u8) -> Result<BigUint, String> {
    let bits = type_bits(fhe_type)?;
    Ok(value >> shift_amount(rhs, bits))
}

fn rotate(value: BigUint, rhs: &BigUint, fhe_type: u8, left: bool) -> Result<BigUint, String> {
    let bits = type_bits(fhe_type)?;
    let amount = shift_amount(rhs, bits);
    if amount == 0 {
        return Ok(value);
    }
    let result = if left {
        (&value << amount) | (&value >> (bits - amount))
    } else {
        (&value >> amount) | (&value << (bits - amount))
    };
    Ok(result & mask(fhe_type)?)
}

fn validation_handle(fhe_type: u8) -> Handle {
    let mut handle = [0; 32];
    handle[30] = fhe_type;
    handle
}

fn random_biguint(random: &mut StdRng) -> BigUint {
    let mut bytes = [0; 32];
    random.fill_bytes(&mut bytes);
    BigUint::from_bytes_be(&bytes)
}

// ---------------------------------------------------------------------------
// The cleartext ledger
// ---------------------------------------------------------------------------

/// What one instruction's `fhe_execute` replay covered.
pub struct FheReplay {
    /// Distinct `fhe_execute` CPIs decoded from the inner instructions.
    pub executions: usize,
    /// Persistent (`StoredValue`) outputs bound to their end-of-instruction handles.
    pub persistent_outputs: usize,
}

/// Cleartext mirror of every encrypted handle the tests touch.
#[derive(Default)]
pub struct CleartextLedger {
    pub values: ClearInputs,
}

impl CleartextLedger {
    /// Seeds a `euint64` amount for `handle`.
    pub fn seed_amount(&mut self, handle: [u8; 32], value: u64) {
        self.values
            .insert(handle, TypedClearValue::from_u64(BALANCE_FHE_TYPE, value));
    }

    /// Replays every `fhe_execute` CPI the instruction issued — in order, so a later execution
    /// can consume an earlier execution's persisted outputs. Each instruction writes any
    /// encrypted value account at most once, so binding results to the end-of-instruction
    /// persisted handles is exact.
    pub fn replay_fhe_cpis(&mut self, context: &Ctx, result: &InstructionResult) -> FheReplay {
        let message = result
            .message
            .as_ref()
            .expect("Mollusk result must include its compiled message");
        let execute_args = result
            .inner_instructions
            .iter()
            .filter(|inner| {
                message
                    .account_keys()
                    .get(inner.instruction.program_id_index as usize)
                    .copied()
                    == Some(zama_host::id())
            })
            .filter_map(|inner| decode_fhe_execute_args(&inner.instruction.data))
            .collect::<Vec<_>>();

        let mut persistent_outputs = 0;
        for args in &execute_args {
            let outputs = evaluate(args, &self.values)
                .expect("every emitted FHE batch must be valid in cleartext");
            for (step, value) in args.steps.iter().zip(outputs) {
                let zama_host::FheExecuteOutput::StoredValue {
                    output_domain_index,
                    output_account_index,
                    output_label_index,
                    ..
                } = execution_step_output(step)
                else {
                    continue;
                };
                let encrypted_value_id = zama_solana_acl::derive_encrypted_value_id(
                    args.dictionary_bytes(*output_domain_index)
                        .expect("valid dictionary index"),
                    args.dictionary_bytes(*output_account_index)
                        .expect("valid dictionary index"),
                    args.dictionary_bytes(*output_label_index)
                        .expect("valid dictionary index"),
                );
                let address = zama_host::encrypted_value_address(encrypted_value_id).0;
                let persisted = read_encrypted_value(context, address);
                self.values.insert(persisted.current_handle, value);
                persistent_outputs += 1;
            }
        }
        FheReplay {
            executions: execute_args.len(),
            persistent_outputs,
        }
    }

    /// The `u64` cleartext behind the current handle of the encrypted value account at
    /// `encrypted_value`.
    pub fn u64_at(&self, context: &Ctx, encrypted_value: Pubkey) -> u64 {
        let handle = read_encrypted_value(context, encrypted_value).current_handle;
        let value = self
            .values
            .get(&handle)
            .expect("missing cleartext value for persisted handle");
        assert_eq!(value.fhe_type, BALANCE_FHE_TYPE);
        assert_eq!(value.value[..24], [0; 24], "cleartext value exceeds u64");
        u64::from_be_bytes(value.value[24..].try_into().unwrap())
    }
}
