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
use std::{borrow::Cow, collections::HashMap, sync::Mutex};
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
        return Err(ValidationError::new("must_start_with_0x")
            .with_message("Address should start with 0x".into()));
    }
    if address.len() != 42 {
        return Err(ValidationError::new("invalid_length")
            .with_message("Address should be 42 characters long".into()));
    }
    // The `hex` crate robustly checks if the string slice (after "0x") is valid hex.
    if hex::decode(&address[2..]).is_err() {
        return Err(ValidationError::new("invalid_hex_characters"));
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
        return Err(ValidationError::new("must_not_start_with_0x")
            .with_message("it should not start with 0x".into()));
    };

    if hex::decode(hex_str).is_err() {
        return Err(ValidationError::new("invalid_hex_characters")
            .with_message("invalid hex string".into()));
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
        return Err(ValidationError::new("invalid_value")
            .with_message("the only allow value is 0x00".into()));
    }
    Ok(())
}

pub fn validate_u32_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u32>().is_err() {
        return Err(ValidationError::new("invalid_u32_string")
            .with_message("the value is not a valid u32 string".into()));
    }
    Ok(())
}

pub fn validate_u64_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u64>().is_err() {
        return Err(ValidationError::new("invalid_u64_string")
            .with_message("the value is not a valid u64 string".into()));
    }
    Ok(())
}

pub fn validate_u256_string(value: &str) -> Result<(), ValidationError> {
    if U256::from_str(value).is_err() {
        return Err(ValidationError::new("invalid_u256_string")
            .with_message("the value is not a valid U256 string".into()));
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
    BadRequest(validator::ValidationErrors),
    Unprocessable(Cow<'static, str>),
    InternalServerError(Cow<'static, str>),
}

impl<V: serde::Serialize> AppResponse<V> {
    /// Creates a new success response with the given data.
    pub fn success(data: V) -> Self {
        AppResponse::Success(data)
    }

    /// Creates a new bad request response with the given validation errors.
    pub fn bad_request(errors: validator::ValidationErrors) -> Self {
        AppResponse::BadRequest(errors)
    }

    /// Creates a new unprocessable entity response with the given message.
    pub fn unprocessable<S: Into<Cow<'static, str>>>(message: S) -> Self {
        AppResponse::Unprocessable(message.into())
    }

    /// Creates a new internal server error response with the given message.
    pub fn internal_server_error<S: Into<Cow<'static, str>>>(message: S) -> Self {
        AppResponse::InternalServerError(message.into())
    }
}

#[derive(serde::Serialize)]
struct ErrorDetail {
    code: String,
    message: String,
}

// Implement `IntoResponse` so Axum can convert our enum into an HTTP response.
impl<V: serde::Serialize> IntoResponse for AppResponse<V> {
    fn into_response(self) -> Response {
        match self {
            AppResponse::Success(data) => (StatusCode::OK, Json(data)).into_response(),
            AppResponse::BadRequest(errors) => {
                let error_map = errors
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        (
                            to_camel_case(field),
                            errors
                                .iter()
                                .map(|e| ErrorDetail {
                                    message: e
                                        .message
                                        .as_ref()
                                        .map(|m| m.to_string())
                                        .unwrap_or_default(),
                                    code: e.code.to_string(),
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let body = Json(serde_json::json!({
                    "status": StatusCode::BAD_REQUEST.as_u16(),
                    "type": "validation",
                    "errors": error_map
                }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppResponse::Unprocessable(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "message": message })),
            )
                .into_response(),
            AppResponse::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "message": message })),
            )
                .into_response(),
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
