use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use alloy::primitives::{Address, Bytes, U256};

use crate::core::event::{
    AttestationFormat, HandleContractPair, RequestValidity, UserDecryptRequest,
};
use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Enum representing the type of user decrypt request
/// Maps to the `user_decrypt_req_type` SQL enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_decrypt_req_type", rename_all = "snake_case")]
pub enum UserDecryptReqType {
    /// Legacy requests (existing format before type distinction)
    Legacy,
    /// v2 direct user decryption request
    UserDecrypt,
    /// v2 delegated user decryption request
    DelegatedUserDecrypt,
    /// v3 unified EIP-712 user decryption request
    Unified,
}

/// Typed wrapper for user decrypt request data.
///
/// `UserDecrypt`, `DelegatedUserDecrypt`, and `Unified` all carry the unified
/// in-memory `UserDecryptRequest`; the variant discriminator is preserved so
/// the `user_decrypt_req_type` SQL enum maps cleanly without dropping
/// existing on-disk values. `parse_req_data` knows how to lift pre-refactor
/// rows (which had flat top-level fields) into the new tagged shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserDecryptReqData {
    /// Legacy requests stored as raw JSON (for backward compatibility)
    Legacy(Value),
    /// v2 direct user decryption request (payload = LegacyDirect)
    UserDecrypt(UserDecryptRequest),
    /// v2 delegated user decryption request (payload = LegacyDelegated)
    DelegatedUserDecrypt(UserDecryptRequest),
    /// v3 unified EIP-712 user decryption request (payload = Unified)
    Unified(UserDecryptRequest),
}

impl UserDecryptReqData {
    /// Convert to JSON Value for database storage
    pub fn to_value(&self) -> Result<Value, serde_json::Error> {
        match self {
            UserDecryptReqData::Legacy(v) => Ok(v.clone()),
            UserDecryptReqData::UserDecrypt(req)
            | UserDecryptReqData::DelegatedUserDecrypt(req)
            | UserDecryptReqData::Unified(req) => serde_json::to_value(req),
        }
    }

    pub fn req_type(&self) -> UserDecryptReqType {
        match self {
            UserDecryptReqData::Legacy(_) => UserDecryptReqType::Legacy,
            UserDecryptReqData::UserDecrypt(_) => UserDecryptReqType::UserDecrypt,
            UserDecryptReqData::DelegatedUserDecrypt(_) => UserDecryptReqType::DelegatedUserDecrypt,
            UserDecryptReqData::Unified(_) => UserDecryptReqType::Unified,
        }
    }
}

/// Pre-refactor flat shape for `UserDecryptRequest` rows: top-level
/// `user_address` and no `payload` tag. Used solely to deserialize rows
/// written before the tagged-payload refactor.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct LegacyDirectFlatRow {
    ct_handle_contract_pairs: Vec<HandleContractPair>,
    request_validity: RequestValidity,
    contracts_chain_id: u64,
    contract_addresses: Vec<Address>,
    user_address: Address,
    signature: Bytes,
    public_key: Bytes,
    extra_data: Bytes,
}

impl From<LegacyDirectFlatRow> for UserDecryptRequest {
    fn from(v: LegacyDirectFlatRow) -> Self {
        UserDecryptRequest {
            signature: v.signature,
            public_key: v.public_key,
            extra_data: v.extra_data,
            attestation: AttestationFormat::LegacyDirect {
                ct_handle_contract_pairs: v.ct_handle_contract_pairs,
                request_validity: v.request_validity,
                contracts_chain_id: v.contracts_chain_id,
                contract_addresses: v.contract_addresses,
                user_address: v.user_address,
            },
        }
    }
}

/// Pre-refactor flat shape for `DelegatedUserDecryptRequest` rows:
/// top-level `delegator_address`, `delegate_address`, `startTimestamp`,
/// `durationDays` and no `payload` tag. Used solely to deserialize rows
/// written before the tagged-payload refactor.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct LegacyDelegatedFlatRow {
    ct_handle_contract_pairs: Vec<HandleContractPair>,
    contracts_chain_id: u64,
    contract_addresses: Vec<Address>,
    delegator_address: Address,
    delegate_address: Address,
    #[serde(rename = "startTimestamp")]
    start_timestamp: U256,
    #[serde(rename = "durationDays")]
    duration_days: U256,
    signature: Bytes,
    public_key: Bytes,
    extra_data: Bytes,
}

impl From<LegacyDelegatedFlatRow> for UserDecryptRequest {
    fn from(v: LegacyDelegatedFlatRow) -> Self {
        UserDecryptRequest {
            signature: v.signature,
            public_key: v.public_key,
            extra_data: v.extra_data,
            attestation: AttestationFormat::LegacyDelegated {
                ct_handle_contract_pairs: v.ct_handle_contract_pairs,
                request_validity: RequestValidity {
                    start_timestamp: v.start_timestamp,
                    duration_days: v.duration_days,
                },
                contracts_chain_id: v.contracts_chain_id,
                contract_addresses: v.contract_addresses,
                delegator_address: v.delegator_address,
                delegate_address: v.delegate_address,
            },
        }
    }
}

/// Try the new tagged shape first; if absent (pre-refactor row), fall back
/// to the supplied flat-row deserialization. This keeps rows persisted by
/// older relayer versions usable after the refactor.
fn deserialize_user_decrypt_request_compat<F>(
    value: Value,
    legacy: F,
) -> Result<UserDecryptRequest, serde_json::Error>
where
    F: FnOnce(Value) -> Result<UserDecryptRequest, serde_json::Error>,
{
    match serde_json::from_value::<UserDecryptRequest>(value.clone()) {
        Ok(req) => Ok(req),
        Err(_) => legacy(value),
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
    /// Parse the request data into the appropriate typed variant.
    ///
    /// For `UserDecrypt` / `DelegatedUserDecrypt` rows, first tries the new
    /// tagged `UserDecryptRequest` JSON shape; on failure falls back to the
    /// pre-refactor flat shape (for rows written by older relayer versions).
    pub fn parse_req_data(&self) -> Result<UserDecryptReqData, serde_json::Error> {
        match self.req_type {
            UserDecryptReqType::Legacy => Ok(UserDecryptReqData::Legacy(self.req.clone())),
            UserDecryptReqType::UserDecrypt => {
                let req = deserialize_user_decrypt_request_compat(self.req.clone(), |v| {
                    serde_json::from_value::<LegacyDirectFlatRow>(v).map(UserDecryptRequest::from)
                })?;
                Ok(UserDecryptReqData::UserDecrypt(req))
            }
            UserDecryptReqType::DelegatedUserDecrypt => {
                let req = deserialize_user_decrypt_request_compat(self.req.clone(), |v| {
                    serde_json::from_value::<LegacyDelegatedFlatRow>(v)
                        .map(UserDecryptRequest::from)
                })?;
                Ok(UserDecryptReqData::DelegatedUserDecrypt(req))
            }
            UserDecryptReqType::Unified => {
                // Unified rows are only written by post-refactor relayer
                // versions, so the tagged-payload JSON shape is the only
                // format we ever see here — no flat-shape fallback needed.
                let req: UserDecryptRequest = serde_json::from_value(self.req.clone())?;
                Ok(UserDecryptReqData::Unified(req))
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
