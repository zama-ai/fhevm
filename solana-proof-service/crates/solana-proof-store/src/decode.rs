//! Decodes zama-host `EncryptedValue` instructions from raw compiled instruction
//! data (Anchor discriminator + borsh args), independent of transaction/RPC
//! shape so it can be unit-tested against synthetic data.
//!
//! The instruction/event names matched below are the ingest allowlist. CI keeps
//! that catalog partitioned against the vendored host IDL via
//! `solana/scripts/check_proof_store_idl.py` (decoded ∪ ignored = all host
//! instructions; required lifecycle events must stay wired).
//!
//! One exception needs sibling context: a born-public (`make_public=true`)
//! `fhe_eval` durable output commits a public-decrypt leaf to the eval OUTPUT
//! handle, which is derived on-chain from slot entropy and appears in no
//! instruction arg. The host therefore emits one narrow lifecycle batch from a
//! self-CPI. [`decode_program_instructions`] accepts it only when it exactly
//! matches the enclosing successful `fhe_eval`; the confirmed MMR peak check
//! remains the defense-in-depth authority at proof time (DD-035).
//!
//! Relayers must support a lifecycle version before that producer version is
//! deployed; an unknown version intentionally halts ingestion. RPC responses
//! must also include `stackHeight` for every inner instruction so the batch can
//! be bound to its immediate enclosing frame.

use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use zama_host::state::{FheEvalArgs, FheEvalOutput, FheEvalStep};

pub use solana_proof_source::RawInstruction;

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
        /// `Some(output_handle)` when the durable output was born public
        /// (`make_public=true`) AND its output handle was resolved from the
        /// lifecycle batch this transaction emitted. `None` otherwise — the output
        /// handle is derived on-chain from slot entropy and appears in no
        /// instruction arg, so without the event it stays unresolved and any
        /// born-public leaf fails closed at proof time rather than serving a wrong result.
        make_public_handle: Option<[u8; 32]>,
    },
    FheEvalUpdateEncryptedValue {
        encrypted_value: [u8; 32],
        previous_handle: [u8; 32],
        previous_subjects: Vec<[u8; 32]>,
        output_subjects: Vec<[u8; 32]>,
        /// See [`DecodedInstruction::FheEvalCreateEncryptedValue::make_public_handle`].
        make_public_handle: Option<[u8; 32]>,
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

/// Anchor-style 8-byte event discriminator: `sha256("event:<name>")[..8]`.
fn event_discriminator(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("event:{name}").as_bytes());
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

/// Anchor's `emit_cpi!` self-invocation sentinel prefixing the event bytes in an
/// inner instruction (little-endian of `0x1d9acb512ea545e4`). Mirrors the
/// host-listener's `ANCHOR_EVENT_IX_TAG_LE`.
const ANCHOR_EVENT_IX_TAG_LE: [u8; 8] = 0x1d9acb512ea545e4_u64.to_le_bytes();

#[derive(Clone, Debug, PartialEq, Eq)]
struct BornPublicOutput {
    step_index: u16,
    encrypted_value: [u8; 32],
    output_handle: [u8; 32],
}

const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;
const REMOVE_SUBJECT_ACCOUNT_INDEX: usize = 1;
/// `remaining_accounts` follow the 10 named `fhe_eval` accounts — payer,
/// compute_subject, app_account_authority, host_config, system_program,
/// hcu_authority, hcu_block_meter, hcu_trusted_app_record, then `#[event_cpi]`'s
/// event_authority + program (see `FheEval` in fhe_eval.rs). The two optional HCU
/// accounts are always present as program-id placeholders when `None`, so the base
/// is fixed. Must stay in lockstep with the host-listener's `FHE_EVAL_REMAINING_BASE`.
const FHE_EVAL_REMAINING_BASE: usize = 10;

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
    #[error("missing or invalid Solana instruction stack metadata")]
    InvalidStackMetadata,
    #[error("born-public lifecycle event is missing for fhe_eval")]
    MissingBornPublicEvent,
    #[error("unexpected or unconsumed born-public lifecycle event")]
    UnexpectedBornPublicEvent,
    #[error("unexpected zama-host descendant inside fhe_eval execution frame")]
    UnexpectedHostDescendant,
    #[error("born-public lifecycle event has an invalid host self-CPI envelope")]
    InvalidBornPublicEnvelope,
    #[error("born-public lifecycle event has unsupported version {0}")]
    UnsupportedBornPublicVersion(u8),
    #[error("born-public lifecycle event is malformed: {0}")]
    MalformedBornPublicEvent(String),
    #[error("born-public lifecycle event does not exactly match fhe_eval outputs")]
    BornPublicMismatch,
    #[error("born-public lifecycle event contains duplicate accounts or handles")]
    DuplicateBornPublicOutput,
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
        // Without the lifecycle batch (single-instruction decode), no born-public
        // output handle can be resolved; those leaves fail closed at proof time.
        decode_fhe_eval_instruction(ix, &[])
    } else {
        Ok(Vec::new())
    }
}

