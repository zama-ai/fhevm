use crate::{ErrorBody, ProxyError, Route};
use alloy::primitives::Address;
use http::uri::Authority;
use std::{collections::HashSet, net::SocketAddr, path::PathBuf, time::Duration};

/// What the [`crate::Proxy`] runs with. Each deployment loads it from its own configuration.
#[derive(Clone, Debug)]
pub struct ProxyConfig {
    /// The public bind address of the proxy.
    pub bind_address: SocketAddr,
    /// The TLS configuration of the proxy.
    pub tls_config: TlsConfig,

    /// The Ethereum chain id, part of the `FhevmSig` (RFC-038) EIP-712 domain.
    pub ethereum_chain_id: u64,
    /// The `ProtocolConfig` contract on Ethereum, the `verifyingContract` of the domain.
    pub protocol_config_address: Address,
    /// The addresses allowed to sign requests.
    pub allowed_senders: HashSet<Address>,
    /// The `audience` the accepted headers are signed for: this deployment's address.
    pub audience: Address,
    /// Renders the errors the proxy answers itself in the deployment's error body.
    pub error_body: fn(&ProxyError) -> ErrorBody,

    /// The routes forwarded to the endpoints; any other request is rejected.
    pub routes: Vec<Route>,
    /// A route the proxy answers `401` to without credentials, requested by its own TLS
    /// listener healthcheck.
    pub tls_self_check_path: String,

    /// The `host:port` addresses of the endpoints to forward requests to.
    pub endpoint_addresses: Vec<Authority>,
    /// The path the endpoints are probed on in the background; a healthy endpoint answers `200`.
    pub endpoint_healthcheck_path: String,
    /// The timeout to establish a TCP connection to an endpoint.
    pub endpoint_connect_timeout: Duration,
    /// How often the endpoints are probed in the background to filter which ones receive requests.
    pub endpoint_healthcheck_frequency: Duration,
    /// How long the proxy waits for an endpoint response before answering `502`.
    pub endpoint_response_timeout: Duration,
    /// How long an idle pooled endpoint connection is kept before being closed.
    pub endpoint_idle_timeout: Duration,

    /// The maximum accepted size of a request body.
    pub max_body_bytes: usize,
    /// How long the proxy waits for the client to send request bytes before dropping it.
    pub request_read_timeout: Duration,
    /// How long in-flight requests are given to complete after `SIGTERM`.
    pub shutdown_grace_period: Duration,

    /// The name of the proxy's Pingora service.
    pub service_name: String,
    /// The timeout to perform each external service connection healthcheck.
    pub healthcheck_timeout: Duration,
}

/// The TLS material used by the proxy to terminate TLS.
#[derive(Clone, Debug, PartialEq)]
pub struct TlsConfig {
    /// The path of the PEM certificate chain.
    pub cert_path: PathBuf,
    /// The path of the PEM private key.
    pub key_path: PathBuf,
}
