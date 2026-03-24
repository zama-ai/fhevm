//! HTTP handlers for admin configuration endpoints.
//!
//! This module provides the HTTP handlers for updating and reading
//! admin-configurable parameters via the REST API.

use super::config_param::{ConfigError, ConfigParam, ConfigValue};
use super::registry::AdminConfigRegistry;
use crate::http::retry_after::RetryAfterState;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Request body for updating a configuration parameter.
#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    /// Parameter name in snake_case (e.g., "input_proof_throttler_tps")
    #[serde(alias = "name")]
    pub param: String,
    /// New value for the parameter
    pub value: ConfigValue,
}

/// Successful response for config update.
/// Note: Uses "name" field for backward compatibility with existing clients.
#[derive(Debug, Serialize)]
pub struct UpdateConfigResponse {
    /// Parameter name that was updated (named "name" for backward compatibility)
    pub name: String,
    /// New value
    pub value: ConfigValue,
    /// Human-readable message
    pub message: String,
}

/// Error response for admin endpoints.
#[derive(Debug, Serialize)]
pub struct AdminErrorResponse {
    /// Error message
    pub error: String,
}

/// Response for GET /admin/config endpoint.
#[derive(Debug, Serialize)]
pub struct GetConfigResponse {
    /// All current configuration values
    pub values: HashMap<String, ConfigValue>,
    /// Available parameters with their descriptions and constraints
    pub available_params: Vec<ParamInfo>,
}

/// Information about a configurable parameter.
#[derive(Debug, Serialize)]
pub struct ParamInfo {
    /// Parameter name in snake_case
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Constraint information
    pub constraints: ConstraintInfo,
}

/// Constraint information for a parameter.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ConstraintInfo {
    /// Integer constraints
    #[serde(rename = "u32")]
    U32 { min: u32, max: u32 },
    /// Float constraints
    #[serde(rename = "f32")]
    F32 { min: f32, max: f32 },
}

/// Update a configuration parameter.
///
/// # Request
///
/// ```json
/// POST /admin/config
/// {
///     "param": "input_proof_throttler_tps",
///     "value": 50
/// }
/// ```
///
/// Note: "name" is also accepted as an alias for "param" for backward compatibility.
///
/// # Response
///
/// On success (200):
/// ```json
/// {
///     "param": "input_proof_throttler_tps",
///     "value": 50,
///     "message": "Configuration updated successfully"
/// }
/// ```
///
/// On error (400/403/500):
/// ```json
/// {
///     "error": "Error description"
/// }
/// ```
pub async fn update_config(
    Extension(registry): Extension<Option<Arc<AdminConfigRegistry>>>,
    Extension(retry_after_state): Extension<Option<Arc<RetryAfterState>>>,
    Json(payload): Json<UpdateConfigRequest>,
) -> Response {
    // Check if admin endpoints are enabled (need either registry or retry-after state)
    let registry = registry.as_ref();
    let retry_state = retry_after_state.as_ref();

    if registry.is_none() && retry_state.is_none() {
        warn!("Admin endpoint called but admin endpoints are disabled");
        return (
            StatusCode::FORBIDDEN,
            Json(AdminErrorResponse {
                error: "Admin endpoints are not enabled".to_string(),
            }),
        )
            .into_response();
    }

    // Parse param name to enum (serde handles snake_case conversion)
    let param: ConfigParam =
        match serde_json::from_value(serde_json::Value::String(payload.param.clone())) {
            Ok(p) => p,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(AdminErrorResponse {
                        error: format!("Unknown configuration parameter: {}", payload.param),
                    }),
                )
                    .into_response();
            }
        };

    // Validate against constraints
    if let Err(e) = param.constraints().validate(&payload.value) {
        let error_msg = match &e {
            ConfigError::ValidationError(msg) => format_validation_error(&payload.param, msg),
            _ => e.to_string(),
        };
        return (
            StatusCode::BAD_REQUEST,
            Json(AdminErrorResponse { error: error_msg }),
        )
            .into_response();
    }

    // Route to appropriate handler based on param type
    if param.is_retry_after_param() {
        // Route to RetryAfterState
        let Some(state) = retry_state else {
            return (
                StatusCode::BAD_REQUEST,
                Json(AdminErrorResponse {
                    error: "Retry-after state not configured".to_string(),
                }),
            )
                .into_response();
        };

        match apply_retry_after_update(state, param, &payload.value).await {
            Ok(_) => {
                info!(
                    param = %param,
                    value = %payload.value,
                    "ADMIN_CONFIG_UPDATE: Retry-after param updated"
                );
                (
                    StatusCode::OK,
                    Json(UpdateConfigResponse {
                        name: payload.param,
                        value: payload.value,
                        message: "Configuration updated successfully".to_string(),
                    }),
                )
                    .into_response()
            }
            Err(msg) => (
                StatusCode::BAD_REQUEST,
                Json(AdminErrorResponse { error: msg }),
            )
                .into_response(),
        }
    } else if param.is_tps_param() {
        // Route to AdminConfigRegistry for TPS params
        let Some(reg) = registry else {
            return (
                StatusCode::BAD_REQUEST,
                Json(AdminErrorResponse {
                    error: "TPS throttler not configured".to_string(),
                }),
            )
                .into_response();
        };

        if !reg.is_enabled() {
            return (
                StatusCode::FORBIDDEN,
                Json(AdminErrorResponse {
                    error: "TPS throttlers are not enabled".to_string(),
                }),
            )
                .into_response();
        }

        match reg.update(param, payload.value.clone()).await {
            Ok(_) => (
                StatusCode::OK,
                Json(UpdateConfigResponse {
                    name: payload.param,
                    value: payload.value,
                    message: "Configuration updated successfully".to_string(),
                }),
            )
                .into_response(),
            Err(e) => {
                let (status, error_msg) = match &e {
                    ConfigError::ChannelError(msg) if msg.contains("busy") => {
                        (StatusCode::SERVICE_UNAVAILABLE, e.to_string())
                    }
                    ConfigError::ChannelError(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                    }
                    _ => (StatusCode::BAD_REQUEST, e.to_string()),
                };
                (status, Json(AdminErrorResponse { error: error_msg })).into_response()
            }
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(AdminErrorResponse {
                error: format!("Parameter {} is not admin-configurable", payload.param),
            }),
        )
            .into_response()
    }
}

