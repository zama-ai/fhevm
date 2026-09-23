use crate::monitoring::otlp::default_dispatcher;
use alloy::{
    providers::Provider,
    transports::http::reqwest::{self, StatusCode, Url},
};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};
use std::{
    env,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str::FromStr,
    time::Duration,
};
use tracing::{error, info};

/// Interface to perform the healthchecks of the different services of the KMS Connector.
pub trait Healthcheck {
    /// Returns an `HttpResponse` containing the health status of the service.
    fn healthcheck(&self) -> impl Future<Output = actix_web::HttpResponse>;

    /// Returns the name of the service.
    fn service_name() -> &'static str;
}

pub fn default_healthcheck_timeout() -> Duration {
    Duration::from_secs(3)
}

/// Performs the database healthcheck.
///
/// Returns `Ok(())` on success, or an `Err` containing a formatted error message on failure.
pub async fn database_healthcheck(
    db_pool: &Pool<Postgres>,
    timeout: Duration,
) -> Result<(), String> {
    match tokio::time::timeout(timeout, db_pool.acquire()).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("Database connection failed: {e}")),
        Err(e) => Err(format!("Database connection timed out: {e}")),
    }
}

/// Performs the healthcheck of a blockchain RPC node.
///
/// Uses `eth_blockNumber` for this.
///
/// Returns `Ok(())` on success, or an `Err` containing a formatted error message on failure.
pub async fn rpc_node_healthcheck<P: Provider>(
    provider: P,
    timeout: Duration,
    chain_name: &str,
) -> Result<(), String> {
    match tokio::time::timeout(timeout, provider.get_block_number()).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("{chain_name} connection failed: {e}")),
        Err(e) => Err(format!("{chain_name} connection timed out: {e}")),
    }
}

pub async fn query_healthcheck_endpoint<S: Debug + DeserializeOwned>(
    url: Option<Url>,
) -> anyhow::Result<()> {
    let _dispatcher_guard = tracing::dispatcher::set_default(&default_dispatcher());
    query_healthcheck_endpoint_inner::<S>(url)
        .await
        .inspect_err(|e| error!("{e}"))
}

async fn query_healthcheck_endpoint_inner<S: Debug + DeserializeOwned>(
    url: Option<Url>,
) -> anyhow::Result<()> {
    let healthz_url = match url {
        Some(url) => url,
        None => monitoring_url_from_env()?,
    };
    let healthcheck_response = reqwest::get(healthz_url).await?;
    let status_code = healthcheck_response.status();
    let app_state = healthcheck_response.json::<S>().await?;
    if status_code == StatusCode::OK {
        info!("Healthcheck success: {app_state:?}");
        Ok(())
    } else {
        Err(anyhow!("Healthcheck failed: {app_state:?}"))
    }
}

fn monitoring_url_from_env() -> anyhow::Result<Url> {
    let monitoring_bind_address = env::var("KMS_CONNECTOR_MONITORING_ENDPOINT")
        .map_err(|e| anyhow!("Failed to access KMS_CONNECTOR_MONITORING_ENDPOINT: {e}"))?
        .parse()
        .map_err(|e| anyhow!("Failed to parse monitoring endpoint: {e}"))?;
    monitoring_url(monitoring_bind_address)
}

fn monitoring_url(mut bind_address: SocketAddr) -> anyhow::Result<Url> {
    if bind_address.ip().is_unspecified() {
        bind_address.set_ip(match bind_address.ip() {
            IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
        });
    }
    Url::from_str(&format!("http://{bind_address}/healthz"))
        .map_err(|e| anyhow!("Failed to build healthcheck url for {bind_address}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitoring_url_replaces_wildcard_with_loopback() {
        let url = monitoring_url("0.0.0.0:9100".parse().unwrap()).unwrap();
        assert_eq!(url.as_str(), "http://127.0.0.1:9100/healthz");

        let url = monitoring_url("[::]:9100".parse().unwrap()).unwrap();
        assert_eq!(url.as_str(), "http://[::1]:9100/healthz");
    }

    #[test]
    fn monitoring_url_keeps_explicit_address() {
        let url = monitoring_url("10.0.0.2:9101".parse().unwrap()).unwrap();
        assert_eq!(url.as_str(), "http://10.0.0.2:9101/healthz");
    }
}
