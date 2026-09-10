//! The cleartext oracle: evaluates `fhe_execute` step programs without Solana or TFHE, and
//! replays the `fhe_execute` CPIs an instruction issued so encrypted state can be asserted in
//! cleartext.
//!
//! This is the same move the EVM test suite's mock tier makes — real cryptography is covered by
//! the coprocessor's own tests and the live tier; here the signal is orchestration, accounting,
//! and access control. Arithmetic follows the canonical host/worker width and type rules, and
//! evaluation calls `zama-host`'s own validators, so the oracle cannot drift from the program's
//! admission rules.

use std::collections::{HashMap, HashSet};

use anchor_lang::{AnchorDeserialize, Discriminator};
use mollusk_svm::result::InstructionResult;
use num_bigint::BigUint;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use solana_sdk::pubkey::Pubkey;
use zama_host::{
    assert_binary_operand_types, assert_is_in_operand_types, assert_mul_div_operand_types,
    assert_sum_operand_types, assert_supported_fhe_type, assert_unary_operand_type,
    assert_valid_bounded_rand_upper_bound, computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_sum_handle, computed_eval_ternary_handle,
    computed_eval_trivial_handle, computed_eval_unary_handle, handle_fhe_type, FheBinaryOpCode,
    FheExecuteArgs, FheExecuteOperand, FheExecuteStep, FheTernaryOpCode, FheUnaryOpCode,
    HandleDerivationContext,
};

use crate::{decode_fhe_execute_args, Ctx, BALANCE_FHE_TYPE};

pub type Handle = [u8; 32];
pub type ClearInputs = HashMap<Handle, TypedClearValue>;

fn resolve_handle_operand(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced: &[Handle],
) -> Option<Handle> {
    match operand {
        FheExecuteOperand::StateSlot { handle_index, .. }
        | FheExecuteOperand::TransientResult { handle_index, .. } => {
            dictionary.get(*handle_index as usize).copied()
        }
        FheExecuteOperand::EarlierStep { producer_index } => {
            produced.get(*producer_index as usize).copied()
        }
        FheExecuteOperand::VerifiedInput { attestation } => Some(attestation.input_handle),
        FheExecuteOperand::Scalar { .. } => None,
    }
}

fn resolve_handle_rhs(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced: &[Handle],
) -> Option<(Handle, bool)> {
    match operand {
        FheExecuteOperand::Scalar { value_index } => dictionary
            .get(*value_index as usize)
            .copied()
            .map(|value| (value, true)),
        _ => resolve_handle_operand(operand, dictionary, produced).map(|value| (value, false)),
    }
}