fn decode_fhe_eval_instruction(
    ix: &RawInstruction,
    born_public_handles: &[Option<[u8; 32]>],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let (disc, mut body) = ix.data.split_at(8);
    debug_assert_eq!(disc, discriminator("fhe_eval"));
    let plan = FheEvalArgs::deserialize(&mut body).map_err(borsh_err)?;
    decode_fhe_eval_durable_outputs(ix, &plan, born_public_handles)
}

/// Back-compat helper for tests and single-output callers. Transaction replay
/// uses [`decode_program_instructions`] so multi-output `fhe_eval` frames and
/// their lifecycle batches are validated together.
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
    // `remaining_index` is the plan's `output_encrypted_value_index`, relative to
    // `remaining_accounts`. Those always start at a fixed offset past the 10 named
    // `fhe_eval` accounts (the optional HCU accounts are program-id placeholders
    // when absent, so the offset never shifts). Any deny record lives inside
    // `remaining_accounts` and is already accounted for by the plan index.
    let absolute_index = FHE_EVAL_REMAINING_BASE + usize::from(remaining_index);
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
    born_public_handles: &[Option<[u8; 32]>],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let mut out = Vec::new();
    for (step_index, step) in plan.steps.iter().enumerate() {
        let FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_subjects,
            previous_handle,
            previous_subjects,
            make_public,
            ..
        } = fhe_eval_step_output(step)
        else {
            continue;
        };
        let encrypted_value = fhe_eval_durable_output_account(ix, *output_encrypted_value_index)?;
        let make_public_handle = if *make_public {
            born_public_handles.get(step_index).copied().flatten()
        } else {
            None
        };
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
                make_public_handle,
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
                    make_public_handle,
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
        | FheEvalStep::Unary { output, .. }
        | FheEvalStep::RandBounded { output, .. }
        | FheEvalStep::Sum { output, .. }
        | FheEvalStep::IsIn { output, .. }
        | FheEvalStep::MulDiv { output, .. } => output,
    }
}

fn is_born_public_event(ix: &RawInstruction, program_id: [u8; 32]) -> bool {
    ix.program_id == program_id
        && ix.data.starts_with(&ANCHOR_EVENT_IX_TAG_LE)
        && ix.data.get(8..16) == Some(event_discriminator("PublicOutputsProducedEvent").as_slice())
}

