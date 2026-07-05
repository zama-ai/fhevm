//! Phase 2: reconstruct zama-host compute events from decoded instruction data +
//! block context, WITHOUT relying on on-chain `emit_cpi!`/`emit!`.
//!
//! Covers the `fhe_eval` step-plan walk (recomputing each step's handle via the
//! program's own `computed_*` functions, byte-identical to on-chain emission)
//! and the event-free `EncryptedValue` instruction decode (RFC-024).
#![cfg(feature = "solana-reconstruct")]

use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use zama_host::state::{
    computed_bound_eval_handle, computed_bound_eval_rand_seed,
    computed_bound_eval_ternary_handle, computed_bound_eval_trivial_handle,
    computed_eval_handle, computed_eval_rand_seed,
    computed_eval_ternary_handle, computed_eval_trivial_handle,
    computed_rand_handle, FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs,
    FheEvalOperand, FheEvalOutput, FheEvalStep,
    FheTernaryOpCode as PgmTernaryOpCode,
};

use crate::generated::{
    FheBinaryOpCode, FheBinaryOpEvent, FheRandEvent, FheTernaryOpCode,
    FheTernaryOpEvent, TrivialEncryptEvent, EVENT_VERSION,
};
use std::collections::HashMap;

use crate::solana_adapter::{
    acl_record_fetch, SolanaFinalizedAccountFetch, SolanaHostEvent,
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

/// The `acl_record_bound` allow-fetch a `*_and_bind` instruction produces: the
/// bound handle (identical to the reconstructed compute event's `result`) keyed
/// by the AclRecord PDA the instruction wrote. This allow-reason is what flips
/// `is_allowed=true` on the same-tx compute row so the tfhe-worker materializes
/// the ciphertext. `acl_record` is the AclRecord account address, resolved from
/// the instruction's accounts by the caller (the transport wiring).
pub fn reconstruct_acl_record_bound_fetch(
    acl_record: [u8; 32],
    bound_handle: [u8; 32],
) -> SolanaFinalizedAccountFetch {
    acl_record_fetch(acl_record, bound_handle, "acl_record_bound")
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
// `instructions/encrypted_value.rs` module doc): the program does not
// `emit!`/`emit_cpi!` for `create_encrypted_value` / `update_encrypted_value` /
// `make_handle_public` / `allow_subjects`. There is therefore no event to
// decode — the four instructions themselves ARE the allow signal, and must be
// decoded directly from instruction data (top-level AND inner/CPI, since an
// app program may invoke these via CPI) by their Anchor discriminator
// (`sha256("global:<name>")[..8]`).

/// One subject grant, matching `zama_host::instructions::encrypted_value::
/// EncryptedValueSubjectGrant`'s wire layout (`Pubkey` + `u8`, borsh).
#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct EncryptedValueSubjectGrant {
    pub subject: [u8; 32],
    pub role_flags: u8,
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
pub struct AllowSubjectsArgs {
    pub subjects: Vec<EncryptedValueSubjectGrant>,
}

/// A decoded `EncryptedValue` instruction, args-only (accounts are resolved by
/// the caller from the instruction's account list — see
/// [`ENCRYPTED_VALUE_ACCOUNT_INDEX`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncryptedValueInstruction {
    Create(CreateEncryptedValueArgs),
    Update(UpdateEncryptedValueArgs),
    /// No args: which handle/lineage this affects is resolved from
    /// [`EncryptedValueLineageTracker`], not from the instruction.
    MakeHandlePublic,
    AllowSubjects(AllowSubjectsArgs),
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

/// Every `EncryptedValue` instruction accounts struct places `encrypted_value`
/// at this index (`payer`, `app_account_authority`/`authority`,
/// `encrypted_value`, ...) — see `CreateEncryptedValue`,
/// `AllowEncryptedValueSubjects`, `UpdateEncryptedValue`,
/// `MakeEncryptedValueHandlePublic` in zama-host's `encrypted_value.rs`.
pub const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;

/// Decodes one `EncryptedValue` instruction from raw instruction data
/// (discriminator + borsh args), or `None` if the discriminator doesn't match
/// any of the four (i.e. it is not an `EncryptedValue` instruction at all —
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
            Some(EncryptedValueInstruction::MakeHandlePublic)
        }
        d if d == ALLOW_SUBJECTS_DISCRIMINATOR => {
            AllowSubjectsArgs::try_from_slice(payload)
                .ok()
                .map(EncryptedValueInstruction::AllowSubjects)
        }
        _ => None,
    }
}

