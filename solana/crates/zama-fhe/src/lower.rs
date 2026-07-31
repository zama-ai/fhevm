//! Lowers builder operands/outputs to the interned wire format.

use zama_host::{CoprocessorInputAttestation, FheEvalOperand, FheEvalOutput};

use crate::accounts::{EvalAccountMeta, EvalAccountPurpose, EvalAppAuthority};
use crate::acl::{Output, OutputKind};
use crate::operand::{EvalBuilderScope, Operand, OperandKind};
use crate::{EvalBuildError, Result};

pub(crate) fn lower_operand(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    pool: &mut Vec<[u8; 32]>,
    produced_count: usize,
    builder_scope: EvalBuilderScope,
    durable_producers: &[(anchor_lang::prelude::Pubkey, u16)],
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheEvalOperand> {
    match operand.0 {
        OperandKind::Durable(durable) => {
            if let Some((_, producer_index)) = durable_producers
                .iter()
                .rev()
                .find(|(account, _)| *account == durable.encrypted_value)
            {
                let producer_index =
                    u8::try_from(*producer_index).map_err(|_| EvalBuildError::TooManyOps)?;
                return Ok(FheEvalOperand::AllowedLocal { producer_index });
            }
            let handle_index = pool_index(pool, durable.handle)?;
            let encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::readonly(
                    durable.encrypted_value,
                    EvalAccountPurpose::DurableInputAcl,
                ),
            )?;
            Ok(FheEvalOperand::AllowedDurable {
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
            value_index: pool_index(pool, value)?,
        }),
    }
}

pub(crate) fn lower_output(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    pool: &mut Vec<[u8; 32]>,
    app_authority: EvalAppAuthority,
    durable_producers: &mut Vec<(anchor_lang::prelude::Pubkey, u16)>,
    producer_index: u16,
    output: Output,
) -> Result<FheEvalOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheEvalOutput::AllowedLocal),
        OutputKind::Durable(output) => {
            let birth = output.birth()?;
            let encrypted_value = birth.encrypted_value();
            let output_encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::writable(
                    birth.encrypted_value(),
                    EvalAccountPurpose::DurableOutputAcl,
                ),
            )?;
            let output_app_account_authority_index =
                if birth.app_account() == app_authority.pubkey() {
                    None
                } else {
                    Some(account_index(
                        remaining_accounts,
                        EvalAccountMeta::readonly_signer(
                            birth.app_account(),
                            EvalAccountPurpose::DurableOutputAuthority,
                        ),
                    )?)
                };
            let output_subject_indexes = birth
                .host_subjects()
                .into_iter()
                .map(|subject| pool_index(pool, subject.to_bytes()))
                .collect::<Result<Vec<u8>>>()?;
            let output = FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                output_app_account_authority_index,
                output_acl_domain_key_index: pool_index(pool, birth.acl_domain_key().to_bytes())?,
                output_app_account_index: pool_index(pool, birth.app_account().to_bytes())?,
                output_encrypted_value_label_index: pool_index(
                    pool,
                    birth.encrypted_value_label(),
                )?,
                output_subject_indexes,
                previous_handle: birth.previous_handle(),
                previous_subjects: birth.previous_subjects().map(|s| s.to_vec()),
                make_public: birth.make_public(),
            };
            durable_producers.push((encrypted_value, producer_index));
            Ok(output)
        }
    }
}

/// Interns a 32-byte constant into the frame pool, reusing an existing entry byte-for-byte.
fn pool_index(pool: &mut Vec<[u8; 32]>, bytes: [u8; 32]) -> Result<u8> {
    if let Some(index) = pool.iter().position(|entry| *entry == bytes) {
        return u8::try_from(index).map_err(|_| EvalBuildError::TooManyPoolEntries);
    }
    let index = u8::try_from(pool.len()).map_err(|_| EvalBuildError::TooManyPoolEntries)?;
    pool.push(bytes);
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
