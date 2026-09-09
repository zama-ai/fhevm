//! Phase 2: reconstruct zama-host decoded op records from decoded instruction data +
//! block context, WITHOUT relying on on-chain `emit_cpi!`/`emit!`.
//!
//! Covers the `fhe_execute` execution walk (recomputing each step's handle via the
//! program's own `computed_*` functions, byte-identical to on-chain emission), the
//! persistent output each step writes (the handle it installs, the keys it allows,
//! whether it is made public: the leaf record of RFC 035), and the event-free
//! `make_handle_public` instruction decode.

use anchor_lang::Discriminator;
use zama_host::state::{
    computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_sum_handle,
    computed_eval_ternary_handle, computed_eval_trivial_handle,
    computed_eval_unary_handle, computed_rand_bounded_handle,
    computed_rand_handle, FheExecuteArgs, FheExecuteOperand, FheExecuteOutput,
    FheExecuteStep,
};
use zama_host::{FheExecuteRandomSeed, FheExecuteRandomSeedsEvent};

use crate::solana_adapter::SolanaHostRecord;
use zama_host::records::{
    FheBinaryOp, FheIsIn, FheMulDiv, FheRand, FheRandBounded, FheSum,
    FheTernaryOp, FheUnaryOp, TrivialEncrypt,
};
use zama_host::EVENT_VERSION;

/// Block + config context the deterministic handle derivation needs, taken from
/// the transaction's slot/block (`previous_bank_hash`, `unix_timestamp`) and the
/// host's on-chain config (`chain_id`). This IS the program's own derivation
/// context type — re-exported so reconstruction cannot drift from it.
pub use zama_host::state::HandleDerivationContext as ReconstructContext;

pub fn is_fhe_execute_instruction(instruction_data: &[u8]) -> bool {
    zama_host::decode::is_fhe_execute_instruction(instruction_data)
}

/// Decodes a `fhe_execute` instruction's data into the program's own `FheExecuteArgs`
/// execution through `zama_host::decode` (so there is no bespoke decoder to
/// drift from the on-chain layout). The decoded execution is the input to the execution
/// walk that reconstructs one op record per step — a separate pass that
/// recomputes each step's handle and therefore depends on `previous_bank_hash`.
pub fn decode_fhe_execute_args(
    instruction_data: &[u8],
) -> Option<FheExecuteArgs> {
    match zama_host::decode::decode_instruction(instruction_data) {
        Ok(Some(zama_host::decode::ZamaHostInstruction::FheExecute(args))) => {
            Some(args)
        }
        _ => None,
    }
}

pub fn decode_fhe_execute_random_seeds_event(
    instruction_data: &[u8],
) -> Option<FheExecuteRandomSeedsEvent> {
    let event: FheExecuteRandomSeedsEvent =
        zama_host::decode::decode_event_cpi(instruction_data)?;
    (event.version == zama_host::EVENT_VERSION).then_some(event)
}

// --- `make_handle_public` instruction decode ---------------------------------
//
// `EncryptedValue` is event-free by design (see zama-host's
// `instructions/encrypted_value.rs` module doc): every leaf is carried by the
// instruction that sealed it, a `fhe_execute` persistent output or
// `make_handle_public`. Instruction data is decoded directly (top-level AND
// inner/CPI, since an app program invokes the host via CPI) by Anchor
// discriminator (`sha256("global:<name>")[..8]`).

pub fn is_make_handle_public_instruction(data: &[u8]) -> bool {
    data.get(..8)
        == Some(zama_host::instruction::MakeHandlePublic::DISCRIMINATOR)
}

/// `make_handle_public` places `encrypted_value` at this index (`payer`,
/// `authority`, `encrypted_value`, ...) — see `MakeEncryptedValueHandlePublic`
/// in zama-host's `encrypted_value.rs`.
pub const ENCRYPTED_VALUE_ACCOUNT_INDEX: usize = 2;

