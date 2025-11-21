use crate::{
    http::userdecrypt_http_listener::RequestValidityJson,
    orchestrator::traits::{Event, EventHandler},
};
use alloy::primitives::U256;
use axum::{
    body::Bytes,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{de, de::DeserializeOwned, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;
use validator::{ValidationError, ValidationErrors};

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

                let mut error = ValidationError::new("json_error");
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

// Generic validation error messages (reusable across fields)
pub mod validation_messages {
    pub const GENERIC_REQUIRED_BUT_MISSING: &str = "Required but missing";

    pub const NUMBER_DECIMAL_OR_HEX: &str = "Must be decimal number or 0x hex string";

    pub const HEX_MUST_START_WITH_0X: &str = "Must start with 0x";
    pub const HEX_MUST_NOT_START_WITH_0X: &str = "Must not start with 0x";
    pub const HEX_INVALID_CHARACTERS: &str = "Contains invalid hex characters";
    pub const HEX_INVALID_STRING: &str = "Invalid hex string";

    // Generic length validation messages
    pub const LENGTH_MUST_BE_42_CHARACTERS: &str = "Must be 42 characters long"; // Keep for backward compatibility
    pub const LENGTH_MUST_BE_64_CHARACTERS: &str = "Must be 64 characters long"; // Keep for backward compatibility
    pub const LENGTH_MUST_BE_132_CHARACTERS: &str = "Must be 132 characters long";

    // Generic collection validation messages
    pub const MUST_NOT_BE_EMPTY: &str = "Must not be empty";

    pub const EXACT_MUST_BE_0X00: &str = "Must be 0x00";
    pub const TIMESTAMP_MUST_NOT_BE_IN_FUTURE: &str = "Timestamp must not be in the future";
}

pub struct OnceHandler<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
}

impl<T> OnceHandler<T> {
    pub fn new() -> (Self, oneshot::Receiver<T>) {
        let (tx, rx) = oneshot::channel();
        (
            Self {
                tx: Mutex::new(Some(tx)),
            },
            rx,
        )
    }
}

#[async_trait::async_trait]
impl<E> EventHandler<E> for OnceHandler<E>
where
    E: Event + Send + Sync + 'static,
{
    async fn handle_event(&self, event: E) {
        let mut lock = self.tx.lock().unwrap();
        if let Some(tx) = lock.take() {
            let _ = tx.send(event);
        }
    }
}

pub fn de_string_or_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(num) => format!("{num}"),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

// Custom validation function for a standard Ethereum-style blockchain address.
// It must start with "0x", be 42 characters long, and contain hex characters.
pub fn validate_blockchain_address(address: &str) -> Result<(), ValidationError> {
    if !address.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_START_WITH_0X.into()));
    }
    if address.len() != 42 {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::LENGTH_MUST_BE_42_CHARACTERS.into()));
    }
    // The `hex` crate robustly checks if the string slice (after "0x") is valid hex.
    if hex::decode(&address[2..]).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_CHARACTERS.into()));
    }
    Ok(())
}

pub fn validate_blockchain_addresses(addresses: &Vec<String>) -> Result<(), ValidationError> {
    for address in addresses {
        validate_blockchain_address(address)?;
    }
    Ok(())
}

pub fn validate_no_0x_hex(hex_str: &str) -> Result<(), ValidationError> {
    if hex_str.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_NOT_START_WITH_0X.into()));
    };

    if hex::decode(hex_str).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_STRING.into()));
    }
    Ok(())
}

pub fn validate_0x_hex(hex_str: &str) -> Result<(), ValidationError> {
    if !hex_str.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_START_WITH_0X.into()));
    };

    if hex::decode(&hex_str[2..]).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_STRING.into()));
    }
    Ok(())
}

pub fn validate_0x_hexs(hex_strs: &Vec<String>) -> Result<(), ValidationError> {
    for hex_str in hex_strs {
        validate_0x_hex(hex_str)?;
    }
    Ok(())
}

