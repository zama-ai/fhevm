use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::ValidationErrors;

/// Error type for parsing and validation operations.
/// This enum provides clean separation between parsing/validation logic and HTTP response creation.
#[derive(Debug)]
pub enum ParseError {
    /// JSON syntax error or malformed JSON
    MalformedJson(String),
    /// Field-specific JSON issues (missing field, wrong type, unknown field)
    FieldSpecificJson {
        field_name: String,
        issue: String,
        error_type: FieldJsonErrorType,
    },
    /// Validation errors from the validator
    ValidationFailed(ValidationErrors),
    /// Conversion error from JsonType to RequestType
    ConversionFailed(String),
}

/// Type of field-specific JSON error
#[derive(Debug)]
pub enum FieldJsonErrorType {
    Missing,
    InvalidType,
    Unknown,
}

impl ParseError {
    /// Convert ParseError to AppResponse with request_id
    pub fn to_app_response<V: serde::Serialize>(&self, request_id: &str) -> AppResponse<V> {
        let mut response = match self {
            ParseError::MalformedJson(message) => AppResponse::malformed_json(message),
            ParseError::FieldSpecificJson {
                field_name,
                issue,
                error_type,
            } => {
                let mut errors = ValidationErrors::new();

                // Map field name to static str
                let field_key: &'static str = match field_name.as_str() {
                    "contractChainId" => "contract_chain_id",
                    "contractsChainId" => "contracts_chain_id",
                    "contractAddress" => "contract_address",
                    "contractAddresses" => "contract_addresses",
                    "userAddress" => "user_address",
                    "ciphertextWithInputVerification" => "ciphertext_with_input_verification",
                    "extraData" => "extra_data",
                    "ciphertextHandles" => "ciphertext_handles",
                    "handleContractPairs" => "handle_contract_pairs",
                    "requestValidity" => "request_validity",
                    "publicKey" => "public_key",
                    "signature" => "signature",
                    _ => "request",
                };

                let mut error = validator::ValidationError::new("json_error");
                error.message = Some(issue.clone().into());
                errors.add(field_key, error);

                match error_type {
                    FieldJsonErrorType::Missing => AppResponse::missing_fields(errors),
                    FieldJsonErrorType::InvalidType | FieldJsonErrorType::Unknown => {
                        AppResponse::validation_failed(errors)
                    }
                }
            }
            ParseError::ValidationFailed(errors) => AppResponse::validation_failed(errors.clone()),
            ParseError::ConversionFailed(message) => {
                tracing::error!(
                    "Internal error: Conversion failed after validation passed: {}",
                    message
                );
                AppResponse::internal_server_error("Internal processing error")
            }
        };

        response.set_request_id(request_id);
        response
    }
}

/// A generic enum to handle various API response types.
/// The generic parameter `V` allows the success variant to hold any serializable data.
#[derive(Debug)]
pub enum AppResponse<V: serde::Serialize> {
    Success(V),
    BadRequest {
        label: ErrorLabel,
        message: String,
        details: Option<Vec<ErrorDetail>>,
        request_id: Option<String>,
    },
    InternalServerError {
        label: ErrorLabel,
        message: String,
        request_id: Option<String>,
    },
    TooManyRequests {
        label: ErrorLabel,
        message: String,
        retry_after: String,
        request_id: Option<String>,
    },
}

impl<V: serde::Serialize> AppResponse<V> {
    /// Creates a new success response with the given data.
    pub fn success(data: V) -> Self {
        AppResponse::Success(data)
    }

