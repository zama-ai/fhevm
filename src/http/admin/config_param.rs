//! Configuration parameter definitions and validation constraints.
//!
//! This module defines all admin-configurable parameters with their metadata,
//! validation constraints, and value types.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// All admin-configurable parameters with their metadata.
///
/// Uses snake_case serialization for API compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigParam {
    // TX Throttler TPS (existing)
    /// Input proof TX throttler rate (TPS)
    InputProofThrottlerTps,
    /// User decrypt TX throttler rate (TPS)
    UserDecryptThrottlerTps,
    /// Public decrypt TX throttler rate (TPS)
    PublicDecryptThrottlerTps,

    // Nominal processing times
    /// Expected readiness check time in seconds
    NominalReadinessCheckSeconds,
    /// Expected input proof processing time in seconds
    NominalInputProofProcessingSeconds,
    /// Expected user decrypt processing time in seconds
    NominalUserDecryptProcessingSeconds,
    /// Expected public decrypt processing time in seconds
    NominalPublicDecryptProcessingSeconds,
    /// Expected TX confirmation time in milliseconds
    NominalTxConfirmationMs,

    // Retry-after bounds (new)
    /// Minimum retry-after interval in seconds
    RetryAfterMinSeconds,
    /// Maximum retry-after interval in seconds
    RetryAfterMaxSeconds,

    // Safety margin for ETA (new)
    /// Safety margin multiplier for ETA (0.0-1.0)
    /// Applied as: ETA * (1 + safety_margin)
    RetryAfterSafetyMargin,
}

impl ConfigParam {
    /// Get validation constraints for this parameter.
    pub fn constraints(&self) -> ParamConstraints {
        use ConfigParam::*;
        match self {
            // TPS: 1-1000
            InputProofThrottlerTps | UserDecryptThrottlerTps | PublicDecryptThrottlerTps => {
                ParamConstraints::U32 { min: 1, max: 1000 }
            }

            // Nominal times in seconds: 1-3600 (1 hour max)
            NominalReadinessCheckSeconds
            | NominalInputProofProcessingSeconds
            | NominalUserDecryptProcessingSeconds
            | NominalPublicDecryptProcessingSeconds => ParamConstraints::U32 { min: 1, max: 3600 },

            // Nominal TX confirmation time in milliseconds: 1-60000 (60 seconds max)
            NominalTxConfirmationMs => ParamConstraints::U32 { min: 1, max: 60000 },

            // Retry bounds: 1-3600 seconds
            RetryAfterMinSeconds | RetryAfterMaxSeconds => {
                ParamConstraints::U32 { min: 1, max: 3600 }
            }

            // Safety margin: 0.0 to 1.0
            RetryAfterSafetyMargin => ParamConstraints::F32 { min: 0.0, max: 1.0 },
        }
    }

