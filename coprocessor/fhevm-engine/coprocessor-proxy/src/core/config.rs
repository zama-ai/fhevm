use alloy::{primitives::Address, transports::http::reqwest::Url};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use http::uri::Authority;
use serde::{Deserialize, Deserializer};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    time::Duration,
};
use tracing::Level;

/// The prefix of the environment variables, with `__` between nested keys, e.g.
/// `COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH`.
const ENV_PREFIX: &str = "COPROCESSOR_PROXY";

/// Configuration of the `Proxy` service.
///
/// It derives `Debug` for tests only: `ethereum_rpc_url` can carry an API key.
#[derive(Clone, Deserialize, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Config {
    /// The public bind address of the proxy.
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,
    /// The TLS configuration of the proxy.
    pub tls_config: TlsConfig,

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    pub ethereum_chain_id: u64,
    /// The `ProtocolConfig` contract on Ethereum: part of the `FhevmSig` EIP-712 domain, and the
    /// source of the accepted signers, the KMS tx-senders.
    pub protocol_config_address: Address,
    /// The Ethereum RPC the KMS tx-senders are read through. Never logged.
    #[serde(deserialize_with = "deserialize_secret_url")]
    pub ethereum_rpc_url: Url,
    /// How often the latest finalized block is checked; the KMS tx-senders are read again when it
    /// moved forward. The role of RFC-038's `KMS_REGISTRY_REFRESH_INTERVAL`.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_kms_registry_poll_interval"
    )]
    pub kms_registry_poll_interval: Duration,
    /// This coprocessor's registered signer address: the `audience` the accepted headers are
    /// signed for.
    pub coprocessor_signer_address: Address,

    /// The `host:port` addresses of the endpoints to forward requests to.
    #[serde(deserialize_with = "parse_endpoint_addresses")]
    pub endpoint_addresses: Vec<Authority>,
    /// The timeout to establish a TCP connection to an endpoint.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_connect_timeout"
    )]
    pub endpoint_connect_timeout: Duration,
    /// How often the endpoints are probed in the background to filter which ones receive requests.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_healthcheck_frequency"
    )]
    pub endpoint_healthcheck_frequency: Duration,
    /// How long the proxy waits for an endpoint response before answering `504`.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_response_timeout"
    )]
    pub endpoint_response_timeout: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_idle_timeout"
    )]
    pub endpoint_idle_timeout: Duration,

    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_request_read_timeout"
    )]
    pub request_read_timeout: Duration,
    /// How long the proxy keeps accepting connections after `SIGTERM`, closing each one after its
    /// next response, before the shutdown grace period starts.
    #[serde(with = "humantime_serde", default)]
    pub shutdown_delay: Duration,
    /// How long in-flight requests are given to complete after the shutdown delay.
    #[serde(with = "humantime_serde", default = "default_shutdown_grace_period")]
    pub shutdown_grace_period: Duration,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(deserialize_with = "deserialize_level", default = "default_log_level")]
    pub log_level: Level,
    /// The port of `/healthz`, `/liveness` and `/version`.
    #[serde(default = "default_healthcheck_port")]
    pub healthcheck_port: u16,
    /// The address of the Prometheus `/metrics` endpoint.
    #[serde(default = "default_metrics_endpoint")]
    pub metrics_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_healthcheck_timeout"
    )]
    pub healthcheck_timeout: Duration,
}

impl Config {
    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    pub fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> anyhow::Result<Self> {
        let mut builder = ConfigBuilder::builder();
        if let Some(path) = path {
            let path = path.as_ref();
            let path = path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid config path {}", path.display()))?;
            builder = builder.add_source(File::with_name(path).format(FileFormat::Toml));
        }
        builder = builder.add_source(
            Environment::with_prefix(ENV_PREFIX)
                .prefix_separator("_")
                .separator("__")
                .list_separator(",")
                .with_list_parse_key("endpoint_addresses")
                .try_parsing(true),
        );
        Ok(builder.build()?.try_deserialize()?)
    }
}

