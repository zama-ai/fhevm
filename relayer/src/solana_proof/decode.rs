//! Decodes zama-host `EncryptedValue` instructions from raw compiled instruction
//! data (Anchor discriminator + borsh args), independent of transaction/RPC
//! shape so it can be unit-tested against synthetic data.

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use zama_host::state::{FheEvalArgs, FheEvalOutput, FheEvalStep};

/// A single compiled instruction, already resolved to full account pubkeys
/// (32-byte, no base58). Produced from either a transaction's top-level
/// `message.instructions` or a `meta.innerInstructions` entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

/// One subject grant as carried in `create_encrypted_value`/`allow_subjects` args.
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubjectGrant {
    pub subject: [u8; 32],
}

/// The zama-host `EncryptedValue` instruction, decoded from one compiled instruction.
///
/// Direct create/allow/update/make-public instructions carry `encrypted_value`
/// at account index 2. `remove_subject` uses index 1, and `fhe_eval` durable
/// outputs reference `remaining_accounts` by index inside the eval plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodedInstruction {
    CreateEncryptedValue {
        encrypted_value: [u8; 32],
        handle: [u8; 32],
        subjects: Vec<SubjectGrant>,
    },
    AllowSubjects {
        encrypted_value: [u8; 32],
        subjects: Vec<SubjectGrant>,
    },
    UpdateEncryptedValue {
        encrypted_value: [u8; 32],
        new_handle: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
    },
    RemoveSubject {
        encrypted_value: [u8; 32],
        subject: [u8; 32],
    },
    FheEvalCreateEncryptedValue {
        encrypted_value: [u8; 32],
        subjects: Vec<SubjectGrant>,
    },
    FheEvalUpdateEncryptedValue {
        encrypted_value: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
        output_subjects: Vec<[u8; 32]>,
    },
    MakeHandlePublic {
        encrypted_value: [u8; 32],
        handle: [u8; 32],
    },
}

impl DecodedInstruction {
    pub fn encrypted_value(&self) -> [u8; 32] {
        match self {
            DecodedInstruction::CreateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::AllowSubjects {
                encrypted_value, ..
            }
            | DecodedInstruction::UpdateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::RemoveSubject {
                encrypted_value, ..
            }
            | DecodedInstruction::FheEvalCreateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::FheEvalUpdateEncryptedValue {
                encrypted_value, ..
            }
            | DecodedInstruction::MakeHandlePublic {
                encrypted_value, ..
            } => *encrypted_value,
        }
    }
}

/// Anchor-style 8-byte global instruction discriminator: `sha256("global:<name>")[..8]`.
fn discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("global:{name}").as_bytes());
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;
const REMOVE_SUBJECT_ACCOUNT_INDEX: usize = 1;
const FHE_EVAL_REMAINING_BASE: usize = 7;
const FHE_EVAL_REMAINING_BASE_WITH_DENY_RECORD: usize = 8;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    #[error("instruction data shorter than the 8-byte discriminator")]
    DataTooShort,
    #[error("unrecognized discriminator (not a zama-host EncryptedValue instruction)")]
    UnknownDiscriminator,
    #[error("missing account at index {0} (encrypted_value)")]
    MissingAccount(usize),
    #[error("missing fhe_eval durable output account at remaining index {remaining_index} (absolute account index {absolute_index})")]
    MissingFheEvalOutputAccount {
        remaining_index: u16,
        absolute_index: usize,
    },
    #[error("fhe_eval durable output has mismatched previous_handle/previous_subjects options")]
    InvalidFheEvalPreviousState,
    #[error("borsh decode failed: {0}")]
    Borsh(String),
}

