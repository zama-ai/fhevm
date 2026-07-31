//! CPI assembly: turns an `EvalPlan` plus resolved accounts into the host call.

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
use crate::accounts::{EvalAccountResolutionError, EvalAppAuthority, ResolvedEvalAccounts};
#[cfg(feature = "cpi")]
use crate::builder::EvalBuilder;
#[cfg(feature = "cpi")]
use crate::plan::EvalPlan;
#[cfg(feature = "cpi")]
use crate::{EvalBuildError, Result};

#[cfg(feature = "cpi")]
pub struct EvalCpiAccounts<'a, 'info> {
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
trait EvalAccountResolver<'info> {
    fn resolve_eval_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>>;
}

#[cfg(feature = "cpi")]
impl<'info> EvalAccountResolver<'info> for ResolvedEvalAccounts<'info> {
    fn resolve_eval_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>> {
        self.resolve(pubkey)
    }
}

/// Failure returned by the closure-based CPI eval helper.
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub enum EvalInvokeError {
    /// The closure produced an invalid eval frame.
    Build(EvalBuildError),
    /// The supplied dynamic accounts or output authority witnesses do not
    /// satisfy the built plan.
    AccountResolution(EvalAccountResolutionError),
    /// The host CPI returned an Anchor error.
    Cpi(anchor_lang::error::Error),
}

#[cfg(feature = "cpi")]
impl From<EvalBuildError> for EvalInvokeError {
    fn from(error: EvalBuildError) -> Self {
        Self::Build(error)
    }
}

#[cfg(feature = "cpi")]
impl From<EvalAccountResolutionError> for EvalInvokeError {
    fn from(error: EvalAccountResolutionError) -> Self {
        Self::AccountResolution(error)
    }
}

#[cfg(feature = "cpi")]
impl From<anchor_lang::error::Error> for EvalInvokeError {
    fn from(error: anchor_lang::error::Error) -> Self {
        Self::Cpi(error)
    }
}

/// Builds an eval plan with a closure, resolves its dynamic accounts, and
/// invokes `zama-host::fhe_eval`.
///
/// `dynamic_accounts` and additional `output_authorities` may be in any order.
/// The fixed CPI `account_authority` is included automatically. The SDK
/// validates the supplied accounts against the plan produced by the closure
/// before constructing the ordered host account list used by
/// [`invoke_eval_signed_resolved`].
#[cfg(feature = "cpi")]
pub fn invoke_eval_signed_with_builder<'a, 'info, T, F>(
    app_authority: EvalAppAuthority,
    accounts: EvalCpiAccounts<'a, 'info>,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    signer_seeds: &[&[&[u8]]],
    build: F,
) -> std::result::Result<(), EvalInvokeError>
where
    F: FnOnce(&mut EvalBuilder) -> Result<T>,
{
    let plan = EvalPlan::build(app_authority, build)?;
    let mut output_authorities = output_authorities.into_iter().collect::<Vec<_>>();
    output_authorities.insert(0, accounts.account_authority.clone());
    let resolved_accounts = plan.resolve_accounts(dynamic_accounts, output_authorities)?;
    invoke_eval_signed_resolved(&plan, accounts, &resolved_accounts, signer_seeds)?;
    Ok(())
}

/// Invokes `zama-host::fhe_eval` with accounts pre-resolved from an [`EvalPlan`].
#[cfg(feature = "cpi")]
pub fn invoke_eval_signed_resolved<'a, 'info>(
    plan: &EvalPlan,
    accounts: EvalCpiAccounts<'a, 'info>,
    resolved_accounts: &ResolvedEvalAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()> {
    invoke_eval_signed_with_resolver(plan, accounts, resolved_accounts, signer_seeds)
}

#[cfg(feature = "cpi")]
fn invoke_eval_signed_with_resolver<'a, 'info, R>(
    plan: &EvalPlan,
    accounts: EvalCpiAccounts<'a, 'info>,
    resolver: &R,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()>
where
    R: EvalAccountResolver<'info> + ?Sized,
{
    if accounts.account_authority.key() != plan.app_authority.pubkey() {
        return Err(anchor_lang::error::ErrorCode::ConstraintAddress.into());
    }
    let deny_subject_records = accounts.deny_subject_records;
    let fixed_accounts = zama_host::cpi::accounts::FheEval {
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
    for required in &plan.remaining_accounts {
        let account = resolver
            .resolve_eval_account(required.pubkey)
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

    // The frame self-describes its `remaining_accounts` length (DD-033). Deny-record
    // witnesses are appended per transaction, so the final count is only known here.
    let mut args = plan.args.clone();
    args.account_count = u8::try_from(plan.remaining_accounts.len() + deny_subject_records.len())
        .map_err(|_| anchor_lang::error::ErrorCode::AccountNotEnoughKeys)?;

    let instruction = Instruction {
        program_id: fixed_accounts.program.key(),
        accounts: account_metas,
        data: zama_host::instruction::FheEval { args }.data(),
    };

    invoke_signed(&instruction, &account_infos, signer_seeds)?;
    Ok(())
}
