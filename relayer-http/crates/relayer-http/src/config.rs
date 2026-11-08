//! Process configuration: one YAML file, `APP_<SECTION>__<FIELD>` environment overrides.

use std::net::SocketAddr;
use std::time::Duration;

use config::{Config, Environment, File};
use serde::Deserialize;

use crate::kms_aggregator::config::MAX_TIMEOUT;
use crate::kms_aggregator::{ConfigError, KmsAggregatorConfig};

/// The whole process configuration.
///
/// No `deny_unknown_fields` at this level: every `APP_*` variable of the pod lands here as a key, and a typo in
/// `kms_aggregator` still fails as a missing field. The nested structs are strict.
#[derive(Debug, Clone, Deserialize)]
pub struct RelayerConfig {
    pub name: String,
    #[serde(default)]
    pub log: LogConfig,
    pub http: HttpConfig,
    #[serde(default)]
    pub metrics: MetricsConfig,
    pub kms_aggregator: KmsAggregatorConfig,
}

/// The Prometheus listener, on its own port as in the relayer. Optional: defaults to `0.0.0.0:9898`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsConfig {
    /// Bind address of `GET /metrics`.
    pub endpoint: SocketAddr,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            endpoint: SocketAddr::from(([0, 0, 0, 0], 9898)),
        }
    }
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
    /// Total time to receive and parse a request body, from the handler's start (headers already read). Bounds a
    /// client trickling its body: after headers, a request lasts at most `body_read_timeout + call.timeout`
    /// (10 s + 5 s by default). Unit required (`10s`, `500ms`); within `1ms..=60s`.
    #[serde(with = "humantime_serde", default = "default_body_read_timeout")]
    pub body_read_timeout: Duration,
    /// Host chain ids a ciphertext handle may carry (bytes 22..30 of the handle).
    pub supported_chain_ids: Vec<u64>,
    /// Most handles in one decryption request; the connectors' `max_handles_per_decryption`, so set it to theirs
    /// when they change it. At least 1.
    #[serde(default = "default_max_handles_per_decryption")]
    pub max_handles_per_decryption: usize,
    /// Most `allowedContracts` in one user decryption request; the connectors' `max_allowed_contracts`, so set it to
    /// theirs when they change it.
    #[serde(default = "default_max_allowed_contracts")]
    pub max_allowed_contracts: usize,
}

/// 50, the connector's default `max_handles_per_decryption`.
fn default_max_handles_per_decryption() -> usize {
    50
}

/// 10, the connector's default `max_allowed_contracts`.
fn default_max_allowed_contracts() -> usize {
    10
}

/// 1 MiB, the connector endpoint's and proxy's limit: a bigger body could never be forwarded.
fn default_max_body_bytes() -> usize {
    1024 * 1024
}

/// 10 s: a 1 MiB body still fits at ~100 KiB/s.
fn default_body_read_timeout() -> Duration {
    Duration::from_secs(10)
}

