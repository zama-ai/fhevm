use serde::Deserialize;

use super::settings::AppConfigError;

/// Configuration for dynamic retry-after computation. All fields required.
#[derive(Debug, Deserialize, Clone)]
pub struct RetryAfterConfig {
    /// Minimum retry interval in seconds (floor).
    pub min_seconds: u32,

    /// Maximum retry interval in seconds (ceiling).
    pub max_seconds: u32,

    /// Safety margin applied as `ETA * (1 + safety_margin)`. Range: 0.0 to 1.0.
    pub safety_margin: f32,

    /// Nominal processing times for each stage.
    pub nominal_times: NominalProcessingTimes,

    /// Backoff intervals for ReceiptReceived state (Copro/KMS wait).
    pub copro_kms_backoff_intervals: Vec<BackoffInterval>,
}

/// Nominal processing times for ETA computation. All fields required.
#[derive(Debug, Deserialize, Clone)]
pub struct NominalProcessingTimes {
    /// Readiness check time (user/public decrypt), in seconds.
    pub readiness_check_seconds: u32,
    /// Input proof processing time, in seconds.
    pub input_proof_processing_seconds: u32,
    /// User decrypt processing time, in seconds.
    pub user_decrypt_processing_seconds: u32,
    /// Public decrypt processing time, in seconds.
    pub public_decrypt_processing_seconds: u32,
    /// TX confirmation time, in milliseconds.
    pub tx_confirmation_ms: u32,
}

/// Backoff interval for ReceiptReceived state (Copro/KMS wait).
#[derive(Debug, Deserialize, Clone)]
pub struct BackoffInterval {
    /// Elapsed time threshold in seconds.
    pub elapsed_threshold_secs: u32,
    /// Retry interval in seconds when threshold is reached.
    pub retry_interval_secs: u32,
}

