//! Lowers builder operands/outputs to the interned wire format.
//!
//! This file also owns the whole heap-tally protocol for a step: the three shared tables are
//! [`TalliedVec`]s that pay for their own growth, and every exact-size allocation lowering
//! causes (attestation embeds, subject index lists, purpose tables) goes through
//! [`StepTables::tally_bytes`]. Nothing outside this file and `builder.rs` touches the tally —
//! `accounts.rs` in particular is plain account metadata with no allocator accounting in it.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput};

use crate::accounts::{
    ExecutionAccountMeta, ExecutionAccountPurpose, ExecutionEncryptedValueAccountAuthority,
};
use crate::acl::{Output, OutputKind};
use crate::heap_tally::{tally_push, TalliedVec};
use crate::operand::{Operand, OperandKind};
use crate::{FheExecutionBuildError, Result};

/// What one in-place account widening changed, small enough to record without allocating. The
/// record is only complete because the widening in [`StepTables::account_index`] does exactly
/// two things — OR the flags and append purposes — so anything added there has to be added to
/// the demote arm of [`StepTables::rollback`] in the same edit.
#[derive(Debug)]
struct MetaPromotion {
    was_writable: bool,
    was_signer: bool,
    purposes_len: usize,
}

/// The builder's intern tables for the duration of one step, borrowed in place, plus the undo log
/// that lets a step that fails half-way leave them exactly as it found them.
///
/// Lowering only ever appends to the three tables, with one exception: an account that is already
/// interned is widened in place. So an undo is the recorded lengths plus one small record per
/// promotion — no table is copied, which is what keeps an execution built on-chain inside the
/// SBF entrypoint's fixed 32 KB bump heap.
pub(crate) struct StepTables<'b> {
    remaining_accounts: &'b mut TalliedVec<ExecutionAccountMeta>,
    dictionary: &'b mut TalliedVec<[u8; 32]>,
    persistent_producers: &'b mut TalliedVec<anchor_lang::prelude::Pubkey>,
    /// The builder's explicit counter for exact-size allocations: attestation embeds, subject
    /// index lists, purpose tables. Growth of the shared tables pays into the tables themselves;
    /// rollback keeps every request — on the never-freeing bump region a rolled-back step's
    /// requests are spent all the same.
    explicit_bytes: &'b mut usize,
    remaining_accounts_len: usize,
    dictionary_len: usize,
    persistent_producers_len: usize,
    promotions: TalliedVec<(usize, MetaPromotion)>,
}

impl Drop for StepTables<'_> {
    /// The undo log is step-local; its growth is builder cost like everything else, harvested
    /// into the explicit counter whether the step committed or rolled back.
    fn drop(&mut self) {
        *self.explicit_bytes += self.promotions.requested_bytes();
    }
}

impl<'b> StepTables<'b> {
    pub(crate) fn open(
        remaining_accounts: &'b mut TalliedVec<ExecutionAccountMeta>,
        dictionary: &'b mut TalliedVec<[u8; 32]>,
        persistent_producers: &'b mut TalliedVec<anchor_lang::prelude::Pubkey>,
        explicit_bytes: &'b mut usize,
    ) -> Self {
        Self {
            remaining_accounts_len: remaining_accounts.len(),
            dictionary_len: dictionary.len(),
            persistent_producers_len: persistent_producers.len(),
            remaining_accounts,
            dictionary,
            persistent_producers,
            explicit_bytes,
            promotions: TalliedVec::new(),
        }
    }

    /// Undoes everything this step wrote: promotions newest-first, so an entry promoted twice ends
    /// on the oldest record, then the appended tails. Requested bytes stay requested.
    pub(crate) fn rollback(&mut self) {
        for index in (0..self.promotions.len()).rev() {
            let (meta_index, undo) = &self.promotions[index];
            let meta = self
                .remaining_accounts
                .get_mut(*meta_index)
                .expect("promotion records index an interned account");
            meta.is_writable = undo.was_writable;
            meta.is_signer = undo.was_signer;
            meta.purposes.truncate(undo.purposes_len);
        }
        self.promotions.truncate(0);
        self.remaining_accounts
            .truncate(self.remaining_accounts_len);
        self.dictionary.truncate(self.dictionary_len);
        self.persistent_producers
            .truncate(self.persistent_producers_len);
    }