/// Decodes the handle a `make_handle_public` instruction seals, through
/// `zama_host::decode`, or `None` if the data is not a well-formed
/// `make_handle_public` instruction.
pub fn decode_make_handle_public(data: &[u8]) -> Option<[u8; 32]> {
    match zama_host::decode::decode_instruction(data).ok()? {
        Some(zama_host::decode::ZamaHostInstruction::MakeHandlePublic {
            handle,
        }) => Some(handle),
        _ => None,
    }
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

// `previous_bank_hash` sourcing lives in `solana_slot_hashes` (feature-independent
// so the gRPC transport can use it too); re-exported here for the reconstruction API.
pub use crate::solana_slot_hashes::{
    previous_bank_hash_from_slot_hashes, SLOT_HASHES_SYSVAR,
};

/// Seed for the singleton HostConfig PDA (`PDA("host-config")`), re-exported so the
/// transport can derive its address to fetch the on-chain config.
pub use zama_host::constants::HOST_CONFIG_SEED;

/// Reads the on-chain `HostConfig` account and returns the chain id used for
/// handle derivation, reusing the program's own type so the layout cannot drift.
pub fn parse_host_config(account_data: &[u8]) -> anyhow::Result<u64> {
    use anchor_lang::AccountDeserialize;
    let config =
        zama_host::state::HostConfig::try_deserialize(&mut &account_data[..])
            .map_err(|e| anyhow::anyhow!("decode HostConfig account: {e}"))?;
    Ok(config.chain_id)
}

/// Resolves an execution operand to its handle by reusing already-produced step
/// results and the execution's interned dictionary — no on-chain account reads.
/// `Scalar` is only valid as a binary rhs (handled by [`resolve_rhs`]); seeing
/// it here means a malformed execution.
fn resolve_operand(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced: &[[u8; 32]],
) -> Option<[u8; 32]> {
    match operand {
        FheExecuteOperand::StoredValue { handle_index, .. } => {
            dictionary.get(usize::from(*handle_index)).copied()
        }
        FheExecuteOperand::EarlierStep { producer_index } => {
            produced.get(usize::from(*producer_index)).copied()
        }
        // The verified-input handle is known from the operand itself; the
        // program resolves it to `attestation.input_handle` (admission re-verifies
        // the attestation authoritatively, but the operand handle is structural).
        FheExecuteOperand::VerifiedInput { attestation } => {
            Some(attestation.input_handle)
        }
        FheExecuteOperand::Scalar { .. } => None,
    }
}

/// Resolves a binary rhs operand, reporting whether it is a scalar (the program
/// sets `scalar = true` only for a `Scalar` rhs).
fn resolve_rhs(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced: &[[u8; 32]],
) -> Option<([u8; 32], bool)> {
    match operand {
        FheExecuteOperand::Scalar { value_index } => dictionary
            .get(usize::from(*value_index))
            .copied()
            .map(|bytes| (bytes, true)),
        other => {
            resolve_operand(other, dictionary, produced).map(|h| (h, false))
        }
    }
}

/// Reconstructs the per-step op records a `fhe_execute` execution produces, mirroring
/// the program's `walk_steps`: walk steps in order, resolve operands
/// (`Transient` referring to earlier steps' produced handles), recompute each
/// step's result handle via the program's execution primitives, and produce one record
/// per step. Persistent and instruction-local outputs derive the identical base
/// handle — no per-output binding (matches EVM `FHEVMExecutor`).
///
/// Returns `None` on a malformed execution (operand or dictionary reference out of range,
/// or a `Scalar` where only an encrypted operand is valid). `ctx` supplies
/// chain_id / previous_bank_hash / unix_timestamp. Random seeds are supplied by
/// the host's signed event-CPI execution because their anchor includes the live
/// rand nonce, which is not caller-provided instruction data.
pub fn reconstruct_fhe_execute_steps(
    execution: &FheExecuteArgs,
    random_seeds: &[FheExecuteRandomSeed],
    ctx: &ReconstructContext,
) -> Option<Vec<ReconstructedExecutionStep>> {
    let expected_random_steps = execution
        .steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            matches!(
                step,
                FheExecuteStep::Rand { .. }
                    | FheExecuteStep::RandBounded { .. }
            )
            .then_some(index as u16)
        })
        .collect::<Vec<_>>();
    if expected_random_steps.len() != random_seeds.len()
        || expected_random_steps
            .iter()
            .zip(random_seeds)
            .any(|(expected, actual)| *expected != actual.step_index)
    {
        return None;
    }
    let mut produced: Vec<[u8; 32]> = Vec::with_capacity(execution.steps.len());
    let mut steps_out: Vec<ReconstructedExecutionStep> =
        Vec::with_capacity(execution.steps.len());

    for (index, step) in execution.steps.iter().enumerate() {
        let op_index = index as u16;
        let record = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let lhs_handle =
                    resolve_operand(lhs, &execution.dictionary, &produced)?;
                let (rhs_handle, scalar) =
                    resolve_rhs(rhs, &execution.dictionary, &produced)?;
                let result = computed_eval_handle(
                    *op,
                    lhs_handle,
                    rhs_handle,
                    scalar,
                    *output_fhe_type,
                    ctx,
                );
                produced.push(result);
                SolanaHostRecord::FheBinaryOp(FheBinaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    lhs: lhs_handle,
                    rhs: rhs_handle,
                    scalar,
                    result,
                })
            }
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                let c =
                    resolve_operand(control, &execution.dictionary, &produced)?;
                let t =
                    resolve_operand(if_true, &execution.dictionary, &produced)?;
                let f = resolve_operand(
                    if_false,
                    &execution.dictionary,
                    &produced,
                )?;
                let result = computed_eval_ternary_handle(
                    *op,
                    c,
                    t,
                    f,
                    *output_fhe_type,
                    ctx,
                );
                produced.push(result);
                SolanaHostRecord::FheTernaryOp(FheTernaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    control: c,
                    if_true: t,
                    if_false: f,
                    result,
                })
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => {
                let result =
                    computed_eval_trivial_handle(*plaintext, *fhe_type, ctx);
                produced.push(result);
                SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                    version: EVENT_VERSION,
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheExecuteStep::Rand { fhe_type, .. } => {
                let seed = random_seeds
                    .iter()
                    .find(|entry| entry.step_index == op_index)?
                    .seed;
                let result =
                    computed_rand_handle(seed, *fhe_type, ctx.chain_id);
                produced.push(result);
                SolanaHostRecord::FheRand(FheRand {
                    version: EVENT_VERSION,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
                output: _,
            } => {
                let operand_handle =
                    resolve_operand(operand, &execution.dictionary, &produced)?;
                let result = computed_eval_unary_handle(
                    *op,
                    operand_handle,
                    *output_fhe_type,
                    ctx,
                );
                produced.push(result);
                SolanaHostRecord::FheUnaryOp(FheUnaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    operand: operand_handle,
                    result,
                })
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
                ..
            } => {
                let seed = random_seeds
                    .iter()
                    .find(|entry| entry.step_index == op_index)?
                    .seed;
                let result = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    ctx.chain_id,
                );
                produced.push(result);
                SolanaHostRecord::FheRandBounded(FheRandBounded {
                    version: EVENT_VERSION,
                    upper_bound: *upper_bound,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheExecuteStep::Sum {
                operands,
                fhe_type,
                output: _,
            } => {
                let operand_handles: Vec<[u8; 32]> = operands
                    .iter()
                    .map(|operand| {
                        resolve_operand(
                            operand,
                            &execution.dictionary,
                            &produced,
                        )
                    })
                    .collect::<Option<_>>()?;
                let result =
                    computed_eval_sum_handle(&operand_handles, *fhe_type, ctx);
                produced.push(result);
                SolanaHostRecord::FheSum(FheSum {
                    version: EVENT_VERSION,
                    operands: operand_handles,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
                output: _,
            } => {
                let value_handle =
                    resolve_operand(value, &execution.dictionary, &produced)?;
                let set_handles: Vec<[u8; 32]> = set
                    .iter()
                    .map(|operand| {
                        resolve_operand(
                            operand,
                            &execution.dictionary,
                            &produced,
                        )
                    })
                    .collect::<Option<_>>()?;
                let result = computed_eval_is_in_handle(
                    value_handle,
                    &set_handles,
                    *fhe_type,
                    ctx,
                );
                produced.push(result);
                SolanaHostRecord::FheIsIn(FheIsIn {
                    version: EVENT_VERSION,
                    value: value_handle,
                    set: set_handles,
                    fhe_type: *fhe_type,
                    result,
                })
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
                output: _,
            } => {
                let factor1_handle =
                    resolve_operand(factor1, &execution.dictionary, &produced)?;
                let (factor2_handle, scalar) =
                    resolve_rhs(factor2, &execution.dictionary, &produced)?;
                let result = computed_eval_mul_div_handle(
                    factor1_handle,
                    factor2_handle,
                    *divisor,
                    scalar,
                    *output_fhe_type,
                    ctx,
                );
                produced.push(result);
                SolanaHostRecord::FheMulDiv(FheMulDiv {
                    version: EVENT_VERSION,
                    factor1: factor1_handle,
                    factor2: factor2_handle,
                    divisor: *divisor,
                    scalar,
                    result,
                })
            }
        };
        let result = *produced.last()?;
        steps_out.push(ReconstructedExecutionStep {
            record,
            persistent: fhe_execute_step_persistent_output(
                step,
                &execution.dictionary,
                result,
            )?,
        });
    }
    Some(steps_out)
}

/// One reconstructed `fhe_execute` step: the decoded op record plus, for a
/// `StoredValue` output, the write it performs on its encrypted value account.
pub struct ReconstructedExecutionStep {
    pub record: SolanaHostRecord,
    pub persistent: Option<PersistentOutput>,
}

/// What a `StoredValue` step writes, read off instruction data alone: the
/// account (as a `remaining_accounts` index the transport resolves), its
/// identity fields, the handle it installs and the leaves it seals. Every field
/// is validated on chain before the write, so a confirmed transaction's values
/// are authoritative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistentOutput {
    /// Index into `remaining_accounts` of the output `EncryptedValue` PDA.
    pub encrypted_value_index: u8,
    pub program: [u8; 32],
    pub encrypted_value_account_authority: [u8; 32],
    pub scope: [u8; 32],
    pub label: [u8; 32],
    /// The handle being replaced: `None` on create.
    pub previous_handle: Option<[u8; 32]>,
    /// The handle the step installs.
    pub handle: [u8; 32],
    /// Keys allowed on `handle`, one historical-access leaf each, in this order.
    pub allowed_keys: Vec<[u8; 32]>,
    /// Whether a public-decrypt leaf for `handle` follows the allow leaves.
    pub make_public: bool,
}

/// Resolves a step's `StoredValue` output against the execution dictionary, or
/// `None` on a dictionary reference out of range (a malformed execution).
fn fhe_execute_step_persistent_output(
    step: &FheExecuteStep,
    dictionary: &[[u8; 32]],
    handle: [u8; 32],
) -> Option<Option<PersistentOutput>> {
    let FheExecuteOutput::StoredValue {
        output_encrypted_value_index,
        output_program_index,
        output_authority_key_index,
        output_scope_index,
        output_label_index,
        output_allow_indexes,
        previous_handle_index,
        make_public,
        ..
    } = fhe_execute_step_output(step)
    else {
        return Some(None);
    };
    let entry = |index: &u8| dictionary.get(usize::from(*index)).copied();
    Some(Some(PersistentOutput {
        encrypted_value_index: *output_encrypted_value_index,
        program: entry(output_program_index)?,
        encrypted_value_account_authority: entry(output_authority_key_index)?,
        scope: entry(output_scope_index)?,
        label: entry(output_label_index)?,
        previous_handle: match previous_handle_index {
            Some(index) => Some(entry(index)?),
            None => None,
        },
        handle,
        allowed_keys: output_allow_indexes
            .iter()
            .map(entry)
            .collect::<Option<Vec<_>>>()?,
        make_public: *make_public,
    }))
}

/// The output policy of an execution step, independent of step kind.
fn fhe_execute_step_output(step: &FheExecuteStep) -> &FheExecuteOutput {
    match step {
        FheExecuteStep::Binary { output, .. }
        | FheExecuteStep::Ternary { output, .. }
        | FheExecuteStep::TrivialEncrypt { output, .. }
        | FheExecuteStep::Rand { output, .. }
        | FheExecuteStep::Unary { output, .. }
        | FheExecuteStep::RandBounded { output, .. }
        | FheExecuteStep::Sum { output, .. }
        | FheExecuteStep::IsIn { output, .. }
        | FheExecuteStep::MulDiv { output, .. } => output,
    }
}

/// Reconstructs just the per-step op records (without ACL-record indices) —
/// the shape the shadow-compare consumes. Thin wrapper over
/// [`reconstruct_fhe_execute_steps`].
pub fn reconstruct_fhe_execute_records(
    execution: &FheExecuteArgs,
    random_seeds: &[FheExecuteRandomSeed],
    ctx: &ReconstructContext,
) -> Option<Vec<SolanaHostRecord>> {
    Some(
        reconstruct_fhe_execute_steps(execution, random_seeds, ctx)?
            .into_iter()
            .map(|step| step.record)
            .collect(),
    )
}

#[cfg(test)]
mod make_handle_public_decode_tests {
    use super::*;
    use anchor_lang::AnchorSerialize;

    fn encode(discriminator: &[u8], args: impl AnchorSerialize) -> Vec<u8> {
        let mut bytes = discriminator.to_vec();
        args.serialize(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn decodes_make_handle_public_handle_arg() {
        let data = encode(
            zama_host::instruction::MakeHandlePublic::DISCRIMINATOR,
            [4u8; 32],
        );
        assert!(is_make_handle_public_instruction(&data));
        assert_eq!(decode_make_handle_public(&data), Some([4; 32]));
    }

    #[test]
    fn decode_matches_anchor_trailing_byte_semantics() {
        let mut data = encode(
            zama_host::instruction::MakeHandlePublic::DISCRIMINATOR,
            [4u8; 32],
        );
        data.extend_from_slice(&[0xAA, 0xBB]);
        assert_eq!(decode_make_handle_public(&data), Some([4; 32]));
    }

    #[test]
    fn malformed_args_fail_closed() {
        let data =
            zama_host::instruction::MakeHandlePublic::DISCRIMINATOR.to_vec();
        assert!(is_make_handle_public_instruction(&data));
        assert_eq!(decode_make_handle_public(&data), None);
    }

    #[test]
    fn other_discriminators_decode_to_none() {
        assert!(!is_make_handle_public_instruction(&[0xFFu8; 8]));
        assert!(decode_make_handle_public(&[0xFFu8; 8]).is_none());
        let fhe_execute =
            zama_host::instruction::FheExecute::DISCRIMINATOR.to_vec();
        assert!(!is_make_handle_public_instruction(&fhe_execute));
        assert!(decode_make_handle_public(&fhe_execute).is_none());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_host::state::{FheBinaryOpCode, FheUnaryOpCode};

    fn ctx() -> ReconstructContext {
        ReconstructContext {
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
            previous_bank_hash: [3u8; 32],
            unix_timestamp: 1_700_000_000,
        }
    }

    #[test]
    fn fhe_execute_batch_round_trips_via_program_type() {
        use anchor_lang::AnchorSerialize;
        use zama_host::state::{
            FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand,
            FheExecuteOutput, FheExecuteStep,
        };
        let execution = FheExecuteArgs {
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
            ],
        };
        // Serialize like the on-chain instruction: discriminator + borsh args.
        let mut bytes =
            zama_host::instruction::FheExecute::DISCRIMINATOR.to_vec();
        execution
            .serialize(&mut bytes)
            .expect("serialize execution");
        let decoded =
            decode_fhe_execute_args(&bytes).expect("decode execution");
        assert_eq!(decoded, execution);
        assert_eq!(decoded.steps.len(), 2);
        // Wrong/missing discriminator -> None.
        assert!(decode_fhe_execute_args(&bytes[1..]).is_none());

        bytes.extend_from_slice(&[0xAA, 0xBB]);
        assert_eq!(decode_fhe_execute_args(&bytes), Some(execution));
    }

    #[test]
    fn decodes_host_derived_random_seed_batch() {
        let event = FheExecuteRandomSeedsEvent {
            version: zama_host::EVENT_VERSION,
            seeds: vec![FheExecuteRandomSeed {
                step_index: 3,
                seed: [7; 16],
            }],
        };
        let data = anchor_lang::event::EVENT_IX_TAG_LE
            .iter()
            .copied()
            .chain(anchor_lang::Event::data(&event))
            .collect::<Vec<_>>();

        let decoded =
            decode_fhe_execute_random_seeds_event(&data).expect("decode event");
        assert_eq!(decoded.version, zama_host::EVENT_VERSION);
        assert_eq!(decoded.seeds, event.seeds);
    }

    #[test]
    fn rejects_unknown_random_seed_event_version() {
        let event = FheExecuteRandomSeedsEvent {
            version: zama_host::EVENT_VERSION.wrapping_add(1),
            seeds: vec![],
        };
        let data = anchor_lang::event::EVENT_IX_TAG_LE
            .iter()
            .copied()
            .chain(anchor_lang::Event::data(&event))
            .collect::<Vec<_>>();

        assert!(decode_fhe_execute_random_seeds_event(&data).is_none());
    }

    #[test]
    fn fhe_execute_walk_chains_transient_handles() {
        let execution = FheExecuteArgs {
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
            ],
        };
        let records = reconstruct_fhe_execute_records(&execution, &[], &ctx())
            .expect("walk");
        assert_eq!(records.len(), 2);
        let step0 = match &records[0] {
            SolanaHostRecord::TrivialEncrypt(e) => {
                assert_eq!(e.plaintext, [7u8; 32]);
                e.result
            }
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        match &records[1] {
            SolanaHostRecord::FheBinaryOp(e) => {
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
    fn fhe_execute_walk_reconstructs_bounded_rand() {
        let upper_bound = {
            let mut bytes = [0u8; 32];
            bytes[31] = 10;
            bytes
        };
        // On-chain preflight requires a rand execution to anchor at least one persistent
        // output (fhevm-internal#1853 W4), so the fixture binds one.
        let execution = FheExecuteArgs {
            account_count: 1,
            dictionary: vec![[0xA1; 32], [0xA2; 32], [0xA3; 32], [0xA4; 32]],
            steps: vec![FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type: 5,
                output: FheExecuteOutput::StoredValue {
                    output_encrypted_value_index: 0,
                    output_authority_index: None,
                    output_program_index: 0,
                    output_authority_key_index: 1,
                    output_scope_index: 1,
                    output_label_index: 2,
                    output_authority_seeds: vec![],
                    output_allow_indexes: vec![3],
                    previous_handle_index: None,
                    make_public: false,
                },
            }],
        };

        let random_seeds = [FheExecuteRandomSeed {
            step_index: 0,
            seed: [7; 16],
        }];
        let records =
            reconstruct_fhe_execute_records(&execution, &random_seeds, &ctx())
                .expect("walk");
        match &records[..] {
            [SolanaHostRecord::FheRandBounded(event)] => {
                assert_eq!(event.upper_bound, upper_bound);
                assert_eq!(event.fhe_type, 5);
            }
            other => panic!("expected FheRandBounded, got {other:?}"),
        }
    }

    #[test]
    fn fhe_execute_walk_reconstructs_composite_and_unary_ops() {
        let cx = ctx();
        let ub = {
            let mut b = [0u8; 32];
            b[31] = 128; // power-of-two upper bound
            b
        };
        // The execution ends in a rand step, so it anchors a persistent output
        // (fhevm-internal#1853 W4); dictionary entries 1..=4 are its identity and allow list.
        let execution = FheExecuteArgs {
            account_count: 1,
            dictionary: vec![
                [2u8; 32], [0xA1; 32], [0xA2; 32], [0xA3; 32], [0xA4; 32],
            ],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [9u8; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [4u8; 32],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::Unary {
                    op: FheUnaryOpCode::Neg,
                    operand: FheExecuteOperand::EarlierStep {
                        producer_index: 0,
                    },
                    output_fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::Sum {
                    operands: vec![
                        FheExecuteOperand::EarlierStep { producer_index: 0 },
                        FheExecuteOperand::EarlierStep { producer_index: 1 },
                    ],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::IsIn {
                    value: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    set: vec![FheExecuteOperand::EarlierStep {
                        producer_index: 1,
                    }],
                    fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::MulDiv {
                    factor1: FheExecuteOperand::EarlierStep {
                        producer_index: 0,
                    },
                    factor2: FheExecuteOperand::Scalar { value_index: 0 },
                    divisor: [3u8; 32],
                    output_fhe_type: 5,
                    output: FheExecuteOutput::Transient,
                },
                FheExecuteStep::RandBounded {
                    upper_bound: ub,
                    fhe_type: 5,
                    output: FheExecuteOutput::StoredValue {
                        output_encrypted_value_index: 0,
                        output_authority_index: None,
                        output_program_index: 1,
                        output_authority_key_index: 2,
                        output_scope_index: 2,
                        output_label_index: 3,
                        output_authority_seeds: vec![],
                        output_allow_indexes: vec![4],
                        previous_handle_index: None,
                        make_public: false,
                    },
                },
            ],
        };
        let random_seed = [9; 16];
        let records = reconstruct_fhe_execute_records(
            &execution,
            &[FheExecuteRandomSeed {
                step_index: 6,
                seed: random_seed,
            }],
            &cx,
        )
        .expect("walk");
        assert_eq!(records.len(), 7);
        let h0 = match &records[0] {
            SolanaHostRecord::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        let h1 = match &records[1] {
            SolanaHostRecord::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        // Each op resolves its transient operands to prior steps' handles and
        // derives the result via the program's own `computed_*` functions.
        match &records[2] {
            SolanaHostRecord::FheUnaryOp(e) => {
                assert_eq!(e.op, FheUnaryOpCode::Neg);
                assert_eq!(e.operand, h0);
                assert_eq!(
                    e.result,
                    computed_eval_unary_handle(FheUnaryOpCode::Neg, h0, 5, &cx,)
                );
            }
            other => panic!("expected FheUnaryOp, got {other:?}"),
        }
        match &records[3] {
            SolanaHostRecord::FheSum(e) => {
                assert_eq!(e.operands, vec![h0, h1]);
                assert_eq!(
                    e.result,
                    computed_eval_sum_handle(&[h0, h1], 5, &cx,)
                );
            }
            other => panic!("expected FheSum, got {other:?}"),
        }
        match &records[4] {
            SolanaHostRecord::FheIsIn(e) => {
                assert_eq!(e.value, h0);
                assert_eq!(e.set, vec![h1]);
                assert_eq!(
                    e.result,
                    computed_eval_is_in_handle(h0, &[h1], 5, &cx,)
                );
            }
            other => panic!("expected FheIsIn, got {other:?}"),
        }
        match &records[5] {
            SolanaHostRecord::FheMulDiv(e) => {
                assert_eq!(e.factor1, h0);
                assert_eq!(e.factor2, [2u8; 32]);
                assert!(e.scalar);
                assert_eq!(e.divisor, [3u8; 32]);
                assert_eq!(
                    e.result,
                    computed_eval_mul_div_handle(
                        h0, [2u8; 32], [3u8; 32], true, 5, &cx,
                    )
                );
            }
            other => panic!("expected FheMulDiv, got {other:?}"),
        }
        match &records[6] {
            SolanaHostRecord::FheRandBounded(e) => {
                assert_eq!(e.upper_bound, ub);
                assert_eq!(e.seed, random_seed);
                assert_eq!(
                    e.result,
                    computed_rand_bounded_handle(
                        ub,
                        random_seed,
                        5,
                        cx.chain_id
                    )
                );
            }
            other => panic!("expected FheRandBounded, got {other:?}"),
        }
    }

    #[test]
    fn fhe_execute_walk_rejects_forward_transient_reference() {
        // A first step referencing a not-yet-produced step -> None.
        let execution = FheExecuteArgs {
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![FheExecuteStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheExecuteOperand::EarlierStep { producer_index: 5 },
                rhs: FheExecuteOperand::Scalar { value_index: 0 },
                output_fhe_type: 5,
                output: FheExecuteOutput::Transient,
            }],
        };
        assert!(
            reconstruct_fhe_execute_records(&execution, &[], &ctx()).is_none()
        );
    }

    /// A `StoredValue` output is read off instruction data: the handle the step
    /// installs, the keys it allows and the public flag become the account's
    /// leaves, and the identity fields name the account.
    #[test]
    fn fhe_execute_walk_extracts_persistent_output() {
        let execution = FheExecuteArgs {
            account_count: 1,
            dictionary: vec![
                [0xA1; 32], [0xA2; 32], [0xA3; 32], [0xA4; 32], [0xB1; 32],
                [0xB2; 32], [0xC1; 32],
            ],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
                output: FheExecuteOutput::StoredValue {
                    output_encrypted_value_index: 3,
                    output_authority_index: None,
                    output_program_index: 0,
                    output_authority_key_index: 1,
                    output_scope_index: 2,
                    output_label_index: 3,
                    output_authority_seeds: vec![],
                    output_allow_indexes: vec![4, 5],
                    previous_handle_index: Some(6),
                    make_public: true,
                },
            }],
        };
        let steps = reconstruct_fhe_execute_steps(&execution, &[], &ctx())
            .expect("walk");
        let handle = match &steps[0].record {
            SolanaHostRecord::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        assert_eq!(
            steps[0].persistent,
            Some(PersistentOutput {
                encrypted_value_index: 3,
                program: [0xA1; 32],
                encrypted_value_account_authority: [0xA2; 32],
                scope: [0xA3; 32],
                label: [0xA4; 32],
                previous_handle: Some([0xC1; 32]),
                handle,
                allowed_keys: vec![[0xB1; 32], [0xB2; 32]],
                make_public: true,
            })
        );

        let transient = FheExecuteArgs {
            account_count: 0,
            dictionary: vec![],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
                output: FheExecuteOutput::Transient,
            }],
        };
        let steps = reconstruct_fhe_execute_steps(&transient, &[], &ctx())
            .expect("walk");
        assert!(steps[0].persistent.is_none());
    }

    #[test]
    fn fhe_execute_walk_rejects_persistent_output_dictionary_overflow() {
        let execution = FheExecuteArgs {
            account_count: 1,
            dictionary: vec![[0xA1; 32]],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
                output: FheExecuteOutput::StoredValue {
                    output_encrypted_value_index: 0,
                    output_authority_index: None,
                    output_program_index: 0,
                    output_authority_key_index: 0,
                    output_scope_index: 0,
                    output_label_index: 0,
                    output_authority_seeds: vec![],
                    // Allow index past the dictionary: malformed.
                    output_allow_indexes: vec![1],
                    previous_handle_index: None,
                    make_public: false,
                },
            }],
        };
        assert!(
            reconstruct_fhe_execute_steps(&execution, &[], &ctx()).is_none()
        );
    }
}