/// The TLS material used by the proxy to terminate TLS.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TlsConfig {
    /// The path of the PEM certificate chain.
    pub cert_path: PathBuf,
    /// The path of the PEM private key.
    pub key_path: PathBuf,
}

fn parse_endpoint_addresses<'de, D>(d: D) -> Result<Vec<http::uri::Authority>, D::Error>
where
    D: Deserializer<'de>,
{
    let addresses = <Vec<String>>::deserialize(d)?;
    if addresses.is_empty() {
        return Err(serde::de::Error::custom(
            "`endpoint_addresses` must contain at least one address",
        ));
    }
    addresses
        .iter()
        .map(|address| {
            address.parse().map_err(|e| {
                serde::de::Error::custom(format!("invalid endpoint address `{address}`: {e}"))
            })
        })
        .collect()
}

/// Deserializes a humantime `Duration` and rejects `0s`.
fn deserialize_non_zero_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let duration: Duration = humantime_serde::deserialize(deserializer)?;
    if duration.is_zero() {
        return Err(serde::de::Error::custom("duration must be greater than 0"));
    }
    Ok(duration)
}

/// Parses a URL that can carry an API key: unlike `Url`'s own deserializer, the error leaves the
/// value out.
fn deserialize_secret_url<'de, D>(d: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)?
        .parse()
        .map_err(|e| serde::de::Error::custom(format!("invalid URL: {e}")))
}

fn deserialize_level<'de, D>(d: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn default_service_name() -> String {
    "coprocessor-proxy".to_string()
}

pub fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8443)
}

fn default_kms_registry_poll_interval() -> Duration {
    Duration::from_secs(5)
}

fn default_endpoint_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_healthcheck_frequency() -> Duration {
    Duration::from_secs(3)
}

// An endpoint can hold a request until its result is ready: the response timeout must exceed how
// long the callers wait, so only they give up on a held request. The shutdown grace covers such a
// request.
const HELD_REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

fn default_endpoint_response_timeout() -> Duration {
    HELD_REQUEST_TIMEOUT
}

fn default_endpoint_idle_timeout() -> Duration {
    Duration::from_secs(10)
}

fn default_request_read_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_shutdown_grace_period() -> Duration {
    HELD_REQUEST_TIMEOUT
}

fn default_log_level() -> Level {
    Level::INFO
}

fn default_healthcheck_port() -> u16 {
    8080
}

fn default_metrics_endpoint() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 9100)
}

