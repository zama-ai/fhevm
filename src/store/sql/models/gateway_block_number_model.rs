use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Represents a row in the `gateway_block_number_store` table.
///
/// This table stores the last processed block number and hash for each listener instance,
/// enabling the relayer to resume processing from the correct block after restarts.
#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct GatewayBlockNumber {
    pub instance_id: i32,
    pub last_block_number: i64,
    pub last_block_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