    /// Creates a new malformed JSON response (for JSON syntax errors).
    pub fn malformed_json<S: Into<String>>(message: S) -> Self {
        AppResponse::BadRequest {
            label: ErrorLabel::MalformedJson,
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Creates a new request error response (for body read failures, etc.).
    pub fn request_error<S: Into<String>>(message: S) -> Self {
        AppResponse::BadRequest {
            label: ErrorLabel::RequestError,
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Creates a new missing fields response.
    pub fn missing_fields(errors: validator::ValidationErrors) -> Self {
        use crate::http::utils::validation_messages;

        // For missing fields, create details with empty issue strings
        let details: Vec<ErrorDetail> = errors
            .field_errors()
            .keys()
            .map(|field| ErrorDetail {
                field: to_camel_case(field),
                issue: validation_messages::GENERIC_REQUIRED_BUT_MISSING.to_string(),
            })
            .collect();

        let field_names: Vec<String> = details.iter().map(|d| d.field.clone()).collect();
        let count = field_names.len();

        let message = if count == 1 {
            format!(
                "Missing 1 required field in the request: {}",
                field_names[0]
            )
        } else {
            format!(
                "Missing {} required fields in the request: {}",
                count,
                field_names.join(", ")
            )
        };

        AppResponse::BadRequest {
            label: ErrorLabel::MissingFields,
            message,
            details: if details.is_empty() {
                None
            } else {
                Some(details)
            },
            request_id: None,
        }
    }

    /// Creates a new validation error response.
    pub fn validation_failed(errors: validator::ValidationErrors) -> Self {
        let details = Self::extract_field_details(errors);
        let field_names: Vec<String> = details.iter().map(|d| d.field.clone()).collect();
        let count = field_names.len();

        let message = if count == 1 {
            format!(
                "Validation failed for 1 field in the request: {}",
                field_names[0]
            )
        } else {
            format!(
                "Validation failed for {} fields in the request: {}",
                count,
                field_names.join(", ")
            )
        };

        AppResponse::BadRequest {
            label: ErrorLabel::ValidationFailed,
            message,
            details: if details.is_empty() {
                None
            } else {
                Some(details)
            },
            request_id: None,
        }
    }

    /// @deprecated Use request_error instead
    pub fn bad_request<S: Into<String>>(message: S) -> Self {
        Self::request_error(message)
    }

    /// @deprecated Use missing_fields or validation_failed instead
    pub fn invalid_request(errors: validator::ValidationErrors) -> Self {
        Self::validation_failed(errors)
    }

    /// Helper to extract field details from validation errors
    fn extract_field_details(errors: validator::ValidationErrors) -> Vec<ErrorDetail> {
        errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors.iter().map(move |e| ErrorDetail {
                    field: to_camel_case(field),
                    issue: e
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid {}", field)),
                })
            })
            .collect()
    }

    /// Creates a new internal server error response with the given message.
    pub fn internal_server_error<S: Into<String>>(message: S) -> Self {
        AppResponse::InternalServerError {
            label: ErrorLabel::InternalServerError,
            message: message.into(),
            request_id: None,
        }
    }

    /// Creates a new internal server error response with request ID emphasized in the message.
    pub fn internal_server_error_with_request_id<S: Into<String>>(request_id: S) -> Self {
        let request_id_str = request_id.into();
        AppResponse::InternalServerError {
            label: ErrorLabel::InternalServerError,
            message: "Internal server error".to_string(),
            request_id: Some(request_id_str),
        }
    }

    /// Creates a new rate limited response.
    pub fn rate_limited<S: Into<String>>(reason: S, retry_after: S) -> Self {
        let reason: String = reason.into();
        AppResponse::TooManyRequests {
            label: ErrorLabel::RateLimited,
            message: format!("Rate limit exceeded: {}", reason).to_string(),
            retry_after: retry_after.into(),
            request_id: None,
        }
    }

    /// Creates a new protocol overloaded response.
    ///
    /// TEMPORARY FIX FOR AUCTION: Currently returns `rate_limited` label instead of
    /// `protocol_overload` to enable rate limiter retry logic without additional client changes.
    /// This should be restored to the correct version (commented below) after the auction.
    pub fn protocol_overloaded<S: Into<String>>(reason: S, retry_after: S, request_id: S) -> Self {
        let reason: String = reason.into();
        let request_id_str = request_id.into();
        AppResponse::TooManyRequests {
            label: ErrorLabel::RateLimited,
            message: format!("Server is experiencing high processing load: {}", reason).to_string(),
            retry_after: retry_after.into(),
            request_id: Some(request_id_str),
        }
    }

    // ORIGINAL VERSION (commented out temporarily for auction):
    // /// Creates a new protocol overloaded response.
    // pub fn protocol_overloaded<S: Into<String>>(reason: S, retry_after: S, request_id: S) -> Self {
    //     let reason: String = reason.into();
    //     let request_id_str = request_id.into();
    //     AppResponse::TooManyRequests {
    //         label: ErrorLabel::ProtocolOverload,
    //         message: format!("Protocol overloaded: {}", reason).to_string(),
    //         retry_after: retry_after.into(),
    //         request_id: Some(request_id_str),
    //     }
    // }

    /// Creates a host chain ID not supported response.
    pub fn host_chain_id_not_supported(chain_id: u64) -> Self {
        AppResponse::BadRequest {
            label: ErrorLabel::HostChainIdNotSupported,
            message: format!(
                "Host chain ID {} is not supported by this relayer",
                chain_id
            ),
            details: None,
            request_id: None,
        }
    }

    /// Sets the request ID for error responses
    pub fn set_request_id(&mut self, request_id: &str) {
        match self {
            AppResponse::BadRequest {
                request_id: ref mut rid,
                ..
            } => {
                *rid = Some(request_id.to_string());
            }
            AppResponse::InternalServerError {
                request_id: ref mut rid,
                ..
            } => {
                *rid = Some(request_id.to_string());
            }
            AppResponse::TooManyRequests {
                request_id: ref mut rid,
                ..
            } => {
                *rid = Some(request_id.to_string());
            }
            AppResponse::Success(_) => {
                // Success responses don't need request IDs in error context
            }
        }
    }
}

/// Non changeable without integrating a breaking change.
/// This is used by the client to create UX logic on this code.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = "validation_failed")]
pub enum ErrorLabel {
    /// JSON syntax error or malformed JSON
    MalformedJson,
    /// Required fields are missing from the request
    MissingFields,
    /// Field validation failed (invalid format, out of range, etc.)
    ValidationFailed,
    /// General request errors (body read failures, version errors, etc.)
    RequestError,
    /// Rate limit exceeded
    RateLimited,
    /// Internal server processing error
    InternalServerError,
    /// Protocol Overload used for bouncing
    ProtocolOverload,
    /// Host chain ID not supported
    HostChainIdNotSupported,
}

impl ErrorLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorLabel::MalformedJson => "malformed_json",
            ErrorLabel::MissingFields => "missing_fields",
            ErrorLabel::ValidationFailed => "validation_failed",
            ErrorLabel::RequestError => "request_error",
            ErrorLabel::RateLimited => "rate_limited",
            ErrorLabel::InternalServerError => "internal_server_error",
            ErrorLabel::ProtocolOverload => "protocol_overload",
            ErrorLabel::HostChainIdNotSupported => "host_chain_id_not_supported",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct ErrorDetail {
    pub field: String,
    pub issue: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct ApiError {
    pub label: ErrorLabel,
    pub message: String,
    pub request_id: Option<String>,
    /// Relative seconds indicating when client should retry (e.g. "10").
    /// Only used in the case of Rate limit errors.
    #[schema(example = "10")]
    pub retry_after: Option<String>,
    /// Only used in Bad Requests
    pub details: Option<Vec<ErrorDetail>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: ApiError,
}

// Implement `IntoResponse` so Axum can convert our enum into an HTTP response.
impl<V: serde::Serialize> IntoResponse for AppResponse<V> {
    fn into_response(self) -> Response {
        use axum::http::HeaderValue;
        match self {
            AppResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            AppResponse::BadRequest {
                label,
                message,
                details,
                request_id,
            } => {
                let api_error = ApiError {
                    label,
                    message,
                    request_id,
                    retry_after: None,
                    details,
                };

                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::InternalServerError {
                label,
                message,
                request_id,
            } => {
                let api_error = ApiError {
                    label,
                    message,
                    request_id,
                    retry_after: None,
                    details: None,
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::TooManyRequests {
                label,
                message,
                retry_after,
                request_id,
            } => {
                // For 429 errors, retry_after should only be in header, not in body
                let api_error = ApiError {
                    label,
                    message,
                    request_id,
                    retry_after: None, // For 429, retry_after only in header. This will be used for 202.
                    details: None,
                };

                let mut response = (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response();

                // Add Retry-After header with the timestamp
                if let Ok(header_value) = HeaderValue::from_str(&retry_after) {
                    response.headers_mut().insert("Retry-After", header_value);
                }

                response
            }
        }
    }
}

/// Converts a string to camel case.
///
/// The string is split at each underscore and dash, and the first letter of each
/// resulting substring is capitalized. All other characters are converted to
/// lower case.
pub fn to_camel_case<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c.is_whitespace() || c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}
