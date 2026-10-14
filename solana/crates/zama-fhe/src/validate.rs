//! The FHE type a handle carries, and the check that the execution authority can sign. The host
//! checks every execution rule itself.

use anchor_lang::prelude::Pubkey;

use crate::{FheExecutionError, Result};

pub(crate) fn validate_authority(authority: Pubkey) -> Result<()> {
    if authority == Pubkey::default() {
        return Err(FheExecutionError::InvalidExecutionAuthority);
    }
    Ok(())
}

pub(crate) fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}