fn decode_born_public_event(
    ix: &RawInstruction,
    program_id: [u8; 32],
) -> Result<Vec<BornPublicOutput>, DecodeError> {
    if ix.program_id != program_id
        || ix.accounts.as_slice() != [zama_host::EVENT_AUTHORITY_AND_BUMP.0.to_bytes()]
    {
        return Err(DecodeError::InvalidBornPublicEnvelope);
    }
    // RPC compiled inner instructions do not expose CPI AccountMeta flags.
    // Because failed transactions are discarded in `chain`, successful Anchor
    // dispatch with this exact PDA proves the host supplied its signer seeds;
    // #3136 separately pins the on-chain meta as signed and readonly.
    let mut body = ix
        .data
        .strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)
        .and_then(|data| data.strip_prefix(&event_discriminator("PublicOutputsProducedEvent")))
        .ok_or(DecodeError::InvalidBornPublicEnvelope)?;
    let version = u8::deserialize(&mut body)
        .map_err(|e| DecodeError::MalformedBornPublicEvent(e.to_string()))?;
    if version != zama_host::PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION {
        return Err(DecodeError::UnsupportedBornPublicVersion(version));
    }
    const RECORD_BYTES: usize = 66;
    let count_bytes: [u8; 4] = body
        .get(..4)
        .ok_or_else(|| DecodeError::MalformedBornPublicEvent("missing record count".to_string()))?
        .try_into()
        .expect("record count slice has fixed length");
    let count = u32::from_le_bytes(count_bytes) as usize;
    let expected_len = count
        .checked_mul(RECORD_BYTES)
        .and_then(|records| records.checked_add(4))
        .ok_or_else(|| {
            DecodeError::MalformedBornPublicEvent(
                "record count overflows encoded length".to_string(),
            )
        })?;
    // The producer emits no event for an empty batch, so an encoded batch must
    // contain at least one record.
    if !(1..=zama_host::MAX_FHE_EVAL_OPS).contains(&count) || body.len() != expected_len {
        return Err(DecodeError::MalformedBornPublicEvent(format!(
            "invalid record count or encoded length: count={count}, bytes={}",
            body.len()
        )));
    }
    let outputs = <Vec<zama_host::events::ProducedPublicOutput>>::deserialize(&mut body)
        .map_err(|e| DecodeError::MalformedBornPublicEvent(e.to_string()))?;
    if !body.is_empty() {
        return Err(DecodeError::MalformedBornPublicEvent(
            "trailing-byte payload".to_string(),
        ));
    }
    Ok(outputs
        .into_iter()
        .map(|output| BornPublicOutput {
            step_index: output.step_index,
            encrypted_value: output.encrypted_value.to_bytes(),
            output_handle: output.output_handle,
        })
        .collect())
}

fn expected_born_public_outputs(
    ix: &RawInstruction,
    plan: &FheEvalArgs,
) -> Result<Vec<(u16, [u8; 32])>, DecodeError> {
    plan.steps
        .iter()
        .enumerate()
        .filter_map(|(step_index, step)| {
            let FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                make_public: true,
                ..
            } = fhe_eval_step_output(step)
            else {
                return None;
            };
            Some(
                fhe_eval_durable_output_account(ix, *output_encrypted_value_index)
                    .map(|account| (step_index as u16, account)),
            )
        })
        .collect()
}

fn validate_born_public_event(
    expected: &[(u16, [u8; 32])],
    actual: Vec<BornPublicOutput>,
    step_count: usize,
) -> Result<Vec<Option<[u8; 32]>>, DecodeError> {
    let mut accounts = std::collections::HashSet::with_capacity(actual.len());
    let mut handles = std::collections::HashSet::with_capacity(actual.len());
    if actual.iter().any(|record| {
        !accounts.insert(record.encrypted_value) || !handles.insert(record.output_handle)
    }) {
        return Err(DecodeError::DuplicateBornPublicOutput);
    }
    if actual.len() != expected.len()
        || actual
            .iter()
            .zip(expected)
            .any(|(record, (step_index, account))| {
                record.step_index != *step_index || record.encrypted_value != *account
            })
    {
        return Err(DecodeError::BornPublicMismatch);
    }
    let mut handles_by_step = vec![None; step_count];
    for record in actual {
        let slot = handles_by_step
            .get_mut(usize::from(record.step_index))
            .ok_or(DecodeError::BornPublicMismatch)?;
        *slot = Some(record.output_handle);
    }
    Ok(handles_by_step)
}

