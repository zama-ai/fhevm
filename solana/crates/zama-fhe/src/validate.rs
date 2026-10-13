//! The builder's own input gates: the FHE types its typed API admits, the type an operand
//! carries, and an authority that can sign. The host checks every execution rule itself.

use anchor_lang::prelude::Pubkey;

use zama_host::{is_supported_fhe_type, is_supported_uint_fhe_type};

use crate::operand::{Operand, OperandKind};
use crate::{FheExecutionError, Result};

pub(crate) fn operand_fhe_type<F>(
    operand: &Operand,
    produced_count: usize,
    produced_type: &F,
) -> Result<Option<u8>>
where
    F: Fn(u8) -> Option<u8>,
{
    match &operand.0 {
        OperandKind::StoreSlot { handle, .. } | OperandKind::Granted { handle, .. } => {
            Ok(Some(handle_fhe_type(*handle)))
        }
        OperandKind::Transient { producer_index } => {
            if *producer_index as usize >= produced_count {
                return Err(FheExecutionError::InvalidTransientReference);
            }
            produced_type(*producer_index)
                .map(Some)
                .ok_or(FheExecutionError::InvalidTransientReference)
        }
        OperandKind::VerifiedInput { input_handle, .. } => Ok(Some(handle_fhe_type(*input_handle))),
        OperandKind::Scalar(_) => Ok(None),
    }
}

pub(crate) fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if is_supported_fhe_type(fhe_type) {
        Ok(())
    } else {
        Err(FheExecutionError::UnsupportedFheType)
    }
}

pub(crate) fn validate_uint_fhe_type(fhe_type: u8) -> Result<()> {
    if is_supported_uint_fhe_type(fhe_type) {
        Ok(())
    } else {
        Err(FheExecutionError::UnsupportedFheType)
    }
}

pub(crate) fn validate_authority(authority: Pubkey) -> Result<()> {
    if authority == Pubkey::default() {
        return Err(FheExecutionError::InvalidExecutionAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
