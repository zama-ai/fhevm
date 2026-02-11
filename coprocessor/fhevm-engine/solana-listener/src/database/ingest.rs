use anyhow::Result;
use fhevm_engine_common::types::{AllowEvents, SupportedFheOperations};
use sha3::{Digest, Keccak256};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

use crate::contracts::{FinalizedEventEnvelope, ProgramEvent};

#[derive(Clone, Debug)]
pub struct ComputationInsert {
    pub tenant_id: i32,
    pub output_handle: Vec<u8>,
    pub dependencies: Vec<Vec<u8>>,
    pub fhe_operation: i16,
    pub is_scalar: bool,
    pub dependence_chain_id: Vec<u8>,
    pub transaction_id: Option<Vec<u8>>,
    pub is_allowed: bool,
    pub schedule_order: PrimitiveDateTime,
    pub is_completed: bool,
}

#[derive(Clone, Debug)]
pub struct AllowedHandleInsert {
    pub tenant_id: i32,
    pub handle: Vec<u8>,
    pub account_address: String,
    pub event_type: i16,
    pub transaction_id: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct PbsComputationInsert {
    pub tenant_id: i32,
    pub handle: Vec<u8>,
    pub transaction_id: Option<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct BlockValidInsert {
    pub chain_id: i64,
    pub block_hash: Vec<u8>,
    pub block_number: i64,
}

#[derive(Clone, Debug)]
pub struct CursorUpdate {
    pub chain_id: i64,
    pub last_caught_up_block: i64,
}

#[derive(Clone, Debug, Default)]
pub struct IngestActions {
    pub computations: Vec<ComputationInsert>,
    pub allowed_handles: Vec<AllowedHandleInsert>,
    pub pbs_computations: Vec<PbsComputationInsert>,
    pub blocks_valid: Vec<BlockValidInsert>,
    pub cursor_update: Option<CursorUpdate>,
}

pub fn map_envelope_to_actions(
    envelope: &FinalizedEventEnvelope,
    tenant_id: i32,
) -> Result<IngestActions> {
    // Intentional PoC duplication with host-listener ingest semantics from
    // `host-listener/src/database/tfhe_event_propagate.rs`:
    // 1) computation insert contract (`computations`)
    // 2) ACL unlock flow (`allowed_handles` + `pbs_computations`)
    // 3) block/cursor persistence (`host_chain_blocks_valid`, poller state)
    // Keep these in sync to preserve cross-chain behavior parity.
    envelope.validate()?;

    let schedule_order = compute_schedule_order(
        envelope.block_time_unix,
        envelope.tx_index,
        envelope.op_index,
    )?;
    let tx_signature = Some(envelope.tx_signature.clone());

    let mut actions = IngestActions::default();
    let mut push_computation =
        |output_handle: &[_; 32],
         dependencies: Vec<Vec<u8>>,
         is_scalar: bool,
         fhe_operation: SupportedFheOperations| {
            actions.computations.push(ComputationInsert {
                tenant_id,
                output_handle: output_handle.to_vec(),
                dependencies,
                fhe_operation: fhe_operation as i16,
                is_scalar,
                dependence_chain_id: dependence_chain_from_signature(&envelope.tx_signature),
                transaction_id: tx_signature.clone(),
                // Keep ACL semantics explicit: requested operations are queued but
                // not runnable until a matching allow event unlocks the handle.
                is_allowed: false,
                schedule_order,
                is_completed: false,
            });
        };
    let mut push_binary = |lhs: &[_; 32],
                           rhs: &[_; 32],
                           is_scalar: bool,
                           result_handle: &[_; 32],
                           fhe_operation: SupportedFheOperations| {
        push_computation(
            result_handle,
            vec![lhs.to_vec(), rhs.to_vec()],
            is_scalar,
            fhe_operation,
        );
    };

    match &envelope.event {
        ProgramEvent::OpRequestedAdd {
            lhs,
            rhs,
            is_scalar,
            result_handle,
            ..
        } => push_binary(
            lhs,
            rhs,
            *is_scalar,
            result_handle,
            SupportedFheOperations::FheAdd,
        ),
        ProgramEvent::OpRequestedSub {
            lhs,
            rhs,
            is_scalar,
            result_handle,
            ..
        } => push_binary(
            lhs,
            rhs,
            *is_scalar,
            result_handle,
            SupportedFheOperations::FheSub,
        ),
        ProgramEvent::OpRequestedBinary {
            lhs,
            rhs,
            is_scalar,
            result_handle,
            opcode,
            ..
        } => push_binary(
            lhs,
            rhs,
            *is_scalar,
            result_handle,
            binary_opcode_to_operation(*opcode)?,
        ),
        ProgramEvent::OpRequestedUnary {
            input,
            result_handle,
            opcode,
            ..
        } => push_computation(
            result_handle,
            vec![input.to_vec()],
            false,
            unary_opcode_to_operation(*opcode)?,
        ),
        ProgramEvent::OpRequestedIfThenElse {
            control,
            if_true,
            if_false,
            result_handle,
            ..
        } => push_computation(
            result_handle,
            vec![control.to_vec(), if_true.to_vec(), if_false.to_vec()],
            false,
            SupportedFheOperations::FheIfThenElse,
        ),
        ProgramEvent::OpRequestedCast {
            input,
            to_type,
            result_handle,
            ..
        } => push_computation(
            result_handle,
            vec![input.to_vec(), vec![*to_type]],
            true,
            SupportedFheOperations::FheCast,
        ),
        ProgramEvent::OpRequestedTrivialEncrypt {
            pt,
            to_type,
            result_handle,
            ..
        } => push_computation(
            result_handle,
            vec![pt.to_vec(), vec![*to_type]],
            true,
            SupportedFheOperations::FheTrivialEncrypt,
        ),
        ProgramEvent::OpRequestedRand {
            rand_type,
            seed,
            result_handle,
            ..
        } => push_computation(
            result_handle,
            vec![seed.to_vec(), vec![*rand_type]],
            true,
            SupportedFheOperations::FheRand,
        ),
        ProgramEvent::OpRequestedRandBounded {
            upper_bound,
            rand_type,
            seed,
            result_handle,
            ..
        } => push_computation(
            result_handle,
            vec![seed.to_vec(), upper_bound.to_vec(), vec![*rand_type]],
            true,
            SupportedFheOperations::FheRandBounded,
        ),
        ProgramEvent::HandleAllowed {
            handle, account, ..
        } => {
            actions.allowed_handles.push(AllowedHandleInsert {
                tenant_id,
                handle: handle.to_vec(),
                account_address: hex::encode(account.to_bytes()),
                event_type: AllowEvents::AllowedAccount as i16,
                transaction_id: tx_signature.clone(),
            });
            actions.pbs_computations.push(PbsComputationInsert {
                tenant_id,
                handle: handle.to_vec(),
                transaction_id: tx_signature,
            });
        }
    };

    actions.blocks_valid.push(BlockValidInsert {
        chain_id: envelope.host_chain_id,
        block_hash: envelope.block_hash.clone(),
        block_number: envelope.slot as i64,
    });

    actions.cursor_update = Some(CursorUpdate {
        chain_id: envelope.host_chain_id,
        last_caught_up_block: envelope.slot as i64,
    });

    Ok(actions)
}

fn binary_opcode_to_operation(opcode: u8) -> Result<SupportedFheOperations> {
    let op = match opcode {
        0 => SupportedFheOperations::FheAdd,
        1 => SupportedFheOperations::FheSub,
        2 => SupportedFheOperations::FheMul,
        3 => SupportedFheOperations::FheDiv,
        4 => SupportedFheOperations::FheRem,
        5 => SupportedFheOperations::FheBitAnd,
        6 => SupportedFheOperations::FheBitOr,
        7 => SupportedFheOperations::FheBitXor,
        8 => SupportedFheOperations::FheShl,
        9 => SupportedFheOperations::FheShr,
        10 => SupportedFheOperations::FheRotl,
        11 => SupportedFheOperations::FheRotr,
        12 => SupportedFheOperations::FheEq,
        13 => SupportedFheOperations::FheNe,
        14 => SupportedFheOperations::FheGe,
        15 => SupportedFheOperations::FheGt,
        16 => SupportedFheOperations::FheLe,
        17 => SupportedFheOperations::FheLt,
        18 => SupportedFheOperations::FheMin,
        19 => SupportedFheOperations::FheMax,
        _ => anyhow::bail!("unsupported binary opcode: {opcode}"),
    };
    Ok(op)
}

fn unary_opcode_to_operation(opcode: u8) -> Result<SupportedFheOperations> {
    let op = match opcode {
        20 => SupportedFheOperations::FheNeg,
        21 => SupportedFheOperations::FheNot,
        _ => anyhow::bail!("unsupported unary opcode: {opcode}"),
    };
    Ok(op)
}

pub fn compute_schedule_order(
    block_time_unix: i64,
    tx_index: u32,
    op_index: u16,
) -> Result<PrimitiveDateTime> {
    let offset = OffsetDateTime::from_unix_timestamp(block_time_unix)?;
    let base = PrimitiveDateTime::new(offset.date(), offset.time());
    let order_offset = (tx_index as i64) * 10_000 + (op_index as i64);
    Ok(base.saturating_add(Duration::microseconds(order_offset)))
}

fn dependence_chain_from_signature(tx_signature: &[u8]) -> Vec<u8> {
    // Keep one deterministic chain id per transaction while preserving fixed
    // 32-byte shape expected by existing DB semantics.
    Keccak256::digest(tx_signature).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{FinalizedEventEnvelope, ProgramEvent, INTERFACE_VERSION};
    use solana_pubkey::Pubkey;

    fn fixed_sig() -> Vec<u8> {
        vec![42u8; 64]
    }

    #[test]
    fn map_add_event_to_computation() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 120,
            block_hash: vec![9u8; 32],
            block_time_unix: 1_700_000_000,
            tx_signature: fixed_sig(),
            tx_index: 2,
            op_index: 3,
            event: ProgramEvent::OpRequestedAdd {
                caller: Pubkey::new_from_array([1u8; 32]),
                lhs: [2u8; 32],
                rhs: [3u8; 32],
                is_scalar: true,
                result_handle: [4u8; 32],
            },
        };

        let actions = map_envelope_to_actions(&envelope, 7).expect("mapping should work");
        assert_eq!(actions.computations.len(), 1);
        assert_eq!(actions.allowed_handles.len(), 0);
        assert_eq!(actions.pbs_computations.len(), 0);
        assert_eq!(actions.blocks_valid.len(), 1);
        assert!(actions.cursor_update.is_some());

        let computation = &actions.computations[0];
        assert_eq!(computation.tenant_id, 7);
        assert_eq!(computation.fhe_operation, 0);
        assert!(computation.is_scalar);
        assert_eq!(computation.dependencies.len(), 2);
        assert_eq!(computation.dependence_chain_id.len(), 32);
        assert_eq!(
            computation.dependence_chain_id,
            Keccak256::digest(fixed_sig()).to_vec()
        );
        assert!(!computation.is_allowed);
    }

