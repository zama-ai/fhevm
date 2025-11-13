use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Represents a row in the `input_proof_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct InputProofReq {
    pub id: i32,
    pub ext_req_id: Uuid,
    pub internal_input_proof_id: Uuid,
    pub gw_input_proof_id: Option<i32>,
    pub req: Value,
    pub res: Option<Value>,
    pub req_status: ReqStatus,
    pub tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Partial data returned for a GET request on an input proof.
#[derive(Debug, FromRow)]
pub struct InputProofReqStatus {
    pub res: Option<Value>,
    pub internal_input_proof_id: Uuid,
    pub req_status: ReqStatus,
}