pub fn validate_extra_data_field(extra_data: &str) -> Result<(), ValidationError> {
    if extra_data != "0x00" {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::EXACT_MUST_BE_0X00.into()));
    }
    Ok(())
}

pub fn validate_u32_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u32>().is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Value must be a valid u32 number".into()));
    }
    Ok(())
}

pub fn validate_u64_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u64>().is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Value must be a valid u64 number".into()));
    }
    Ok(())
}

pub fn validate_timestamp(value: &str) -> Result<(), ValidationError> {
    let u256_value = U256::from_str(value).map_err(|_| {
        ValidationError::new("validation_error")
            .with_message("Value must be a valid U256 number".into())
    })?;

    // U256 to u64 conversion is truncating. It's safe for timestamps for the foreseeable future.
    let timestamp = u256_value.to::<u64>();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            ValidationError::new("internal_server_error")
                .with_message("System time is before UNIX epoch.".into())
        })?
        .as_secs();

    if timestamp > now {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::TIMESTAMP_MUST_NOT_BE_IN_FUTURE.into()));
    }

    Ok(())
}

pub fn validate_chain_id_string(value: &str) -> Result<(), ValidationError> {
    // Match the logic in parse_chain_id() function
    let result = if let Some(stripped) = value.strip_prefix("0x") {
        // Parse as hex if it starts with 0x
        u64::from_str_radix(stripped, 16)
    } else {
        // Parse as decimal otherwise
        value.parse::<u64>()
    };

    if result.is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::NUMBER_DECIMAL_OR_HEX.into()));
    }
    Ok(())
}

pub fn validate_handle_contract_pairs(
    pairs: &Vec<crate::http::userdecrypt_http_listener::HandleContractPairJson>,
) -> Result<(), ValidationError> {
    for pair in pairs {
        validate_0x_hex(&pair.handle)?;

        // Validate handle length
        if pair.handle.len() != 66 {
            return Err(ValidationError::new("validation_error")
                .with_message(validation_messages::LENGTH_MUST_BE_64_CHARACTERS.into()));
        }

        // Validate contract address
        validate_blockchain_address(&pair.contract_address)?;
    }
    Ok(())
}

pub fn validate_request_validity(
    request_validity: &RequestValidityJson,
) -> Result<(), ValidationError> {
    validate_timestamp(&request_validity.start_timestamp)?;
    validate_u32_string(&request_validity.duration_days)?;
    Ok(())
}

pub fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}

/// Generic parser function that handles JSON parsing, validation, and conversion in one place.
/// This consolidates all parsing logic and ensures consistent error handling across endpoints.
/// Returns a clean ParseError that can be converted to an AppResponse by the HTTP handlers.
pub fn parse_and_validate<JsonType, RequestType>(body: &[u8]) -> Result<RequestType, ParseError>
where
    JsonType: DeserializeOwned + validator::Validate,
    RequestType: TryFrom<JsonType>,
    <RequestType as TryFrom<JsonType>>::Error: std::fmt::Display,
{
    // 1. Parse JSON with custom error handling
    // TODO: Change the serde json parser by a custom parser for populating all errors.
    let payload: JsonType = match serde_json::from_slice(body) {
        Ok(payload) => payload,
        Err(e) => {
            let error_msg = e.to_string();

            // Check if it's a field-specific error vs true JSON syntax error
            if error_msg.contains("missing field")
                || error_msg.contains("invalid type")
                || error_msg.contains("unknown field")
            {
                // Field-specific JSON issues
                let field_name = extract_field_from_serde_error(&e);
                let issue = format_serde_error_message(&e);

                let error_type = if error_msg.contains("missing field") {
                    FieldJsonErrorType::Missing
                } else if error_msg.contains("unknown field") {
                    FieldJsonErrorType::Unknown
                } else {
                    FieldJsonErrorType::InvalidType
                };

                return Err(ParseError::FieldSpecificJson {
                    field_name,
                    issue,
                    error_type,
                });
            } else {
                // True malformed JSON syntax error
                return Err(ParseError::MalformedJson("Invalid JSON format".to_string()));
            }
        }
    };

    // 2. Validate the parsed payload
    if let Err(errors) = payload.validate() {
        return Err(ParseError::ValidationFailed(errors));
    }

    // 3. Convert to final request type
    match RequestType::try_from(payload) {
        Ok(request) => Ok(request),
        Err(error) => {
            // If validation passed but conversion failed, this is an internal error
            Err(ParseError::ConversionFailed(error.to_string()))
        }
    }
}

