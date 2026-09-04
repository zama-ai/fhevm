use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use alloy::primitives::{Address, Bytes, U256};

use crate::core::event::{
    HandleContractPair, HandleEntry, RequestValidity, RequestValiditySeconds, UserDecryptRequest,
};
use crate::store::sql::models::req_status_enum_model::ReqStatus;

/// Enum representing the type of user decrypt request. Maps to the
/// `user_decrypt_req_type` SQL enum.
///
/// `UserDecrypt` and `DelegatedUserDecrypt` are deprecated: kept only to
/// keep already-persisted v2 rows readable until the legacy EIP-712
/// formats (direct + delegated) are removed. New writes use `Unified`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_decrypt_req_type", rename_all = "snake_case")]
pub enum UserDecryptReqType {
    /// Legacy EIP-712 direct user decryption request.
    /// Deprecated; should be removed once the legacy EIP-712 formats are removed.
    UserDecrypt,
    /// Legacy EIP-712 delegated user decryption request.
    /// Deprecated; should be removed once the legacy EIP-712 formats are removed.
    DelegatedUserDecrypt,
    /// Unified EIP-712 user decryption request.
    Unified,
}

/// Typed wrapper for user decrypt request data.
///
/// All three variants carry the same in-memory `UserDecryptRequest`; the
/// variant drives the `user_decrypt_req_type` SQL enum, which selects the
/// stored payload's shape (see [`from_stored_value`]).
///
/// `UserDecrypt` and `DelegatedUserDecrypt` are deprecated and should be
/// removed once the legacy EIP-712 formats (direct + delegated) are
/// removed.
#[derive(Debug, Clone)]
pub enum UserDecryptReqData {
    /// Legacy EIP-712 direct user decryption (payload = LegacyDirect).
    /// Deprecated; should be removed once the legacy EIP-712 formats are removed.
    UserDecrypt(UserDecryptRequest),
    /// Legacy EIP-712 delegated user decryption (payload = LegacyDelegated).
    /// Deprecated; should be removed once the legacy EIP-712 formats are removed.
    DelegatedUserDecrypt(UserDecryptRequest),
    /// Unified EIP-712 user decryption (payload = Eip712UnifiedV1).
    Unified(UserDecryptRequest),
}

impl UserDecryptReqData {
    /// Convert to JSON Value for database storage
    pub fn to_value(&self) -> Result<Value, serde_json::Error> {
        match self {
            UserDecryptReqData::UserDecrypt(req)
            | UserDecryptReqData::DelegatedUserDecrypt(req)
            | UserDecryptReqData::Unified(req) => to_stored_value(req),
        }
    }

    pub fn req_type(&self) -> UserDecryptReqType {
        match self {
            UserDecryptReqData::UserDecrypt(_) => UserDecryptReqType::UserDecrypt,
            UserDecryptReqData::DelegatedUserDecrypt(_) => UserDecryptReqType::DelegatedUserDecrypt,
            UserDecryptReqData::Unified(_) => UserDecryptReqType::Unified,
        }
    }
}

/// Stored shape of a `user_decrypt` row.
#[derive(Serialize, Deserialize)]
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
        UserDecryptRequest::LegacyDirect {
            ct_handle_contract_pairs: v.ct_handle_contract_pairs,
            request_validity: v.request_validity,
            contracts_chain_id: v.contracts_chain_id,
            contract_addresses: v.contract_addresses,
            user_address: v.user_address,
            signature: v.signature,
            public_key: v.public_key,
            extra_data: v.extra_data,
        }
    }
}

/// Stored shape of a `delegated_user_decrypt` row. The validity window is flat
/// here; the in-memory variant nests it under `request_validity`.
#[derive(Serialize, Deserialize)]
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
        UserDecryptRequest::LegacyDelegated {
            ct_handle_contract_pairs: v.ct_handle_contract_pairs,
            request_validity: RequestValidity {
                start_timestamp: v.start_timestamp,
                duration_days: v.duration_days,
            },
            contracts_chain_id: v.contracts_chain_id,
            contract_addresses: v.contract_addresses,
            delegator_address: v.delegator_address,
            delegate_address: v.delegate_address,
            signature: v.signature,
            public_key: v.public_key,
            extra_data: v.extra_data,
        }
    }
}

