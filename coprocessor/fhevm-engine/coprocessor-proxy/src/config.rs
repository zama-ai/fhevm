use alloy::primitives::Address;
use anyhow::Context;
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use fhevm_sig_proxy::{ProxyConfig, Route, SignerSource, Upstream};
use http::Method;
use serde::{Deserialize, Deserializer};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    time::Duration,
};
use tracing::Level;
use url::Url;

/// The prefix of the environment variables that override the configuration file, e.g.
/// `COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH`.
pub const ENV_PREFIX: &str = "COPROCESSOR_PROXY";

/// Configuration of the coprocessor proxy.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Config {
    /// The public bind address of the proxy.
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,
    /// The TLS configuration of the proxy.
    pub tls_config: TlsConfig,

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    pub ethereum_chain_id: u64,
    /// The Ethereum RPC the KMS tx-senders are read from.
    pub ethereum_rpc_url: Url,
    /// The `ProtocolConfig` contract on Ethereum: the domain's `verifyingContract`, and the list of
    /// the KMS tx-senders allowed to sign requests.
    pub protocol_config_contract: ContractConfig,
    /// How often the KMS tx-senders are read from `ProtocolConfig`.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_signer_refresh_interval"
    )]
    pub signer_refresh_interval: Duration,
    /// This coprocessor's signer address: the `audience` the accepted headers are signed for.
    pub coprocessor_signer_address: Address,

    /// The pools of endpoints the routes are forwarded to.
    pub upstreams: Vec<UpstreamConfig>,
    /// The routes forwarded to the endpoints; any other request is rejected.
    pub routes: Vec<RouteConfig>,
    /// The timeout to establish a TCP connection to an endpoint.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_endpoint_connect_timeout"
    )]
    pub endpoint_connect_timeout: Duration,
    /// How often the endpoints are probed in the background to filter which ones receive requests.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_endpoint_healthcheck_frequency"
    )]
    pub endpoint_healthcheck_frequency: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_endpoint_idle_timeout"
    )]
    pub endpoint_idle_timeout: Duration,

    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_request_read_timeout"
    )]
    pub request_read_timeout: Duration,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_shutdown_grace_period"
    )]
    pub shutdown_grace_period: Duration,

    /// The service name used in logs.
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
    /// The timeout to perform each healthcheck.
    #[serde(
        deserialize_with = "deserialize_duration",
        default = "default_healthcheck_timeout"
    )]
    pub healthcheck_timeout: Duration,
}

/// The TLS material used by the proxy to terminate TLS.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TlsConfig {
    /// The path of the PEM certificate chain.
    pub cert_path: PathBuf,
    /// The path of the PEM private key.
    pub key_path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ContractConfig {
    pub address: Address,
}

/// A pool of interchangeable endpoints, see [`Upstream`].
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UpstreamConfig {
    pub name: String,
    /// The `host:port` addresses of the endpoints, reached over plain HTTP.
    pub endpoint_addresses: Vec<String>,
    /// The path the endpoints are probed on; a healthy endpoint answers `200`.
    #[serde(default = "default_healthcheck_path")]
    pub healthcheck_path: String,
}

/// A forwarded route, see [`Route`].
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RouteConfig {
    pub methods: Vec<String>,
    /// The path template, e.g. `/v1/prf/{prf}/labels/{label}/{padLength}`.
    pub path: String,
    /// The name of the upstream the route is forwarded to.
    pub upstream: String,
    /// How long the proxy waits for an endpoint response before answering `504`. It must exceed
    /// the time the endpoint holds a request, so the client gets the endpoint's own answer.
    #[serde(deserialize_with = "deserialize_duration")]
    pub endpoint_response_timeout: Duration,
    /// The maximum accepted size of a request body; `0` refuses any body.
    #[serde(default)]
    pub max_body_bytes: usize,
}

