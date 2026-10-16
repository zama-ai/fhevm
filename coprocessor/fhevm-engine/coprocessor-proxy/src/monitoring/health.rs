use crate::core::Config;
use alloy::transports::http::reqwest::{self, Url};
use anyhow::{Context, anyhow};
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use fhevm_sig_auth::protocol_config::Loaded;
use pingora::lb::{Backend, LoadBalancer, health_check::HttpHealthCheck, selection::RoundRobin};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

/// A path of a forwarded route, requested without credentials by the TLS listener healthcheck.
const SELF_CHECK_PATH: &str =
    "/v1/prf/0/labels/0000000000000000000000000000000000000000000000000000000000000000/1";
/// The route each endpoint answers `200` on while it can serve requests.
const ENDPOINT_HEALTH_PATH: &str = "/healthz";

/// The struct used to monitor the health of the `Proxy`.
#[derive(Clone)]
pub struct State {
    self_check_url: Url,
    endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
    kms_tx_senders: Loaded,
    healthcheck_timeout: Duration,
    /// The client used to reach the proxy's own TLS listener through the loopback interface.
    loopback_client: reqwest::Client,
}

impl State {
    pub fn new(
        bind_address: SocketAddr,
        endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
        kms_tx_senders: Loaded,
        healthcheck_timeout: Duration,
    ) -> anyhow::Result<Self> {
        // The proxy's certificate is issued for its public hostname, not for the loopback
        // address used here, so its verification is disabled for this client only.
        let loopback_client = reqwest::Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .build()
            .context("Failed to build the healthcheck HTTP client")?;

        Ok(Self {
            self_check_url: self_check_url(bind_address),
            endpoint_balancer,
            kms_tx_senders,
            healthcheck_timeout,
            loopback_client,
        })
    }

    /// Splits the endpoints into the healthy and unhealthy ones, as last observed by the
    /// load-balancer's background healthcheck.
    fn endpoints_health(&self) -> (Vec<String>, Vec<String>) {
        let backends = self.endpoint_balancer.backends();
        let all = backends.get_backend();
        let (healthy, unhealthy) = all.iter().partition(|b| backends.ready(b));
        let addresses =
            |backends: Vec<&Backend>| backends.into_iter().map(|b| b.addr.to_string()).collect();
        (addresses(healthy), addresses(unhealthy))
    }

    /// Performs the healthcheck of the proxy's TLS listener by querying a forwarded route through
    /// the loopback interface.
    ///
    /// No credentials are sent, so the proxy is expected to answer `401`: what is checked here is
    /// that the listener accepts TLS connections and answers HTTP requests, not the routing.
    async fn tls_listener_healthcheck(&self) -> Result<(), String> {
        let url = &self.self_check_url;
        let request = self.loopback_client.get(url.clone()).send();
        match tokio::time::timeout(self.healthcheck_timeout, request).await {
            Ok(Ok(response)) if response.status() == reqwest::StatusCode::UNAUTHORIZED => Ok(()),
            Ok(Ok(response)) => Err(format!(
                "TLS listener answered {} to unauthenticated {url}",
                response.status()
            )),
            Ok(Err(e)) => Err(format!("TLS listener connection failed: {e}")),
            Err(e) => Err(format!("TLS listener connection timed out: {e}")),
        }
    }
}

impl HealthCheckService for State {
    /// Healthy when the TLS listener answers, the KMS tx-senders have been read once and at
    /// least one endpoint is healthy.
    async fn health_check(&self) -> HealthStatus {
        let tls_listener_result = self.tls_listener_healthcheck().await;
        let (healthy_endpoints, unhealthy_endpoints) = self.endpoints_health();
        let kms_tx_senders_loaded = self.kms_tx_senders.is_loaded();

        let mut status = HealthStatus::default();
        status.set_custom_check("tls_listener", tls_listener_result.is_ok(), true);
        status.set_custom_check("kms_tx_senders", kms_tx_senders_loaded, true);
        status.set_custom_check("endpoints", !healthy_endpoints.is_empty(), true);
        if let Err(e) = tls_listener_result {
            status.add_error_details(e);
        }
        if !kms_tx_senders_loaded {
            status.add_error_details("KMS tx-senders not read yet".to_string());
        }
        if !unhealthy_endpoints.is_empty() {
            status.add_error_details(format!(
                "Unhealthy endpoints: {}",
                unhealthy_endpoints.join(", ")
            ));
        }
        if healthy_endpoints.is_empty() {
            status.add_error_details("No healthy endpoint available".to_string());
        }
        status
    }

    async fn is_alive(&self) -> bool {
        true
    }

    // `default_get_version` would name fhevm-engine-common, the crate it is compiled in.
    fn get_version(&self) -> Version {
        Version {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            build: option_env!("BUILD_ID").unwrap_or("unknown"),
        }
    }
}

/// The healthcheck run against each endpoint: a `GET /healthz` over HTTP/2, the protocol of the
/// proxied requests, must be answered with `200`.
pub fn endpoint_health_check(config: &Config) -> anyhow::Result<HttpHealthCheck> {
    // The endpoints answer whatever the `Host` header says, but HTTP/2 needs a non-empty
    // `:authority`.
    let mut health_check = HttpHealthCheck::new("localhost", false);
    let health_uri = ENDPOINT_HEALTH_PATH
        .parse()
        .map_err(|e| anyhow!("Failed to build the endpoint healthcheck request: {e}"))?;
    health_check.req.set_uri(health_uri);
    // The connection phase is capped by `healthcheck_timeout` so a large `endpoint_connect_timeout`
    // cannot make a probe outlast the healthcheck period.
    let connect_timeout = config
        .endpoint_connect_timeout
        .min(config.healthcheck_timeout);
    let options = &mut health_check.peer_template.options;
    options.connection_timeout = Some(connect_timeout);
    options.total_connection_timeout = Some(connect_timeout);
    options.read_timeout = Some(config.healthcheck_timeout);
    options.set_http_version(2, 2);
    Ok(health_check)
}

/// Builds the loopback URL of [`SELF_CHECK_PATH`] from the bind address of the TLS listener.
fn self_check_url(bind_address: SocketAddr) -> Url {
    let ip = match bind_address.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    let loopback = SocketAddr::new(ip, bind_address.port());
    Url::parse(&format!("https://{loopback}{SELF_CHECK_PATH}"))
        .expect("loopback socket address is a valid URL")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Route, match_route};
    use http::Method;

    #[test]
    fn test_self_check_url_uses_loopback_for_unspecified_ip() {
        assert_eq!(
            self_check_url("0.0.0.0:8443".parse().unwrap()).as_str(),
            format!("https://127.0.0.1:8443{SELF_CHECK_PATH}")
        );
        assert_eq!(
            self_check_url("[::]:8443".parse().unwrap()).as_str(),
            format!("https://[::1]:8443{SELF_CHECK_PATH}")
        );
        assert_eq!(
            self_check_url("10.0.0.3:9443".parse().unwrap()).as_str(),
            format!("https://10.0.0.3:9443{SELF_CHECK_PATH}")
        );
    }

    #[test]
    fn the_self_check_path_reaches_the_authentication() {
        assert_eq!(match_route(&Method::GET, SELF_CHECK_PATH), Ok(Route::Prf));
    }
}
