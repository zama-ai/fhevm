//! Decodes zama-host `EncryptedValue` instructions from raw compiled instruction
//! data (Anchor discriminator + borsh args), independent of transaction/RPC
//! shape so it can be unit-tested against synthetic data.
//!
//! One exception needs sibling context: a born-public (`make_public=true`)
//! `fhe_eval` durable output commits a public-decrypt leaf to the eval OUTPUT
//! handle, which is derived on-chain from slot entropy and appears in no
//! instruction arg. That handle is recovered from the per-step op event the
//! host emits via `emit_cpi!` (an inner instruction of the same `fhe_eval`), so
//! [`decode_program_instructions`] correlates each `fhe_eval` with the op events
//! that immediately follow it. The resolved handle is untrusted and guarded by
//! the on-chain peak cross-check at proof time (DD-035).

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
        /// `Some(output_handle)` when the durable output was born public
        /// (`make_public=true`) AND its output handle was resolved from the
        /// op event this transaction emitted. `None` otherwise — the output
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

/// The `fhe_eval` op events, one emitted per plan step in step order. Every
/// one carries its verified output handle as the final `result: [u8; 32]` field
/// (see `zama_host::events`), so the handle is the last 32 bytes of the payload.
const OP_EVENT_NAMES: [&str; 9] = [
    "FheBinaryOpEvent",
    "FheTernaryOpEvent",
    "TrivialEncryptEvent",
    "FheRandEvent",
    "FheRandBoundedEvent",
    "FheUnaryOpEvent",
    "FheSumEvent",
    "FheIsInEvent",
    "FheMulDivEvent",
];

/// Extracts the verified output handle (`result`) from an `emit_cpi!` op-event
/// inner instruction (self-CPI to the host program). Returns `None` for any
/// instruction that is not a recognized op-event CPI. The handle is UNTRUSTED:
/// callers rely on the on-chain peak cross-check (DD-035) to fail closed if a
/// resolved handle is wrong — it is never trusted to authorize on its own.
pub fn op_event_result(ix: &RawInstruction, program_id: [u8; 32]) -> Option<[u8; 32]> {
    if ix.program_id != program_id {
        return None;
    }
    let after_tag = ix.data.strip_prefix(&ANCHOR_EVENT_IX_TAG_LE)?;
    if after_tag.len() < 8 + 32 {
        return None;
    }
    let disc: [u8; 8] = after_tag[..8].try_into().ok()?;
    if !OP_EVENT_NAMES
        .iter()
        .any(|name| event_discriminator(name) == disc)
    {
        return None;
    }
    // `result` is the last field of every op event; borsh has no trailing bytes.
    after_tag[after_tag.len() - 32..].try_into().ok()
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
        // Without the op events (single-instruction decode), no born-public
        // output handle can be resolved; those leaves fail closed at proof time.
        decode_fhe_eval_instruction(ix, &[])
    } else {
        Ok(Vec::new())
    }
}

