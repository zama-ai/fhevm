use alloy::primitives::U256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/// Parameters for inserting a share (data that gets stored in the database)
#[derive(Debug, Clone)]
pub struct ShareInsertParams<'a> {
    pub gw_reference_id: U256,
    pub share_index: U256,
    pub share: &'a str,
    pub kms_signature: &'a str,
    pub extra_data: &'a str,
    pub tx_hash: &'a str,
}

/// Represents a row in the `user_decrypt_share` table.
#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserDecryptShare {
    pub id: i32,
    pub gw_reference_id: Vec<u8>,
    pub tx_hash: Option<String>,
    pub share_index: i32,
    pub share: String,
    pub kms_signature: String,
    pub extra_data: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
