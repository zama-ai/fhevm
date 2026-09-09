//! Lowers builder operands/outputs to the interned wire format.
//!
//! Intern tables are [`TalliedVec`]s that grow through a [`HeapBudget`]. Exact-size
//! allocations (attestation embeds, allow index lists) go through [`HeapBudget::admit`] and
//! [`TalliedVec::try_with_capacity`]. Purpose lists are a stack array and never touch the heap.
//! On drop, an uncommitted step rolls back interned tables in place so a failed step leaves
//! the builder as it found them.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput, PdaSeed};

use crate::accounts::{
    ExecutionAccountMeta, ExecutionAccountPurpose, ExecutionEncryptedValueAccountAuthority,
};
use crate::acl::{Output, OutputKind};
use crate::heap_tally::{HeapBudget, TalliedVec};
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
    budget: &'b mut HeapBudget,
    remaining_accounts_len: usize,
    dictionary_len: usize,
    persistent_producers_len: usize,
    promotions: TalliedVec<(usize, MetaPromotion)>,
    committed: bool,
}

impl Drop for StepTables<'_> {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}

impl<'b> StepTables<'b> {
    pub(crate) fn open(
        remaining_accounts: &'b mut TalliedVec<ExecutionAccountMeta>,
        dictionary: &'b mut TalliedVec<[u8; 32]>,
        persistent_producers: &'b mut TalliedVec<anchor_lang::prelude::Pubkey>,
        budget: &'b mut HeapBudget,
    ) -> Self {
        Self {
            remaining_accounts_len: remaining_accounts.len(),
            dictionary_len: dictionary.len(),
            persistent_producers_len: persistent_producers.len(),
            remaining_accounts,
            dictionary,
            persistent_producers,
            budget,
            promotions: TalliedVec::new(),
            committed: false,
        }
    }

    pub(crate) fn budget(&mut self) -> &mut HeapBudget {
        self.budget
    }

    /// Marks the step as kept. Drop then leaves interned tables as they are.
    pub(crate) fn commit(&mut self) {
        self.committed = true;
    }

    /// Undoes everything this step wrote: promotions newest-first, so an entry promoted twice ends
    /// on the oldest record, then the appended tails. Requested bytes stay requested.
    fn rollback(&mut self) {
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
                meta.purposes.try_insert(purpose);
            }
            // A lookup that changed nothing needs no undo record: a reduction reading one
            // value sixty times would otherwise grow the promotion log sixty entries deep.
            let promoted = meta.is_writable != undo.was_writable
                || meta.is_signer != undo.was_signer
                || meta.purposes.len() != undo.purposes_len;
            if promoted {
                self.promotions.try_push(self.budget, (index, undo))?;
            }
            return u8::try_from(index)
                .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts);
        }
        let index = u8::try_from(self.remaining_accounts.len())
            .map_err(|_| FheExecutionBuildError::TooManyRemainingAccounts)?;
        self.remaining_accounts.try_push(self.budget, required)?;
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
        self.dictionary.try_push(self.budget, bytes)?;
        Ok(index)
    }

    /// The signer slot for a persistent value's authority: none when it is the execution's fixed
    /// CPI signer, otherwise a readonly signing remaining account.
    fn value_authority_index(
        &mut self,
        authority: anchor_lang::prelude::Pubkey,
        execution_authority: ExecutionEncryptedValueAccountAuthority,
    ) -> Result<Option<u8>> {
        if authority == execution_authority.pubkey() {
            return Ok(None);
        }
        self.account_index(ExecutionAccountMeta::readonly_signer(
            authority,
            ExecutionAccountPurpose::PersistentValueAuthority,
        ))
        .map(Some)
    }

    fn persistent_already_written(&self, encrypted_value: &anchor_lang::prelude::Pubkey) -> bool {
        self.persistent_producers.contains(encrypted_value)
    }

    fn record_persistent_producer(
        &mut self,
        encrypted_value: anchor_lang::prelude::Pubkey,
    ) -> Result<()> {
        self.persistent_producers
            .try_push(self.budget, encrypted_value)
    }
}

