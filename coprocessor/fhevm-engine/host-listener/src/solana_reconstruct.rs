//! Phase 2: reconstruct zama-host compute events from decoded instruction data +
//! block context, WITHOUT relying on on-chain `emit_cpi!`/`emit!`.
//!
//! Lean by design — recompute a result handle ONLY when it is neither carried in
//! the instruction args nor authoritatively stored elsewhere:
//!   * `trivial_encrypt_and_bind`, `fhe_rand_and_bind`, `fhe_rand_bounded_and_bind`
//!     compute their result on-chain and do NOT pass it as an arg, so we recompute
//!     it here.
//!   * `fhe_binary_op*` / `fhe_ternary_op*` pass `result` as a (program-verified)
//!     arg — read it from args, no recompute (handled by the caller, not here).
//!   * token balance/total-supply handles come from on-chain `fhe_eval` and are
//!     read from account state (the account-witness route), not recomputed.
//!
//! Recomputation reuses the program's OWN `computed_*_handle` functions (the
//! `zama-host` crate built with `no-entrypoint`) so the derived handles are
//! byte-identical to what the program emits on-chain — parity a hand-copied
//! implementation could silently lose.
#![cfg(feature = "solana-reconstruct")]

use anchor_lang::AnchorDeserialize;
use zama_host::state::{
    computed_bound_eval_handle, computed_bound_eval_rand_seed,
    computed_bound_eval_ternary_handle, computed_bound_eval_trivial_handle,
    computed_eval_handle, computed_eval_rand_seed,
    computed_eval_ternary_handle, computed_eval_trivial_handle,
    computed_rand_bounded_handle, computed_rand_handle, computed_rand_seed,
    computed_trivial_handle, FheBinaryOpCode as PgmBinaryOpCode, FheEvalArgs,
    FheEvalOperand, FheEvalOutput, FheEvalStep,
    FheTernaryOpCode as PgmTernaryOpCode,
};

