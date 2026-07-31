//! Lowers builder operands/outputs to the interned wire format.

use zama_host::{CoprocessorInputAttestation, FheExecuteOperand, FheExecuteOutput};

use crate::accounts::{BatchAccountMeta, BatchAccountPurpose, BatchAppAuthority};
use crate::acl::{Output, OutputKind};
use crate::operand::{EvalBuilderScope, Operand, OperandKind};
use crate::{BatchBuildError, Result};

pub(crate) fn lower_operand(
    remaining_accounts: &mut Vec<BatchAccountMeta>,
    dictionary: &mut Vec<[u8; 32]>,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    persistent_producers: &[(anchor_lang::prelude::Pubkey, u16)],
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheExecuteOperand> {
    match operand.0 {
        OperandKind::Persistent(persistent) => {
            if persistent_producers
                .iter()
                .any(|(account, _)| *account == persistent.encrypted_value)
            {
                return Err(BatchBuildError::PersistentOperandWrittenEarlier);
            }
            let handle_index = dictionary_index(dictionary, persistent.handle)?;
            let encrypted_value_index = account_index(
                remaining_accounts,
                BatchAccountMeta::readonly(
                    persistent.encrypted_value,
                    BatchAccountPurpose::PersistentInputAcl,
                ),
            )?;
            Ok(FheExecuteOperand::AllowedPersistent {
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
            let producer_index =
                u8::try_from(producer_index).map_err(|_| BatchBuildError::TooManyOps)?;
            Ok(FheExecuteOperand::AllowedLocal { producer_index })
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
            value_index: dictionary_index(dictionary, value)?,
        }),
    }
}

pub(crate) fn lower_output(
    remaining_accounts: &mut Vec<BatchAccountMeta>,
    dictionary: &mut Vec<[u8; 32]>,
    app_authority: BatchAppAuthority,
    persistent_producers: &mut Vec<(anchor_lang::prelude::Pubkey, u16)>,
    producer_index: u16,
    output: Output,
) -> Result<FheExecuteOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheExecuteOutput::AllowedLocal),
        OutputKind::Persistent(output) => {
            let binding = output.binding()?;
            let encrypted_value = binding.encrypted_value();
            let output_encrypted_value_index = account_index(
                remaining_accounts,
                BatchAccountMeta::writable(
                    binding.encrypted_value(),
                    BatchAccountPurpose::PersistentOutputAcl,
                ),
            )?;
            let output_account_authority_index = if binding.account() == app_authority.pubkey() {
                None
            } else {
                Some(account_index(
                    remaining_accounts,
                    BatchAccountMeta::readonly_signer(
                        binding.account(),
                        BatchAccountPurpose::PersistentOutputAuthority,
                    ),
                )?)
            };
            let output_subject_indexes = binding
                .host_subjects()
                .into_iter()
                .map(|subject| dictionary_index(dictionary, subject.to_bytes()))
                .collect::<Result<Vec<u8>>>()?;
            let output = FheExecuteOutput::AllowedPersistent {
                output_encrypted_value_index,
                output_account_authority_index,
                output_domain_index: dictionary_index(dictionary, binding.domain().to_bytes())?,
                output_account_index: dictionary_index(dictionary, binding.account().to_bytes())?,
                output_label_index: dictionary_index(dictionary, binding.label())?,
                output_subject_indexes,
                previous_handle: binding.previous_handle(),
                previous_subjects: binding.previous_subjects().map(|s| s.to_vec()),
                make_public: binding.make_public(),
            };
            persistent_producers.push((encrypted_value, producer_index));
            Ok(output)
        }
    }
}

/// Interns a 32-byte constant into the batch dictionary, reusing an existing entry byte-for-byte.
fn dictionary_index(dictionary: &mut Vec<[u8; 32]>, bytes: [u8; 32]) -> Result<u8> {
    if let Some(index) = dictionary.iter().position(|entry| *entry == bytes) {
        return u8::try_from(index).map_err(|_| BatchBuildError::TooManyDictionaryEntries);
    }
    let index =
        u8::try_from(dictionary.len()).map_err(|_| BatchBuildError::TooManyDictionaryEntries)?;
    dictionary.push(bytes);
    Ok(index)
}

fn account_index(
    remaining_accounts: &mut Vec<BatchAccountMeta>,
    required: BatchAccountMeta,
) -> Result<u8> {
    if let Some(index) = remaining_accounts
        .iter()
        .position(|candidate| candidate.pubkey == required.pubkey)
    {
        remaining_accounts[index].promote(required);
        return u8::try_from(index).map_err(|_| BatchBuildError::TooManyRemainingAccounts);
    }
    let index = u8::try_from(remaining_accounts.len())
        .map_err(|_| BatchBuildError::TooManyRemainingAccounts)?;
    remaining_accounts.push(required);
    Ok(index)
}