    /// Human-readable description for documentation/errors.
    pub fn description(&self) -> &'static str {
        use ConfigParam::*;
        match self {
            InputProofThrottlerTps => "Input proof TX throttler rate (TPS)",
            UserDecryptThrottlerTps => "User decrypt TX throttler rate (TPS)",
            PublicDecryptThrottlerTps => "Public decrypt TX throttler rate (TPS)",
            NominalReadinessCheckSeconds => "Expected readiness check time (seconds)",
            NominalInputProofProcessingSeconds => "Expected input proof processing time (seconds)",
            NominalUserDecryptProcessingSeconds => {
                "Expected user decrypt processing time (seconds)"
            }
            NominalPublicDecryptProcessingSeconds => {
                "Expected public decrypt processing time (seconds)"
            }
            NominalTxConfirmationMs => "Expected TX confirmation time (ms)",
            RetryAfterMinSeconds => "Minimum retry-after interval",
            RetryAfterMaxSeconds => "Maximum retry-after interval",
            RetryAfterSafetyMargin => "Safety margin multiplier for ETA (0.0-1.0)",
        }
    }

    /// Returns all available config parameters.
    pub fn all() -> &'static [ConfigParam] {
        &[
            ConfigParam::InputProofThrottlerTps,
            ConfigParam::UserDecryptThrottlerTps,
            ConfigParam::PublicDecryptThrottlerTps,
            ConfigParam::NominalReadinessCheckSeconds,
            ConfigParam::NominalInputProofProcessingSeconds,
            ConfigParam::NominalUserDecryptProcessingSeconds,
            ConfigParam::NominalPublicDecryptProcessingSeconds,
            ConfigParam::NominalTxConfirmationMs,
            ConfigParam::RetryAfterMinSeconds,
            ConfigParam::RetryAfterMaxSeconds,
            ConfigParam::RetryAfterSafetyMargin,
        ]
    }

    /// Check if this parameter is a TPS throttler parameter.
    pub fn is_tps_param(&self) -> bool {
        matches!(
            self,
            ConfigParam::InputProofThrottlerTps
                | ConfigParam::UserDecryptThrottlerTps
                | ConfigParam::PublicDecryptThrottlerTps
        )
    }

    /// Check if this parameter is a retry-after parameter.
    pub fn is_retry_after_param(&self) -> bool {
        matches!(
            self,
            ConfigParam::NominalReadinessCheckSeconds
                | ConfigParam::NominalInputProofProcessingSeconds
                | ConfigParam::NominalUserDecryptProcessingSeconds
                | ConfigParam::NominalPublicDecryptProcessingSeconds
                | ConfigParam::NominalTxConfirmationMs
                | ConfigParam::RetryAfterMinSeconds
                | ConfigParam::RetryAfterMaxSeconds
                | ConfigParam::RetryAfterSafetyMargin
        )
    }
}

impl fmt::Display for ConfigParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Validation constraints for configuration parameters.
#[derive(Debug, Clone)]
pub enum ParamConstraints {
    /// Unsigned 32-bit integer constraints
    U32 { min: u32, max: u32 },
    /// 32-bit floating point constraints
    F32 { min: f32, max: f32 },
}

impl ParamConstraints {
    /// Validate a configuration value against these constraints.
    pub fn validate(&self, value: &ConfigValue) -> Result<(), ConfigError> {
        match (self, value) {
            (ParamConstraints::U32 { min, max }, ConfigValue::U32(v)) => {
                if v < min || v > max {
                    return Err(ConfigError::ValidationError(format!(
                        "Value {} is out of range [{}, {}]",
                        v, min, max
                    )));
                }
                Ok(())
            }
            (ParamConstraints::F32 { min, max }, ConfigValue::F32(v)) => {
                if v < min || v > max {
                    return Err(ConfigError::ValidationError(format!(
                        "Value {} is out of range [{}, {}]",
                        v, min, max
                    )));
                }
                Ok(())
            }
            (ParamConstraints::U32 { .. }, ConfigValue::F32(_)) => Err(ConfigError::TypeMismatch {
                expected: "u32".to_string(),
                got: "f32".to_string(),
            }),
            (ParamConstraints::F32 { .. }, ConfigValue::U32(_)) => Err(ConfigError::TypeMismatch {
                expected: "f32".to_string(),
                got: "u32".to_string(),
            }),
        }
    }
}

/// Configuration value types.
///
/// Uses untagged serialization so values appear as raw numbers in JSON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConfigValue {
    /// Unsigned 32-bit integer value
    U32(u32),
    /// 32-bit floating point value
    F32(f32),
}

impl ConfigValue {
    /// Try to get the value as a u32.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ConfigValue::U32(v) => Some(*v),
            _ => None,
        }
    }

    /// Try to get the value as an f32.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            ConfigValue::F32(v) => Some(*v),
            _ => None,
        }
    }
}

impl fmt::Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigValue::U32(v) => write!(f, "{}", v),
            ConfigValue::F32(v) => write!(f, "{}", v),
        }
    }
}

