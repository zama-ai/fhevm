use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
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

// ── V2 API error body types ──────────────────────────────────────────────────
//
// Two shapes: a simple {label, message} for most errors, and an extended
// variant that adds a `details` array for validation errors.

/// All machine-readable error labels the API can return.
#[derive(ToSchema)]
#[schema(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum V2ErrorLabel {
    MalformedJson,
    MissingFields,
    ValidationFailed,
    RequestError,
    NotAllowedOnHostAcl,
    HostChainIdNotSupported,
    NotFound,
    RateLimited,
    InternalServerError,
    HostAclFailed,
    ProtocolPaused,
    InsufficientBalance,
    InsufficientAllowance,
    GatewayNotReachable,
    ReadinessCheckTimedOut,
    ResponseTimedOut,
}

/// Canonical list of all error labels, kept in sync with `V2ErrorLabel` and
/// the constructor methods on `V2ErrorResponseBody`.
///
/// Used by `openapi-export` to verify its post-processor labels haven't drifted.
/// Labels defined but not yet wired to any handler endpoint.
///
/// These are exempt from the "every label must appear in the catalog" test.
pub const UNWIRED_LABELS: &[&str] = &["gateway_not_reachable"];

/// Derives the canonical label list from [`ERROR_LABEL_DEFS`].
pub fn all_error_labels() -> Vec<&'static str> {
    crate::http::openapi::expected_labels::ERROR_LABEL_DEFS
        .iter()
        .map(|d| d.label)
        .collect()
}

/// Simple error body — used for 400 (no details), 404, 429, 500, and 503.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct V2ApiError {
    /// Machine-readable error label for client UX logic.
    #[schema(value_type = V2ErrorLabel, example = "internal_server_error")]
    pub label: String,
    /// Human-readable error message.
    #[schema(example = "Internal server error")]
    pub message: String,
}

/// Extended error body — used for 400 responses with field-level details.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct V2ApiErrorWithDetails {
    /// Machine-readable error label for client UX logic.
    #[schema(value_type = V2ErrorLabel, example = "validation_failed")]
    pub label: String,
    /// Human-readable error message.
    #[schema(example = "Request validation failed")]
    pub message: String,
    /// Per-field validation issues.
    pub details: Vec<RelayerV2ErrorDetail>,
}

#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct RelayerV2ErrorDetail {
    #[schema(example = "contractAddress")]
    pub field: String,
    #[schema(example = "Must be a valid 42-character hex address with 0x prefix")]
    pub issue: String,
}

/// Union type for all V2 API error bodies.
///
/// Used as the concrete type for the `error` field in status and failed
/// responses. The `#[serde(untagged)]` attribute ensures the JSON output is
/// a flat `{label, message, ...}` object without a discriminator key.
///
/// **Deserialization order matters**: `WithDetails` is tried first so that
/// the `details` array is not silently dropped.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum V2ErrorResponseBody {
    WithDetails(V2ApiErrorWithDetails),
    Simple(V2ApiError),
}

impl V2ErrorResponseBody {
    /// Returns the error label string for assertions and logging.
    pub fn label(&self) -> &str {
        match self {
            Self::Simple(e) => &e.label,
            Self::WithDetails(e) => &e.label,
        }
    }

    /// Returns the error message string.
    pub fn message(&self) -> &str {
        match self {
            Self::Simple(e) => &e.message,
            Self::WithDetails(e) => &e.message,
        }
    }

    /// Returns the details array if this is a with-details variant.
    pub fn details(&self) -> Option<&[RelayerV2ErrorDetail]> {
        match self {
            Self::WithDetails(e) => Some(&e.details),
            Self::Simple(_) => None,
        }
    }
}

// ── Constructor helpers on V2ErrorResponseBody ─────────────────────────────
//
// Grouped by HTTP status code for clarity.

impl V2ErrorResponseBody {
    // ── 400 Bad Request (no details) ──