/// Tracks each `EncryptedValue` lineage's current handle across the
/// instructions the listener has decoded so far (in on-chain order), so
/// `make_handle_public` and `allow_subjects` — which carry no handle
/// themselves — resolve to the right handle without an extra account fetch.
///
/// Chosen over finalized-account-fetcher resolution: at decode time the
/// transaction is only confirmed, not finalized, so the fetcher hasn't run
/// yet and re-fetching mid-decode would make decode async/fallible on RPC
/// availability. The handle is fully determined by the most recent
/// create/update this tracker has seen for the same `encrypted_value` PDA —
/// exactly the on-chain precondition `update_encrypted_value` itself enforces
/// (`current_handle == previous_handle`) — so a listener that processes
/// instructions in order can reconstruct it for free. Persist one tracker
/// across the whole listener run (keyed by the `encrypted_value` account
/// address, which is stable for a lineage's lifetime).
#[derive(Default, Debug)]
pub struct EncryptedValueLineageTracker {
    current_handle: HashMap<[u8; 32], [u8; 32]>,
}

impl EncryptedValueLineageTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(
        &mut self,
        encrypted_value_account: [u8; 32],
        handle: [u8; 32],
    ) {
        self.current_handle.insert(encrypted_value_account, handle);
    }

    pub fn current_handle(
        &self,
        encrypted_value_account: [u8; 32],
    ) -> Option<[u8; 32]> {
        self.current_handle.get(&encrypted_value_account).copied()
    }
}

/// The allow-fetch(es) one decoded `EncryptedValue` instruction produces,
/// updating `tracker` as a side effect for `create`/`update` (so later
/// `make_handle_public`/`allow_subjects` in this or a later transaction
/// resolve correctly). `encrypted_value_account` is
/// `accounts[ENCRYPTED_VALUE_ACCOUNT_INDEX]`, resolved by the caller.
///
/// `update_encrypted_value`'s `previous_handle` is included as its own fetch
/// (reason `handle_superseded` is reused for the new handle; the previous
/// handle keeps its historical decrypt path alive via the SAME reason on the
/// old handle) so a still-outstanding decrypt of the superseded handle is not
/// silently dropped just because the lineage moved on.
pub fn encrypted_value_instruction_fetches(
    instruction: &EncryptedValueInstruction,
    encrypted_value_account: [u8; 32],
    tracker: &mut EncryptedValueLineageTracker,
) -> Vec<SolanaFinalizedAccountFetch> {
    match instruction {
        EncryptedValueInstruction::Create(args) => {
            tracker.record(encrypted_value_account, args.handle);
            vec![acl_record_fetch(
                encrypted_value_account,
                args.handle,
                "encrypted_value_created",
            )]
        }
        EncryptedValueInstruction::Update(args) => {
            tracker.record(encrypted_value_account, args.new_handle);
            vec![
                acl_record_fetch(
                    encrypted_value_account,
                    args.new_handle,
                    "handle_superseded",
                ),
                // The outgoing handle must remain decryptable historically
                // (SNS/ct128 prep already exists for it); still fetch it so
                // the finalized consumer doesn't treat it as dropped.
                acl_record_fetch(
                    encrypted_value_account,
                    args.previous_handle,
                    "handle_superseded",
                ),
            ]
        }
        EncryptedValueInstruction::MakeHandlePublic => tracker
            .current_handle(encrypted_value_account)
            .map(|handle| {
                vec![acl_record_fetch(
                    encrypted_value_account,
                    handle,
                    "handle_made_public",
                )]
            })
            .unwrap_or_default(),
        EncryptedValueInstruction::AllowSubjects(_) => tracker
            .current_handle(encrypted_value_account)
            .map(|handle| {
                vec![acl_record_fetch(
                    encrypted_value_account,
                    handle,
                    "subject_allowed",
                )]
            })
            .unwrap_or_default(),
    }
}