/// Stored shape of a `unified` row.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct UnifiedFlatRow {
    handles: Vec<HandleEntry>,
    user_address: Address,
    allowed_contracts: Vec<Address>,
    request_validity: RequestValiditySeconds,
    signature: Bytes,
    public_key: Bytes,
    extra_data: Bytes,
}

impl From<UnifiedFlatRow> for UserDecryptRequest {
    fn from(v: UnifiedFlatRow) -> Self {
        UserDecryptRequest::Eip712UnifiedV1 {
            handles: v.handles,
            user_address: v.user_address,
            allowed_contracts: v.allowed_contracts,
            request_validity: v.request_validity,
            signature: v.signature,
            public_key: v.public_key,
            extra_data: v.extra_data,
        }
    }
}

fn to_stored_value(request: &UserDecryptRequest) -> Result<Value, serde_json::Error> {
    match request {
        UserDecryptRequest::LegacyDirect {
            ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses,
            user_address,
            signature,
            public_key,
            extra_data,
        } => serde_json::to_value(LegacyDirectFlatRow {
            ct_handle_contract_pairs: ct_handle_contract_pairs.clone(),
            request_validity: request_validity.clone(),
            contracts_chain_id: *contracts_chain_id,
            contract_addresses: contract_addresses.clone(),
            user_address: *user_address,
            signature: signature.clone(),
            public_key: public_key.clone(),
            extra_data: extra_data.clone(),
        }),
        UserDecryptRequest::LegacyDelegated {
            ct_handle_contract_pairs,
            request_validity,
            contracts_chain_id,
            contract_addresses,
            delegator_address,
            delegate_address,
            signature,
            public_key,
            extra_data,
        } => serde_json::to_value(LegacyDelegatedFlatRow {
            ct_handle_contract_pairs: ct_handle_contract_pairs.clone(),
            contracts_chain_id: *contracts_chain_id,
            contract_addresses: contract_addresses.clone(),
            delegator_address: *delegator_address,
            delegate_address: *delegate_address,
            start_timestamp: request_validity.start_timestamp,
            duration_days: request_validity.duration_days,
            signature: signature.clone(),
            public_key: public_key.clone(),
            extra_data: extra_data.clone(),
        }),
        UserDecryptRequest::Eip712UnifiedV1 {
            handles,
            user_address,
            allowed_contracts,
            request_validity,
            signature,
            public_key,
            extra_data,
        } => serde_json::to_value(UnifiedFlatRow {
            handles: handles.clone(),
            user_address: *user_address,
            allowed_contracts: allowed_contracts.clone(),
            request_validity: request_validity.clone(),
            signature: signature.clone(),
            public_key: public_key.clone(),
            extra_data: extra_data.clone(),
        }),
    }
}

