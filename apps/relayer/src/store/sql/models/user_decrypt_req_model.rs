use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Represents a row in the `user_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct UserDecryptReq {
    pub id: i32,
    pub ext_req_id: Uuid,
    pub internal_decryption_id: String,
    pub gw_decryption_id: Option<i32>,
    pub req: Value,
    pub res: Option<Value>,
    pub req_status: ReqStatus,
    pub tx_hash: Option<String>,
    pub consensus_reached: bool,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Partial data returned for a GET request on a user decryption.
#[derive(Debug, FromRow)]
pub struct UserDecryptReqStatus {
    pub res: Option<Value>,
    pub internal_decryption_id: String,
    pub req_status: ReqStatus,
}