/// Decodes the `EncryptedValue` allow-fetches for one transaction's
/// instructions, given in on-chain execution order and MUST include inner
/// (CPI-invoked) instructions interleaved in that same order — an app program
/// may invoke `allow_subjects`/`make_handle_public`/etc. via CPI, and the
/// lineage tracker's ordering guarantee only holds if CPI instructions are not
/// dropped. Non-`EncryptedValue`/non-matching-program instructions are
/// skipped. `tracker` persists across transactions (own one per listener run).
pub fn decode_encrypted_value_instructions<'a>(
    instructions: impl IntoIterator<Item = (&'a str, &'a [u8], &'a [[u8; 32]])>,
    zama_host_program_id: &str,
    tracker: &mut EncryptedValueLineageTracker,
) -> Vec<SolanaFinalizedAccountFetch> {
    instructions
        .into_iter()
        .filter(|(program_id, _, _)| *program_id == zama_host_program_id)
        .filter_map(|(_, data, accounts)| {
            let instruction = decode_encrypted_value_instruction(data)?;
            let encrypted_value_account =
                accounts.get(ENCRYPTED_VALUE_ACCOUNT_INDEX).copied()?;
            Some(encrypted_value_instruction_fetches(
                &instruction,
                encrypted_value_account,
                tracker,
            ))
        })
        .flatten()
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
        PgmBinaryOpCode::Ge => FheBinaryOpCode::Ge,
    }
}

fn map_pgm_ternary_op(op: PgmTernaryOpCode) -> FheTernaryOpCode {
    match op {
        PgmTernaryOpCode::IfThenElse => FheTernaryOpCode::IfThenElse,
    }
}

/// Resolves the `(value_key, sequence)` nonce pair a durable eval output binds
/// its handle with, mirroring the program's `output_binding_from_account`:
/// `value_key` is fully determined by the variant's lineage coordinates
/// (`derive_value_key(acl_domain_key, app_account, label)`); `sequence` is the
/// output lineage's MMR `leaf_count` BEFORE this instruction ran — 0 on create
/// (`previous_handle: None`, account absent on-chain), and the caller-supplied
/// pre-instruction leaf count on update (`lineage_leaf_counts`, keyed by
/// value_key).
///
/// Returns `Some(None)` for `AllowedLocal` (unbound), and `None` when an
/// update's pre-instruction leaf count is unknown — the handle cannot be
/// recomputed, so the caller must fall back to another source.
fn resolve_durable_binding(
    output: &FheEvalOutput,
    lineage_leaf_counts: &HashMap<[u8; 32], u64>,
) -> Option<Option<([u8; 32], u64)>> {
    match output {
        FheEvalOutput::AllowedLocal => Some(None),
        FheEvalOutput::AllowedDurable {
            output_acl_domain_key,
            output_app_account,
            output_encrypted_value_label,
            previous_handle,
            ..
        } => {
            let value_key = zama_solana_acl::derive_value_key(
                output_acl_domain_key.to_bytes(),
                output_app_account.to_bytes(),
                *output_encrypted_value_label,
            );
            let sequence = match previous_handle {
                None => 0,
                Some(_) => lineage_leaf_counts.get(&value_key).copied()?,
            };
            Some(Some((value_key, sequence)))
        }
    }
}

