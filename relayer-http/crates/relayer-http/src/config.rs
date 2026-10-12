//! Process configuration: one YAML file, `APP_<SECTION>__<FIELD>` environment overrides.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::time::Duration;

use config::{Config, Environment, File};
use serde::{Deserialize, Deserializer};
use zama_solana_acl::host_chain::is_solana_host_chain_id;

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
    /// Total time to receive and parse a request body, from the handler's start (headers already read). Bounds a
    /// client trickling its body: after headers, a request lasts at most `body_read_timeout + call.timeout`
    /// (10 s + 5 s by default). Unit required (`10s`, `500ms`); within `1ms..=60s`.
    #[serde(with = "humantime_serde", default = "default_body_read_timeout")]
    pub body_read_timeout: Duration,
    /// EVM host chain ids a ciphertext handle may carry (bytes 22..30 of the handle).
    #[serde(default)]
    pub supported_chain_ids: Vec<u64>,
    /// The Solana host chains served, keyed by chain id.
    #[serde(default)]
    pub solana_chains: BTreeMap<u64, SolanaChainConfig>,
}

/// A supported Solana host chain. Both values come from the deployment's Solana environment.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolanaChainConfig {
    /// The zama-host program id, base58. A user-decrypt permit must name it.
    #[serde(deserialize_with = "program_id_from_base58")]
    pub program_id: [u8; 32],
    /// Identical public-decrypt answers needed. zama-host verifies a certificate against
    /// `kms_context.thresholds.public_decryption` (2t + 1), more than EVM's t + 1.
    pub public_decrypt_threshold: usize,
}

/// A base58 Solana address of exactly 32 bytes.
fn program_id_from_base58<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<[u8; 32], D::Error> {
    let text = String::deserialize(deserializer)?;
    let bytes = bs58::decode(&text)
        .into_vec()
        .map_err(|e| serde::de::Error::custom(format!("program_id: {e}")))?;
    bytes.try_into().map_err(|b: Vec<u8>| {
        serde::de::Error::custom(format!("program_id is {} bytes, expected 32", b.len()))
    })
}

/// 1 MiB, the connector endpoint's and proxy's limit: a bigger body could never be forwarded. The largest valid
/// request (1024 handles, the 2048-bit budget) is about 215 KiB.
fn default_max_body_bytes() -> usize {
    1024 * 1024
}

/// 10 s: a 1 MiB body still fits at ~100 KiB/s.
fn default_body_read_timeout() -> Duration {
    Duration::from_secs(10)
}

impl HttpConfig {
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
        if self.supported_chain_ids.is_empty() && self.solana_chains.is_empty() {
            return Err(ConfigError(
                "http.supported_chain_ids and http.solana_chains must not both be empty".to_owned(),
            ));
        }
        let mut seen = std::collections::HashSet::new();
        if let Some(dup) = self.supported_chain_ids.iter().find(|id| !seen.insert(*id)) {
            return Err(ConfigError(format!(
                "http.supported_chain_ids contains {dup} twice"
            )));
        }
        if let Some(id) = self
            .supported_chain_ids
            .iter()
            .find(|id| is_solana_host_chain_id(**id))
        {
            return Err(ConfigError(format!(
                "http.supported_chain_ids contains the Solana chain {id}: list it under http.solana_chains"
            )));
        }
        Ok(())
    }

    /// Every entry is a Solana chain, with a public-decrypt threshold within `1..=nodes`.
    pub fn validate_solana_chains(&self, nodes: usize) -> Result<(), ConfigError> {
        for (id, chain) in &self.solana_chains {
            if !is_solana_host_chain_id(*id) {
                return Err(ConfigError(format!(
                    "http.solana_chains.{id} is not a Solana chain id"
                )));
            }
            let threshold = chain.public_decrypt_threshold;
            if threshold == 0 || threshold > nodes {
                return Err(ConfigError(format!(
                    "http.solana_chains.{id}.public_decrypt_threshold ({threshold}) must be within 1..={nodes}"
                )));
            }
        }
        Ok(())
    }
}

