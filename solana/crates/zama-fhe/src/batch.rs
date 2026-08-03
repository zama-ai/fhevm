//! The validated, lowered batch request handed to the CPI helpers.

use anchor_lang::prelude::Pubkey;

use zama_host::{FheExecuteArgs, FheExecuteOutput, FheExecuteStep};

#[cfg(feature = "cpi")]
use crate::accounts::{resolve_batch_accounts, BatchAccountResolutionError, ResolvedBatchAccounts};
use crate::accounts::{
    BatchAccountMeta, BatchAccountPurpose, BatchAccountRequirement, BatchAppAuthority,
    BatchOutputAuthorityRequirement,
};
use crate::builder::BatchBuilder;
#[cfg(feature = "cpi")]
use crate::cpi::BatchCpiAccounts;
use crate::Result;

#[cfg(feature = "cpi")]
use anchor_lang::prelude::AccountInfo;

/// Opaque lowered batch request produced by [`Batch::build`].
///
/// App code passes this to [`Batch::execute`] instead of editing raw host
/// args or dynamic account roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub(crate) app_authority: BatchAppAuthority,
    pub(crate) args: FheExecuteArgs,
    /// Exact dynamic `remaining_accounts` order referenced by the `u8` indices
    /// inside `args`. Keep this coupled to `args`; `finish` validates every
    /// index before constructing the batch.
    pub(crate) remaining_accounts: Vec<BatchAccountMeta>,
}

impl Batch {
    /// Builds and validates a batch through a closure. This is the only way to get a
    /// [`BatchBuilder`]: the closure receives it under a fresh `'brand` lifetime that nothing
    /// outside the closure can name, which is what makes a transient value of one builder
    /// unusable in another — the compiler rejects it instead of a runtime tag that on-chain was
    /// the same constant for every builder.
    ///
    /// The closure adds steps and returns nothing: its values belong to the builder, so letting one
    /// out would defeat the brand.
    ///
    /// ```
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Batch, BatchAppAuthority, Output, Scalar, Uint};
    ///
    /// let authority = BatchAppAuthority::new(Pubkey::new_unique());
    /// let batch = Batch::build(authority, |builder| {
    ///     let value = builder.trivial_encrypt_u64(7, Output::transient())?;
    ///     builder.add(value, Scalar::<Uint<64>>::u64(1), Output::transient())?;
    ///     Ok(())
    /// });
    /// assert!(batch.is_ok());
    /// ```
    ///
    /// Feeding one builder's value to another does not compile:
    ///
    /// ```compile_fail
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Batch, BatchAppAuthority, Output, Scalar, Uint};
    ///
    /// let authority = BatchAppAuthority::new(Pubkey::new_unique());
    /// Batch::build(authority, |outer| {
    ///     let borrowed = outer.trivial_encrypt_u64(7, Output::transient())?;
    ///     Batch::build(authority, |inner| {
    ///         inner.add(borrowed, Scalar::<Uint<64>>::u64(1), Output::transient())?;
    ///         Ok(())
    ///     })
    ///     .unwrap();
    ///     Ok(())
    /// })
    /// .unwrap();
    /// ```
    pub fn build<F>(app_authority: BatchAppAuthority, build: F) -> Result<Self>
    where
        F: for<'brand> FnOnce(&mut BatchBuilder<'brand>) -> Result<()>,
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
    ) -> impl ExactSizeIterator<Item = BatchAccountRequirement> + '_ {
        self.remaining_accounts
            .iter()
            .map(BatchAccountRequirement::from)
    }

    #[cfg(feature = "cpi")]
    /// Resolves unordered app-supplied accounts into the exact host
    /// `remaining_accounts` order for this batch.
    ///
    /// `dynamic_accounts` must contain only non-authority batch accounts such as
    /// persistent input ACLs, permission records, transient sessions, and writable
    /// persistent output ACL records. `output_authorities` must contain signer
    /// witnesses for persistent outputs whose app account is not the fixed CPI
    /// `account_authority`.
    pub fn resolve_accounts<'info>(
        &self,
        dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    ) -> std::result::Result<ResolvedBatchAccounts<'info>, BatchAccountResolutionError> {
        resolve_batch_accounts(self, dynamic_accounts, output_authorities)
    }

    pub fn output_authority_requirements(
        &self,
    ) -> impl Iterator<Item = BatchOutputAuthorityRequirement> + '_ {
        std::iter::once(BatchOutputAuthorityRequirement {
            pubkey: self.app_authority.pubkey(),
            cpi_account_authority: true,
        })
        .chain(self.additional_output_authorities().map(|pubkey| {
            BatchOutputAuthorityRequirement {
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

    #[cfg(feature = "cpi")]
    /// Invokes `zama-host::fhe_execute` for this batch with accounts already
    /// resolved by [`Batch::resolve_accounts`].
    pub fn execute<'a, 'info>(
        &self,
        accounts: BatchCpiAccounts<'a, 'info>,
        resolved_accounts: &ResolvedBatchAccounts<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> anchor_lang::prelude::Result<()> {
        crate::cpi::invoke_batch_signed_resolved(self, accounts, resolved_accounts, signer_seeds)
    }

    /// Subjects this batch newly grants through persistent outputs: every output
    /// subject on a create, and `output_subjects \ previous_state.subjects` on a
    /// update that replaces its audience. The host deny-list-checks each of
    /// these exactly like `allow_subjects`, so an app forwarding deny-record
    /// witnesses must cover them alongside the output authorities.
    pub fn newly_granted_subjects(&self) -> Vec<Pubkey> {
        let mut added = Vec::new();
        for step in &self.args.steps {
            let FheExecuteOutput::StoredValue {
                output_subject_indexes,
                previous_state,
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
                let already_stored = previous_state
                    .as_ref()
                    .is_some_and(|previous| previous.subjects.contains(&subject));
                if !already_stored && !added.contains(&subject) {
                    added.push(subject);
                }
            }
        }
        added
    }
}

/// The output policy of a batch step, independent of step kind.
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