fn default_healthcheck_timeout() -> Duration {
    Duration::from_secs(3)
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            tls_config: TlsConfig {
                cert_path: PathBuf::from("/etc/coprocessor-proxy/tls/tls.crt"),
                key_path: PathBuf::from("/etc/coprocessor-proxy/tls/tls.key"),
            },
            ethereum_chain_id: 11155111,
            protocol_config_address: Address::ZERO,
            ethereum_rpc_url: "http://localhost:8545".parse().unwrap(),
            kms_registry_poll_interval: default_kms_registry_poll_interval(),
            coprocessor_signer_address: Address::ZERO,
            endpoint_addresses: vec!["endpoint:8080".parse().unwrap()],
            endpoint_connect_timeout: default_endpoint_connect_timeout(),
            endpoint_healthcheck_frequency: default_endpoint_healthcheck_frequency(),
            endpoint_response_timeout: default_endpoint_response_timeout(),
            endpoint_idle_timeout: default_endpoint_idle_timeout(),
            request_read_timeout: default_request_read_timeout(),
            shutdown_delay: Duration::ZERO,
            shutdown_grace_period: default_shutdown_grace_period(),
            service_name: default_service_name(),
            log_level: default_log_level(),
            healthcheck_port: default_healthcheck_port(),
            metrics_endpoint: default_metrics_endpoint(),
            healthcheck_timeout: default_healthcheck_timeout(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::{env, path::PathBuf};

    fn example_config_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/proxy.toml")
    }

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("COPROCESSOR_PROXY_BIND_ADDRESS");
            env::remove_var("COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH");
            env::remove_var("COPROCESSOR_PROXY_TLS_CONFIG__KEY_PATH");
            env::remove_var("COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID");
            env::remove_var("COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS");
            env::remove_var("COPROCESSOR_PROXY_ETHEREUM_RPC_URL");
            env::remove_var("COPROCESSOR_PROXY_KMS_REGISTRY_POLL_INTERVAL");
            env::remove_var("COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS");
            env::remove_var("COPROCESSOR_PROXY_ENDPOINT_ADDRESSES");
            env::remove_var("COPROCESSOR_PROXY_ENDPOINT_CONNECT_TIMEOUT");
            env::remove_var("COPROCESSOR_PROXY_ENDPOINT_HEALTHCHECK_FREQUENCY");
            env::remove_var("COPROCESSOR_PROXY_ENDPOINT_RESPONSE_TIMEOUT");
            env::remove_var("COPROCESSOR_PROXY_ENDPOINT_IDLE_TIMEOUT");
            env::remove_var("COPROCESSOR_PROXY_REQUEST_READ_TIMEOUT");
            env::remove_var("COPROCESSOR_PROXY_SHUTDOWN_DELAY");
            env::remove_var("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD");
            env::remove_var("COPROCESSOR_PROXY_SERVICE_NAME");
            env::remove_var("COPROCESSOR_PROXY_LOG_LEVEL");
            env::remove_var("COPROCESSOR_PROXY_HEALTHCHECK_PORT");
            env::remove_var("COPROCESSOR_PROXY_METRICS_ENDPOINT");
            env::remove_var("COPROCESSOR_PROXY_HEALTHCHECK_TIMEOUT");
        }
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_valid_config_from_file() {
        cleanup_env_vars();
        let default_config = Config::default();
        let example_config = Config::from_env_and_file(Some(example_config_path())).unwrap();
        assert_eq!(default_config, example_config);
    }

    #[test]
    #[serial(config_tests)]
    fn test_load_from_env() {
        cleanup_env_vars();
        unsafe {
            env::set_var("COPROCESSOR_PROXY_BIND_ADDRESS", "127.0.0.1:9443");
            env::set_var(
                "COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH",
                "/etc/proxy/tls.crt",
            );
            env::set_var(
                "COPROCESSOR_PROXY_TLS_CONFIG__KEY_PATH",
                "/etc/proxy/tls.key",
            );
            env::set_var("COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID", "31444");
            env::set_var(
                "COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS",
                "0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0",
            );
            env::set_var(
                "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
                "https://rpc.example/v1/key",
            );
            env::set_var("COPROCESSOR_PROXY_KMS_REGISTRY_POLL_INTERVAL", "2m");
            env::set_var(
                "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS",
                "0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
            );
            env::set_var(
                "COPROCESSOR_PROXY_ENDPOINT_ADDRESSES",
                "endpoint-1:8080,endpoint-2:8080",
            );
            env::set_var("COPROCESSOR_PROXY_ENDPOINT_CONNECT_TIMEOUT", "1s");
            env::set_var("COPROCESSOR_PROXY_ENDPOINT_HEALTHCHECK_FREQUENCY", "500ms");
            env::set_var("COPROCESSOR_PROXY_ENDPOINT_RESPONSE_TIMEOUT", "45s");
            env::set_var("COPROCESSOR_PROXY_ENDPOINT_IDLE_TIMEOUT", "7s");
            env::set_var("COPROCESSOR_PROXY_REQUEST_READ_TIMEOUT", "12s");
            env::set_var("COPROCESSOR_PROXY_SHUTDOWN_DELAY", "9s");
            env::set_var("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD", "15s");
            env::set_var("COPROCESSOR_PROXY_SERVICE_NAME", "coprocessor-proxy-test");
            env::set_var("COPROCESSOR_PROXY_LOG_LEVEL", "debug");
            env::set_var("COPROCESSOR_PROXY_HEALTHCHECK_PORT", "8081");
            env::set_var("COPROCESSOR_PROXY_METRICS_ENDPOINT", "127.0.0.1:9101");
            env::set_var("COPROCESSOR_PROXY_HEALTHCHECK_TIMEOUT", "7s");
        }

        let config = Config::from_env_and_file::<&str>(None).unwrap();

        assert_eq!(config.bind_address, "127.0.0.1:9443".parse().unwrap());
        assert_eq!(
            config.tls_config,
            TlsConfig {
                cert_path: PathBuf::from("/etc/proxy/tls.crt"),
                key_path: PathBuf::from("/etc/proxy/tls.key"),
            }
        );
        assert_eq!(config.ethereum_chain_id, 31444);
        assert_eq!(config.protocol_config_address, Address::repeat_byte(0xc0));
        assert_eq!(
            config.ethereum_rpc_url,
            "https://rpc.example/v1/key".parse().unwrap()
        );
        assert_eq!(config.kms_registry_poll_interval, Duration::from_secs(120));
        assert_eq!(
            config.coprocessor_signer_address,
            Address::repeat_byte(0xa1)
        );
        assert_eq!(
            config.endpoint_addresses,
            vec![
                "endpoint-1:8080".parse::<Authority>().unwrap(),
                "endpoint-2:8080".parse::<Authority>().unwrap(),
            ]
        );
        assert_eq!(config.endpoint_connect_timeout, Duration::from_secs(1));
        assert_eq!(
            config.endpoint_healthcheck_frequency,
            Duration::from_millis(500)
        );
        assert_eq!(config.endpoint_response_timeout, Duration::from_secs(45));
        assert_eq!(config.endpoint_idle_timeout, Duration::from_secs(7));
        assert_eq!(config.request_read_timeout, Duration::from_secs(12));
        assert_eq!(config.shutdown_delay, Duration::from_secs(9));
        assert_eq!(config.shutdown_grace_period, Duration::from_secs(15));
        assert_eq!(config.service_name, "coprocessor-proxy-test");
        assert_eq!(config.log_level, Level::DEBUG);
        assert_eq!(config.healthcheck_port, 8081);
        assert_eq!(config.metrics_endpoint, "127.0.0.1:9101".parse().unwrap());
        assert_eq!(config.healthcheck_timeout, Duration::from_secs(7));

        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_missing_required_field_fails() {
        cleanup_env_vars();
        let toml = r#"endpoint_addresses = ["localhost:8080"]"#;
        let tmp = std::env::temp_dir().join(format!("proxy-cfg-{}.toml", std::process::id()));
        std::fs::write(&tmp, toml).unwrap();
        assert!(Config::from_env_and_file(Some(&tmp)).is_err());
        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    #[serial(config_tests)]
    fn test_invalid_rpc_url_error_leaves_the_url_out() {
        cleanup_env_vars();
        unsafe {
            // No scheme: a relative URL, refused.
            env::set_var(
                "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
                "eth.example/v2/SECRET-KEY",
            );
        }
        let Err(err) = Config::from_env_and_file(Some(example_config_path())) else {
            panic!("a relative RPC URL must be refused");
        };
        let message = format!("{err:#}");
        assert!(message.contains("invalid URL"), "{message}");
        assert!(!message.contains("SECRET-KEY"), "{message}");
        cleanup_env_vars();
    }

    #[test]
    #[serial(config_tests)]
    fn test_zero_poll_interval_is_refused() {
        cleanup_env_vars();
        unsafe {
            // The reader would poll the RPC without pause.
            env::set_var("COPROCESSOR_PROXY_KMS_REGISTRY_POLL_INTERVAL", "0s");
        }
        let Err(err) = Config::from_env_and_file(Some(example_config_path())) else {
            panic!("a zero poll interval must be refused");
        };
        let message = format!("{err:#}");
        assert!(message.contains("greater than 0"), "{message}");
        cleanup_env_vars();
    }
}
