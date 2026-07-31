//! The validated, lowered eval request handed to the CPI helpers.

use anchor_lang::prelude::Pubkey;

use zama_host::{FheExecuteArgs, FheExecuteOutput, FheExecuteStep};

#[cfg(feature = "cpi")]
use crate::accounts::{resolve_eval_accounts, EvalAccountResolutionError, ResolvedEvalAccounts};
use crate::accounts::{
    BatchAccountMeta, BatchAccountPurpose, BatchAppAuthority, EvalAccountRequirement,
    EvalOutputAuthorityRequirement,
};
use crate::builder::BatchBuilder;
use crate::Result;

#[cfg(feature = "cpi")]
use anchor_lang::prelude::AccountInfo;

/// Opaque lowered eval request produced by [`BatchBuilder::finish`] or
/// [`Batch::build`].
///
/// App code passes this to [`invoke_batch_signed_resolved`] instead of editing
/// raw host args or dynamic account roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub(crate) app_authority: BatchAppAuthority,
    pub(crate) args: FheExecuteArgs,
    /// Exact dynamic `remaining_accounts` order referenced by the `u16` indices
    /// inside `args`. Keep this coupled to `args`; `finish` validates every
    /// index before constructing the plan.
    pub(crate) remaining_accounts: Vec<BatchAccountMeta>,
}

impl Batch {
    /// Builds and validates an eval plan through a closure.
    ///
    /// This keeps transient values scoped to one builder while removing the
    /// need for app code to call [`BatchBuilder::finish`] explicitly.
    pub fn build<T, F>(app_authority: BatchAppAuthority, build: F) -> Result<Self>
    where
        F: FnOnce(&mut BatchBuilder) -> Result<T>,
    {
        let mut builder = BatchBuilder::new(app_authority);
        build(&mut builder)?;
        builder.finish()
    }

    pub fn app_authority(&self) -> BatchAppAuthority {
        self.app_authority
    }

    pub fn dynamic_account_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = EvalAccountRequirement> + '_ {
        self.remaining_accounts
            .iter()
            .map(EvalAccountRequirement::from)
    }

    #[cfg(feature = "cpi")]
    /// Resolves unordered app-supplied accounts into the exact host
    /// `remaining_accounts` order for this plan.
    ///
    /// `dynamic_accounts` must contain only non-authority plan accounts such as
    /// persistent input ACLs, permission records, transient sessions, and writable
    /// persistent output ACL records. `output_authorities` must contain signer
    /// witnesses for persistent outputs whose app account is not the fixed CPI
    /// `account_authority`.
    pub fn resolve_accounts<'info>(
        &self,
        dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    ) -> std::result::Result<ResolvedEvalAccounts<'info>, EvalAccountResolutionError> {
        resolve_eval_accounts(self, dynamic_accounts, output_authorities)
    }

    pub fn output_authority_requirements(
        &self,
    ) -> impl Iterator<Item = EvalOutputAuthorityRequirement> + '_ {
        std::iter::once(EvalOutputAuthorityRequirement {
            pubkey: self.app_authority.pubkey(),
            cpi_account_authority: true,
        })
        .chain(self.additional_output_authorities().map(|pubkey| {
            EvalOutputAuthorityRequirement {
                pubkey,
                cpi_account_authority: false,
            }
        }))
    }

    pub fn output_authorities(&self) -> impl Iterator<Item = Pubkey> + '_ {
        self.output_authority_requirements()
            .map(|requirement| requirement.pubkey())
    }

    pub fn additional_output_authorities(&self) -> impl Iterator<Item = Pubkey> + '_ {
        self.remaining_accounts
            .iter()
            .filter(|account| {
                account
                    .purposes
                    .contains(&BatchAccountPurpose::PersistentOutputAuthority)
            })
            .map(|account| account.pubkey)
    }

    /// Subjects this plan newly grants through persistent outputs: every output
    /// subject on a create, and `output_subjects \ previous_subjects` on a
    /// update that replaces its audience. The host deny-list-checks each of
    /// these exactly like `allow_subjects`, so an app forwarding deny-record
    /// witnesses must cover them alongside the output authorities.
    pub fn newly_granted_subjects(&self) -> Vec<Pubkey> {
        let mut added = Vec::new();
        for step in &self.args.steps {
            let FheExecuteOutput::AllowedPersistent {
                output_subject_indexes,
                previous_subjects,
                ..
            } = fhe_execute_step_output(step)
            else {
                continue;
            };
            for index in output_subject_indexes {
                // `finish` validated every dictionary index, so resolution cannot fail here.
                let Ok(subject) = self.args.dictionary_key(*index) else {
                    continue;
                };
                let already_stored = previous_subjects
                    .as_ref()
                    .is_some_and(|previous| previous.contains(&subject));
                if !already_stored && !added.contains(&subject) {
                    added.push(subject);
                }
            }
        }
        added
    }
}

/// The output policy of an eval step, independent of step kind.
pub(crate) fn fhe_execute_step_output(step: &FheExecuteStep) -> &FheExecuteOutput {
    match step {
        FheExecuteStep::Binary { output, .. }
        | FheExecuteStep::Ternary { output, .. }
        | FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::Unary { output, .. }
        | FheExecuteStep::RandBounded { output, .. }
        | FheExecuteStep::Sum { output, .. }
        | FheExecuteStep::IsIn { output, .. }
        | FheExecuteStep::MulDiv { output, .. } => output,
    }
}