use crate::generated::zama_host_instructions::{
    AllowForDecryptionArgs, FheBinaryOpCode as InstrBinaryOpCode,
    FheRandAndBindArgs, FheRandBoundedAndBindArgs,
    FheTernaryOpCode as InstrTernaryOpCode, TrivialEncryptAndBindArgs,
    ZamaHostInstruction,
};
use crate::generated::{
    FheBinaryOpCode, FheBinaryOpEvent, FheRandBoundedEvent, FheRandEvent,
    FheTernaryOpCode, FheTernaryOpEvent, TrivialEncryptEvent, EVENT_VERSION,
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

/// Reconstructs the `TrivialEncryptEvent` a `trivial_encrypt_and_bind` instruction
/// emits, deriving the result handle exactly as the program does on-chain.
/// `subject` is the compute subject (derived from the instruction signer by the
/// caller and threaded in, not re-derived here).
pub fn reconstruct_trivial_encrypt(
    args: &TrivialEncryptAndBindArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> TrivialEncryptEvent {
    let result = computed_trivial_handle(
        args.plaintext,
        args.fhe_type,
        ctx.chain_id,
        ctx.previous_bank_hash,
        ctx.unix_timestamp,
        args.output_nonce_key,
        args.output_nonce_sequence,
    );
    TrivialEncryptEvent {
        version: EVENT_VERSION,
        subject,
        plaintext: args.plaintext,
        fhe_type: args.fhe_type,
        result,
    }
}

/// Reconstructs the `FheRandEvent` a `fhe_rand_and_bind` instruction emits. The
/// seed and result handle are both computed on-chain (not passed as args), so we
/// recompute both via the program's functions.
pub fn reconstruct_fhe_rand(
    args: &FheRandAndBindArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> FheRandEvent {
    let seed = computed_rand_seed(
        ctx.chain_id,
        ctx.previous_bank_hash,
        ctx.unix_timestamp,
        args.output_nonce_key,
        args.output_nonce_sequence,
    );
    let result = computed_rand_handle(seed, args.fhe_type, ctx.chain_id);
    FheRandEvent {
        version: EVENT_VERSION,
        subject,
        seed,
        fhe_type: args.fhe_type,
        result,
    }
}

/// Reconstructs the `FheRandBoundedEvent` a `fhe_rand_bounded_and_bind`
/// instruction emits.
pub fn reconstruct_fhe_rand_bounded(
    args: &FheRandBoundedAndBindArgs,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> FheRandBoundedEvent {
    let seed = computed_rand_seed(
        ctx.chain_id,
        ctx.previous_bank_hash,
        ctx.unix_timestamp,
        args.output_nonce_key,
        args.output_nonce_sequence,
    );
    let result = computed_rand_bounded_handle(
        args.upper_bound,
        seed,
        args.fhe_type,
        ctx.chain_id,
    );
    FheRandBoundedEvent {
        version: EVENT_VERSION,
        subject,
        upper_bound: args.upper_bound,
        seed,
        fhe_type: args.fhe_type,
        result,
    }
}

/// Maps a decoded zama-host instruction to the compute event it would emit.
///
/// Recompute for `trivial_encrypt`/`fhe_rand[_bounded]` (handle not in args);
/// `result`-from-args for binary/ternary (program-verified on success). Returns
/// `None` for instructions that produce only ACL/fetch records — those are
/// reconstructed from `output_subjects`/account state in a separate path.
/// `subject` is the compute subject (derived from the instruction signer by the
/// caller and threaded in).
pub fn reconstruct_compute_event(
    instruction: &ZamaHostInstruction,
    subject: [u8; 32],
    ctx: &ReconstructContext,
) -> Option<SolanaHostEvent> {
    use ZamaHostInstruction as I;
    Some(match instruction {
        I::TrivialEncryptAndBind(a) => SolanaHostEvent::TrivialEncrypt(
            reconstruct_trivial_encrypt(a, subject, ctx),
        ),
        I::FheRandAndBind(a) => {
            SolanaHostEvent::FheRand(reconstruct_fhe_rand(a, subject, ctx))
        }
        I::FheRandBoundedAndBind(a) => SolanaHostEvent::FheRandBounded(
            reconstruct_fhe_rand_bounded(a, subject, ctx),
        ),
        I::FheBinaryOp(a) => SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
            version: EVENT_VERSION,
            op: map_binary_op(a.op),
            subject,
            lhs: a.lhs,
            rhs: a.rhs,
            scalar: a.scalar,
            result: a.result,
        }),
        I::FheBinaryOpAndBindOutput(a) => {
            SolanaHostEvent::FheBinaryOp(FheBinaryOpEvent {
                version: EVENT_VERSION,
                op: map_binary_op(a.op),
                subject,
                lhs: a.lhs,
                rhs: a.rhs,
                scalar: a.scalar,
                result: a.result,
            })
        }
        I::FheTernaryOpAndBindOutput(a) => {
            SolanaHostEvent::FheTernaryOp(FheTernaryOpEvent {
                version: EVENT_VERSION,
                op: map_ternary_op(a.op),
                subject,
                control: a.control,
                if_true: a.if_true,
                if_false: a.if_false,
                result: a.result,
            })
        }
        // Produce ACL/fetch records, not compute events; handled elsewhere.
        I::AllowForDecryption(_)
        | I::AllowAclSubjects(_)
        | I::CommitHandleMaterial(_) => return None,
    })
}

/// Reconstructs the full ingestable event set (compute event + ACL allow-fetches)
/// for a decoded zama-host instruction, resolving the ACL-record account from the
/// instruction's `accounts` (raw 32-byte, in account order). Mirrors what the
/// program's emits decode to, so ingest is byte-identical.
///
/// Wired for `trivial_encrypt_and_bind`; other (already-decodable) instructions
/// return `None` until their per-instruction account layout + fetches are added
/// (the caller falls back to emit-decode for those while emits are still on). New
/// `ZamaHostInstruction` variants must be handled explicitly — no wildcard arm.
pub fn reconstruct_instruction_events(
    instruction: &ZamaHostInstruction,
    accounts: &[[u8; 32]],
    ctx: &ReconstructContext,
    acl_handles: &HashMap<[u8; 32], [u8; 32]>,
) -> Option<Vec<SolanaHostEvent>> {
    use ZamaHostInstruction as I;
    match instruction {
        // Compute (+ optional durable bind): (compute_subject index, Some(
        // output_acl_record index) if it binds, subject count). fhe_binary_op is
        // non-bind (compute_subject at 0, no durable output).
        I::FheBinaryOp(_) => {
            compute_with_bind(instruction, accounts, ctx, 0, None, 0)
        }
        I::TrivialEncryptAndBind(a) => compute_with_bind(
            instruction,
            accounts,
            ctx,
            1,
            Some(4),
            a.output_subjects.len(),
        ),
        I::FheBinaryOpAndBindOutput(a) => compute_with_bind(
            instruction,
            accounts,
            ctx,
            1,
            Some(8),
            a.output_subjects.len(),
        ),
        I::FheTernaryOpAndBindOutput(a) => compute_with_bind(
            instruction,
            accounts,
            ctx,
            1,
            Some(10),
            a.output_subjects.len(),
        ),
        I::FheRandAndBind(a) => compute_with_bind(
            instruction,
            accounts,
            ctx,
            1,
            Some(4),
            a.output_subjects.len(),
        ),
        I::FheRandBoundedAndBind(a) => compute_with_bind(
            instruction,
            accounts,
            ctx,
            1,
            Some(4),
            a.output_subjects.len(),
        ),
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

/// Builds a compute instruction's events: the compute event (via
/// [`reconstruct_compute_event`]) plus, for a durable bind, an `acl_record_bound`
/// fetch and one `acl_subject_allowed` fetch per output subject (same record +
/// handle; the `AclAllowed` event maps to no DB row and is omitted).
fn compute_with_bind(
    instruction: &ZamaHostInstruction,
    accounts: &[[u8; 32]],
    ctx: &ReconstructContext,
    subject_index: usize,
    output_acl_record_index: Option<usize>,
    subject_count: usize,
) -> Option<Vec<SolanaHostEvent>> {
    let subject = accounts.get(subject_index).copied()?;
    let compute = reconstruct_compute_event(instruction, subject, ctx)?;
    let result = match &compute {
        SolanaHostEvent::FheBinaryOp(e) => e.result,
        SolanaHostEvent::FheTernaryOp(e) => e.result,
        SolanaHostEvent::TrivialEncrypt(e) => e.result,
        SolanaHostEvent::FheRand(e) => e.result,
        SolanaHostEvent::FheRandBounded(e) => e.result,
        SolanaHostEvent::FinalizedAccountFetch(_)
        | SolanaHostEvent::AclAllowed(_) => return None,
    };

    let mut events = vec![compute];
    if let Some(index) = output_acl_record_index {
        let output_acl_record = accounts.get(index).copied()?;
        events.push(SolanaHostEvent::FinalizedAccountFetch(
            reconstruct_acl_record_bound_fetch(output_acl_record, result),
        ));
        for _ in 0..subject_count {
            events.push(SolanaHostEvent::FinalizedAccountFetch(
                acl_record_fetch(
                    output_acl_record,
                    result,
                    "acl_subject_allowed",
                ),
            ));
        }
    }
    Some(events)
}

/// The instruction and event IDLs define the same opcode enum; map between the
/// two generated copies by variant (a build-time exhaustiveness check guards drift).
fn map_binary_op(op: InstrBinaryOpCode) -> FheBinaryOpCode {
    match op {
        InstrBinaryOpCode::Add => FheBinaryOpCode::Add,
        InstrBinaryOpCode::Sub => FheBinaryOpCode::Sub,
        InstrBinaryOpCode::Ge => FheBinaryOpCode::Ge,
    }
}

fn map_ternary_op(op: InstrTernaryOpCode) -> FheTernaryOpCode {
    match op {
        InstrTernaryOpCode::IfThenElse => FheTernaryOpCode::IfThenElse,
    }
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
        PgmBinaryOpCode::Ge => FheBinaryOpCode::Ge,
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

    fn trivial_args() -> TrivialEncryptAndBindArgs {
        TrivialEncryptAndBindArgs {
            plaintext: [7u8; 32],
            fhe_type: 5,
            output_nonce_key: [9u8; 32],
            output_nonce_sequence: 42,
            output_acl_domain_key: [0u8; 32],
            output_app_account: [0u8; 32],
            output_encrypted_value_label: [0u8; 32],
            output_subjects: Vec::new(),
            output_public_decrypt: false,
        }
    }

    fn rand_args() -> FheRandAndBindArgs {
        FheRandAndBindArgs {
            fhe_type: 5,
            output_nonce_key: [9u8; 32],
            output_nonce_sequence: 42,
            output_acl_domain_key: [0u8; 32],
            output_app_account: [0u8; 32],
            output_encrypted_value_label: [0u8; 32],
            output_subjects: Vec::new(),
            output_public_decrypt: false,
        }
    }

    fn rand_bounded_args() -> FheRandBoundedAndBindArgs {
        FheRandBoundedAndBindArgs {
            upper_bound: [4u8; 32],
            fhe_type: 5,
            output_nonce_key: [9u8; 32],
            output_nonce_sequence: 42,
            output_acl_domain_key: [0u8; 32],
            output_app_account: [0u8; 32],
            output_encrypted_value_label: [0u8; 32],
            output_subjects: Vec::new(),
            output_public_decrypt: false,
        }
    }

    // Golden vectors: pin the program's on-chain derivation so any change to
    // zama-host's computed_*_handle (which would desync reconstruction from
    // emission) breaks these tests loudly. Captured from a first run.
    const GOLDEN_TRIVIAL: [u8; 32] = [
        91, 123, 147, 51, 77, 108, 124, 18, 206, 116, 17, 238, 84, 157, 204,
        183, 145, 232, 243, 169, 74, 255, 0, 0, 0, 0, 0, 0, 48, 57, 5, 0,
    ];
    const GOLDEN_RAND: [u8; 32] = [
        53, 128, 99, 77, 114, 224, 136, 138, 99, 172, 88, 145, 172, 101, 179,
        182, 81, 120, 194, 48, 85, 255, 0, 0, 0, 0, 0, 0, 48, 57, 5, 0,
    ];
    const GOLDEN_RAND_BOUNDED: [u8; 32] = [
        103, 194, 68, 202, 243, 135, 241, 40, 137, 170, 183, 235, 26, 236, 71,
        222, 188, 240, 224, 67, 213, 255, 0, 0, 0, 0, 0, 0, 48, 57, 5, 0,
    ];

    #[test]
    fn handles_are_pinned() {
        let t = reconstruct_trivial_encrypt(&trivial_args(), SUBJECT, &ctx());
        let r = reconstruct_fhe_rand(&rand_args(), SUBJECT, &ctx());
        let rb =
            reconstruct_fhe_rand_bounded(&rand_bounded_args(), SUBJECT, &ctx());
        assert_eq!(t.subject, SUBJECT);
        assert_eq!(t.fhe_type, 5);
        assert_eq!(t.result, GOLDEN_TRIVIAL);
        assert_eq!(r.result, GOLDEN_RAND);
        assert_eq!(rb.result, GOLDEN_RAND_BOUNDED);
    }

    #[test]
    fn handles_are_deterministic() {
        let a = reconstruct_trivial_encrypt(&trivial_args(), SUBJECT, &ctx());
        let b = reconstruct_trivial_encrypt(&trivial_args(), SUBJECT, &ctx());
        assert_eq!(a.result, b.result);
    }

    #[test]
    fn binary_op_takes_result_from_args() {
        let args = crate::generated::zama_host_instructions::FheBinaryOpArgs {
            op: InstrBinaryOpCode::Add,
            lhs: [2u8; 32],
            rhs: [3u8; 32],
            scalar: false,
            output_fhe_type: 5,
            result: [42u8; 32],
        };
        let ev = reconstruct_compute_event(
            &ZamaHostInstruction::FheBinaryOp(args),
            SUBJECT,
            &ctx(),
        )
        .expect("binary op yields a compute event");
        match ev {
            SolanaHostEvent::FheBinaryOp(e) => {
                assert_eq!(e.op, FheBinaryOpCode::Add);
                assert_eq!(e.subject, SUBJECT);
                // result is taken verbatim from the (program-verified) arg, not recomputed.
                assert_eq!(e.result, [42u8; 32]);
            }
            other => panic!("expected FheBinaryOp, got {other:?}"),
        }
    }

    #[test]
    fn dispatcher_routes_trivial_encrypt() {
        let ev = reconstruct_compute_event(
            &ZamaHostInstruction::TrivialEncryptAndBind(trivial_args()),
            SUBJECT,
            &ctx(),
        )
        .expect("trivial encrypt yields a compute event");
        match ev {
            SolanaHostEvent::TrivialEncrypt(e) => {
                assert_eq!(e.result, GOLDEN_TRIVIAL)
            }
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        }
    }

    #[test]
    fn allow_for_decryption_has_no_compute_event() {
        let args =
            crate::generated::zama_host_instructions::AllowForDecryptionArgs {
                handle: [1u8; 32],
            };
        assert!(reconstruct_compute_event(
            &ZamaHostInstruction::AllowForDecryption(args),
            SUBJECT,
            &ctx(),
        )
        .is_none());
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
    fn instruction_events_non_bind_binary_has_no_fetch() {
        // fhe_binary_op: compute_subject=0, no durable output -> compute event only.
        let args = crate::generated::zama_host_instructions::FheBinaryOpArgs {
            op: InstrBinaryOpCode::Add,
            lhs: [2u8; 32],
            rhs: [3u8; 32],
            scalar: false,
            output_fhe_type: 5,
            result: [42u8; 32],
        };
        let accounts = vec![[0x10u8; 32]]; // accounts[0] = compute_subject
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::FheBinaryOp(args),
            &accounts,
            &ctx(),
            &HashMap::new(),
        )
        .expect("binary op reconstructs");
        assert_eq!(evs.len(), 1, "non-bind: compute event only, no fetch");
        match &evs[0] {
            SolanaHostEvent::FheBinaryOp(e) => {
                assert_eq!(e.subject, [0x10u8; 32]);
                assert_eq!(e.result, [42u8; 32]);
            }
            o => panic!("expected FheBinaryOp, got {o:?}"),
        }
    }

    #[test]
    fn instruction_events_rand_bind_resolves_accounts() {
        use crate::solana_adapter::SolanaFinalizedAccountFetchKind;
        // fhe_rand_and_bind: compute_subject=1, output_acl_record=4; no subjects.
        let mut accounts = vec![[0u8; 32]; 6];
        accounts[1] = [0x11u8; 32];
        accounts[4] = [0x44u8; 32];
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::FheRandAndBind(rand_args()),
            &accounts,
            &ctx(),
            &HashMap::new(),
        )
        .expect("rand bind reconstructs");
        assert_eq!(evs.len(), 2, "compute + acl_record_bound (no subjects)");
        match &evs[0] {
            SolanaHostEvent::FheRand(e) => {
                assert_eq!(e.subject, [0x11u8; 32]); // accounts[1]
                assert_eq!(e.result, GOLDEN_RAND); // subject-independent
            }
            o => panic!("expected FheRand, got {o:?}"),
        }
        match &evs[1] {
            SolanaHostEvent::FinalizedAccountFetch(f) => {
                assert_eq!(f.account_key, [0x44u8; 32]); // accounts[4]
                assert_eq!(f.kind, SolanaFinalizedAccountFetchKind::AclRecord);
                assert_eq!(f.reason, "acl_record_bound");
            }
            o => panic!("expected FinalizedAccountFetch, got {o:?}"),
        }
    }

    #[test]
    fn instruction_events_each_subject_yields_acl_subject_allowed() {
        // One output subject -> one extra acl_subject_allowed fetch (same record).
        let mut args = rand_bounded_args();
        args.output_subjects =
            vec![crate::generated::zama_host_instructions::AclSubjectEntry {
                pubkey: [0x55u8; 32],
                role_flags: 1,
            }];
        let mut accounts = vec![[0u8; 32]; 6];
        accounts[1] = [0x11u8; 32];
        accounts[4] = [0x44u8; 32];
        let evs = reconstruct_instruction_events(
            &ZamaHostInstruction::FheRandBoundedAndBind(args),
            &accounts,
            &ctx(),
            &HashMap::new(),
        )
        .expect("rand bounded bind reconstructs");
        assert_eq!(
            evs.len(),
            3,
            "compute + acl_record_bound + acl_subject_allowed"
        );
        for (i, reason) in
            [(1usize, "acl_record_bound"), (2, "acl_subject_allowed")]
        {
            match &evs[i] {
                SolanaHostEvent::FinalizedAccountFetch(f) => {
                    assert_eq!(f.account_key, [0x44u8; 32]);
                    assert_eq!(f.reason, reason);
                }
                o => panic!("expected fetch at {i}, got {o:?}"),
            }
        }
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