/// Errors that can occur during configuration operations.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Validation error when value is out of range or invalid
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Type mismatch between expected and provided value types
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    /// Unknown parameter name
    #[error("Unknown parameter: {0}")]
    UnknownParam(String),

    /// Channel communication error (e.g., when notifying throttler workers)
    #[error("Channel error: {0}")]
    ChannelError(String),

    /// Admin endpoints are disabled
    #[error("Admin endpoints are not enabled")]
    AdminDisabled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_param_serialization() {
        let param = ConfigParam::InputProofThrottlerTps;
        let json = serde_json::to_string(&param).unwrap();
        assert_eq!(json, r#""input_proof_throttler_tps""#);

        let deserialized: ConfigParam = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, param);
    }

    #[test]
    fn test_config_value_u32_serialization() {
        let value = ConfigValue::U32(100);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "100");

        let deserialized: ConfigValue = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_config_value_f32_serialization() {
        let value = ConfigValue::F32(0.5);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "0.5");

        let deserialized: ConfigValue = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn test_u32_constraints_validation() {
        let constraints = ParamConstraints::U32 { min: 1, max: 1000 };

        // Valid value
        assert!(constraints.validate(&ConfigValue::U32(100)).is_ok());

        // Out of range (too low)
        assert!(constraints.validate(&ConfigValue::U32(0)).is_err());

        // Out of range (too high)
        assert!(constraints.validate(&ConfigValue::U32(1001)).is_err());

        // Type mismatch
        let result = constraints.validate(&ConfigValue::F32(0.5));
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }

    #[test]
    fn test_f32_constraints_validation() {
        let constraints = ParamConstraints::F32 { min: 0.0, max: 1.0 };

        // Valid value
        assert!(constraints.validate(&ConfigValue::F32(0.5)).is_ok());

        // Out of range (too low)
        assert!(constraints.validate(&ConfigValue::F32(-0.1)).is_err());

        // Out of range (too high)
        assert!(constraints.validate(&ConfigValue::F32(1.1)).is_err());

        // Type mismatch
        let result = constraints.validate(&ConfigValue::U32(1));
        assert!(matches!(result, Err(ConfigError::TypeMismatch { .. })));
    }

    #[test]
    fn test_config_param_all() {
        let all = ConfigParam::all();
        assert_eq!(all.len(), 11);
        assert!(all.contains(&ConfigParam::InputProofThrottlerTps));
        assert!(all.contains(&ConfigParam::RetryAfterSafetyMargin));
    }

    #[test]
    fn test_config_param_is_tps_param() {
        assert!(ConfigParam::InputProofThrottlerTps.is_tps_param());
        assert!(ConfigParam::UserDecryptThrottlerTps.is_tps_param());
        assert!(ConfigParam::PublicDecryptThrottlerTps.is_tps_param());
        assert!(!ConfigParam::RetryAfterMinSeconds.is_tps_param());
        assert!(!ConfigParam::RetryAfterSafetyMargin.is_tps_param());
    }

    #[test]
    fn test_config_param_is_retry_after_param() {
        // Retry-after params
        assert!(ConfigParam::RetryAfterMinSeconds.is_retry_after_param());
        assert!(ConfigParam::RetryAfterMaxSeconds.is_retry_after_param());
        assert!(ConfigParam::RetryAfterSafetyMargin.is_retry_after_param());
        assert!(ConfigParam::NominalReadinessCheckSeconds.is_retry_after_param());
        assert!(ConfigParam::NominalInputProofProcessingSeconds.is_retry_after_param());
        assert!(ConfigParam::NominalUserDecryptProcessingSeconds.is_retry_after_param());
        assert!(ConfigParam::NominalPublicDecryptProcessingSeconds.is_retry_after_param());
        assert!(ConfigParam::NominalTxConfirmationMs.is_retry_after_param());

        // TPS params are not retry-after params
        assert!(!ConfigParam::InputProofThrottlerTps.is_retry_after_param());
        assert!(!ConfigParam::UserDecryptThrottlerTps.is_retry_after_param());
        assert!(!ConfigParam::PublicDecryptThrottlerTps.is_retry_after_param());
    }

    #[test]
    fn test_config_param_display() {
        assert_eq!(
            ConfigParam::InputProofThrottlerTps.to_string(),
            "InputProofThrottlerTps"
        );
        assert_eq!(
            ConfigParam::RetryAfterSafetyMargin.to_string(),
            "RetryAfterSafetyMargin"
        );
    }
}