    pub fn request_error(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "request_error".to_string(),
            message: message.to_string(),
        })
    }

    pub fn not_allowed_on_host_acl(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "not_allowed_on_host_acl".to_string(),
            message: message.to_string(),
        })
    }

    pub fn host_chain_id_not_supported(chain_id: u64) -> Self {
        Self::Simple(V2ApiError {
            label: "host_chain_id_not_supported".to_string(),
            message: format!(
                "Host chain ID {} is not supported by this relayer",
                chain_id
            ),
        })
    }

    pub fn malformed_json(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "malformed_json".to_string(),
            message: message.to_string(),
        })
    }

    // ── 400 Bad Request (with details) ──

    pub fn validation_failed(message: String, details: Vec<RelayerV2ErrorDetail>) -> Self {
        Self::WithDetails(V2ApiErrorWithDetails {
            label: "validation_failed".to_string(),
            message,
            details,
        })
    }

    pub fn missing_fields(message: String, details: Vec<RelayerV2ErrorDetail>) -> Self {
        Self::WithDetails(V2ApiErrorWithDetails {
            label: "missing_fields".to_string(),
            message,
            details,
        })
    }

    pub fn invalid_signature() -> Self {
        Self::WithDetails(V2ApiErrorWithDetails {
            label: "validation_failed".to_string(),
            message: "Validation failed for 1 field(s)".to_string(),
            details: vec![RelayerV2ErrorDetail {
                field: "signature".to_string(),
                issue: "Signature is invalid".to_string(),
            }],
        })
    }

    // ── 404 Not Found ──

    pub fn not_found(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "not_found".to_string(),
            message: message.to_string(),
        })
    }

    // ── 429 Too Many Requests ──

    pub fn rate_limited(message: String) -> Self {
        Self::Simple(V2ApiError {
            label: "rate_limited".to_string(),
            message,
        })
    }

    // ── 500 Internal Server Error ──

    pub fn internal_server_error(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "internal_server_error".to_string(),
            message: message.to_string(),
        })
    }

    pub fn host_acl_failed(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "host_acl_failed".to_string(),
            message: message.to_string(),
        })
    }

    // ── 503 Service Unavailable ──

    #[allow(dead_code)]
    pub fn protocol_paused(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "protocol_paused".to_string(),
            message: message.to_string(),
        })
    }

    #[allow(dead_code)]
    pub fn insufficient_balance(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "insufficient_balance".to_string(),
            message: message.to_string(),
        })
    }

    #[allow(dead_code)]
    pub fn insufficient_allowance(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "insufficient_allowance".to_string(),
            message: message.to_string(),
        })
    }

    #[allow(dead_code)]
    pub fn gateway_not_reachable(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "gateway_not_reachable".to_string(),
            message: message.to_string(),
        })
    }

    pub fn readiness_check_timed_out(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "readiness_check_timed_out".to_string(),
            message: message.to_string(),
        })
    }

    pub fn response_timed_out(message: &str) -> Self {
        Self::Simple(V2ApiError {
            label: "response_timed_out".to_string(),
            message: message.to_string(),
        })
    }
}

// ── Response wrappers ────────────────────────────────────────────────────────

/// Failed response wrapper (POST error responses).
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RelayerV2ResponseFailed {
    #[schema(value_type = String, example = "failed")]
    pub status: ApiResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: Option<String>,
    pub error: V2ErrorResponseBody,
}

// ── Per-status-code GET response schemas (for OpenAPI documentation) ──────────
//
// These types exist solely for the utoipa `responses(...)` annotations on GET
// endpoints so that each HTTP status code has its own schema describing exactly
// which fields are present. The runtime handler code continues to use the
// endpoint-specific `*StatusResponseJson` struct.

/// GET 202 — request is still queued/processing (no result, no error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct V2StatusQueued {
    #[schema(value_type = String, example = "queued")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
}

/// GET 4xx/5xx — request failed (no result, has error).
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct V2StatusFailed {
    #[schema(value_type = String, example = "failed")]
    pub status: ApiResponseStatus,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub request_id: String,
    pub error: V2ErrorResponseBody,
}

// ── Helper methods for RelayerV2ResponseFailed ───────────────────────────────

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
                error: V2ErrorResponseBody::host_chain_id_not_supported(chain_id),
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
                error: V2ErrorResponseBody::request_error(message),
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
                        error: V2ErrorResponseBody::malformed_json(message),
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

                let details = vec![RelayerV2ErrorDetail {
                    field: field_name.clone(),
                    issue: detail_issue,
                }];

                let error = if label == "missing_fields" {
                    V2ErrorResponseBody::missing_fields(message, details)
                } else {
                    V2ErrorResponseBody::validation_failed(message, details)
                };

                (
                    status_code,
                    Json(Self {
                        status: ApiResponseStatus::Failed,
                        request_id: Some(request_id.to_string()),
                        error,
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

                let label = "validation_failed";
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
                        error: V2ErrorResponseBody::validation_failed(message, details),
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
            error: V2ErrorResponseBody::rate_limited(message),
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
                error: V2ErrorResponseBody::internal_server_error(message),
            }),
        )
    }

    /// Creates an internal server error response (500) without a request ID.
    ///
    /// Used by endpoints that have no request lifecycle (e.g. keyurl).
    pub fn internal_server_error_simple(message: &str) -> (StatusCode, Json<Self>) {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let label = "internal_server_error";

        info!(
            http_status = status_code.as_u16(),
            label, message, "HTTP response"
        );

        (
            status_code,
            Json(Self {
                status: ApiResponseStatus::Failed,
                request_id: None,
                error: V2ErrorResponseBody::internal_server_error(message),
            }),
        )
    }
}