/// Apply a retry-after config update to RetryAfterState.
async fn apply_retry_after_update(
    state: &RetryAfterState,
    param: ConfigParam,
    value: &ConfigValue,
) -> Result<(), String> {
    match param {
        ConfigParam::RetryAfterMinSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_min_seconds(v).await;
            Ok(())
        }
        ConfigParam::RetryAfterMaxSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_max_seconds(v).await;
            Ok(())
        }
        ConfigParam::RetryAfterSafetyMargin => {
            let v = value.as_f32().ok_or("Expected f32 value")?;
            state.set_safety_margin(v).await;
            Ok(())
        }
        ConfigParam::NominalReadinessCheckSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_nominal_readiness_seconds(v).await;
            Ok(())
        }
        ConfigParam::NominalInputProofProcessingSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_nominal_input_proof_seconds(v).await;
            Ok(())
        }
        ConfigParam::NominalUserDecryptProcessingSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_nominal_user_decrypt_seconds(v).await;
            Ok(())
        }
        ConfigParam::NominalPublicDecryptProcessingSeconds => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_nominal_public_decrypt_seconds(v).await;
            Ok(())
        }
        ConfigParam::NominalTxConfirmationMs => {
            let v = value.as_u32().ok_or("Expected u32 value")?;
            state.set_nominal_tx_ms(v).await;
            Ok(())
        }
        _ => Err(format!("Param {} is not a retry-after param", param)),
    }
}

