use super::error::{ApiResponseStatus, V2ErrorResponseBody};
use crate::http::endpoints::common::types::{ChainId, HandleContractPairJson, RequestValidityJson};
use crate::http::utils::redact::{redact_count, redact_len};
use crate::http::{de_string_or_number, serialize_vec_as_hex};
use alloy::primitives::Bytes;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// Request type for user decryption
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_handle_contract_pairs")
    )]
    pub handle_contract_pairs: Vec<HandleContractPairJson>,
    #[validate(custom(function = "crate::http::validate_request_validity"))]
    pub request_validity: RequestValidityJson,
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contracts_chain_id: String,
    #[validate(length(min = 1, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_blockchain_addresses"))]
    pub contract_addresses: Vec<String>,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub user_address: String,
    #[validate(
        length(equal = 130, message = "Must be 130 characters long"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    #[derivative(Debug(format_with = "redact_len"))]
    pub signature: String,
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[derivative(Debug(format_with = "redact_len"))]
    pub public_key: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// Request type for delegated user decryption
#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DelegatedUserDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_handle_contract_pairs")
    )]
    pub handle_contract_pairs: Vec<HandleContractPairJson>,
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contracts_chain_id: String,
    #[validate(length(min = 1, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_blockchain_addresses"))]
    pub contract_addresses: Vec<String>,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub delegator_address: String,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub delegate_address: String,
    #[validate(custom(function = "crate::http::validate_timestamp"))]
    pub start_timestamp: String,
    #[validate(custom(function = "crate::http::validate_u32_string"))]
    pub duration_days: String,
    #[validate(
        length(equal = 130, message = "Must be 130 characters long"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    pub signature: String,
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    pub public_key: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptPostResponseJson {
    pub status: ApiResponseStatus,
    pub request_id: String,
    pub result: UserDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptQueuedResult {
    pub job_id: String,
}

// Response format defined for TKMS library compatibility on client-side plaintext reconstruction
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
pub struct UserDecryptResponseJson {
    #[derivative(Debug(format_with = "redact_count"))]
    pub result: Vec<UserDecryptResponsePayloadJson>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptResponsePayloadJson {
    #[schema(value_type = String)]
    pub payload: Bytes,
    #[schema(value_type = String)]
    pub signature: Bytes,
    #[schema(value_type = String)]
    pub extra_data: String,
}

impl Serialize for UserDecryptResponsePayloadJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UserDecryptResponsePayloadJson", 3)?;
        state.serialize_field("payload", &serialize_vec_as_hex(&self.payload.to_vec()))?;
        state.serialize_field("signature", &serialize_vec_as_hex(&self.signature.to_vec()))?;
        state.serialize_field("extraData", &self.extra_data)?;
        state.end()
    }
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptStatusResponseJson {
    pub status: ApiResponseStatus,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<UserDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<V2ErrorResponseBody>,
}

// Implementation for converting core event to response format
impl From<crate::core::event::UserDecryptResponse> for UserDecryptResponseJson {
    fn from(response: crate::core::event::UserDecryptResponse) -> Self {
        let mut result_items = Vec::new();

        for (i, share) in response.reencrypted_shares.iter().enumerate() {
            let signature = response.signatures.get(i).cloned().unwrap_or_default();

            result_items.push(UserDecryptResponsePayloadJson {
                payload: share.clone(),
                signature,
                extra_data: response.extra_data.clone(),
            });
        }

        UserDecryptResponseJson {
            result: result_items,
        }
    }
}

// Implementation for converting database model to response format
impl TryFrom<crate::store::sql::models::user_decrypt_req_model::UserDecryptResponseModel>
    for UserDecryptResponseJson
{
    type Error = String;

    fn try_from(
        model: crate::store::sql::models::user_decrypt_req_model::UserDecryptResponseModel,
    ) -> Result<Self, Self::Error> {
        let mut result_items = Vec::new();

        for share in model.shares.0 {
            // Convert hex strings back to bytes
            let payload_bytes =
                hex::decode(&share.share).map_err(|e| format!("Failed to decode share: {}", e))?;
            let signature_bytes = hex::decode(&share.kms_signature)
                .map_err(|e| format!("Failed to decode kms_signature: {}", e))?;

            result_items.push(UserDecryptResponsePayloadJson {
                payload: Bytes::from(payload_bytes),
                signature: Bytes::from(signature_bytes),
                extra_data: share.extra_data,
            });
        }

        Ok(UserDecryptResponseJson {
            result: result_items,
        })
    }
}
