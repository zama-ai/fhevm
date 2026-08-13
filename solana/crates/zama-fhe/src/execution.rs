//! The validated, lowered execution request handed to the CPI helpers.
//!
//! Public API surface: app programs. The requirement accessors
//! ([`FheExecution::dynamic_account_requirements`],
//! [`FheExecution::output_authority_requirements`]) are how a caller that assembles the
//! transaction's account list — an off-chain client or a wrapping instruction — learns which
//! dynamic accounts the execution needs and in which roles; resolution itself re-reads the
//! account metas directly.

use anchor_lang::prelude::Pubkey;

use zama_host::{FheExecuteArgs, FheExecuteOutput, FheExecuteStep};

#[cfg(feature = "cpi")]
use crate::accounts::{
    resolve_execution_accounts, ExecutionAccountResolutionError, ResolvedExecutionAccounts,
};
use crate::accounts::{
    ExecutionAccountMeta, ExecutionAccountPurpose, ExecutionAccountRequirement,
    ExecutionEncryptedValueAccountAuthority, ExecutionOutputAuthorityRequirement,
};
use crate::builder::FheExecutionBuilder;
#[cfg(feature = "cpi")]
use crate::cpi::ExecutionCpiAccounts;
use crate::Result;

#[cfg(feature = "cpi")]
use anchor_lang::prelude::AccountInfo;

/// Opaque lowered execution request produced by [`FheExecution::build`].
///
/// App code passes this to [`FheExecution::invoke`] instead of editing raw host
/// args or dynamic account roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FheExecution {
    pub(crate) encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    pub(crate) args: FheExecuteArgs,
    /// Exact dynamic `remaining_accounts` order referenced by the `u8` indices
    /// inside `args`. Keep this coupled to `args`; `finish` validates every
    /// index before constructing the execution.
    pub(crate) remaining_accounts: Vec<ExecutionAccountMeta>,
    /// Shape-derived cost against the transaction ceilings, computed by `finish`.
    pub(crate) cost: crate::cost::FheExecutionCost,
}

impl FheExecution {
    /// Builds and validates an execution through a closure. This is the only way to get a
    /// [`FheExecutionBuilder`]: the closure receives it under a fresh `'id` lifetime that nothing
    /// outside the closure can name, which is what makes a transient value of one builder
    /// unusable in another — the compiler rejects it instead of a runtime tag that on-chain was
    /// the same constant for every builder.
    ///
    /// The closure adds steps and returns nothing: its values belong to the builder, so letting one
    /// out would defeat it.
    ///
    /// ```
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{FheExecution, ExecutionEncryptedValueAccountAuthority, Output, Scalar, Uint};
    ///
    /// let authority = ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique());
    /// let execution = FheExecution::build(authority, |builder| {
    ///     let value = builder.trivial_encrypt_u64(7, Output::transient())?;
    ///     builder.add(value, Scalar::<Uint<64>>::u64(1), Output::transient())?;
    ///     Ok(())
    /// });
    /// assert!(execution.is_ok());
    /// ```
    ///
    /// Feeding one builder's value to another does not compile:
    ///
    /// ```compile_fail
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{FheExecution, ExecutionEncryptedValueAccountAuthority, Output, Scalar, Uint};
    ///
    /// let authority = ExecutionEncryptedValueAccountAuthority::new(Pubkey::new_unique());
    /// FheExecution::build(authority, |outer| {
    ///     let borrowed = outer.trivial_encrypt_u64(7, Output::transient())?;
    ///     FheExecution::build(authority, |inner| {
    ///         inner.add(borrowed, Scalar::<Uint<64>>::u64(1), Output::transient())?;
    ///         Ok(())
    ///     })
    ///     .unwrap();
    ///     Ok(())
    /// })
    /// .unwrap();
    /// ```
    pub fn build<F>(
        encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
        build: F,
    ) -> Result<Self>
    where
        F: for<'id> FnOnce(&mut FheExecutionBuilder<'id>) -> Result<()>,
    {
        let mut builder = FheExecutionBuilder::new(encrypted_value_account_authority);
        build(&mut builder)?;
        builder.finish()
    }

    pub fn encrypted_value_account_authority(&self) -> ExecutionEncryptedValueAccountAuthority {
        self.encrypted_value_account_authority
    }

    /// What this execution costs against the transaction ceilings: exact packet bytes, the
    /// guaranteed instruction-trace floor, and the state-dependent worst case. An app composing
    /// a transaction with more than the minimal wrapper budgets its own instructions and CPIs
    /// out of what [`crate::TRANSACTION_INSTRUCTION_TRACE_LIMIT`] leaves over the floor.
    pub fn cost(&self) -> crate::cost::FheExecutionCost {
        self.cost
    }

    pub fn dynamic_account_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = ExecutionAccountRequirement> + '_ {
        self.remaining_accounts
            .iter()
            .map(ExecutionAccountRequirement::from)
    }

    #[cfg(feature = "cpi")]
    /// Resolves unordered app-supplied accounts into the exact host
    /// `remaining_accounts` order for this execution.
    ///
    /// `dynamic_accounts` must contain only non-authority execution accounts such as
    /// persistent input ACLs, permission records, transient sessions, and writable
    /// persistent output ACL records. `output_authorities` must contain signer
    /// witnesses for persistent outputs whose authority is not the fixed CPI
    /// `encrypted_value_account_authority`.
    pub fn resolve_accounts<'info>(
        &self,
        dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    ) -> std::result::Result<ResolvedExecutionAccounts<'info>, ExecutionAccountResolutionError>
    {
        resolve_execution_accounts(self, dynamic_accounts, output_authorities)
    }

    pub fn output_authority_requirements(
        &self,
    ) -> impl Iterator<Item = ExecutionOutputAuthorityRequirement> + '_ {
        std::iter::once(ExecutionOutputAuthorityRequirement {
            pubkey: self.encrypted_value_account_authority.pubkey(),
        })
        .chain(
            self.additional_output_authorities()
                .map(|pubkey| ExecutionOutputAuthorityRequirement { pubkey }),
        )
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
                    .contains(&ExecutionAccountPurpose::PersistentOutputAuthority)
            })
            .map(|account| account.pubkey)
    }

    #[cfg(feature = "cpi")]
    /// Invokes `zama-host::fhe_execute` for this execution with accounts already
    /// resolved by [`FheExecution::resolve_accounts`].
    ///
    /// Consumes the execution: the final account count is stamped into the args in place and the
    /// packet serialized once — an execution is a single-use request, and keeping it borrowable
    /// here would force a deep copy of every step and dictionary entry on the never-freeing
    /// program heap just to set one byte.
    pub fn invoke<'a, 'info>(
        mut self,
        accounts: ExecutionCpiAccounts<'a, 'info>,
        resolved_accounts: &ResolvedExecutionAccounts<'info>,
        signer_seeds: &[&[&[u8]]],
    ) -> anchor_lang::prelude::Result<()> {
        crate::cpi::invoke_execution_signed_resolved(
            &mut self,
            accounts,
            resolved_accounts,
            signer_seeds,
        )
    }

    /// Subjects this execution newly grants through persistent outputs: every output
    /// subject on a create, and `output_subjects \ previous_state.subjects` on a
    /// update that replaces its audience. The host deny-list-checks each of
    /// these exactly like `allow_subjects`, so an app forwarding deny-record
    /// witnesses must cover them alongside the grant authority whenever the
    /// execution actually adds a subject.
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

