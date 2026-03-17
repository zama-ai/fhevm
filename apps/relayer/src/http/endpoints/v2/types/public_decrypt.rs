use super::error::ApiResponseStatus;
use crate::http::utils::redact::{redact_count, redact_len};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hexs")
    )]
    pub ciphertext_handles: Vec<String>,
    #[schema(value_type = String, example = "0x00")]
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptPostResponseJson {
    pub status: ApiResponseStatus,
    pub request_id: String,
    pub result: PublicDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptQueuedResult {
    pub job_id: String,
}

// GET response when completed
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptResponseJson {
    #[schema(value_type = String)]
    #[derivative(Debug(format_with = "redact_len"))]
    pub decrypted_value: String, // Hex string without 0x prefix
    #[schema(value_type = Vec<String>)]
    #[derivative(Debug(format_with = "redact_count"))]
    pub signatures: Vec<String>, // Hex strings without 0x prefix
    #[schema(value_type = String)]
    pub extra_data: String, // Hex string WITH 0x prefix
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptStatusResponseJson {
    pub status: ApiResponseStatus,
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PublicDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>, // Structured error object
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptErrorResponseJson {
    pub message: String,
}

// Standard serialization implementations for v2 API types
impl From<crate::core::event::PublicDecryptResponse> for PublicDecryptResponseJson {
    fn from(response: crate::core::event::PublicDecryptResponse) -> Self {
        let signatures: Vec<String> = response.signatures.iter().map(hex::encode).collect();

        PublicDecryptResponseJson {
            decrypted_value: hex::encode(&response.decrypted_value),
            signatures,
            extra_data: response.extra_data, // Already a string
        }
    }
}
