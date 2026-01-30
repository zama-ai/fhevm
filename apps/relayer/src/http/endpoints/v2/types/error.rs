use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::http::utils::responses::{to_camel_case, FieldJsonErrorType, ParseError};

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
    /// Creates a request error response (400) for body read failures, etc.
    pub fn request_error(message: &str, request_id: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(Self {
                status: "failed".to_string(),
                request_id: Some(request_id.to_string()),
                error: RelayerV2ApiError400NoDetails::validation_error(message),
            }),
        )
    }

    /// Creates an error response from a ParseError
    pub fn from_parse_error(parse_error: &ParseError, request_id: &str) -> (StatusCode, Json<Self>) {
        match parse_error {
            ParseError::MalformedJson(message) => (
                StatusCode::BAD_REQUEST,
                Json(Self {
                    status: "failed".to_string(),
                    request_id: Some(request_id.to_string()),
                    error: serde_json::to_value(RelayerV2ApiError400NoDetails {
                        label: "malformed_json".to_string(),
                        message: message.clone(),
                    })
                    .unwrap(),
                }),
            ),
            ParseError::FieldSpecificJson {
                field_name,
                issue,
                error_type,
            } => {
                let field_name = to_camel_case(field_name);
                let (label, message) = match error_type {
                    FieldJsonErrorType::Missing => (
                        "missing_fields".to_string(),
                        format!("Missing 1 required field in the request: {}", field_name),
                    ),
                    FieldJsonErrorType::InvalidType | FieldJsonErrorType::Unknown => (
                        "validation_failed".to_string(),
                        format!("Validation failed for 1 field in the request: {}", field_name),
                    ),
                };
                (
                    StatusCode::BAD_REQUEST,
                    Json(Self {
                        status: "failed".to_string(),
                        request_id: Some(request_id.to_string()),
                        error: serde_json::to_value(RelayerV2ApiError400WithDetails {
                            label,
                            message,
                            details: vec![RelayerV2ErrorDetail {
                                field: field_name,
                                issue: issue.clone(),
                            }],
                        })
                        .unwrap(),
                    }),
                )
            }
            ParseError::ValidationFailed(errors) => {
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

                (
                    StatusCode::BAD_REQUEST,
                    Json(Self {
                        status: "failed".to_string(),
                        request_id: Some(request_id.to_string()),
                        error: serde_json::to_value(RelayerV2ApiError400WithDetails {
                            label: "validation_failed".to_string(),
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

        let response = Self {
            status: "failed".to_string(),
            request_id: Some(request_id.to_string()),
            error: serde_json::to_value(RelayerV2ApiError429 {
                label: "rate_limited".to_string(),
                message: format!("Server is experiencing high processing load: {}", reason),
            })
            .unwrap(),
        };

        let mut http_response = (StatusCode::TOO_MANY_REQUESTS, Json(response)).into_response();

        // Add Retry-After header
        if let Ok(header_value) = retry_after.parse() {
            http_response.headers_mut().insert(header::RETRY_AFTER, header_value);
        }

        http_response
    }

    /// Creates an internal server error response (500)
    pub fn internal_server_error(request_id: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Self {
                status: "failed".to_string(),
                request_id: Some(request_id.to_string()),
                error: RelayerV2ApiError500::internal_server_error("Internal server error"),
            }),
        )
    }

    /// Creates a service unavailable response (503)
    pub fn service_unavailable(message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(Self {
                status: "failed".to_string(),
                request_id: None,
                error: RelayerV2ApiError503::readiness_check_timed_out(message),
            }),
        )
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
        assert_eq!(response.status, "failed");
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "request_error");
    }

    #[test]
    fn test_internal_server_error_has_status_and_request_id() {
        let (status_code, json) =
            RelayerV2ResponseFailed::internal_server_error("test-request-id");

        assert_eq!(status_code, StatusCode::INTERNAL_SERVER_ERROR);

        let response = json.0;
        assert_eq!(response.status, "failed");
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "internal_server_error");
    }

    #[test]
    fn test_service_unavailable_has_status_no_request_id() {
        let (status_code, json) =
            RelayerV2ResponseFailed::service_unavailable("Key URL not yet initialized");

        assert_eq!(status_code, StatusCode::SERVICE_UNAVAILABLE);

        let response = json.0;
        assert_eq!(response.status, "failed");
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
        assert_eq!(response.status, "failed");
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
        assert_eq!(response.status, "failed");
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "missing_fields");
        assert!(response.error["details"].is_array());
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
        assert_eq!(response.status, "failed");
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error["label"], "validation_failed");
    }

    #[test]
    fn test_serialized_error_response_format() {
        let (_, json) = RelayerV2ResponseFailed::request_error("test error", "test-request-id");

        let serialized = serde_json::to_string(&json.0).expect("Failed to serialize");
        let parsed: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse");

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
