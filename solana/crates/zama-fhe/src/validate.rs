//! Local mirrors of the host preflight rules so malformed executions fail before the CPI.

use anchor_lang::prelude::Pubkey;

use zama_host::{
    binary_output_type_ok, is_supported_fhe_type, is_supported_uint_fhe_type,
    scalar_is_zero_for_type, unary_output_type_ok, FheBinaryOpCode, FheExecuteEffect,
    FheExecuteOperand, FheExecuteStep, FheUnaryOpCode,
};

use crate::accounts::ExecutionAccountMeta;
use crate::operand::{Operand, OperandKind};
use crate::{FheExecutionBuildError, Result};

pub(crate) fn validate_lowered_execution(
    steps: &[FheExecuteStep],
    effects: &[FheExecuteEffect],
    remaining_accounts: &[ExecutionAccountMeta],
    dictionary: &[[u8; 32]],
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    debug_assert_eq!(used_accounts.len(), remaining_accounts.len());
    debug_assert_eq!(used_dictionary.len(), dictionary.len());
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
        // A State authority is found by key, never by wire index: it counts as used by the
        // operand or output whose authority it is.
        if account.requires_state_authority() {
            used_accounts[index] = true;
        }
    }

    for (step_index, step) in steps.iter().enumerate() {
        validate_lowered_step(step, step_index, used_accounts, used_dictionary)?;
    }
    for effect in effects {
        if usize::from(effect.result.step_index) >= steps.len() || effect.result.output_index != 0 {
            return Err(FheExecutionBuildError::InvalidTransientReference);
        }
        validate_lowered_effect(effect, used_accounts, used_dictionary)?;
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
        FheExecuteStep::Binary { lhs, rhs, .. } => {
            validate_lowered_encrypted_operand(lhs, step_index, used_accounts, used_dictionary)?;
            validate_lowered_rhs_operand(rhs, step_index, used_accounts, used_dictionary)?;
        }
        FheExecuteStep::Ternary {
            control,
            if_true,
            if_false,
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
        }
        FheExecuteStep::TrivialEncrypt { .. }
        | FheExecuteStep::Rand { .. }
        | FheExecuteStep::RandBounded { .. } => {}
        FheExecuteStep::Unary { operand, .. } => {
            validate_lowered_encrypted_operand(
                operand,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
        }
        FheExecuteStep::Sum { operands, .. } => {
            for operand in operands {
                validate_lowered_encrypted_operand(
                    operand,
                    step_index,
                    used_accounts,
                    used_dictionary,
                )?;
            }
        }
        FheExecuteStep::IsIn { value, set, .. } => {
            validate_lowered_encrypted_operand(value, step_index, used_accounts, used_dictionary)?;
            for operand in set {
                validate_lowered_encrypted_operand(
                    operand,
                    step_index,
                    used_accounts,
                    used_dictionary,
                )?;
            }
        }
        FheExecuteStep::MulDiv {
            factor1, factor2, ..
        } => {
            validate_lowered_encrypted_operand(
                factor1,
                step_index,
                used_accounts,
                used_dictionary,
            )?;
            validate_lowered_rhs_operand(factor2, step_index, used_accounts, used_dictionary)?;
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
        FheExecuteOperand::StateSlot {
            handle_index,
            state_index,
            key_index,
        } => {
            mark_lowered_account(used_accounts, *state_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *handle_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *key_index)?;
        }
        FheExecuteOperand::TransientResult {
            handle_index,
            consumer_state_index,
        } => {
            mark_lowered_account(used_accounts, *consumer_state_index)?;
            mark_lowered_dictionary_entry(used_dictionary, *handle_index)?;
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

fn validate_lowered_effect(
    effect: &FheExecuteEffect,
    used_accounts: &mut [bool],
    used_dictionary: &mut [bool],
) -> Result<()> {
    let FheExecuteEffect {
        state_index,
        slot,
        allow_indexes,
        grants,
        ..
    } = effect;
    mark_lowered_account(used_accounts, *state_index)?;
    if let Some(slot) = slot {
        mark_lowered_dictionary_entry(used_dictionary, slot.key_index)?;
        if let Some(index) = slot.previous_handle_index {
            mark_lowered_dictionary_entry(used_dictionary, index)?;
        }
    }
    for index in allow_indexes {
        mark_lowered_dictionary_entry(used_dictionary, *index)?;
    }
    for grant in grants {
        mark_lowered_account(used_accounts, grant.consumer_state_index)?;
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
        // Eq/Ne accept Bool and Uint8..Uint128; ordered comparisons Uint8..Uint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            if !is_supported_fhe_type(lhs_type) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            if !is_supported_uint_fhe_type(lhs_type) {
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
                OperandKind::StateSlot { .. }
                | OperandKind::Granted { .. }
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
    if !unary_output_type_ok(op, output_fhe_type) {
        return Err(FheExecutionBuildError::UnsupportedFheType);
    }
    let operand_type = operand_fhe_type(operand, produced_count, &produced_type)?
        .ok_or(FheExecutionBuildError::ScalarEncryptedOperand)?;
    validate_supported_fhe_type(operand_type)?;
    match op {
        FheUnaryOpCode::Neg => {
            if !is_supported_uint_fhe_type(operand_type) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Not => {
            if !is_supported_fhe_type(operand_type) {
                return Err(FheExecutionBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(FheExecutionBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Cast => {
            // Cast input set: Bool | Uint8..Uint128 (no eaddress/Uint160). Solana host max is euint128.
            if !is_supported_fhe_type(operand_type) {
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
        OperandKind::StateSlot { handle, .. } | OperandKind::Granted { handle, .. } => {
            Ok(Some(handle_fhe_type(*handle)))
        }
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
    if !binary_output_type_ok(op, output_fhe_type) {
        return Err(FheExecutionBuildError::UnsupportedBinaryOutputType);
    }
    Ok(())
}

pub(crate) fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if is_supported_fhe_type(fhe_type) {
        Ok(())
    } else {
        Err(FheExecutionBuildError::UnsupportedFheType)
    }
}

pub(crate) fn validate_uint_fhe_type(fhe_type: u8) -> Result<()> {
    if is_supported_uint_fhe_type(fhe_type) {
        Ok(())
    } else {
        Err(FheExecutionBuildError::UnsupportedFheType)
    }
}

/// Mirrors the host's `InvalidAllowKey`: no zero key, no duplicate. An empty list is legal.
pub(crate) fn validate_allow_keys(keys: &[Pubkey]) -> Result<()> {
    for (index, key) in keys.iter().enumerate() {
        if *key == Pubkey::default() || keys[..index].contains(key) {
            return Err(FheExecutionBuildError::InvalidAllowKey);
        }
    }
    Ok(())
}

pub(crate) fn validate_authority(authority: Pubkey) -> Result<()> {
    if authority == Pubkey::default() {
        return Err(FheExecutionBuildError::InvalidExecutionAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