    pub(crate) fn account_index(&mut self, required: ExecutionAccountMeta) -> Result<u8> {
        // Every constructed meta allocated its one-purpose table, kept, merged away, or rolled
        // back.
        *self.explicit_bytes +=
            required.purposes.len() * std::mem::size_of::<ExecutionAccountPurpose>();
        if let Some(index) = self
            .remaining_accounts
            .iter()
            .position(|candidate| candidate.pubkey == required.pubkey)
        {
            let meta = self
                .remaining_accounts
                .get_mut(index)
                .expect("position returned a valid index");
            let undo = MetaPromotion {
                was_writable: meta.is_writable,
                was_signer: meta.is_signer,
                purposes_len: meta.purposes.len(),
            };
            meta.is_writable |= required.is_writable;
            meta.is_signer |= required.is_signer;
            for purpose in required.purposes {
                if !meta.purposes.contains(&purpose) {
                    tally_push(&meta.purposes, self.explicit_bytes);
                    meta.purposes.push(purpose);
                }
            }
            self.promotions.push((index, undo));
            return u8::try_from(index)
                .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts);
        }
        let index = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        self.remaining_accounts.push(required);
        Ok(index)
    }

    /// Pays step-local allocation bytes into the builder's tally (per-step operand tables that
    /// are not one of the three shared tables).
    pub(crate) fn tally_bytes(&mut self, bytes: usize) {
        *self.explicit_bytes += bytes;
    }

    /// Every byte requested so far by the parts of the builder this step borrows: the explicit
    /// counter, the three shared tables, and the live undo log. `commit_step` adds the tables it
    /// still holds and checks the per-step budget against the sum.
    pub(crate) fn requested_bytes(&self) -> usize {
        *self.explicit_bytes
            + self.remaining_accounts.requested_bytes()
            + self.dictionary.requested_bytes()
            + self.persistent_producers.requested_bytes()
            + self.promotions.requested_bytes()
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

    fn persistent_already_written(&self, encrypted_value: &anchor_lang::prelude::Pubkey) -> bool {
        self.persistent_producers.contains(encrypted_value)
    }

    fn record_persistent_producer(&mut self, encrypted_value: anchor_lang::prelude::Pubkey) {
        self.persistent_producers.push(encrypted_value);
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
            if tables.persistent_already_written(&persistent.encrypted_value) {
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
            // The embed is the attestation's boxed clone: the box itself plus the exact bytes
            // of its three cloned tables. Reusing one verified input across steps pays this per
            // consuming step.
            tables.tally_bytes(
                std::mem::size_of::<CoprocessorInputAttestation>()
                    + attestation.ct_handles.len() * std::mem::size_of::<[u8; 32]>()
                    + attestation.extra_data.len()
                    + attestation.signatures.len() * std::mem::size_of::<[u8; 65]>(),
            );
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
            // Lowering owns the output, so the binding moves the subject list and previous
            // state instead of cloning them — a persistent output allocates nothing here for
            // data the app already built (the interned dictionary entries and the subject
            // index list are the step's only new bytes).
            let binding = output.into_binding()?;
            let encrypted_value = binding.encrypted_value;
            let output_encrypted_value_index =
                tables.account_index(ExecutionAccountMeta::writable(
                    encrypted_value,
                    ExecutionAccountPurpose::PersistentOutputAcl,
                ))?;
            // Both sides are encrypted value account authorities; what differs is the scope. The
            // local is the one this *output* declares, the parameter is the execution's fixed CPI
            // signer. Equal means the output rides that signer and needs no extra account.
            let output_authority = binding.encrypted_value_account_authority;
            let output_authority_index =
                if output_authority == encrypted_value_account_authority.pubkey() {
                    None
                } else {
                    Some(tables.account_index(ExecutionAccountMeta::readonly_signer(
                        output_authority,
                        ExecutionAccountPurpose::PersistentOutputAuthority,
                    ))?)
                };
            // One exact allocation for the index list, sized explicitly so the tally is exact.
            tables.tally_bytes(binding.subjects.len() * std::mem::size_of::<u8>());
            let mut output_subject_indexes = Vec::with_capacity(binding.subjects.len());
            for subject in &binding.subjects {
                output_subject_indexes.push(tables.dictionary_index(subject.to_bytes())?);
            }
            let output = FheExecuteOutput::StoredValue {
                output_encrypted_value_index,
                output_authority_index,
                output_domain_index: tables.dictionary_index(binding.domain.pubkey().to_bytes())?,
                output_account_index: tables.dictionary_index(output_authority.to_bytes())?,
                output_label_index: tables.dictionary_index(binding.label)?,
                output_subject_indexes,
                previous_state: binding.previous.map(|previous| zama_host::PreviousState {
                    handle: previous.handle,
                    subjects: previous.subjects,
                }),
                make_public: binding.make_public,
            };
            tables.record_persistent_producer(encrypted_value);
            Ok(output)
        }
    }
}
