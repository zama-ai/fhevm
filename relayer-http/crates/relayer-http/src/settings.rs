//! Process configuration: one YAML file, `APP_<SECTION>__<FIELD>` environment overrides.

use std::net::SocketAddr;

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
    pub http: HttpConfig,
    pub kms_aggregator: KmsAggregatorConfig,
}

/// The HTTP server: the API and the Kubernetes probes share one port.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpConfig {
    /// Bind address, e.g. `0.0.0.0:8080`.
    pub endpoint: SocketAddr,
    /// Largest accepted request body; bigger ones are answered `400 malformed`.
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,
    /// Host chain ids a ciphertext handle may carry (bytes 22..30 of the handle).
    pub supported_chain_ids: Vec<u64>,
}

fn default_max_body_bytes() -> usize {
    2 << 20
}

impl HttpConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_body_bytes < 1024 {
            return Err(ConfigError(
                "http.max_body_bytes must be >= 1024".to_owned(),
            ));
        }
        if self.supported_chain_ids.is_empty() {
            return Err(ConfigError(
                "http.supported_chain_ids must not be empty".to_owned(),
            ));
        }
        let mut seen = std::collections::HashSet::new();
        if let Some(dup) = self.supported_chain_ids.iter().find(|id| !seen.insert(*id)) {
            return Err(ConfigError(format!(
                "http.supported_chain_ids contains {dup} twice"
            )));
        }
        Ok(())
    }
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
        settings.http.validate()?;
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
    fn http_config_is_loaded_and_validated() {
        let settings = Settings::load(EXAMPLE).unwrap();
        assert_eq!(settings.http.endpoint.port(), 8080);
        assert_eq!(settings.http.max_body_bytes, 2 << 20);
        assert_eq!(settings.http.supported_chain_ids, vec![1, 137]);

        let e = Settings::load_with(EXAMPLE, env(&[("APP_HTTP__MAX_BODY_BYTES", "10")]))
            .err()
            .unwrap();
        assert!(e.0.contains("http.max_body_bytes"), "{e}");
        let e = Settings::load_with(EXAMPLE, env(&[("APP_HTTP__ENDPOINT", "not-an-address")]))
            .err()
            .unwrap();
        assert!(e.0.contains("endpoint"), "{e}");
    }

    #[test]
    fn http_config_rules() {
        let mut http = HttpConfig {
            endpoint: "127.0.0.1:8080".parse().unwrap(),
            max_body_bytes: 4096,
            supported_chain_ids: vec![1, 137],
        };
        http.validate().unwrap();
        http.supported_chain_ids = vec![1, 1];
        assert!(http.validate().unwrap_err().0.contains("twice"));
        http.supported_chain_ids = vec![];
        assert!(http.validate().unwrap_err().0.contains("must not be empty"));
    }

    #[test]
    fn log_config_default() {
        let log = LogConfig::default();
        assert_eq!(log.level, "info");
        assert!(!log.json);
    }
}