impl Config {
    /// Loads the configuration from an optional TOML file, then from the environment variables
    /// prefixed with [`ENV_PREFIX`], which take precedence.
    pub fn from_env_and_file(path: Option<&Path>) -> anyhow::Result<Self> {
        let mut builder = ConfigBuilder::builder();
        if let Some(path) = path {
            let path = path.to_str().context("config path is not valid UTF-8")?;
            builder = builder.add_source(File::with_name(path).format(FileFormat::Toml));
        }
        builder = builder.add_source(
            Environment::with_prefix(ENV_PREFIX)
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true),
        );
        Ok(builder.build()?.try_deserialize()?)
    }

    /// The configuration of the proxy itself.
    pub fn proxy_config(&self) -> anyhow::Result<ProxyConfig> {
        let upstreams = self
            .upstreams
            .iter()
            .map(|upstream| {
                let endpoint_addresses = upstream
                    .endpoint_addresses
                    .iter()
                    .map(|address| address.parse())
                    .collect::<Result<_, _>>()
                    .with_context(|| format!("upstream {}: endpoint_addresses", upstream.name))?;
                Ok(Upstream {
                    name: upstream.name.clone(),
                    endpoint_addresses,
                    healthcheck_path: upstream.healthcheck_path.clone(),
                })
            })
            .collect::<anyhow::Result<_>>()?;
        let routes = self
            .routes
            .iter()
            .map(|route| {
                let methods = route
                    .methods
                    .iter()
                    .map(|method| Method::from_bytes(method.as_bytes()))
                    .collect::<Result<_, _>>()
                    .with_context(|| format!("route {}: methods", route.path))?;
                Ok(Route {
                    methods,
                    path: route.path.clone(),
                    upstream: route.upstream.clone(),
                    max_body_bytes: route.max_body_bytes,
                    endpoint_response_timeout: route.endpoint_response_timeout,
                })
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(ProxyConfig {
            bind_address: self.bind_address,
            tls_config: fhevm_sig_proxy::TlsConfig {
                cert_path: self.tls_config.cert_path.clone(),
                key_path: self.tls_config.key_path.clone(),
            },
            ethereum_chain_id: self.ethereum_chain_id,
            protocol_config_address: self.protocol_config_contract.address,
            signers: SignerSource::ProtocolConfig {
                rpc_url: self.ethereum_rpc_url.clone(),
                refresh_interval: self.signer_refresh_interval,
            },
            audience: self.coprocessor_signer_address,
            error_body: crate::error::error_body,
            // prf-service keeps per-connector slots, so it needs the caller's identity: with the
            // signed header it verifies that identity itself instead of trusting one the proxy sets.
            forward_authorization: true,
            routes,
            upstreams,
            endpoint_connect_timeout: self.endpoint_connect_timeout,
            endpoint_healthcheck_frequency: self.endpoint_healthcheck_frequency,
            endpoint_idle_timeout: self.endpoint_idle_timeout,
            request_read_timeout: self.request_read_timeout,
            shutdown_grace_period: self.shutdown_grace_period,
            service_name: self.service_name.clone(),
            healthcheck_timeout: self.healthcheck_timeout,
        })
    }
}

/// Deserializes a humantime `Duration`, e.g. `"3s"`.
fn deserialize_duration<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    humantime::parse_duration(&String::deserialize(d)?).map_err(serde::de::Error::custom)
}

fn deserialize_level<'de, D>(d: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn default_bind_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8443)
}

fn default_signer_refresh_interval() -> Duration {
    Duration::from_secs(60)
}

fn default_endpoint_connect_timeout() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_healthcheck_frequency() -> Duration {
    Duration::from_secs(3)
}

fn default_endpoint_idle_timeout() -> Duration {
    // Short on purpose: a Kubernetes ClusterIP Service picks the target pod once per TCP
    // connection, so a short idle timeout lets new or restarted pods receive requests.
    Duration::from_secs(10)
}

fn default_healthcheck_path() -> String {
    "/healthz".to_owned()
}

fn default_request_read_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_shutdown_grace_period() -> Duration {
    Duration::from_secs(60)
}

fn default_service_name() -> String {
    "coprocessor-proxy".to_owned()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn example_config_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/proxy.toml")
    }

    fn example_with(from: &str, to: &str) -> anyhow::Result<ProxyConfig> {
        let example = std::fs::read_to_string(example_config_path()).unwrap();
        assert!(example.contains(from), "{from}");
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "coprocessor-proxy-{}-{}.toml",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, example.replace(from, to)).unwrap();
        let config = Config::from_env_and_file(Some(&path)).and_then(|c| c.proxy_config());
        std::fs::remove_file(path).ok();
        let config = config?;
        config.validate()?;
        Ok(config)
    }

    #[test]
    fn the_example_config_is_a_valid_prf_proxy() {
        let config = Config::from_env_and_file(Some(&example_config_path()))
            .unwrap()
            .proxy_config()
            .unwrap();
        config.validate().unwrap();
        assert_eq!(config.routes.len(), 1);
        assert_eq!(config.routes[0].methods, [Method::GET, Method::HEAD]);
        assert_eq!(config.routes[0].upstream, "prf-service");
        assert_eq!(config.routes[0].max_body_bytes, 0);
        assert_eq!(
            config.signers,
            SignerSource::ProtocolConfig {
                rpc_url: "http://localhost:8545".parse().unwrap(),
                refresh_interval: Duration::from_secs(60),
            }
        );
        assert!(config.forward_authorization);
        assert_eq!(
            config.upstreams[0].endpoint_addresses[0],
            "prf-service:8080"
        );
    }

    #[test]
    fn an_invalid_example_fails() {
        for (from, to) in [
            ("{prf}", "{prf:u16}"),
            ("upstream = \"prf-service\"", "upstream = \"other\""),
            ("[\"prf-service:8080\"]", "[]"),
            ("[\"prf-service:8080\"]", "[\"prf service\"]"),
            ("[\"GET\", \"HEAD\"]", "[\"G T\"]"),
            (
                "signer_refresh_interval = \"60s\"",
                "signer_refresh_interval = \"0s\"",
            ),
            ("max_body_bytes = 0", "max_body_bytes = 0\nname = \"prf\""),
        ] {
            assert!(example_with(from, to).is_err(), "{to}");
        }
    }

    #[test]
    fn a_missing_required_field_fails() {
        assert!(example_with("ethereum_chain_id = 11155111\n", "").is_err());
    }
}
