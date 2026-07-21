//! v3 user-decrypt JSON wire types.
//!
//! The body is a typed-attestation envelope (see the issue comment
//! 4278777024). Internally the `attestedPayload` is the EIP-712 Unified
//! User-Decryption Request defined by the unified EIP-712 payload. The relayer never re-hashes
//! the payload — `signature` is opaque and forwarded verbatim to the
//! gateway (the KMS Connector verifies it off-chain, #1288).

use crate::http::endpoints::common::types::{HandleEntryJson, RequestValiditySecondsJson};
use crate::http::utils::redact::redact_len;
use derivative::Derivative;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

/// v3 user-decrypt request envelope. The relayer dispatches strictly by
/// `attestationType`; the currently supported value is
/// `"eip712-unified-user-decrypt-v1"`. Adding a Solana attestation later is
/// a one-line widening of the dispatch table, not a v4 bump.
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttestedUserDecryptRequestJson {
    /// The attestation/signature scheme used for the `signature` bytes.
    /// Must equal `"eip712-unified-user-decrypt-v1"` for the current
    /// release.
    #[validate(custom(function = "crate::http::validate_v3_attestation_type"))]
    #[schema(example = "eip712-unified-user-decrypt-v1")]
    pub attestation_type: String,

    /// The EIP-712 Unified User-Decryption Request payload that the
    /// `signature` attests over.
    #[validate(nested)]
    pub attested_payload: Eip712UnifiedUserDecryptPayloadJson,

    /// Attestation signature: `0x`-hex, or empty for the ERC-1271
    /// empty-signature path.
    #[validate(custom(function = "crate::http::validate_0x_hex_allow_empty"))]
    #[derivative(Debug(format_with = "redact_len"))]
    #[schema(example = "0xaabbccddeeff")]
    pub signature: String,
}

/// The EIP-712 Unified User-Decryption Request payload (the
/// `attestedPayload` body of the envelope).
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Eip712UnifiedUserDecryptPayloadJson {
    /// Must equal `"2.0"`.
    #[validate(custom(function = "crate::http::validate_v3_version"))]
    #[schema(example = "2.0")]
    pub version: String,

    /// Must equal `"user_decryption"`.
    #[serde(rename = "type")]
    #[validate(custom(function = "crate::http::validate_v3_payload_type"))]
    #[schema(example = "user_decryption")]
    pub r#type: String,

    /// One entry per ciphertext handle to decrypt. The list must be
    /// non-empty and must not exceed the existing v2 handle-count bound
    /// applied via `validate_handle_entries`.
    #[validate(
        length(min = 1, message = "Must not be empty"),
        custom(function = "crate::http::validate_handle_entries")
    )]
    pub handles: Vec<HandleEntryJson>,

    /// On-chain caller for the unified gateway call. `0x` + 40 hex chars.
    #[validate(custom(function = "crate::http::validate_blockchain_address"))]
    #[schema(example = "0x1234567890123456789012345678901234567890")]
    pub user_address: String,

    /// Allowlist of contracts whose handles may be decrypted under this
    /// request. May be empty (permissive mode).
    #[validate(custom(function = "crate::http::validate_blockchain_addresses_allow_empty"))]
    #[schema(example = json!(["0x1234567890123456789012345678901234567890"]))]
    pub allowed_contracts: Vec<String>,

    /// Validity window for the request, in seconds.
    #[validate(custom(function = "crate::http::validate_request_validity_seconds"))]
    pub request_validity: RequestValiditySecondsJson,

    /// User's public key for re-encryption. `0x` + hex, minimum 2 hex
    /// chars after the prefix.
    #[validate(
        length(min = 4, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hex")
    )]
    #[schema(example = "0x04b8e5d3f1a2c4e6d8f0a1b3c5d7e9f1a2b4c6d8e0f2a3b5c7d9e1f3a5b7c9d1")]
    pub public_key: String,

    /// Extra data forwarded verbatim to the gateway contract. Accepts `"0x00"`,
    /// version `0x01` (`0x01` + 32-byte contextId), or version `0x02`
    /// (`0x02` + 32-byte contextId + 32-byte epochId). contextId must be
    /// 0x07-tagged and epochId must be 0x08-tagged (first byte of each).
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    #[schema(example = "0x00")]
    pub extra_data: String,
}
