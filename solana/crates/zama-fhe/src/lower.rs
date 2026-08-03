//! Lowers builder operands/outputs to the interned wire format.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput};

use crate::accounts::{BatchAccountMeta, BatchAccountPurpose, BatchAppAuthority, MetaPromotion};
use crate::acl::{Output, OutputKind};
use crate::operand::{BatchBuilderScope, Operand, OperandKind};
use crate::{BatchBuildError, Result};

/// The builder's intern tables for the duration of one step, borrowed in place, plus the undo log
/// that lets a step that fails half-way leave them exactly as it found them.
///
/// Lowering only ever appends to the three tables, with one exception: an account that is already
/// interned is widened in place (`BatchAccountMeta::promote`). So an undo is the recorded lengths
/// plus one small record per promotion — no table is copied, which is what keeps a batch built
/// on-chain inside Anchor's default 32 KB bump heap.
pub(crate) struct StepTables<'b> {
    remaining_accounts: &'b mut Vec<BatchAccountMeta>,
    dictionary: &'b mut Vec<[u8; 32]>,
    persistent_producers: &'b mut Vec<(anchor_lang::prelude::Pubkey, u8)>,
    remaining_accounts_len: usize,
    dictionary_len: usize,
    persistent_producers_len: usize,
    promotions: Vec<(usize, MetaPromotion)>,
}

impl<'b> StepTables<'b> {
    pub(crate) fn open(
        remaining_accounts: &'b mut Vec<BatchAccountMeta>,
        dictionary: &'b mut Vec<[u8; 32]>,
        persistent_producers: &'b mut Vec<(anchor_lang::prelude::Pubkey, u8)>,
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

    pub(crate) fn account_index(&mut self, required: BatchAccountMeta) -> Result<u8> {
        if let Some(index) = self
            .remaining_accounts
            .iter()
            .position(|candidate| candidate.pubkey == required.pubkey)
        {
            let undo = self.remaining_accounts[index].promote(required);
            self.promotions.push((index, undo));
            return u8::try_from(index).map_err(|_| BatchBuildError::TooManyRemainingAccounts);
        }
        let index = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| BatchBuildError::TooManyRemainingAccounts)?;
        self.remaining_accounts.push(required);
        Ok(index)
    }

    /// Interns a 32-byte constant into the batch dictionary, reusing an existing entry
    /// byte-for-byte.
    pub(crate) fn dictionary_index(&mut self, bytes: [u8; 32]) -> Result<u8> {
        if let Some(index) = self.dictionary.iter().position(|entry| *entry == bytes) {
            return u8::try_from(index).map_err(|_| BatchBuildError::TooManyDictionaryEntries);
        }
        let index = u8::try_from(self.dictionary.len())
            .map_err(|_| BatchBuildError::TooManyDictionaryEntries)?;
        self.dictionary.push(bytes);
        Ok(index)
    }
}

pub(crate) fn lower_operand(
    tables: &mut StepTables<'_>,
    produced_count: usize,
    builder_scope: BatchBuilderScope,
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheExecuteOperand> {
    match operand.0 {
        OperandKind::Persistent(persistent) => {
            if tables
                .persistent_producers
                .iter()
                .any(|(account, _)| *account == persistent.encrypted_value)
            {
                return Err(BatchBuildError::PersistentOperandWrittenEarlier);
            }
            let handle_index = tables.dictionary_index(persistent.handle)?;
            let encrypted_value_index = tables.account_index(BatchAccountMeta::readonly(
                persistent.encrypted_value,
                BatchAccountPurpose::PersistentInputAcl,
            ))?;
            Ok(FheExecuteOperand::StoredValue {
                handle_index,
                encrypted_value_index,
            })
        }
        OperandKind::Transient {
            producer_index,
            builder_scope: operand_builder_scope,
        } => {
            if operand_builder_scope != builder_scope {
                return Err(BatchBuildError::InvalidTransientReference);
            }
            if producer_index as usize >= produced_count {
                return Err(BatchBuildError::InvalidTransientReference);
            }
            Ok(FheExecuteOperand::EarlierStep { producer_index })
        }
        OperandKind::VerifiedInput {
            attestation_index, ..
        } => {
            let attestation = verified_inputs
                .get(attestation_index as usize)
                .ok_or(BatchBuildError::MissingVerifiedInput)?
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
    app_authority: BatchAppAuthority,
    producer_index: u8,
    output: Output,
) -> Result<FheExecuteOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheExecuteOutput::Transient),
        OutputKind::Persistent(output) => {
            let binding = output.binding()?;
            let encrypted_value = binding.encrypted_value();
            let output_encrypted_value_index = tables.account_index(BatchAccountMeta::writable(
                binding.encrypted_value(),
                BatchAccountPurpose::PersistentOutputAcl,
            ))?;
            let output_account_authority_index = if binding.account() == app_authority.pubkey() {
                None
            } else {
                Some(tables.account_index(BatchAccountMeta::readonly_signer(
                    binding.account(),
                    BatchAccountPurpose::PersistentOutputAuthority,
                ))?)
            };
            let output_subject_indexes = binding
                .host_subjects()
                .into_iter()
                .map(|subject| tables.dictionary_index(subject.to_bytes()))
                .collect::<Result<Vec<u8>>>()?;
            let output = FheExecuteOutput::StoredValue {
                output_encrypted_value_index,
                output_account_authority_index,
                output_domain_index: tables.dictionary_index(binding.domain().to_bytes())?,
                output_account_index: tables.dictionary_index(binding.account().to_bytes())?,
                output_label_index: tables.dictionary_index(binding.label())?,
                output_subject_indexes,
                previous_state: binding.previous_state(),
                make_public: binding.make_public(),
            };
            tables
                .persistent_producers
                .push((encrypted_value, producer_index));
            Ok(output)
        }
    }
}