/// The output policy of an execution step, independent of step kind.
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

/// Exact byte length of the `fhe_execute` instruction packet — discriminator plus borsh-encoded
/// args — counted through a sink writer, so measuring a packet allocates nothing. `finish`
/// checks this against the CPI data limit; the CPI path sizes its one real buffer with it.
pub(crate) fn packet_byte_count(args: &FheExecuteArgs) -> usize {
    use anchor_lang::{AnchorSerialize, Discriminator};

    struct CountingWriter(usize);
    impl std::io::Write for CountingWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0 += buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut counter = CountingWriter(0);
    args.serialize(&mut counter)
        .expect("counting borsh writer cannot fail");
    zama_host::instruction::FheExecute::DISCRIMINATOR.len() + counter.0
}

/// The `fhe_execute` instruction packet, serialized once into a right-sized buffer.
///
/// The generated `zama_host::instruction::FheExecute` wrapper takes the args by value, which
/// would force a deep copy of every step and dictionary entry; writing the discriminator and then
/// borsh-serializing the borrowed args produces byte-identical data (asserted below). The
/// counting pre-pass matters as much as the avoided copy: a packet-sized `Vec` growing by
/// doubling abandons roughly another packet of bytes on the never-freeing program heap.
// Gated with its callers (the CPI path and the heap-budget tests) so a per-crate build
// without `cpi` does not report it dead.
#[cfg(any(feature = "cpi", test))]
pub(crate) fn fhe_execute_instruction_data(args: &FheExecuteArgs) -> Vec<u8> {
    use anchor_lang::{AnchorSerialize, Discriminator};

    let mut data = Vec::with_capacity(packet_byte_count(args));
    data.extend_from_slice(zama_host::instruction::FheExecute::DISCRIMINATOR);
    args.serialize(&mut data)
        .expect("borsh serialization into a Vec cannot fail");
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::InstructionData as _;

    /// Pins [`fhe_execute_instruction_data`] to the derived encoder: byte-for-byte what
    /// `zama_host::instruction::FheExecute { args }.data()` produces, without the copy.
    #[test]
    fn hand_assembled_packet_matches_the_generated_wrapper() {
        let args = FheExecuteArgs {
            account_count: 3,
            dictionary: vec![[7u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [1u8; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::Binary {
                    op: zama_host::FheBinaryOpCode::Add,
                    lhs: zama_host::FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: zama_host::FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
            ],
        };
        assert_eq!(
            fhe_execute_instruction_data(&args),
            zama_host::instruction::FheExecute { args }.data(),
        );
    }
}