/// Decodes one `fhe_eval` instruction's durable outputs, resolving each
/// born-public output's handle from `op_event_results` (the transaction's
/// op events, in step/emission order). Pass an empty slice when the events are
/// unavailable.
fn decode_fhe_eval_instruction(
    ix: &RawInstruction,
    op_event_results: &[[u8; 32]],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let (disc, mut body) = ix.data.split_at(8);
    debug_assert_eq!(disc, discriminator("fhe_eval"));
    let plan = FheEvalArgs::deserialize(&mut body).map_err(borsh_err)?;
    decode_fhe_eval_durable_outputs(ix, &plan, op_event_results)
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
    op_event_results: &[[u8; 32]],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    // Correlation contract: the host emits exactly one op event per plan step,
    // in step order (see `fhe_eval/walk.rs`), so the durable output produced by
    // step `i` binds `op_event_results[i]`. Only trust the events when their
    // count matches the plan exactly; a mismatch means the events for this
    // instruction were not captured whole (e.g. log-transported large frames we
    // do not yet read), so resolve no handles and let born-public leaves fail
    // closed rather than risk a wrong correlation. A wrong-but-well-formed handle
    // is still caught by the on-chain peak cross-check at proof time (DD-035).
    let output_handles = (op_event_results.len() == plan.steps.len()).then_some(op_event_results);
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
            output_handles.map(|handles| handles[step_index])
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

/// Decodes every instruction in a transaction targeting `program_id`, in the
/// order supplied (caller is responsible for top-level/inner interleaving —
/// see `chain::flatten_execution_order`). Ignores instructions for other programs.
pub fn decode_program_instructions(
    program_id: [u8; 32],
    instructions: &[RawInstruction],
) -> Result<Vec<DecodedInstruction>, DecodeError> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < instructions.len() {
        let ix = &instructions[index];
        if ix.program_id != program_id {
            index += 1;
            continue;
        }
        if is_fhe_eval(ix) {
            // `fhe_eval`'s `emit_cpi!` op events are its own inner instructions,
            // which `chain::flatten_execution_order` places immediately after it
            // in execution order. Collect that contiguous run to resolve
            // born-public output handles for this frame.
            let mut op_event_results = Vec::new();
            let mut next = index + 1;
            while let Some(result) = instructions
                .get(next)
                .and_then(|ev| op_event_result(ev, program_id))
            {
                op_event_results.push(result);
                next += 1;
            }
            out.extend(decode_fhe_eval_instruction(ix, &op_event_results)?);
            index = next;
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
    use zama_host::state::{AclSubjectEntry, FheEvalOperand, FheEvalOutput, FheEvalStep};

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

    /// Builds the `emit_cpi!` op-event inner instruction the host emits per step,
    /// carrying the verified output `result` handle (here a `TrivialEncryptEvent`).
    fn op_event_ix(result: [u8; 32]) -> RawInstruction {
        op_event_ix_named("TrivialEncryptEvent", result)
    }

    fn op_event_ix_named(event_name: &str, result: [u8; 32]) -> RawInstruction {
        use zama_host::events::{
            FheBinaryOpEvent, FheIsInEvent, FheMulDivEvent, FheRandBoundedEvent, FheRandEvent,
            FheSumEvent, FheTernaryOpEvent, FheUnaryOpEvent, TrivialEncryptEvent,
        };
        use zama_host::state::{FheBinaryOpCode, FheTernaryOpCode, FheUnaryOpCode};

        let mut data = ANCHOR_EVENT_IX_TAG_LE.to_vec();
        data.extend_from_slice(&event_discriminator(event_name));
        match event_name {
            "FheBinaryOpEvent" => FheBinaryOpEvent {
                version: 1,
                op: FheBinaryOpCode::Add,
                subject: pk(0x30),
                lhs: pk(0x10),
                rhs: pk(0x11),
                scalar: false,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheTernaryOpEvent" => FheTernaryOpEvent {
                version: 1,
                op: FheTernaryOpCode::IfThenElse,
                subject: pk(0x30),
                control: pk(0x10),
                if_true: pk(0x11),
                if_false: pk(0x12),
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "TrivialEncryptEvent" => TrivialEncryptEvent {
                version: 1,
                subject: pk(0x30),
                plaintext: pk(0x02),
                fhe_type: 5,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheRandEvent" => FheRandEvent {
                version: 1,
                subject: pk(0x30),
                seed: [0x44; 16],
                fhe_type: 5,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheRandBoundedEvent" => FheRandBoundedEvent {
                version: 1,
                subject: pk(0x30),
                upper_bound: pk(0x13),
                seed: [0x44; 16],
                fhe_type: 5,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheUnaryOpEvent" => FheUnaryOpEvent {
                version: 1,
                op: FheUnaryOpCode::Neg,
                subject: pk(0x30),
                operand: pk(0x10),
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheSumEvent" => FheSumEvent {
                version: 1,
                subject: pk(0x30),
                operands: vec![pk(0x10), pk(0x11)],
                fhe_type: 5,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheIsInEvent" => FheIsInEvent {
                version: 1,
                subject: pk(0x30),
                value: pk(0x10),
                set: vec![pk(0x11), pk(0x12)],
                fhe_type: 5,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            "FheMulDivEvent" => FheMulDivEvent {
                version: 1,
                subject: pk(0x30),
                factor1: pk(0x10),
                factor2: pk(0x11),
                divisor: pk(0x12),
                scalar: true,
                result,
            }
            .serialize(&mut data)
            .unwrap(),
            other => panic!("unhandled test event type: {other}"),
        };
        RawInstruction {
            program_id: program_id(),
            accounts: vec![pk(0xEE), program_id()],
            data,
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

    #[test]
    fn op_event_result_extracts_handle_from_emit_cpi_event() {
        let ix = op_event_ix(pk(0x21));
        assert_eq!(op_event_result(&ix, program_id()), Some(pk(0x21)));
    }

    #[test]
    fn op_event_result_accepts_all_fhe_eval_event_kinds() {
        for event_name in OP_EVENT_NAMES {
            let ix = op_event_ix_named(event_name, pk(0x21));
            assert_eq!(
                op_event_result(&ix, program_id()),
                Some(pk(0x21)),
                "{event_name} should carry a decodable result handle"
            );
        }
    }

    #[test]
    fn op_event_result_ignores_non_event_and_foreign_program_instructions() {
        // A zama-host instruction that is not an emit_cpi event.
        let not_event = ix_with_data(vec![], "make_handle_public", pk(0x20));
        assert_eq!(op_event_result(&not_event, program_id()), None);
        // The right bytes but a different program id (not a self-CPI).
        let mut foreign = op_event_ix(pk(0x21));
        foreign.program_id = pk(0xFF);
        assert_eq!(op_event_result(&foreign, program_id()), None);
    }

    #[test]
    fn born_public_output_handle_is_resolved_from_the_following_op_event() {
        let ev = pk(0xE0);
        let burn_handle = pk(0x21);
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: pk(0x02),
                fhe_type: 5,
                output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
            }],
        };
        let eval_ix = ix_with_anchor_data(fhe_eval_accounts(&[ev]), "fhe_eval", plan);
        let decoded =
            decode_program_instructions(program_id(), &[eval_ix, op_event_ix(burn_handle)])
                .unwrap();
        assert_eq!(
            decoded,
            vec![DecodedInstruction::FheEvalUpdateEncryptedValue {
                encrypted_value: ev,
                previous_handle: pk(0x20),
                previous_subjects: vec![pk(0x30)],
                output_subjects: vec![pk(0x30)],
                make_public_handle: Some(burn_handle),
            }]
        );
    }

    #[test]
    fn born_public_output_stays_unresolved_when_event_count_mismatches_plan() {
        // One durable step but no op event captured: the correlation count check
        // fails, so no handle is resolved and the born-public leaf fails closed.
        let ev = pk(0xE0);
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![FheEvalStep::TrivialEncrypt {
                plaintext: pk(0x02),
                fhe_type: 5,
                output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
            }],
        };
        let eval_ix = ix_with_anchor_data(fhe_eval_accounts(&[ev]), "fhe_eval", plan);
        let decoded = decode_program_instructions(program_id(), &[eval_ix]).unwrap();
        assert_eq!(
            decoded,
            vec![DecodedInstruction::FheEvalUpdateEncryptedValue {
                encrypted_value: ev,
                previous_handle: pk(0x20),
                previous_subjects: vec![pk(0x30)],
                output_subjects: vec![pk(0x30)],
                make_public_handle: None,
            }]
        );
    }

    #[test]
    fn multi_step_born_public_binds_each_output_to_its_own_step_event() {
        // Two durable outputs; the born-public one is the SECOND step, so it must
        // bind the SECOND event's handle, proving per-step (not positional-in-
        // durable-subset) correlation.
        let ev0 = pk(0xE0);
        let ev1 = pk(0xE1);
        let first_handle = pk(0x50);
        let second_handle = pk(0x51);
        let plan = FheEvalArgs {
            context_id: pk(0x01),
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: pk(0x02),
                    fhe_type: 5,
                    output: durable_output(0, &[0x30], None, None),
                },
                FheEvalStep::TrivialEncrypt {
                    plaintext: pk(0x03),
                    fhe_type: 5,
                    output: make_public_durable_output(1, &[0x30], Some(pk(0x20)), Some(&[0x30])),
                },
            ],
        };
        let eval_ix = ix_with_anchor_data(fhe_eval_accounts(&[ev0, ev1]), "fhe_eval", plan);
        let decoded = decode_program_instructions(
            program_id(),
            &[
                eval_ix,
                op_event_ix(first_handle),
                op_event_ix(second_handle),
            ],
        )
        .unwrap();
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
                    previous_subjects: vec![pk(0x30)],
                    output_subjects: vec![pk(0x30)],
                    make_public_handle: Some(second_handle),
                },
            ]
        );
    }

    #[test]
    fn born_public_output_handle_is_resolved_for_new_operator_variants() {
        let ev = pk(0xE0);
        let burn_handle = pk(0x21);
        let operators = [
            (
                "unary",
                FheEvalStep::Unary {
                    op: zama_host::state::FheUnaryOpCode::Neg,
                    operand: FheEvalOperand::AllowedDurable {
                        handle: pk(0x10),
                        encrypted_value_index: 1,
                    },
                    output_fhe_type: 5,
                    output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
                },
                "FheUnaryOpEvent",
            ),
            (
                "sum",
                FheEvalStep::Sum {
                    operands: vec![FheEvalOperand::AllowedDurable {
                        handle: pk(0x10),
                        encrypted_value_index: 1,
                    }],
                    fhe_type: 5,
                    output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
                },
                "FheSumEvent",
            ),
            (
                "is_in",
                FheEvalStep::IsIn {
                    value: FheEvalOperand::AllowedDurable {
                        handle: pk(0x10),
                        encrypted_value_index: 1,
                    },
                    set: vec![FheEvalOperand::AllowedDurable {
                        handle: pk(0x11),
                        encrypted_value_index: 2,
                    }],
                    fhe_type: 5,
                    output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
                },
                "FheIsInEvent",
            ),
            (
                "mul_div",
                FheEvalStep::MulDiv {
                    factor1: FheEvalOperand::AllowedDurable {
                        handle: pk(0x10),
                        encrypted_value_index: 1,
                    },
                    factor2: FheEvalOperand::Scalar(pk(0x11)),
                    divisor: pk(0x12),
                    output_fhe_type: 5,
                    output: make_public_durable_output(0, &[0x30], Some(pk(0x20)), Some(&[0x30])),
                },
                "FheMulDivEvent",
            ),
        ];

        for (name, step, event_name) in operators {
            let plan = FheEvalArgs {
                context_id: pk(0x01),
                steps: vec![step],
            };
            let eval_ix = ix_with_anchor_data(
                fhe_eval_accounts(&[ev, pk(0xD1), pk(0xD2)]),
                "fhe_eval",
                plan,
            );
            let decoded = decode_program_instructions(
                program_id(),
                &[eval_ix, op_event_ix_named(event_name, burn_handle)],
            )
            .unwrap();
            assert_eq!(
                decoded,
                vec![DecodedInstruction::FheEvalUpdateEncryptedValue {
                    encrypted_value: ev,
                    previous_handle: pk(0x20),
                    previous_subjects: vec![pk(0x30)],
                    output_subjects: vec![pk(0x30)],
                    make_public_handle: Some(burn_handle),
                }],
                "{name} born-public output should resolve from its op event"
            );
        }
    }
}
