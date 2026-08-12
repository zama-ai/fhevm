//! Lowers builder operands/outputs to the interned wire format.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput};

use crate::accounts::{
    ExecutionAccountMeta, ExecutionAccountPurpose, ExecutionEncryptedValueAccountAuthority,
    MetaPromotion,
};
use crate::acl::{Output, OutputKind};
use crate::operand::{Operand, OperandKind};
use crate::{FheExecutionBuildError, Result};

/// The builder's intern tables for the duration of one step, borrowed in place, plus the undo log
/// that lets a step that fails half-way leave them exactly as it found them.
///
/// Lowering only ever appends to the three tables, with one exception: an account that is already
/// interned is widened in place (`ExecutionAccountMeta::promote`). So an undo is the recorded lengths
/// plus one small record per promotion — no table is copied, which is what keeps an execution built
/// on-chain inside the SBF entrypoint's fixed 32 KB bump heap.
pub(crate) struct StepTables<'b> {
    remaining_accounts: &'b mut Vec<ExecutionAccountMeta>,
    dictionary: &'b mut Vec<[u8; 32]>,
    persistent_producers: &'b mut Vec<anchor_lang::prelude::Pubkey>,
    remaining_accounts_len: usize,
    dictionary_len: usize,
    persistent_producers_len: usize,
    promotions: Vec<(usize, MetaPromotion)>,
}

impl<'b> StepTables<'b> {
    pub(crate) fn open(
        remaining_accounts: &'b mut Vec<ExecutionAccountMeta>,
        dictionary: &'b mut Vec<[u8; 32]>,
        persistent_producers: &'b mut Vec<anchor_lang::prelude::Pubkey>,
    ) -> Self {
        Self {
            remaining_accounts_len: remaining_accounts.len(),
            dictionary_len: dictionary.len(),
            persistent_producers_len: persistent_producers.len(),
            remaining_accounts,
            dictionary,
            persistent_producers,
            promotions: Vec::new(),
        }
    }

    /// Undoes everything this step wrote: promotions newest-first, so an entry promoted twice ends
    /// on the oldest record, then the appended tails.
    pub(crate) fn rollback(self) {
        for (index, undo) in self.promotions.into_iter().rev() {
            self.remaining_accounts[index].demote(undo);
        }
        self.remaining_accounts
            .truncate(self.remaining_accounts_len);
        self.dictionary.truncate(self.dictionary_len);
        self.persistent_producers
            .truncate(self.persistent_producers_len);
    }

    pub(crate) fn account_index(&mut self, required: ExecutionAccountMeta) -> Result<u8> {
        if let Some(index) = self
            .remaining_accounts
            .iter()
            .position(|candidate| candidate.pubkey == required.pubkey)
        {
            let undo = self.remaining_accounts[index].promote(required);
            self.promotions.push((index, undo));
            return u8::try_from(index)
                .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts);
        }
        let index = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        self.remaining_accounts.push(required);
        Ok(index)
    }

    /// Interns a 32-byte constant into the execution dictionary, reusing an existing entry
    /// byte-for-byte.
    pub(crate) fn dictionary_index(&mut self, bytes: [u8; 32]) -> Result<u8> {
        if let Some(index) = self.dictionary.iter().position(|entry| *entry == bytes) {
            return u8::try_from(index)
                .map_err(|_| FheExecutionBuildError::TooManyDictionaryEntries);
        }
        let index = u8::try_from(self.dictionary.len())
            .map_err(|_| FheExecutionBuildError::TooManyDictionaryEntries)?;
        self.dictionary.push(bytes);
        Ok(index)
    }
}

pub(crate) fn lower_operand(
    tables: &mut StepTables<'_>,
    produced_count: usize,
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheExecuteOperand> {
    match operand.0 {
        OperandKind::Persistent(persistent) => {
            if tables
                .persistent_producers
                .contains(&persistent.encrypted_value)
            {
                return Err(FheExecutionBuildError::PersistentOperandWrittenEarlier);
            }
            let handle_index = tables.dictionary_index(persistent.handle)?;
            let encrypted_value_index = tables.account_index(ExecutionAccountMeta::readonly(
                persistent.encrypted_value,
                ExecutionAccountPurpose::PersistentInputAcl,
            ))?;
            Ok(FheExecuteOperand::StoredValue {
                handle_index,
                encrypted_value_index,
            })
        }
        OperandKind::Transient { producer_index } => {
            if producer_index as usize >= produced_count {
                return Err(FheExecutionBuildError::InvalidTransientReference);
            }
            Ok(FheExecuteOperand::EarlierStep { producer_index })
        }
        OperandKind::VerifiedInput {
            attestation_index, ..
        } => {
            let attestation = verified_inputs
                .get(attestation_index as usize)
                .ok_or(FheExecutionBuildError::MissingVerifiedInput)?
                .clone();
            Ok(FheExecuteOperand::VerifiedInput {
                attestation: Box::new(attestation),
            })
        }
        OperandKind::Scalar(value) => Ok(FheExecuteOperand::Scalar {
            value_index: tables.dictionary_index(value)?,
        }),
    }
}

pub(crate) fn lower_output(
    tables: &mut StepTables<'_>,
    encrypted_value_account_authority: ExecutionEncryptedValueAccountAuthority,
    output: Output,
) -> Result<FheExecuteOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheExecuteOutput::Transient),
        OutputKind::Persistent(output) => {
            let binding = output.binding()?;
            let encrypted_value = binding.encrypted_value();
            let output_encrypted_value_index =
                tables.account_index(ExecutionAccountMeta::writable(
                    binding.encrypted_value(),
                    ExecutionAccountPurpose::PersistentOutputAcl,
                ))?;
            // Both sides are encrypted value account authorities; what differs is the scope. The
            // local is the one this *output* declares, the parameter is the execution's fixed CPI
            // signer. Equal means the output rides that signer and needs no extra account.
            let output_authority = binding.encrypted_value_account_authority();
            let output_authority_index =
                if output_authority == encrypted_value_account_authority.pubkey() {
                    None
                } else {
                    Some(tables.account_index(ExecutionAccountMeta::readonly_signer(
                        output_authority,
                        ExecutionAccountPurpose::PersistentOutputAuthority,
                    ))?)
                };
            let output_subject_indexes = binding
                .host_subjects()
                .into_iter()
                .map(|subject| tables.dictionary_index(subject.to_bytes()))
                .collect::<Result<Vec<u8>>>()?;
            let output = FheExecuteOutput::StoredValue {
                output_encrypted_value_index,
                output_authority_index,
                output_domain_index: tables
                    .dictionary_index(binding.domain().pubkey().to_bytes())?,
                output_account_index: tables
                    .dictionary_index(binding.encrypted_value_account_authority().to_bytes())?,
                output_label_index: tables.dictionary_index(binding.encrypted_value_label())?,
                output_subject_indexes,
                previous_state: binding.previous_state(),
                make_public: binding.make_public(),
            };
            tables.persistent_producers.push(encrypted_value);
            Ok(output)
        }
    }
}
