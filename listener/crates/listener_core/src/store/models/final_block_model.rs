use alloy::network::AnyRpcBlock;
use alloy::primitives::B256;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a row in the `final_blocks` table.
///
/// Finalized blocks never reorg, so unlike [`Block`](super::Block) there is
/// no status dimension: exactly one final block exists per
/// (chain_id, block_number).
#[derive(Debug, Clone)]
pub struct FinalBlock {
    pub id: Uuid,
    pub chain_id: i64,
    pub block_number: u64,
    pub block_hash: B256,
    pub parent_hash: B256,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new final block
#[derive(Debug, Clone)]
pub struct NewFinalBlock {
    pub block_number: u64,
    pub block_hash: B256,
    pub parent_hash: B256,
}

impl NewFinalBlock {
    pub fn from_rpc_block(block: &AnyRpcBlock) -> Self {
        Self {
            block_number: block.header.number,
            block_hash: block.header.hash,
            parent_hash: block.header.parent_hash,
        }
    }
}
