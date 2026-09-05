//! CPI assembly: turns an `FheExecution` plus resolved accounts into the host call.

#[cfg(feature = "cpi")]
use anchor_lang::{
    prelude::AccountInfo,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program::invoke_signed,
    },
    Key, ToAccountInfos, ToAccountMetas,
};

#[cfg(feature = "cpi")]
use anchor_lang::prelude::Pubkey;

#[cfg(feature = "cpi")]
use crate::accounts::ResolvedExecutionAccounts;
#[cfg(feature = "cpi")]
use crate::execution::FheExecution;

/// The fixed accounts of one `fhe_execute` CPI. The per-execution ones — the application's deny
/// record and the host's rand nonce — are derived from the built execution: `FheExecution::app`
/// names the `(program, scope)` the deny record is keyed on, `FheExecution::has_rand_step` says
/// whether the nonce is needed.
#[cfg(feature = "cpi")]
pub struct ExecutionCpiAccounts<'info> {
    pub payer: AccountInfo<'info>,
    pub encrypted_value_account_authority: AccountInfo<'info>,
    pub host_config: AccountInfo<'info>,
    /// The application's `DenyScopeRecord`, required while the host's deny list is enabled and
    /// refused while it is disabled.
    pub deny_scope_record: Option<AccountInfo<'info>>,
    pub system_program: AccountInfo<'info>,
    /// Per-application HCU block meter (mut), keyed on the execution's `(program, scope)`.
    /// Untrusted applications in the metering band supply it; trusted applications and the
    /// unrestricted default pass `None`.
    pub hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness (read-only), keyed on the execution's `(program, scope)`. `Some` + valid
    /// ⇒ bypass; `None` ⇒ untrusted (metered).
    pub hcu_trusted_app_record: Option<AccountInfo<'info>>,
    /// The host's rand nonce (mut), required exactly when the execution has a rand step.
    pub rand_nonce: Option<AccountInfo<'info>>,
    pub event_authority: AccountInfo<'info>,
    pub program: AccountInfo<'info>,
}

#[cfg(feature = "cpi")]
pub(crate) trait ExecutionAccountResolver<'info> {
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
pub(crate) fn invoke_execution_signed_resolved<'info>(
    execution: &mut FheExecution,
    accounts: ExecutionCpiAccounts<'info>,
    resolved_accounts: &ResolvedExecutionAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()> {
    invoke_execution_signed_with_resolver(execution, accounts, resolved_accounts, signer_seeds)
}

#[cfg(feature = "cpi")]
fn invoke_execution_signed_with_resolver<'info, R>(
    execution: &mut FheExecution,
    accounts: ExecutionCpiAccounts<'info>,
    resolver: &R,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()>
where
    R: ExecutionAccountResolver<'info> + ?Sized,
{
    if accounts.encrypted_value_account_authority.key()
        != execution.encrypted_value_account_authority.pubkey()
    {
        return Err(anchor_lang::error::ErrorCode::ConstraintAddress.into());
    }
    let deny_scope_record = accounts.deny_scope_record;
    let fixed_accounts = zama_host::cpi::accounts::FheExecute {
        payer: accounts.payer,
        encrypted_value_account_authority: accounts.encrypted_value_account_authority,
        host_config: accounts.host_config,
        system_program: accounts.system_program,
        hcu_block_meter: accounts.hcu_block_meter,
        hcu_trusted_app_record: accounts.hcu_trusted_app_record,
        rand_nonce: accounts.rand_nonce,
        event_authority: accounts.event_authority,
        program: accounts.program,
    };
    let (account_metas, account_infos) = fhe_execute_account_tables(
        &fixed_accounts,
        execution,
        resolver,
        deny_scope_record.as_ref(),
    )?;

    // The execution self-describes its `remaining_accounts` length (DD-033). The deny-record
    // witness is appended per transaction, so the final count is only known here — stamped in
    // place: `invoke` consumed the execution, so nothing can observe the mutation.
    execution.args.account_count =
        u8::try_from(execution.remaining_accounts.len() + usize::from(deny_scope_record.is_some()))
            .map_err(|_| anchor_lang::error::ErrorCode::AccountNotEnoughKeys)?;

    let instruction = Instruction {
        program_id: fixed_accounts.program.key(),
        accounts: account_metas,
        data: crate::execution::fhe_execute_instruction_data(&execution.args),
    };

    invoke_signed(&instruction, &account_infos, signer_seeds)?;
    Ok(())
}

/// Assembles the `fhe_execute` CPI account tables exactly as
/// [`crate::heap_tally::invoke_table_heap_bytes`] charges them at `build()`: Anchor's generated
/// accessors grow the fixed accounts from empty, then one exact reservation sizes the dynamic
/// tail — no doubling on the never-freeing bump heap past the fixed accounts. The heap-budget
/// invoke measurement runs this function under a counting allocator, so the model and this
/// assembly cannot drift apart silently.
#[cfg(feature = "cpi")]
pub(crate) fn fhe_execute_account_tables<'info, R>(
    fixed_accounts: &zama_host::cpi::accounts::FheExecute<'info>,
    execution: &FheExecution,
    resolver: &R,
    deny_scope_record: Option<&AccountInfo<'info>>,
) -> anchor_lang::prelude::Result<(Vec<AccountMeta>, Vec<AccountInfo<'info>>)>
where
    R: ExecutionAccountResolver<'info> + ?Sized,
{
    let mut account_metas = fixed_accounts.to_account_metas(None);
    let mut account_infos = fixed_accounts.to_account_infos();
    let dynamic_tail =
        execution.remaining_accounts.len() + usize::from(deny_scope_record.is_some());
    account_metas.reserve_exact(dynamic_tail);
    account_infos.reserve_exact(dynamic_tail);
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
    if let Some(record) = deny_scope_record {
        account_metas.push(AccountMeta::new_readonly(record.key(), false));
        account_infos.push(record.clone());
    }
    Ok((account_metas, account_infos))
}
