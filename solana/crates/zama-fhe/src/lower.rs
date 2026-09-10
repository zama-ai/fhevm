//! Lowers builder operands/outputs to the interned wire format.
//!
//! Intern tables are [`TalliedVec`]s that grow through a [`HeapBudget`]. Exact-size
//! allocations (attestation embeds, allow index lists) go through [`HeapBudget::admit`] and
//! [`TalliedVec::try_with_capacity`]. Purpose lists are a stack array and never touch the heap.
//! On drop, an uncommitted step rolls back interned tables in place so a failed step leaves
//! the builder as it found them.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput};

use crate::accounts::{ExecutionAccountMeta, ExecutionAccountPurpose, ExecutionAuthority};
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
    persistent_producers: &'b mut TalliedVec<(u8, Option<u8>)>,
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
        persistent_producers: &'b mut TalliedVec<(u8, Option<u8>)>,
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

    /// The signer slot for a State authority: none when it is the execution's fixed
    /// CPI signer, otherwise a readonly signing remaining account.
    fn state_authority_index(
        &mut self,
        authority: anchor_lang::prelude::Pubkey,
        execution_authority: ExecutionAuthority,
    ) -> Result<Option<u8>> {
        if authority == execution_authority.pubkey() {
            return Ok(None);
        }
        self.account_index(ExecutionAccountMeta::readonly_signer(
            authority,
            ExecutionAccountPurpose::StateAuthority,
        ))
        .map(Some)
    }
}

pub(crate) fn lower_operand(
    tables: &mut StepTables<'_>,
    execution_authority: ExecutionAuthority,
    produced_count: usize,
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheExecuteOperand> {
    match operand.0 {
        OperandKind::StateSlot { state, key, handle } => {
            let state_index = tables.account_index(ExecutionAccountMeta::readonly(
                state.address,
                ExecutionAccountPurpose::StateInput,
            ))?;
            let key_index = tables.dictionary_index(key)?;
            if tables
                .persistent_producers
                .contains(&(state_index, Some(key_index)))
            {
                return Err(FheExecutionBuildError::StateSlotWrittenEarlier);
            }
            tables.state_authority_index(state.authority, execution_authority)?;
            Ok(FheExecuteOperand::StateSlot {
                state_index,
                key_index,
                handle_index: tables.dictionary_index(handle)?,
            })
        }
        OperandKind::Granted {
            consumer,
            scratch,
            handle,
        } => {
            let consumer_state_index = tables.account_index(ExecutionAccountMeta::readonly(
                consumer.address,
                ExecutionAccountPurpose::StateInput,
            ))?;
            tables.state_authority_index(consumer.authority, execution_authority)?;
            let scratch_index = tables.account_index(ExecutionAccountMeta::readonly(
                scratch,
                ExecutionAccountPurpose::StateInput,
            ))?;
            Ok(FheExecuteOperand::TransientResult {
                consumer_state_index,
                scratch_index,
                handle_index: tables.dictionary_index(handle)?,
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
    execution_authority: ExecutionAuthority,
    output: Output,
) -> Result<FheExecuteOutput> {
    match output.0 {
        OutputKind::State(output) => {
            crate::validate::validate_allow_keys(&output.allows)?;
            if output.grants.len() > zama_host::MAX_TRANSIENT_GRANTS {
                return Err(FheExecutionBuildError::TooManyResultGrants);
            }
            let state_index = tables.account_index(
                if output.slot.is_some() || !output.allows.is_empty() || output.make_public {
                    ExecutionAccountMeta::writable(
                        output.state.address,
                        ExecutionAccountPurpose::StateOutput,
                    )
                } else {
                    ExecutionAccountMeta::readonly(
                        output.state.address,
                        ExecutionAccountPurpose::StateOutput,
                    )
                },
            )?;
            tables.state_authority_index(output.state.authority, execution_authority)?;
            let slot = output
                .slot
                .map(|(key, previous)| -> Result<_> {
                    Ok(zama_host::SlotWrite {
                        key_index: tables.dictionary_index(key)?,
                        previous_handle_index: previous
                            .map(|h| tables.dictionary_index(h))
                            .transpose()?,
                    })
                })
                .transpose()?;
            if let Some(slot) = &slot {
                let claim = (state_index, Some(slot.key_index));
                if tables.persistent_producers.contains(&claim) {
                    return Err(FheExecutionBuildError::StateSlotWrittenEarlier);
                }
                tables.persistent_producers.try_push(tables.budget, claim)?;
            }
            let mut allows = TalliedVec::try_with_capacity(tables.budget(), output.allows.len())?;
            for subject in output.allows {
                let index = tables.dictionary_index(subject.to_bytes())?;
                allows.try_push(tables.budget(), index)?;
            }
            let mut grants = TalliedVec::try_with_capacity(tables.budget(), output.grants.len())?;
            for (initiating, consumer) in output.grants {
                tables.state_authority_index(initiating.authority, execution_authority)?;
                let initiating_state_index =
                    tables.account_index(ExecutionAccountMeta::readonly(
                        initiating.address,
                        ExecutionAccountPurpose::StateInput,
                    ))?;
                let consumer_state_index = tables.account_index(ExecutionAccountMeta::readonly(
                    consumer.address,
                    ExecutionAccountPurpose::StateInput,
                ))?;
                let scratch_index = tables.account_index(ExecutionAccountMeta::writable(
                    initiating.scratch_address(),
                    ExecutionAccountPurpose::StateOutput,
                ))?;
                grants.try_push(
                    tables.budget(),
                    zama_host::ResultGrant {
                        initiating_state_index,
                        consumer_state_index,
                        scratch_index,
                    },
                )?;
            }
            Ok(FheExecuteOutput::State {
                state_index,
                previous_leaf_count: output.previous_leaf_count,
                slot,
                allow_indexes: allows.into_inner(),
                make_public: output.make_public,
                grants: grants.into_inner(),
            })
        }

        OutputKind::Transient => Ok(FheExecuteOutput::Transient),
    }
}
