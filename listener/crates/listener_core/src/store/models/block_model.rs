use alloy::network::AnyRpcBlock;
use alloy::primitives::B256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Maps to PostgreSQL enum type `block_status`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "block_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockStatus {
    Canonical,
    Finalized,
    Uncle,
}

/// Represents a row in the `blocks` table
#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub chain_id: i64,
    pub block_number: u64,
    pub block_hash: B256,
    pub parent_hash: B256,
    pub status: BlockStatus,
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new block
#[derive(Debug, Clone)]
pub struct NewDatabaseBlock {
    pub block_number: u64,
    pub block_hash: B256,
    pub parent_hash: B256,
    pub status: BlockStatus,
}

impl NewDatabaseBlock {
    pub fn from_rpc_block(block: &AnyRpcBlock, status: BlockStatus) -> Self {
        Self {
            block_number: block.header.number,
            block_hash: block.header.hash,
            parent_hash: block.header.parent_hash,
            status,
        }
    }
}

/// Result of an upsert_block_canonical operation.
///
/// # Status Codes
/// - `Inserted (0)` - A new block was inserted into the database
/// - `Updated (1)` - An existing block was updated (was UNCLE, now CANONICAL)
/// - `NoOp (2)` - Block was already CANONICAL, no changes made
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UpsertResult {
    /// A new block was inserted (status code: 0)
    Inserted = 0,
    /// An existing block was updated to CANONICAL (status code: 1)
    Updated = 1,
    /// Block was already CANONICAL, no operation performed (status code: 2)
    NoOp = 2,
}

impl UpsertResult {
    pub fn as_code(&self) -> u8 {
        *self as u8
    }
}
