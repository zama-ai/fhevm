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
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_blockchain_addresses"))]
    #[schema(value_type = Vec<String>, example = json!(["0x1234567890123456789012345678901234567890"]))]
    pub contract_addresses: Vec<String>,
    /// Optional native host contract identities. Each item is a `0x`-prefixed bytes32 hex string.
    #[serde(default)]
    #[schema(
        value_type = Option<Vec<String>>,
        example = json!(["0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"]),
        nullable = true
    )]
    pub contract_ids: Option<Vec<String>>,
    /// Ethereum address of the user requesting decryption. `0x` + 40 hex chars.
    /// Required for EVM host chains. May be omitted for native host chains (e.g. Solana)
    /// when `userId` is provided.
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    #[schema(example = "0x1234567890123456789012345678901234567890", nullable = true)]
    pub user_address: Option<String>,
    /// Optional native host user identity. Must be a `0x`-prefixed bytes32 hex string.
    #[serde(default)]
    #[schema(
        example = "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        nullable = true
    )]
    pub user_id: Option<String>,
    /// Versioned auth signature over the decryption request. Raw hex, no `0x` prefix.
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[derivative(Debug(format_with = "redact_len"))]
    #[schema(
        example = "aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd0011223344"
    )]
    pub signature: String,
    /// User's public key for re-encryption. Raw hex, no `0x` prefix, minimum 2 chars.
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[derivative(Debug(format_with = "redact_len"))]
    #[schema(example = "04b8e5d3f1a2c4e6d8f0a1b3c5d7e9f1a2b4c6d8e0f2a3b5c7d9e1f3a5b7c9d1")]
    pub public_key: String,
    /// Extra data forwarded to the gateway contract. Supports legacy `"0x00"` and versioned formats.
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
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_blockchain_addresses"))]
    #[schema(value_type = Vec<String>, example = json!(["0x1234567890123456789012345678901234567890"]))]
    pub contract_addresses: Vec<String>,
    /// Optional native host contract identities. Each item is a `0x`-prefixed bytes32 hex string.
    #[serde(default)]
    #[schema(
        value_type = Option<Vec<String>>,
        example = json!(["0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"]),
        nullable = true
    )]
    pub contract_ids: Option<Vec<String>>,
    /// Ethereum address of the delegator (the user who owns the ciphertexts). `0x` + 40 hex chars.
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    #[schema(example = "0x1234567890123456789012345678901234567890", nullable = true)]
    pub delegator_address: Option<String>,
    /// Optional native host delegator identity. Must be a `0x`-prefixed bytes32 hex string.
    #[serde(default)]
    #[schema(
        example = "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        nullable = true
    )]
    pub delegator_id: Option<String>,
    /// Ethereum address of the delegate (the party authorized to decrypt). `0x` + 40 hex chars.
    #[serde(default)]
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    #[schema(example = "0x1234567890123456789012345678901234567890", nullable = true)]
    pub delegate_address: Option<String>,
    /// Optional native host delegate identity. Must be a `0x`-prefixed bytes32 hex string.
    #[serde(default)]
    #[schema(
        example = "0xabcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        nullable = true
    )]
    pub delegate_id: Option<String>,
    /// Unix timestamp (seconds) when the delegation starts. Decimal string.
    #[validate(custom(function = "crate::http::validate_timestamp"))]
    #[schema(example = "1700000000")]
    pub start_timestamp: String,
    /// Duration of the delegation in days. Decimal string.
    #[validate(custom(function = "crate::http::validate_u32_string"))]
    #[schema(example = "1")]
    pub duration_days: String,
    /// Versioned auth signature over the delegation request. Raw hex, no `0x` prefix.
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[schema(
        example = "aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd00112233445566778899aabbccdd0011223344"
    )]
    pub signature: String,
    /// Delegate's public key for re-encryption. Raw hex, no `0x` prefix, minimum 2 chars.
    #[validate(length(min = 2, message = "Must not be empty"))]
    #[validate(custom(function = "crate::http::validate_no_0x_hex"))]
    #[schema(example = "04b8e5d3f1a2c4e6d8f0a1b3c5d7e9f1a2b4c6d8e0f2a3b5c7d9e1f3a5b7c9d1")]
    pub public_key: String,
    /// Extra data forwarded to the gateway contract. Supports legacy `"0x00"` and versioned formats.
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}

// POST response with job ID and request tracking
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptPostResponseJson {
    #[schema(value_type = String, example = "queued")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: UserDecryptQueuedResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptQueuedResult {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
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
    /// Re-encrypted share payload. Raw hex, no `0x` prefix.
    #[schema(value_type = String, example = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2")]
    pub payload: Bytes,
    /// KMS signature over the payload. Raw hex, no `0x` prefix.
    #[schema(value_type = String, example = "1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b3c4d5e6f1a2b")]
    pub signature: Bytes,
    // Note: extra_data field is not serialized for TKMS library compatibility (used while decrypting to plain text)
    // serde(skip) - field is completely skipped during serialization AND deserialization
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
    #[schema(example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<UserDecryptResponseJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<V2ErrorResponseBody>,
}

/// GET 200 — user decryption succeeded (has result, no error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDecryptSucceededStatusResponse {
    #[schema(value_type = String, example = "succeeded")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub result: UserDecryptResponseJson,
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
