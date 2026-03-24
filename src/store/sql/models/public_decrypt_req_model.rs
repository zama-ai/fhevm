use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Represents a row in the `public_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct PublicDecryptReq {
    pub id: i32,
    pub ext_job_id: Uuid,
    pub int_job_id: Vec<u8>,
    pub gw_reference_id: Option<Vec<u8>>,
    pub req: Value,
    pub res: Option<Value>,
    pub req_status: ReqStatus,
    pub gw_req_tx_hash: Option<String>,
    pub gw_response_tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct PublicReqStateModel {
    pub int_job_id: Vec<u8>,
    pub req_status: ReqStatus,
    pub updated_at: DateTime<Utc>,
    pub err_reason: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct PublicReqStateModelWithOldStatusAndTimestamp {
    pub int_job_id: Vec<u8>,
    pub req_status: ReqStatus,
    pub updated_at: DateTime<Utc>,
    pub err_reason: Option<String>,
    pub old_status: ReqStatus,
    pub old_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDecryptResponseModel {
    pub ext_job_id: Uuid,
    pub req_status: ReqStatus,
    pub res: Option<Value>,
    pub err_reason: Option<String>,
    pub updated_at: DateTime<Utc>,
}
