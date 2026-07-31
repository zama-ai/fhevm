//! Local mirrors of the host preflight rules so malformed batches fail before the CPI.

use anchor_lang::prelude::Pubkey;

use zama_host::{
    FheBinaryOpCode, FheExecuteOperand, FheExecuteOutput, FheExecuteStep, FheUnaryOpCode,
};

use crate::accounts::{BatchAccountMeta, BatchAppAuthority};
use crate::acl::EncryptedValueId;
use crate::operand::{BatchBuilderScope, Operand, OperandKind};
use crate::{BatchBuildError, Result};

/// Mirrors the host preflight rule (fhevm-internal#1853 W4): rand seeds are anchored to
/// the batch's persistent writes, so a batch with a rand step and no persistent output is
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
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::Ternary {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::TrivialEncrypt {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::Rand {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::Unary {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::RandBounded {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::Sum {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::IsIn {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            } | FheExecuteStep::MulDiv {
                output: FheExecuteOutput::AllowedPersistent { .. },
                ..
            }
        )
    });
    if !persists {
        return Err(BatchBuildError::RandRequiresPersistentOutput);
    }
    Ok(())
}

pub(crate) fn validate_lowered_batch(
    steps: &[FheExecuteStep],
    remaining_accounts: &[BatchAccountMeta],
    dictionary: &[[u8; 32]],
) -> Result<()> {
    if u8::try_from(remaining_accounts.len()).is_err() {
        return Err(BatchBuildError::TooManyRemainingAccounts);
    }
    if u8::try_from(dictionary.len()).is_err() {
        return Err(BatchBuildError::TooManyDictionaryEntries);
    }
    for (index, account) in remaining_accounts.iter().enumerate() {
        if account.pubkey == Pubkey::default() || account.purposes.is_empty() {
            return Err(BatchBuildError::InvalidRemainingAccountReference);
        }
        if remaining_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.pubkey == account.pubkey)
        {
            return Err(BatchBuildError::InvalidRemainingAccountReference);
        }
    }

    let mut used_accounts = vec![false; remaining_accounts.len()];
    let mut used_dictionary = vec![false; dictionary.len()];
    for (step_index, step) in steps.iter().enumerate() {
        validate_lowered_step(step, step_index, &mut used_accounts, &mut used_dictionary)?;
    }
    if used_accounts.iter().any(|used| !*used) {
        return Err(BatchBuildError::InvalidRemainingAccountReference);
    }
    // Mirrors the host's whole-batch dictionary hygiene rule: every interned entry must be referenced.
    if used_dictionary.iter().any(|used| !*used) {
        return Err(BatchBuildError::UnreferencedDictionaryEntry);
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
        FheExecuteOperand::AllowedPersistent {
            handle_index,
            encrypted_value_index,
        } => {
            mark_lowered_dictionary_entry(used_dictionary, *handle_index)?;
            mark_lowered_account(used_accounts, *encrypted_value_index)?;
        }
        FheExecuteOperand::AllowedLocal { producer_index } => {
            if usize::from(*producer_index) >= step_index {
                return Err(BatchBuildError::InvalidTransientReference);
            }
        }
        FheExecuteOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-batch.
        }
        FheExecuteOperand::Scalar { .. } => return Err(BatchBuildError::ScalarEncryptedOperand),
    }
    Ok(())
}

