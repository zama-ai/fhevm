//! Batcher-local FHE helpers.
//!
//! The batcher drives its own ZamaHost evals (join re-materialization, the
//! quit reset, and the claim MulDiv) with one identity: the per-batch
//! authority PDA is simultaneously the eval's `compute_subject` (it reads the
//! deposit encrypted value accounts it is a subject of) and its `encrypted_value_account_authority` (it
//! authorizes the batcher-owned persistent outputs), both signed through a single
//! `invoke_signed`.

use anchor_lang::prelude::*;
use zama_host::EncryptedValue;

use crate::constants::BATCH_AUTHORITY_SEED;
use crate::errors::BatcherError;

/// Decodes a canonical, host-owned `EncryptedValue` account.
pub(crate) fn read_encrypted_value(info: &AccountInfo) -> Result<EncryptedValue> {
    require_keys_eq!(
        *info.owner,
        zama_host::ID,
        BatcherError::EncryptedValueInvalid
    );
    let data = info.try_borrow_data()?;
    let mut slice: &[u8] = &data;
    EncryptedValue::try_deserialize(&mut slice)
        .map_err(|_| BatcherError::EncryptedValueInvalid.into())
}

/// A persistent eval output bound to the exact `EncryptedValue` encrypted value account it may
/// create or update, mirroring the confidential-token pattern: create when
/// the PDA does not exist yet, update (pinning the stored previous handle
/// and subjects) when it does.
pub(crate) struct PersistentBinding<'info> {
    account: AccountInfo<'info>,
    output: Box<zama_fhe::PersistentOutput>,
    previous_handle: Option<[u8; 32]>,
}

impl<'info> PersistentBinding<'info> {
    pub(crate) fn bind(
        account: AccountInfo<'info>,
        key: zama_fhe::EncryptedValueId,
        subjects: Vec<Pubkey>,
    ) -> Result<Self> {
        require_keys_eq!(
            account.key(),
            key.address(),
            BatcherError::DerivedAccountMismatch
        );
        let (output, previous_handle) = if *account.owner == System::id() {
            require!(
                account.data_is_empty() && !account.executable,
                BatcherError::EncryptedValueInvalid
            );
            (zama_fhe::PersistentOutput::create(key, subjects), None)
        } else {
            let value = read_encrypted_value(&account)?;
            (
                zama_fhe::PersistentOutput::update(key, subjects, &value),
                Some(value.current_handle),
            )
        };
        output.binding().map_err(invalid_execution)?;
        Ok(Self {
            account,
            output: Box::new(output),
            previous_handle,
        })
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::persistent((*self.output).clone())
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }

    /// The encrypted value account's handle before this eval, when the encrypted value account already existed.
    pub(crate) fn previous_handle(&self) -> Option<[u8; 32]> {
        self.previous_handle
    }

    /// Reads the handle the host bound into the encrypted value account. Call only after the
    /// eval CPI carrying this output has executed.
    pub(crate) fn handle_after_execute(&self) -> Result<[u8; 32]> {
        Ok(read_encrypted_value(&self.account)?.current_handle)
    }
}

/// Fixed ZamaHost CPI accounts for an eval signed by the batch authority PDA.
pub(crate) struct BatchAuthorityExecute<'a, 'info> {
    pub(crate) batch: Pubkey,
    pub(crate) authority_bump: u8,
    pub(crate) batch_authority: AccountInfo<'info>,
    pub(crate) payer: AccountInfo<'info>,
    pub(crate) host_config: AccountInfo<'info>,
    pub(crate) zama_event_authority: AccountInfo<'info>,
    pub(crate) zama_program: AccountInfo<'info>,
    pub(crate) system_program: AccountInfo<'info>,
    pub(crate) deny_subject_records: &'a [AccountInfo<'info>],
}

/// Builds and invokes one `fhe_execute` execution with the batch authority as both
/// compute subject and app account authority.
pub(crate) fn execute_as_batch_authority<'info>(
    eval: BatchAuthorityExecute<'_, 'info>,
    dynamic_accounts: Vec<AccountInfo<'info>>,
    build: impl for<'brand> FnOnce(&mut zama_fhe::FheExecutionBuilder<'brand>) -> zama_fhe::Result<()>,
) -> Result<()> {
    let bump = [eval.authority_bump];
    let authority_seeds: &[&[u8]] = &[BATCH_AUTHORITY_SEED, eval.batch.as_ref(), &bump];
    let execution = zama_fhe::FheExecution::build(
        zama_fhe::ExecutionAppAuthority::new(eval.batch_authority.key()),
        build,
    )
    .map_err(invalid_execution)?;
    // Every persistent output of a batcher eval is authorized by the batch authority itself, so it
    // is the only output authority witness the execution can require.
    let resolved_accounts = execution
        .resolve_accounts(dynamic_accounts, [eval.batch_authority.clone()])
        .map_err(|error| {
            msg!("invalid batcher fhe_execute accounts: {:?}", error);
            error!(BatcherError::InvalidFheExecution)
        })?;
    // Host/CPI errors propagate unchanged so callers and tests keep seeing the host's error code.
    execution.invoke(
        zama_fhe::ExecutionCpiAccounts {
            payer: eval.payer,
            compute_subject: eval.batch_authority.clone(),
            encrypted_value_account_authority: eval.batch_authority,
            host_config: eval.host_config,
            deny_subject_records: eval.deny_subject_records,
            system_program: eval.system_program,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: eval.zama_event_authority,
            program: eval.zama_program,
        },
        &resolved_accounts,
        &[authority_seeds],
    )
}

pub(crate) fn invalid_execution(
    error: zama_fhe::FheExecutionBuildError,
) -> anchor_lang::error::Error {
    msg!("invalid FHE execution: {:?}", error);
    error!(BatcherError::InvalidFheExecution)
}

/// Builds a euint64 persistent operand from an encrypted value account's own canonical fields, so
/// the operand slot always matches the account the host re-validates.
pub(crate) fn uint64_operand(value: &EncryptedValue) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::persistent(
        value.current_handle,
        zama_fhe::EncryptedValueId::new(
            zama_fhe::Domain::new(value.domain),
            value.encrypted_value_account_authority,
            zama_fhe::EncryptedValueLabel::new(value.label),
        ),
    )
    .map_err(invalid_execution)
}
