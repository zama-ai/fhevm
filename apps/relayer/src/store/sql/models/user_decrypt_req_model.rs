use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Represents a row in the `user_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct UserDecryptReq {
    pub id: i32,
    pub ext_reference_id: Uuid,
    pub int_indexer_id: Vec<u8>,
    pub gw_reference_id: Option<i32>,
    pub req: Value,
    pub req_status: ReqStatus,
    pub gw_req_tx_hash: Option<String>,
    pub gw_consensus_tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