/// Decodes every instruction in a transaction targeting `program_id`, in the
/// order supplied (caller is responsible for top-level/inner interleaving —
/// see `chain::flatten_execution_order`). Ignores instructions for other programs.
pub fn decode_program_instructions(
    program_id: [u8; 32],
    instructions: &[RawInstruction],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    if instructions
        .iter()
        .any(|ix| matches!(ix.stack_height, None | Some(0)))
    {
        return Err(DecodeError::InvalidStackMetadata);
    }
    let mut out = Vec::new();
    let mut index = 0;
    while index < instructions.len() {
        let ix = &instructions[index];
        if is_born_public_event(ix, program_id) {
            return Err(DecodeError::UnexpectedBornPublicEvent);
        }
        if ix.program_id != program_id {
            index += 1;
            continue;
        }
        if is_fhe_eval(ix) {
            let mut body = &ix.data[8..];
            let plan = FheEvalArgs::deserialize(&mut body).map_err(borsh_err)?;
            let expected = expected_born_public_outputs(ix, &plan)?;
            let eval_height = ix.stack_height.ok_or(DecodeError::InvalidStackMetadata)?;
            let event_height = eval_height
                .checked_add(1)
                .ok_or(DecodeError::InvalidStackMetadata)?;
            let mut frame_end = index + 1;
            while let Some(child) = instructions.get(frame_end) {
                let child_height = child
                    .stack_height
                    .ok_or(DecodeError::InvalidStackMetadata)?;
                if child.top_level_index != ix.top_level_index || child_height <= eval_height {
                    break;
                }
                frame_end += 1;
            }
            let event_indexes = (index + 1..frame_end)
                .filter(|&child| is_born_public_event(&instructions[child], program_id))
                .collect::<Vec<_>>();
            if instructions[index + 1..frame_end].iter().any(|child| {
                child.program_id == program_id && !is_born_public_event(child, program_id)
            }) {
                return Err(DecodeError::UnexpectedHostDescendant);
            }
            let handles = match (expected.is_empty(), event_indexes.as_slice()) {
                (true, []) => vec![None; plan.steps.len()],
                (true, _) => return Err(DecodeError::UnexpectedBornPublicEvent),
                (false, []) => return Err(DecodeError::MissingBornPublicEvent),
                (false, [event_index]) => {
                    let event_ix = &instructions[*event_index];
                    if event_ix.stack_height != Some(event_height) {
                        return Err(DecodeError::InvalidBornPublicEnvelope);
                    }
                    let actual = decode_born_public_event(event_ix, program_id)?;
                    validate_born_public_event(&expected, actual, plan.steps.len())?
                }
                (false, _) => return Err(DecodeError::UnexpectedBornPublicEvent),
            };
            out.extend(decode_fhe_eval_durable_outputs(ix, &plan, &handles)?);
            index = frame_end;
        } else {
            out.extend(decode_instructions(ix)?);
            index += 1;
        }
    }
    Ok(out)
}

