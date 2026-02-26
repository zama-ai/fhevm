use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use utoipa::ToSchema;

use crate::http::utils::responses::{to_camel_case, FieldJsonErrorType, ParseError};
use crate::http::utils::validation_messages;

/// Status field values for V2 API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ApiResponseStatus {
    /// Request is queued for processing
    Queued,
    /// Request completed successfully
    Succeeded,
    /// Request failed
    Failed,
}

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
    // Gateway chain timeout errors are rendered as 503 instead of 504 because
    // CloudFlare overrides 504 errors with its own error page which cannot be
    // disabled in our setup. Labels include: 'protocol_paused', 'gateway_not_reachable',
    // 'readiness_check_timed_out', 'response_timed_out'
    pub label: String,
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
    pub status: ApiResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub error: serde_json::Value, // One of the RelayerV2ApiError* types above
}

// Queued response (202)
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ResponseQueued {
    pub status: ApiResponseStatus,
    pub request_id: String,
    pub result: RelayerV2ResultQueued,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ResultQueued {
    pub job_id: String,
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

    pub fn host_acl_failed(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError500 {
            label: "host_acl_failed".to_string(),
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

    pub fn not_allowed_on_host_acl(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError400NoDetails {
            label: "not_allowed_on_host_acl".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }

    pub fn host_chain_id_not_supported(chain_id: u64) -> Value {
        serde_json::to_value(RelayerV2ApiError400NoDetails {
            label: "host_chain_id_not_supported".to_string(),
            message: format!(
                "Host chain ID {} is not supported by this relayer",
                chain_id
            ),
        })
        .unwrap()
    }

    pub fn invalid_signature() -> Value {
        serde_json::to_value(RelayerV2ApiError400WithDetails {
            label: "validation_failed".to_string(),
            message: "Validation failed for 1 field(s)".to_string(),
            details: vec![RelayerV2ErrorDetail {
                field: "signature".to_string(),
                issue: "Signature is invalid".to_string(),
            }],
        })
        .unwrap()
    }
}

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
    pub fn insufficient_balance(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "insufficient_balance".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }

    #[allow(dead_code)]
    pub fn insufficient_allowance(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "insufficient_allowance".to_string(),
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

    pub fn readiness_check_timed_out(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "readiness_check_timed_out".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }

    pub fn response_timed_out(message: &str) -> Value {
        serde_json::to_value(RelayerV2ApiError503 {
            label: "response_timed_out".to_string(),
            message: message.to_string(),
        })
        .unwrap()
    }
}

// Helper methods for RelayerV2ResponseFailed to create consistent V2 error responses
impl RelayerV2ResponseFailed {
    /// Creates a host chain ID not supported response (400).
    pub fn host_chain_id_not_supported(
        chain_id: u64,
        request_id: &str,
    ) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::BAD_REQUEST;
        let label = "host_chain_id_not_supported";
        let message = format!(
            "Host chain ID {} is not supported by this relayer",
            chain_id
        );

        info!(
            request_id,
            http_status = status_code.as_u16(),
            label,
            message = message.as_str(),
            "HTTP response"
        );

        (
            status_code,
            Json(Self {
                status: ApiResponseStatus::Failed,
                request_id: Some(request_id.to_string()),
                error: RelayerV2ApiError400NoDetails::host_chain_id_not_supported(chain_id),
            }),
        )
    }

    /// Creates a request error response (400) for body read failures, etc.
    pub fn request_error(message: &str, request_id: &str) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::BAD_REQUEST;
        let label = "request_error";

        info!(
            request_id,
            http_status = status_code.as_u16(),
            label,
            message,
            "HTTP response"
        );

        (
            status_code,
            Json(Self {
                status: ApiResponseStatus::Failed,
                request_id: Some(request_id.to_string()),
                error: RelayerV2ApiError400NoDetails::validation_error(message),
            }),
        )
    }

    /// Creates an error response from a ParseError
    pub fn from_parse_error(
        parse_error: &ParseError,
        request_id: &str,
    ) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::BAD_REQUEST;

        match parse_error {
            ParseError::MalformedJson(message) => {
                let label = "malformed_json";

                info!(
                    request_id,
                    http_status = status_code.as_u16(),
                    label,
                    message = message.as_str(),
                    "HTTP response"
                );

                (
                    status_code,
                    Json(Self {
                        status: ApiResponseStatus::Failed,
                        request_id: Some(request_id.to_string()),
                        error: serde_json::to_value(RelayerV2ApiError400NoDetails {
                            label: label.to_string(),
                            message: message.clone(),
                        })
                        .unwrap(),
                    }),
                )
            }
            ParseError::FieldSpecificJson {
                field_name,
                issue,
                error_type,
            } => {
                // field_name is already camelCase from serde (unlike ValidationFailed which uses snake_case)
                let (label, message, detail_issue) = match error_type {
                    FieldJsonErrorType::Missing => (
                        "missing_fields",
                        format!("Missing 1 required field in the request: {}", field_name),
                        validation_messages::GENERIC_REQUIRED_BUT_MISSING.to_string(),
                    ),
                    FieldJsonErrorType::InvalidType | FieldJsonErrorType::Unknown => (
                        "validation_failed",
                        format!(
                            "Validation failed for 1 field in the request: {}",
                            field_name
                        ),
                        issue.clone(),
                    ),
                };

                info!(
                    request_id,
                    http_status = status_code.as_u16(),
                    label,
                    message = message.as_str(),
                    "HTTP response"
                );

                (
                    status_code,
                    Json(Self {
                        status: ApiResponseStatus::Failed,
                        request_id: Some(request_id.to_string()),
                        error: serde_json::to_value(RelayerV2ApiError400WithDetails {
                            label: label.to_string(),
                            message,
                            details: vec![RelayerV2ErrorDetail {
                                field: field_name.clone(),
                                issue: detail_issue,
                            }],
                        })
                        .unwrap(),
                    }),
                )
            }
            ParseError::ValidationFailed(errors) => {
                let label = "validation_failed";
                let details: Vec<RelayerV2ErrorDetail> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, field_errors)| {
                        let field = to_camel_case(field);
                        field_errors.iter().map(move |e| RelayerV2ErrorDetail {
                            field: field.clone(),
                            issue: e
                                .message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| format!("Invalid {}", field)),
                        })
                    })
                    .collect();

                let field_count = details.len();
                let message = if field_count == 1 {
                    format!(
                        "Validation failed for 1 field in the request: {}",
                        details[0].field
                    )
                } else {
                    let field_names: Vec<String> =
                        details.iter().map(|d| d.field.clone()).collect();
                    format!(
                        "Validation failed for {} fields in the request: {}",
                        field_count,
                        field_names.join(", ")
                    )
                };

                info!(
                    request_id,
                    http_status = status_code.as_u16(),
                    label,
                    message = message.as_str(),
                    "HTTP response"
                );

                (
                    status_code,
                    Json(Self {
                        status: ApiResponseStatus::Failed,
                        request_id: Some(request_id.to_string()),
                        error: serde_json::to_value(RelayerV2ApiError400WithDetails {
                            label: label.to_string(),
                            message,
                            details,
                        })
                        .unwrap(),
                    }),
                )
            }
            ParseError::ConversionFailed(message) => {
                tracing::error!(
                    "Internal error: Conversion failed after validation passed: {}",
                    message
                );
                Self::internal_server_error(request_id)
            }
        }
    }

    /// Creates a protocol overloaded (rate limited) response (429)
    pub fn protocol_overloaded(
        reason: &str,
        retry_after: &str,
        request_id: &str,
    ) -> impl IntoResponse {
        use axum::http::header;

        let status_code = StatusCode::TOO_MANY_REQUESTS;
        let label = "rate_limited";
        let message = format!("Server is experiencing high processing load: {}", reason);

        info!(
            request_id,
            http_status = status_code.as_u16(),
            label,
            message = message.as_str(),
            "HTTP response"
        );

        let response = Self {
            status: ApiResponseStatus::Failed,
            request_id: Some(request_id.to_string()),
            error: serde_json::to_value(RelayerV2ApiError429 {
                label: label.to_string(),
                message,
            })
            .unwrap(),
        };

        let mut http_response = (status_code, Json(response)).into_response();

        // Add Retry-After header
        if let Ok(header_value) = retry_after.parse() {
            http_response
                .headers_mut()
                .insert(header::RETRY_AFTER, header_value);
        }

        http_response
    }

    /// Creates an internal server error response (500)
    pub fn internal_server_error(request_id: &str) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let label = "internal_server_error";
        let message = "Internal server error";

        info!(
            request_id,
            http_status = status_code.as_u16(),
            label,
            message,
            "HTTP response"
        );

        (
            status_code,
            Json(Self {
                status: ApiResponseStatus::Failed,
                request_id: Some(request_id.to_string()),
                error: RelayerV2ApiError500::internal_server_error(message),
            }),
        )
    }

    /// Creates a service unavailable response (503)
    pub fn service_unavailable(message: &str) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::SERVICE_UNAVAILABLE;
        let label = "readiness_check_timed_out";

        info!(
            http_status = status_code.as_u16(),
            label, message, "HTTP response"
        );

        (
            status_code,
            Json(Self {
                status: ApiResponseStatus::Failed,
                request_id: None,
                error: RelayerV2ApiError503::readiness_check_timed_out(message),
            }),
        )
    }
}

