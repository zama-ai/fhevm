//! Process configuration: one YAML file, `APP_<SECTION>__<FIELD>` environment overrides.

use config::{Config, Environment, File};
use serde::Deserialize;

use crate::kms_aggregator::{ConfigError, KmsAggregatorConfig};

/// The whole process configuration.
///
/// No `deny_unknown_fields` at this level: every `APP_*` variable of the pod lands here as a key, and a typo in
/// `kms_aggregator` still fails as a missing field. The nested structs are strict.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub name: String,
    #[serde(default)]
    pub log: LogConfig,
    pub kms_aggregator: KmsAggregatorConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
    /// tracing filter directive, e.g. `info` or `info,relayer_http=debug`; `RUST_LOG` wins when set.
    #[serde(default = "default_level")]
    pub level: String,
    /// JSON lines instead of human-readable text.
    #[serde(default)]
    pub json: bool,
}

fn default_level() -> String {
    "info".to_owned()
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            json: false,
        }
    }
}

impl Settings {
    /// Reads `path`, applies `APP_*` environment overrides, validates. Durations need a unit (`5s`, `500ms`).
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        Self::load_with(path, Environment::with_prefix("APP"))
    }

    fn load_with(path: &str, env: Environment) -> Result<Self, ConfigError> {
        let settings: Settings = Config::builder()
            .add_source(File::with_name(path))
            .add_source(env.prefix_separator("_").separator("__"))
            .build()
            .and_then(Config::try_deserialize)
            .map_err(|e| ConfigError(format!("{path}: {e}")))?;
        settings.kms_aggregator.validate()?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;

    use super::*;

    const EXAMPLE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../config/config.yaml");

    fn env(vars: &[(&str, &str)]) -> Environment {
        let source: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        Environment::with_prefix("APP").source(Some(source))
    }

    #[test]
    fn example_config_loads_and_validates() {
        let settings = Settings::load(EXAMPLE).unwrap();
        assert_eq!(settings.name, "zama-relayer-http");
        assert_eq!(settings.log.level, "info");
        assert_eq!(settings.kms_aggregator.endpoints.len(), 13);
        assert_eq!(settings.kms_aggregator.user_decrypt.threshold, 9);
        assert_eq!(settings.kms_aggregator.public_decrypt.threshold, 5);
        assert_eq!(
            settings.kms_aggregator.call.timeout,
            Duration::from_millis(5000)
        );
    }

    #[test]
    fn environment_overrides_nested_fields() {
        let settings = Settings::load_with(
            EXAMPLE,
            env(&[
                ("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "7s"),
                ("APP_KMS_AGGREGATOR__MAX_CONCURRENT_CALLS", "128"),
                ("APP_LOG__JSON", "true"),
            ]),
        )
        .unwrap();
        assert_eq!(settings.kms_aggregator.call.timeout, Duration::from_secs(7));
        assert_eq!(settings.kms_aggregator.max_concurrent_calls, 128);
        assert!(settings.log.json);
    }

    #[test]
    fn unitless_duration_is_rejected() {
        let e = Settings::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "5000")]),
        )
        .err()
        .unwrap();
        assert!(e.0.contains("timeout"), "{e}");
    }

    #[test]
    fn unknown_nested_field_is_rejected() {
        let e = Settings::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUTS", "5s")]),
        )
        .err()
        .unwrap();
        assert!(e.0.contains("unknown field"), "{e}");
    }

    #[test]
    fn invalid_value_fails_validation() {
        let e = Settings::load_with(EXAMPLE, env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "0s")]))
            .err()
            .unwrap();
        assert!(e.0.contains("call.timeout"), "{e}");
    }

    #[test]
    fn missing_file_names_the_path() {
        let e = Settings::load("config/does-not-exist.yaml").err().unwrap();
        assert!(e.0.starts_with("config/does-not-exist.yaml"), "{e}");
    }

    #[test]
    fn log_config_default() {
        let log = LogConfig::default();
        assert_eq!(log.level, "info");
        assert!(!log.json);
    }
}