fn is_fhe_eval(ix: &RawInstruction) -> bool {
    ix.data.len() >= 8 && ix.data[..8] == discriminator("fhe_eval")
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
            top_level_index: 0,
            stack_height: Some(1),
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
            top_level_index: 0,
            stack_height: Some(1),
        }
    }

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn pubkey(tag: u8) -> Pubkey {
        Pubkey::new_from_array(pk(tag))
    }

    /// The 10 named `fhe_eval` accounts (payer, compute_subject,
    /// app_account_authority, host_config, system_program, hcu_authority,
    /// hcu_block_meter, hcu_trusted_app_record, event_authority, program) followed
    /// by `remaining_accounts` — matching the real anchor account layout so the
    /// durable output resolves at `FHE_EVAL_REMAINING_BASE`.
    fn fhe_eval_accounts(remaining: &[[u8; 32]]) -> Vec<[u8; 32]> {
        let mut accounts = vec![
            pk(0xA0),     // 0 payer
            pk(0xA1),     // 1 compute_subject
            pk(0xA2),     // 2 app_account_authority
            pk(0xA3),     // 3 host_config
            pk(0xA4),     // 4 system_program
            pk(0xA5),     // 5 hcu_authority
            program_id(), // 6 hcu_block_meter (None placeholder)
            program_id(), // 7 hcu_trusted_app_record (None placeholder)
            pk(0xA8),     // 8 event_authority
            program_id(), // 9 program (event_cpi)
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
            make_public: false,
        }
    }

    fn make_public_durable_output(
        output_encrypted_value_index: u16,
        subject_tags: &[u8],
        previous_handle: Option<[u8; 32]>,
        previous_subject_tags: Option<&[u8]>,
    ) -> FheEvalOutput {
        let mut output = durable_output(
            output_encrypted_value_index,
            subject_tags,
            previous_handle,
            previous_subject_tags,
        );
        if let FheEvalOutput::AllowedDurable { make_public, .. } = &mut output {
            *make_public = true;
        }
        output
    }

    fn born_public_event_ix(records: &[(u16, [u8; 32], [u8; 32])]) -> RawInstruction {
        let mut data = ANCHOR_EVENT_IX_TAG_LE.to_vec();
        data.extend_from_slice(&event_discriminator("PublicOutputsProducedEvent"));
        data.push(zama_host::PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION);
        let records = records
            .iter()
            .map(|(step_index, encrypted_value, output_handle)| {
                zama_host::events::ProducedPublicOutput {
                    step_index: *step_index,
                    encrypted_value: Pubkey::new_from_array(*encrypted_value),
                    output_handle: *output_handle,
                }
            })
            .collect::<Vec<_>>();
        AnchorSerialize::serialize(&records, &mut data).unwrap();
        RawInstruction {
            program_id: program_id(),
            accounts: vec![zama_host::EVENT_AUTHORITY_AND_BUMP.0.to_bytes()],
            data,
            top_level_index: 0,
            stack_height: Some(2),
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
                    make_public_handle: None,
                },
                DecodedInstruction::FheEvalUpdateEncryptedValue {
                    encrypted_value: ev1,
                    previous_handle: pk(0x20),
                    previous_subjects: vec![pk(0x31), pk(0x32)],
                    output_subjects: vec![pk(0x31), pk(0x32)],
                    make_public_handle: None,
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
            top_level_index: 0,
            stack_height: Some(1),
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
            top_level_index: 0,
            stack_height: Some(1),
        };
        assert_eq!(decode_instruction(&ix), Err(DecodeError::DataTooShort));
    }

    fn two_born_public_outputs() -> (RawInstruction, RawInstruction) {
        let ev0 = pk(0xE0);
        let ev1 = pk(0xE1);
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: pk(0x02),
                    fhe_type: 5,
                    output: make_public_durable_output(0, &[0x30], None, None),
                },
                FheEvalStep::TrivialEncrypt {
                    plaintext: pk(0x03),
                    fhe_type: 5,
                    output: make_public_durable_output(1, &[0x31], Some(pk(0x20)), Some(&[0x31])),
                },
            ],
        };
        let eval = ix_with_anchor_data(fhe_eval_accounts(&[ev0, ev1]), "fhe_eval", plan);
        let event = born_public_event_ix(&[(0, ev0, pk(0x50)), (1, ev1, pk(0x51))]);
        (eval, event)
    }

    #[test]
    fn exact_born_public_batch_resolves_each_durable_output() {
        let (eval, event) = two_born_public_outputs();
        let decoded = decode_program_instructions(program_id(), &[eval, event]).unwrap();
        assert_eq!(
            decoded,
            vec![
                DecodedInstruction::FheEvalCreateEncryptedValue {
                    encrypted_value: pk(0xE0),
                    subjects: vec![SubjectGrant { subject: pk(0x30) }],
                    make_public_handle: Some(pk(0x50)),
                },
                DecodedInstruction::FheEvalUpdateEncryptedValue {
                    encrypted_value: pk(0xE1),
                    previous_handle: pk(0x20),
                    previous_subjects: vec![pk(0x31)],
                    output_subjects: vec![pk(0x31)],
                    make_public_handle: Some(pk(0x51)),
                },
            ]
        );
    }

    #[test]
    fn batches_are_associated_with_their_enclosing_fhe_eval() {
        let (mut first_eval, mut first_event) = two_born_public_outputs();
        first_eval.stack_height = Some(2);
        first_event.stack_height = Some(3);
        let (mut second_eval, mut second_event) = two_born_public_outputs();
        second_eval.stack_height = Some(2);
        second_event.stack_height = Some(3);
        second_event.data =
            born_public_event_ix(&[(0, pk(0xE0), pk(0x60)), (1, pk(0xE1), pk(0x61))]).data;

        let decoded = decode_program_instructions(
            program_id(),
            &[first_eval, first_event, second_eval, second_event],
        )
        .unwrap();

        assert_eq!(decoded.len(), 4);
        assert!(matches!(
            &decoded[2],
            DecodedInstruction::FheEvalCreateEncryptedValue {
                make_public_handle: Some(handle),
                ..
            } if *handle == pk(0x60)
        ));
    }

    #[test]
    fn missing_born_public_batch_fails_closed() {
        let (eval, _) = two_born_public_outputs();
        assert_eq!(
            decode_program_instructions(program_id(), &[eval]),
            Err(DecodeError::MissingBornPublicEvent)
        );
    }

    #[test]
    fn extra_batch_for_non_public_frame_fails_closed() {
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: pk(0x02),
                fhe_type: 5,
                output: durable_output(0, &[0x30], None, None),
            }],
        };
        let eval = ix_with_anchor_data(fhe_eval_accounts(&[pk(0xE0)]), "fhe_eval", plan);
        let event = born_public_event_ix(&[(0, pk(0xE0), pk(0x50))]);
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, event]),
            Err(DecodeError::UnexpectedBornPublicEvent)
        );
    }

    #[test]
    fn reordered_or_mismatched_born_public_batch_fails_closed() {
        let (eval, _) = two_born_public_outputs();
        let reordered = born_public_event_ix(&[(1, pk(0xE1), pk(0x51)), (0, pk(0xE0), pk(0x50))]);
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, reordered]),
            Err(DecodeError::BornPublicMismatch)
        );
    }

    #[test]
    fn duplicate_accounts_and_handles_fail_closed() {
        let (eval, _) = two_born_public_outputs();
        let duplicate_account =
            born_public_event_ix(&[(0, pk(0xE0), pk(0x50)), (1, pk(0xE0), pk(0x51))]);
        assert_eq!(
            decode_program_instructions(program_id(), &[eval.clone(), duplicate_account],),
            Err(DecodeError::DuplicateBornPublicOutput)
        );
        let duplicate_handle =
            born_public_event_ix(&[(0, pk(0xE0), pk(0x50)), (1, pk(0xE1), pk(0x50))]);
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, duplicate_handle]),
            Err(DecodeError::DuplicateBornPublicOutput)
        );
    }

    #[test]
    fn malformed_unknown_version_and_wrong_envelope_fail_closed() {
        let (eval, mut malformed) = two_born_public_outputs();
        malformed.data.push(0);
        assert!(matches!(
            decode_program_instructions(program_id(), &[eval.clone(), malformed]),
            Err(DecodeError::MalformedBornPublicEvent(_))
        ));

        let (_, mut unknown_version) = two_born_public_outputs();
        unknown_version.data[16] = 99;
        assert_eq!(
            decode_program_instructions(program_id(), &[eval.clone(), unknown_version],),
            Err(DecodeError::UnsupportedBornPublicVersion(99))
        );

        let (_, mut wrong_envelope) = two_born_public_outputs();
        wrong_envelope.accounts = vec![pk(0xEE)];
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, wrong_envelope]),
            Err(DecodeError::InvalidBornPublicEnvelope)
        );
    }

    #[test]
    fn huge_declared_record_count_fails_before_record_decode() {
        let (eval, mut event) = two_born_public_outputs();
        event.data.truncate(21);
        event.data[17..21].copy_from_slice(&u32::MAX.to_le_bytes());

        assert!(matches!(
            decode_program_instructions(program_id(), &[eval, event]),
            Err(DecodeError::MalformedBornPublicEvent(_))
        ));
    }

    #[test]
    fn oversized_batch_fails_closed() {
        let (eval, _) = two_born_public_outputs();
        let records = (0..=zama_host::MAX_FHE_EVAL_OPS)
            .map(|index| (index as u16, [index as u8; 32], [(index + 1) as u8; 32]))
            .collect::<Vec<_>>();
        let oversized = born_public_event_ix(&records);
        assert!(matches!(
            decode_program_instructions(program_id(), &[eval, oversized]),
            Err(DecodeError::MalformedBornPublicEvent(_))
        ));
    }

    #[test]
    fn extra_unconsumed_and_wrongly_nested_batches_fail_closed() {
        let (eval, event) = two_born_public_outputs();
        assert_eq!(
            decode_program_instructions(program_id(), &[event.clone(), eval.clone()]),
            Err(DecodeError::UnexpectedBornPublicEvent)
        );

        let mut wrong_nesting = event;
        wrong_nesting.stack_height = Some(3);
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, wrong_nesting]),
            Err(DecodeError::InvalidBornPublicEnvelope)
        );
    }

    #[test]
    fn unexpected_host_descendant_inside_eval_frame_fails_closed() {
        let (eval, event) = two_born_public_outputs();
        let mut nested_host_instruction = ix_with_data(
            vec![pk(0xA), pk(0xB), pk(0xE0)],
            "make_handle_public",
            pk(0x50),
        );
        nested_host_instruction.stack_height = Some(2);

        assert_eq!(
            decode_program_instructions(program_id(), &[eval, nested_host_instruction, event],),
            Err(DecodeError::UnexpectedHostDescendant)
        );
    }

    #[test]
    fn missing_stack_metadata_fails_closed() {
        let (eval, mut event) = two_born_public_outputs();
        event.stack_height = None;
        assert_eq!(
            decode_program_instructions(program_id(), &[eval, event]),
            Err(DecodeError::InvalidStackMetadata)
        );
    }
}