fn reconstruct_handles(
    args: &FheExecuteArgs,
    context: &HandleDerivationContext,
    random_seeds: &[zama_host::FheExecuteRandomSeed],
    produced_in_tx: &mut HashSet<Handle>,
) -> Option<Vec<Handle>> {
    let mut produced = Vec::with_capacity(args.steps.len());
    for (step_index, step) in args.steps.iter().enumerate() {
        let mask = |handles: &[Option<Handle>]| {
            zama_host::operand_boundary_mask(
                handles
                    .iter()
                    .map(|handle| handle.is_some_and(|h| !produced_in_tx.contains(&h))),
            )
            .ok()
        };
        let handle = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let lhs = resolve_handle_operand(lhs, &args.dictionary, &produced)?;
                let (rhs, scalar) = resolve_handle_rhs(rhs, &args.dictionary, &produced)?;
                computed_eval_handle(
                    *op,
                    lhs,
                    rhs,
                    scalar,
                    *output_fhe_type,
                    mask(&[Some(lhs), (!scalar).then_some(rhs)])?,
                    context,
                )
            }
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                let control = resolve_handle_operand(control, &args.dictionary, &produced)?;
                let if_true = resolve_handle_operand(if_true, &args.dictionary, &produced)?;
                let if_false = resolve_handle_operand(if_false, &args.dictionary, &produced)?;
                computed_eval_ternary_handle(
                    *op,
                    control,
                    if_true,
                    if_false,
                    *output_fhe_type,
                    mask(&[Some(control), Some(if_true), Some(if_false)])?,
                    context,
                )
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => computed_eval_trivial_handle(*plaintext, *fhe_type, context),
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                ..
            } => {
                let operand = resolve_handle_operand(operand, &args.dictionary, &produced)?;
                computed_eval_unary_handle(
                    *op,
                    operand,
                    *output_fhe_type,
                    mask(&[Some(operand)])?,
                    context,
                )
            }
            FheExecuteStep::Sum {
                operands, fhe_type, ..
            } => {
                let operands = operands
                    .iter()
                    .map(|operand| resolve_handle_operand(operand, &args.dictionary, &produced))
                    .collect::<Option<Vec<_>>>()?;
                computed_eval_sum_handle(
                    &operands,
                    *fhe_type,
                    zama_host::operand_boundary_mask(
                        operands
                            .iter()
                            .map(|handle| !produced_in_tx.contains(handle)),
                    )
                    .ok()?,
                    context,
                )
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
                ..
            } => {
                let set = set
                    .iter()
                    .map(|operand| resolve_handle_operand(operand, &args.dictionary, &produced))
                    .collect::<Option<Vec<_>>>()?;
                let value = resolve_handle_operand(value, &args.dictionary, &produced)?;
                computed_eval_is_in_handle(
                    value,
                    &set,
                    *fhe_type,
                    zama_host::operand_boundary_mask(
                        std::iter::once(&value)
                            .chain(&set)
                            .map(|handle| !produced_in_tx.contains(handle)),
                    )
                    .ok()?,
                    context,
                )
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                ..
            } => {
                let factor1 = resolve_handle_operand(factor1, &args.dictionary, &produced)?;
                let (factor2, scalar) = resolve_handle_rhs(factor2, &args.dictionary, &produced)?;
                computed_eval_mul_div_handle(
                    factor1,
                    factor2,
                    *divisor,
                    scalar,
                    *output_fhe_type,
                    mask(&[Some(factor1), (!scalar).then_some(factor2)])?,
                    context,
                )
            }
            FheExecuteStep::Rand { fhe_type } => {
                let seed = random_seeds
                    .iter()
                    .find(|seed| usize::from(seed.step_index) == step_index)?
                    .seed;
                zama_host::computed_rand_handle(seed, *fhe_type, context.chain_id)
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
            } => {
                let seed = random_seeds
                    .iter()
                    .find(|seed| usize::from(seed.step_index) == step_index)?
                    .seed;
                zama_host::computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    context.chain_id,
                )
            }
        };
        produced.push(handle);
        produced_in_tx.insert(handle);
    }
    Some(produced)
}

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
                // Mirrors `assert_ternary_operand_types`; keep the malformed-ternary
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
                canonical(assert_supported_fhe_type(*fhe_type), "rand")?;
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
        FheExecuteOperand::StateSlot { handle_index, .. }
        | FheExecuteOperand::TransientResult { handle_index, .. } => {
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
        _ => {
            // Operands are widened before the host's own gate runs, so an unshipped type must
            // fail here with the host's error rather than an oracle-only message.
            canonical(assert_supported_fhe_type(fhe_type), "fhe type")?;
            unreachable!("the host admits an FHE type the oracle has no width for: {fhe_type}")
        }
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
    /// Persistent (`FheHandle`) outputs bound to their end-of-instruction handles.
    pub persistent_outputs: usize,
}

/// Cleartext mirror of every encrypted handle the tests touch.
#[derive(Default)]
pub struct CleartextLedger {
    pub values: ClearInputs,
    state_leaves: HashMap<Pubkey, Vec<[u8; 32]>>,
}

