//! Phase 2: reconstruct zama-host events from decoded instruction data + block
//! context, WITHOUT relying on on-chain `emit_cpi!`/`emit!`.
//!
//! Compute reconstruction is centralized on `fhe_eval`: token
//! balance/total-supply handles come from on-chain `fhe_eval` and are read from
//! account state or reconstructed from the eval plan. Standalone compute
//! instructions are intentionally not decoded here.
//!
//! Recomputation reuses the program's OWN `computed_*_handle` functions (the
//! `zama-host` crate built with `no-entrypoint`) so the derived handles are
//! byte-identical to what the program emits on-chain — parity a hand-copied
//! implementation could silently lose.
use anchor_lang::AnchorDeserialize;
use zama_host::state::{
    computed_bound_eval_handle, computed_bound_eval_is_in_handle,
    computed_bound_eval_mul_div_handle, computed_bound_eval_rand_seed,
    computed_bound_eval_sum_handle, computed_bound_eval_ternary_handle,
    computed_bound_eval_trivial_handle, computed_bound_eval_unary_handle,
    computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_rand_seed,
    computed_eval_sum_handle, computed_eval_ternary_handle,
    computed_eval_trivial_handle, computed_eval_unary_handle,
    computed_rand_bounded_handle, computed_rand_handle,
    FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs, FheEvalOperand,
    FheEvalOutput, FheEvalStep, FheTernaryOpCode as PgmTernaryOpCode,
    FheUnaryOpCode as PgmUnaryOpCode,
};

use crate::generated::zama_host_instructions::{
    AllowForDecryptionArgs, ZamaHostInstruction,
};
use crate::generated::{
    FheBinaryOpCode, FheBinaryOpEvent, FheIsInEvent, FheMulDivEvent,
    FheRandBoundedEvent, FheRandEvent, FheSumEvent, FheTernaryOpCode,
    FheTernaryOpEvent, FheUnaryOpCode, FheUnaryOpEvent, TrivialEncryptEvent,
    EVENT_VERSION,
};
use std::collections::HashMap;

