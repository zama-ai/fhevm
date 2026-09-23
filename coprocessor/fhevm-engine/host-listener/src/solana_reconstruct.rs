//! Reconstruct zama-host operations and State effects from instruction data and
//! each execution's `FheExecutedEvent`.
//!
//! Covers the `fhe_execute` execution walk (records carry the result handles the host
//! emitted, and each is re-derived through the program's own `computed_*` functions as a
//! check), and the encrypted-state history the effects append (the keys they allow and
//! whether a result is made public: the leaf record of RFC 035).

use std::collections::HashSet;

use zama_host::state::{
    computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_sum_handle,
    computed_eval_ternary_handle, computed_eval_trivial_handle,
    computed_eval_unary_handle, computed_rand_bounded_handle,
    computed_rand_handle, FheExecuteArgs, FheExecuteEffect, FheExecuteOperand,
    FheExecuteStep, HandleDerivationContext,
};
use zama_host::FheExecutedEvent;

use crate::solana_adapter::SolanaHostRecord;
use zama_host::records::{
    FheBinaryOp, FheIsIn, FheMulDiv, FheRand, FheRandBounded, FheSum,
    FheTernaryOp, FheUnaryOp, TrivialEncrypt,
};
use zama_host::EVENT_VERSION;

use anchor_lang::prelude::Pubkey;
use anchor_lang::{AnchorDeserialize, Discriminator};

pub fn is_fhe_execute_instruction(instruction_data: &[u8]) -> bool {
    zama_host::decode::is_fhe_execute_instruction(instruction_data)
}

/// Decodes a `fhe_execute` instruction's data into the program's own `FheExecuteArgs`
/// execution through `zama_host::decode` (so there is no bespoke decoder to
/// drift from the on-chain layout). The decoded execution is the input to the execution
/// walk that reconstructs one op record per step, together with the execution's
/// `FheExecutedEvent`.
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

pub fn decode_fhe_executed_event(
    instruction_data: &[u8],
) -> Option<FheExecutedEvent> {
    let event: FheExecutedEvent =
        zama_host::decode::decode_event_cpi(instruction_data)?;
    (event.version == zama_host::EVENT_VERSION).then_some(event)
}

pub const MAKE_STATE_ENCRYPTED_STORE_INDEX: usize = 2;

pub fn is_make_store_handle_public_instruction(data: &[u8]) -> bool {
    data.get(..8)
        == Some(zama_host::instruction::MakeStoreHandlePublic::DISCRIMINATOR)
}

