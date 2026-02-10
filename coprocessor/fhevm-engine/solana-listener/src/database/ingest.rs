use std::cmp::min;

use anyhow::Result;
use fhevm_engine_common::types::{AllowEvents, SupportedFheOperations};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

use crate::contracts::{FinalizedEventEnvelope, ProgramEventV0};

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
    // Keep this mapper intentionally aligned with host-listener SQL semantics in
    // `host-listener/src/database/tfhe_event_propagate.rs` (computation insert,
    // ACL unlock, block_valid, cursor update) so PoC parity stays easy to reason
    // about across chains.
    envelope.validate()?;

    let schedule_order = compute_schedule_order(
        envelope.block_time_unix,
        envelope.tx_index,
        envelope.op_index,
    )?;
    let tx_signature = Some(envelope.tx_signature.clone());

    let mut actions = IngestActions::default();

    match &envelope.event {
        ProgramEventV0::OpRequestedAddV1 {
            lhs,
            rhs,
            is_scalar,
            result_handle,
            ..
        } => {
            actions.computations.push(ComputationInsert {
                tenant_id,
                output_handle: result_handle.to_vec(),
                dependencies: vec![lhs.to_vec(), rhs.to_vec()],
                fhe_operation: SupportedFheOperations::FheAdd as i16,
                is_scalar: *is_scalar,
                dependence_chain_id: dependence_chain_from_signature(&envelope.tx_signature),
                transaction_id: tx_signature,
                // Keep ACL semantics explicit: request_add is queued but not runnable
                // until a matching allow event unlocks the handle.
                is_allowed: false,
                schedule_order,
                is_completed: false,
            });
        }
        ProgramEventV0::HandleAllowedV1 {
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
        block_hash: pseudo_block_hash_from_slot(envelope.slot),
        block_number: envelope.slot as i64,
    });

    actions.cursor_update = Some(CursorUpdate {
        chain_id: envelope.host_chain_id,
        last_caught_up_block: envelope.slot as i64,
    });

    Ok(actions)
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
    let mut chain = vec![0u8; 32];
    let limit = min(chain.len(), tx_signature.len());
    chain[..limit].copy_from_slice(&tx_signature[..limit]);
    chain
}

fn pseudo_block_hash_from_slot(slot: u64) -> Vec<u8> {
    let mut block_hash = vec![0u8; 32];
    block_hash[..8].copy_from_slice(&slot.to_be_bytes());
    block_hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{FinalizedEventEnvelope, ProgramEventV0, INTERFACE_V0_VERSION};
    use solana_pubkey::Pubkey;

    fn fixed_sig() -> Vec<u8> {
        vec![42u8; 64]
    }

    #[test]
    fn map_add_event_to_computation() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_V0_VERSION,
            host_chain_id: 4242,
            slot: 120,
            block_time_unix: 1_700_000_000,
            tx_signature: fixed_sig(),
            tx_index: 2,
            op_index: 3,
            event: ProgramEventV0::OpRequestedAddV1 {
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
        assert!(!computation.is_allowed);
    }

    #[test]
    fn map_allow_event_to_allowed_and_pbs() {
        let envelope = FinalizedEventEnvelope {
            version: INTERFACE_V0_VERSION,
            host_chain_id: 4242,
            slot: 121,
            block_time_unix: 1_700_000_100,
            tx_signature: fixed_sig(),
            tx_index: 0,
            op_index: 0,
            event: ProgramEventV0::HandleAllowedV1 {
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
}