/// Classify an error by its ABI revert selector and return the matching HTTP status + JSON body.
///
/// The contract can revert with many selectors — we validate most conditions before
/// sending the transaction, so only a subset can appear here. Plain-text messages
/// (no selector) fall through to Unknown / 500.
pub fn classify_revert_error(error_msg: &str) -> (StatusCode, serde_json::Value) {
    use crate::gateway::utils::{classify_revert_selector, extract_revert_selector, RevertReason};

    let reason = if let Some(selector) = extract_revert_selector(error_msg) {
        classify_revert_selector(&selector)
    } else {
        RevertReason::Unknown
    };

    match reason {
        RevertReason::ContractPaused => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::protocol_paused(error_msg),
        ),
        RevertReason::InsufficientBalance => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::insufficient_balance(error_msg),
        ),
        RevertReason::InsufficientAllowance => (
            StatusCode::SERVICE_UNAVAILABLE,
            RelayerV2ApiError503::insufficient_allowance(error_msg),
        ),
        RevertReason::InvalidSignature => (
            StatusCode::BAD_REQUEST,
            RelayerV2ApiError400NoDetails::invalid_signature(),
        ),
        RevertReason::Unknown => (
            StatusCode::INTERNAL_SERVER_ERROR,
            RelayerV2ApiError500::internal_server_error(error_msg),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_error_has_status_and_request_id() {
        let (status_code, json) =
            RelayerV2ResponseFailed::request_error("test error", "test-request-id");

        assert_eq!(status_code, StatusCode::BAD_REQUEST);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "request_error");
    }

    #[test]
    fn test_internal_server_error_has_status_and_request_id() {
        let (status_code, json) = RelayerV2ResponseFailed::internal_server_error("test-request-id");

        assert_eq!(status_code, StatusCode::INTERNAL_SERVER_ERROR);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "internal_server_error");
    }

    #[test]
    fn test_service_unavailable_has_status_no_request_id() {
        let (status_code, json) =
            RelayerV2ResponseFailed::service_unavailable("Key URL not yet initialized");

        assert_eq!(status_code, StatusCode::SERVICE_UNAVAILABLE);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, None);
        assert_eq!(response.error["label"], "readiness_check_timed_out");
    }

    #[test]
    fn test_from_parse_error_malformed_json() {
        let parse_error = ParseError::MalformedJson("Invalid JSON format".to_string());
        let (status_code, json) =
            RelayerV2ResponseFailed::from_parse_error(&parse_error, "test-request-id");

        assert_eq!(status_code, StatusCode::BAD_REQUEST);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "malformed_json");
    }

    #[test]
    fn test_from_parse_error_missing_field() {
        let parse_error = ParseError::FieldSpecificJson {
            field_name: "contractChainId".to_string(),
            issue: "This field is required".to_string(),
            error_type: FieldJsonErrorType::Missing,
        };
        let (status_code, json) =
            RelayerV2ResponseFailed::from_parse_error(&parse_error, "test-request-id");

        assert_eq!(status_code, StatusCode::BAD_REQUEST);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "missing_fields");
        assert!(response.error["details"].is_array());

        // Verify field name is preserved as camelCase (not corrupted to lowercase)
        let details = response.error["details"].as_array().unwrap();
        assert_eq!(details[0]["field"], "contractChainId");
        assert_eq!(details[0]["issue"], "Required but missing");
    }

    #[test]
    fn test_from_parse_error_validation_failed() {
        let mut errors = validator::ValidationErrors::new();
        let mut error = validator::ValidationError::new("invalid_format");
        error.message = Some("Invalid format".into());
        errors.add("ciphertext_handles", error);

        let parse_error = ParseError::ValidationFailed(errors);
        let (status_code, json) =
            RelayerV2ResponseFailed::from_parse_error(&parse_error, "test-request-id");

        assert_eq!(status_code, StatusCode::BAD_REQUEST);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "validation_failed");
    }

    #[test]
    fn test_serialized_error_response_format() {
        let (_, json) = RelayerV2ResponseFailed::request_error("test error", "test-request-id");

        let serialized = serde_json::to_string(&json.0).expect("Failed to serialize");
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("Failed to parse");

        // Verify the top-level structure has status, request_id, and error fields
        assert!(parsed.get("status").is_some(), "Should have status field");
        assert!(
            parsed.get("request_id").is_some(),
            "Should have request_id field"
        );
        assert!(parsed.get("error").is_some(), "Should have error field");

        // Verify the error has the expected structure
        let error = parsed.get("error").unwrap();
        assert!(error.get("label").is_some(), "Error should have label");
        assert!(error.get("message").is_some(), "Error should have message");
    }
}