impl RetryAfterConfig {
    /// Validates the retry-after configuration.
    ///
    /// Checks:
    /// - `min_seconds` < `max_seconds`
    /// - `safety_margin` is in range 0.0 to 1.0
    /// - `copro_kms_backoff_intervals` are sorted by `elapsed_threshold_secs` in ascending order
    pub fn validate(&self) -> Result<(), AppConfigError> {
        // Check min_seconds < max_seconds
        if self.min_seconds >= self.max_seconds {
            return Err(AppConfigError::Config(format!(
                "retry_after.min_seconds ({}) must be less than max_seconds ({})",
                self.min_seconds, self.max_seconds
            )));
        }

        // Check safety_margin is in valid range [0.0, 1.0]
        if !(0.0..=1.0).contains(&self.safety_margin) {
            return Err(AppConfigError::Config(format!(
                "retry_after.safety_margin ({}) must be between 0.0 and 1.0",
                self.safety_margin
            )));
        }

        // Check backoff intervals are sorted by threshold
        if !self.copro_kms_backoff_intervals.is_empty() {
            let mut prev_threshold = 0u32;
            for (i, interval) in self.copro_kms_backoff_intervals.iter().enumerate() {
                if i > 0 && interval.elapsed_threshold_secs <= prev_threshold {
                    return Err(AppConfigError::Config(format!(
                        "retry_after.copro_kms_backoff_intervals must be sorted by elapsed_threshold_secs in ascending order. \
                         Found threshold {} at index {} which is not greater than previous threshold {}",
                        interval.elapsed_threshold_secs, i, prev_threshold
                    )));
                }
                prev_threshold = interval.elapsed_threshold_secs;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RetryAfterConfig {
        RetryAfterConfig {
            min_seconds: 1,
            max_seconds: 300,
            safety_margin: 0.2,
            nominal_times: NominalProcessingTimes {
                readiness_check_seconds: 4,
                input_proof_processing_seconds: 2,
                user_decrypt_processing_seconds: 6,
                public_decrypt_processing_seconds: 6,
                tx_confirmation_ms: 250,
            },
            copro_kms_backoff_intervals: vec![
                BackoffInterval {
                    elapsed_threshold_secs: 0,
                    retry_interval_secs: 4,
                },
                BackoffInterval {
                    elapsed_threshold_secs: 60,
                    retry_interval_secs: 10,
                },
            ],
        }
    }

    #[test]
    fn test_validate_success() {
        let config = test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_min_greater_than_max() {
        let mut config = test_config();
        config.min_seconds = 100;
        config.max_seconds = 50;

        let result = config.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("min_seconds"));
        assert!(err.to_string().contains("max_seconds"));
    }

    #[test]
    fn test_validate_min_equals_max() {
        let mut config = test_config();
        config.min_seconds = 100;
        config.max_seconds = 100;

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_safety_margin_negative() {
        let mut config = test_config();
        config.safety_margin = -0.1;

        let result = config.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("safety_margin"));
    }

    #[test]
    fn test_validate_safety_margin_too_high() {
        let mut config = test_config();
        config.safety_margin = 1.5;

        let result = config.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("safety_margin"));
    }

    #[test]
    fn test_validate_safety_margin_boundary_values() {
        // 0.0 should be valid
        let mut config_zero = test_config();
        config_zero.safety_margin = 0.0;
        assert!(config_zero.validate().is_ok());

        // 1.0 should be valid
        let mut config_one = test_config();
        config_one.safety_margin = 1.0;
        assert!(config_one.validate().is_ok());
    }

    #[test]
    fn test_validate_unsorted_backoff_intervals() {
        let mut config = test_config();
        config.copro_kms_backoff_intervals = vec![
            BackoffInterval {
                elapsed_threshold_secs: 0,
                retry_interval_secs: 4,
            },
            BackoffInterval {
                elapsed_threshold_secs: 120,
                retry_interval_secs: 30,
            },
            BackoffInterval {
                elapsed_threshold_secs: 60, // Out of order
                retry_interval_secs: 10,
            },
        ];

        let result = config.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("sorted"));
    }

    #[test]
    fn test_validate_duplicate_thresholds() {
        let mut config = test_config();
        config.copro_kms_backoff_intervals = vec![
            BackoffInterval {
                elapsed_threshold_secs: 0,
                retry_interval_secs: 4,
            },
            BackoffInterval {
                elapsed_threshold_secs: 60,
                retry_interval_secs: 10,
            },
            BackoffInterval {
                elapsed_threshold_secs: 60, // Duplicate threshold
                retry_interval_secs: 20,
            },
        ];

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_backoff_intervals() {
        let mut config = test_config();
        config.copro_kms_backoff_intervals = vec![];

        // Empty intervals should be valid (though maybe not useful)
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_deserialize_config() {
        let yaml = r#"
min_seconds: 2
max_seconds: 600
safety_margin: 0.3
nominal_times:
  readiness_check_seconds: 10
  input_proof_processing_seconds: 3
  user_decrypt_processing_seconds: 8
  public_decrypt_processing_seconds: 8
  tx_confirmation_ms: 500
copro_kms_backoff_intervals:
  - elapsed_threshold_secs: 0
    retry_interval_secs: 2
  - elapsed_threshold_secs: 30
    retry_interval_secs: 5
"#;

        let config: RetryAfterConfig = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(config.min_seconds, 2);
        assert_eq!(config.max_seconds, 600);
        assert!((config.safety_margin - 0.3).abs() < f32::EPSILON);
        assert_eq!(config.nominal_times.readiness_check_seconds, 10);
        assert_eq!(config.nominal_times.input_proof_processing_seconds, 3);
        assert_eq!(config.nominal_times.user_decrypt_processing_seconds, 8);
        assert_eq!(config.nominal_times.public_decrypt_processing_seconds, 8);
        assert_eq!(config.nominal_times.tx_confirmation_ms, 500);
        assert_eq!(config.copro_kms_backoff_intervals.len(), 2);
    }

    #[test]
    fn test_all_fields_required() {
        // Missing min_seconds should fail
        let yaml = r#"
max_seconds: 300
safety_margin: 0.2
nominal_times:
  readiness_check_seconds: 4
  input_proof_processing_seconds: 2
  user_decrypt_processing_seconds: 6
  public_decrypt_processing_seconds: 6
  tx_confirmation_ms: 250
copro_kms_backoff_intervals: []
"#;
        assert!(serde_yaml::from_str::<RetryAfterConfig>(yaml).is_err());
    }
}
