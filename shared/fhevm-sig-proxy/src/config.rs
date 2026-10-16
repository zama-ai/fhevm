use crate::{Route, defaults, routing::RouteTable};
use alloy::primitives::Address;
use anyhow::ensure;
use http::uri::Authority;
use serde::{Deserialize, Deserializer, de::Error};
use std::{
    collections::HashSet, fmt::Display, net::SocketAddr, path::PathBuf, str::FromStr,
    time::Duration,
};
use url::Url;

/// What the [`crate::Proxy`] runs with. A deployment deserializes it from its configuration
/// sources, durations written in humantime (`"3s"`), or builds it from its own settings.
#[derive(Clone, Debug, Deserialize)]
pub struct ProxyConfig {
    /// The public bind address of the proxy.
    pub bind_address: SocketAddr,
    /// The TLS configuration of the proxy.
    pub tls_config: TlsConfig,

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    pub ethereum_chain_id: u64,
    /// The `ProtocolConfig` contract on Ethereum, the `verifyingContract` of the domain.
    pub protocol_config_address: Address,
    /// Where the addresses allowed to sign requests come from.
    pub signers: SignerSource,
    /// The `audience` the accepted headers are signed for: this deployment's address.
    pub audience: Address,
    /// Whether the `Authorization` header is forwarded to the endpoints, for endpoints that read
    /// the `FhevmSig` themselves. Otherwise, and by default, it is removed.
    #[serde(default)]
    pub forward_authorization: bool,

    /// The routes forwarded to the endpoints; any other request is rejected.
    pub routes: Vec<Route>,
    /// The pools of endpoints the routes are forwarded to.
    pub upstreams: Vec<Upstream>,

    /// The timeout to establish a TCP connection to an endpoint.
    #[serde(
        with = "humantime_serde",
        default = "defaults::endpoint_connect_timeout"
    )]
    pub endpoint_connect_timeout: Duration,
    /// How often the endpoints are probed in the background to filter which ones receive requests.
    #[serde(
        with = "humantime_serde",
        default = "defaults::endpoint_healthcheck_frequency"
    )]
    pub endpoint_healthcheck_frequency: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    #[serde(with = "humantime_serde", default = "defaults::endpoint_idle_timeout")]
    pub endpoint_idle_timeout: Duration,

    /// How long the proxy waits for the client to send request bytes before dropping it.
    #[serde(with = "humantime_serde", default = "defaults::request_read_timeout")]
    pub request_read_timeout: Duration,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    #[serde(with = "humantime_serde", default = "defaults::shutdown_grace_period")]
    pub shutdown_grace_period: Duration,

    /// The name of the proxy's Pingora service.
    pub service_name: String,
    /// The timeout to perform each external service connection healthcheck.
    #[serde(with = "humantime_serde", default = "defaults::healthcheck_timeout")]
    pub healthcheck_timeout: Duration,
}

impl ProxyConfig {
    /// Checks the configuration as [`crate::Proxy::from_config`] does, without resolving the
    /// endpoint addresses.
    pub fn validate(&self) -> anyhow::Result<()> {
        self.checked().map(drop)
    }

    /// Checks the configuration and returns its parsed routes.
    pub(crate) fn checked(&self) -> anyhow::Result<RouteTable> {
        let routes = RouteTable::new(&self.routes, &self.upstreams)?;
        match &self.signers {
            SignerSource::Static(signers) => ensure!(!signers.is_empty(), "no signer configured"),
            SignerSource::ProtocolConfig {
                refresh_interval, ..
            } => ensure!(
                !refresh_interval.is_zero(),
                "signer refresh interval is zero"
            ),
        }
        Ok(routes)
    }
}

/// Where the addresses allowed to sign requests come from: `static = [..]` or
/// `protocol_config = { rpc_url, refresh_interval }`.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SignerSource {
    /// A fixed set.
    Static(HashSet<Address>),
    /// The KMS tx-senders listed by the `ProtocolConfig` contract at
    /// [`ProxyConfig::protocol_config_address`], read through `rpc_url` every `refresh_interval`.
    /// The proxy refuses every request, and is not ready, until the first read succeeds.
    ProtocolConfig {
        rpc_url: Url,
        #[serde(
            with = "humantime_serde",
            default = "defaults::signer_refresh_interval"
        )]
        refresh_interval: Duration,
    },
}

/// The TLS material used by the proxy to terminate TLS.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TlsConfig {
    /// The path of the PEM certificate chain.
    pub cert_path: PathBuf,
    /// The path of the PEM private key.
    pub key_path: PathBuf,
}

/// A pool of interchangeable endpoints, load-balanced round-robin and probed in the background.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Upstream {
    /// The name the routes refer to the upstream by.
    pub name: String,
    /// The `host:port` addresses of the endpoints, reached over plain HTTP.
    #[serde(deserialize_with = "parsed")]
    pub endpoint_addresses: Vec<Authority>,
    /// The path the endpoints are probed on; a healthy endpoint answers `200`.
    pub healthcheck_path: String,
}

/// Deserializes a list of strings, each parsed into a `T`, e.g. a [`http::Method`].
pub(crate) fn parsed<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    Vec::<String>::deserialize(deserializer)?
        .iter()
        .map(|value| {
            value
                .parse()
                .map_err(|error| D::Error::custom(format!("{value:?}: {error}")))
        })
        .collect()
}