/// Classify an error by its ABI revert selector and return the matching HTTP status + JSON body.
///
/// The contract can revert with many selectors — we validate most conditions before
/// sending the transaction, so only a subset can appear here. Plain-text messages
/// (no selector) fall through to Unknown / 500.
pub fn classify_revert_error(error_msg: &str) -> (StatusCode, V2ErrorResponseBody) {
    use crate::gateway::utils::{classify_revert_selector, extract_revert_selector, RevertReason};

    let reason = if let Some(selector) = extract_revert_selector(error_msg) {
        classify_revert_selector(&selector)
    } else {
        RevertReason::Unknown
    };

    match reason {
        RevertReason::ContractPaused => (
            StatusCode::SERVICE_UNAVAILABLE,
            V2ErrorResponseBody::protocol_paused(error_msg),
        ),
        RevertReason::InsufficientBalance => (
            StatusCode::SERVICE_UNAVAILABLE,
            V2ErrorResponseBody::insufficient_balance(error_msg),
        ),
        RevertReason::InsufficientAllowance => (
            StatusCode::SERVICE_UNAVAILABLE,
            V2ErrorResponseBody::insufficient_allowance(error_msg),
        ),
        RevertReason::InvalidSignature => (
            StatusCode::BAD_REQUEST,
            V2ErrorResponseBody::invalid_signature(),
        ),
        RevertReason::Unknown => (
            StatusCode::INTERNAL_SERVER_ERROR,
            V2ErrorResponseBody::internal_server_error(error_msg),
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
        assert_eq!(response.error.label(), "request_error");
    }

    #[test]
    fn test_internal_server_error_has_status_and_request_id() {
        let (status_code, json) = RelayerV2ResponseFailed::internal_server_error("test-request-id");

        assert_eq!(status_code, StatusCode::INTERNAL_SERVER_ERROR);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, Some("test-request-id".to_string()));
        assert_eq!(response.error.label(), "internal_server_error");
    }

    #[test]
    fn test_internal_server_error_simple_has_no_request_id() {
        let (status_code, json) =
            RelayerV2ResponseFailed::internal_server_error_simple("Key URL not yet initialized");

        assert_eq!(status_code, StatusCode::INTERNAL_SERVER_ERROR);

        let response = json.0;
        assert_eq!(response.status, ApiResponseStatus::Failed);
        assert_eq!(response.request_id, None);
        assert_eq!(response.error.label(), "internal_server_error");
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
        assert_eq!(response.error.label(), "malformed_json");
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
        assert_eq!(response.error.label(), "missing_fields");

        let details = response.error.details().expect("Should have details");
        assert_eq!(details[0].field, "contractChainId");
        assert_eq!(details[0].issue, "Required but missing");
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
        assert_eq!(response.error.label(), "validation_failed");
    }

    #[test]
    fn test_serialized_error_response_format() {
        let (_, json) = RelayerV2ResponseFailed::request_error("test error", "test-request-id");

        let serialized = serde_json::to_string(&json.0).expect("Failed to serialize");
        let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("Failed to parse");

        // Verify the top-level structure has status, requestId, and error fields
        assert!(parsed.get("status").is_some(), "Should have status field");
        assert!(
            parsed.get("requestId").is_some(),
            "Should have requestId field"
        );
        assert!(parsed.get("error").is_some(), "Should have error field");

        // Verify the error has the expected structure (untagged serialization = flat object)
        let error = parsed.get("error").unwrap();
        assert!(error.get("label").is_some(), "Error should have label");
        assert!(error.get("message").is_some(), "Error should have message");
    }

    /// Verify every constructor produces a label that is in `all_error_labels()`.
    ///
    /// If this test fails, a new constructor was added without updating
    /// `ERROR_LABEL_DEFS` (or vice-versa).
    #[test]
    fn all_constructor_labels_are_in_canonical_list() {
        let all_labels = all_error_labels();
        let detail = vec![RelayerV2ErrorDetail {
            field: "f".into(),
            issue: "i".into(),
        }];

        let constructed: Vec<V2ErrorResponseBody> = vec![
            V2ErrorResponseBody::malformed_json("m"),
            V2ErrorResponseBody::missing_fields("m".into(), detail.clone()),
            V2ErrorResponseBody::validation_failed("m".into(), detail),
            V2ErrorResponseBody::request_error("m"),
            V2ErrorResponseBody::not_allowed_on_host_acl("m"),
            V2ErrorResponseBody::host_chain_id_not_supported(1),
            V2ErrorResponseBody::not_found("m"),
            V2ErrorResponseBody::rate_limited("m".into()),
            V2ErrorResponseBody::internal_server_error("m"),
            V2ErrorResponseBody::host_acl_failed("m"),
            V2ErrorResponseBody::protocol_paused("m"),
            V2ErrorResponseBody::insufficient_balance("m"),
            V2ErrorResponseBody::insufficient_allowance("m"),
            V2ErrorResponseBody::gateway_not_reachable("m"),
            V2ErrorResponseBody::readiness_check_timed_out("m"),
            V2ErrorResponseBody::response_timed_out("m"),
        ];

        for err in &constructed {
            assert!(
                all_labels.contains(&err.label()),
                "Label {:?} from constructor is missing from all_error_labels()",
                err.label()
            );
        }

        // Reverse check: every label in the list must be produced by some constructor
        let produced: Vec<&str> = constructed.iter().map(|e| e.label()).collect();
        for label in &all_labels {
            assert!(
                produced.contains(label),
                "all_error_labels() contains {:?} but no constructor produces it",
                label
            );
        }
    }
}