fn validate_lowered_output(
    output: &FheExecuteOutput,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    match output {
        FheExecuteOutput::AllowedLocal => {}
        FheExecuteOutput::AllowedPersistent {
            output_encrypted_value_index,
            output_account_authority_index,
            output_domain_index,
            output_account_index,
            output_label_index,
            output_subject_indexes,
            ..
        } => {
            mark_lowered_account(used_accounts, *output_encrypted_value_index)?;
            if let Some(index) = output_account_authority_index {
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
        .ok_or(BatchBuildError::InvalidRemainingAccountReference)?;
    *used = true;
    Ok(())
}

fn mark_lowered_dictionary_entry(used_dictionary: &mut [bool], index: u8) -> Result<()> {
    let used = used_dictionary
        .get_mut(usize::from(index))
        .ok_or(BatchBuildError::DictionaryIndexOutOfBounds)?;
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
    builder_scope: BatchBuilderScope,
    produced_type: F,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_binary_output_type(op, output_fhe_type)?;

    let lhs_type = operand_fhe_type(lhs, produced_count, builder_scope, &produced_type)?
        .ok_or(BatchBuildError::ScalarLhsOperand)?;
    match op {
        // Eq/Ne accept the widest operand set (Bool..Uint256); ordered comparisons Uint8..Uint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            if !matches!(lhs_type, 0 | 2..=8) {
                return Err(BatchBuildError::UnsupportedFheType);
            }
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            if !matches!(lhs_type, 2..=6) {
                return Err(BatchBuildError::UnsupportedFheType);
            }
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            if lhs_type != output_fhe_type {
                return Err(BatchBuildError::BinaryOperandTypeMismatch);
            }
            match &rhs.0 {
                OperandKind::Scalar(value) => {
                    if scalar_is_zero_for_type(*value, output_fhe_type) {
                        return Err(BatchBuildError::DivisionByZero);
                    }
                }
                OperandKind::Persistent(_)
                | OperandKind::Transient { .. }
                | OperandKind::VerifiedInput { .. } => {
                    return Err(BatchBuildError::DivisorMustBeScalar)
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
                return Err(BatchBuildError::BinaryOperandTypeMismatch);
            }
        }
    }
    if let Some(rhs_type) = operand_fhe_type(rhs, produced_count, builder_scope, &produced_type)? {
        if rhs_type != lhs_type {
            return Err(BatchBuildError::BinaryOperandTypeMismatch);
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
    builder_scope: BatchBuilderScope,
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
        return Err(BatchBuildError::UnsupportedFheType);
    }
    let operand_type = operand_fhe_type(operand, produced_count, builder_scope, &produced_type)?
        .ok_or(BatchBuildError::ScalarEncryptedOperand)?;
    validate_supported_fhe_type(operand_type)?;
    match op {
        FheUnaryOpCode::Neg => {
            if !matches!(operand_type, 2..=6 | 8) {
                return Err(BatchBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(BatchBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Not => {
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(BatchBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(BatchBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Cast => {
            // EVM `cast` input set: Bool | Uint8..Uint128 | Uint256 (no eaddress/Uint160).
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(BatchBuildError::UnsupportedFheType);
            }
            // Same-type cast is rejected (EVM InvalidType parity).
            if operand_type == output_fhe_type {
                return Err(BatchBuildError::UnsupportedFheType);
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
    builder_scope: BatchBuilderScope,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;

    let control_type = operand_fhe_type(control, produced_count, builder_scope, &produced_type)?
        .ok_or(BatchBuildError::ScalarEncryptedOperand)?;
    let true_type = operand_fhe_type(if_true, produced_count, builder_scope, &produced_type)?
        .ok_or(BatchBuildError::ScalarEncryptedOperand)?;
    let false_type = operand_fhe_type(if_false, produced_count, builder_scope, &produced_type)?
        .ok_or(BatchBuildError::ScalarEncryptedOperand)?;

    if control_type != 0 || true_type != output_fhe_type || false_type != output_fhe_type {
        return Err(BatchBuildError::TernaryOperandTypeMismatch);
    }
    Ok(())
}

pub(crate) fn operand_fhe_type<F>(
    operand: &Operand,
    produced_count: usize,
    builder_scope: BatchBuilderScope,
    produced_type: &F,
) -> Result<Option<u8>>
where
    F: Fn(u16) -> Option<u8>,
{
    match &operand.0 {
        OperandKind::Persistent(persistent) => Ok(Some(handle_fhe_type(persistent.handle))),
        OperandKind::Transient {
            producer_index,
            builder_scope: operand_builder_scope,
        } => {
            if *operand_builder_scope != builder_scope {
                return Err(BatchBuildError::InvalidTransientReference);
            }
            if *producer_index as usize >= produced_count {
                return Err(BatchBuildError::InvalidTransientReference);
            }
            produced_type(*producer_index)
                .map(Some)
                .ok_or(BatchBuildError::InvalidTransientReference)
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
        return Err(BatchBuildError::UnsupportedBinaryOutputType);
    }
    Ok(())
}

pub(crate) fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8) {
        Ok(())
    } else {
        Err(BatchBuildError::UnsupportedFheType)
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
        Err(BatchBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_supported_rand_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8) {
        Ok(())
    } else {
        Err(BatchBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_subjects(subjects: &[Pubkey]) -> Result<()> {
    if subjects.is_empty() || subjects.len() > zama_solana_acl::MAX_ENCRYPTED_VALUE_SUBJECTS {
        return Err(BatchBuildError::InvalidSubjects);
    }
    for (index, subject) in subjects.iter().enumerate() {
        if *subject == Pubkey::default() {
            return Err(BatchBuildError::InvalidSubjects);
        }
        if subjects[..index].contains(subject) {
            return Err(BatchBuildError::InvalidSubjects);
        }
    }
    Ok(())
}

pub(crate) fn validate_encrypted_value_id(key: &EncryptedValueId) -> Result<()> {
    if key.domain == Pubkey::default() || key.account == Pubkey::default() {
        return Err(BatchBuildError::InvalidEncryptedValueId);
    }
    Ok(())
}

pub(crate) fn validate_app_authority(authority: BatchAppAuthority) -> Result<()> {
    if authority.pubkey() == Pubkey::default() {
        return Err(BatchBuildError::InvalidAppAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
