use crate::orchestrator::traits::{Event, EventHandler};
use alloy::primitives::U256;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;
use std::{borrow::Cow, sync::Mutex};
use tokio::sync::oneshot;
use validator::ValidationError;

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
            .with_message("Address must start with 0x".into()));
    }
    if address.len() != 42 {
        return Err(ValidationError::new("validation_error")
            .with_message("Address must be 42 characters long".into()));
    }
    // The `hex` crate robustly checks if the string slice (after "0x") is valid hex.
    if hex::decode(&address[2..]).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Address contains invalid hex characters".into()));
    }
    Ok(())
}

pub fn validate_blockchain_addresses(addresses: &Vec<String>) -> Result<(), ValidationError> {
    for address in addresses {
        validate_blockchain_address(address)?;
    }
    Ok(())
}

// Custom validation function for a hex string that must NOT have a "0x" prefix.
pub fn validate_hex_string(hex_str: &str) -> Result<(), ValidationError> {
    // Allow both with and without "0x" prefix
    if hex_str.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message("Hex string must not start with 0x".into()));
    };

    if hex::decode(hex_str).is_err() {
        return Err(
            ValidationError::new("validation_error").with_message("Invalid hex string".into())
        );
    }
    Ok(())
}

pub fn validate_hex_strings(hex_strs: &Vec<String>) -> Result<(), ValidationError> {
    for hex_str in hex_strs {
        validate_hex_string(hex_str)?;
    }
    Ok(())
}

pub fn validate_extra_data_field(extra_data: &str) -> Result<(), ValidationError> {
    if extra_data != "0x00" {
        return Err(
            ValidationError::new("validation_error").with_message("Extra data must be 0x00".into())
        );
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

pub fn validate_u256_string(value: &str) -> Result<(), ValidationError> {
    if U256::from_str(value).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Value must be a valid U256 number".into()));
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
            .with_message("Chain ID must be a valid decimal number (e.g., '123456') or hex string with 0x prefix (e.g., '0x1e240')".into()));
    }
    Ok(())
}

pub fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}

/// A generic enum to handle various API response types.
/// The generic parameter `V` allows the success variant to hold any serializable data.
#[derive(Debug)]
pub enum AppResponse<V: serde::Serialize> {
    Success(V),
    BadRequest(Cow<'static, str>),
    ValidationError(validator::ValidationErrors),
    InternalServerError(Cow<'static, str>),
}

impl<V: serde::Serialize> AppResponse<V> {
    /// Creates a new success response with the given data.
    pub fn success(data: V) -> Self {
        AppResponse::Success(data)
    }

    /// Creates a new unprocessable entity response with the given message.
    pub fn bad_request<S: Into<Cow<'static, str>>>(message: S) -> Self {
        AppResponse::BadRequest(message.into())
    }

    /// Creates a new bad request response with the given validation errors.
    pub fn invalid_request(errors: validator::ValidationErrors) -> Self {
        AppResponse::ValidationError(errors)
    }

    /// Creates a new internal server error response with the given message.
    pub fn internal_server_error<S: Into<Cow<'static, str>>>(message: S) -> Self {
        AppResponse::InternalServerError(message.into())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    BadRequest,
    InvalidRequest,
    RateLimited,
    InternalServerError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::BadRequest => "bad_request",
            ErrorCode::InvalidRequest => "invalid_request",
            ErrorCode::RateLimited => "rate_limited",
            ErrorCode::InternalServerError => "internal_server_error",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorDetail {
    pub field: String,
    pub issue: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub request_id: Option<String>,
    pub retry_after_seconds: Option<u64>,
    pub reason: Option<String>,
    pub details: Option<Vec<ErrorDetail>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ApiError,
}

// Implement `IntoResponse` so Axum can convert our enum into an HTTP response.
impl<V: serde::Serialize> IntoResponse for AppResponse<V> {
    fn into_response(self) -> Response {
        match self {
            AppResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            AppResponse::BadRequest(message) => {
                let api_error = ApiError {
                    code: ErrorCode::BadRequest,
                    message: message.to_string(),
                    request_id: None,
                    retry_after_seconds: None,
                    reason: None,
                    details: None,
                };

                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::ValidationError(errors) => {
                let details = errors
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
                    .collect::<Vec<_>>();

                let api_error = ApiError {
                    code: ErrorCode::InvalidRequest,
                    message: "One or more fields are invalid".to_string(),
                    request_id: None,
                    retry_after_seconds: None,
                    reason: None,
                    details: if details.is_empty() {
                        None
                    } else {
                        Some(details)
                    },
                };

                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": api_error })),
                )
                    .into_response()
            }
            AppResponse::InternalServerError(message) => {
                let api_error = ApiError {
                    code: ErrorCode::InternalServerError,
                    message: message.to_string(),
                    request_id: None,
                    retry_after_seconds: None,
                    reason: None,
                    details: None,
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
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
