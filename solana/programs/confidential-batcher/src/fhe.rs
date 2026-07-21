//! Batcher-local FHE helpers.
//!
//! The batcher drives its own ZamaHost evals (join re-materialization, the
//! quit reset, and the claim MulDiv) with one identity: the per-batch
//! authority PDA is simultaneously the eval's `compute_subject` (it reads the
//! deposit lineages it is a subject of) and its `app_account_authority` (it
//! authorizes the batcher-owned durable outputs), both signed through a single
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

/// A durable eval output bound to the exact `EncryptedValue` lineage it may
/// create or supersede, mirroring the confidential-token pattern: create when
/// the PDA does not exist yet, supersede (pinning the stored previous handle
/// and subjects) when it does.
pub(crate) struct DurableBinding<'info> {
    account: AccountInfo<'info>,
    output: Box<zama_fhe::DurableOutput>,
    previous_handle: Option<[u8; 32]>,
}

impl<'info> DurableBinding<'info> {
    pub(crate) fn bind(
        account: AccountInfo<'info>,
        slot: zama_fhe::DurableSlot,
        access: zama_fhe::AccessPolicy,
    ) -> Result<Self> {
        require_keys_eq!(
            account.key(),
            slot.address(),
            BatcherError::DerivedAccountMismatch
        );
        let (output, previous_handle) = if *account.owner == System::id() {
            require!(
                account.data_is_empty() && !account.executable,
                BatcherError::EncryptedValueInvalid
            );
            (zama_fhe::DurableOutput::create(slot, access), None)
        } else {
            let value = read_encrypted_value(&account)?;
            (
                zama_fhe::DurableOutput::supersede(
                    slot,
                    access,
                    value.current_handle,
                    value.subjects.clone(),
                ),
                Some(value.current_handle),
            )
        };
        output.birth().map_err(invalid_eval_plan)?;
        Ok(Self {
            account,
            output: Box::new(output),
            previous_handle,
        })
    }

    pub(crate) fn output(&self) -> zama_fhe::Output {
        zama_fhe::Output::durable_output((*self.output).clone())
    }

    pub(crate) fn account_info(&self) -> AccountInfo<'info> {
        self.account.clone()
    }

    /// The lineage's handle before this eval, when the lineage already existed.
    pub(crate) fn previous_handle(&self) -> Option<[u8; 32]> {
        self.previous_handle
    }

    /// Reads the handle the host bound into the lineage. Call only after the
    /// eval CPI carrying this output has executed.
    pub(crate) fn handle_after_eval(&self) -> Result<[u8; 32]> {
        Ok(read_encrypted_value(&self.account)?.current_handle)
    }
}

/// Fixed ZamaHost CPI accounts for an eval signed by the batch authority PDA.
pub(crate) struct BatchAuthorityEval<'a, 'info> {
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

/// Builds and invokes one `fhe_eval` frame with the batch authority as both
/// compute subject and app account authority.
pub(crate) fn eval_as_batch_authority<'info, T>(
    eval: BatchAuthorityEval<'_, 'info>,
    context_id: zama_fhe::EvalContextId,
    dynamic_accounts: Vec<AccountInfo<'info>>,
    build: impl FnOnce(&mut zama_fhe::EvalBuilder) -> zama_fhe::Result<T>,
) -> Result<()> {
    let bump = [eval.authority_bump];
    let authority_seeds: &[&[u8]] = &[BATCH_AUTHORITY_SEED, eval.batch.as_ref(), &bump];
    zama_fhe::invoke_eval_signed_with_builder(
        context_id,
        zama_fhe::EvalAppAuthority::new(eval.batch_authority.key()),
        zama_fhe::EvalCpiAccounts {
            payer: eval.payer,
            compute_subject: eval.batch_authority.clone(),
            app_account_authority: eval.batch_authority,
            host_config: eval.host_config,
            deny_subject_records: eval.deny_subject_records,
            system_program: eval.system_program,
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: eval.zama_event_authority,
            program: eval.zama_program,
        },
        dynamic_accounts,
        [],
        &[authority_seeds],
        build,
    )
    .map_err(|error| match error {
        // Keep host/CPI error codes visible to callers and tests.
        zama_fhe::EvalInvokeError::Cpi(error) => error,
        other => {
            msg!("invalid batcher FHE eval: {:?}", other);
            error!(BatcherError::InvalidFheEvalPlan)
        }
    })?;
    Ok(())
}

pub(crate) fn invalid_eval_plan(error: zama_fhe::EvalBuildError) -> anchor_lang::error::Error {
    msg!("invalid FHE eval plan: {:?}", error);
    error!(BatcherError::InvalidFheEvalPlan)
}

/// Builds a euint64 durable operand from a lineage's own canonical fields, so
/// the operand slot always matches the account the host re-validates.
pub(crate) fn uint64_operand(value: &EncryptedValue) -> Result<zama_fhe::Uint64Handle> {
    zama_fhe::Uint64Handle::durable(
        value.current_handle,
        zama_fhe::DurableSlot::new(
            value.acl_domain_key,
            value.app_account,
            zama_fhe::DurableLabel::new(value.encrypted_value_label),
        ),
    )
    .map_err(invalid_eval_plan)
}
