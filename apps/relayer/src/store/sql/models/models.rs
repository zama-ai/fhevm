use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::models::model_request_status_enum::ReqStatus;

/// Represents a row in the `user_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct UserDecryptReq {
    pub id: i32,
    pub ext_req_id: Uuid,
    pub internal_decryption_id: String,
    pub gw_decryption_id: Option<i32>,
    pub req: Value,
    pub res: Option<Value>,
    pub status: ReqStatus,
    pub tx_hash: Option<String>,
    pub consensus_reached: bool,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a row in the `user_decrypt_share` table.
#[derive(Debug, FromRow, Clone)]
pub struct UserDecryptShare {
    pub id: i32,
    pub gw_decryption_id: i32,
    pub share_index: i32,
    pub share: String,
    pub signature: String,
    pub extra_data: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a row in the `public_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct PublicDecryptReq {
    pub id: i32,
    pub ext_req_id: Uuid,
    pub internal_decryption_id: String,
    pub gw_decryption_id: Option<i32>,
    pub req: Value,
    pub res: Option<Value>,
    pub status: ReqStatus,
    pub tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a row in the `input_proof_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct InputProofReq {
    pub id: i32,
    pub ext_req_id: Uuid,
    pub internal_input_proof_id: Uuid,
    pub gw_input_proof_id: Option<i32>,
    pub req: Value,
    pub res: Option<Value>,
    pub status: ReqStatus,
    pub tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// --- Structs for partial query results ---

/// Partial data returned for a GET request on a user decryption.
#[derive(Debug, FromRow)]
pub struct UserDecryptReqStatus {
    pub res: Option<Value>,
    pub internal_decryption_id: String,
    pub status: ReqStatus,
}

/// Partial data returned for a GET request on a public decryption.
#[derive(Debug, FromRow)]
pub struct PublicDecryptReqStatus {
    pub res: Option<Value>,
    pub internal_decryption_id: String,
    pub status: ReqStatus,
}

/// Partial data returned for a GET request on an input proof.
#[derive(Debug, FromRow)]
pub struct InputProofReqStatus {
    pub res: Option<Value>,
    pub internal_input_proof_id: Uuid,
    pub status: ReqStatus,
}
