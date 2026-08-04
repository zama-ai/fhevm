//! Local mirrors of the host preflight rules so malformed executions fail before the CPI.

use anchor_lang::prelude::Pubkey;

use zama_host::{
    FheBinaryOpCode, FheExecuteOperand, FheExecuteOutput, FheExecuteStep, FheUnaryOpCode,
};

use crate::accounts::{ExecutionAccountMeta, ExecutionEncryptedValueAccountAuthority};
use crate::acl::EncryptedValueId;
use crate::operand::{Operand, OperandKind};
use crate::{FheExecutionBuildError, Result};

/// Mirrors the host preflight rule (fhevm-internal#1853 W4): rand seeds are anchored to
/// the execution's persistent writes, so an execution with a rand step and no persistent output is
/// rejected here before it fails on-chain with `FheExecuteRandRequiresPersistentOutput`.
pub(crate) fn validate_rand_steps_anchor_persistent_output(steps: &[FheExecuteStep]) -> Result<()> {
    let has_rand = steps.iter().any(|step| {
        matches!(
            step,
            FheExecuteStep::Rand { .. } | FheExecuteStep::RandBounded { .. }
        )
    });
    if !has_rand {
        return Ok(());
    }
    let persists = steps.iter().any(|step| {
        matches!(
            step,
            FheExecuteStep::Binary {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::Ternary {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::TrivialEncrypt {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::Rand {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::Unary {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::RandBounded {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::Sum {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::IsIn {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            } | FheExecuteStep::MulDiv {
                output: FheExecuteOutput::StoredValue { .. },
                ..
            }
        )
    });
    if !persists {
        return Err(FheExecutionBuildError::RandRequiresPersistentOutput);
    }
    Ok(())
}

pub(crate) fn validate_lowered_execution(
    steps: &[FheExecuteStep],
    remaining_accounts: &[ExecutionAccountMeta],
    dictionary: &[[u8; 32]],
) -> Result<()> {
    if u8::try_from(remaining_accounts.len()).is_err() {
        return Err(FheExecutionBuildError::TooManyRemainingAccounts);
    }
    if u8::try_from(dictionary.len()).is_err() {
        return Err(FheExecutionBuildError::TooManyDictionaryEntries);
    }
    for (index, account) in remaining_accounts.iter().enumerate() {
        if account.pubkey == Pubkey::default() || account.purposes.is_empty() {
            return Err(FheExecutionBuildError::InvalidRemainingAccountReference);
        }
        if remaining_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.pubkey == account.pubkey)
        {
            return Err(FheExecutionBuildError::InvalidRemainingAccountReference);
        }
    }

    let mut used_accounts = vec![false; remaining_accounts.len()];
    let mut used_dictionary = vec![false; dictionary.len()];
    for (step_index, step) in steps.iter().enumerate() {
        validate_lowered_step(step, step_index, &mut used_accounts, &mut used_dictionary)?;
    }
    if used_accounts.iter().any(|used| !*used) {
        return Err(FheExecutionBuildError::InvalidRemainingAccountReference);
    }
    // Mirrors the host's whole-execution dictionary hygiene rule: every interned entry must be referenced.
    if used_dictionary.iter().any(|used| !*used) {
        return Err(FheExecutionBuildError::UnreferencedDictionaryEntry);
    }
    Ok(())
}

fn validate_lowered_step(
    step: &FheExecuteStep,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match step {
        FheExecuteStep::Binary {
            lhs, rhs, output, ..
        } => {
            validate_lowered_encrypted_operand(lhs, step_index, used_accounts, used_dictionary)?;
            validate_lowered_rhs_operand(rhs, step_index, used_accounts, used_dictionary)?;
            validate_lowered_output(output, used_accounts, used_dictionary)?;
        }
        FheExecuteStep::Ternary {
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
        FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::RandBounded { output, .. } => {
            validate_lowered_output(output, used_accounts, used_dictionary)?
        }
        FheExecuteStep::Unary {
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
        FheExecuteStep::Sum {
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
        FheExecuteStep::IsIn {
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
        FheExecuteStep::MulDiv {
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
    operand: &FheExecuteOperand,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match operand {
        FheExecuteOperand::Scalar { value_index } => {
            mark_lowered_dictionary_entry(used_dictionary, *value_index)
        }
        _ => {
            validate_lowered_encrypted_operand(operand, step_index, used_accounts, used_dictionary)
        }
    }
}

fn validate_lowered_encrypted_operand(
    operand: &FheExecuteOperand,
    step_index: usize,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match operand {
        FheExecuteOperand::StoredValue {
            handle_index,
            encrypted_value_index,
        } => {
            mark_lowered_dictionary_entry(used_dictionary, *handle_index)?;
            mark_lowered_account(used_accounts, *encrypted_value_index)?;
        }
        FheExecuteOperand::EarlierStep { producer_index } => {
            if usize::from(*producer_index) >= step_index {
                return Err(FheExecutionBuildError::InvalidTransientReference);
            }
        }
        FheExecuteOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-execution.
        }
        FheExecuteOperand::Scalar { .. } => {
            return Err(FheExecutionBuildError::ScalarEncryptedOperand)
        }
    }
    Ok(())
}

fn validate_lowered_output(
    output: &FheExecuteOutput,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match output {
        FheExecuteOutput::Transient => {}
        FheExecuteOutput::StoredValue {
            output_encrypted_value_index,
            output_authority_index,
            output_domain_index,
            output_account_index,
            output_label_index,
            output_subject_indexes,
            ..
        } => {
            mark_lowered_account(used_accounts, *output_encrypted_value_index)?;
            if let Some(index) = output_authority_index {
                mark_lowered_account(used_accounts, *index)?;
            }
            mark_lowered_dictionary_entry(used_dictionary, *output_domain_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *output_account_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *output_label_index)?;
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
        .ok_or(FheExecutionBuildError::InvalidRemainingAccountReference)?;
    *used = true;
    Ok(())
}

fn mark_lowered_dictionary_entry(used_dictionary: &mut [bool], index: u8) -> Result<()> {
    let used = used_dictionary
        .get_mut(usize::from(index))
        .ok_or(FheExecutionBuildError::DictionaryIndexOutOfBounds)?;
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
    produced_type: F,
) -> Result<()>
where
    F: Fn(u8) -> Option<u8>,
{
    validate_supported_binary_output_type(op, output_fhe_type)?;

    let lhs_type = operand_fhe_type(lhs, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarLhsOperand)?;
    match op {
        // Eq/Ne accept the widest operand set (Bool..Uint256); ordered comparisons Uint8..Uint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            if !matches!(lhs_type, 0 | 2..=8) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            if !matches!(lhs_type, 2..=6) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            if lhs_type != output_fhe_type {
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
            match &rhs.0 {
                OperandKind::Scalar(value) => {
                    if scalar_is_zero_for_type(*value, output_fhe_type) {
                        return Err(FheExecutionBuildError::DivisionByZero);
                    }
                }
                OperandKind::Persistent(_)
                | OperandKind::Transient { .. }
                | OperandKind::VerifiedInput { .. } => {
                    return Err(FheExecutionBuildError::DivisorMustBeScalar)
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
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
        }
    }
    if let Some(rhs_type) = operand_fhe_type(rhs, produced_count, &produced_type)? {
        if rhs_type != lhs_type {
            return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
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
    produced_type: F,
) -> Result<()>
where
    F: Fn(u8) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;
    let valid_output = match op {
        FheUnaryOpCode::Neg => matches!(output_fhe_type, 2..=6 | 8),
        FheUnaryOpCode::Not => matches!(output_fhe_type, 0 | 2..=6 | 8),
        // EVM `cast` output set: Uint8..Uint128 | Uint256 (no ebool, no eaddress/Uint160).
        FheUnaryOpCode::Cast => matches!(output_fhe_type, 2..=6 | 8),
    };
    if !valid_output {
        return Err(FheExecutionBuildError::UnsupportedFheType);
    }
    let operand_type = operand_fhe_type(operand, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarEncryptedOperand)?;
    validate_supported_fhe_type(operand_type)?;
    match op {
        FheUnaryOpCode::Neg => {
            if !matches!(operand_type, 2..=6 | 8) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Not => {
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Cast => {
            // EVM `cast` input set: Bool | Uint8..Uint128 | Uint256 (no eaddress/Uint160).
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
            // Same-type cast is rejected (EVM InvalidType parity).
            if operand_type == output_fhe_type {
                return Err(FheExecutionBuildError::UnsupportedFheType);
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
) -> Result<()>
where
    F: Fn(u8) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;

    let control_type = operand_fhe_type(control, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarEncryptedOperand)?;
    let true_type = operand_fhe_type(if_true, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarEncryptedOperand)?;
    let false_type = operand_fhe_type(if_false, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarEncryptedOperand)?;

    if control_type != 0 || true_type != output_fhe_type || false_type != output_fhe_type {
        return Err(FheExecutionBuildError::TernaryOperandTypeMismatch);
    }
    Ok(())
}

pub(crate) fn operand_fhe_type<F>(
    operand: &Operand,
    produced_count: usize,
    produced_type: &F,
) -> Result<Option<u8>>
where
    F: Fn(u8) -> Option<u8>,
{
    match &operand.0 {
        OperandKind::Persistent(persistent) => Ok(Some(handle_fhe_type(persistent.handle))),
        OperandKind::Transient { producer_index } => {
            if *producer_index as usize >= produced_count {
                return Err(FheExecutionBuildError::InvalidTransientReference);
            }
            produced_type(*producer_index)
                .map(Some)
                .ok_or(FheExecutionBuildError::InvalidTransientReference)
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
        return Err(FheExecutionBuildError::UnsupportedBinaryOutputType);
    }
    Ok(())
}

pub(crate) fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8) {
        Ok(())
    } else {
        Err(FheExecutionBuildError::UnsupportedFheType)
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
        Err(FheExecutionBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_supported_rand_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8) {
        Ok(())
    } else {
        Err(FheExecutionBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_subjects(subjects: &[Pubkey]) -> Result<()> {
    if subjects.is_empty() || subjects.len() > zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS {
        return Err(FheExecutionBuildError::InvalidSubjects);
    }
    for (index, subject) in subjects.iter().enumerate() {
        if *subject == Pubkey::default() {
            return Err(FheExecutionBuildError::InvalidSubjects);
        }
        if subjects[..index].contains(subject) {
            return Err(FheExecutionBuildError::InvalidSubjects);
        }
    }
    Ok(())
}

pub(crate) fn validate_encrypted_value_id(key: &EncryptedValueId) -> Result<()> {
    if key.domain.pubkey() == Pubkey::default()
        || key.encrypted_value_account_authority == Pubkey::default()
    {
        return Err(FheExecutionBuildError::InvalidEncryptedValueId);
    }
    Ok(())
}

pub(crate) fn validate_encrypted_value_account_authority(
    authority: ExecutionEncryptedValueAccountAuthority,
) -> Result<()> {
    if authority.pubkey() == Pubkey::default() {
        return Err(FheExecutionBuildError::InvalidAppAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
