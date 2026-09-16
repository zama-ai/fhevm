use crate::core::Config;
use actix_web::http::StatusCode;
use alloy::transports::http::reqwest::{self, Url};
use anyhow::{Context, anyhow};
use connector_utils::monitoring::health::Healthcheck;
use kms_connector_api::VERSION_ROUTE;
use pingora::lb::{Backend, LoadBalancer, health_check::HttpHealthCheck, selection::RoundRobin};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

/// The struct used to monitor the state of the `Proxy`.
#[derive(Clone)]
pub struct State {
    proxy_version_url: Url,
    endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
    healthcheck_timeout: Duration,
    /// The client used to reach the proxy's own TLS listener through the loopback interface.
    loopback_client: reqwest::Client,
}

impl State {
    pub fn new(
        bind_address: SocketAddr,
        endpoint_balancer: Arc<LoadBalancer<RoundRobin>>,
        healthcheck_timeout: Duration,
    ) -> anyhow::Result<Self> {
        // The proxy's certificate is issued for its public hostname, not for the loopback
        // address used here, so its verification is disabled for this client only.
        let loopback_client = reqwest::Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .build()
            .context("Failed to build the healthcheck HTTP client")?;

        Ok(Self {
            proxy_version_url: version_url(bind_address),
            endpoint_balancer,
            healthcheck_timeout,
            loopback_client,
        })
    }

    /// Runs the load-balancer's healthcheck against every endpoint.
    ///
    /// Splits them into the healthy and unhealthy ones.
    async fn endpoints_healthcheck(&self) -> (Vec<String>, Vec<String>) {
        let backends = self.endpoint_balancer.backends();
        backends.run_health_check(true).await;
        let all = backends.get_backend();
        let (healthy, unhealthy) = all.iter().partition(|b| backends.ready(b));
        let addresses =
            |backends: Vec<&Backend>| backends.into_iter().map(|b| b.addr.to_string()).collect();
        (addresses(healthy), addresses(unhealthy))
    }

    /// Performs the healthcheck of the proxy's TLS listener by querying the `v1/version` route
    /// through the loopback interface.
    ///
    /// No credentials are sent, so the proxy is expected to answer `401`: what is checked here is
    /// that the listener accepts TLS connections and answers HTTP requests, not the routing.
    async fn tls_listener_healthcheck(&self) -> Result<(), String> {
        let url = &self.proxy_version_url;
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

impl Healthcheck for State {
    async fn healthcheck(&self) -> actix_web::HttpResponse {
        let (tls_listener_result, (healthy_endpoints, unhealthy_endpoints)) = tokio::join!(
            self.tls_listener_healthcheck(),
            self.endpoints_healthcheck()
        );

        let mut errors = vec![];
        let tls_listener_reachable = tls_listener_result.map_err(|e| errors.push(e)).is_ok();
        if !unhealthy_endpoints.is_empty() {
            errors.push(format!(
                "Unhealthy endpoints: {}",
                unhealthy_endpoints.join(", ")
            ));
        }
        if healthy_endpoints.is_empty() {
            errors.push("No healthy endpoint available".to_string());
        }

        let healthy = tls_listener_reachable && !healthy_endpoints.is_empty();
        let status_code = if healthy {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        let status = HealthStatus {
            healthy,
            tls_listener_reachable,
            healthy_endpoints,
            unhealthy_endpoints,
            details: errors.join("; "),
        };

        actix_web::HttpResponse::build(status_code).json(status)
    }

    fn service_name() -> &'static str {
        "kms-connector-proxy"
    }
}

/// Serializable representation of `Proxy`'s health status.
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    /// Overall health of the service: the TLS listener answers and at least one endpoint is
    /// healthy.
    pub healthy: bool,
    /// Whether the public TLS listener accepts connections and answers requests.
    pub tls_listener_reachable: bool,
    /// The endpoints currently receiving requests.
    pub healthy_endpoints: Vec<String>,
    /// The endpoints that failed their healthcheck. Non-empty is degraded, but not unhealthy.
    pub unhealthy_endpoints: Vec<String>,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}

/// The healthcheck run against each endpoint: a `GET v1/version` must be answered with `200`.
pub fn endpoint_health_check(config: &Config) -> anyhow::Result<HttpHealthCheck> {
    // The endpoints answer whatever the `Host` header says, so it is left empty
    let mut health_check = HttpHealthCheck::new("", false);
    let version_uri = VERSION_ROUTE
        .parse()
        .map_err(|e| anyhow!("Failed to build the endpoint healthcheck request: {e}"))?;
    health_check.req.set_uri(version_uri);
    health_check.peer_template.options.connection_timeout = Some(config.endpoint_connect_timeout);
    health_check.peer_template.options.read_timeout = Some(config.healthcheck_timeout);
    Ok(health_check)
}

/// Builds the loopback URL of the `v1/version` route from the bind address of the TLS listener.
fn version_url(bind_address: SocketAddr) -> Url {
    let ip = match bind_address.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    let loopback = SocketAddr::new(ip, bind_address.port());
    Url::parse(&format!("https://{loopback}{VERSION_ROUTE}"))
        .expect("loopback socket address is a valid URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_url_uses_loopback_for_unspecified_ip() {
        assert_eq!(
            version_url("0.0.0.0:8443".parse().unwrap()).as_str(),
            format!("https://127.0.0.1:8443{VERSION_ROUTE}")
        );
        assert_eq!(
            version_url("[::]:8443".parse().unwrap()).as_str(),
            format!("https://[::1]:8443{VERSION_ROUTE}")
        );
        assert_eq!(
            version_url("10.0.0.3:9443".parse().unwrap()).as_str(),
            format!("https://10.0.0.3:9443{VERSION_ROUTE}")
        );
    }
}
