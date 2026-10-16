use alloy::{primitives::Address, transports::http::reqwest::Url};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use http::uri::Authority;
use serde::{Deserialize, Deserializer};
#[cfg(test)]
use serde::{Serialize, Serializer};
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
#[cfg_attr(test, derive(Debug, Serialize))]
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
    pub ethereum_rpc_url: Url,
    /// How often the KMS tx-senders are read again (RFC-038 `KMS_REGISTRY_REFRESH_INTERVAL`).
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_kms_registry_refresh_interval"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub kms_registry_refresh_interval: Duration,
    /// This coprocessor's registered signer address: the `audience` the accepted headers are
    /// signed for.
    pub coprocessor_signer_address: Address,

    /// The `host:port` addresses of the endpoints to forward requests to.
    #[serde(deserialize_with = "parse_endpoint_addresses")]
    #[cfg_attr(test, serde(serialize_with = "serialize_endpoint_addresses"))]
    pub endpoint_addresses: Vec<Authority>,
    /// The timeout to establish a TCP connection to an endpoint.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_connect_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub endpoint_connect_timeout: Duration,
    /// How often the endpoints are probed in the background to filter which ones receive requests.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_healthcheck_frequency"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub endpoint_healthcheck_frequency: Duration,
    /// How long the proxy waits for an endpoint response before answering `504`.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_response_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub endpoint_response_timeout: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_endpoint_idle_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub endpoint_idle_timeout: Duration,
    /// The maximum number of concurrent requests on one HTTP/2 endpoint connection.
    #[serde(default = "default_endpoint_max_h2_streams")]
    pub endpoint_max_h2_streams: usize,

    /// The maximum accepted size of a request body.
    #[serde(default)]
    pub max_body_bytes: usize,
    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_request_read_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub request_read_timeout: Duration,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    #[serde(with = "humantime_serde", default = "default_shutdown_grace_period")]
    pub shutdown_grace_period: Duration,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    #[serde(deserialize_with = "deserialize_level", default = "default_log_level")]
    #[cfg_attr(test, serde(serialize_with = "serialize_level"))]
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
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
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
#[cfg_attr(test, derive(Serialize))]
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

fn default_kms_registry_refresh_interval() -> Duration {
    Duration::from_secs(60)
}

fn default_endpoint_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_healthcheck_frequency() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_response_timeout() -> Duration {
    // An endpoint can hold a request until its result is ready: this must exceed how long the
    // callers wait, so only they give up on a held request.
    Duration::from_secs(120)
}

fn default_endpoint_idle_timeout() -> Duration {
    // Short on purpose: a Kubernetes ClusterIP Service picks the target pod once per TCP
    // connection, not per request, so all requests sent over a long-lived pooled connection
    // keep landing on the same pod. A short idle timeout forces the pool to reconnect
    // regularly, letting the Service's balancing reach newly scaled-up or restarted pods.
    Duration::from_secs(10)
}

fn default_endpoint_max_h2_streams() -> usize {
    100
}

