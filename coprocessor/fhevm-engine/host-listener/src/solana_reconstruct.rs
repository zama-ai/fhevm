//! Phase 2: reconstruct zama-host compute events from decoded instruction data +
//! block context, WITHOUT relying on on-chain `emit_cpi!`/`emit!`.
//!
//! Covers the `fhe_eval` step-plan walk (recomputing each step's handle via the
//! program's own `computed_*` functions, byte-identical to on-chain emission)
//! and the event-free `EncryptedValue` instruction decode (RFC-024).

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use zama_host::state::{
    computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_rand_seed,
    computed_eval_sum_handle, computed_eval_ternary_handle,
    computed_eval_trivial_handle, computed_eval_unary_handle,
    computed_rand_bounded_handle, computed_rand_handle,
    FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs, FheEvalOperand,
    FheEvalOutput, FheEvalStep, FheTernaryOpCode as PgmTernaryOpCode,
    FheUnaryOpCode as PgmUnaryOpCode,
};

#[cfg(test)]
use crate::database::tfhe_event_propagate::Handle;
use crate::generated::{
    FheBinaryOpCode, FheBinaryOpEvent, FheIsInEvent, FheMulDivEvent,
    FheRandBoundedEvent, FheRandEvent, FheSumEvent, FheTernaryOpCode,
    FheTernaryOpEvent, FheUnaryOpCode, FheUnaryOpEvent, TrivialEncryptEvent,
    EVENT_VERSION,
};
use crate::solana_adapter::{
    material_request, SolanaHostEvent, SolanaMaterialRequest,
};

/// Block + config context the deterministic handle derivation needs, taken from
/// the transaction's slot/block (`previous_bank_hash`, `unix_timestamp`) and the
/// host's on-chain config (`chain_id`).
#[derive(Clone, Copy, Debug)]
pub struct ReconstructContext {
    pub chain_id: u64,
    pub previous_bank_hash: [u8; 32],
    pub unix_timestamp: i64,
}

/// The material request for a durable output handle reconstructed from
/// `fhe_eval`.
pub fn reconstruct_acl_record_bound_material_request(
    bound_handle: [u8; 32],
) -> SolanaMaterialRequest {
    material_request(bound_handle)
}

/// A material request for either side of a durable-handle supersession.
pub fn reconstruct_handle_superseded_material_request(
    handle: [u8; 32],
) -> SolanaMaterialRequest {
    material_request(handle)
}

/// Discriminator for the `fhe_eval` instruction (sha256("global:fhe_eval")[..8]).
const FHE_EVAL_DISCRIMINATOR: [u8; 8] = [176, 42, 63, 177, 244, 167, 120, 109];

/// Decodes a `fhe_eval` instruction's data into the program's own `FheEvalArgs`
/// step plan, reusing the zama-host type (so there is no bespoke decoder to drift
/// from the on-chain layout). The decoded plan is the input to the eval walk that
/// reconstructs one compute event per step — a separate pass that recomputes each
/// step's handle and therefore depends on `previous_bank_hash`.
pub fn decode_fhe_eval_args(instruction_data: &[u8]) -> Option<FheEvalArgs> {
    let payload = instruction_data.strip_prefix(&FHE_EVAL_DISCRIMINATOR)?;
    FheEvalArgs::try_from_slice(payload).ok()
}

// --- RFC-024 `EncryptedValue` instruction decode -----------------------------
//
// `EncryptedValue` is event-free by design (see zama-host's
// `instructions/encrypted_value.rs` module doc): active ACL changes are carried
// by `fhe_eval` durable outputs, `make_handle_public`, `allow_subjects`, and
// `remove_subject`; raw create/update ABI stubs are decoded only as legacy
// historical data if encountered. There is no ACL event to decode — instruction
// data is the allow signal and must be decoded directly (top-level AND
// inner/CPI, since an app program may invoke these via CPI) by Anchor
// discriminator
// (`sha256("global:<name>")[..8]`).

