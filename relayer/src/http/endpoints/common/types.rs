use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;
use validator::Validate;

/// Chain Id
///
/// It does support an ID as an integer or a 0x prefixed hex string
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum ChainId {
    #[schema(examples("0xaa36a7", "11155111"))]
    String(String),
    #[schema(example = 11155111)]
    Int(u64),
}

#[derive(Debug, Deserialize, Clone, Serialize, Hash, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HandleContractPairJson {
    /// Ciphertext handle from an on-chain FHE operation. `0x` + 64 hex chars.
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")]
    pub handle: String,
    /// Address of the contract that produced this ciphertext handle. `0x` + 40 hex chars.
    #[schema(example = "0x1234567890123456789012345678901234567890")]
    pub contract_address: String,
}

impl Display for HandleContractPairJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ct-handle: {}, contract-address: {}",
            self.handle, self.contract_address
        )
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestValidityJson {
    /// Unix timestamp (seconds) when this request becomes valid. Decimal string.
    #[schema(example = "1700000000")]
    pub start_timestamp: String,
    /// Number of days the request remains valid. Decimal string.
    #[schema(example = "1")]
    pub duration_days: String,
}

/// A single handle entry in the unified EIP-712 user-decryption payload.
/// Carries the `ownerAddress` for that handle alongside the handle and its
/// originating contract address.
#[derive(Debug, Deserialize, Clone, Serialize, Hash, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HandleEntryJson {
    /// Ciphertext handle from an on-chain FHE operation. `0x` + 64 hex chars.
    #[schema(example = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")]
    pub ct_handle: String,
    /// Address of the contract that produced this ciphertext handle. `0x` + 40 hex chars.
    #[schema(example = "0x1234567890123456789012345678901234567890")]
    pub contract_address: String,
    /// Owner address for this handle. For direct-access handles this equals
    /// the request's `userAddress`; for delegated handles it differs.
    /// `0x` + 40 hex chars.
    #[schema(example = "0x1234567890123456789012345678901234567890")]
    pub owner_address: String,
}

impl Display for HandleEntryJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ct-handle: {}, contract-address: {}, owner-address: {}",
            self.ct_handle, self.contract_address, self.owner_address
        )
    }
}

/// Request-validity window for the unified EIP-712 payload. Like
/// `RequestValidityJson` but in seconds instead of days.
#[derive(Debug, Deserialize, Clone, Serialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestValiditySecondsJson {
    /// Unix timestamp (seconds) when this request becomes valid. Decimal string.
    #[schema(example = "1700000000")]
    pub start_timestamp: String,
    /// Number of seconds the request remains valid. Decimal string.
    #[schema(example = "604800")]
    pub duration_seconds: String,
}
