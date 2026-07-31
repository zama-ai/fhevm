//! Local mirrors of the host preflight rules so malformed frames fail before the CPI.

use anchor_lang::prelude::Pubkey;

use zama_host::{FheBinaryOpCode, FheEvalOperand, FheEvalOutput, FheEvalStep, FheUnaryOpCode};

use crate::accounts::{EvalAccountMeta, EvalAppAuthority};
use crate::acl::EncryptedValueKey;
use crate::operand::{EvalBuilderScope, Operand, OperandKind};
use crate::{EvalBuildError, Result};

/// Mirrors the host preflight rule (fhevm-internal#1853 W4): rand seeds are anchored to
/// the frame's durable writes, so a frame with a rand step and no durable output is
/// rejected here before it fails on-chain with `FheEvalRandRequiresDurableOutput`.
pub(crate) fn validate_rand_steps_anchor_durable_output(steps: &[FheEvalStep]) -> Result<()> {
    let has_rand = steps.iter().any(|step| {
        matches!(
            step,
            FheEvalStep::Rand { .. } | FheEvalStep::RandBounded { .. }
        )
    });
    if !has_rand {
        return Ok(());
    }
    let persists = steps.iter().any(|step| {
        matches!(
            step,
            FheEvalStep::Binary {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::Ternary {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::TrivialEncrypt {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::Rand {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::Unary {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::RandBounded {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::Sum {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::IsIn {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            } | FheEvalStep::MulDiv {
                output: FheEvalOutput::AllowedDurable { .. },
                ..
            }
        )
    });
    if !persists {
        return Err(EvalBuildError::RandRequiresDurableOutput);
    }
    Ok(())
}

pub(crate) fn validate_lowered_eval_plan(
    steps: &[FheEvalStep],
    remaining_accounts: &[EvalAccountMeta],
    dictionary: &[[u8; 32]],
) -> Result<()> {
    if u8::try_from(remaining_accounts.len()).is_err() {
        return Err(EvalBuildError::TooManyRemainingAccounts);
    }
    if u8::try_from(dictionary.len()).is_err() {
        return Err(EvalBuildError::TooManyDictionaryEntries);
    }
    for (index, account) in remaining_accounts.iter().enumerate() {
        if account.pubkey == Pubkey::default() || account.purposes.is_empty() {
            return Err(EvalBuildError::InvalidRemainingAccountReference);
        }
        if remaining_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.pubkey == account.pubkey)
        {
            return Err(EvalBuildError::InvalidRemainingAccountReference);
        }
    }

    let mut used_accounts = vec![false; remaining_accounts.len()];
    let mut used_dictionary = vec![false; dictionary.len()];
    for (step_index, step) in steps.iter().enumerate() {
        validate_lowered_step(step, step_index, &mut used_accounts, &mut used_dictionary)?;
    }
    if used_accounts.iter().any(|used| !*used) {
        return Err(EvalBuildError::InvalidRemainingAccountReference);
    }
    // Mirrors the host's whole-frame dictionary hygiene rule: every interned entry must be referenced.
    if used_dictionary.iter().any(|used| !*used) {
        return Err(EvalBuildError::UnreferencedDictionaryEntry);
    }
    Ok(())
}

fn validate_lowered_step(
    step: &FheEvalStep,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match step {
        FheEvalStep::Binary {
            lhs, rhs, output, ..
        } => {
            validate_lowered_encrypted_operand(lhs, step_index, used_accounts, used_dictionary)?;
            validate_lowered_rhs_operand(rhs, step_index, used_accounts, used_dictionary)?;
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheEvalStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => {
            validate_lowered_encrypted_operand(
                control,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_encrypted_operand(
                if_true,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_encrypted_operand(
                if_false,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::RandBounded { output, .. } => {
            validate_lowered_output(output, used_accounts, used_dictionary)?
        }
        FheEvalStep::Unary {
            operand, output, ..
        } => {
            validate_lowered_encrypted_operand(
                operand,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheEvalStep::Sum {
            operands, output, ..
        } => {
            for operand in operands {
                validate_lowered_encrypted_operand(
                    operand,
                    step_index,
                    used_accounts,
                    used_dictionary,
                )?;
            }
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheEvalStep::IsIn {
            value, set, output, ..
        } => {
            validate_lowered_encrypted_operand(value, step_index, used_accounts, used_dictionary)?;
            for operand in set {
                validate_lowered_encrypted_operand(
                    operand,
                    step_index,
                    used_accounts,
                    used_dictionary,
                )?;
            }
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheEvalStep::MulDiv {
            factor1,
            factor2,
            output,
            ..
        } => {
            validate_lowered_encrypted_operand(
                factor1,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_rhs_operand(factor2, step_index, used_accounts, used_dictionary)?;
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
    }
    Ok(())
}

fn validate_lowered_rhs_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match operand {
        FheEvalOperand::Scalar { value_index } => {
            mark_lowered_dictionary_entry(used_dictionary, *value_index)
        }
        _ => {
            validate_lowered_encrypted_operand(operand, step_index, used_accounts, used_dictionary)
        }
    }
}

fn validate_lowered_encrypted_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match operand {
        FheEvalOperand::AllowedDurable {
            handle_index,
            encrypted_value_index,
        } => {
            mark_lowered_dictionary_entry(used_dictionary, *handle_index)?;
            mark_lowered_account(used_accounts, *encrypted_value_index)?;
        }
        FheEvalOperand::AllowedLocal { producer_index } => {
            if usize::from(*producer_index) >= step_index {
                return Err(EvalBuildError::InvalidTransientReference);
            }
        }
        FheEvalOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-frame.
        }
        FheEvalOperand::Scalar { .. } => return Err(EvalBuildError::ScalarEncryptedOperand),
    }
    Ok(())
}

fn validate_lowered_output(
    output: &FheEvalOutput,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match output {
        FheEvalOutput::AllowedLocal => {}
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index,
            output_acl_domain_key_index,
            output_app_account_index,
            output_encrypted_value_label_index,
            output_subject_indexes,
            ..
        } => {
            mark_lowered_account(used_accounts, *output_encrypted_value_index)?;
            if let Some(index) = output_app_account_authority_index {
                mark_lowered_account(used_accounts, *index)?;
            }
            mark_lowered_dictionary_entry(used_dictionary, *output_acl_domain_key_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *output_app_account_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *output_encrypted_value_label_index)?;
            for index in output_subject_indexes {
                mark_lowered_dictionary_entry(used_dictionary, *index)?;
            }
        }
    }
    Ok(())
}

fn mark_lowered_account(used_accounts: &mut [bool], index: u8) -> Result<()> {
    let used = used_accounts
        .get_mut(usize::from(index))
        .ok_or(EvalBuildError::InvalidRemainingAccountReference)?;
    *used = true;
    Ok(())
}

fn mark_lowered_dictionary_entry(used_dictionary: &mut [bool], index: u8) -> Result<()> {
    let used = used_dictionary
        .get_mut(usize::from(index))
        .ok_or(EvalBuildError::DictionaryIndexOutOfBounds)?;
    *used = true;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_binary_step<F>(
    op: FheBinaryOpCode,
    lhs: &Operand,
    rhs: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    produced_type: F,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_binary_output_type(op, output_fhe_type)?;

    let lhs_type = operand_fhe_type(lhs, produced_count, builder_scope, &produced_type)?
        .ok_or(EvalBuildError::ScalarLhsOperand)?;
    match op {
        // Eq/Ne accept the widest operand set (Bool..Uint256); ordered comparisons Uint8..Uint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            if !matches!(lhs_type, 0 | 2..=8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            if !matches!(lhs_type, 2..=6) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            if lhs_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
            match &rhs.0 {
                OperandKind::Scalar(value) => {
                    if scalar_is_zero_for_type(*value, output_fhe_type) {
                        return Err(EvalBuildError::DivisionByZero);
                    }
                }
                OperandKind::Durable(_)
                | OperandKind::Transient { .. }
                | OperandKind::VerifiedInput { .. } => {
                    return Err(EvalBuildError::DivisorMustBeScalar)
                }
            }
        }
        // Remaining ops: operand type must equal the (op-gated) output type.
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
            if lhs_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
    }
    if let Some(rhs_type) = operand_fhe_type(rhs, produced_count, builder_scope, &produced_type)? {
        if rhs_type != lhs_type {
            return Err(EvalBuildError::BinaryOperandTypeMismatch);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_unary_step<F>(
    op: FheUnaryOpCode,
    operand: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    produced_type: F,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;
    let valid_output = match op {
        FheUnaryOpCode::Neg => matches!(output_fhe_type, 2..=6 | 8),
        FheUnaryOpCode::Not => matches!(output_fhe_type, 0 | 2..=6 | 8),
        // EVM `cast` output set: Uint8..Uint128 | Uint256 (no ebool, no eaddress/Uint160).
        FheUnaryOpCode::Cast => matches!(output_fhe_type, 2..=6 | 8),
    };
    if !valid_output {
        return Err(EvalBuildError::UnsupportedFheType);
    }
    let operand_type = operand_fhe_type(operand, produced_count, builder_scope, &produced_type)?
        .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    validate_supported_fhe_type(operand_type)?;
    match op {
        FheUnaryOpCode::Neg => {
            if !matches!(operand_type, 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Not => {
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Cast => {
            // EVM `cast` input set: Bool | Uint8..Uint128 | Uint256 (no eaddress/Uint160).
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            // Same-type cast is rejected (EVM InvalidType parity).
            if operand_type == output_fhe_type {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_ternary_step<F>(
    control: &Operand,
    if_true: &Operand,
    if_false: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    produced_type: F,
    builder_scope: EvalBuilderScope,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;

    let control_type = operand_fhe_type(control, produced_count, builder_scope, &produced_type)?
        .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    let true_type = operand_fhe_type(if_true, produced_count, builder_scope, &produced_type)?
        .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    let false_type = operand_fhe_type(if_false, produced_count, builder_scope, &produced_type)?
        .ok_or(EvalBuildError::ScalarEncryptedOperand)?;

    if control_type != 0 || true_type != output_fhe_type || false_type != output_fhe_type {
        return Err(EvalBuildError::TernaryOperandTypeMismatch);
    }
    Ok(())
}

pub(crate) fn operand_fhe_type<F>(
    operand: &Operand,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    produced_type: &F,
) -> Result<Option<u8>>
where
    F: Fn(u16) -> Option<u8>,
{
    match &operand.0 {
        OperandKind::Durable(durable) => Ok(Some(handle_fhe_type(durable.handle))),
        OperandKind::Transient {
            producer_index,
            builder_scope: operand_builder_scope,
        } => {
            if *operand_builder_scope != builder_scope {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            if *producer_index as usize >= produced_count {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            produced_type(*producer_index)
                .map(Some)
                .ok_or(EvalBuildError::InvalidTransientReference)
        }
        OperandKind::VerifiedInput { input_handle, .. } => Ok(Some(handle_fhe_type(*input_handle))),
        OperandKind::Scalar(_) => Ok(None),
    }
}

pub(crate) fn validate_supported_binary_output_type(
    op: FheBinaryOpCode,
    output_fhe_type: u8,
) -> Result<()> {
    validate_supported_fhe_type(output_fhe_type)?;
    let valid = match op {
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::Div
        | FheBinaryOpCode::Rem
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => matches!(output_fhe_type, 2..=6),
        FheBinaryOpCode::And | FheBinaryOpCode::Or | FheBinaryOpCode::Xor => {
            matches!(output_fhe_type, 0 | 2..=6 | 8)
        }
        FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr => matches!(output_fhe_type, 2..=6 | 8),
        FheBinaryOpCode::Eq
        | FheBinaryOpCode::Ne
        | FheBinaryOpCode::Ge
        | FheBinaryOpCode::Gt
        | FheBinaryOpCode::Le
        | FheBinaryOpCode::Lt => output_fhe_type == 0,
    };
    if !valid {
        return Err(EvalBuildError::UnsupportedBinaryOutputType);
    }
    Ok(())
}

pub(crate) fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

/// Mirrors the host `scalar_is_zero_for_type` (EVM `_isScalarZeroForType`): zero after width truncation.
pub(crate) fn scalar_is_zero_for_type(scalar: [u8; 32], fhe_type: u8) -> bool {
    let width = match fhe_type {
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 8,
        6 => 16,
        _ => 32,
    };
    scalar[32 - width..].iter().all(|byte| *byte == 0)
}

/// Coprocessor max operand count for FheSum / FheIsIn: 100 for narrow types, 60 for wider ones.
pub(crate) fn max_reduction_operands(fhe_type: u8) -> usize {
    match fhe_type {
        2..=4 => 100,
        _ => 60,
    }
}

pub(crate) fn validate_uint_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 2..=6) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_supported_rand_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_subjects(subjects: &[Pubkey]) -> Result<()> {
    if subjects.is_empty() || subjects.len() > zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS {
        return Err(EvalBuildError::InvalidSubjects);
    }
    for (index, subject) in subjects.iter().enumerate() {
        if *subject == Pubkey::default() {
            return Err(EvalBuildError::InvalidSubjects);
        }
        if subjects[..index].contains(subject) {
            return Err(EvalBuildError::InvalidSubjects);
        }
    }
    Ok(())
}

pub(crate) fn validate_encrypted_value_key(key: &EncryptedValueKey) -> Result<()> {
    if key.namespace == Pubkey::default() || key.account == Pubkey::default() {
        return Err(EvalBuildError::InvalidEncryptedValueKey);
    }
    Ok(())
}

pub(crate) fn validate_app_authority(authority: EvalAppAuthority) -> Result<()> {
    if authority.pubkey() == Pubkey::default() {
        return Err(EvalBuildError::InvalidAppAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
