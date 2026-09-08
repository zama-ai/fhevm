use alloy::primitives::B256;
use connector_utils::config::DeserializeConfig;
use http::uri::Authority;
use serde::{Deserialize, Deserializer};
#[cfg(test)]
use serde::{Serialize, Serializer};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

/// Configuration of the `Proxy` service.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(Serialize))]
pub struct Config {
    /// The public bind address of the proxy.
    #[serde(alias = "proxy_bind_address", default = "default_bind_address")]
    pub bind_address: SocketAddr,
    /// The TLS configuration of the proxy.
    pub tls_config: TlsConfig,
    /// The SHA-256 digest of the API key the relayer authenticates with. The plaintext key is
    /// never stored by the proxy.
    pub api_key_digest: B256,

    /// The `host:port` addresses of the endpoints to forward requests to.
    #[serde(deserialize_with = "parse_endpoint_addresses")]
    #[cfg_attr(test, serde(serialize_with = "serialize_endpoint_addresses"))]
    pub endpoint_addresses: Vec<Authority>,
    /// The timeout to establish a TCP connection to an endpoint.
    #[serde(with = "humantime_serde", default = "default_endpoint_connect_timeout")]
    pub endpoint_connect_timeout: Duration,
    /// How long the proxy waits for an endpoint response before answering `502`.
    #[serde(
        with = "humantime_serde",
        default = "default_endpoint_response_timeout"
    )]
    pub endpoint_response_timeout: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    #[serde(with = "humantime_serde", default = "default_endpoint_idle_timeout")]
    pub endpoint_idle_timeout: Duration,

    /// The maximum accepted size of a request body.
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,
    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(with = "humantime_serde", default = "default_request_read_timeout")]
    pub request_read_timeout: Duration,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    #[serde(with = "humantime_serde", default = "default_shutdown_grace_period")]
    pub shutdown_grace_period: Duration,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
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

impl DeserializeConfig for Config {}

fn default_service_name() -> String {
    "kms-connector-proxy".to_string()
}

pub fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8443)
}

fn default_endpoint_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_response_timeout() -> Duration {
    // Should exceed the endpoint's `decryption_timeout` (30s by default) so the endpoint's own
    // `504 timeout` reaches the relayer instead of a proxy `502`.
    Duration::from_secs(60)
}

fn default_endpoint_idle_timeout() -> Duration {
    // Short on purpose: a Kubernetes ClusterIP Service picks the target pod once per TCP
    // connection, not per request, so all requests sent over a long-lived pooled connection
    // keep landing on the same pod. A short idle timeout forces the pool to reconnect
    // regularly, letting the Service's balancing reach newly scaled-up or restarted pods.
    Duration::from_secs(10)
}

fn default_max_body_bytes() -> usize {
    1024 * 1024 // 1 MiB, mirrors the endpoint's default
}

fn default_request_read_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_shutdown_grace_period() -> Duration {
    Duration::from_secs(60)
}

// Default implementation for testing purpose
impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            tls_config: TlsConfig {
                cert_path: PathBuf::from("/etc/kms-connector/proxy/tls.crt"),
                key_path: PathBuf::from("/etc/kms-connector/proxy/tls.key"),
            },
            api_key_digest: "0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
                .parse()
                .unwrap(),
            endpoint_addresses: vec!["kms-connector-endpoint:8080".parse().unwrap()],
            endpoint_connect_timeout: default_endpoint_connect_timeout(),
            endpoint_response_timeout: default_endpoint_response_timeout(),
            endpoint_idle_timeout: default_endpoint_idle_timeout(),
            max_body_bytes: default_max_body_bytes(),
            request_read_timeout: default_request_read_timeout(),
            shutdown_grace_period: default_shutdown_grace_period(),
            service_name: default_service_name(),
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
mod tests {
    use super::*;
    use serial_test::serial;
    use std::{env, path::PathBuf};

    fn example_config_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/proxy.toml")
    }

    fn cleanup_env_vars() {
        unsafe {
            env::remove_var("KMS_CONNECTOR_PROXY_BIND_ADDRESS");
            env::remove_var("KMS_CONNECTOR_TLS_CONFIG__CERT_PATH");
            env::remove_var("KMS_CONNECTOR_TLS_CONFIG__KEY_PATH");
            env::remove_var("KMS_CONNECTOR_API_KEY_DIGEST");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_ADDRESSES");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_CONNECT_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_RESPONSE_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_IDLE_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_MAX_BODY_BYTES");
            env::remove_var("KMS_CONNECTOR_REQUEST_READ_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_SHUTDOWN_GRACE_PERIOD");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
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
            env::set_var("KMS_CONNECTOR_PROXY_BIND_ADDRESS", "127.0.0.1:9443");
            env::set_var("KMS_CONNECTOR_TLS_CONFIG__CERT_PATH", "/etc/proxy/tls.crt");
            env::set_var("KMS_CONNECTOR_TLS_CONFIG__KEY_PATH", "/etc/proxy/tls.key");
            env::set_var(
                "KMS_CONNECTOR_API_KEY_DIGEST",
                "0x0000000000000000000000000000000000000000000000000000000000000001",
            );
            env::set_var(
                "KMS_CONNECTOR_ENDPOINT_ADDRESSES",
                "endpoint-1:8080,endpoint-2:8080",
            );
            env::set_var("KMS_CONNECTOR_ENDPOINT_CONNECT_TIMEOUT", "1s");
            env::set_var("KMS_CONNECTOR_ENDPOINT_RESPONSE_TIMEOUT", "45s");
            env::set_var("KMS_CONNECTOR_ENDPOINT_IDLE_TIMEOUT", "7s");
            env::set_var("KMS_CONNECTOR_MAX_BODY_BYTES", "2048");
            env::set_var("KMS_CONNECTOR_REQUEST_READ_TIMEOUT", "12s");
            env::set_var("KMS_CONNECTOR_SHUTDOWN_GRACE_PERIOD", "15s");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
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
        assert_eq!(config.api_key_digest, B256::with_last_byte(1));
        assert_eq!(
            config.endpoint_addresses,
            vec![
                "endpoint-1:8080".parse::<Authority>().unwrap(),
                "endpoint-2:8080".parse::<Authority>().unwrap(),
            ]
        );
        assert_eq!(config.endpoint_connect_timeout, Duration::from_secs(1));
        assert_eq!(config.endpoint_response_timeout, Duration::from_secs(45));
        assert_eq!(config.endpoint_idle_timeout, Duration::from_secs(7));
        assert_eq!(config.max_body_bytes, 2048);
        assert_eq!(config.request_read_timeout, Duration::from_secs(12));
        assert_eq!(config.shutdown_grace_period, Duration::from_secs(15));
        assert_eq!(config.service_name, "kms-connector-test");

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
