use crate::http::de_string_or_number;
use crate::http::endpoints::common::types::{ChainId, HandleContractPairJson, RequestValidityJson};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// Same request type as v1
#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
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
    pub signature: String,
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    pub public_key: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptPostResponseJson {
    pub status: String,
    pub request_id: String,
    pub result: UserDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptQueuedResult {
    pub job_id: String,
    pub retry_after_seconds: u32,
}

// GET response when completed
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptResponseJson {
    #[schema(value_type = Vec<String>)]
    pub payloads: Vec<String>, // Hex strings without 0x prefix
    #[schema(value_type = Vec<String>)]
    pub signatures: Vec<String>, // Hex strings without 0x prefix
    #[schema(value_type = Vec<String>)]
    pub extra_data: Vec<String>, // Hex strings with 0x prefix
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptStatusResponseJson {
    pub status: String, // "pending", "completed", "failed"
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<UserDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>, // Structured error object
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptErrorResponseJson {
    pub message: String,
}

// Standard serialization implementations for v2 API types
impl From<crate::core::event::UserDecryptResponse> for UserDecryptResponseJson {
    fn from(response: crate::core::event::UserDecryptResponse) -> Self {
        let payloads: Vec<String> = response
            .reencrypted_shares
            .iter()
            .map(hex::encode)
            .collect();

        let signatures: Vec<String> = response.signatures.iter().map(hex::encode).collect();

        // Format extra_data with 0x prefix and create array matching payloads length
        let formatted_extra_data = format!("0x{}", hex::encode(&response.extra_data));
        let extra_data: Vec<String> = vec![formatted_extra_data; payloads.len()];

        UserDecryptResponseJson {
            payloads,
            signatures,
            extra_data,
        }
    }
}

// From implementation for converting database model to v2 API response
impl From<crate::store::sql::models::user_decrypt_req_model::UserDecryptResponseModel>
    for UserDecryptResponseJson
{
    fn from(
        model: crate::store::sql::models::user_decrypt_req_model::UserDecryptResponseModel,
    ) -> Self {
        let mut payloads: Vec<String> = Vec::new();
        let mut signatures: Vec<String> = Vec::new();

        for share in model.shares.0 {
            // Shares are already hex strings in the database
            payloads.push(share.share);
            signatures.push(share.kms_signature);
        }

        // Extract extra_data from original request and create array matching shares count
        let extra_data_value = model
            .req
            .get("extra_data")
            .and_then(|v| v.as_str())
            .unwrap_or("0x00");

        // Ensure extra_data has 0x prefix for consistency with other v2 endpoints
        let formatted_extra_data = if extra_data_value.starts_with("0x") {
            extra_data_value.to_string()
        } else {
            format!("0x{}", extra_data_value)
        };

        // Create extra_data array with same length as payloads/signatures
        let extra_data: Vec<String> = vec![formatted_extra_data; payloads.len()];

        UserDecryptResponseJson {
            payloads,
            signatures,
            extra_data,
        }
    }
}
