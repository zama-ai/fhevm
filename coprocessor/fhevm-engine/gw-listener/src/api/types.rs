use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyInputRequest {
    pub request_id: U256,
    pub ciphertext_with_zkpok: Bytes,
    pub contract_chain_id: U256,
    pub contract_address: Address,
    pub user_address: Address,
    pub user_signature: Bytes,
    pub commitment: FixedBytes<32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyInputResponse {
    pub status: ResponseStatus,
    pub request_id: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handles: Option<Vec<FixedBytes<32>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_id: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CiphertextResponse {
    pub status: CiphertextStatus,
    pub handle: FixedBytes<32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_ciphertext: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_ciphertext_digest: Option<FixedBytes<32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_format: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_id: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Bytes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub version: String,
    pub signer_address: Address,
    pub uptime: u64,
    pub last_block_processed: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_ciphertexts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verifications: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Verified,
    Pending,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CiphertextStatus {
    Found,
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl ApiError {
    pub fn invalid_request(message: &str) -> Self {
        Self {
            error: ApiErrorDetail {
                code: "INVALID_REQUEST".to_string(),
                message: message.to_string(),
                retry_after: None,
            },
        }
    }

    pub fn commitment_mismatch() -> Self {
        Self {
            error: ApiErrorDetail {
                code: "COMMITMENT_MISMATCH".to_string(),
                message: "Payload hash does not match on-chain commitment".to_string(),
                retry_after: None,
            },
        }
    }

    pub fn request_not_found(request_id: &str) -> Self {
        Self {
            error: ApiErrorDetail {
                code: "REQUEST_NOT_FOUND".to_string(),
                message: format!("Request {} not found", request_id),
                retry_after: None,
            },
        }
    }

    pub fn internal_error(message: &str) -> Self {
        Self {
            error: ApiErrorDetail {
                code: "INTERNAL_ERROR".to_string(),
                message: message.to_string(),
                retry_after: None,
            },
        }
    }
}
