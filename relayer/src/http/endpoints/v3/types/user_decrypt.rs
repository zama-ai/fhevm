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
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// v3 user-decrypt request envelope. The relayer dispatches strictly by
/// `attestationType`. Supported values:
/// - `"eip712-unified-user-decrypt-v1"` — EVM EIP-712 (verified on-chain by the gateway).
/// - `"solana-ed25519-user-decrypt-v2"` — Solana ed25519 (verified off-chain per-party by the
///   kms-connector). Both route to the same gateway V2 `userDecryptionRequest` calldata; the
///   relayer forwards `signature` + `extraData` opaquely and never verifies them.
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttestedUserDecryptRequestJson {
    /// The attestation/signature scheme used for the `signature` bytes.
    /// Must equal `"eip712-unified-user-decrypt-v1"` (EVM EIP-712) or
    /// `"solana-ed25519-user-decrypt-v2"` (Solana ed25519).
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

    #[schema(schema_with = crate::http::extra_data_decryption_schema)]
    #[validate(custom(function = "crate::http::validate_extra_data_field_decryption"))]
    pub extra_data: String,
}

/// v3 Solana user-decrypt envelope (`solana-srfc38-user-decrypt-v1`). A Solana-native shape
/// with no EVM placeholders: the ed25519 `signature` attests over the permit fields, and the
/// relayer forwards everything opaquely into the host-generic gateway `userDecryptionRequest`
/// overload (`hostKind = Solana`). Each KMS party's connector verifies the signature off-chain.
#[derive(Deserialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SolanaAttestedUserDecryptRequestJson {
    /// Must equal `"solana-srfc38-user-decrypt-v1"`.
    #[validate(custom(function = "crate::http::validate_v3_attestation_type"))]
    #[schema(example = "solana-srfc38-user-decrypt-v1")]
    pub attestation_type: String,

    /// The Solana permit + handle-evidence payload the `signature` attests over.
    #[validate(nested)]
    pub attested_payload: SolanaSrfc38UserDecryptPayloadJson,

    /// The ed25519 signature over the reconstructed sRFC-38 permit envelope. `0x`-hex.
    /// Opaque to the relayer — never verified here.
    #[validate(custom(function = "crate::http::validate_0x_hex"))]
    #[derivative(Debug(format_with = "redact_len"))]
    #[schema(example = "0xaabbccdd")]
    pub signature: String,
}

/// The Solana-native user-decryption payload: the eight signed permit fields (§3.1) plus the
/// per-handle access evidence. No `userAddress`, no `nonce`, no EVM per-handle addresses.
#[derive(Deserialize, Serialize, Clone, ToSchema, Validate, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SolanaSrfc38UserDecryptPayloadJson {
    /// The subject's 32-byte ed25519 pubkey (`0x` + 64 hex).
    #[validate(custom(function = "crate::http::validate_0x_hex"))]
    pub user_pubkey: String,

    /// The transport (re-encryption) public key: the tfhe safe-serialized ML-KEM-512 container
    /// (`0x` + hex). The exact length is enforced downstream by the permit decode.
    #[validate(custom(function = "crate::http::validate_0x_hex"))]
    pub transport_key: String,

    /// The signed ACL domain-key scope (each a 32-byte pubkey, `0x` + 64 hex). May be empty
    /// (permissive mode).
    pub allowed_acl_domain_keys: Vec<String>,

    /// Validity window (`startTimestamp` + `durationSeconds`), in seconds.
    #[validate(nested)]
    pub request_validity: RequestValiditySecondsJson,

    /// The 32-byte Solana program id the permit was signed for (`0x` + 64 hex).
    #[validate(custom(function = "crate::http::validate_0x_hex"))]
    pub verifying_program_id: String,

    /// The host chain id the handles belong to (decimal string; carries the chain-type high bit).
    pub chain_id: String,

    /// The signed KMS routing bytes (version `0x02` ‖ contextId ‖ epochId), `0x`-hex.
    #[validate(custom(function = "crate::http::validate_0x_hex"))]
    pub extra_data: String,

    /// One entry per ciphertext handle. Non-empty; the on-chain cap is applied at the gateway.
    #[validate(length(min = 1, message = "Must not be empty"))]
    pub handles: Vec<SolanaHandleJson>,
}

/// One Solana handle entry: the handle plus its self-authenticating access evidence. None of
/// these fields are signed — a substituted value can fail the request but never widen access.
#[derive(Deserialize, Serialize, Clone, ToSchema, Derivative)]
#[derivative(Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SolanaHandleJson {
    /// The 32-byte ciphertext handle (`0x` + 64 hex).
    pub handle: String,
    /// The 32-byte ciphertext owner: the signer for a direct entry, the delegator for a
    /// delegated one (`0x` + 64 hex).
    pub owner: String,
    /// The 32-byte encrypted value ID naming the `EncryptedValue` account (`0x` + 64 hex).
    pub encrypted_value_id: String,
    /// The `leaf_count` the access proof was built against; `"0"` in current mode (decimal string).
    pub proof_leaf_count: String,
    /// Empty (`"0x"`) for current access; otherwise the borsh `MmrProof` blob (`0x` + hex).
    pub access_proof: String,
}