/// Reconstructs the per-step compute events a `fhe_eval` plan emits, mirroring
/// the program's `walk_eval_frame`: walk steps in order, resolve operands
/// (`Transient` referring to earlier steps' produced handles), recompute each
/// step's result handle via the program's eval primitives (`Durable` output →
/// bound variant, otherwise unbound), and record one event per step.
///
/// Returns `None` on a malformed plan (operand referencing a not-yet-produced
/// step, or a `Scalar` where only an encrypted operand is valid), or when a
/// durable output supersedes an existing lineage whose pre-instruction MMR
/// leaf count is missing from `lineage_leaf_counts` (see
/// [`resolve_durable_binding`]). `context_id` comes from the plan; `ctx`
/// supplies chain_id / previous_bank_hash / unix_timestamp; `subject` is the
/// compute subject.
pub fn reconstruct_fhe_eval_steps(
    plan: &FheEvalArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
    lineage_leaf_counts: &HashMap<[u8; 32], u64>,
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
                output,
            } => {
                let lhs_handle = resolve_operand(lhs, &produced)?;
                let (rhs_handle, scalar) = resolve_rhs(rhs, &produced)?;
                let result =
                    match resolve_durable_binding(output, lineage_leaf_counts)?
                    {
                        Some((value_key, sequence)) => {
                            computed_bound_eval_handle(
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
                                value_key,
                                sequence,
                            )
                        }
                        None => computed_eval_handle(
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
                        ),
                    };
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
                output,
            } => {
                let c = resolve_operand(control, &produced)?;
                let t = resolve_operand(if_true, &produced)?;
                let f = resolve_operand(if_false, &produced)?;
                let result =
                    match resolve_durable_binding(output, lineage_leaf_counts)?
                    {
                        Some((value_key, sequence)) => {
                            computed_bound_eval_ternary_handle(
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
                                value_key,
                                sequence,
                            )
                        }
                        None => computed_eval_ternary_handle(
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
                        ),
                    };
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
                output,
            } => {
                let result =
                    match resolve_durable_binding(output, lineage_leaf_counts)?
                    {
                        Some((value_key, sequence)) => {
                            computed_bound_eval_trivial_handle(
                                *plaintext,
                                *fhe_type,
                                ctx.chain_id,
                                ctx.previous_bank_hash,
                                ctx.unix_timestamp,
                                context_id,
                                op_index,
                                value_key,
                                sequence,
                            )
                        }
                        None => computed_eval_trivial_handle(
                            *plaintext,
                            *fhe_type,
                            ctx.chain_id,
                            ctx.previous_bank_hash,
                            ctx.unix_timestamp,
                            context_id,
                            op_index,
                        ),
                    };
                produced.push(result);
                SolanaHostEvent::TrivialEncrypt(TrivialEncryptEvent {
                    version: EVENT_VERSION,
                    subject,
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheEvalStep::Rand { fhe_type, output } => {
                let seed =
                    match resolve_durable_binding(output, lineage_leaf_counts)?
                    {
                        Some((value_key, sequence)) => {
                            computed_bound_eval_rand_seed(
                                ctx.chain_id,
                                ctx.previous_bank_hash,
                                ctx.unix_timestamp,
                                context_id,
                                op_index,
                                value_key,
                                sequence,
                            )
                        }
                        None => computed_eval_rand_seed(
                            ctx.chain_id,
                            ctx.previous_bank_hash,
                            ctx.unix_timestamp,
                            context_id,
                            op_index,
                        ),
                    };
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
        };
        let durable_encrypted_value_index = match fhe_eval_step_output(step) {
            FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                ..
            } => Some(*output_encrypted_value_index),
            FheEvalOutput::AllowedLocal => None,
        };
        steps_out.push(ReconstructedEvalStep {
            event,
            durable_encrypted_value_index,
        });
    }
    Some(steps_out)
}

/// One reconstructed `fhe_eval` step: the compute event plus, for a `Durable`
/// output, the `remaining_accounts` index of the output `EncryptedValue` PDA.
/// The transport resolves that index to an account address to rebuild the
/// `acl_record_bound` allow-fetch (the same lineage the program's bind wrote),
/// which flips `is_allowed=true` on the compute row.
pub struct ReconstructedEvalStep {
    pub event: SolanaHostEvent,
    pub durable_encrypted_value_index: Option<u16>,
}

/// The output policy of an eval step, independent of step kind.
fn fhe_eval_step_output(step: &FheEvalStep) -> &FheEvalOutput {
    match step {
        FheEvalStep::Binary { output, .. }
        | FheEvalStep::Ternary { output, .. }
        | FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. } => output,
    }
}

