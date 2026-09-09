use actix_web::http::StatusCode;
use alloy::transports::http::reqwest::{self, Url};
use connector_utils::monitoring::health::{Healthcheck, database_healthcheck};
use kms_connector_api::VERSION_ROUTE;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::sync::Semaphore;

/// The struct used to monitor the state of the `Endpoint`.
#[derive(Clone)]
pub struct State {
    db_pool: Pool<Postgres>,
    endpoint_version_url: Url,
    in_flight_limiter: Arc<Semaphore>,
    max_in_flight_decryptions: usize,
    healthcheck_timeout: Duration,
}

impl State {
    pub fn new(
        db_pool: Pool<Postgres>,
        http_endpoint: SocketAddr,
        in_flight_limiter: Arc<Semaphore>,
        max_in_flight_decryptions: usize,
        healthcheck_timeout: Duration,
    ) -> Self {
        Self {
            db_pool,
            endpoint_version_url: version_url(http_endpoint),
            in_flight_limiter,
            max_in_flight_decryptions,
            healthcheck_timeout,
        }
    }

    /// Returns the number of decryption requests currently held in-flight by this replica.
    fn in_flight_decryptions(&self) -> usize {
        self.max_in_flight_decryptions
            .saturating_sub(self.in_flight_limiter.available_permits())
    }
}

impl Healthcheck for State {
    async fn healthcheck(&self) -> actix_web::HttpResponse {
        let (db_res, http_res) = tokio::join!(
            database_healthcheck(&self.db_pool, self.healthcheck_timeout),
            http_server_healthcheck(&self.endpoint_version_url, self.healthcheck_timeout),
        );

        let mut errors = vec![];
        let database_connected = db_res.map_err(|e| errors.push(e)).is_ok();
        let http_server_reachable = http_res.map_err(|e| errors.push(e)).is_ok();

        let (status_code, healthy) = if errors.is_empty() {
            (StatusCode::OK, true)
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, false)
        };

        let status = HealthStatus {
            healthy,
            database_connected,
            http_server_reachable,
            in_flight_decryptions: self.in_flight_decryptions(),
            max_in_flight_decryptions: self.max_in_flight_decryptions,
            details: errors.join("; "),
        };

        actix_web::HttpResponse::build(status_code).json(status)
    }

    fn service_name() -> &'static str {
        "kms-connector-endpoint"
    }
}

/// Serializable representation of `Endpoint`'s health status.
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    /// Overall health of the service.
    pub healthy: bool,
    /// Database connection status.
    pub database_connected: bool,
    /// Whether the public `v1` HTTP server answers requests.
    pub http_server_reachable: bool,
    /// Number of decryption requests currently held in-flight by this replica (informational).
    pub in_flight_decryptions: usize,
    /// Maximum number of in-flight decryption requests of this replica (informational).
    pub max_in_flight_decryptions: usize,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}

/// Performs the healthcheck of the public HTTP server by querying its `v1/version` route.
async fn http_server_healthcheck(url: &Url, timeout: Duration) -> Result<(), String> {
    match tokio::time::timeout(timeout, reqwest::get(url.clone())).await {
        Ok(Ok(response)) if response.status().is_success() => Ok(()),
        Ok(Ok(response)) => Err(format!(
            "HTTP server answered {} to {url}",
            response.status()
        )),
        Ok(Err(e)) => Err(format!("HTTP server connection failed: {e}")),
        Err(e) => Err(format!("HTTP server connection timed out: {e}")),
    }
}

/// Builds the loopback URL of the `v1/version` route from the bind address of the HTTP server.
fn version_url(http_endpoint: SocketAddr) -> Url {
    let ip = match http_endpoint.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    let loopback = SocketAddr::new(ip, http_endpoint.port());
    Url::parse(&format!("http://{loopback}{VERSION_ROUTE}"))
        .expect("loopback socket address is a valid URL")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_url_uses_loopback_for_unspecified_ip() {
        assert_eq!(
            version_url("0.0.0.0:8080".parse().unwrap()).as_str(),
            format!("http://127.0.0.1:8080{VERSION_ROUTE}")
        );
        assert_eq!(
            version_url("[::]:8080".parse().unwrap()).as_str(),
            format!("http://[::1]:8080{VERSION_ROUTE}")
        );
        assert_eq!(
            version_url("10.0.0.3:9000".parse().unwrap()).as_str(),
            format!("http://10.0.0.3:9000{VERSION_ROUTE}")
        );
    }
}