impl CleartextLedger {
    /// Seeds a `euint64` amount for `handle`.
    pub fn seed_amount(&mut self, handle: [u8; 32], value: u64) {
        self.values
            .insert(handle, TypedClearValue::from_u64(BALANCE_FHE_TYPE, value));
    }

    pub fn u64_for_handle(&self, handle: [u8; 32]) -> u64 {
        let value = self
            .values
            .get(&handle)
            .expect("cleartext value for handle");
        assert_eq!(value.fhe_type, BALANCE_FHE_TYPE);
        assert_eq!(value.value[..24], [0; 24]);
        u64::from_be_bytes(value.value[24..].try_into().unwrap())
    }

    pub fn seed_state_allow(&mut self, state: Pubkey, handle: [u8; 32], key: Pubkey) {
        let leaves = self.state_leaves.entry(state).or_default();
        let commitment = zama_solana_acl::historical_access_leaf_commitment(
            state.to_bytes(),
            0,
            handle,
            key.to_bytes(),
        );
        if leaves.is_empty() {
            leaves.push(commitment);
        } else {
            assert_eq!(leaves[0], commitment, "different initial state history");
        }
    }

    /// Replays every `fhe_execute` CPI the instruction issued — in order, so a later execution
    /// can consume an earlier execution's persisted outputs. Each instruction writes any
    /// encrypted State at most once, so binding results to the end-of-instruction
    /// persisted handles is exact.
    pub fn replay_fhe_cpis(&mut self, context: &Ctx, result: &InstructionResult) -> FheReplay {
        let message = result
            .message
            .as_ref()
            .expect("Mollusk result must include its compiled message");
        enum HostReplay<'a> {
            Execute(
                FheExecuteArgs,
                &'a [u8],
                Vec<zama_host::FheExecuteRandomSeed>,
            ),
            MakePublic(zama_host::instruction::MakeStateHandlePublic, &'a [u8]),
        }
        let host_instructions = result
            .inner_instructions
            .iter()
            .enumerate()
            .filter(|(_, inner)| {
                message
                    .account_keys()
                    .get(inner.instruction.program_id_index as usize)
                    .copied()
                    == Some(zama_host::id())
            })
            .filter_map(|(index, inner)| {
                let accounts = inner.instruction.accounts.as_slice();
                if let Some(args) = decode_fhe_execute_args(&inner.instruction.data) {
                    let mut events = result.inner_instructions[index + 1..]
                        .iter()
                        .take_while(|child| child.stack_height > inner.stack_height)
                        .filter(|child| {
                            message.account_keys()[child.instruction.program_id_index as usize]
                                == zama_host::ID
                        })
                        .filter_map(|child| {
                            crate::decode_anchor_event::<zama_host::FheExecuteRandomSeedsEvent>(
                                &child.instruction.data,
                            )
                        });
                    let seeds = events
                        .next()
                        .map(|event| {
                            assert_eq!(event.version, zama_host::EVENT_VERSION);
                            event.seeds
                        })
                        .unwrap_or_default();
                    assert!(
                        events.next().is_none(),
                        "one random-seeds event per execution"
                    );
                    return Some(HostReplay::Execute(args, accounts, seeds));
                }
                let payload = inner
                    .instruction
                    .data
                    .strip_prefix(zama_host::instruction::MakeStateHandlePublic::DISCRIMINATOR)?;
                zama_host::instruction::MakeStateHandlePublic::deserialize(&mut &*payload)
                    .ok()
                    .map(|args| HostReplay::MakePublic(args, accounts))
            })
            .collect::<Vec<_>>();

        let mut produced_in_tx = HashSet::new();
        let mut executions = 0;
        let mut persistent_outputs = 0;
        for instruction in host_instructions {
            let HostReplay::Execute(args, accounts, random_seeds) = instruction else {
                let HostReplay::MakePublic(args, accounts) = instruction else {
                    unreachable!()
                };
                let account_index = accounts[2] as usize;
                let address = message.account_keys()[account_index];
                let leaves = self.state_leaves.entry(address).or_default();
                assert_eq!(
                    leaves.len() as u64,
                    args.previous_leaf_count,
                    "oracle missed EncryptedState history before public sealing {address}"
                );
                leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
                    address.to_bytes(),
                    leaves.len() as u64,
                    args.handle,
                ));
                continue;
            };
            executions += 1;
            let outputs = evaluate(&args, &self.values)
                .expect("every emitted FHE batch must be valid in cleartext");
            let slot = context.mollusk.sysvars.clock.slot;
            let previous_bank_hash = context
                .mollusk
                .sysvars
                .slot_hashes
                .iter()
                .find(|(candidate, _)| *candidate < slot)
                .map(|(_, hash)| hash.to_bytes())
                .expect("test runtime must contain a previous bank hash");
            let handle_context = HandleDerivationContext {
                chain_id: zama_host::SOLANA_POC_CHAIN_ID,
                previous_bank_hash,
                unix_timestamp: context.mollusk.sysvars.clock.unix_timestamp,
            };
            let handles =
                reconstruct_handles(&args, &handle_context, &random_seeds, &mut produced_in_tx)
                    .expect("host CPI and its random-seeds event must reconstruct every result");
            for (&handle, value) in handles.iter().zip(outputs) {
                self.values.insert(handle, value);
            }
            for effect in &args.effects {
                let handle = handles[usize::from(effect.result.step_index)];
                let account_index = accounts
                    [zama_host::FHE_EXECUTE_FIXED_ACCOUNTS + usize::from(effect.state_index)]
                    as usize;
                let address = message.account_keys()[account_index];
                let leaves = self.state_leaves.entry(address).or_default();
                assert_eq!(
                    leaves.len() as u64,
                    effect.previous_leaf_count,
                    "oracle missed EncryptedState history before {address}: {effect:?}"
                );
                for allow_index in &effect.allow_indexes {
                    let key = args
                        .dictionary_bytes(*allow_index)
                        .expect("valid allow dictionary index");
                    leaves.push(zama_solana_acl::historical_access_leaf_commitment(
                        address.to_bytes(),
                        leaves.len() as u64,
                        handle,
                        key,
                    ));
                }
                if effect.make_public {
                    leaves.push(zama_solana_acl::public_decrypt_leaf_commitment(
                        address.to_bytes(),
                        leaves.len() as u64,
                        handle,
                    ));
                }
                persistent_outputs += usize::from(effect.slot.is_some());
            }
        }
        FheReplay {
            executions,
            persistent_outputs,
        }
    }

    pub fn u64_in_state(&self, context: &Ctx, address: Pubkey, key: [u8; 32]) -> u64 {
        let state: zama_host::EncryptedState = crate::read_account(context, address);
        let value = self
            .values
            .get(&state.get(&key).expect("state slot"))
            .expect("cleartext value");
        assert_eq!(value.fhe_type, BALANCE_FHE_TYPE);
        assert_eq!(value.value[..24], [0; 24]);
        u64::from_be_bytes(value.value[24..].try_into().unwrap())
    }

    pub fn public_decrypt_proof(
        &self,
        state: Pubkey,
        handle: [u8; 32],
    ) -> zama_host::instructions::MmrInclusionProof {
        let leaves = self
            .state_leaves
            .get(&state)
            .expect("oracle history for encrypted state");
        let leaf_index = leaves
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, commitment)| {
                (*commitment
                    == zama_solana_acl::public_decrypt_leaf_commitment(
                        state.to_bytes(),
                        index as u64,
                        handle,
                    ))
                .then_some(index as u64)
            })
            .expect("public-decrypt leaf for handle");
        let proof = zama_solana_acl::mmr_build_proof(leaves, leaf_index)
            .expect("public-decrypt inclusion proof");
        zama_host::instructions::MmrInclusionProof {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings,
        }
    }
}
