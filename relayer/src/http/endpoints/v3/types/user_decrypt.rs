//! v3 user-decrypt JSON wire types.
//!
//! The body is a typed-attestation envelope discriminated by `attestationType` (see the
//! issue comment 4278777024); the field name is the external JSON boundary and is kept as
//! it stands. [`UserDecryptV3RequestJson`] is that envelope: serde's internally tagged
//! enum is the discriminant, so an unknown tag never reaches an arm.
//!
//! The one scheme carries the unified EIP-712 User-Decryption Request as `attestedPayload`; the
//! relayer never re-hashes it, because the gateway contract verifies the EIP-712 signature on
//! chain (#1288).

use crate::http::endpoints::common::types::{HandleEntryJson, RequestValiditySecondsJson};
use crate::http::utils::redact::redact_len;
use derivative::Derivative;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationErrors};

/// POST `/v3/user-decrypt` body: `{ attestationType, attestedPayload, signature }`.
///
/// Internally tagged so the HTTP handler parses one type. Each arm's inner struct is
/// `deny_unknown_fields` and does not repeat the tag. An unknown tag is a serde
/// error; it never becomes an arm.
#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(tag = "attestationType")]
pub enum UserDecryptV3RequestJson {
    #[serde(rename = "eip712-unified-user-decrypt-v1")]
    Eip712Unified(AttestedUserDecryptRequestJson),
}

impl Validate for UserDecryptV3RequestJson {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            Self::Eip712Unified(inner) => inner.validate(),
        }
    }
}

/// The arm of the v3 user-decrypt envelope selected by
/// `attestationType == "eip712-unified-user-decrypt-v1"`. Its EIP-712 signature is verified on
/// chain by the gateway, so the relayer forwards `signature` and `extraData` verbatim without
/// re-hashing anything.
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttestedUserDecryptRequestJson {
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

    #[schema(schema_with = crate::http::extra_data_decryption_schema)]
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    pub extra_data: String,
}