fn default_request_read_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_shutdown_grace_period() -> Duration {
    // Covers a request held for the whole `endpoint_response_timeout`.
    Duration::from_secs(120)
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
            kms_registry_refresh_interval: default_kms_registry_refresh_interval(),
            coprocessor_signer_address: Address::ZERO,
            endpoint_addresses: vec!["endpoint:8080".parse().unwrap()],
            endpoint_connect_timeout: default_endpoint_connect_timeout(),
            endpoint_healthcheck_frequency: default_endpoint_healthcheck_frequency(),
            endpoint_response_timeout: default_endpoint_response_timeout(),
            endpoint_idle_timeout: default_endpoint_idle_timeout(),
            endpoint_max_h2_streams: default_endpoint_max_h2_streams(),
            max_body_bytes: 0,
            request_read_timeout: default_request_read_timeout(),
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
fn serialize_endpoint_addresses<S>(
    addresses: &[http::uri::Authority],
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_seq(addresses.iter().map(Authority::as_str))
}

#[cfg(test)]
fn serialize_level<S: Serializer>(level: &Level, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(level.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::{env, path::PathBuf};

    fn example_config_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/proxy.toml")
    }

    const ENV_VARS: [&str; 22] = [
        "COPROCESSOR_PROXY_BIND_ADDRESS",
        "COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH",
        "COPROCESSOR_PROXY_TLS_CONFIG__KEY_PATH",
        "COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID",
        "COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS",
        "COPROCESSOR_PROXY_ETHEREUM_RPC_URL",
        "COPROCESSOR_PROXY_KMS_REGISTRY_REFRESH_INTERVAL",
        "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS",
        "COPROCESSOR_PROXY_ENDPOINT_ADDRESSES",
        "COPROCESSOR_PROXY_ENDPOINT_CONNECT_TIMEOUT",
        "COPROCESSOR_PROXY_ENDPOINT_HEALTHCHECK_FREQUENCY",
        "COPROCESSOR_PROXY_ENDPOINT_RESPONSE_TIMEOUT",
        "COPROCESSOR_PROXY_ENDPOINT_IDLE_TIMEOUT",
        "COPROCESSOR_PROXY_ENDPOINT_MAX_H2_STREAMS",
        "COPROCESSOR_PROXY_MAX_BODY_BYTES",
        "COPROCESSOR_PROXY_REQUEST_READ_TIMEOUT",
        "COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD",
        "COPROCESSOR_PROXY_SERVICE_NAME",
        "COPROCESSOR_PROXY_LOG_LEVEL",
        "COPROCESSOR_PROXY_HEALTHCHECK_PORT",
        "COPROCESSOR_PROXY_METRICS_ENDPOINT",
        "COPROCESSOR_PROXY_HEALTHCHECK_TIMEOUT",
    ];

    fn cleanup_env_vars() {
        for name in ENV_VARS {
            unsafe { env::remove_var(name) };
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
        for (name, value) in [
            ("BIND_ADDRESS", "127.0.0.1:9443"),
            ("TLS_CONFIG__CERT_PATH", "/etc/proxy/tls.crt"),
            ("TLS_CONFIG__KEY_PATH", "/etc/proxy/tls.key"),
            ("ETHEREUM_CHAIN_ID", "31444"),
            (
                "PROTOCOL_CONFIG_ADDRESS",
                "0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0",
            ),
            ("ETHEREUM_RPC_URL", "https://rpc.example/v1/key"),
            ("KMS_REGISTRY_REFRESH_INTERVAL", "2m"),
            (
                "COPROCESSOR_SIGNER_ADDRESS",
                "0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
            ),
            ("ENDPOINT_ADDRESSES", "endpoint-1:8080,endpoint-2:8080"),
            ("ENDPOINT_CONNECT_TIMEOUT", "1s"),
            ("ENDPOINT_HEALTHCHECK_FREQUENCY", "500ms"),
            ("ENDPOINT_RESPONSE_TIMEOUT", "45s"),
            ("ENDPOINT_IDLE_TIMEOUT", "7s"),
            ("ENDPOINT_MAX_H2_STREAMS", "50"),
            ("MAX_BODY_BYTES", "2048"),
            ("REQUEST_READ_TIMEOUT", "12s"),
            ("SHUTDOWN_GRACE_PERIOD", "15s"),
            ("SERVICE_NAME", "coprocessor-proxy-test"),
            ("LOG_LEVEL", "debug"),
            ("HEALTHCHECK_PORT", "8081"),
            ("METRICS_ENDPOINT", "127.0.0.1:9101"),
            ("HEALTHCHECK_TIMEOUT", "7s"),
        ] {
            unsafe { env::set_var(format!("{ENV_PREFIX}_{name}"), value) };
        }

        let config = Config::from_env_and_file::<&str>(None).unwrap();

        assert_eq!(
            config,
            Config {
                bind_address: "127.0.0.1:9443".parse().unwrap(),
                tls_config: TlsConfig {
                    cert_path: PathBuf::from("/etc/proxy/tls.crt"),
                    key_path: PathBuf::from("/etc/proxy/tls.key"),
                },
                ethereum_chain_id: 31444,
                protocol_config_address: Address::repeat_byte(0xc0),
                ethereum_rpc_url: "https://rpc.example/v1/key".parse().unwrap(),
                kms_registry_refresh_interval: Duration::from_secs(120),
                coprocessor_signer_address: Address::repeat_byte(0xa1),
                endpoint_addresses: vec![
                    "endpoint-1:8080".parse().unwrap(),
                    "endpoint-2:8080".parse().unwrap(),
                ],
                endpoint_connect_timeout: Duration::from_secs(1),
                endpoint_healthcheck_frequency: Duration::from_millis(500),
                endpoint_response_timeout: Duration::from_secs(45),
                endpoint_idle_timeout: Duration::from_secs(7),
                endpoint_max_h2_streams: 50,
                max_body_bytes: 2048,
                request_read_timeout: Duration::from_secs(12),
                shutdown_grace_period: Duration::from_secs(15),
                service_name: "coprocessor-proxy-test".to_string(),
                log_level: Level::DEBUG,
                healthcheck_port: 8081,
                metrics_endpoint: "127.0.0.1:9101".parse().unwrap(),
                healthcheck_timeout: Duration::from_secs(7),
            }
        );

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
}
