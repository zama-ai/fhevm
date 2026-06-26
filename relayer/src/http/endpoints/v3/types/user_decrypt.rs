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
/// `attestationType`. Supported values:
/// - `"eip712-unified-user-decrypt-v1"` — EVM EIP-712 (verified on-chain by the gateway).
/// - `"solana-ed25519-user-decrypt-v1"` — Solana ed25519 (verified off-chain per-party by the
///   kms-connector). Both route to the same gateway V2 `userDecryptionRequest` calldata; the
///   relayer forwards `signature` + `extraData` opaquely and never verifies them.
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttestedUserDecryptRequestJson {
    /// The attestation/signature scheme used for the `signature` bytes.
    /// Must equal `"eip712-unified-user-decrypt-v1"` (EVM EIP-712) or
    /// `"solana-ed25519-user-decrypt-v1"` (Solana ed25519).
    #[validate(custom(function = "crate::http::validate_v3_attestation_type"))]
    #[schema(example = "eip712-unified-user-decrypt-v1")]
    pub attestation_type: String,

    /// The EIP-712 Unified User-Decryption Request payload that the
    /// `signature` attests over.
    #[validate(nested)]
    pub attested_payload: Eip712UnifiedUserDecryptPayloadJson,

    /// Attestation signature. Opaque to the relayer — forwarded verbatim
    /// to the gateway, where the KMS Connector verifies it.
    /// `0x` + arbitrary hex.
    #[validate(
        length(min = 4, message = "Must not be empty"),
        custom(function = "crate::http::validate_0x_hex")
    )]
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

    /// Extra data forwarded verbatim to the gateway contract. `"0x00"` /
    /// `0x01`-versioned (KMS context id). For the Solana ed25519 attestation
    /// type, the ed25519 auth fields are carried as the typed `solana*` fields
    /// below rather than packed here, so `extraData` is context-only on both
    /// paths. Opaque to the relayer.
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    #[schema(example = "0x00")]
    pub extra_data: String,

    /// RFC-021 Solana ed25519 identity (32-byte pubkey, `0x` + 64 hex). Required for the
    /// `solana-ed25519-user-decrypt-v1` attestation type; absent for EVM. The ed25519 `signature`
    /// is verified against this identity off-chain by the KMS Connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = "0x1111111111111111111111111111111111111111111111111111111111111111")]
    pub solana_user_identity: Option<String>,

    /// RFC-021 per-request anti-replay nonce (`0x` + 64 hex) bound into the ed25519 signing
    /// preimage. Required for the Solana ed25519 attestation type; absent for EVM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = "0x2222222222222222222222222222222222222222222222222222222222222222")]
    pub solana_nonce: Option<String>,

    /// RFC-021 allowed Solana ACL domain keys (each a 32-byte pubkey, `0x` + 64 hex) — the Solana
    /// analog of `allowedContracts`. May be empty (permissive mode). Absent for EVM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = json!(["0x3333333333333333333333333333333333333333333333333333333333333333"]))]
    pub solana_allowed_acl_domain_keys: Option<Vec<String>>,

    /// Encrypted-value-ACL lineage identity (`acl_nonce_key`, `0x` + 64 hex) for a HISTORICAL or
    /// PUBLIC confidential-balance decrypt. Absent for a current-ACL decrypt. Pure pass-through to
    /// the gateway; bound into the ed25519 signing preimage and verified by the KMS Connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = "0x4444444444444444444444444444444444444444444444444444444444444444")]
    pub solana_acl_value_key: Option<String>,

    /// MMR inclusion proof for the decrypt: a mode-prefixed (`0x01` historical / `0x02` public)
    /// Borsh blob, `0x`-hex. Absent for a current-ACL decrypt. The SDK rebuilds and resubmits this
    /// (with a fresh `solana_proof_slot`) if the KMS reports the proof stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = "0x01...")]
    pub solana_mmr_proof: Option<String>,

    /// The lineage `leaf_count` the proof was built against — the staleness marker. Absent (0) for
    /// a current-ACL decrypt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(example = 42)]
    pub solana_proof_slot: Option<u64>,
}
