//! CPI assembly: turns an `FheExecution` plus resolved accounts into the host call.

#[cfg(feature = "cpi")]
use anchor_lang::{
    prelude::AccountInfo,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program::invoke_signed,
    },
    InstructionData, Key, ToAccountInfos, ToAccountMetas,
};

#[cfg(feature = "cpi")]
use anchor_lang::prelude::Pubkey;

#[cfg(feature = "cpi")]
use crate::accounts::ResolvedExecutionAccounts;
#[cfg(feature = "cpi")]
use crate::execution::FheExecution;

#[cfg(feature = "cpi")]
pub struct ExecutionCpiAccounts<'a, 'info> {
    pub payer: AccountInfo<'info>,
    pub compute_subject: AccountInfo<'info>,
    pub account_authority: AccountInfo<'info>,
    pub host_config: AccountInfo<'info>,
    pub deny_subject_records: &'a [AccountInfo<'info>],
    pub system_program: AccountInfo<'info>,
    /// Per-`compute_subject` HCU block meter (mut). The host keys the meter on `compute_subject`, so
    /// untrusted subjects in the metering band supply it; trusted subjects and the unrestricted
    /// default pass `None`.
    pub hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness (read-only), keyed on `compute_subject`. `Some` + valid ⇒ bypass; `None` ⇒
    /// untrusted (metered).
    pub hcu_trusted_app_record: Option<AccountInfo<'info>>,
    pub event_authority: AccountInfo<'info>,
    pub program: AccountInfo<'info>,
}

#[cfg(feature = "cpi")]
trait ExecutionAccountResolver<'info> {
    fn resolve_execution_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>>;
}

#[cfg(feature = "cpi")]
impl<'info> ExecutionAccountResolver<'info> for ResolvedExecutionAccounts<'info> {
    fn resolve_execution_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>> {
        self.resolve(pubkey)
    }
}

/// Invokes `zama-host::fhe_execute` with accounts pre-resolved from a [`FheExecution`].
/// App-facing surface: [`FheExecution::invoke`].
#[cfg(feature = "cpi")]
pub(crate) fn invoke_execution_signed_resolved<'a, 'info>(
    execution: &FheExecution,
    accounts: ExecutionCpiAccounts<'a, 'info>,
    resolved_accounts: &ResolvedExecutionAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()> {
    invoke_execution_signed_with_resolver(execution, accounts, resolved_accounts, signer_seeds)
}

#[cfg(feature = "cpi")]
fn invoke_execution_signed_with_resolver<'a, 'info, R>(
    execution: &FheExecution,
    accounts: ExecutionCpiAccounts<'a, 'info>,
    resolver: &R,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()>
where
    R: ExecutionAccountResolver<'info> + ?Sized,
{
    if accounts.account_authority.key() != execution.app_authority.pubkey() {
        return Err(anchor_lang::error::ErrorCode::ConstraintAddress.into());
    }
    let deny_subject_records = accounts.deny_subject_records;
    let fixed_accounts = zama_host::cpi::accounts::FheExecute {
        payer: accounts.payer,
        compute_subject: accounts.compute_subject,
        account_authority: accounts.account_authority,
        host_config: accounts.host_config,
        system_program: accounts.system_program,
        hcu_block_meter: accounts.hcu_block_meter,
        hcu_trusted_app_record: accounts.hcu_trusted_app_record,
        event_authority: accounts.event_authority,
        program: accounts.program,
    };
    let mut account_metas = fixed_accounts.to_account_metas(None);
    let mut account_infos = fixed_accounts.to_account_infos();
    for required in &execution.remaining_accounts {
        let account = resolver
            .resolve_execution_account(required.pubkey)
            .ok_or(anchor_lang::error::ErrorCode::AccountNotEnoughKeys)?;
        let meta = if required.is_writable {
            AccountMeta::new(required.pubkey, required.is_signer)
        } else {
            AccountMeta::new_readonly(required.pubkey, required.is_signer)
        };
        account_metas.push(meta);
        account_infos.push(account);
    }
    for record in deny_subject_records.iter().cloned() {
        account_metas.push(AccountMeta::new_readonly(record.key(), false));
        account_infos.push(record);
    }

    // The execution self-describes its `remaining_accounts` length (DD-033). Deny-record
    // witnesses are appended per transaction, so the final count is only known here.
    let mut args = execution.args.clone();
    args.account_count =
        u8::try_from(execution.remaining_accounts.len() + deny_subject_records.len())
            .map_err(|_| anchor_lang::error::ErrorCode::AccountNotEnoughKeys)?;

    let instruction = Instruction {
        program_id: fixed_accounts.program.key(),
        accounts: account_metas,
        data: zama_host::instruction::FheExecute { args }.data(),
    };

    invoke_signed(&instruction, &account_infos, signer_seeds)?;
    Ok(())
}