/// Decodes a `RawInstruction` known to target the zama-host program id.
/// Returns an empty vector for zama-host instructions this module does not care
/// about, and `Err` only for malformed data.
///
/// Most lifecycle instructions produce one decoded instruction. `fhe_eval` can
/// produce several because one eval frame can bind several durable outputs.
pub fn decode_instructions(ix: &RawInstruction) -> Result<Vec<DecodedInstruction>, DecodeError> {
    if ix.data.len() < 8 {
        return Err(DecodeError::DataTooShort);
    }
    let (disc, mut body) = ix.data.split_at(8);

    if disc == discriminator("create_encrypted_value") {
        let acl_domain_key = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let app_account = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let _ = (acl_domain_key, app_account);
        let _encrypted_value_label = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let subjects = <Vec<SubjectGrant>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(vec![DecodedInstruction::CreateEncryptedValue {
            encrypted_value: account_at(ix, ENCRYPTED_VALUE_ACCOUNT_INDEX)?,
            handle,
            subjects,
        }])
    } else if disc == discriminator("allow_subjects") {
        let subjects = <Vec<SubjectGrant>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(vec![DecodedInstruction::AllowSubjects {
            encrypted_value: account_at(ix, ENCRYPTED_VALUE_ACCOUNT_INDEX)?,
            subjects,
        }])
    } else if disc == discriminator("update_encrypted_value") {
        let new_handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let previous_handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        let previous_subjects = <Vec<[u8; 32]>>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(vec![DecodedInstruction::UpdateEncryptedValue {
            encrypted_value: account_at(ix, ENCRYPTED_VALUE_ACCOUNT_INDEX)?,
            new_handle,
            previous_handle,
            previous_subjects,
        }])
    } else if disc == discriminator("remove_subject") {
        let subject = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(vec![DecodedInstruction::RemoveSubject {
            encrypted_value: account_at(ix, REMOVE_SUBJECT_ACCOUNT_INDEX)?,
            subject,
        }])
    } else if disc == discriminator("make_handle_public") {
        let handle = <[u8; 32]>::deserialize(&mut body).map_err(borsh_err)?;
        Ok(vec![DecodedInstruction::MakeHandlePublic {
            encrypted_value: account_at(ix, ENCRYPTED_VALUE_ACCOUNT_INDEX)?,
            handle,
        }])
    } else if disc == discriminator("fhe_eval") {
        let plan = FheEvalArgs::deserialize(&mut body).map_err(borsh_err)?;
        decode_fhe_eval_durable_outputs(ix, &plan)
    } else {
        Ok(Vec::new())
    }
}

/// Back-compat helper for tests and single-output callers. Transaction replay
/// uses [`decode_instructions`] so multi-output `fhe_eval` frames are preserved.
pub fn decode_instruction(ix: &RawInstruction) -> Result<Option<DecodedInstruction>, DecodeError> {
    Ok(decode_instructions(ix)?.into_iter().next())
}

fn borsh_err(e: std::io::Error) -> DecodeError {
    DecodeError::Borsh(e.to_string())
}

fn account_at(ix: &RawInstruction, index: usize) -> Result<[u8; 32], DecodeError> {
    ix.accounts
        .get(index)
        .copied()
        .ok_or(DecodeError::MissingAccount(index))
}

fn fhe_eval_durable_output_account(
    ix: &RawInstruction,
    remaining_index: u16,
) -> Result<[u8; 32], DecodeError> {
    // In the Anchor `#[event_cpi]` account list the fixed `fhe_eval` accounts
    // are followed by event_authority + program, then remaining_accounts. When
    // the optional deny record is present, the host program id shifts from index
    // 6 to 7; that is enough to choose the same base as the host listener.
    let base = if ix.accounts.get(7) == Some(&ix.program_id) {
        FHE_EVAL_REMAINING_BASE_WITH_DENY_RECORD
    } else {
        FHE_EVAL_REMAINING_BASE
    };
    let absolute_index = base + usize::from(remaining_index);
    ix.accounts
        .get(absolute_index)
        .copied()
        .ok_or(DecodeError::MissingFheEvalOutputAccount {
            remaining_index,
            absolute_index,
        })
}