/// Reconstructs just the per-step compute events (without ACL-record indices) —
/// the shape the shadow-compare consumes. Thin wrapper over
/// [`reconstruct_fhe_eval_steps`].
pub fn reconstruct_fhe_eval_events(
    plan: &FheEvalArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
    lineage_leaf_counts: &HashMap<[u8; 32], u64>,
) -> Option<Vec<SolanaHostEvent>> {
    Some(
        reconstruct_fhe_eval_steps(plan, subject, ctx, lineage_leaf_counts)?
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

    #[test]
    fn decodes_create_encrypted_value() {
        let args = CreateEncryptedValueArgs {
            acl_domain_key: [1; 32],
            app_account: [2; 32],
            encrypted_value_label: [3; 32],
            handle: [4; 32],
            subjects: vec![EncryptedValueSubjectGrant {
                subject: [5; 32],
                role_flags: 1,
            }],
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
    fn decodes_make_handle_public_with_no_args() {
        let data = MAKE_HANDLE_PUBLIC_DISCRIMINATOR.to_vec();
        assert_eq!(
            decode_encrypted_value_instruction(&data),
            Some(EncryptedValueInstruction::MakeHandlePublic)
        );
    }

    #[test]
    fn decodes_allow_subjects() {
        let args = AllowSubjectsArgs {
            subjects: vec![EncryptedValueSubjectGrant {
                subject: [6; 32],
                role_flags: 2,
            }],
        };
        let data = encode(ALLOW_SUBJECTS_DISCRIMINATOR, args.clone());
        let decoded = decode_encrypted_value_instruction(&data)
            .expect("must decode allow_subjects");
        assert_eq!(decoded, EncryptedValueInstruction::AllowSubjects(args));
    }

    #[test]
    fn unknown_discriminator_decodes_to_none() {
        let data = [0xFFu8; 8].to_vec();
        assert!(decode_encrypted_value_instruction(&data).is_none());
    }

    #[test]
    fn create_produces_encrypted_value_created_fetch_and_records_lineage() {
        let mut tracker = EncryptedValueLineageTracker::new();
        let args = CreateEncryptedValueArgs {
            acl_domain_key: [1; 32],
            app_account: [2; 32],
            encrypted_value_label: [3; 32],
            handle: [4; 32],
            subjects: vec![],
        };
        let fetches = encrypted_value_instruction_fetches(
            &EncryptedValueInstruction::Create(args),
            ENCRYPTED_VALUE,
            &mut tracker,
        );
        assert_eq!(fetches.len(), 1);
        assert_eq!(fetches[0].account_key, ENCRYPTED_VALUE);
        assert_eq!(fetches[0].reason, "encrypted_value_created");
        assert_eq!(
            tracker.current_handle(ENCRYPTED_VALUE),
            Some([4; 32]),
            "lineage tracker must record the birth handle"
        );
    }

    #[test]
    fn update_keeps_previous_handle_decryptable_and_advances_lineage() {
        let mut tracker = EncryptedValueLineageTracker::new();
        tracker.record(ENCRYPTED_VALUE, [8; 32]);
        let args = UpdateEncryptedValueArgs {
            new_handle: [9; 32],
            previous_handle: [8; 32],
            previous_subjects: vec![],
        };
        let fetches = encrypted_value_instruction_fetches(
            &EncryptedValueInstruction::Update(args),
            ENCRYPTED_VALUE,
            &mut tracker,
        );
        assert_eq!(
            fetches.len(),
            2,
            "both the new and the superseded handle must be fetched"
        );
        let handles: Vec<_> =
            fetches.iter().map(|f| f.handle.unwrap()).collect();
        assert!(handles.contains(
            &crate::database::tfhe_event_propagate::Handle::from([9; 32])
        ));
        assert!(
            handles.contains(&crate::database::tfhe_event_propagate::Handle::from([8; 32])),
            "superseded handle must remain fetchable so historical decrypts keep working"
        );
        assert_eq!(
            tracker.current_handle(ENCRYPTED_VALUE),
            Some([9; 32]),
            "lineage tracker must advance to the new handle"
        );
    }

    #[test]
    fn make_handle_public_resolves_current_handle_from_lineage_tracker() {
        let mut tracker = EncryptedValueLineageTracker::new();
        tracker.record(ENCRYPTED_VALUE, [4; 32]);
        let fetches = encrypted_value_instruction_fetches(
            &EncryptedValueInstruction::MakeHandlePublic,
            ENCRYPTED_VALUE,
            &mut tracker,
        );
        assert_eq!(fetches.len(), 1);
        assert_eq!(fetches[0].reason, "handle_made_public");
        assert_eq!(
            fetches[0].handle,
            Some(crate::database::tfhe_event_propagate::Handle::from([4; 32]))
        );
    }

    #[test]
    fn make_handle_public_with_unknown_lineage_yields_no_fetch() {
        // No prior create/update seen for this account: cannot resolve which
        // handle -> no fetch (rather than guessing).
        let mut tracker = EncryptedValueLineageTracker::new();
        let fetches = encrypted_value_instruction_fetches(
            &EncryptedValueInstruction::MakeHandlePublic,
            ENCRYPTED_VALUE,
            &mut tracker,
        );
        assert!(fetches.is_empty());
    }

    #[test]
    fn allow_subjects_resolves_current_handle_from_lineage_tracker() {
        let mut tracker = EncryptedValueLineageTracker::new();
        tracker.record(ENCRYPTED_VALUE, [4; 32]);
        let args = AllowSubjectsArgs {
            subjects: vec![EncryptedValueSubjectGrant {
                subject: [6; 32],
                role_flags: 1,
            }],
        };
        let fetches = encrypted_value_instruction_fetches(
            &EncryptedValueInstruction::AllowSubjects(args),
            ENCRYPTED_VALUE,
            &mut tracker,
        );
        assert_eq!(fetches.len(), 1);
        assert_eq!(fetches[0].reason, "subject_allowed");
        assert_eq!(
            fetches[0].handle,
            Some(crate::database::tfhe_event_propagate::Handle::from([4; 32]))
        );
    }

    #[test]
    fn transaction_decode_includes_inner_cpi_instructions() {
        // A top-level instruction from a different (app) program CPIs into
        // zama_host's create_encrypted_value, then allow_subjects; both must be
        // picked up even though only the second is literally top-level here —
        // the walk must not special-case position, only program id.
        let mut tracker = EncryptedValueLineageTracker::new();
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
                subjects: vec![EncryptedValueSubjectGrant {
                    subject: [6; 32],
                    role_flags: 1,
                }],
            },
        );
        let accounts = accounts_with_encrypted_value_at_index_2();
        let app_program = "AppProgram111111111111111111111111111111";
        let instructions: Vec<(&str, &[u8], &[[u8; 32]])> = vec![
            (app_program, b"unrelated top-level ix data...", &accounts),
            // Inner (CPI-invoked) instruction from the app's top-level call.
            (ZAMA_HOST, &create_data, &accounts),
            (ZAMA_HOST, &allow_data, &accounts),
        ];
        let fetches = decode_encrypted_value_instructions(
            instructions,
            ZAMA_HOST,
            &mut tracker,
        );
        assert_eq!(
            fetches.len(),
            2,
            "both inner zama_host instructions must decode to a fetch"
        );
        assert_eq!(fetches[0].reason, "encrypted_value_created");
        assert_eq!(fetches[1].reason, "subject_allowed");
        assert_eq!(
            fetches[1].handle,
            Some(crate::database::tfhe_event_propagate::Handle::from(
                [4; 32]
            )),
            "allow_subjects (CPI) must resolve the handle create_encrypted_value (also CPI) just bound"
        );
    }

    #[test]
    fn non_zama_host_program_instructions_are_skipped() {
        let mut tracker = EncryptedValueLineageTracker::new();
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
        let instructions: Vec<(&str, &[u8], &[[u8; 32]])> = vec![(
            "SomeOtherProgram1111111111111111111111111",
            &data,
            &accounts,
        )];
        let fetches = decode_encrypted_value_instructions(
            instructions,
            ZAMA_HOST,
            &mut tracker,
        );
        assert!(fetches.is_empty());
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
    fn acl_record_bound_fetch_matches_event_path() {
        use crate::solana_adapter::SolanaFinalizedAccountFetchKind;
        let acl_record = [8u8; 32];
        let handle = [9u8; 32];
        let f = reconstruct_acl_record_bound_fetch(acl_record, handle);
        // Identical shape to the AclRecordBound event-decode path (acl_record_fetch).
        assert_eq!(f.account_key, acl_record);
        assert_eq!(f.kind, SolanaFinalizedAccountFetchKind::AclRecord);
        assert_eq!(f.reason, "acl_record_bound");
        assert!(f.handle.is_some());
        assert_eq!(f.related_account, None);
        assert_eq!(f.subject, None);
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
        let events = reconstruct_fhe_eval_events(
            &plan,
            SUBJECT,
            &ctx(),
            &HashMap::new(),
        )
        .expect("walk");
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
        assert!(reconstruct_fhe_eval_events(
            &plan,
            SUBJECT,
            &ctx(),
            &HashMap::new()
        )
        .is_none());
    }
}