pub(crate) fn lower_operand(
    tables: &mut StepTables<'_>,
    execution_authority: ExecutionEncryptedValueAccountAuthority,
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
            // Reading a value is admitted by its authority's signature: the host looks the
            // signer up by key, so the slot is not referenced from the wire.
            tables.value_authority_index(
                persistent.encrypted_value_account_authority,
                execution_authority,
            )?;
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
                .ok_or(FheExecutionBuildError::MissingVerifiedInput)?;
            // Admit from the borrowed tables, then clone. On the never-freeing bump a rejected
            // embed must not have already spent those bytes.
            tables.budget.admit(
                std::mem::size_of::<CoprocessorInputAttestation>()
                    + attestation.ct_handles.len() * std::mem::size_of::<[u8; 32]>()
                    + attestation.extra_data.len()
                    + attestation.signatures.len() * std::mem::size_of::<[u8; 65]>(),
            )?;
            Ok(FheExecuteOperand::VerifiedInput {
                attestation: Box::new(attestation.clone()),
            })
        }
        OperandKind::Scalar(value) => Ok(FheExecuteOperand::Scalar {
            value_index: tables.dictionary_index(value)?,
        }),
    }
}

pub(crate) fn lower_output(
    tables: &mut StepTables<'_>,
    execution_authority: ExecutionEncryptedValueAccountAuthority,
    output: Output,
) -> Result<FheExecuteOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheExecuteOutput::Transient),
        OutputKind::Persistent(output) => {
            // Lowering owns the output, so the binding moves the seed and allow lists instead
            // of cloning them — a persistent output allocates nothing here for data the app
            // already built (the interned dictionary entries and the allow index list are the
            // step's only new bytes).
            let binding = output.into_binding()?;
            let encrypted_value = binding.encrypted_value;
            let output_encrypted_value_index =
                tables.account_index(ExecutionAccountMeta::writable(
                    encrypted_value,
                    ExecutionAccountPurpose::PersistentOutputAcl,
                ))?;
            let output_authority = binding.encrypted_value_account_authority;
            let output_authority_index =
                tables.value_authority_index(output_authority, execution_authority)?;
            // Seeds move in place: a 32-byte literal becomes an interned index (the mint or
            // owner is usually already in the dictionary as the scope or authority), anything
            // else stays a literal. No new allocation either way.
            let mut output_authority_seeds = binding.authority_seeds;
            for seed in &mut output_authority_seeds {
                if let PdaSeed::Literal { bytes } = seed {
                    if let Ok(entry) = <[u8; 32]>::try_from(bytes.as_slice()) {
                        *seed = PdaSeed::Interned {
                            index: tables.dictionary_index(entry)?,
                        };
                    }
                }
            }
            // One exact allocation for the allow index list, sized explicitly so the tally is exact.
            let mut output_allow_indexes =
                TalliedVec::try_with_capacity(tables.budget(), binding.allows.len())?;
            for key in &binding.allows {
                let index = tables.dictionary_index(key.to_bytes())?;
                output_allow_indexes.try_push(tables.budget(), index)?;
            }
            let previous_handle_index = binding
                .previous_handle
                .map(|handle| tables.dictionary_index(handle))
                .transpose()?;
            let output = FheExecuteOutput::StoredValue {
                output_encrypted_value_index,
                output_authority_index,
                output_program_index: tables.dictionary_index(binding.app.program.to_bytes())?,
                output_authority_key_index: tables.dictionary_index(output_authority.to_bytes())?,
                output_scope_index: tables.dictionary_index(binding.app.scope)?,
                output_label_index: tables.dictionary_index(binding.label)?,
                output_authority_seeds,
                output_allow_indexes: output_allow_indexes.into_inner(),
                previous_handle_index,
                make_public: binding.make_public,
            };
            tables.record_persistent_producer(encrypted_value)?;
            Ok(output)
        }
    }
}