fn decode_fhe_eval_durable_outputs(
    ix: &RawInstruction,
    plan: &FheEvalArgs,
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let mut out = Vec::new();
    for step in &plan.steps {
        let FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_subjects,
            previous_handle,
            previous_subjects,
            ..
        } = fhe_eval_step_output(step)
        else {
            continue;
        };
        let encrypted_value = fhe_eval_durable_output_account(ix, *output_encrypted_value_index)?;
        let output_subjects = output_subjects
            .iter()
            .map(|subject| subject.pubkey.to_bytes())
            .collect::<Vec<_>>();
        match (previous_handle, previous_subjects) {
            (None, None) => out.push(DecodedInstruction::FheEvalCreateEncryptedValue {
                encrypted_value,
                subjects: output_subjects
                    .iter()
                    .copied()
                    .map(|subject| SubjectGrant { subject })
                    .collect(),
            }),
            (Some(previous_handle), Some(previous_subjects)) => {
                out.push(DecodedInstruction::FheEvalUpdateEncryptedValue {
                    encrypted_value,
                    previous_handle: *previous_handle,
                    previous_subjects: previous_subjects
                        .iter()
                        .map(|subject| subject.to_bytes())
                        .collect(),
                    output_subjects,
                });
            }
            _ => return Err(DecodeError::InvalidFheEvalPreviousState),
        }
    }
    Ok(out)
}

fn fhe_eval_step_output(step: &FheEvalStep) -> &FheEvalOutput {
    match step {
        FheEvalStep::Binary { output, .. }
        | FheEvalStep::Ternary { output, .. }
        | FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::RandBounded { output, .. } => output,
    }
}