/// Custom JSON extractor that handles both parsing and validation errors consistently.
/// Uses our standardized error response format for all failures.
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + validator::Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extract raw body
        let body = match Bytes::from_request(req, state).await {
            Ok(body) => body,
            Err(_) => {
                return Err(
                    AppResponse::<()>::request_error("Failed to read request body").into_response(),
                );
            }
        };

        // 2. Parse JSON with custom error handling
        let payload: T = match serde_json::from_slice(&body) {
            Ok(payload) => payload,
            Err(e) => {
                let error_msg = e.to_string();

                // Check if it's a field-specific error (missing field, wrong type, unknown field)
                // vs true JSON syntax error
                if error_msg.contains("missing field")
                    || error_msg.contains("invalid type")
                    || error_msg.contains("unknown field")
                {
                    // Field-specific JSON issues - use validation error structure
                    let mut errors = validator::ValidationErrors::new();
                    let field_name = extract_field_from_serde_error(&e);
                    let issue = format_serde_error_message(&e);

                    // Map field name to static str to satisfy lifetime requirements
                    let field_key: &'static str = match field_name.as_str() {
                        "contractChainId" => "contract_chain_id",
                        "contractAddress" => "contract_address",
                        "userAddress" => "user_address",
                        "ciphertextWithInputVerification" => "ciphertext_with_input_verification",
                        "extraData" => "extra_data",
                        "ciphertextHandles" => "ciphertext_handles",
                        _ => "request",
                    };

                    let mut error = validator::ValidationError::new("json_error");
                    error.message = Some(issue.into());
                    errors.add(field_key, error);

                    if error_msg.contains("missing field") {
                        return Err(AppResponse::<()>::missing_fields(errors).into_response());
                    } else {
                        return Err(AppResponse::<()>::validation_failed(errors).into_response());
                    }
                } else {
                    // True malformed JSON syntax error - no field details
                    return Err(
                        AppResponse::<()>::malformed_json("Invalid JSON format").into_response()
                    );
                }
            }
        };

        // 3. Validate with custom error handling
        if let Err(errors) = payload.validate() {
            return Err(AppResponse::<()>::validation_failed(errors).into_response());
        }

        Ok(ValidatedJson(payload))
    }
}

/// A generic enum to handle various API response types.
/// The generic parameter `V` allows the success variant to hold any serializable data.
#[derive(Debug)]
pub enum AppResponse<V: serde::Serialize> {
    Success(V),
    BadRequest {
        code: ErrorCode,
        message: String,
        details: Option<Vec<ErrorDetail>>,
        request_id: Option<String>,
    },
    InternalServerError {
        code: ErrorCode,
        message: String,
        request_id: Option<String>,
    },
    TooManyRequests {
        code: ErrorCode,
        message: String,
        reason: String,
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
            code: ErrorCode::MalformedJson,
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Creates a new request error response (for body read failures, etc.).
    pub fn request_error<S: Into<String>>(message: S) -> Self {
        AppResponse::BadRequest {
            code: ErrorCode::RequestError,
            message: message.into(),
            details: None,
            request_id: None,
        }
    }

    /// Creates a new missing fields response.
    pub fn missing_fields(errors: validator::ValidationErrors) -> Self {
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
            format!("Missing required field (1): {}", field_names[0])
        } else {
            format!(
                "Missing required fields ({}): {}",
                count,
                field_names.join(", ")
            )
        };

        AppResponse::BadRequest {
            code: ErrorCode::MissingFields,
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
            format!("Validation failed (1): {}", field_names[0])
        } else {
            format!("Validation failed ({}): {}", count, field_names.join(", "))
        };

        AppResponse::BadRequest {
            code: ErrorCode::ValidationFailed,
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
            code: ErrorCode::InternalServerError,
            message: message.into(),
            request_id: None,
        }
    }

