//! Lowers builder operands/outputs to the interned wire format.

use zama_host::{CoprocessorInputAttestation, FheEvalOperand, FheEvalOutput};

use crate::accounts::{EvalAccountMeta, EvalAccountPurpose, EvalAppAuthority};
use crate::acl::{Output, OutputKind};
use crate::operand::{EvalBuilderScope, Operand, OperandKind};
use crate::{EvalBuildError, Result};

pub(crate) fn lower_operand(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    dictionary: &mut Vec<[u8; 32]>,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    persistent_producers: &[(anchor_lang::prelude::Pubkey, u16)],
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheEvalOperand> {
    match operand.0 {
        OperandKind::Persistent(persistent) => {
            if persistent_producers
                .iter()
                .any(|(account, _)| *account == persistent.encrypted_value)
            {
                return Err(EvalBuildError::PersistentOperandWrittenEarlier);
            }
            let handle_index = dictionary_index(dictionary, persistent.handle)?;
            let encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::readonly(
                    persistent.encrypted_value,
                    EvalAccountPurpose::PersistentInputAcl,
                ),
            )?;
            Ok(FheEvalOperand::AllowedPersistent {
                handle_index,
                encrypted_value_index,
            })
        }
        OperandKind::Transient {
            producer_index,
            builder_scope: operand_builder_scope,
        } => {
            if operand_builder_scope != builder_scope {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            if producer_index as usize >= produced_count {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            let producer_index =
                u8::try_from(producer_index).map_err(|_| EvalBuildError::TooManyOps)?;
            Ok(FheEvalOperand::AllowedLocal { producer_index })
        }
        OperandKind::VerifiedInput {
            attestation_index, ..
        } => {
            let attestation = verified_inputs
                .get(attestation_index as usize)
                .ok_or(EvalBuildError::MissingVerifiedInput)?
                .clone();
            Ok(FheEvalOperand::VerifiedInput {
                attestation: Box::new(attestation),
            })
        }
        OperandKind::Scalar(value) => Ok(FheEvalOperand::Scalar {
            value_index: dictionary_index(dictionary, value)?,
        }),
    }
}

pub(crate) fn lower_output(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    dictionary: &mut Vec<[u8; 32]>,
    app_authority: EvalAppAuthority,
    persistent_producers: &mut Vec<(anchor_lang::prelude::Pubkey, u16)>,
    producer_index: u16,
    output: Output,
) -> Result<FheEvalOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheEvalOutput::AllowedLocal),
        OutputKind::Persistent(output) => {
            let binding = output.binding()?;
            let encrypted_value = binding.encrypted_value();
            let output_encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::writable(
                    binding.encrypted_value(),
                    EvalAccountPurpose::PersistentOutputAcl,
                ),
            )?;
            let output_app_account_authority_index =
                if binding.app_account() == app_authority.pubkey() {
                    None
                } else {
                    Some(account_index(
                        remaining_accounts,
                        EvalAccountMeta::readonly_signer(
                            binding.app_account(),
                            EvalAccountPurpose::PersistentOutputAuthority,
                        ),
                    )?)
                };
            let output_subject_indexes = binding
                .host_subjects()
                .into_iter()
                .map(|subject| dictionary_index(dictionary, subject.to_bytes()))
                .collect::<Result<Vec<u8>>>()?;
            let output = FheEvalOutput::AllowedPersistent {
                output_encrypted_value_index,
                output_app_account_authority_index,
                output_acl_domain_key_index: dictionary_index(
                    dictionary,
                    binding.acl_domain_key().to_bytes(),
                )?,
                output_app_account_index: dictionary_index(
                    dictionary,
                    binding.app_account().to_bytes(),
                )?,
                output_encrypted_value_label_index: dictionary_index(
                    dictionary,
                    binding.encrypted_value_label(),
                )?,
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

/// Interns a 32-byte constant into the frame dictionary, reusing an existing entry byte-for-byte.
fn dictionary_index(dictionary: &mut Vec<[u8; 32]>, bytes: [u8; 32]) -> Result<u8> {
    if let Some(index) = dictionary.iter().position(|entry| *entry == bytes) {
        return u8::try_from(index).map_err(|_| EvalBuildError::TooManyDictionaryEntries);
    }
    let index =
        u8::try_from(dictionary.len()).map_err(|_| EvalBuildError::TooManyDictionaryEntries)?;
    dictionary.push(bytes);
    Ok(index)
}

fn account_index(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    required: EvalAccountMeta,
) -> Result<u8> {
    if let Some(index) = remaining_accounts
        .iter()
        .position(|candidate| candidate.pubkey == required.pubkey)
    {
        remaining_accounts[index].promote(required);
        return u8::try_from(index).map_err(|_| EvalBuildError::TooManyRemainingAccounts);
    }
    let index = u8::try_from(remaining_accounts.len())
        .map_err(|_| EvalBuildError::TooManyRemainingAccounts)?;
    remaining_accounts.push(required);
    Ok(index)
}
