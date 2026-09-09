//! Batcher FHE execution under each JoinRecord authority.

use anchor_lang::prelude::*;
use crate::errors::BatcherError;

pub(crate) fn invalid_execution(
    error: zama_fhe::FheExecutionBuildError,
) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
    error!(BatcherError::InvalidFheExecution)
}

pub(crate) fn read_state(info: &AccountInfo) -> Result<zama_host::EncryptedState> {
    require_keys_eq!(*info.owner, zama_host::ID, BatcherError::EncryptedValueInvalid);
    let state = zama_host::EncryptedState::try_deserialize(&mut &info.try_borrow_data()?[..])?;
    require_keys_eq!(info.key(), state.canonical_address().0, BatcherError::DerivedAccountMismatch);
    Ok(state)
}

pub(crate) struct JoinExecute<'a, 'info> {
    pub batch: Pubkey,
    pub user: Pubkey,
    pub bump: u8,
    pub record: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub host_config: AccountInfo<'info>,
    pub event_authority: AccountInfo<'info>,
    pub program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub deny_records: &'a [AccountInfo<'info>],
}

impl<'info> JoinExecute<'_, 'info> {
    pub(crate) fn invoke(
        self,
        execution: zama_fhe::ReturningFheExecution<zama_fhe::Uint<64>>,
        dynamic: Vec<AccountInfo<'info>>,
    ) -> Result<[u8; 32]> {
        let resolved = execution.execution().resolve_accounts(dynamic, [self.record.clone()]).map_err(|error| { msg!("invalid execution accounts: {:?}", error); error!(BatcherError::InvalidFheExecution) })?;
        let bump = [self.bump];
        let seeds: &[&[u8]] = &[crate::constants::JOIN_RECORD_SEED, self.batch.as_ref(), self.user.as_ref(), &bump];
        execution.invoke(zama_fhe::ExecutionCpiAccounts {
            payer: self.payer,
            encrypted_value_account_authority: self.record,
            host_config: self.host_config,
            deny_scope_records: self.deny_records.to_vec(),
            system_program: self.system_program,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            rand_nonce: None,
            event_authority: self.event_authority,
            program: self.program,
        }, &resolved, &[seeds])
    }
}
