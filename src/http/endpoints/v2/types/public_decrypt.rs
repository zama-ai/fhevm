use super::error::{ApiResponseStatus, V2ErrorResponseBody};
use crate::http::utils::redact::{redact_count, redact_len};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptRequestJson {
    /// Ciphertext handles to decrypt. Each is `0x` + 64 hex chars, obtained from an on-chain FHE operation.
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hexs")
    )]
    #[schema(min_items = 1, example = json!(["0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"]))]
    pub ciphertext_handles: Vec<String>,
    /// Extra data forwarded to the gateway contract. Always `"0x00"` in the current protocol version.
    #[schema(value_type = String, example = "0x00")]
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptPostResponseJson {
    #[schema(value_type = String, example = "queued")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: PublicDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptQueuedResult {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub job_id: String,
}

// GET response when completed
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptResponseJson {
    /// Decrypted plaintext value. Raw hex, no `0x` prefix.
    #[schema(value_type = String, example = "00000000000000000000000000000001")]
    #[derivative(Debug(format_with = "redact_len"))]
    pub decrypted_value: String,
    /// Gateway signatures over the decrypted value. Raw hex, no `0x` prefix.
    #[schema(value_type = Vec<String>, example = json!(["1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d"]))]
    #[derivative(Debug(format_with = "redact_count"))]
    pub signatures: Vec<String>,
    /// Extra data echoed back from the gateway contract. `0x`-prefixed hex.
    #[schema(value_type = String, example = "0x00")]
    pub extra_data: String,
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptStatusResponseJson {
    #[schema(example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<PublicDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<V2ErrorResponseBody>,
}

/// GET 200 — public decryption succeeded (has result, no error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicDecryptSucceededStatusResponse {
    #[schema(value_type = String, example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: PublicDecryptResponseJson,
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
