use crate::http::utils::responses::{AppResponse, FieldJsonErrorType, ParseError};
use axum::{
    body::Bytes,
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

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
                        "handleContractPairs" => "handle_contract_pairs",
                        "contractChainId" => "contract_chain_id",
                        "contractAddress" => "contract_address",
                        "userAddress" => "user_address",
                        "delegatorAddress" => "delegator_address",
                        "delegateAddress" => "delegate_address",
                        "startTimestamp" => "start_timestamp",
                        "durationDays" => "duration_days",
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

/// Extract field name from serde JSON error message
pub fn extract_field_from_serde_error(error: &serde_json::Error) -> String {
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
pub fn format_serde_error_message(error: &serde_json::Error) -> String {
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