/// One subject grant, matching `zama_host::instructions::encrypted_value::
/// EncryptedValueSubjectGrant`'s wire layout (`Pubkey`, borsh).
#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct EncryptedValueSubjectGrant {
    pub subject: [u8; 32],
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateEncryptedValueArgs {
    pub acl_domain_key: [u8; 32],
    pub app_account: [u8; 32],
    pub encrypted_value_label: [u8; 32],
    pub handle: [u8; 32],
    pub subjects: Vec<EncryptedValueSubjectGrant>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateEncryptedValueArgs {
    pub new_handle: [u8; 32],
    pub previous_handle: [u8; 32],
    pub previous_subjects: Vec<[u8; 32]>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub struct MakeHandlePublicArgs {
    pub handle: [u8; 32],
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub struct AllowSubjectsArgs {
    pub subjects: Vec<EncryptedValueSubjectGrant>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub struct RemoveSubjectArgs {
    pub subject: [u8; 32],
}

/// A decoded `EncryptedValue` instruction, args-only (accounts are resolved by
/// the caller from the instruction's account list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncryptedValueInstruction {
    Create(CreateEncryptedValueArgs),
    Update(UpdateEncryptedValueArgs),
    MakeHandlePublic(MakeHandlePublicArgs),
    AllowSubjects(AllowSubjectsArgs),
    RemoveSubject(RemoveSubjectArgs),
}

/// `sha256("global:create_encrypted_value")[..8]`.
const CREATE_ENCRYPTED_VALUE_DISCRIMINATOR: [u8; 8] =
    [16, 78, 219, 132, 226, 111, 211, 78];
/// `sha256("global:update_encrypted_value")[..8]`.
const UPDATE_ENCRYPTED_VALUE_DISCRIMINATOR: [u8; 8] =
    [134, 7, 12, 247, 233, 80, 35, 215];
/// `sha256("global:make_handle_public")[..8]`.
const MAKE_HANDLE_PUBLIC_DISCRIMINATOR: [u8; 8] =
    [66, 199, 252, 247, 244, 172, 42, 118];
/// `sha256("global:allow_subjects")[..8]`.
const ALLOW_SUBJECTS_DISCRIMINATOR: [u8; 8] =
    [186, 205, 31, 20, 183, 17, 5, 26];
/// `sha256("global:remove_subject")[..8]`.
const REMOVE_SUBJECT_DISCRIMINATOR: [u8; 8] =
    [66, 88, 46, 123, 6, 107, 208, 50];

/// Every pre-`remove_subject` `EncryptedValue` instruction accounts struct places
/// `encrypted_value` at this index (`payer`, `app_account_authority`/`authority`,
/// `encrypted_value`, ...) — see `CreateEncryptedValue`,
/// `AllowEncryptedValueSubjects`, `UpdateEncryptedValue`,
/// `MakeEncryptedValueHandlePublic` in zama-host's `encrypted_value.rs`.
pub const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;

/// Decodes one `EncryptedValue` instruction from raw instruction data
/// (discriminator + borsh args), or `None` if the discriminator doesn't match
/// any known lifecycle discriminator (i.e. it is not an `EncryptedValue` instruction at all —
/// the caller tries this decoder against every top-level and inner
/// instruction of the zama-host program).
pub fn decode_encrypted_value_instruction(
    data: &[u8],
) -> Option<EncryptedValueInstruction> {
    if data.len() < 8 {
        return None;
    }
    let (discriminator, payload) = data.split_at(8);
    match discriminator {
        d if d == CREATE_ENCRYPTED_VALUE_DISCRIMINATOR => {
            CreateEncryptedValueArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::Create)
        }
        d if d == UPDATE_ENCRYPTED_VALUE_DISCRIMINATOR => {
            UpdateEncryptedValueArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::Update)
        }
        d if d == MAKE_HANDLE_PUBLIC_DISCRIMINATOR => {
            MakeHandlePublicArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::MakeHandlePublic)
        }
        d if d == ALLOW_SUBJECTS_DISCRIMINATOR => {
            AllowSubjectsArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::AllowSubjects)
        }
        d if d == REMOVE_SUBJECT_DISCRIMINATOR => {
            RemoveSubjectArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::RemoveSubject)
        }
        _ => None,
    }
}

pub(crate) fn encrypted_value_account_index(
    instruction: &EncryptedValueInstruction,
) -> usize {
    match instruction {
        EncryptedValueInstruction::RemoveSubject(_) => 1,
        EncryptedValueInstruction::Create(_)
        | EncryptedValueInstruction::Update(_)
        | EncryptedValueInstruction::MakeHandlePublic(_)
        | EncryptedValueInstruction::AllowSubjects(_) => {
            ENCRYPTED_VALUE_ACCOUNT_INDEX
        }
    }
}

/// The material request(s) one decoded `EncryptedValue` instruction produces,
/// using only handles carried by the instruction. Subject changes emit no
/// request: material was already prepared when the current handle was created,
/// and KMS reads live ACL state when authorizing decrypts.
///
/// A raw legacy `update_encrypted_value` instruction's `previous_handle` is
/// included because it remains decryptable through the lineage's MMR history.
pub fn encrypted_value_material_requests(
    instruction: &EncryptedValueInstruction,
) -> Vec<SolanaMaterialRequest> {
    match instruction {
        EncryptedValueInstruction::Create(args) => {
            vec![material_request(args.handle)]
        }
        EncryptedValueInstruction::Update(args) => vec![
            material_request(args.new_handle),
            // The outgoing handle remains decryptable through MMR history.
            material_request(args.previous_handle),
        ],
        EncryptedValueInstruction::MakeHandlePublic(args) => {
            vec![material_request(args.handle)]
        }
        EncryptedValueInstruction::AllowSubjects(_)
        | EncryptedValueInstruction::RemoveSubject(_) => Vec::new(),
    }
}

/// Decodes the `EncryptedValue` material requests for one transaction's
/// instructions, given in on-chain execution order and including inner
/// (CPI-invoked) instructions interleaved in that same order. Non-matching
/// instructions are skipped; no state is retained across transactions.
pub fn decode_encrypted_value_instructions<'a>(
    instructions: impl IntoIterator<Item = (&'a str, &'a [u8], &'a [[u8; 32]])>,
    zama_host_program_id: &str,
) -> Vec<SolanaMaterialRequest> {
    instructions
        .into_iter()
        .filter(|(program_id, _, _)| *program_id == zama_host_program_id)
        .filter_map(|(_, data, accounts)| {
            let instruction = decode_encrypted_value_instruction(data)?;
            let encrypted_value_index =
                encrypted_value_account_index(&instruction);
            accounts.get(encrypted_value_index)?;
            Some(encrypted_value_material_requests(&instruction))
        })
        .flatten()
        .collect()
}

