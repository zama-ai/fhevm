//! The checks behind a deployment's healthcheck: the proxy's own TLS listener, and the endpoints
//! as observed by the load-balancer's background probes.

use crate::ProxyConfig;
use alloy::transports::http::reqwest::{self, Url};
use anyhow::{Context, anyhow};
use pingora::lb::{Backend, LoadBalancer, health_check::HttpHealthCheck, selection::RoundRobin};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

/// The struct used to monitor the health of the `Proxy`.
#[derive(Clone)]
pub struct State {
    self_check_url: Url,
    /// Each upstream's name and load-balancer.
    upstreams: Vec<(String, Arc<LoadBalancer<RoundRobin>>)>,
    healthcheck_timeout: Duration,
    /// The client used to reach the proxy's own TLS listener through the loopback interface.
    loopback_client: reqwest::Client,
}

impl State {
    pub fn new(
        bind_address: SocketAddr,
        self_check_path: &str,
        upstreams: Vec<(String, Arc<LoadBalancer<RoundRobin>>)>,
        healthcheck_timeout: Duration,
    ) -> anyhow::Result<Self> {
        // The proxy's certificate is issued for its public hostname, not for the loopback
        // address used here, so its verification is disabled for this client only.
        let loopback_client = reqwest::Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .build()
            .context("Failed to build the healthcheck HTTP client")?;

        Ok(Self {
            self_check_url: self_check_url(bind_address, self_check_path),
            upstreams,
            healthcheck_timeout,
            loopback_client,
        })
    }

    /// Splits each upstream's endpoints into the healthy and unhealthy ones, as last observed by
    /// the load-balancers' background healthchecks.
    pub fn endpoints_health(&self) -> EndpointsHealth {
        let addresses =
            |backends: Vec<&Backend>| backends.into_iter().map(|b| b.addr.to_string()).collect();
        EndpointsHealth(
            self.upstreams
                .iter()
                .map(|(name, balancer)| {
                    let backends = balancer.backends();
                    let all = backends.get_backend();
                    let (healthy, unhealthy) = all.iter().partition(|b| backends.ready(b));
                    UpstreamHealth {
                        name: name.clone(),
                        healthy: addresses(healthy),
                        unhealthy: addresses(unhealthy),
                    }
                })
                .collect(),
        )
    }

    /// Performs the healthcheck of the proxy's TLS listener by querying a route through the
    /// loopback interface.
    ///
    /// No credentials are sent, so the proxy is expected to answer `401`: what is checked here is
    /// that the listener accepts TLS connections and answers HTTP requests, not the routing.
    pub async fn tls_listener_healthcheck(&self) -> Result<(), String> {
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

/// The endpoints' health, read once from every upstream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointsHealth(pub Vec<UpstreamHealth>);

/// One upstream's endpoints, split by their last background healthcheck.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpstreamHealth {
    pub name: String,
    pub healthy: Vec<String>,
    pub unhealthy: Vec<String>,
}

impl EndpointsHealth {
    /// Whether every upstream has a healthy endpoint, so that every route can be forwarded.
    pub fn serving(&self) -> bool {
        self.0.iter().all(|upstream| !upstream.healthy.is_empty())
    }

    /// The healthy endpoints of all upstreams.
    pub fn healthy_endpoints(&self) -> Vec<String> {
        self.0.iter().flat_map(|u| u.healthy.clone()).collect()
    }

    /// The unhealthy endpoints of all upstreams.
    pub fn unhealthy_endpoints(&self) -> Vec<String> {
        self.0.iter().flat_map(|u| u.unhealthy.clone()).collect()
    }
}

/// The healthcheck run against each endpoint: a `GET` of `healthcheck_path` must be answered with
/// `200`.
pub fn endpoint_health_check(
    config: &ProxyConfig,
    healthcheck_path: &str,
) -> anyhow::Result<HttpHealthCheck> {
    // The endpoints answer whatever the `Host` header says, so it is left empty
    let mut health_check = HttpHealthCheck::new("", false);
    let uri = healthcheck_path
        .parse()
        .map_err(|e| anyhow!("Failed to build the endpoint healthcheck request: {e}"))?;
    health_check.req.set_uri(uri);
    // The connection phase is capped by `healthcheck_timeout` so a large `endpoint_connect_timeout`
    // cannot make a probe outlast the healthcheck period.
    let connect_timeout = config
        .endpoint_connect_timeout
        .min(config.healthcheck_timeout);
    let options = &mut health_check.peer_template.options;
    options.connection_timeout = Some(connect_timeout);
    options.total_connection_timeout = Some(connect_timeout);
    options.read_timeout = Some(config.healthcheck_timeout);
    Ok(health_check)
}

/// Builds the loopback URL of `path` from the bind address of the TLS listener.
fn self_check_url(bind_address: SocketAddr, path: &str) -> Url {
    let ip = match bind_address.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    let loopback = SocketAddr::new(ip, bind_address.port());
    Url::parse(&format!("https://{loopback}{path}"))
        .expect("loopback socket address is a valid URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_check_url_uses_loopback_for_unspecified_ip() {
        assert_eq!(
            self_check_url("0.0.0.0:8443".parse().unwrap(), "/version").as_str(),
            "https://127.0.0.1:8443/version"
        );
        assert_eq!(
            self_check_url("[::]:8443".parse().unwrap(), "/version").as_str(),
            "https://[::1]:8443/version"
        );
        assert_eq!(
            self_check_url("10.0.0.3:9443".parse().unwrap(), "/version").as_str(),
            "https://10.0.0.3:9443/version"
        );
    }

    #[test]
    fn serving_needs_a_healthy_endpoint_in_every_upstream() {
        let upstream = |name: &str, healthy: &[&str]| UpstreamHealth {
            name: name.to_owned(),
            healthy: healthy.iter().map(|a| a.to_string()).collect(),
            unhealthy: vec!["10.0.0.9:80".to_owned()],
        };
        let one_down = EndpointsHealth(vec![upstream("a", &["10.0.0.1:80"]), upstream("b", &[])]);
        assert!(!one_down.serving());
        assert_eq!(one_down.healthy_endpoints(), ["10.0.0.1:80"]);
        assert_eq!(
            one_down.unhealthy_endpoints(),
            ["10.0.0.9:80", "10.0.0.9:80"]
        );
        let both_up = EndpointsHealth(vec![
            upstream("a", &["10.0.0.1:80"]),
            upstream("b", &["10.0.0.2:80"]),
        ]);
        assert!(both_up.serving());
    }
}
