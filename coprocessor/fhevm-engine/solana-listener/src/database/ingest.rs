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
    use alloy::primitives::{Address, FixedBytes, Uint};
    use host_listener::contracts::AclContract as EvmAcl;
    use host_listener::contracts::AclContract::AclContractEvents as EvmAclEvent;
    use host_listener::contracts::TfheContract as EvmTfhe;
    use host_listener::contracts::TfheContract::TfheContractEvents as EvmTfheEvent;
    use solana_pubkey::Pubkey;

    fn fixed_sig() -> Vec<u8> {
        vec![42u8; 64]
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct CanonicalComputation {
        output_handle: Vec<u8>,
        dependencies: Vec<Vec<u8>>,
        fhe_operation: i16,
        is_scalar: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct CanonicalAllow {
        handle: Vec<u8>,
        event_type: i16,
        has_pbs_row: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum CanonicalAction {
        Computation(CanonicalComputation),
        Allow(CanonicalAllow),
    }

    enum EvmCase {
        Tfhe(EvmTfheEvent),
        Acl(EvmAclEvent),
    }

    fn evm_caller() -> Address {
        Address::from_slice(&[0x11u8; 20])
    }

    fn h32(value: [u8; 32]) -> FixedBytes<32> {
        FixedBytes::from(value)
    }

    fn scalar_byte(value: bool) -> FixedBytes<1> {
        if value {
            FixedBytes::from([1u8])
        } else {
            FixedBytes::from([0u8])
        }
    }

    fn u256(value: [u8; 32]) -> Uint<256, 4> {
        Uint::<256, 4>::from_be_slice(&value)
    }

    fn h16(value: [u8; 16]) -> FixedBytes<16> {
        FixedBytes::from(value)
    }

    fn canonical_from_solana(actions: &IngestActions) -> Vec<CanonicalAction> {
        let mut out = Vec::new();
        for row in &actions.computations {
            let mut dependencies = row.dependencies.clone();
            if (row.fhe_operation == SupportedFheOperations::FheRand as i16
                || row.fhe_operation == SupportedFheOperations::FheRandBounded as i16)
                && !dependencies.is_empty()
                && dependencies[0].len() == 32
            {
                // EVM ABI exposes rand seed as bytes16; Solana v0 currently emits
                // a [u8; 32] seed. Normalize to a shared 16-byte view for parity diff.
                dependencies[0].truncate(16);
            }
            out.push(CanonicalAction::Computation(CanonicalComputation {
                output_handle: row.output_handle.clone(),
                dependencies,
                fhe_operation: row.fhe_operation,
                is_scalar: row.is_scalar,
            }));
        }
        for row in &actions.allowed_handles {
            let has_pbs_row = actions
                .pbs_computations
                .iter()
                .any(|pbs| pbs.handle == row.handle);
            out.push(CanonicalAction::Allow(CanonicalAllow {
                handle: row.handle.clone(),
                event_type: row.event_type,
                has_pbs_row,
            }));
        }
        out
    }

    fn evm_operation_id(event: &EvmTfheEvent) -> i16 {
        let op = match event {
            EvmTfheEvent::FheAdd(_) => SupportedFheOperations::FheAdd,
            EvmTfheEvent::FheSub(_) => SupportedFheOperations::FheSub,
            EvmTfheEvent::FheMul(_) => SupportedFheOperations::FheMul,
            EvmTfheEvent::FheDiv(_) => SupportedFheOperations::FheDiv,
            EvmTfheEvent::FheRem(_) => SupportedFheOperations::FheRem,
            EvmTfheEvent::FheBitAnd(_) => SupportedFheOperations::FheBitAnd,
            EvmTfheEvent::FheBitOr(_) => SupportedFheOperations::FheBitOr,
            EvmTfheEvent::FheBitXor(_) => SupportedFheOperations::FheBitXor,
            EvmTfheEvent::FheShl(_) => SupportedFheOperations::FheShl,
            EvmTfheEvent::FheShr(_) => SupportedFheOperations::FheShr,
            EvmTfheEvent::FheRotl(_) => SupportedFheOperations::FheRotl,
            EvmTfheEvent::FheRotr(_) => SupportedFheOperations::FheRotr,
            EvmTfheEvent::FheEq(_) => SupportedFheOperations::FheEq,
            EvmTfheEvent::FheNe(_) => SupportedFheOperations::FheNe,
            EvmTfheEvent::FheGe(_) => SupportedFheOperations::FheGe,
            EvmTfheEvent::FheGt(_) => SupportedFheOperations::FheGt,
            EvmTfheEvent::FheLe(_) => SupportedFheOperations::FheLe,
            EvmTfheEvent::FheLt(_) => SupportedFheOperations::FheLt,
            EvmTfheEvent::FheMin(_) => SupportedFheOperations::FheMin,
            EvmTfheEvent::FheMax(_) => SupportedFheOperations::FheMax,
            EvmTfheEvent::FheNeg(_) => SupportedFheOperations::FheNeg,
            EvmTfheEvent::FheNot(_) => SupportedFheOperations::FheNot,
            EvmTfheEvent::Cast(_) => SupportedFheOperations::FheCast,
            EvmTfheEvent::TrivialEncrypt(_) => SupportedFheOperations::FheTrivialEncrypt,
            EvmTfheEvent::FheIfThenElse(_) => SupportedFheOperations::FheIfThenElse,
            EvmTfheEvent::FheRand(_) => SupportedFheOperations::FheRand,
            EvmTfheEvent::FheRandBounded(_) => SupportedFheOperations::FheRandBounded,
            EvmTfheEvent::Initialized(_)
            | EvmTfheEvent::Upgraded(_)
            | EvmTfheEvent::VerifyInput(_) => {
                panic!("evm_operation_id called on non-computation event")
            }
        };
        op as i16
    }

    fn canonical_from_evm(case: EvmCase) -> Vec<CanonicalAction> {
        match case {
            EvmCase::Tfhe(event) => {
                let computation = match event {
                    EvmTfheEvent::Cast(EvmTfhe::Cast {
                        ct, toType, result, ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![ct.to_vec(), vec![toType]],
                        fhe_operation: SupportedFheOperations::FheCast as i16,
                        is_scalar: true,
                    }),
                    EvmTfheEvent::FheAdd(EvmTfhe::FheAdd {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheSub(EvmTfhe::FheSub {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheMul(EvmTfhe::FheMul {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheDiv(EvmTfhe::FheDiv {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheRem(EvmTfhe::FheRem {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheBitAnd(EvmTfhe::FheBitAnd {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheBitOr(EvmTfhe::FheBitOr {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheBitXor(EvmTfhe::FheBitXor {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheShl(EvmTfhe::FheShl {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheShr(EvmTfhe::FheShr {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheRotl(EvmTfhe::FheRotl {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheRotr(EvmTfhe::FheRotr {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheEq(EvmTfhe::FheEq {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheNe(EvmTfhe::FheNe {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheGe(EvmTfhe::FheGe {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheGt(EvmTfhe::FheGt {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheLe(EvmTfhe::FheLe {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheLt(EvmTfhe::FheLt {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheMin(EvmTfhe::FheMin {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    })
                    | EvmTfheEvent::FheMax(EvmTfhe::FheMax {
                        lhs,
                        rhs,
                        scalarByte,
                        result,
                        ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![lhs.to_vec(), rhs.to_vec()],
                        fhe_operation: evm_operation_id(&event),
                        is_scalar: !scalarByte.is_zero(),
                    }),
                    EvmTfheEvent::FheIfThenElse(EvmTfhe::FheIfThenElse {
                        control,
                        ifTrue,
                        ifFalse,
                        result,
                        ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![control.to_vec(), ifTrue.to_vec(), ifFalse.to_vec()],
                        fhe_operation: SupportedFheOperations::FheIfThenElse as i16,
                        is_scalar: false,
                    }),
                    EvmTfheEvent::FheNeg(EvmTfhe::FheNeg { ct, result, .. })
                    | EvmTfheEvent::FheNot(EvmTfhe::FheNot { ct, result, .. }) => {
                        Some(CanonicalComputation {
                            output_handle: result.to_vec(),
                            dependencies: vec![ct.to_vec()],
                            fhe_operation: evm_operation_id(&event),
                            is_scalar: false,
                        })
                    }
                    EvmTfheEvent::FheRand(EvmTfhe::FheRand {
                        randType,
                        seed,
                        result,
                        ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![seed.to_vec(), vec![randType]],
                        fhe_operation: SupportedFheOperations::FheRand as i16,
                        is_scalar: true,
                    }),
                    EvmTfheEvent::FheRandBounded(EvmTfhe::FheRandBounded {
                        upperBound,
                        randType,
                        seed,
                        result,
                        ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![
                            seed.to_vec(),
                            upperBound.to_be_bytes_vec(),
                            vec![randType],
                        ],
                        fhe_operation: SupportedFheOperations::FheRandBounded as i16,
                        is_scalar: true,
                    }),
                    EvmTfheEvent::TrivialEncrypt(EvmTfhe::TrivialEncrypt {
                        pt,
                        toType,
                        result,
                        ..
                    }) => Some(CanonicalComputation {
                        output_handle: result.to_vec(),
                        dependencies: vec![pt.to_be_bytes_vec(), vec![toType]],
                        fhe_operation: SupportedFheOperations::FheTrivialEncrypt as i16,
                        is_scalar: true,
                    }),
                    EvmTfheEvent::Initialized(_)
                    | EvmTfheEvent::Upgraded(_)
                    | EvmTfheEvent::VerifyInput(_) => None,
                };
                computation
                    .map(CanonicalAction::Computation)
                    .into_iter()
                    .collect()
            }
            EvmCase::Acl(event) => match event {
                EvmAclEvent::Allowed(allowed) => vec![CanonicalAction::Allow(CanonicalAllow {
                    handle: allowed.handle.to_vec(),
                    event_type: AllowEvents::AllowedAccount as i16,
                    has_pbs_row: true,
                })],
                _ => Vec::new(),
            },
        }
    }

    fn fixed_envelope(event: ProgramEvent, op_index: u16) -> FinalizedEventEnvelope {
        FinalizedEventEnvelope {
            version: INTERFACE_VERSION,
            host_chain_id: 12345,
            slot: 999,
            block_hash: vec![0xAA; 32],
            block_time_unix: 1_700_000_500,
            tx_signature: fixed_sig(),
            tx_index: 4,
            op_index,
            event,
        }
    }

    fn assert_parity(name: &str, solana_event: ProgramEvent, evm_case: EvmCase, op_index: u16) {
        let actions = map_envelope_to_actions(&fixed_envelope(solana_event, op_index), 77)
            .expect("solana mapping should work");
        let solana = canonical_from_solana(&actions);
        let evm = canonical_from_evm(evm_case);
        assert_eq!(solana, evm, "parity mismatch for case {name}");
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

    #[test]
    fn parity_diff_matches_evm_semantics_for_v0_surface() {
        let caller = Pubkey::new_from_array([0x11u8; 32]);
        let allow_account = Pubkey::new_from_array([0x22u8; 32]);

        assert_parity(
            "add",
            ProgramEvent::OpRequestedAdd {
                caller,
                lhs: [1u8; 32],
                rhs: [2u8; 32],
                is_scalar: true,
                result_handle: [3u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::FheAdd(EvmTfhe::FheAdd {
                caller: evm_caller(),
                lhs: h32([1u8; 32]),
                rhs: h32([2u8; 32]),
                scalarByte: scalar_byte(true),
                result: h32([3u8; 32]),
            })),
            0,
        );

        assert_parity(
            "sub",
            ProgramEvent::OpRequestedSub {
                caller,
                lhs: [4u8; 32],
                rhs: [5u8; 32],
                is_scalar: false,
                result_handle: [6u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::FheSub(EvmTfhe::FheSub {
                caller: evm_caller(),
                lhs: h32([4u8; 32]),
                rhs: h32([5u8; 32]),
                scalarByte: scalar_byte(false),
                result: h32([6u8; 32]),
            })),
            1,
        );

        assert_parity(
            "binary_mul",
            ProgramEvent::OpRequestedBinary {
                caller,
                lhs: [7u8; 32],
                rhs: [8u8; 32],
                is_scalar: true,
                result_handle: [9u8; 32],
                opcode: 2,
            },
            EvmCase::Tfhe(EvmTfheEvent::FheMul(EvmTfhe::FheMul {
                caller: evm_caller(),
                lhs: h32([7u8; 32]),
                rhs: h32([8u8; 32]),
                scalarByte: scalar_byte(true),
                result: h32([9u8; 32]),
            })),
            2,
        );

        assert_parity(
            "unary_neg",
            ProgramEvent::OpRequestedUnary {
                caller,
                input: [10u8; 32],
                result_handle: [11u8; 32],
                opcode: 20,
            },
            EvmCase::Tfhe(EvmTfheEvent::FheNeg(EvmTfhe::FheNeg {
                caller: evm_caller(),
                ct: h32([10u8; 32]),
                result: h32([11u8; 32]),
            })),
            3,
        );

        assert_parity(
            "if_then_else",
            ProgramEvent::OpRequestedIfThenElse {
                caller,
                control: [12u8; 32],
                if_true: [13u8; 32],
                if_false: [14u8; 32],
                result_handle: [15u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::FheIfThenElse(EvmTfhe::FheIfThenElse {
                caller: evm_caller(),
                control: h32([12u8; 32]),
                ifTrue: h32([13u8; 32]),
                ifFalse: h32([14u8; 32]),
                result: h32([15u8; 32]),
            })),
            4,
        );

        assert_parity(
            "cast",
            ProgramEvent::OpRequestedCast {
                caller,
                input: [16u8; 32],
                to_type: 9,
                result_handle: [17u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::Cast(EvmTfhe::Cast {
                caller: evm_caller(),
                ct: h32([16u8; 32]),
                toType: 9,
                result: h32([17u8; 32]),
            })),
            5,
        );

        assert_parity(
            "trivial_encrypt",
            ProgramEvent::OpRequestedTrivialEncrypt {
                caller,
                pt: [18u8; 32],
                to_type: 7,
                result_handle: [19u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::TrivialEncrypt(EvmTfhe::TrivialEncrypt {
                caller: evm_caller(),
                pt: u256([18u8; 32]),
                toType: 7,
                result: h32([19u8; 32]),
            })),
            6,
        );

        assert_parity(
            "rand",
            ProgramEvent::OpRequestedRand {
                caller,
                rand_type: 3,
                seed: [20u8; 32],
                result_handle: [21u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::FheRand(EvmTfhe::FheRand {
                caller: evm_caller(),
                randType: 3,
                seed: h16([20u8; 16]),
                result: h32([21u8; 32]),
            })),
            7,
        );

        assert_parity(
            "rand_bounded",
            ProgramEvent::OpRequestedRandBounded {
                caller,
                upper_bound: [22u8; 32],
                rand_type: 4,
                seed: [23u8; 32],
                result_handle: [24u8; 32],
            },
            EvmCase::Tfhe(EvmTfheEvent::FheRandBounded(EvmTfhe::FheRandBounded {
                caller: evm_caller(),
                upperBound: u256([22u8; 32]),
                randType: 4,
                seed: h16([23u8; 16]),
                result: h32([24u8; 32]),
            })),
            8,
        );

        assert_parity(
            "allow",
            ProgramEvent::HandleAllowed {
                caller,
                handle: [25u8; 32],
                account: allow_account,
            },
            EvmCase::Acl(EvmAclEvent::Allowed(EvmAcl::Allowed {
                caller: evm_caller(),
                account: evm_caller(),
                handle: h32([25u8; 32]),
            })),
            9,
        );
    }
}
