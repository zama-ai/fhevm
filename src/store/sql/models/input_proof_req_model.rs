use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Represents a row in the `input_proof_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct InputProofReq {
    pub id: i32,
    pub ext_job_id: Uuid,
    pub int_job_id: Vec<u8>,
    pub gw_reference_id: Option<Vec<u8>>,
    pub accepted: Option<bool>,
    pub req: Value,
    pub res: Option<Value>,
    pub req_status: ReqStatus,
    pub gw_req_tx_hash: Option<String>,
    pub gw_response_tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputProofResponseModel {
    pub req_status: ReqStatus,
    pub res: Option<Value>,
    pub err_reason: Option<String>,
    pub accepted: Option<bool>,
    pub updated_at: DateTime<Utc>,
}