impl HttpConfig {
    /// Test only: `chain_ids`, a 4 KiB body limit, a 2 s body read timeout, the default request limits.
    #[cfg(test)]
    pub fn for_tests(chain_ids: &[u64]) -> Self {
        Self {
            endpoint: "127.0.0.1:0".parse().unwrap(),
            max_body_bytes: 4096,
            body_read_timeout: Duration::from_secs(2),
            supported_chain_ids: chain_ids.to_vec(),
            max_handles_per_decryption: default_max_handles_per_decryption(),
            max_allowed_contracts: default_max_allowed_contracts(),
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_body_bytes < 1024 {
            return Err(ConfigError(
                "http.max_body_bytes must be >= 1024".to_owned(),
            ));
        }
        if self.body_read_timeout.is_zero() || self.body_read_timeout > MAX_TIMEOUT {
            return Err(ConfigError(format!(
                "http.body_read_timeout must be within 1ms..={MAX_TIMEOUT:?}"
            )));
        }
        if self.supported_chain_ids.is_empty() {
            return Err(ConfigError(
                "http.supported_chain_ids must not be empty".to_owned(),
            ));
        }
        if self.max_handles_per_decryption == 0 {
            return Err(ConfigError(
                "http.max_handles_per_decryption must be >= 1".to_owned(),
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

/// The current relayer's log configuration, same keys and meaning. Optional: without a `log:` block the process
/// logs JSON lines (production). Overrides: `APP_LOG__FORMAT`, `APP_LOG__SHOW_FILE_LINE`, ...; the level filter is
/// `RUST_LOG` (default `warn,relayer_http=debug`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LogConfig {
    /// `json` (default), `pretty` or `compact`.
    pub format: String,
    /// File and line of the log call.
    pub show_file_line: bool,
    pub show_thread_ids: bool,
    pub show_timestamp: bool,
    /// The module path of the log call.
    pub show_target: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            format: "json".to_owned(),
            show_file_line: false,
            show_thread_ids: false,
            show_timestamp: true,
            show_target: true,
        }
    }
}

impl RelayerConfig {
    /// Reads `path`, applies `APP_*` environment overrides, validates. Durations need a unit (`5s`, `500ms`); the
    /// retry waits are one comma-separated value (`APP_KMS_AGGREGATOR__CALL__RETRIES__WAITS=200ms,500ms,1s`).
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        Self::load_with(path, Environment::with_prefix("APP"))
    }

    /// `load` with the `APP_*` overrides read from `env` (tests pass them as a map).
    pub(crate) fn load_with(path: &str, env: Environment) -> Result<Self, ConfigError> {
        let env = env
            .prefix_separator("_")
            .separator("__")
            // The only list in the configuration; `try_parsing` is what lets a value become one.
            .try_parsing(true)
            .list_separator(",")
            .with_list_parse_key("kms_aggregator.call.retries.waits");
        let config: Self = Config::builder()
            .add_source(File::with_name(path))
            .add_source(env)
            .build()
            .and_then(Config::try_deserialize)
            .map_err(|e| ConfigError(format!("{path}: {e}")))?;
        config.http.validate()?;
        config.kms_aggregator.validate()?;
        Ok(config)
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
        let config = RelayerConfig::load(EXAMPLE).unwrap();
        assert_eq!(config.name, "zama-relayer-http");
        assert_eq!(config.log.format, "json");
        assert!(config.log.show_timestamp && config.log.show_target);
        assert_eq!(config.kms_aggregator.endpoints.len(), 13);
        assert_eq!(config.kms_aggregator.user_decrypt.threshold, 9);
        assert_eq!(config.kms_aggregator.public_decrypt.threshold, 5);
        assert!(!config.kms_aggregator.skip_tls_verification);
        assert_eq!(
            config.kms_aggregator.call.timeout,
            Duration::from_millis(5000)
        );
    }

    #[test]
    fn environment_overrides_nested_fields() {
        let config = RelayerConfig::load_with(
            EXAMPLE,
            env(&[
                ("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "7s"),
                ("APP_KMS_AGGREGATOR__CALL__RETRIES__WAITS", "200ms,500ms,1s"),
                ("APP_KMS_AGGREGATOR__MAX_CONCURRENT_CALLS", "128"),
                ("APP_LOG__FORMAT", "compact"),
                ("APP_LOG__SHOW_FILE_LINE", "true"),
            ]),
        )
        .unwrap();
        assert_eq!(config.kms_aggregator.call.timeout, Duration::from_secs(7));
        assert_eq!(
            config.kms_aggregator.call.retries.waits,
            [200, 500, 1000].map(Duration::from_millis)
        );
        assert_eq!(config.kms_aggregator.max_concurrent_calls, 128);
        assert_eq!(config.log.format, "compact");
        assert!(config.log.show_file_line);
    }

    #[test]
    fn environment_overrides_the_retry_waits_with_one_entry() {
        let config = RelayerConfig::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__CALL__RETRIES__WAITS", "1s")]),
        )
        .unwrap();
        assert_eq!(
            config.kms_aggregator.call.retries.waits,
            [Duration::from_secs(1)]
        );
    }

    #[test]
    fn unitless_duration_is_rejected() {
        let e = RelayerConfig::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "5000")]),
        )
        .err()
        .unwrap();
        assert!(e.0.contains("timeout"), "{e}");
    }

    #[test]
    fn skip_tls_verification_is_off_unless_set() {
        let example = std::fs::read_to_string(EXAMPLE).unwrap();
        let without = example.replace("skip_tls_verification: false", "");
        assert_ne!(without, example);
        let config: RelayerConfig = Config::builder()
            .add_source(File::from_str(&without, config::FileFormat::Yaml))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();
        assert!(!config.kms_aggregator.skip_tls_verification);
        let config = RelayerConfig::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__SKIP_TLS_VERIFICATION", "true")]),
        )
        .unwrap();
        assert!(config.kms_aggregator.skip_tls_verification);
    }

    #[test]
    fn unknown_nested_field_is_rejected() {
        let e = RelayerConfig::load_with(
            EXAMPLE,
            env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUTS", "5s")]),
        )
        .err()
        .unwrap();
        assert!(e.0.contains("unknown field"), "{e}");
    }

    #[test]
    fn invalid_value_fails_validation() {
        let e =
            RelayerConfig::load_with(EXAMPLE, env(&[("APP_KMS_AGGREGATOR__CALL__TIMEOUT", "0s")]))
                .err()
                .unwrap();
        assert!(e.0.contains("call.timeout"), "{e}");
    }

    #[test]
    fn missing_file_names_the_path() {
        let e = RelayerConfig::load("config/does-not-exist.yaml")
            .err()
            .unwrap();
        assert!(e.0.starts_with("config/does-not-exist.yaml"), "{e}");
    }

    #[test]
    fn http_config_is_loaded_and_validated() {
        let config = RelayerConfig::load(EXAMPLE).unwrap();
        assert_eq!(config.http.endpoint.port(), 8080);
        assert_eq!(config.http.max_body_bytes, 1024 * 1024);
        assert_eq!(config.http.body_read_timeout, Duration::from_secs(10));
        assert_eq!(config.http.supported_chain_ids, vec![1, 137]);

        let e = RelayerConfig::load_with(EXAMPLE, env(&[("APP_HTTP__MAX_BODY_BYTES", "10")]))
            .err()
            .unwrap();
        assert!(e.0.contains("http.max_body_bytes"), "{e}");
        let e = RelayerConfig::load_with(EXAMPLE, env(&[("APP_HTTP__ENDPOINT", "not-an-address")]))
            .err()
            .unwrap();
        assert!(e.0.contains("endpoint"), "{e}");
    }

    #[test]
    fn request_limits_default_to_the_connectors_and_can_be_overridden() {
        let without: HttpConfig =
            serde_json::from_str(r#"{"endpoint": "127.0.0.1:1", "supported_chain_ids": [1]}"#)
                .unwrap();
        assert_eq!(
            (
                without.max_handles_per_decryption,
                without.max_allowed_contracts
            ),
            (50, 10)
        );
        let config = RelayerConfig::load_with(
            EXAMPLE,
            env(&[
                ("APP_HTTP__MAX_HANDLES_PER_DECRYPTION", "100"),
                ("APP_HTTP__MAX_ALLOWED_CONTRACTS", "20"),
            ]),
        )
        .unwrap();
        assert_eq!(
            (
                config.http.max_handles_per_decryption,
                config.http.max_allowed_contracts
            ),
            (100, 20)
        );
    }

    #[test]
    fn body_read_timeout_defaults_overrides_and_bounds() {
        let without: HttpConfig =
            serde_json::from_str(r#"{"endpoint": "127.0.0.1:1", "supported_chain_ids": [1]}"#)
                .unwrap();
        assert_eq!(without.body_read_timeout, Duration::from_secs(10));

        let config =
            RelayerConfig::load_with(EXAMPLE, env(&[("APP_HTTP__BODY_READ_TIMEOUT", "3s")]))
                .unwrap();
        assert_eq!(config.http.body_read_timeout, Duration::from_secs(3));

        for (value, expected) in [
            ("0s", "http.body_read_timeout must be within"),
            ("61s", "http.body_read_timeout must be within"),
            ("-1s", "body_read_timeout"),
            ("10", "body_read_timeout"),
        ] {
            let e =
                RelayerConfig::load_with(EXAMPLE, env(&[("APP_HTTP__BODY_READ_TIMEOUT", value)]))
                    .err()
                    .unwrap();
            assert!(e.0.contains(expected), "{value}: {e}");
        }
    }

    #[test]
    fn http_config_rules() {
        let mut http = HttpConfig::for_tests(&[1, 137]);
        http.validate().unwrap();
        http.max_handles_per_decryption = 0;
        assert!(
            http.validate()
                .unwrap_err()
                .0
                .contains("max_handles_per_decryption")
        );
        http.max_handles_per_decryption = 1;
        http.supported_chain_ids = vec![1, 1];
        assert!(http.validate().unwrap_err().0.contains("twice"));
        http.supported_chain_ids = vec![];
        assert!(http.validate().unwrap_err().0.contains("must not be empty"));
    }

    #[test]
    fn log_config_defaults_to_production_json() {
        let log = LogConfig::default();
        assert_eq!(log.format, "json");
        assert!(log.show_timestamp && log.show_target);
        assert!(!log.show_file_line && !log.show_thread_ids);
        // A file without a `log:` block, or with a partial one, gets the same defaults.
        let parsed: LogConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(parsed, LogConfig::default());
        let partial: LogConfig = serde_json::from_str(r#"{"format": "pretty"}"#).unwrap();
        assert_eq!(partial.format, "pretty");
        assert!(partial.show_timestamp);
        assert!(serde_json::from_str::<LogConfig>(r#"{"level": "info"}"#).is_err());
    }

    #[test]
    fn config_without_log_block_loads_and_logs_json() {
        let dir = std::env::temp_dir().join(format!("relayer-http-config-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("no-log.yaml");
        let without_log: String = std::fs::read_to_string(EXAMPLE)
            .unwrap()
            .lines()
            .filter(|line| {
                !line.starts_with("log:")
                    && !line.starts_with("  format:")
                    && !line.starts_with("  show_")
            })
            .map(|line| format!("{line}\n"))
            .collect();
        std::fs::write(&path, without_log).unwrap();
        let config = RelayerConfig::load(path.to_str().unwrap()).unwrap();
        assert_eq!(config.log, LogConfig::default());
        let _ = std::fs::remove_dir_all(dir);
    }
}
