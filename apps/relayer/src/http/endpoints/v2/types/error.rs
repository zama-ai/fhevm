use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

// Error response structures for the v2 API

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError400NoDetails {
    pub label: String, // 'malformed_json' | 'request_error' | 'not_ready_for_decryption'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError400WithDetails {
    pub label: String, // 'missing_fields' | 'validation_failed'
    pub message: String,
    pub details: Vec<RelayerV2ErrorDetail>,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError404 {
    pub label: String, // 'not_found'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError429 {
    pub label: String, // 'rate_limited'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError500 {
    pub label: String, // 'internal_server_error'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError503 {
    pub label: String, // 'protocol_paused' | 'gateway_not_reachable'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ApiError504 {
    pub label: String, // 'readiness_check_timedout' | 'response_timedout'
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ErrorDetail {
    pub field: String,
    pub issue: String,
}

// Failed response wrapper
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ResponseFailed {
    pub status: String, // "failed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub error: serde_json::Value, // One of the RelayerV2ApiError* types above
}

// Queued response (202)
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ResponseQueued {
    pub status: String, // "queued"
    pub request_id: String,
    pub result: RelayerV2ResultQueued,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ResultQueued {
    pub job_id: String,
    pub retry_after_seconds: u32,
}

// Helper functions to create standard v2 error responses
impl RelayerV2ApiError500 {
    pub fn internal_server_error(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError500 {
            label: "internal_server_error".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}

impl RelayerV2ApiError404 {
    pub fn not_found(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError404 {
            label: "not_found".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}

impl RelayerV2ApiError400NoDetails {
    pub fn validation_error(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError400NoDetails {
            label: "request_error".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}

// TODO: Implement helper functions for 503 errors when needed
impl RelayerV2ApiError503 {
    #[allow(dead_code)]
    pub fn protocol_paused(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "protocol_paused".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }

    #[allow(dead_code)]
    pub fn gateway_not_reachable(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "gateway_not_reachable".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}

// TODO: Implement helper functions for 504 errors when needed
impl RelayerV2ApiError504 {
    #[allow(dead_code)]
    pub fn readiness_check_timedout(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError504 {
            label: "readiness_check_timedout".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }

    #[allow(dead_code)]
    pub fn response_timedout(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError504 {
            label: "response_timedout".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}
