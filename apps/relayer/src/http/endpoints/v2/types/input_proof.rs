use super::ChainId;
use crate::http::de_string_or_number;
use crate::http::utils::redact::{redact_count_opt, redact_len};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// Same request type as v1
#[derive(Deserialize, Serialize, Validate, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputProofRequestJson {
    #[serde(deserialize_with = "de_string_or_number")]
    #[schema(value_type = ChainId)]
    #[validate(custom(function = "crate::http::validate_chain_id_string"))]
    pub contract_chain_id: String,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub contract_address: String,
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    pub user_address: String,
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_no_0x_hex")
    )]
    #[derivative(Debug(format_with = "redact_len"))]
    pub ciphertext_with_input_verification: String,
    #[validate(custom(function = "crate::http::validate_extra_data_field"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofPostResponseJson {
    pub status: String,
    pub request_id: String,
    pub result: InputProofQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofQueuedResult {
    pub job_id: String,
}

// GET response when completed
#[derive(Serialize, Deserialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputProofResponseJson {
    pub accepted: bool,
    #[schema(value_type = String)]
    pub extra_data: String, // Hex string WITH 0x prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    pub handles: Option<Vec<String>>, // Only present if accepted=true, hex with 0x prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<String>>)]
    #[derivative(Debug(format_with = "redact_count_opt"))]
    pub signatures: Option<Vec<String>>, // Only present if accepted=true, hex with 0x prefix
}

// GET response for status check
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofStatusResponseJson {
    pub status: String, // "queued", "succeeded", "failed"
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<InputProofResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>, // Structured error object
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InputProofErrorResponseJson {
    pub message: String,
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