/// Decodes every instruction in a transaction targeting `program_id`, in the
/// order supplied (caller is responsible for top-level/inner interleaving —
/// see `chain::flatten_execution_order`). Ignores instructions for other programs.
pub fn decode_program_instructions(
    program_id: [u8; 32],
    instructions: &[RawInstruction],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let mut out = Vec::new();
    for ix in instructions {
        if ix.program_id != program_id {
            continue;
        }
        out.extend(decode_instructions(ix)?);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use anchor_lang::AnchorSerialize;
    use borsh::BorshSerialize;
    use zama_host::state::{AclSubjectEntry, FheEvalOutput, FheEvalStep};

    fn program_id() -> [u8; 32] {
        [7u8; 32]
    }

    fn ix_with_data(
        accounts: Vec<[u8; 32]>,
        name: &str,
        args: impl BorshSerialize,
    ) -> RawInstruction {
        let mut data = discriminator(name).to_vec();
        args.serialize(&mut data).unwrap();
        RawInstruction {
            program_id: program_id(),
            accounts,
            data,
        }
    }

    fn ix_with_anchor_data(
        accounts: Vec<[u8; 32]>,
        name: &str,
        args: impl AnchorSerialize,
    ) -> RawInstruction {
        let mut data = discriminator(name).to_vec();
        args.serialize(&mut data).unwrap();
        RawInstruction {
            program_id: program_id(),
            accounts,
            data,
        }
    }

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn pubkey(tag: u8) -> Pubkey {
        Pubkey::new_from_array(pk(tag))
    }

    fn fhe_eval_accounts(remaining: &[[u8; 32]]) -> Vec<[u8; 32]> {
        let mut accounts = vec![
            pk(0xA0),
            pk(0xA1),
            pk(0xA2),
            pk(0xA3),
            pk(0xA4),
            pk(0xA5),
            program_id(),
        ];
        accounts.extend_from_slice(remaining);
        accounts
    }

    fn durable_output(
        output_encrypted_value_index: u16,
        subject_tags: &[u8],
        previous_handle: Option<[u8; 32]>,
        previous_subject_tags: Option<&[u8]>,
    ) -> FheEvalOutput {
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index: None,
            output_acl_domain_key: pubkey(0x40),
            output_app_account: pubkey(0x41),
            output_encrypted_value_label: pk(0x42),
            output_subjects: subject_tags
                .iter()
                .map(|tag| AclSubjectEntry {
                    pubkey: pubkey(*tag),
                })
                .collect(),
            previous_handle,
            previous_subjects: previous_subject_tags
                .map(|subjects| subjects.iter().map(|tag| pubkey(*tag)).collect()),
        }
    }

    #[test]
    fn decodes_create_encrypted_value() {
        let ev = pk(1);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        #[derive(BorshSerialize)]
        struct Args {
            acl_domain_key: [u8; 32],
            app_account: [u8; 32],
            label: [u8; 32],
            handle: [u8; 32],
            subjects: Vec<SubjectGrant>,
        }
        let args = Args {
            acl_domain_key: pk(0x10),
            app_account: pk(0x11),
            label: pk(0x12),
            handle: pk(0x20),
            subjects: vec![SubjectGrant { subject: pk(0x30) }],
        };
        let ix = ix_with_data(accounts, "create_encrypted_value", args);
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::CreateEncryptedValue {
                encrypted_value: ev,
                handle: pk(0x20),
                subjects: vec![SubjectGrant { subject: pk(0x30) }],
            }
        );
    }

    #[test]
    fn decodes_update_encrypted_value() {
        let ev = pk(2);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        #[derive(BorshSerialize)]
        struct Args {
            new_handle: [u8; 32],
            previous_handle: [u8; 32],
            previous_subjects: Vec<[u8; 32]>,
        }
        let args = Args {
            new_handle: pk(0x21),
            previous_handle: pk(0x20),
            previous_subjects: vec![pk(0x30), pk(0x31)],
        };
        let ix = ix_with_data(accounts, "update_encrypted_value", args);
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::UpdateEncryptedValue {
                encrypted_value: ev,
                new_handle: pk(0x21),
                previous_handle: pk(0x20),
                previous_subjects: vec![pk(0x30), pk(0x31)],
            }
        );
    }

    #[test]
    fn decodes_make_handle_public_with_handle_arg() {
        let ev = pk(3);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        let ix = ix_with_data(accounts, "make_handle_public", pk(0x20));
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::MakeHandlePublic {
                encrypted_value: ev,
                handle: pk(0x20),
            }
        );
    }

    #[test]
    fn decodes_remove_subject_with_encrypted_value_at_index_1() {
        let ev = pk(4);
        let accounts = vec![pk(0xA), ev, pk(0xC), pk(0xD)];
        let ix = ix_with_data(accounts, "remove_subject", pk(0x30));
        let decoded = decode_instruction(&ix).unwrap().unwrap();
        assert_eq!(
            decoded,
            DecodedInstruction::RemoveSubject {
                encrypted_value: ev,
                subject: pk(0x30),
            }
        );
    }

    #[test]
    fn decodes_fhe_eval_durable_outputs_in_step_order() {
        let ev0 = pk(0xE0);
        let ev1 = pk(0xE1);
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: pk(0x10),
                    fhe_type: 5,
                    output: durable_output(0, &[0x30], None, None),
                },
                FheEvalStep::Rand {
                    fhe_type: 5,
                    output: durable_output(1, &[0x31, 0x32], Some(pk(0x20)), Some(&[0x31, 0x32])),
                },
            ],
        };
        let ix = ix_with_anchor_data(fhe_eval_accounts(&[ev0, ev1]), "fhe_eval", plan);
        let decoded = decode_program_instructions(program_id(), &[ix]).unwrap();
        assert_eq!(
            decoded,
            vec![
                DecodedInstruction::FheEvalCreateEncryptedValue {
                    encrypted_value: ev0,
                    subjects: vec![SubjectGrant { subject: pk(0x30) }],
                },
                DecodedInstruction::FheEvalUpdateEncryptedValue {
                    encrypted_value: ev1,
                    previous_handle: pk(0x20),
                    previous_subjects: vec![pk(0x31), pk(0x32)],
                    output_subjects: vec![pk(0x31), pk(0x32)],
                },
            ]
        );
    }

    #[test]
    fn ignores_unknown_discriminator() {
        let mut data = [9u8; 8].to_vec();
        data.extend_from_slice(&[1, 2, 3]);
        let ix = RawInstruction {
            program_id: program_id(),
            accounts: vec![pk(0), pk(0), pk(0)],
            data,
        };
        assert_eq!(decode_instruction(&ix).unwrap(), None);
    }

    #[test]
    fn other_program_instructions_are_skipped_by_the_program_filter() {
        let ev = pk(4);
        let accounts = vec![pk(0xA), pk(0xB), ev, pk(0xC), pk(0xD)];
        let mut ix = ix_with_data(accounts, "make_handle_public", pk(0x20));
        ix.program_id = pk(0xFF); // a different (CPI caller) program
        let decoded = decode_program_instructions(program_id(), &[ix]).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn rejects_data_shorter_than_discriminator() {
        let ix = RawInstruction {
            program_id: program_id(),
            accounts: vec![],
            data: vec![1, 2, 3],
        };
        assert_eq!(decode_instruction(&ix), Err(DecodeError::DataTooShort));
    }
}