    /// Creates a new rate limited response.
    pub fn rate_limited<S: Into<String>>(reason: S, retry_after: S) -> Self {
        AppResponse::TooManyRequests {
            code: ErrorCode::RateLimited,
            message: "Rate limit exceeded".to_string(),
            reason: reason.into(),
            retry_after: retry_after.into(),
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
pub enum ErrorCode {
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
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::MalformedJson => "malformed_json",
            ErrorCode::MissingFields => "missing_fields",
            ErrorCode::ValidationFailed => "validation_failed",
            ErrorCode::RequestError => "request_error",
            ErrorCode::RateLimited => "rate_limited",
            ErrorCode::InternalServerError => "internal_server_error",
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
    pub code: ErrorCode,
    pub message: String,
    pub request_id: Option<String>,
    /// RFC 7231 timestamp indicating when client should retry (e.g. "Wed, 21 Oct 2015 07:28:00 GMT").
    /// Uses absolute timestamp instead of relative seconds for cache-safety.
    /// retry_after is only used in the case of Rate limit errors.
    #[schema(example = "Thu, 14 Nov 2024 15:30:00 GMT")]
    pub retry_after: Option<String>,
    /// reason is only used in the case of Rate limit errors.
    pub reason: Option<String>,
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
        match self {
            AppResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            AppResponse::BadRequest {
                code,
                message,
                details,
                request_id,
            } => {
                let api_error = ApiError {
                    code,
                    message,
                    request_id,
                    retry_after: None,
                    reason: None,
                    details,
                };

                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::InternalServerError {
                code,
                message,
                request_id,
            } => {
                let api_error = ApiError {
                    code,
                    message,
                    request_id,
                    retry_after: None,
                    reason: None,
                    details: None,
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::TooManyRequests {
                code,
                message,
                reason,
                retry_after,
                request_id,
            } => {
                let api_error = ApiError {
                    code,
                    message,
                    request_id,
                    retry_after: Some(retry_after),
                    reason: Some(reason),
                    details: None,
                };

                (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
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

/// Extract field name from serde JSON error message
fn extract_field_from_serde_error(error: &serde_json::Error) -> String {
    let error_msg = error.to_string();

    // Pattern: "missing field `fieldName`"
    if let Some(start) = error_msg.find("missing field `") {
        let start = start + "missing field `".len();
        if let Some(end) = error_msg[start..].find('`') {
            return error_msg[start..start + end].to_string();
        }
    }

    // Pattern: "invalid type: ... for key `fieldName`"
    if let Some(start) = error_msg.find(" for key `") {
        let start = start + " for key `".len();
        if let Some(end) = error_msg[start..].find('`') {
            return error_msg[start..start + end].to_string();
        }
    }

    // Pattern: "unknown field `fieldName`"
    if let Some(start) = error_msg.find("unknown field `") {
        let start = start + "unknown field `".len();
        if let Some(end) = error_msg[start..].find('`') {
            return error_msg[start..start + end].to_string();
        }
    }

    // Fallback: use generic field name
    "request".to_string()
}

/// Format serde error message to be user-friendly
fn format_serde_error_message(error: &serde_json::Error) -> String {
    let error_msg = error.to_string();

    if error_msg.contains("missing field") {
        "This field is required".to_string()
    } else if error_msg.contains("invalid type") {
        if error_msg.contains("expected a string") {
            "Value must be a string".to_string()
        } else if error_msg.contains("expected a number") {
            "Value must be a number".to_string()
        } else if error_msg.contains("expected a boolean") {
            "Value must be true or false".to_string()
        } else {
            "Invalid data type".to_string()
        }
    } else if error_msg.contains("unknown field") {
        "Unknown field".to_string()
    } else if error_msg.contains("expected") && error_msg.contains("found") {
        "Invalid JSON format".to_string()
    } else {
        "Invalid JSON".to_string()
    }
}