use crate::solana_adapter::{
    acl_record_fetch, material_fetch, SolanaFinalizedAccountFetch,
    SolanaHostEvent,
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

/// Reconstructs the full ingestable event set (compute event + ACL allow-fetches)
/// for a decoded zama-host instruction, resolving the ACL-record account from the
/// instruction's `accounts` (raw 32-byte, in account order). Mirrors what the
/// program's emits decode to, so ingest is byte-identical.
///
/// Wired for fetch-only host instructions. Compute-producing instructions are
/// reconstructed through `fhe_eval`. New `ZamaHostInstruction` variants must be
/// handled explicitly — no wildcard arm.
pub fn reconstruct_instruction_events(
    instruction: &ZamaHostInstruction,
    accounts: &[[u8; 32]],
    _ctx: &ReconstructContext,
    acl_handles: &HashMap<[u8; 32], [u8; 32]>,
) -> Option<Vec<SolanaHostEvent>> {
    use ZamaHostInstruction as I;
    match instruction {
        // Allow instructions: fetch-only (no compute event). The handle was computed
        // by an earlier instruction; these only flip its allow state.
        // allow_for_decryption accounts: authority, authority_permission_record,
        // acl_record(2), host_config, deny_subject_record.
        I::AllowForDecryption(args) => {
            let acl_record = accounts.get(2).copied()?;
            Some(vec![SolanaHostEvent::FinalizedAccountFetch(
                reconstruct_public_decrypt_allowed_fetch(args, acl_record),
            )])
        }
        // allow_acl_subjects accounts: payer, authority, authority_permission_record,
        // acl_record(3), host_config, deny_subject_record, system_program. One
        // `acl_subject_allowed` fetch per subject (the `AclAllowed` event maps to no
        // DB row, omitted; overflow permission records not handled here).
        I::AllowAclSubjects(args) => {
            let acl_record = accounts.get(3).copied()?;
            Some(
                (0..args.subjects.len())
                    .map(|_| {
                        SolanaHostEvent::FinalizedAccountFetch(
                            acl_record_fetch(
                                acl_record,
                                args.handle,
                                "acl_subject_allowed",
                            ),
                        )
                    })
                    .collect(),
            )
        }
        // commit_handle_material: fetch-only, but the handle comes from the
        // acl_record's account STATE (acl_record.handle), pre-fetched by the caller
        // into `acl_handles`. accounts: payer, material_authority, host_config,
        // acl_record(3), material_commitment(4), system_program. Emits both
        // HandleMaterialCommitted + HandleMaterialSealed → three fetches.
        I::CommitHandleMaterial(_) => {
            let acl_record = accounts.get(3).copied()?;
            let material_commitment = accounts.get(4).copied()?;
            let handle = acl_handles.get(&acl_record).copied()?;
            Some(vec![
                SolanaHostEvent::FinalizedAccountFetch(material_fetch(
                    material_commitment,
                    acl_record,
                    handle,
                    "handle_material_committed",
                )),
                SolanaHostEvent::FinalizedAccountFetch(material_fetch(
                    material_commitment,
                    acl_record,
                    handle,
                    "handle_material_sealed",
                )),
                SolanaHostEvent::FinalizedAccountFetch(acl_record_fetch(
                    acl_record,
                    handle,
                    "handle_material_sealed",
                )),
            ])
        }
    }
}

/// Parses the `handle` field from an on-chain `AclRecord` account's data, reusing
/// the program's own `AclRecord` type. The transport reads this for instructions
/// whose emitted handle comes from account state (e.g. `commit_handle_material`),
/// not the instruction args.
pub fn parse_acl_record_handle(account_data: &[u8]) -> Option<[u8; 32]> {
    use anchor_lang::AccountDeserialize;
    zama_host::state::AclRecord::try_deserialize(&mut &account_data[..])
        .ok()
        .map(|record| record.handle)
}

/// The `acl_record_bound` allow-fetch a durable `fhe_eval` output produces: the
/// bound handle (identical to the reconstructed compute event's `result`) keyed
/// by the AclRecord PDA the instruction wrote. This allow-reason is what flips
/// `is_allowed=true` on the same-tx compute row so the tfhe-worker materializes
/// the ciphertext. `acl_record` is the AclRecord account address, resolved from
/// the eval instruction's remaining accounts by the caller.
pub fn reconstruct_acl_record_bound_fetch(
    acl_record: [u8; 32],
    bound_handle: [u8; 32],
) -> SolanaFinalizedAccountFetch {
    acl_record_fetch(acl_record, bound_handle, "acl_record_bound")
}

/// The `public_decrypt_allowed` allow-fetch an `allow_for_decryption` instruction
/// produces. The handle is the instruction arg; `acl_record` is resolved from the
/// instruction's accounts by the caller.
pub fn reconstruct_public_decrypt_allowed_fetch(
    args: &AllowForDecryptionArgs,
    acl_record: [u8; 32],
) -> SolanaFinalizedAccountFetch {
    acl_record_fetch(acl_record, args.handle, "public_decrypt_allowed")
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
/// step's result handle via the program's eval primitives (`Durable` output →
/// bound variant, otherwise unbound), and record one event per step.
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
                output,
            } => {
                let lhs_handle = resolve_operand(lhs, &produced)?;
                let (rhs_handle, scalar) = resolve_rhs(rhs, &produced)?;
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_handle(
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
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_handle(
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
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_ternary_handle(
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
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => {
                        computed_eval_ternary_handle(
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
                        )
                    }
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
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_trivial_handle(
                        *plaintext,
                        *fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => {
                        computed_eval_trivial_handle(
                            *plaintext,
                            *fhe_type,
                            ctx.chain_id,
                            ctx.previous_bank_hash,
                            ctx.unix_timestamp,
                            context_id,
                            op_index,
                        )
                    }
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
                let seed = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_rand_seed(
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_rand_seed(
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
            FheEvalStep::Unary {
                op,
                operand,
                output_fhe_type,
                output,
            } => {
                let operand_handle = resolve_operand(operand, &produced)?;
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_unary_handle(
                        *op,
                        operand_handle,
                        *output_fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_unary_handle(
                        *op,
                        operand_handle,
                        *output_fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                    ),
                };
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
                output,
            } => {
                let seed = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_rand_seed(
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_rand_seed(
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                    ),
                };
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
                output,
            } => {
                let operand_handles: Vec<[u8; 32]> = operands
                    .iter()
                    .map(|operand| resolve_operand(operand, &produced))
                    .collect::<Option<_>>()?;
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_sum_handle(
                        &operand_handles,
                        *fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_sum_handle(
                        &operand_handles,
                        *fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                    ),
                };
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
                output,
            } => {
                let value_handle = resolve_operand(value, &produced)?;
                let set_handles: Vec<[u8; 32]> = set
                    .iter()
                    .map(|operand| resolve_operand(operand, &produced))
                    .collect::<Option<_>>()?;
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_is_in_handle(
                        value_handle,
                        &set_handles,
                        *fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => computed_eval_is_in_handle(
                        value_handle,
                        &set_handles,
                        *fhe_type,
                        ctx.chain_id,
                        ctx.previous_bank_hash,
                        ctx.unix_timestamp,
                        context_id,
                        op_index,
                    ),
                };
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
                output,
            } => {
                let factor1_handle = resolve_operand(factor1, &produced)?;
                let (factor2_handle, scalar) = resolve_rhs(factor2, &produced)?;
                let result = match output {
                    FheEvalOutput::AllowedDurable {
                        output_nonce_key,
                        output_nonce_sequence,
                        ..
                    } => computed_bound_eval_mul_div_handle(
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
                        *output_nonce_key,
                        *output_nonce_sequence,
                    ),
                    FheEvalOutput::AllowedLocal => {
                        computed_eval_mul_div_handle(
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
                        )
                    }
                };
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
        let durable_acl_record_index = match fhe_eval_step_output(step) {
            FheEvalOutput::AllowedDurable {
                output_acl_record_index,
                ..
            } => Some(*output_acl_record_index),
            FheEvalOutput::AllowedLocal => None,
        };
        steps_out.push(ReconstructedEvalStep {
            event,
            durable_acl_record_index,
        });
    }
    Some(steps_out)
}

/// One reconstructed `fhe_eval` step: the compute event plus, for a `Durable`
/// output, the `remaining_accounts` index of the output ACL record PDA. The
/// transport resolves that index to an account address to rebuild the
/// `acl_record_bound` allow-fetch (the same record the program's bind emitted),
/// which flips `is_allowed=true` on the compute row.
pub struct ReconstructedEvalStep {
    pub event: SolanaHostEvent,
    pub durable_acl_record_index: Option<u16>,
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
    fn public_decrypt_allowed_fetch_uses_arg_handle() {
        use crate::solana_adapter::SolanaFinalizedAccountFetchKind;
        let args = AllowForDecryptionArgs { handle: [5u8; 32] };
        let acl_record = [8u8; 32];
        let f = reconstruct_public_decrypt_allowed_fetch(&args, acl_record);
        assert_eq!(f.account_key, acl_record);
        assert_eq!(f.kind, SolanaFinalizedAccountFetchKind::AclRecord);
        assert_eq!(f.reason, "public_decrypt_allowed");
        assert!(f.handle.is_some());
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
        let mut upper_bound = [0u8; 32];
        upper_bound[31] = 8;
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

    #[test]
    fn instruction_events_allow_for_decryption_is_public_decrypt_fetch() {
        use crate::solana_adapter::SolanaFinalizedAccountFetchKind;
        // Fetch-only (no compute event); acl_record is account index 2.
        let args =
            crate::generated::zama_host_instructions::AllowForDecryptionArgs {
                handle: [7u8; 32],
            };
        let mut accounts = vec![[0u8; 32]; 5];
        accounts[2] = [0x22u8; 32];
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::AllowForDecryption(args),
            &accounts,
            &ctx(),
            &HashMap::new(),
        )
        .expect("allow_for_decryption reconstructs");
        assert_eq!(evs.len(), 1, "fetch-only, no compute event");
        match &evs[0] {
            SolanaHostEvent::FinalizedAccountFetch(f) => {
                assert_eq!(f.account_key, [0x22u8; 32]); // accounts[2]
                assert_eq!(f.kind, SolanaFinalizedAccountFetchKind::AclRecord);
                assert_eq!(f.reason, "public_decrypt_allowed");
            }
            o => panic!("expected FinalizedAccountFetch, got {o:?}"),
        }
    }

    #[test]
    fn instruction_events_allow_acl_subjects_one_fetch_per_subject() {
        // Fetch-only; acl_record is account index 3; one fetch per subject.
        let args =
            crate::generated::zama_host_instructions::AllowAclSubjectsArgs {
                handle: [7u8; 32],
                subjects: vec![
                    crate::generated::zama_host_instructions::AclSubjectEntry {
                        pubkey: [1u8; 32],
                        role_flags: 1,
                    },
                    crate::generated::zama_host_instructions::AclSubjectEntry {
                        pubkey: [2u8; 32],
                        role_flags: 1,
                    },
                ],
            };
        let mut accounts = vec![[0u8; 32]; 7];
        accounts[3] = [0x33u8; 32];
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::AllowAclSubjects(args),
            &accounts,
            &ctx(),
            &HashMap::new(),
        )
        .expect("allow_acl_subjects reconstructs");
        assert_eq!(evs.len(), 2, "one acl_subject_allowed fetch per subject");
        for e in &evs {
            match e {
                SolanaHostEvent::FinalizedAccountFetch(f) => {
                    assert_eq!(f.account_key, [0x33u8; 32]); // accounts[3]
                    assert_eq!(f.reason, "acl_subject_allowed");
                }
                o => panic!("expected FinalizedAccountFetch, got {o:?}"),
            }
        }
    }

    #[test]
    fn instruction_events_commit_handle_material_reads_acl_handle() {
        use crate::solana_adapter::SolanaFinalizedAccountFetchKind;
        let args =
            crate::generated::zama_host_instructions::CommitHandleMaterialArgs {
                key_id: [1u8; 32],
                ciphertext_digest: [2u8; 32],
                sns_ciphertext_digest: [3u8; 32],
                coprocessor_set_digest: [4u8; 32],
            };
        // accounts: payer, material_authority, host_config, acl_record(3),
        // material_commitment(4), system_program.
        let mut accounts = vec![[0u8; 32]; 6];
        accounts[3] = [0x33u8; 32]; // acl_record
        accounts[4] = [0x44u8; 32]; // material_commitment
                                    // handle comes from acl_record account state, pre-fetched by the caller.
        let mut acl_handles = HashMap::new();
        acl_handles.insert([0x33u8; 32], [0x99u8; 32]);
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::CommitHandleMaterial(args),
            &accounts,
            &ctx(),
            &acl_handles,
        )
        .expect("commit_handle_material reconstructs");
        assert_eq!(
            evs.len(),
            3,
            "material committed + sealed + acl_record sealed"
        );
        match &evs[0] {
            SolanaHostEvent::FinalizedAccountFetch(f) => {
                assert_eq!(f.account_key, [0x44u8; 32]); // material_commitment
                assert_eq!(
                    f.kind,
                    SolanaFinalizedAccountFetchKind::HandleMaterialCommitment
                );
                assert_eq!(f.reason, "handle_material_committed");
                assert_eq!(f.related_account, Some([0x33u8; 32])); // acl_record
            }
            o => panic!("expected material fetch, got {o:?}"),
        }
        match &evs[2] {
            SolanaHostEvent::FinalizedAccountFetch(f) => {
                assert_eq!(f.account_key, [0x33u8; 32]); // acl_record
                assert_eq!(f.kind, SolanaFinalizedAccountFetchKind::AclRecord);
                assert_eq!(f.reason, "handle_material_sealed");
            }
            o => panic!("expected acl_record fetch, got {o:?}"),
        }
    }
}