/// Format validation error message for backward compatibility.
/// The existing tests expect specific error message formats.
fn format_validation_error(param_name: &str, validation_msg: &str) -> String {
    // Try to parse the validation message to extract min/max values
    // Original format: "Value X is out of range [min, max]"
    if validation_msg.contains("out of range") {
        // Extract the value and range from the message
        if let Some(start) = validation_msg.find('[') {
            if let Some(end) = validation_msg.find(']') {
                let range = &validation_msg[start + 1..end];
                let parts: Vec<&str> = range.split(", ").collect();
                if parts.len() == 2 {
                    let _min = parts[0];
                    let max = parts[1];

                    // Try to extract the value
                    if let Some(val_start) = validation_msg.find("Value ") {
                        let after_value = &validation_msg[val_start + 6..];
                        if let Some(val_end) = after_value.find(' ') {
                            let value_str = &after_value[..val_end];
                            if let Ok(value) = value_str.parse::<u32>() {
                                if value == 0 {
                                    return format!("{} must be greater than 0", param_name);
                                } else if let Ok(max_val) = max.parse::<u32>() {
                                    if value > max_val {
                                        return format!(
                                            "{} must be less than or equal to {}",
                                            param_name, max
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Fallback to original message if parsing fails
    validation_msg.to_string()
}

/// Get all current configuration values.
///
/// # Request
///
/// ```
/// GET /admin/config
/// ```
///
/// # Response
///
/// ```json
/// {
///     "values": {
///         "retry_after_min_seconds": 1,
///         "retry_after_max_seconds": 300,
///         "retry_after_safety_margin": 0.2
///     },
///     "available_params": [
///         {
///             "name": "input_proof_throttler_tps",
///             "description": "Input proof TX throttler rate (TPS)",
///             "constraints": { "type": "u32", "min": 1, "max": 1000 }
///         },
///         ...
///     ]
/// }
/// ```
pub async fn get_config(
    Extension(registry): Extension<Option<Arc<AdminConfigRegistry>>>,
    Extension(retry_after_state): Extension<Option<Arc<RetryAfterState>>>,
) -> Response {
    // Check if admin endpoints are enabled
    let registry = registry.as_ref();
    let retry_state = retry_after_state.as_ref();

    if registry.is_none() && retry_state.is_none() {
        warn!("Admin endpoint called but admin endpoints are disabled");
        return (
            StatusCode::FORBIDDEN,
            Json(AdminErrorResponse {
                error: "Admin endpoints are not enabled".to_string(),
            }),
        )
            .into_response();
    }

    // Build values map from both sources
    let mut values: HashMap<String, ConfigValue> = HashMap::new();

    // Add TPS values from registry if available
    if let Some(reg) = registry {
        for (k, v) in reg.get_all().await {
            values.insert(k.to_string(), v);
        }
    }

    // Add retry-after values from state if available
    if let Some(state) = retry_state {
        values.insert(
            ConfigParam::RetryAfterMinSeconds.to_string(),
            ConfigValue::U32(state.min_seconds().await),
        );
        values.insert(
            ConfigParam::RetryAfterMaxSeconds.to_string(),
            ConfigValue::U32(state.max_seconds().await),
        );
        values.insert(
            ConfigParam::RetryAfterSafetyMargin.to_string(),
            ConfigValue::F32(state.safety_margin().await),
        );
        values.insert(
            ConfigParam::NominalReadinessCheckSeconds.to_string(),
            ConfigValue::U32(state.nominal_readiness_ms().await / 1000),
        );
        values.insert(
            ConfigParam::NominalInputProofProcessingSeconds.to_string(),
            ConfigValue::U32(state.nominal_input_proof_ms().await / 1000),
        );
        values.insert(
            ConfigParam::NominalUserDecryptProcessingSeconds.to_string(),
            ConfigValue::U32(state.nominal_user_decrypt_ms().await / 1000),
        );
        values.insert(
            ConfigParam::NominalPublicDecryptProcessingSeconds.to_string(),
            ConfigValue::U32(state.nominal_public_decrypt_ms().await / 1000),
        );
        values.insert(
            ConfigParam::NominalTxConfirmationMs.to_string(),
            ConfigValue::U32(state.nominal_tx_ms().await),
        );
    }

    // Build available params info
    let available_params: Vec<ParamInfo> = ConfigParam::all()
        .iter()
        .map(|p| {
            let constraints = match p.constraints() {
                super::config_param::ParamConstraints::U32 { min, max } => {
                    ConstraintInfo::U32 { min, max }
                }
                super::config_param::ParamConstraints::F32 { min, max } => {
                    ConstraintInfo::F32 { min, max }
                }
            };
            ParamInfo {
                name: p.to_string(),
                description: p.description().to_string(),
                constraints,
            }
        })
        .collect();

    (
        StatusCode::OK,
        Json(GetConfigResponse {
            values,
            available_params,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_config_request_deserialize() {
        // Test with "param" field
        let json = r#"{"param": "input_proof_throttler_tps", "value": 50}"#;
        let req: UpdateConfigRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.param, "input_proof_throttler_tps");
        assert_eq!(req.value, ConfigValue::U32(50));

        // Test with "name" alias (backward compatibility)
        let json = r#"{"name": "input_proof_throttler_tps", "value": 50}"#;
        let req: UpdateConfigRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.param, "input_proof_throttler_tps");
    }

    #[test]
    fn test_update_config_request_f32_value() {
        let json = r#"{"param": "retry_after_safety_margin", "value": 0.25}"#;
        let req: UpdateConfigRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.param, "retry_after_safety_margin");
        assert_eq!(req.value, ConfigValue::F32(0.25));
    }

    #[test]
    fn test_constraint_info_serialization() {
        let u32_constraint = ConstraintInfo::U32 { min: 1, max: 1000 };
        let json = serde_json::to_string(&u32_constraint).unwrap();
        assert!(json.contains(r#""type":"u32""#));
        assert!(json.contains(r#""min":1"#));
        assert!(json.contains(r#""max":1000"#));

        let f32_constraint = ConstraintInfo::F32 { min: 0.0, max: 1.0 };
        let json = serde_json::to_string(&f32_constraint).unwrap();
        assert!(json.contains(r#""type":"f32""#));
    }

    #[test]
    fn test_format_validation_error_zero_value() {
        let result = format_validation_error(
            "input_proof_throttler_tps",
            "Value 0 is out of range [1, 1000]",
        );
        assert_eq!(result, "input_proof_throttler_tps must be greater than 0");
    }

    #[test]
    fn test_format_validation_error_over_max() {
        let result = format_validation_error(
            "input_proof_throttler_tps",
            "Value 1001 is out of range [1, 1000]",
        );
        assert_eq!(
            result,
            "input_proof_throttler_tps must be less than or equal to 1000"
        );
    }

    #[test]
    fn test_format_validation_error_fallback() {
        // Unknown error format should pass through
        let result = format_validation_error("some_param", "Some other error message");
        assert_eq!(result, "Some other error message");
    }
}
