//! Reconstruct zama-host operations from an `fhe_execute` and its `FheExecutedEvent`.
//!
//! The execution walk emits one computation per step. Computations carry the result handles
//! the host emitted, and each is re-derived through the program's own `computed_*` functions
//! as a check.

use std::collections::HashSet;

use zama_host::state::{
    computed_eval_handle, computed_eval_is_in_handle,
    computed_eval_mul_div_handle, computed_eval_sum_handle,
    computed_eval_ternary_handle, computed_eval_trivial_handle,
    computed_eval_unary_handle, computed_rand_bounded_handle,
    computed_rand_handle, FheExecuteArgs, FheExecuteOperand, FheExecuteStep,
    HandleDerivationContext,
};
use zama_host::FheExecutedEvent;

use crate::database::computation::{Computation, Operand};
use fhevm_engine_common::types::SupportedFheOperations as O;
use zama_host::state::{FheBinaryOpCode, FheTernaryOpCode, FheUnaryOpCode};
#[cfg(test)]
use zama_host::EVENT_VERSION;
use Operand::{Clear as P, Encrypted as H};

use anchor_lang::prelude::Pubkey;

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

/// Reconstructs the per-step computations of one `fhe_execute` from its instruction data
/// and its `FheExecutedEvent`, mirroring the program's `walk_steps`.
///
/// Every computation carries the handle the host emitted, and operands resolve to the emitted
/// handles of earlier steps, so a computation names the chain's handles even where this
/// listener's derivation disagrees. Each result is also re-derived through the program's
/// own `computed_*` functions from the decoded step and the emitted context; a result
/// that differs is reported in `mismatches`, never substituted. Random steps use the
/// emitted seeds, which bind the host's rand nonce and cannot be recomputed.
///
/// `host_operations` already checked that the event holds one result per step and one seed
/// per random step. Returns `None` when the walk still does not fit: an operand or dictionary
/// reference out of range, or a `Scalar` where only an encrypted operand is valid.
/// `program_id` is the deployment followed, not the id this crate was compiled with;
/// `chain_id` comes from the on-chain `HostConfig`.
pub fn reconstruct_fhe_execute(
    execution: &FheExecuteArgs,
    event: &FheExecutedEvent,
    program_id: Pubkey,
    chain_id: u64,
    produced_in_tx: &mut HashSet<[u8; 32]>,
) -> Option<ReconstructedExecution> {
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
    let mut computations = Vec::with_capacity(execution.steps.len());
    let mut mismatches = Vec::new();

    for (index, step) in execution.steps.iter().enumerate() {
        let op_index = index as u16;
        let earlier = event.results.get(..index)?;
        let result = *event.results.get(index)?;
        let (computation, derived) = match step {
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
                let computation = binary_computation(
                    *op, lhs_handle, rhs_handle, scalar, result,
                );
                (computation, derived)
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
                let computation = ternary_computation(*op, c, t, f, result);
                (computation, derived)
            }
            FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type,
                ..
            } => {
                let derived =
                    computed_eval_trivial_handle(*plaintext, *fhe_type, &ctx);
                let computation =
                    Computation::trivial(*plaintext, *fhe_type, result.into());
                (computation, derived)
            }
            FheExecuteStep::Rand { fhe_type, .. } => {
                let seed = seed_of(op_index)?;
                let derived =
                    computed_rand_handle(seed, *fhe_type, program_id, chain_id);
                let computation = random_computation(seed, *fhe_type, result);
                (computation, derived)
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
                let computation =
                    unary_computation(*op, operand_handle, result);
                (computation, derived)
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
                let computation = bounded_random_computation(
                    seed,
                    *upper_bound,
                    *fhe_type,
                    result,
                );
                (computation, derived)
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
                let computation = sum_computation(operand_handles, result);
                (computation, derived)
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
                let computation =
                    is_in_computation(value_handle, set_handles, result);
                (computation, derived)
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
                let computation = mul_div_computation(
                    factor1_handle,
                    factor2_handle,
                    *divisor,
                    scalar,
                    result,
                );
                (computation, derived)
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
        computations.push(computation);
    }
    Some(ReconstructedExecution {
        computations,
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
    pub computations: Vec<Computation>,
    pub mismatches: Vec<HandleMismatch>,
}

fn binary_computation(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    result: [u8; 32],
) -> Computation {
    let operation = match op {
        FheBinaryOpCode::Add => O::FheAdd,
        FheBinaryOpCode::Sub => O::FheSub,
        FheBinaryOpCode::Mul => O::FheMul,
        FheBinaryOpCode::Div => O::FheDiv,
        FheBinaryOpCode::Rem => O::FheRem,
        FheBinaryOpCode::And => O::FheBitAnd,
        FheBinaryOpCode::Or => O::FheBitOr,
        FheBinaryOpCode::Xor => O::FheBitXor,
        FheBinaryOpCode::Shl => O::FheShl,
        FheBinaryOpCode::Shr => O::FheShr,
        FheBinaryOpCode::Rotl => O::FheRotl,
        FheBinaryOpCode::Rotr => O::FheRotr,
        FheBinaryOpCode::Eq => O::FheEq,
        FheBinaryOpCode::Ne => O::FheNe,
        FheBinaryOpCode::Ge => O::FheGe,
        FheBinaryOpCode::Gt => O::FheGt,
        FheBinaryOpCode::Le => O::FheLe,
        FheBinaryOpCode::Lt => O::FheLt,
        FheBinaryOpCode::Min => O::FheMin,
        FheBinaryOpCode::Max => O::FheMax,
    };
    Computation::single(
        operation,
        vec![H(lhs.into()), Operand::binary_rhs(rhs.into(), scalar)],
        result.into(),
    )
    .expect("fixed-width operands")
}

fn ternary_computation(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    result: [u8; 32],
) -> Computation {
    let operation = match op {
        FheTernaryOpCode::IfThenElse => O::FheIfThenElse,
    };
    Computation::single(
        operation,
        vec![H(control.into()), H(if_true.into()), H(if_false.into())],
        result.into(),
    )
    .expect("fixed-width operands")
}

fn random_computation(
    seed: [u8; 16],
    fhe_type: u8,
    result: [u8; 32],
) -> Computation {
    Computation::single(
        O::FheRand,
        vec![P(seed.to_vec()), P(vec![fhe_type])],
        result.into(),
    )
    .expect("fixed-width operands")
}

fn bounded_random_computation(
    seed: [u8; 16],
    upper_bound: [u8; 32],
    fhe_type: u8,
    result: [u8; 32],
) -> Computation {
    Computation::single(
        O::FheRandBounded,
        vec![P(seed.to_vec()), P(upper_bound.to_vec()), P(vec![fhe_type])],
        result.into(),
    )
    .expect("fixed-width operands")
}

fn unary_computation(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    result: [u8; 32],
) -> Computation {
    let mut operands = vec![H(operand.into())];
    let operation = match op {
        FheUnaryOpCode::Neg => O::FheNeg,
        FheUnaryOpCode::Not => O::FheNot,
        FheUnaryOpCode::Cast => {
            operands.push(P(vec![result[30]]));
            O::FheCast
        }
    };
    Computation::single(operation, operands, result.into())
        .expect("fixed-width operands")
}

fn sum_computation(operands: Vec<[u8; 32]>, result: [u8; 32]) -> Computation {
    Computation::single(
        O::FheSum,
        operands.into_iter().map(Operand::encrypted).collect(),
        result.into(),
    )
    .expect("boundary_mask caps operands at 256")
}

fn is_in_computation(
    value: [u8; 32],
    set: Vec<[u8; 32]>,
    result: [u8; 32],
) -> Computation {
    Computation::single(
        O::FheIsIn,
        std::iter::once(value)
            .chain(set)
            .map(Operand::encrypted)
            .collect(),
        result.into(),
    )
    .expect("boundary_mask caps operands at 256")
}

fn mul_div_computation(
    factor1: [u8; 32],
    factor2: [u8; 32],
    divisor: [u8; 32],
    scalar: bool,
    result: [u8; 32],
) -> Computation {
    Computation::single(
        O::FheMulDiv,
        vec![
            H(factor1.into()),
            Operand::binary_rhs(factor2.into(), scalar),
            P(divisor.to_vec()),
        ],
        result.into(),
    )
    .expect("fixed-width operands")
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
    use crate::database::tfhe_event_propagate::Handle;
    use alloy_primitives::FixedBytes;

    fn handle(byte: u8) -> Handle {
        Handle::from([byte; 32])
    }

    #[test]
    fn maps_binary_add_to_computation() {
        let mapped = binary_computation(
            FheBinaryOpCode::Add,
            [1; 32],
            [2; 32],
            false,
            [3; 32],
        );

        assert_eq!(
            mapped,
            Computation::single(
                O::FheAdd,
                vec![H(handle(1)), H(handle(2))],
                handle(3)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_binary_ge_to_computation() {
        let mapped = binary_computation(
            FheBinaryOpCode::Ge,
            [1; 32],
            [2; 32],
            false,
            [3; 32],
        );

        assert_eq!(
            mapped,
            Computation::single(
                O::FheGe,
                vec![H(handle(1)), H(handle(2))],
                handle(3)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_ternary_if_then_else_to_computation() {
        let mapped = ternary_computation(
            FheTernaryOpCode::IfThenElse,
            [1; 32],
            [2; 32],
            [3; 32],
            [4; 32],
        );

        assert_eq!(
            mapped,
            Computation::single(
                O::FheIfThenElse,
                vec![H(handle(1)), H(handle(2)), H(handle(3))],
                handle(4)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_trivial_encrypt_to_computation() {
        let mut plaintext = [0_u8; 32];
        plaintext[31] = 7;

        let mapped = Computation::trivial(plaintext, 5, handle(8));

        assert_eq!(
            mapped,
            Computation::single(
                O::FheTrivialEncrypt,
                vec![P(plaintext.to_vec()), P(vec![5])],
                handle(8)
            )
            .unwrap()
        );
    }

    #[test]
    fn maps_random_to_computation() {
        let mapped = random_computation([7; 16], 5, [8; 32]);

        assert_eq!(
            mapped,
            Computation::single(
                O::FheRand,
                vec![P(vec![7; 16]), P(vec![5])],
                handle(8)
            )
            .unwrap()
        );
    }

    #[test]
    fn binary_decoders_agree_on_operation_operand_order_and_scalar_encoding() {
        use crate::contracts::{
            TfheContract as C, TfheContract::TfheContractEvents as E,
        };
        let caller = alloy_primitives::Address::ZERO;
        let lhs = handle(1);
        let rhs = handle(2);
        let result = handle(3);
        for scalar in [false, true] {
            let scalar_byte = FixedBytes::from([u8::from(scalar)]);
            macro_rules! case {
                ($op:ident, $event:ident) => {
                    (
                        FheBinaryOpCode::$op,
                        O::$event,
                        E::$event(C::$event {
                            caller,
                            lhs,
                            rhs,
                            scalarByte: scalar_byte,
                            result,
                        }),
                    )
                };
            }
            let cases = [
                case!(Add, FheAdd),
                case!(Sub, FheSub),
                case!(Mul, FheMul),
                case!(Div, FheDiv),
                case!(Rem, FheRem),
                case!(And, FheBitAnd),
                case!(Or, FheBitOr),
                case!(Xor, FheBitXor),
                case!(Shl, FheShl),
                case!(Shr, FheShr),
                case!(Rotl, FheRotl),
                case!(Rotr, FheRotr),
                case!(Eq, FheEq),
                case!(Ne, FheNe),
                case!(Ge, FheGe),
                case!(Gt, FheGt),
                case!(Le, FheLe),
                case!(Lt, FheLt),
                case!(Min, FheMin),
                case!(Max, FheMax),
            ];
            for (op, operation, evm) in cases {
                let solana =
                    binary_computation(op, lhs.0, rhs.0, scalar, result.0);
                assert_eq!(
                    solana,
                    Computation::from_evm(&evm).unwrap().unwrap()
                );
                assert_eq!(solana.operation(), operation);
                assert_eq!(
                    solana
                        .operands()
                        .iter()
                        .map(Operand::bytes)
                        .collect::<Vec<_>>(),
                    vec![lhs.to_vec(), rhs.to_vec()]
                );
                assert_eq!(solana.is_scalar(), scalar);
                assert_eq!(
                    solana.inputs(),
                    if scalar { vec![lhs] } else { vec![lhs, rhs] }
                );
                assert_eq!(
                    solana.boundary_mask(|_| false).unwrap()[31],
                    if scalar { 1 } else { 3 }
                );
            }
        }
    }

    #[test]
    fn native_special_operations_preserve_worker_operands() {
        let result = [8; 32];
        let cases = [
            (
                unary_computation(FheUnaryOpCode::Neg, [1; 32], result),
                O::FheNeg,
                vec![H(handle(1))],
                false,
            ),
            (
                unary_computation(FheUnaryOpCode::Not, [1; 32], result),
                O::FheNot,
                vec![H(handle(1))],
                false,
            ),
            (
                unary_computation(FheUnaryOpCode::Cast, [1; 32], result),
                O::FheCast,
                vec![H(handle(1)), P(vec![8])],
                true,
            ),
            (
                bounded_random_computation([1; 16], [2; 32], 5, result),
                O::FheRandBounded,
                vec![P(vec![1; 16]), P(vec![2; 32]), P(vec![5])],
                true,
            ),
            (
                sum_computation(vec![[1; 32], [2; 32]], result),
                O::FheSum,
                vec![H(handle(1)), H(handle(2))],
                false,
            ),
            (
                is_in_computation([1; 32], vec![[2; 32], [3; 32]], result),
                O::FheIsIn,
                vec![H(handle(1)), H(handle(2)), H(handle(3))],
                false,
            ),
            (
                mul_div_computation([1; 32], [2; 32], [3; 32], false, result),
                O::FheMulDiv,
                vec![H(handle(1)), H(handle(2)), P(vec![3; 32])],
                false,
            ),
            (
                mul_div_computation([1; 32], [2; 32], [3; 32], true, result),
                O::FheMulDiv,
                vec![H(handle(1)), P(vec![2; 32]), P(vec![3; 32])],
                true,
            ),
        ];
        for (computation, operation, operands, scalar) in cases {
            assert_eq!(
                computation,
                Computation::single(operation, operands, result.into())
                    .unwrap()
            );
            assert_eq!(computation.is_scalar(), scalar);
        }
    }

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
        let step0 = computed_eval_trivial_handle([7u8; 32], 5, &ctx());
        let step1 = computed_eval_handle(
            FheBinaryOpCode::Add,
            step0,
            [2u8; 32],
            true,
            5,
            [0; 32],
            &ctx(),
        );
        assert_eq!(
            walk(&execution, vec![]).computations,
            vec![
                Computation::trivial([7u8; 32], 5, step0.into()),
                // The EarlierStep operand resolved to step 0's produced handle.
                binary_computation(
                    FheBinaryOpCode::Add,
                    step0,
                    [2u8; 32],
                    true,
                    step1,
                ),
            ]
        );
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

        let computations = walk(
            &execution,
            vec![zama_host::FheExecuteRandomSeed {
                step_index: 0,
                seed: [7; 16],
            }],
        )
        .computations;
        let result = computed_rand_bounded_handle(
            upper_bound,
            [7; 16],
            5,
            ctx().program_id,
            ctx().chain_id,
        );
        assert_eq!(
            computations,
            vec![bounded_random_computation([7; 16], upper_bound, 5, result)]
        );
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
        let computations = walk(
            &execution,
            vec![zama_host::FheExecuteRandomSeed {
                step_index: 6,
                seed: random_seed,
            }],
        )
        .computations;
        let h0 = computed_eval_trivial_handle([9u8; 32], 5, &cx);
        let h1 = computed_eval_trivial_handle([4u8; 32], 5, &cx);
        // Each op resolves its transient operands to prior steps' handles and
        // derives the result via the program's own `computed_*` functions.
        assert_eq!(
            computations,
            vec![
                Computation::trivial([9u8; 32], 5, h0.into()),
                Computation::trivial([4u8; 32], 5, h1.into()),
                unary_computation(
                    FheUnaryOpCode::Neg,
                    h0,
                    computed_eval_unary_handle(
                        FheUnaryOpCode::Neg,
                        h0,
                        5,
                        [0; 32],
                        &cx,
                    ),
                ),
                sum_computation(
                    vec![h0, h1],
                    computed_eval_sum_handle(&[h0, h1], 5, [0; 32], &cx),
                ),
                is_in_computation(
                    h0,
                    vec![h1],
                    computed_eval_is_in_handle(h0, &[h1], 5, [0; 32], &cx),
                ),
                mul_div_computation(
                    h0,
                    [2u8; 32],
                    [3u8; 32],
                    true,
                    computed_eval_mul_div_handle(
                        h0, [2u8; 32], [3u8; 32], true, 5, [0; 32], &cx,
                    ),
                ),
                bounded_random_computation(
                    random_seed,
                    ub,
                    5,
                    computed_rand_bounded_handle(
                        ub,
                        random_seed,
                        5,
                        cx.program_id,
                        cx.chain_id,
                    ),
                ),
            ]
        );
    }

    /// A wrong emitted handle is reported for its step alone. The computation keeps the
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
        assert_eq!(
            rebuilt.computations,
            vec![
                Computation::trivial([7u8; 32], 5, wrong.into()),
                binary_computation(
                    FheBinaryOpCode::Add,
                    wrong,
                    [2u8; 32],
                    true,
                    consumer,
                ),
            ]
        );
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
        .is_none());
    }
}