/// A decoded instruction invocation: program id, instruction data, resolved
/// account addresses, and the top-level instruction frame it belongs to.
#[derive(Clone, Debug)]
pub struct DecodedInstruction {
    pub program: String,
    pub data: Vec<u8>,
    pub accounts: Vec<[u8; 32]>,
    pub top_level_index: u32,
    pub is_inner: bool,
}

pub fn decode_encrypted_value_material_request_events(
    instructions: &[DecodedInstruction],
    zama_host_program_id: &str,
) -> Vec<SolanaHostEvent> {
    decode_encrypted_value_instructions(
        instructions.iter().map(|ix| {
            (
                ix.program.as_str(),
                ix.data.as_slice(),
                ix.accounts.as_slice(),
            )
        }),
        zama_host_program_id,
    )
    .into_iter()
    .map(SolanaHostEvent::MaterialRequest)
    .collect()
}

// `previous_bank_hash` sourcing lives in `solana_slot_hashes` (feature-independent
// so the gRPC transport can use it too); re-exported here for the reconstruction API.
pub use crate::solana_slot_hashes::{
    previous_bank_hash_from_slot_hashes, SLOT_HASHES_SYSVAR,
};

/// Seed for the singleton HostConfig PDA (`PDA("host-config")`), re-exported so the
/// transport can derive its address to fetch the on-chain config.
pub use zama_host::constants::HOST_CONFIG_SEED;

/// Reads the on-chain `HostConfig` account and returns the
/// `(chain_id, zero_birth_entropy)` pair handle derivation needs, reusing the
/// program's own `HostConfig` type (no bespoke layout to drift from).
///
/// `zero_birth_entropy` mirrors `HostConfig::zero_birth_entropy_allowed()` =
/// `test_shims_enabled && POC_FEATURE_ENABLED && chain_id == SOLANA_POC_CHAIN_ID`.
/// The listener cannot read the program's compile-time `POC_FEATURE_ENABLED`, but
/// it need not: the program rejects `chain_id == SOLANA_POC_CHAIN_ID` at init
/// unless built with `poc`, so an on-chain `chain_id == SOLANA_POC_CHAIN_ID`
/// already implies `POC_FEATURE_ENABLED`. The check below is therefore exact.
pub fn parse_host_config(account_data: &[u8]) -> anyhow::Result<(u64, bool)> {
    use anchor_lang::AccountDeserialize;
    let config =
        zama_host::state::HostConfig::try_deserialize(&mut &account_data[..])
            .map_err(|e| anyhow::anyhow!("decode HostConfig account: {e}"))?;
    let zero_birth_entropy = config.test_shims_enabled
        && config.chain_id == zama_host::constants::SOLANA_POC_CHAIN_ID;
    Ok((config.chain_id, zero_birth_entropy))
}

/// Resolves an eval operand to its handle by reusing already-produced step
/// results — no on-chain account reads. `Scalar` is only valid as a binary rhs
/// (handled by [`resolve_rhs`]); seeing it here means a malformed plan.
fn resolve_operand(
    operand: &FheEvalOperand,
    produced: &[[u8; 32]],
) -> Option<[u8; 32]> {
    match operand {
        FheEvalOperand::AllowedDurable { handle, .. } => Some(*handle),
        FheEvalOperand::AllowedLocal { producer_index } => {
            produced.get(*producer_index as usize).copied()
        }
        // The verified-input handle is known from the operand itself; the
        // program resolves it to `attestation.input_handle` (admission re-verifies
        // the attestation authoritatively, but the operand handle is structural).
        FheEvalOperand::VerifiedInput { attestation } => {
            Some(attestation.input_handle)
        }
        FheEvalOperand::Scalar(_) => None,
    }
}

/// Resolves a binary rhs operand, reporting whether it is a scalar (the program
/// sets `scalar = true` only for a `Scalar` rhs).
fn resolve_rhs(
    operand: &FheEvalOperand,
    produced: &[[u8; 32]],
) -> Option<([u8; 32], bool)> {
    match operand {
        FheEvalOperand::Scalar(bytes) => Some((*bytes, true)),
        other => resolve_operand(other, produced).map(|h| (h, false)),
    }
}

fn map_pgm_binary_op(op: PgmBinaryOpCode) -> FheBinaryOpCode {
    match op {
        PgmBinaryOpCode::Add => FheBinaryOpCode::Add,
        PgmBinaryOpCode::Sub => FheBinaryOpCode::Sub,
        PgmBinaryOpCode::Mul => FheBinaryOpCode::Mul,
        PgmBinaryOpCode::Div => FheBinaryOpCode::Div,
        PgmBinaryOpCode::Rem => FheBinaryOpCode::Rem,
        PgmBinaryOpCode::And => FheBinaryOpCode::And,
        PgmBinaryOpCode::Or => FheBinaryOpCode::Or,
        PgmBinaryOpCode::Xor => FheBinaryOpCode::Xor,
        PgmBinaryOpCode::Shl => FheBinaryOpCode::Shl,
        PgmBinaryOpCode::Shr => FheBinaryOpCode::Shr,
        PgmBinaryOpCode::Rotl => FheBinaryOpCode::Rotl,
        PgmBinaryOpCode::Rotr => FheBinaryOpCode::Rotr,
        PgmBinaryOpCode::Eq => FheBinaryOpCode::Eq,
        PgmBinaryOpCode::Ne => FheBinaryOpCode::Ne,
        PgmBinaryOpCode::Ge => FheBinaryOpCode::Ge,
        PgmBinaryOpCode::Gt => FheBinaryOpCode::Gt,
        PgmBinaryOpCode::Le => FheBinaryOpCode::Le,
        PgmBinaryOpCode::Lt => FheBinaryOpCode::Lt,
        PgmBinaryOpCode::Min => FheBinaryOpCode::Min,
        PgmBinaryOpCode::Max => FheBinaryOpCode::Max,
    }
}