/// `req_type` selects the shape; the payload carries no discriminator. Every
/// read of `user_decrypt_req.req` goes through here - a caller that parses one
/// itself can disagree with this one.
pub fn from_stored_value(
    req_type: UserDecryptReqType,
    value: Value,
) -> Result<UserDecryptRequest, serde_json::Error> {
    match req_type {
        UserDecryptReqType::UserDecrypt => {
            serde_json::from_value::<LegacyDirectFlatRow>(value).map(UserDecryptRequest::from)
        }
        UserDecryptReqType::DelegatedUserDecrypt => {
            serde_json::from_value::<LegacyDelegatedFlatRow>(value).map(UserDecryptRequest::from)
        }
        UserDecryptReqType::Unified => {
            serde_json::from_value::<UnifiedFlatRow>(value).map(UserDecryptRequest::from)
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
    /// Parse the request data into the appropriate typed variant, per
    /// [`from_stored_value`].
    pub fn parse_req_data(&self) -> Result<UserDecryptReqData, serde_json::Error> {
        let req = from_stored_value(self.req_type, self.req.clone())?;
        Ok(match self.req_type {
            UserDecryptReqType::UserDecrypt => UserDecryptReqData::UserDecrypt(req),
            UserDecryptReqType::DelegatedUserDecrypt => {
                UserDecryptReqData::DelegatedUserDecrypt(req)
            }
            UserDecryptReqType::Unified => UserDecryptReqData::Unified(req),
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// A `user_decrypt` payload as v0.13 wrote it: no discriminator, validity
    /// nested.
    fn v013_direct_payload() -> Value {
        json!({
            "ct_handle_contract_pairs": [{
                "handle": "0x1",
                "contractAddress": "0x1111111111111111111111111111111111111111"
            }],
            "request_validity": { "startTimestamp": "0x64", "durationDays": "0x1e" },
            "contracts_chain_id": 11155111u64,
            "contract_addresses": ["0x1111111111111111111111111111111111111111"],
            "user_address": "0x2222222222222222222222222222222222222222",
            "signature": "0xdead",
            "public_key": "0xbeef",
            "extra_data": "0x00"
        })
    }

    /// A `delegated_user_decrypt` payload as v0.13 wrote it: validity flat.
    fn v013_delegated_payload() -> Value {
        json!({
            "ct_handle_contract_pairs": [{
                "handle": "0x1",
                "contractAddress": "0x1111111111111111111111111111111111111111"
            }],
            "contracts_chain_id": 11155111u64,
            "contract_addresses": ["0x1111111111111111111111111111111111111111"],
            "delegator_address": "0x2222222222222222222222222222222222222222",
            "delegate_address": "0x3333333333333333333333333333333333333333",
            "startTimestamp": "0x64",
            "durationDays": "0x1e",
            "signature": "0xdead",
            "public_key": "0xbeef",
            "extra_data": "0x00"
        })
    }

    #[test]
    fn reads_payloads_written_by_v013() {
        let direct = from_stored_value(UserDecryptReqType::UserDecrypt, v013_direct_payload())
            .expect("v0.13 direct payload must parse");
        assert!(matches!(direct, UserDecryptRequest::LegacyDirect { .. }));

        let delegated = from_stored_value(
            UserDecryptReqType::DelegatedUserDecrypt,
            v013_delegated_payload(),
        )
        .expect("v0.13 delegated payload must parse");
        assert!(matches!(
            delegated,
            UserDecryptRequest::LegacyDelegated { .. }
        ));
    }

    /// What this relayer writes is byte-identical to what v0.13 wrote, so an
    /// upgrade and a rollback both read the rows the other left behind.
    #[test]
    fn writes_the_shape_v013_wrote() {
        for (req_type, payload) in [
            (UserDecryptReqType::UserDecrypt, v013_direct_payload()),
            (
                UserDecryptReqType::DelegatedUserDecrypt,
                v013_delegated_payload(),
            ),
        ] {
            let parsed = from_stored_value(req_type, payload.clone()).expect("parse");
            let rewritten = to_stored_value(&parsed).expect("serialize");
            assert_eq!(
                rewritten, payload,
                "{req_type:?} round-trip changed the shape"
            );
        }
    }

    #[test]
    fn unified_round_trips() {
        let stored = json!({
            "handles": [{
                "ctHandle": "0x1",
                "contractAddress": "0x1111111111111111111111111111111111111111",
                "ownerAddress": "0x2222222222222222222222222222222222222222"
            }],
            "user_address": "0x2222222222222222222222222222222222222222",
            "allowed_contracts": [],
            "request_validity": { "startTimestamp": "0x64", "durationSeconds": "0xe10" },
            "signature": "0xdead",
            "public_key": "0xbeef",
            "extra_data": "0x00"
        });

        let parsed = from_stored_value(UserDecryptReqType::Unified, stored.clone()).expect("parse");
        assert!(matches!(parsed, UserDecryptRequest::Eip712UnifiedV1 { .. }));
        assert_eq!(to_stored_value(&parsed).expect("serialize"), stored);
    }

    /// `req_type` alone decides the shape: a payload is not sniffed, so the
    /// wrong `req_type` is an error rather than a silently different request.
    #[test]
    fn a_mismatched_req_type_is_an_error() {
        assert!(from_stored_value(
            UserDecryptReqType::DelegatedUserDecrypt,
            v013_direct_payload()
        )
        .is_err());
    }
}