    #[test]
    fn map_allow_event_to_allowed_and_pbs() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 121,
            block_hash: vec![10u8; 32],
            block_time_unix: 1_700_000_100,
            tx_signature: fixed_sig(),
            tx_index: 0,
            op_index: 0,
            event: ProgramEvent::HandleAllowed {
                caller: Pubkey::new_from_array([1u8; 32]),
                handle: [7u8; 32],
                account: Pubkey::new_from_array([8u8; 32]),
            },
        };

        let actions = map_envelope_to_actions(&envelope, 9).expect("mapping should work");
        assert_eq!(actions.computations.len(), 0);
        assert_eq!(actions.allowed_handles.len(), 1);
        assert_eq!(actions.pbs_computations.len(), 1);
        assert_eq!(
            actions.allowed_handles[0].event_type,
            AllowEvents::AllowedAccount as i16
        );
    }

    #[test]
    fn map_sub_event_to_computation() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 122,
            block_hash: vec![11u8; 32],
            block_time_unix: 1_700_000_200,
            tx_signature: fixed_sig(),
            tx_index: 1,
            op_index: 0,
            event: ProgramEvent::OpRequestedSub {
                caller: Pubkey::new_from_array([1u8; 32]),
                lhs: [9u8; 32],
                rhs: [10u8; 32],
                is_scalar: false,
                result_handle: [11u8; 32],
            },
        };

        let actions = map_envelope_to_actions(&envelope, 10).expect("mapping should work");
        assert_eq!(actions.computations.len(), 1);
        assert_eq!(
            actions.computations[0].fhe_operation,
            SupportedFheOperations::FheSub as i16
        );
        assert!(!actions.computations[0].is_scalar);
        assert_eq!(actions.computations[0].dependencies[0], vec![9u8; 32]);
        assert_eq!(actions.computations[0].dependencies[1], vec![10u8; 32]);
    }

    #[test]
    fn map_binary_opcode_event_to_computation() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 123,
            block_hash: vec![12u8; 32],
            block_time_unix: 1_700_000_300,
            tx_signature: fixed_sig(),
            tx_index: 1,
            op_index: 1,
            event: ProgramEvent::OpRequestedBinary {
                caller: Pubkey::new_from_array([1u8; 32]),
                lhs: [12u8; 32],
                rhs: [13u8; 32],
                is_scalar: true,
                result_handle: [14u8; 32],
                opcode: 2,
            },
        };

        let actions = map_envelope_to_actions(&envelope, 11).expect("mapping should work");
        assert_eq!(
            actions.computations[0].fhe_operation,
            SupportedFheOperations::FheMul as i16
        );
        assert!(actions.computations[0].is_scalar);
    }

    #[test]
    fn map_unary_opcode_event_to_computation() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 124,
            block_hash: vec![13u8; 32],
            block_time_unix: 1_700_000_400,
            tx_signature: fixed_sig(),
            tx_index: 1,
            op_index: 2,
            event: ProgramEvent::OpRequestedUnary {
                caller: Pubkey::new_from_array([1u8; 32]),
                input: [15u8; 32],
                result_handle: [16u8; 32],
                opcode: 20,
            },
        };

        let actions = map_envelope_to_actions(&envelope, 12).expect("mapping should work");
        assert_eq!(
            actions.computations[0].fhe_operation,
            SupportedFheOperations::FheNeg as i16
        );
        assert_eq!(actions.computations[0].dependencies, vec![vec![15u8; 32]]);
    }

    #[test]
    fn map_other_tfhe_events_to_computation() {
        let base = FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 4242,
            slot: 125,
            block_hash: vec![14u8; 32],
            block_time_unix: 1_700_000_500,
            tx_signature: fixed_sig(),
            tx_index: 0,
            op_index: 0,
            event: ProgramEvent::OpRequestedIfThenElse {
                caller: Pubkey::new_from_array([1u8; 32]),
                control: [1u8; 32],
                if_true: [2u8; 32],
                if_false: [3u8; 32],
                result_handle: [4u8; 32],
            },
        };
        let if_then_else_actions = map_envelope_to_actions(&base, 13).expect("if-then-else");
        assert_eq!(
            if_then_else_actions.computations[0].fhe_operation,
            SupportedFheOperations::FheIfThenElse as i16
        );

        let cast = FinalizedEventEnvelope {
            event: ProgramEvent::OpRequestedCast {
                caller: Pubkey::new_from_array([1u8; 32]),
                input: [5u8; 32],
                to_type: 4,
                result_handle: [6u8; 32],
            },
            ..base.clone()
        };
        let cast_actions = map_envelope_to_actions(&cast, 13).expect("cast");
        assert_eq!(
            cast_actions.computations[0].fhe_operation,
            SupportedFheOperations::FheCast as i16
        );
        assert_eq!(
            cast_actions.computations[0].dependencies,
            vec![vec![5u8; 32], vec![4u8]]
        );

        let trivial_encrypt = FinalizedEventEnvelope {
            event: ProgramEvent::OpRequestedTrivialEncrypt {
                caller: Pubkey::new_from_array([1u8; 32]),
                pt: [7u8; 32],
                to_type: 3,
                result_handle: [8u8; 32],
            },
            ..base.clone()
        };
        let trivial_actions =
            map_envelope_to_actions(&trivial_encrypt, 13).expect("trivial_encrypt");
        assert_eq!(
            trivial_actions.computations[0].fhe_operation,
            SupportedFheOperations::FheTrivialEncrypt as i16
        );

        let rand = FinalizedEventEnvelope {
            event: ProgramEvent::OpRequestedRand {
                caller: Pubkey::new_from_array([1u8; 32]),
                rand_type: 2,
                seed: [9u8; 32],
                result_handle: [10u8; 32],
            },
            ..base.clone()
        };
        let rand_actions = map_envelope_to_actions(&rand, 13).expect("rand");
        assert_eq!(
            rand_actions.computations[0].fhe_operation,
            SupportedFheOperations::FheRand as i16
        );

        let rand_bounded = FinalizedEventEnvelope {
            event: ProgramEvent::OpRequestedRandBounded {
                caller: Pubkey::new_from_array([1u8; 32]),
                upper_bound: [11u8; 32],
                rand_type: 2,
                seed: [12u8; 32],
                result_handle: [13u8; 32],
            },
            ..base
        };
        let rand_bounded_actions =
            map_envelope_to_actions(&rand_bounded, 13).expect("rand_bounded");
        assert_eq!(
            rand_bounded_actions.computations[0].fhe_operation,
            SupportedFheOperations::FheRandBounded as i16
        );
    }
}