/// The current relayer's log configuration, same keys and meaning. Optional: without a `log:` block the process
/// logs JSON lines (production). Overrides: `APP_LOG__FORMAT`, `APP_LOG__SHOW_FILE_LINE`, ...; the level filter is
/// `RUST_LOG` (default `warn,relayer_http=info`).
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
    /// Reads `path`, applies `APP_*` environment overrides, validates. Durations need a unit (`5s`, `500ms`).
    pub fn load(path: &str) -> Result<Self, ConfigError> {
        Self::load_with(path, Environment::with_prefix("APP"))
    }

    fn load_with(path: &str, env: Environment) -> Result<Self, ConfigError> {
        let config: Self = Config::builder()
            .add_source(File::with_name(path))
            .add_source(env.prefix_separator("_").separator("__"))
            .build()
            .and_then(Config::try_deserialize)
            .map_err(|e| ConfigError(format!("{path}: {e}")))?;
        config.http.validate()?;
        config.kms_aggregator.validate()?;
        config
            .http
            .validate_solana_chains(config.kms_aggregator.endpoints.len())?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;

    use super::*;
    use crate::endpoint::validate::tests::{PROGRAM, SOLANA};

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
        assert_eq!(config.log.format, "pretty");
        assert!(config.log.show_timestamp && config.log.show_target);
        assert_eq!(config.kms_aggregator.endpoints.len(), 13);
        assert_eq!(config.kms_aggregator.user_decrypt.threshold, 9);
        assert_eq!(config.kms_aggregator.public_decrypt.threshold, 5);
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
                ("APP_KMS_AGGREGATOR__MAX_CONCURRENT_CALLS", "128"),
                ("APP_LOG__FORMAT", "compact"),
                ("APP_LOG__SHOW_FILE_LINE", "true"),
            ]),
        )
        .unwrap();
        assert_eq!(config.kms_aggregator.call.timeout, Duration::from_secs(7));
        assert_eq!(config.kms_aggregator.max_concurrent_calls, 128);
        assert_eq!(config.log.format, "compact");
        assert!(config.log.show_file_line);
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
        let mut http = HttpConfig {
            endpoint: "127.0.0.1:8080".parse().unwrap(),
            max_body_bytes: 4096,
            body_read_timeout: Duration::from_secs(10),
            supported_chain_ids: vec![1, 137],
            solana_chains: BTreeMap::new(),
        };
        http.validate().unwrap();
        http.supported_chain_ids = vec![1, 1];
        assert!(http.validate().unwrap_err().0.contains("twice"));
        http.supported_chain_ids = vec![];
        assert!(
            http.validate()
                .unwrap_err()
                .0
                .contains("must not both be empty")
        );
    }

    /// An `http` section serving chain 1 and the Solana chain `SOLANA` on `PROGRAM`.
    fn solana_http() -> HttpConfig {
        HttpConfig {
            supported_chain_ids: vec![1],
            ..crate::endpoint::validate::tests::http()
        }
    }

    #[test]
    fn solana_chain_keys_load_as_chain_ids() {
        let yaml = format!(
            "endpoint: \"0.0.0.0:8080\"\nsolana_chains:\n  \"{SOLANA}\":\n    program_id: \"{}\"\n    public_decrypt_threshold: 9\n",
            bs58::encode(PROGRAM).into_string()
        );
        let http: HttpConfig = Config::builder()
            .add_source(File::from_str(&yaml, config::FileFormat::Yaml))
            .build()
            .and_then(Config::try_deserialize)
            .unwrap();
        assert_eq!(http.solana_chains[&SOLANA].program_id, PROGRAM);
        assert_eq!(http.solana_chains[&SOLANA].public_decrypt_threshold, 9);
        http.validate().unwrap();
    }

    #[test]
    fn program_id_is_32_bytes_of_base58() {
        let parse = |program: &str| {
            serde_json::from_str::<SolanaChainConfig>(&format!(
                r#"{{"program_id": "{program}", "public_decrypt_threshold": 9}}"#
            ))
        };
        assert_eq!(
            parse(&bs58::encode(PROGRAM).into_string())
                .unwrap()
                .program_id,
            PROGRAM
        );
        assert!(
            parse("0OIl")
                .unwrap_err()
                .to_string()
                .contains("program_id")
        );
        assert!(
            parse("11111111")
                .unwrap_err()
                .to_string()
                .contains("program_id is 8 bytes, expected 32")
        );
    }

    #[test]
    fn solana_chains_rules() {
        let mut http = solana_http();
        http.validate().unwrap();
        http.validate_solana_chains(13).unwrap();

        http.supported_chain_ids = vec![1, SOLANA];
        assert!(
            http.validate()
                .unwrap_err()
                .0
                .contains(&format!("contains the Solana chain {SOLANA}"))
        );

        // A Solana-only relayer is valid; one serving no chain is not.
        http.supported_chain_ids = vec![];
        http.validate().unwrap();
        http.solana_chains.clear();
        assert!(
            http.validate()
                .unwrap_err()
                .0
                .contains("must not both be empty")
        );

        let mut http = solana_http();
        let chain = http.solana_chains[&SOLANA].clone();
        http.solana_chains.insert(1, chain);
        assert!(
            http.validate_solana_chains(13)
                .unwrap_err()
                .0
                .contains("http.solana_chains.1 is not a Solana chain id")
        );

        for threshold in [0, 14] {
            let mut http = solana_http();
            http.solana_chains
                .get_mut(&SOLANA)
                .unwrap()
                .public_decrypt_threshold = threshold;
            assert!(
                http.validate_solana_chains(13)
                    .unwrap_err()
                    .0
                    .contains(&format!(
                        "public_decrypt_threshold ({threshold}) must be within 1..=13"
                    )),
                "{threshold}"
            );
        }
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
