use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use crate::core::event::{DelegatedUserDecryptRequest, UserDecryptRequest};
use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Enum representing the type of user decrypt request
/// Maps to the `user_decrypt_req_type` SQL enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_decrypt_req_type", rename_all = "snake_case")]
pub enum UserDecryptReqType {
    /// Legacy requests (existing format before type distinction)
    Legacy,
    /// User decryption request
    UserDecrypt,
    /// Delegated user decryption request
    DelegatedUserDecrypt,
}

/// Typed wrapper for user decrypt request data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserDecryptReqData {
    /// Legacy requests stored as raw JSON (for backward compatibility)
    Legacy(Value),
    /// User decryption request with strongly-typed data
    UserDecrypt(UserDecryptRequest),
    /// Delegated user decryption request with strongly-typed data
    DelegatedUserDecrypt(DelegatedUserDecryptRequest),
}

impl UserDecryptReqData {
    /// Convert to JSON Value for database storage
    pub fn to_value(&self) -> Result<Value, serde_json::Error> {
        match self {
            UserDecryptReqData::Legacy(v) => Ok(v.clone()),
            UserDecryptReqData::UserDecrypt(req) => serde_json::to_value(req),
            UserDecryptReqData::DelegatedUserDecrypt(req) => serde_json::to_value(req),
        }
    }

    pub fn req_type(&self) -> UserDecryptReqType {
        match self {
            UserDecryptReqData::Legacy(_) => UserDecryptReqType::Legacy,
            UserDecryptReqData::UserDecrypt(_) => UserDecryptReqType::UserDecrypt,
            UserDecryptReqData::DelegatedUserDecrypt(_) => UserDecryptReqType::DelegatedUserDecrypt,
        }
    }
}

/// Represents a row in the `user_decrypt_req` table.
#[derive(Debug, FromRow, Clone)]
pub struct UserDecryptReq {
    pub id: i32,
    pub ext_job_id: Uuid,
    pub int_job_id: Vec<u8>,
    pub gw_reference_id: Option<Vec<u8>>,
    pub req: Value,
    pub req_type: UserDecryptReqType,
    pub req_status: ReqStatus,
    pub gw_req_tx_hash: Option<String>,
    pub gw_consensus_tx_hash: Option<String>,
    pub err_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserDecryptReq {
    /// Parse the request data into the appropriate typed variant
    pub fn parse_req_data(&self) -> Result<UserDecryptReqData, serde_json::Error> {
        match self.req_type {
            UserDecryptReqType::Legacy => Ok(UserDecryptReqData::Legacy(self.req.clone())),
            UserDecryptReqType::UserDecrypt => {
                let req: UserDecryptRequest = serde_json::from_value(self.req.clone())?;
                Ok(UserDecryptReqData::UserDecrypt(req))
            }
            UserDecryptReqType::DelegatedUserDecrypt => {
                let req: DelegatedUserDecryptRequest = serde_json::from_value(self.req.clone())?;
                Ok(UserDecryptReqData::DelegatedUserDecrypt(req))
            }
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ConsensusReqState {
    pub req_status: ReqStatus,
    pub updated_at: DateTime<Utc>,
    pub err_reason: Option<String>,
    pub int_job_id: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDecryptResponseShare {
    pub share: String,
    pub kms_signature: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDecryptResponseModel {
    pub ext_job_id: Uuid,
    pub req_status: ReqStatus,
    pub updated_at: DateTime<Utc>,
    pub err_reason: Option<String>,
    pub gw_req_tx_hash: Option<String>,
    pub gw_consensus_tx_hash: Option<String>,
    pub resolved_threshold: Option<i64>,
    pub shares: Json<Vec<UserDecryptResponseShare>>,
}

#[derive(Debug, FromRow)]
pub struct UserDecryptDoneWithTransitionRes {
    pub int_job_id: Vec<u8>,
    pub req_status: ReqStatus,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub err_reason: Option<String>,
    pub old_status: ReqStatus,
    pub old_updated_at: chrono::DateTime<chrono::Utc>,
}
