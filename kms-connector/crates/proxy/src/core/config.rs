use alloy::primitives::Address;
use connector_utils::{
    config::{
        ContractConfig, DeserializeConfig,
        contract::{
            default_protocol_config_contract_config, deserialize_protocol_config_contract_config,
        },
        deserialize_non_zero_duration,
    },
    monitoring::{health::default_healthcheck_timeout, server::default_monitoring_endpoint},
};
use http::uri::Authority;
use serde::{Deserialize, Deserializer};
#[cfg(test)]
use serde::{Serialize, Serializer};
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroU32,
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

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    pub ethereum_chain_id: u64,
    /// The `ProtocolConfig` contract on Ethereum.
    #[serde(deserialize_with = "deserialize_protocol_config_contract_config")]
    pub protocol_config_contract: ContractConfig,
    /// The addresses allowed to sign requests.
    #[serde(deserialize_with = "parse_allowed_senders")]
    pub allowed_senders: HashSet<Address>,
    /// This connector's KMS tx-sender address: the `audience` the accepted headers are signed for.
    pub tx_sender_address: Address,

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
    /// How long the proxy waits for an endpoint response before answering `502`.
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

    /// The maximum accepted size of a request body.
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,
    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_request_read_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub request_read_timeout: Duration,
    /// The maximum number of requests per second accepted from each sender.
    #[serde(default = "default_rate_limit_per_second")]
    pub rate_limit_per_second: NonZeroU32,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    #[serde(with = "humantime_serde", default = "default_shutdown_grace_period")]
    pub shutdown_grace_period: Duration,

    /// The service name used for tracing.
    #[serde(default = "default_service_name")]
    pub service_name: String,
    /// The monitoring server endpoint of the `Proxy` service.
    #[serde(default = "default_monitoring_endpoint")]
    pub monitoring_endpoint: SocketAddr,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(
        deserialize_with = "deserialize_non_zero_duration",
        default = "default_healthcheck_timeout"
    )]
    #[cfg_attr(test, serde(serialize_with = "humantime_serde::serialize"))]
    pub healthcheck_timeout: Duration,
}

fn parse_allowed_senders<'de, D>(d: D) -> Result<HashSet<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let senders = <HashSet<Address>>::deserialize(d)?;
    if senders.is_empty() {
        return Err(serde::de::Error::custom(
            "`allowed_senders` must contain at least one address",
        ));
    }
    Ok(senders)
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

fn default_endpoint_healthcheck_frequency() -> Duration {
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

fn default_rate_limit_per_second() -> NonZeroU32 {
    NonZeroU32::new(100).unwrap()
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
            ethereum_chain_id: 11155111,
            protocol_config_contract: default_protocol_config_contract_config(),
            allowed_senders: HashSet::from([Address::ZERO]),
            tx_sender_address: Address::ZERO,
            endpoint_addresses: vec![
                "kms-connector-endpoint-1:8080".parse().unwrap(),
                "kms-connector-endpoint-2:9090".parse().unwrap(),
            ],
            endpoint_connect_timeout: default_endpoint_connect_timeout(),
            endpoint_healthcheck_frequency: default_endpoint_healthcheck_frequency(),
            endpoint_response_timeout: default_endpoint_response_timeout(),
            endpoint_idle_timeout: default_endpoint_idle_timeout(),
            max_body_bytes: default_max_body_bytes(),
            request_read_timeout: default_request_read_timeout(),
            rate_limit_per_second: default_rate_limit_per_second(),
            shutdown_grace_period: default_shutdown_grace_period(),
            service_name: default_service_name(),
            monitoring_endpoint: default_monitoring_endpoint(),
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
            env::remove_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID");
            env::remove_var("KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS");
            env::remove_var("KMS_CONNECTOR_ALLOWED_SENDERS");
            env::remove_var("KMS_CONNECTOR_TX_SENDER_ADDRESS");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_ADDRESSES");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_CONNECT_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_HEALTHCHECK_FREQUENCY");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_RESPONSE_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_ENDPOINT_IDLE_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_MAX_BODY_BYTES");
            env::remove_var("KMS_CONNECTOR_REQUEST_READ_TIMEOUT");
            env::remove_var("KMS_CONNECTOR_RATE_LIMIT_PER_SECOND");
            env::remove_var("KMS_CONNECTOR_SHUTDOWN_GRACE_PERIOD");
            env::remove_var("KMS_CONNECTOR_SERVICE_NAME");
            env::remove_var("KMS_CONNECTOR_MONITORING_ENDPOINT");
            env::remove_var("KMS_CONNECTOR_HEALTHCHECK_TIMEOUT");
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
            env::set_var("KMS_CONNECTOR_ETHEREUM_CHAIN_ID", "31444");
            env::set_var(
                "KMS_CONNECTOR_PROTOCOL_CONFIG_CONTRACT__ADDRESS",
                "0xc0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0",
            );
            env::set_var(
                "KMS_CONNECTOR_ALLOWED_SENDERS",
                "0x0101010101010101010101010101010101010101,0x0202020202020202020202020202020202020202",
            );
            env::set_var(
                "KMS_CONNECTOR_TX_SENDER_ADDRESS",
                "0xa1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
            );
            env::set_var(
                "KMS_CONNECTOR_ENDPOINT_ADDRESSES",
                "endpoint-1:8080,endpoint-2:8080",
            );
            env::set_var("KMS_CONNECTOR_ENDPOINT_CONNECT_TIMEOUT", "1s");
            env::set_var("KMS_CONNECTOR_ENDPOINT_HEALTHCHECK_FREQUENCY", "500ms");
            env::set_var("KMS_CONNECTOR_ENDPOINT_RESPONSE_TIMEOUT", "45s");
            env::set_var("KMS_CONNECTOR_ENDPOINT_IDLE_TIMEOUT", "7s");
            env::set_var("KMS_CONNECTOR_MAX_BODY_BYTES", "2048");
            env::set_var("KMS_CONNECTOR_REQUEST_READ_TIMEOUT", "12s");
            env::set_var("KMS_CONNECTOR_RATE_LIMIT_PER_SECOND", "1500");
            env::set_var("KMS_CONNECTOR_SHUTDOWN_GRACE_PERIOD", "15s");
            env::set_var("KMS_CONNECTOR_SERVICE_NAME", "kms-connector-test");
            env::set_var("KMS_CONNECTOR_MONITORING_ENDPOINT", "127.0.0.1:9101");
            env::set_var("KMS_CONNECTOR_HEALTHCHECK_TIMEOUT", "7s");
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
        assert_eq!(
            config.protocol_config_contract.address,
            Address::repeat_byte(0xc0)
        );
        assert_eq!(
            config.allowed_senders,
            HashSet::from([Address::repeat_byte(1), Address::repeat_byte(2)])
        );
        assert_eq!(config.tx_sender_address, Address::repeat_byte(0xa1));
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
        assert_eq!(config.max_body_bytes, 2048);
        assert_eq!(config.request_read_timeout, Duration::from_secs(12));
        assert_eq!(config.rate_limit_per_second.get(), 1500);
        assert_eq!(config.shutdown_grace_period, Duration::from_secs(15));
        assert_eq!(config.service_name, "kms-connector-test");
        assert_eq!(
            config.monitoring_endpoint,
            "127.0.0.1:9101".parse().unwrap()
        );
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
}
