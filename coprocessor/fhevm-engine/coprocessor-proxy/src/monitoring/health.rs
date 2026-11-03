use crate::kms_tx_senders::Loaded;
use alloy::transports::http::reqwest::{self, Url};
use anyhow::Context;
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    time::Duration,
};

/// A path no route matches, requested by the TLS listener healthcheck.
const SELF_CHECK_PATH: &str = "/";

/// The struct used to monitor the health of the `Proxy`.
#[derive(Clone)]
pub struct State {
    self_check_url: Url,
    kms_tx_senders_loaded: Loaded,
    healthcheck_timeout: Duration,
    /// The client used to reach the proxy's own TLS listener through the loopback interface.
    loopback_client: reqwest::Client,
}

impl State {
    pub fn new(
        bind_address: SocketAddr,
        kms_tx_senders_loaded: Loaded,
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
            kms_tx_senders_loaded,
            healthcheck_timeout,
            loopback_client,
        })
    }

    /// Performs the healthcheck of the proxy's TLS listener by querying [`SELF_CHECK_PATH`]
    /// through the loopback interface.
    ///
    /// The proxy is expected to answer `404`: what is checked here is that the listener accepts
    /// TLS connections and answers HTTP requests, not the routing.
    async fn tls_listener_healthcheck(&self) -> Result<(), String> {
        let url = &self.self_check_url;
        let request = self.loopback_client.get(url.clone()).send();
        match tokio::time::timeout(self.healthcheck_timeout, request).await {
            Ok(Ok(response)) if response.status() == reqwest::StatusCode::NOT_FOUND => Ok(()),
            Ok(Ok(response)) => Err(format!(
                "TLS listener answered {} to {url}",
                response.status()
            )),
            Ok(Err(e)) => Err(format!("TLS listener connection failed: {e}")),
            Err(e) => Err(format!("TLS listener connection timed out: {e}")),
        }
    }
}

impl HealthCheckService for State {
    /// Ready when the proxy can serve: the TLS listener answers and the KMS tx-senders have been
    /// read once. prf-service is not checked: its Service routes to its ready pods, and without
    /// one the proxy answers `502 upstream_transient`, retryable.
    async fn health_check(&self) -> HealthStatus {
        let tls_listener_result = self.tls_listener_healthcheck().await;
        let kms_tx_senders_loaded = self.kms_tx_senders_loaded.is_loaded();

        let mut status = HealthStatus::default();
        status.set_custom_check("tls_listener", tls_listener_result.is_ok(), true);
        status.set_custom_check("kms_tx_senders", kms_tx_senders_loaded, true);
        if let Err(e) = tls_listener_result {
            status.add_error_details(e);
        }
        if !kms_tx_senders_loaded {
            status.add_error_details("KMS tx-senders not read yet".to_string());
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
}