fn map_pgm_unary_op(op: PgmUnaryOpCode) -> FheUnaryOpCode {
    match op {
        PgmUnaryOpCode::Neg => FheUnaryOpCode::Neg,
        PgmUnaryOpCode::Not => FheUnaryOpCode::Not,
        PgmUnaryOpCode::Cast => FheUnaryOpCode::Cast,
    }
}

fn map_pgm_ternary_op(op: PgmTernaryOpCode) -> FheTernaryOpCode {
    match op {
        PgmTernaryOpCode::IfThenElse => FheTernaryOpCode::IfThenElse,
    }
}

/// Reconstructs the per-step compute events a `fhe_eval` plan emits, mirroring
/// the program's `walk_eval_frame`: walk steps in order, resolve operands
/// (`Transient` referring to earlier steps' produced handles), recompute each
/// step's result handle via the program's eval primitives, and record one event
/// per step. Durable and instruction-local outputs derive the identical base
/// handle — no per-output binding (matches EVM `FHEVMExecutor`).
///
/// Returns `None` on a malformed plan (operand referencing a not-yet-produced
/// step, or a `Scalar` where only an encrypted operand is valid). `context_id`
/// comes from the plan; `ctx` supplies chain_id / previous_bank_hash /
/// unix_timestamp; `subject` is the compute subject.
pub fn reconstruct_fhe_eval_steps(
    plan: &FheEvalArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> Option<Vec<ReconstructedEvalStep>> {
    let context_id = plan.context_id;
    let mut produced: Vec<[u8; 32]> = Vec::with_capacity(plan.steps.len());
    let mut steps_out: Vec<ReconstructedEvalStep> =
        Vec::with_capacity(plan.steps.len());

    for (index, step) in plan.steps.iter().enumerate() {
        let op_index = index as u16;
        let event = match step {
            FheEvalStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let lhs_handle = resolve_operand(lhs, &produced)?;
                let (rhs_handle, scalar) = resolve_rhs(rhs, &produced)?;
                let result = computed_eval_handle(
                    *op,
                    lhs_handle,
                    rhs_handle,
                    scalar,
                    *output_fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                    version: EVENT_VERSION,
                    op: map_pgm_binary_op(*op),
                    subject,
                    lhs: lhs_handle,
                    rhs: rhs_handle,
                    scalar,
                    result,
                })
            }
            FheEvalStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                let c = resolve_operand(control, &produced)?;
                let t = resolve_operand(if_true, &produced)?;
                let f = resolve_operand(if_false, &produced)?;
                let result = computed_eval_ternary_handle(
                    *op,
                    c,
                    t,
                    f,
                    *output_fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheTernaryOp(FheTernaryOpEvent {
                    version: EVENT_VERSION,
                    op: map_pgm_ternary_op(*op),
                    subject,
                    control: c,
                    if_true: t,
                    if_false: f,
                    result,
                })
            }
            FheEvalStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => {
                let result = computed_eval_trivial_handle(
                    *plaintext,
                    *fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject,
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::Rand { fhe_type, .. } => {
                let seed = computed_eval_rand_seed(
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                let result =
                    computed_rand_handle(seed, *fhe_type, ctx.chain_id);
                produced.push(result);
                SolanaHostEvent::FheRand(FheRandEvent {
                    version: EVENT_VERSION,
                    subject,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::Unary {
                op,
                operand,
                output_fhe_type,
                output: _,
            } => {
                let operand_handle = resolve_operand(operand, &produced)?;
                let result = computed_eval_unary_handle(
                    *op,
                    operand_handle,
                    *output_fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheUnaryOp(FheUnaryOpEvent {
                    version: EVENT_VERSION,
                    op: map_pgm_unary_op(*op),
                    subject,
                    operand: operand_handle,
                    result,
                })
            }
            FheEvalStep::RandBounded {
                upper_bound,
                fhe_type,
                ..
            } => {
                let seed = computed_eval_rand_seed(
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                let result = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    ctx.chain_id,
                );
                produced.push(result);
                SolanaHostEvent::FheRandBounded(FheRandBoundedEvent {
                    version: EVENT_VERSION,
                    subject,
                    upper_bound: *upper_bound,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::Sum {
                operands,
                fhe_type,
                output: _,
            } => {
                let operand_handles: Vec<[u8; 32]> = operands
                    .iter()
                    .map(|operand| resolve_operand(operand, &produced))
                    .collect::<Option<_>>()?;
                let result = computed_eval_sum_handle(
                    &operand_handles,
                    *fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheSum(FheSumEvent {
                    version: EVENT_VERSION,
                    subject,
                    operands: operand_handles,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::IsIn {
                value,
                set,
                fhe_type,
                output: _,
            } => {
                let value_handle = resolve_operand(value, &produced)?;
                let set_handles: Vec<[u8; 32]> = set
                    .iter()
                    .map(|operand| resolve_operand(operand, &produced))
                    .collect::<Option<_>>()?;
                let result = computed_eval_is_in_handle(
                    value_handle,
                    &set_handles,
                    *fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheIsIn(FheIsInEvent {
                    version: EVENT_VERSION,
                    subject,
                    value: value_handle,
                    set: set_handles,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                output: _,
            } => {
                let factor1_handle = resolve_operand(factor1, &produced)?;
                let (factor2_handle, scalar) = resolve_rhs(factor2, &produced)?;
                let result = computed_eval_mul_div_handle(
                    factor1_handle,
                    factor2_handle,
                    *divisor,
                    scalar,
                    *output_fhe_type,
                    ctx.chain_id,
                    ctx.previous_bank_hash,
                    ctx.unix_timestamp,
                    context_id,
                    op_index,
                );
                produced.push(result);
                SolanaHostEvent::FheMulDiv(FheMulDivEvent {
                    version: EVENT_VERSION,
                    subject,
                    factor1: factor1_handle,
                    factor2: factor2_handle,
                    divisor: *divisor,
                    scalar,
                    result,
                })
            }
        };
        steps_out.push(ReconstructedEvalStep {
            event,
            durable_encrypted_value_index: fhe_eval_step_durable_output_index(
                step,
            ),
            previous_handle: fhe_eval_step_previous_handle(step),
        });
    }
    Some(steps_out)
}

/// One reconstructed `fhe_eval` step: the compute event plus, for a `Durable`
/// output, the `remaining_accounts` index of the output `EncryptedValue` PDA.
/// The transport resolves that index to the durable handle's material request.
pub struct ReconstructedEvalStep {
    pub event: SolanaHostEvent,
    pub durable_encrypted_value_index: Option<u16>,
    /// Present when the durable output supersedes an existing lineage. The
    /// transport requests material for the outgoing and reconstructed output
    /// handles.
    pub previous_handle: Option<[u8; 32]>,
}

pub fn fhe_eval_step_durable_output_index(step: &FheEvalStep) -> Option<u16> {
    match fhe_eval_step_output(step) {
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            ..
        } => Some(*output_encrypted_value_index),
        FheEvalOutput::AllowedLocal => None,
    }
}

/// The outgoing handle when a durable output supersedes an existing lineage.
pub fn fhe_eval_step_previous_handle(step: &FheEvalStep) -> Option<[u8; 32]> {
    match fhe_eval_step_output(step) {
        FheEvalOutput::AllowedDurable {
            previous_handle, ..
        } => *previous_handle,
        FheEvalOutput::AllowedLocal => None,
    }
}

/// The output policy of an eval step, independent of step kind.
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

/// Reconstructs just the per-step compute events (without ACL-record indices) —
/// the shape the shadow-compare consumes. Thin wrapper over
/// [`reconstruct_fhe_eval_steps`].
pub fn reconstruct_fhe_eval_events(
    plan: &FheEvalArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> Option<Vec<SolanaHostEvent>> {
    Some(
        reconstruct_fhe_eval_steps(plan, subject, ctx)?
            .into_iter()
            .map(|step| step.event)
            .collect(),
    )
}

#[cfg(test)]
mod encrypted_value_decode_tests {
    use super::*;
    use anchor_lang::AnchorSerialize;

    const ZAMA_HOST: &str = "ZamaHost11111111111111111111111111111111";
    const ENCRYPTED_VALUE: [u8; 32] = [0x22; 32];

    type TestInstruction<'a> = (&'a str, &'a [u8], &'a [[u8; 32]]);

    fn encode(discriminator: [u8; 8], args: impl AnchorSerialize) -> Vec<u8> {
        let mut bytes = discriminator.to_vec();
        args.serialize(&mut bytes).unwrap();
        bytes
    }

    fn accounts_with_encrypted_value_at_index_2() -> [[u8; 32]; 6] {
        let mut accounts = [[0u8; 32]; 6];
        accounts[ENCRYPTED_VALUE_ACCOUNT_INDEX] = ENCRYPTED_VALUE;
        accounts
    }

    fn remove_subject_accounts() -> [[u8; 32]; 4] {
        let mut accounts = [[0u8; 32]; 4];
        accounts[1] = ENCRYPTED_VALUE;
        accounts
    }

    #[test]
    fn decodes_create_encrypted_value() {
        let args = CreateEncryptedValueArgs {
            acl_domain_key: [1; 32],
            app_account: [2; 32],
            encrypted_value_label: [3; 32],
            handle: [4; 32],
            subjects: vec![EncryptedValueSubjectGrant { subject: [5; 32] }],
        };
        let data = encode(CREATE_ENCRYPTED_VALUE_DISCRIMINATOR, args.clone());
        let decoded = decode_encrypted_value_instruction(&data)
            .expect("must decode create_encrypted_value");
        assert_eq!(decoded, EncryptedValueInstruction::Create(args));
    }

    #[test]
    fn decodes_update_encrypted_value() {
        let args = UpdateEncryptedValueArgs {
            new_handle: [9; 32],
            previous_handle: [8; 32],
            previous_subjects: vec![[7; 32]],
        };
        let data = encode(UPDATE_ENCRYPTED_VALUE_DISCRIMINATOR, args.clone());
        let decoded = decode_encrypted_value_instruction(&data)
            .expect("must decode update_encrypted_value");
        assert_eq!(decoded, EncryptedValueInstruction::Update(args));
    }

    #[test]
    fn decodes_make_handle_public_handle_arg() {
        let args = MakeHandlePublicArgs { handle: [4; 32] };
        let data = encode(MAKE_HANDLE_PUBLIC_DISCRIMINATOR, args.clone());
        assert_eq!(
            decode_encrypted_value_instruction(&data),
            Some(EncryptedValueInstruction::MakeHandlePublic(args))
        );
    }

    #[test]
    fn make_handle_public_malformed_args_fail_closed() {
        let data = MAKE_HANDLE_PUBLIC_DISCRIMINATOR.to_vec();
        assert_eq!(decode_encrypted_value_instruction(&data), None);
    }

    #[test]
    fn decodes_allow_subjects() {
        let args = AllowSubjectsArgs {
            subjects: vec![EncryptedValueSubjectGrant { subject: [6; 32] }],
        };
        let data = encode(ALLOW_SUBJECTS_DISCRIMINATOR, args.clone());
        let decoded = decode_encrypted_value_instruction(&data)
            .expect("must decode allow_subjects");
        assert_eq!(decoded, EncryptedValueInstruction::AllowSubjects(args));
    }

    #[test]
    fn decodes_remove_subject() {
        let args = RemoveSubjectArgs { subject: [7; 32] };
        let data = encode(REMOVE_SUBJECT_DISCRIMINATOR, args.clone());
        let decoded = decode_encrypted_value_instruction(&data)
            .expect("must decode remove_subject");
        assert_eq!(decoded, EncryptedValueInstruction::RemoveSubject(args));
    }

    #[test]
    fn unknown_discriminator_decodes_to_none() {
        let data = [0xFFu8; 8].to_vec();
        assert!(decode_encrypted_value_instruction(&data).is_none());
    }

    #[test]
    fn create_requests_material_for_its_handle() {
        let args = CreateEncryptedValueArgs {
            acl_domain_key: [1; 32],
            app_account: [2; 32],
            encrypted_value_label: [3; 32],
            handle: [4; 32],
            subjects: vec![],
        };
        let requests = encrypted_value_material_requests(
            &EncryptedValueInstruction::Create(args),
        );
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].handle, Handle::from([4; 32]));
    }

    #[test]
    fn update_requests_material_for_both_handles() {
        let args = UpdateEncryptedValueArgs {
            new_handle: [9; 32],
            previous_handle: [8; 32],
            previous_subjects: vec![],
        };
        let requests = encrypted_value_material_requests(
            &EncryptedValueInstruction::Update(args),
        );
        assert_eq!(
            requests.len(),
            2,
            "both the new and the superseded handle must request material"
        );
        let handles: Vec<_> =
            requests.iter().map(|request| request.handle).collect();
        assert!(handles.contains(
            &crate::database::tfhe_event_propagate::Handle::from([9; 32])
        ));
        assert!(
            handles.contains(&crate::database::tfhe_event_propagate::Handle::from([8; 32])),
            "superseded handle must remain fetchable so historical decrypts keep working"
        );
    }

    #[test]
    fn make_handle_public_requests_decoded_handle() {
        let args = MakeHandlePublicArgs { handle: [4; 32] };
        let requests = encrypted_value_material_requests(
            &EncryptedValueInstruction::MakeHandlePublic(args),
        );
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].handle,
            crate::database::tfhe_event_propagate::Handle::from([4; 32])
        );
    }

    #[test]
    fn allow_subjects_schedules_no_material() {
        let args = AllowSubjectsArgs {
            subjects: vec![
                EncryptedValueSubjectGrant { subject: [6; 32] },
                EncryptedValueSubjectGrant { subject: [7; 32] },
            ],
        };
        let requests = encrypted_value_material_requests(
            &EncryptedValueInstruction::AllowSubjects(args),
        );
        assert!(requests.is_empty());
    }

    #[test]
    fn remove_subject_does_not_delete_prepared_material() {
        let args = RemoveSubjectArgs { subject: [6; 32] };
        let requests = encrypted_value_material_requests(
            &EncryptedValueInstruction::RemoveSubject(args),
        );
        assert!(requests.is_empty());
    }

    #[test]
    fn transaction_decode_includes_inner_cpi_instructions() {
        // A top-level instruction from a different (app) program CPIs into
        // zama_host's create_encrypted_value, then allow_subjects; both must be
        // picked up even though only the second is literally top-level here —
        // the walk must not special-case position, only program id.
        let create_data = encode(
            CREATE_ENCRYPTED_VALUE_DISCRIMINATOR,
            CreateEncryptedValueArgs {
                acl_domain_key: [1; 32],
                app_account: [2; 32],
                encrypted_value_label: [3; 32],
                handle: [4; 32],
                subjects: vec![],
            },
        );
        let allow_data = encode(
            ALLOW_SUBJECTS_DISCRIMINATOR,
            AllowSubjectsArgs {
                subjects: vec![EncryptedValueSubjectGrant { subject: [6; 32] }],
            },
        );
        let accounts = accounts_with_encrypted_value_at_index_2();
        let app_program = "AppProgram111111111111111111111111111111";
        let instructions: Vec<TestInstruction<'_>> = vec![
            (app_program, b"unrelated top-level ix data...", &accounts),
            // Inner (CPI-invoked) instruction from the app's top-level call.
            (ZAMA_HOST, &create_data, &accounts),
            (ZAMA_HOST, &allow_data, &accounts),
        ];
        let requests =
            decode_encrypted_value_instructions(instructions, ZAMA_HOST);
        assert_eq!(
            requests.len(),
            1,
            "create requests material while allow_subjects reuses it"
        );
        assert_eq!(requests[0].handle, Handle::from([4; 32]));
    }

    #[test]
    fn transaction_decode_uses_remove_subject_account_index() {
        let remove_data = encode(
            REMOVE_SUBJECT_DISCRIMINATOR,
            RemoveSubjectArgs { subject: [6; 32] },
        );
        let accounts = remove_subject_accounts();
        let instructions: Vec<TestInstruction<'_>> =
            vec![(ZAMA_HOST, &remove_data, &accounts)];
        let requests =
            decode_encrypted_value_instructions(instructions, ZAMA_HOST);
        assert!(requests.is_empty());
    }

    #[test]
    fn non_zama_host_program_instructions_are_skipped() {
        let data = encode(
            CREATE_ENCRYPTED_VALUE_DISCRIMINATOR,
            CreateEncryptedValueArgs {
                acl_domain_key: [1; 32],
                app_account: [2; 32],
                encrypted_value_label: [3; 32],
                handle: [4; 32],
                subjects: vec![],
            },
        );
        let accounts = accounts_with_encrypted_value_at_index_2();
        let instructions: Vec<TestInstruction<'_>> = vec![(
            "SomeOtherProgram1111111111111111111111111",
            &data,
            &accounts,
        )];
        let requests =
            decode_encrypted_value_instructions(instructions, ZAMA_HOST);
        assert!(requests.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUBJECT: [u8; 32] = [1u8; 32];

    fn ctx() -> ReconstructContext {
        ReconstructContext {
            chain_id: 12345,
            previous_bank_hash: [3u8; 32],
            unix_timestamp: 1_700_000_000,
        }
    }

    #[test]
    fn acl_record_bound_material_request_matches_event_path() {
        let handle = [9u8; 32];
        let request = reconstruct_acl_record_bound_material_request(handle);
        assert_eq!(request.handle, Handle::from(handle));
    }
    #[test]
    fn fhe_eval_plan_round_trips_via_program_type() {
        use anchor_lang::AnchorSerialize;
        use zama_host::state::{
            FheBinaryOpCode as PgmBinaryOp, FheEvalArgs, FheEvalOperand,
            FheEvalOutput, FheEvalStep,
        };
        let plan = FheEvalArgs {
            context_id: [1u8; 32],
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::Binary {
                    op: PgmBinaryOp::Add,
                    lhs: FheEvalOperand::AllowedLocal { producer_index: 0 },
                    rhs: FheEvalOperand::Scalar([2u8; 32]),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
            ],
        };
        // Serialize like the on-chain instruction: discriminator + borsh args.
        let mut bytes = FHE_EVAL_DISCRIMINATOR.to_vec();
        plan.serialize(&mut bytes).expect("serialize plan");
        let decoded = decode_fhe_eval_args(&bytes).expect("decode plan");
        assert_eq!(decoded, plan);
        assert_eq!(decoded.steps.len(), 2);
        // Wrong/missing discriminator -> None.
        assert!(decode_fhe_eval_args(&bytes[1..]).is_none());
    }

    #[test]
    fn fhe_eval_walk_chains_transient_handles() {
        let plan = FheEvalArgs {
            context_id: [1u8; 32],
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::Binary {
                    op: PgmBinaryOpCode::Add,
                    lhs: FheEvalOperand::AllowedLocal { producer_index: 0 },
                    rhs: FheEvalOperand::Scalar([2u8; 32]),
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
            ],
        };
        let events =
            reconstruct_fhe_eval_events(&plan, SUBJECT, &ctx()).expect("walk");
        assert_eq!(events.len(), 2);
        let step0 = match &events[0] {
            SolanaHostEvent::TrivialEncrypt(e) => {
                assert_eq!(e.plaintext, [7u8; 32]);
                e.result
            }
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        match &events[1] {
            SolanaHostEvent::FheBinaryOp(e) => {
                assert_eq!(e.op, FheBinaryOpCode::Add);
                assert!(e.scalar);
                assert_eq!(e.rhs, [2u8; 32]);
                // The Transient operand resolved to step 0's produced handle.
                assert_eq!(e.lhs, step0);
            }
            other => panic!("expected FheBinaryOp, got {other:?}"),
        }
    }

    #[test]
    fn fhe_eval_walk_reconstructs_bounded_rand() {
        let upper_bound = {
            let mut bytes = [0u8; 32];
            bytes[31] = 10;
            bytes
        };
        let plan = FheEvalArgs {
            context_id: [1u8; 32],
            steps: vec![FheEvalStep::RandBounded {
                upper_bound,
                fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            }],
        };

        let events =
            reconstruct_fhe_eval_events(&plan, SUBJECT, &ctx()).expect("walk");
        match &events[..] {
            [SolanaHostEvent::FheRandBounded(event)] => {
                assert_eq!(event.subject, SUBJECT);
                assert_eq!(event.upper_bound, upper_bound);
                assert_eq!(event.fhe_type, 5);
            }
            other => panic!("expected FheRandBounded, got {other:?}"),
        }
    }

    #[test]
    fn fhe_eval_walk_reconstructs_composite_and_unary_ops() {
        let cx = ctx();
        let context_id = [1u8; 32];
        let ub = {
            let mut b = [0u8; 32];
            b[31] = 128; // power-of-two upper bound
            b
        };
        let plan = FheEvalArgs {
            context_id,
            steps: vec![
                FheEvalStep::TrivialEncrypt {
                    plaintext: [9u8; 32],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::TrivialEncrypt {
                    plaintext: [4u8; 32],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::Unary {
                    op: PgmUnaryOpCode::Neg,
                    operand: FheEvalOperand::AllowedLocal { producer_index: 0 },
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::Sum {
                    operands: vec![
                        FheEvalOperand::AllowedLocal { producer_index: 0 },
                        FheEvalOperand::AllowedLocal { producer_index: 1 },
                    ],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::IsIn {
                    value: FheEvalOperand::AllowedLocal { producer_index: 0 },
                    set: vec![FheEvalOperand::AllowedLocal {
                        producer_index: 1,
                    }],
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::MulDiv {
                    factor1: FheEvalOperand::AllowedLocal { producer_index: 0 },
                    factor2: FheEvalOperand::Scalar([2u8; 32]),
                    divisor: [3u8; 32],
                    output_fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
                FheEvalStep::RandBounded {
                    upper_bound: ub,
                    fhe_type: 5,
                    output: FheEvalOutput::AllowedLocal,
                },
            ],
        };
        let events =
            reconstruct_fhe_eval_events(&plan, SUBJECT, &cx).expect("walk");
        assert_eq!(events.len(), 7);
        let h0 = match &events[0] {
            SolanaHostEvent::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        let h1 = match &events[1] {
            SolanaHostEvent::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        // Each op resolves its transient operands to prior steps' handles and
        // derives the result via the program's own `computed_*` functions.
        match &events[2] {
            SolanaHostEvent::FheUnaryOp(e) => {
                assert_eq!(e.op, FheUnaryOpCode::Neg);
                assert_eq!(e.operand, h0);
                assert_eq!(
                    e.result,
                    computed_eval_unary_handle(
                        PgmUnaryOpCode::Neg,
                        h0,
                        5,
                        cx.chain_id,
                        cx.previous_bank_hash,
                        cx.unix_timestamp,
                        context_id,
                        2,
                    )
                );
            }
            other => panic!("expected FheUnaryOp, got {other:?}"),
        }
        match &events[3] {
            SolanaHostEvent::FheSum(e) => {
                assert_eq!(e.operands, vec![h0, h1]);
                assert_eq!(
                    e.result,
                    computed_eval_sum_handle(
                        &[h0, h1],
                        5,
                        cx.chain_id,
                        cx.previous_bank_hash,
                        cx.unix_timestamp,
                        context_id,
                        3,
                    )
                );
            }
            other => panic!("expected FheSum, got {other:?}"),
        }
        match &events[4] {
            SolanaHostEvent::FheIsIn(e) => {
                assert_eq!(e.value, h0);
                assert_eq!(e.set, vec![h1]);
                assert_eq!(
                    e.result,
                    computed_eval_is_in_handle(
                        h0,
                        &[h1],
                        5,
                        cx.chain_id,
                        cx.previous_bank_hash,
                        cx.unix_timestamp,
                        context_id,
                        4,
                    )
                );
            }
            other => panic!("expected FheIsIn, got {other:?}"),
        }
        match &events[5] {
            SolanaHostEvent::FheMulDiv(e) => {
                assert_eq!(e.factor1, h0);
                assert_eq!(e.factor2, [2u8; 32]);
                assert!(e.scalar);
                assert_eq!(e.divisor, [3u8; 32]);
                assert_eq!(
                    e.result,
                    computed_eval_mul_div_handle(
                        h0,
                        [2u8; 32],
                        [3u8; 32],
                        true,
                        5,
                        cx.chain_id,
                        cx.previous_bank_hash,
                        cx.unix_timestamp,
                        context_id,
                        5,
                    )
                );
            }
            other => panic!("expected FheMulDiv, got {other:?}"),
        }
        match &events[6] {
            SolanaHostEvent::FheRandBounded(e) => {
                assert_eq!(e.upper_bound, ub);
                let seed = computed_eval_rand_seed(
                    cx.chain_id,
                    cx.previous_bank_hash,
                    cx.unix_timestamp,
                    context_id,
                    6,
                );
                assert_eq!(e.seed, seed);
                assert_eq!(
                    e.result,
                    computed_rand_bounded_handle(ub, seed, 5, cx.chain_id)
                );
            }
            other => panic!("expected FheRandBounded, got {other:?}"),
        }
    }

    #[test]
    fn fhe_eval_walk_rejects_forward_transient_reference() {
        // A first step referencing a not-yet-produced step -> None.
        let plan = FheEvalArgs {
            context_id: [1u8; 32],
            steps: vec![FheEvalStep::Binary {
                op: PgmBinaryOpCode::Add,
                lhs: FheEvalOperand::AllowedLocal { producer_index: 5 },
                rhs: FheEvalOperand::Scalar([2u8; 32]),
                output_fhe_type: 5,
                output: FheEvalOutput::AllowedLocal,
            }],
        };
        assert!(reconstruct_fhe_eval_events(&plan, SUBJECT, &ctx()).is_none());
    }
}
