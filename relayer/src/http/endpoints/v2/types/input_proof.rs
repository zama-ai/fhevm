use super::error::{ApiResponseStatus, V2ErrorResponseBody};
use super::ChainId;
use crate::http::de_string_or_number;
use crate::http::utils::redact::{redact_count_opt, redact_len};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputProofRequestJson {
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contract_chain_id: String,
    #[serde(default)]
    #[schema(
        example = "0x1234567890123456789012345678901234567890",
        nullable = true
    )]
    pub contract_address: Option<String>,
    #[serde(default)]
    #[schema(
        example = "0x1234567890123456789012345678901234567890",
        nullable = true
    )]
    pub user_address: Option<String>,
    #[serde(default)]
    #[schema(
        example = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        nullable = true
    )]
    pub contract_id: Option<String>,
    #[serde(default)]
    #[schema(
        example = "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        nullable = true
    )]
    pub user_id: Option<String>,
    /// ABI-encoded ciphertext with its ZKPoK input verification data. Raw hex, no `0x` prefix.
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    #[derivative(Debug(format_with = "redact_len"))]
    #[schema(
        example = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0"
    )]
    pub ciphertext_with_input_verification: String,
    /// Extra data forwarded to the gateway contract. `"0x00"` for the legacy address flow, or versioned payloads for native host identities.
    #[validate(custom(function = "crate::http::validate_extra_data_field_input_proof"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofPostResponseJson {
    #[schema(value_type = String, example = "queued")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: InputProofQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofQueuedResult {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub job_id: String,
}

// GET response when completed
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputProofResponseJson {
    /// Whether the input proof verification was accepted by the gateway.
    #[schema(example = true)]
    pub accepted: bool,
    /// Extra data echoed back from the gateway contract. `0x`-prefixed hex.
    #[schema(value_type = String, example = "0x00")]
    pub extra_data: String,
    /// Verified ciphertext handles. Present only when `accepted` is `true`. Each handle is `0x` + 64 hex chars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>, example = json!(["0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"]))]
    pub handles: Option<Vec<String>>,
    /// Gateway signatures over the verified handles. Present only when `accepted` is `true`. Each is `0x`-prefixed hex.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>, example = json!(["0x1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d"]))]
    #[derivative(Debug(format_with = "redact_count_opt"))]
    pub signatures: Option<Vec<String>>,
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofStatusResponseJson {
    #[schema(example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<InputProofResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<V2ErrorResponseBody>,
}

/// GET 200 — input proof verification succeeded (has result, no error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofSucceededStatusResponse {
    #[schema(value_type = String, example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: InputProofResponseJson,
}

// Standard serialization implementations for v2 API types
impl From<crate::core::event::InputProofResponse> for InputProofResponseJson {
    fn from(response: crate::core::event::InputProofResponse) -> Self {
        // If we have handles and signatures, the input proof was accepted
        if !response.handles.is_empty() && !response.signatures.is_empty() {
            InputProofResponseJson {
                accepted: true,
                extra_data: "0x00".to_string(), // Default extra_data
                handles: Some(
                    response
                        .handles
                        .into_iter()
                        .map(|handle| format!("{handle:#x}"))
                        .collect(),
                ),
                signatures: Some(
                    response
                        .signatures
                        .into_iter()
                        .map(|sig| format!("{sig:#x}"))
                        .collect(),
                ),
            }
        } else {
            InputProofResponseJson {
                accepted: false,
                extra_data: "0x00".to_string(), // Default extra_data
                handles: None,
                signatures: None,
            }
        }
    }
}