pub fn decode_make_store_handle_public(
    data: &[u8],
) -> Option<([u8; 32], [u8; 32], u64)> {
    if !is_make_store_handle_public_instruction(data) {
        return None;
    }
    let mut body = data.get(8..)?;
    let args =
        zama_host::instruction::MakeStoreHandlePublic::deserialize(&mut body)
            .ok()?;
    Some((args.key, args.handle, args.previous_leaf_count))
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

/// Resolves an execution operand to its handle from the emitted results of earlier
/// steps and the execution's interned dictionary — no on-chain account reads.
/// `Scalar` is only valid as a binary rhs (handled by [`resolve_rhs`]); seeing
/// it here means a malformed execution.
fn resolve_operand(
    operand: &FheExecuteOperand,
    dictionary: &[[u8; 32]],
    produced: &[[u8; 32]],
) -> Option<[u8; 32]> {
    match operand {
        FheExecuteOperand::StoreSlot { handle_index, .. }
        | FheExecuteOperand::TransientResult { handle_index, .. } => {
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

/// A step whose result handle, re-derived from the decoded step and the emitted
/// context, differs from the handle the host emitted for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandleMismatch {
    pub step_index: u16,
    pub emitted: [u8; 32],
    pub derived: [u8; 32],
}

/// Reconstructs the per-step op records of one `fhe_execute` from its instruction data
/// and its `FheExecutedEvent`, mirroring the program's `walk_steps`.
///
/// Every record carries the handle the host emitted, and operands resolve to the emitted
/// handles of earlier steps, so a record names the chain's handles even where this
/// listener's derivation disagrees. Each result is also re-derived through the program's
/// own `computed_*` functions from the decoded step and the emitted context; a result
/// that differs is reported in `mismatches`, never substituted. Random steps use the
/// emitted seeds, which bind the host's rand nonce and cannot be recomputed.
///
/// Returns `None` when the instruction and the event do not describe the same walk: an
/// operand or dictionary reference out of range, a `Scalar` where only an encrypted
/// operand is valid, a result count other than the step count, or seeds that are not
/// exactly the random steps. `program_id` is the deployment followed, not the id this
/// crate was compiled with; `chain_id` comes from the on-chain `HostConfig`.
pub fn reconstruct_fhe_execute(
    execution: &FheExecuteArgs,
    event: &FheExecutedEvent,
    program_id: Pubkey,
    chain_id: u64,
    produced_in_tx: &mut HashSet<[u8; 32]>,
) -> Option<ReconstructedExecution> {
    let random_steps = execution
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
    if event.results.len() != execution.steps.len()
        || random_steps.len() != event.seeds.len()
        || random_steps
            .iter()
            .zip(&event.seeds)
            .any(|(expected, actual)| *expected != actual.step_index)
    {
        return None;
    }
    let ctx = HandleDerivationContext {
        program_id,
        chain_id,
        previous_bank_hash: event.previous_bank_hash,
        unix_timestamp: event.unix_timestamp,
    };
    let seed_of = |op_index: u16| {
        event
            .seeds
            .iter()
            .find(|entry| entry.step_index == op_index)
            .map(|entry| entry.seed)
    };
    let dictionary = &execution.dictionary;
    let mut records: Vec<SolanaHostRecord> =
        Vec::with_capacity(execution.steps.len());
    let mut mismatches = Vec::new();

    for (index, step) in execution.steps.iter().enumerate() {
        let op_index = index as u16;
        let earlier = &event.results[..index];
        let result = event.results[index];
        let (record, derived) = match step {
            FheExecuteStep::Binary {
                op,
                lhs,
                rhs,
                output_fhe_type,
                ..
            } => {
                let lhs_handle = resolve_operand(lhs, dictionary, earlier)?;
                let (rhs_handle, scalar) =
                    resolve_rhs(rhs, dictionary, earlier)?;
                let derived = computed_eval_handle(
                    *op,
                    lhs_handle,
                    rhs_handle,
                    scalar,
                    *output_fhe_type,
                    boundary_mask(
                        [Some(lhs_handle), (!scalar).then_some(rhs_handle)],
                        produced_in_tx,
                    )?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheBinaryOp(FheBinaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    lhs: lhs_handle,
                    rhs: rhs_handle,
                    scalar,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::Ternary {
                op,
                control,
                if_true,
                if_false,
                output_fhe_type,
                ..
            } => {
                let c = resolve_operand(control, dictionary, earlier)?;
                let t = resolve_operand(if_true, dictionary, earlier)?;
                let f = resolve_operand(if_false, dictionary, earlier)?;
                let derived = computed_eval_ternary_handle(
                    *op,
                    c,
                    t,
                    f,
                    *output_fhe_type,
                    boundary_mask([Some(c), Some(t), Some(f)], produced_in_tx)?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheTernaryOp(FheTernaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    control: c,
                    if_true: t,
                    if_false: f,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => {
                let derived =
                    computed_eval_trivial_handle(*plaintext, *fhe_type, &ctx);
                let record = SolanaHostRecord::TrivialEncrypt(TrivialEncrypt {
                    version: EVENT_VERSION,
                    plaintext: *plaintext,
                    fhe_type: *fhe_type,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::Rand { fhe_type, .. } => {
                let seed = seed_of(op_index)?;
                let derived =
                    computed_rand_handle(seed, *fhe_type, program_id, chain_id);
                let record = SolanaHostRecord::FheRand(FheRand {
                    version: EVENT_VERSION,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::Unary {
                op,
                operand,
                output_fhe_type,
            } => {
                let operand_handle =
                    resolve_operand(operand, dictionary, earlier)?;
                let derived = computed_eval_unary_handle(
                    *op,
                    operand_handle,
                    *output_fhe_type,
                    boundary_mask([Some(operand_handle)], produced_in_tx)?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheUnaryOp(FheUnaryOp {
                    version: EVENT_VERSION,
                    op: *op,
                    operand: operand_handle,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type,
                ..
            } => {
                let seed = seed_of(op_index)?;
                let derived = computed_rand_bounded_handle(
                    *upper_bound,
                    seed,
                    *fhe_type,
                    program_id,
                    chain_id,
                );
                let record = SolanaHostRecord::FheRandBounded(FheRandBounded {
                    version: EVENT_VERSION,
                    upper_bound: *upper_bound,
                    seed,
                    fhe_type: *fhe_type,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::Sum { operands, fhe_type } => {
                let operand_handles: Vec<[u8; 32]> = operands
                    .iter()
                    .map(|operand| {
                        resolve_operand(operand, dictionary, earlier)
                    })
                    .collect::<Option<_>>()?;
                let derived = computed_eval_sum_handle(
                    &operand_handles,
                    *fhe_type,
                    boundary_mask(
                        operand_handles.iter().copied().map(Some),
                        produced_in_tx,
                    )?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheSum(FheSum {
                    version: EVENT_VERSION,
                    operands: operand_handles,
                    fhe_type: *fhe_type,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::IsIn {
                value,
                set,
                fhe_type,
            } => {
                let value_handle = resolve_operand(value, dictionary, earlier)?;
                let set_handles: Vec<[u8; 32]> = set
                    .iter()
                    .map(|operand| {
                        resolve_operand(operand, dictionary, earlier)
                    })
                    .collect::<Option<_>>()?;
                let derived = computed_eval_is_in_handle(
                    value_handle,
                    &set_handles,
                    *fhe_type,
                    boundary_mask(
                        std::iter::once(Some(value_handle))
                            .chain(set_handles.iter().copied().map(Some)),
                        produced_in_tx,
                    )?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheIsIn(FheIsIn {
                    version: EVENT_VERSION,
                    value: value_handle,
                    set: set_handles,
                    fhe_type: *fhe_type,
                    result,
                });
                (record, derived)
            }
            FheExecuteStep::MulDiv {
                factor1,
                factor2,
                divisor,
                output_fhe_type,
            } => {
                let factor1_handle =
                    resolve_operand(factor1, dictionary, earlier)?;
                let (factor2_handle, scalar) =
                    resolve_rhs(factor2, dictionary, earlier)?;
                let derived = computed_eval_mul_div_handle(
                    factor1_handle,
                    factor2_handle,
                    *divisor,
                    scalar,
                    *output_fhe_type,
                    boundary_mask(
                        [
                            Some(factor1_handle),
                            (!scalar).then_some(factor2_handle),
                        ],
                        produced_in_tx,
                    )?,
                    &ctx,
                );
                let record = SolanaHostRecord::FheMulDiv(FheMulDiv {
                    version: EVENT_VERSION,
                    factor1: factor1_handle,
                    factor2: factor2_handle,
                    divisor: *divisor,
                    scalar,
                    result,
                });
                (record, derived)
            }
        };
        if derived != result {
            mismatches.push(HandleMismatch {
                step_index: op_index,
                emitted: result,
                derived,
            });
        }
        produced_in_tx.insert(result);
        records.push(record);
    }
    let mut store_outputs = Vec::new();
    for effect in &execution.effects {
        if effect.result.output_index != 0 {
            return None;
        }
        let handle =
            *event.results.get(usize::from(effect.result.step_index))?;
        // A computation-only grant does not ask the worker to persist its result.
        if effect.slot.is_some()
            || !effect.allow_indexes.is_empty()
            || effect.make_public
        {
            store_outputs.push(state_leaf_output(effect, dictionary, handle)?);
        }
    }
    Some(ReconstructedExecution {
        records,
        store_outputs,
        mismatches,
    })
}

fn boundary_mask(
    operands: impl IntoIterator<Item = Option<[u8; 32]>>,
    produced: &HashSet<[u8; 32]>,
) -> Option<[u8; 32]> {
    zama_host::operand_boundary_mask(operands.into_iter().map(|operand| {
        operand.is_some_and(|handle| !produced.contains(&handle))
    }))
    .ok()
}

pub struct ReconstructedExecution {
    pub records: Vec<SolanaHostRecord>,
    pub store_outputs: Vec<StoreLeafOutput>,
    pub mismatches: Vec<HandleMismatch>,
}

/// The leaves a `State` output appends, read from instruction data the host
/// validated before accepting the confirmed transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreLeafOutput {
    /// Index into `remaining_accounts` of the `EncryptedStore` PDA.
    pub store_index: u8,
    /// The state's leaf count before this output.
    pub previous_leaf_count: u64,
    pub handle: [u8; 32],
    /// Keys allowed on `handle`, one historical-access leaf each, in this order.
    pub allowed_keys: Vec<[u8; 32]>,
    /// Whether a public-decrypt leaf for `handle` follows the allow leaves.
    pub make_public: bool,
}

fn state_leaf_output(
    effect: &FheExecuteEffect,
    dictionary: &[[u8; 32]],
    handle: [u8; 32],
) -> Option<StoreLeafOutput> {
    Some(StoreLeafOutput {
        store_index: effect.store_index,
        previous_leaf_count: effect.previous_leaf_count,
        handle,
        allowed_keys: effect
            .allow_indexes
            .iter()
            .map(|index| dictionary.get(usize::from(*index)).copied())
            .collect::<Option<Vec<_>>>()?,
        make_public: effect.make_public,
    })
}

/// The event a host would emit for `execution`: its results are this listener's own
/// derivation, step by step, so tests can build consistent instructions without a runtime.
/// `produced_in_tx` holds the results of earlier executions in the same transaction.
#[cfg(test)]
pub(crate) fn event_with_derived_results(
    execution: &FheExecuteArgs,
    ctx: &HandleDerivationContext,
    seeds: Vec<zama_host::FheExecuteRandomSeed>,
    produced_in_tx: &HashSet<[u8; 32]>,
) -> FheExecutedEvent {
    let mut event = FheExecutedEvent {
        version: EVENT_VERSION,
        previous_bank_hash: ctx.previous_bank_hash,
        unix_timestamp: ctx.unix_timestamp,
        results: vec![[0; 32]; execution.steps.len()],
        seeds,
    };
    for index in 0..execution.steps.len() {
        let rebuilt = reconstruct_fhe_execute(
            execution,
            &event,
            ctx.program_id,
            ctx.chain_id,
            &mut produced_in_tx.clone(),
        )
        .expect("a well-formed execution");
        if let Some(mismatch) = rebuilt
            .mismatches
            .iter()
            .find(|mismatch| usize::from(mismatch.step_index) == index)
        {
            event.results[index] = mismatch.derived;
        }
    }
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::{prelude::Pubkey, AnchorSerialize};
    use zama_host::state::{FheBinaryOpCode, FheUnaryOpCode};

    fn ctx() -> HandleDerivationContext {
        HandleDerivationContext {
            // Not the compiled-in id: every expected handle below must follow this one.
            program_id: "7DYCAhqwQSKqqL1h8V1XmY1BTcMWxrASQYKNMy87jeg3"
                .parse()
                .unwrap(),
            chain_id: zama_host::SOLANA_POC_CHAIN_ID,
            previous_bank_hash: [3u8; 32],
            unix_timestamp: 1_700_000_000,
        }
    }

    /// Reconstructs `execution` against the event a host would emit for it.
    fn walk(
        execution: &FheExecuteArgs,
        seeds: Vec<zama_host::FheExecuteRandomSeed>,
    ) -> ReconstructedExecution {
        let event = event_with_derived_results(
            execution,
            &ctx(),
            seeds,
            &HashSet::new(),
        );
        let rebuilt = reconstruct_fhe_execute(
            execution,
            &event,
            ctx().program_id,
            ctx().chain_id,
            &mut HashSet::new(),
        )
        .expect("walk");
        assert!(rebuilt.mismatches.is_empty());
        rebuilt
    }

    fn event(
        results: Vec<[u8; 32]>,
        seeds: Vec<zama_host::FheExecuteRandomSeed>,
    ) -> FheExecutedEvent {
        FheExecutedEvent {
            version: EVENT_VERSION,
            previous_bank_hash: ctx().previous_bank_hash,
            unix_timestamp: ctx().unix_timestamp,
            results,
            seeds,
        }
    }

    /// An event of the right shape whose results are placeholders.
    fn placeholder_event(execution: &FheExecuteArgs) -> FheExecutedEvent {
        event(vec![[0; 32]; execution.steps.len()], vec![])
    }

    fn event_instruction_data(event: &FheExecutedEvent) -> Vec<u8> {
        anchor_lang::event::EVENT_IX_TAG_LE
            .iter()
            .copied()
            .chain(anchor_lang::Event::data(event))
            .collect()
    }

    #[test]
    fn decodes_state_public_args_from_program_type() {
        let args = zama_host::instruction::MakeStoreHandlePublic {
            key: [1; 32],
            handle: [2; 32],
            previous_leaf_count: 7,
        };
        let mut data =
            zama_host::instruction::MakeStoreHandlePublic::DISCRIMINATOR
                .to_vec();
        args.serialize(&mut data).unwrap();
        assert!(is_make_store_handle_public_instruction(&data));
        assert_eq!(
            decode_make_store_handle_public(&data),
            Some(([1; 32], [2; 32], 7))
        );
        data.pop();
        assert_eq!(decode_make_store_handle_public(&data), None);

        // A create-state payload starts with enough fixed-width bytes to deserialize as the
        // public-seal arguments if the discriminator is ignored. It must never fabricate a
        // history write.
        let create = zama_host::instruction::CreateEncryptedStore {
            args: zama_host::instructions::CreateEncryptedStoreArgs {
                program: Pubkey::new_unique(),
                scope: [4; 32],
                authority_seeds: vec![vec![12; 4]],
            },
        };
        let mut create_data =
            zama_host::instruction::CreateEncryptedStore::DISCRIMINATOR
                .to_vec();
        create.serialize(&mut create_data).unwrap();
        assert_eq!(decode_make_store_handle_public(&create_data), None);
    }

    #[test]
    fn fhe_execute_batch_round_trips_via_program_type() {
        use anchor_lang::AnchorSerialize;
        use zama_host::state::{
            FheBinaryOpCode, FheExecuteArgs, FheExecuteOperand, FheExecuteStep,
        };
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],

            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
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
    fn decodes_the_executed_event_and_rejects_other_versions() {
        let mut event = FheExecutedEvent {
            version: EVENT_VERSION,
            previous_bank_hash: [3; 32],
            unix_timestamp: 1_700_000_000,
            results: vec![[5; 32]],
            seeds: vec![zama_host::FheExecuteRandomSeed {
                step_index: 0,
                seed: [7; 16],
            }],
        };
        let decoded =
            decode_fhe_executed_event(&event_instruction_data(&event))
                .expect("decode event");
        assert_eq!(decoded.results, event.results);
        assert_eq!(decoded.seeds, event.seeds);

        event.version = EVENT_VERSION.wrapping_add(1);
        assert!(decode_fhe_executed_event(&event_instruction_data(&event))
            .is_none());
    }

    #[test]
    fn fhe_execute_walk_chains_transient_handles() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],

            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                },
            ],
        };
        let records = walk(&execution, vec![]).records;
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
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: Some(zama_host::SlotWrite {
                    key_index: 2,
                    previous_handle_index: None,
                }),
                allow_indexes: vec![3],
                make_public: false,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![[0xA1; 32], [0xA2; 32], [0xA3; 32], [0xA4; 32]],
            steps: vec![FheExecuteStep::RandBounded {
                upper_bound,
                fhe_type: 5,
            }],
        };

        let records = walk(
            &execution,
            vec![zama_host::FheExecuteRandomSeed {
                step_index: 0,
                seed: [7; 16],
            }],
        )
        .records;
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
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 6,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: Some(zama_host::SlotWrite {
                    key_index: 3,
                    previous_handle_index: None,
                }),
                allow_indexes: vec![4],
                make_public: false,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![
                [2u8; 32], [0xA1; 32], [0xA2; 32], [0xA3; 32], [0xA4; 32],
            ],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [9u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [4u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::Unary {
                    op: FheUnaryOpCode::Neg,
                    operand: FheExecuteOperand::EarlierStep {
                        producer_index: 0,
                    },
                    output_fhe_type: 5,
                },
                FheExecuteStep::Sum {
                    operands: vec![
                        FheExecuteOperand::EarlierStep { producer_index: 0 },
                        FheExecuteOperand::EarlierStep { producer_index: 1 },
                    ],
                    fhe_type: 5,
                },
                FheExecuteStep::IsIn {
                    value: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    set: vec![FheExecuteOperand::EarlierStep {
                        producer_index: 1,
                    }],
                    fhe_type: 5,
                },
                FheExecuteStep::MulDiv {
                    factor1: FheExecuteOperand::EarlierStep {
                        producer_index: 0,
                    },
                    factor2: FheExecuteOperand::Scalar { value_index: 0 },
                    divisor: [3u8; 32],
                    output_fhe_type: 5,
                },
                FheExecuteStep::RandBounded {
                    upper_bound: ub,
                    fhe_type: 5,
                },
            ],
        };
        let random_seed = [9; 16];
        let records = walk(
            &execution,
            vec![zama_host::FheExecuteRandomSeed {
                step_index: 6,
                seed: random_seed,
            }],
        )
        .records;
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
                    computed_eval_unary_handle(
                        FheUnaryOpCode::Neg,
                        h0,
                        5,
                        [0; 32],
                        &cx,
                    )
                );
            }
            other => panic!("expected FheUnaryOp, got {other:?}"),
        }
        match &records[3] {
            SolanaHostRecord::FheSum(e) => {
                assert_eq!(e.operands, vec![h0, h1]);
                assert_eq!(
                    e.result,
                    computed_eval_sum_handle(&[h0, h1], 5, [0; 32], &cx,)
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
                    computed_eval_is_in_handle(h0, &[h1], 5, [0; 32], &cx,)
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
                        h0, [2u8; 32], [3u8; 32], true, 5, [0; 32], &cx,
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
                        cx.program_id,
                        cx.chain_id
                    )
                );
            }
            other => panic!("expected FheRandBounded, got {other:?}"),
        }
    }

    /// A wrong emitted handle is reported for its step alone. The record keeps the
    /// emitted handle, and a later step consuming it is re-derived from that handle, so
    /// the mismatch does not cascade.
    #[test]
    fn reports_a_wrong_emitted_handle_without_substituting_it() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],
            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![
                FheExecuteStep::TrivialEncrypt {
                    plaintext: [7u8; 32],
                    fhe_type: 5,
                },
                FheExecuteStep::Binary {
                    op: FheBinaryOpCode::Add,
                    lhs: FheExecuteOperand::EarlierStep { producer_index: 0 },
                    rhs: FheExecuteOperand::Scalar { value_index: 0 },
                    output_fhe_type: 5,
                },
            ],
        };
        let honest = event_with_derived_results(
            &execution,
            &ctx(),
            vec![],
            &HashSet::new(),
        );
        let wrong = [0xEE; 32];
        let consumer = computed_eval_handle(
            FheBinaryOpCode::Add,
            wrong,
            [2u8; 32],
            true,
            5,
            [0; 32],
            &ctx(),
        );
        let event = event(vec![wrong, consumer], vec![]);

        let rebuilt = reconstruct_fhe_execute(
            &execution,
            &event,
            ctx().program_id,
            ctx().chain_id,
            &mut HashSet::new(),
        )
        .expect("walk");
        assert_eq!(
            rebuilt.mismatches,
            vec![HandleMismatch {
                step_index: 0,
                emitted: wrong,
                derived: honest.results[0],
            }]
        );
        match &rebuilt.records[..] {
            [SolanaHostRecord::TrivialEncrypt(first), SolanaHostRecord::FheBinaryOp(second)] =>
            {
                assert_eq!(first.result, wrong);
                assert_eq!(second.lhs, wrong);
                assert_eq!(second.result, consumer);
            }
            other => panic!("unexpected records {other:?}"),
        }
    }

    #[test]
    fn rejects_an_event_that_describes_other_steps() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],
            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![],
            steps: vec![FheExecuteStep::Rand { fhe_type: 5 }],
        };
        let seed = zama_host::FheExecuteRandomSeed {
            step_index: 0,
            seed: [7; 16],
        };
        let result = event_with_derived_results(
            &execution,
            &ctx(),
            vec![seed.clone()],
            &HashSet::new(),
        )
        .results;
        let malformed = [
            event(vec![], vec![seed.clone()]),
            event(result.clone(), vec![]),
            event(
                result,
                vec![zama_host::FheExecuteRandomSeed {
                    step_index: 1,
                    ..seed
                }],
            ),
        ];
        for event in malformed {
            assert!(reconstruct_fhe_execute(
                &execution,
                &event,
                ctx().program_id,
                ctx().chain_id,
                &mut HashSet::new(),
            )
            .is_none());
        }
    }

    #[test]
    fn fhe_execute_walk_rejects_forward_transient_reference() {
        // A first step referencing a not-yet-produced step -> None.
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],

            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![[2u8; 32]],
            steps: vec![FheExecuteStep::Binary {
                op: FheBinaryOpCode::Add,
                lhs: FheExecuteOperand::EarlierStep { producer_index: 5 },
                rhs: FheExecuteOperand::Scalar { value_index: 0 },
                output_fhe_type: 5,
            }],
        };
        assert!(reconstruct_fhe_execute(
            &execution,
            &placeholder_event(&execution),
            ctx().program_id,
            ctx().chain_id,
            &mut HashSet::new()
        )
        .map(|execution| execution.records)
        .is_none());
    }

    /// A `State` output exposes only proof-history inputs; slot and grant policy
    /// do not enter leaf reconstruction.
    #[test]
    fn fhe_execute_walk_extracts_store_output() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 3,
                previous_leaf_count: 9,
                slot: None,
                allow_indexes: vec![0, 1],
                make_public: true,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![[0xB1; 32], [0xB2; 32]],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
            }],
        };
        let steps = walk(&execution, vec![]);
        let handle = match &steps.records[0] {
            SolanaHostRecord::TrivialEncrypt(e) => e.result,
            other => panic!("expected TrivialEncrypt, got {other:?}"),
        };
        assert_eq!(
            steps.store_outputs,
            vec![StoreLeafOutput {
                store_index: 3,
                previous_leaf_count: 9,
                handle,
                allowed_keys: vec![[0xB1; 32], [0xB2; 32]],
                make_public: true,
            }]
        );
    }

    #[test]
    fn transient_has_no_store_output() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],

            returned_results: Vec::new(),
            account_count: 0,
            dictionary: vec![],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
            }],
        };
        let steps = walk(&execution, vec![]);
        assert!(steps.store_outputs.is_empty());
    }

    #[test]
    fn rejects_store_output_dictionary_overflow() {
        let execution = FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![zama_host::FheExecuteEffect {
                result: zama_host::ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count: 0,
                slot: None,
                allow_indexes: vec![1],
                make_public: false,
                grants: vec![],
            }],

            returned_results: Vec::new(),
            account_count: 1,
            dictionary: vec![[0xA1; 32]],
            steps: vec![FheExecuteStep::TrivialEncrypt {
                plaintext: [7u8; 32],
                fhe_type: 5,
            }],
        };
        assert!(reconstruct_fhe_execute(
            &execution,
            &placeholder_event(&execution),
            ctx().program_id,
            ctx().chain_id,
            &mut HashSet::new()
        )
        .is_none());
    }
}
