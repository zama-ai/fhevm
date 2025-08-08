use alloy::{
    providers::Provider,
    transports::http::reqwest::{self, StatusCode},
};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};
use std::{fmt::Debug, net::SocketAddr, time::Duration};
use tracing::{error, info};

use crate::monitoring::otlp::default_dispatcher;

/// Interface to perform the healthchecks of the different services of the KMS Connector.
pub trait Healthcheck {
    /// Returns an `HttpResponse` containing the health status of the service.
    fn healthcheck(&self) -> impl Future<Output = actix_web::HttpResponse>;
}

pub fn default_healthcheck_timeout_secs() -> u64 {
    3 // 3 seconds
}

/// Performs the database healthcheck.
///
/// Stores the potential error in the `errors` vector.
pub async fn database_healthcheck(
    db_pool: &Pool<Postgres>,
    timeout: Duration,
    errors: &mut Vec<String>,
) -> bool {
    match tokio::time::timeout(timeout, db_pool.acquire()).await {
        Ok(Ok(_)) => true,
        Ok(Err(e)) => {
            errors.push(format!("Database connection failed: {e}"));
            false
        }
        Err(e) => {
            errors.push(format!("Database connection timed out: {e}"));
            false
        }
    }
}

/// Performs the Gateway healthcheck.
///
/// Stores the potential error in the `errors` vector.
pub async fn gateway_healthcheck<P: Provider>(
    provider: P,
    timeout: Duration,
    errors: &mut Vec<String>,
) -> bool {
    match tokio::time::timeout(timeout, provider.get_block_number()).await {
        Ok(Ok(_)) => true,
        Ok(Err(e)) => {
            errors.push(format!("Gateway connection failed: {e}"));
            false
        }
        Err(e) => {
            errors.push(format!("Gateway connection timed out: {e}"));
            false
        }
    }
}

pub async fn query_healthcheck_endpoint<S: Debug + DeserializeOwned>(
    endpoint: SocketAddr,
) -> anyhow::Result<()> {
    let _dispatcher_guard = tracing::dispatcher::set_default(&default_dispatcher());
    query_healthcheck_endpoint_inner::<S>(endpoint)
        .await
        .inspect_err(|e| error!("{e}"))
}

async fn query_healthcheck_endpoint_inner<S: Debug + DeserializeOwned>(
    endpoint: SocketAddr,
) -> anyhow::Result<()> {
    let healthcheck_response = reqwest::get(format!("http://{}/healthz", endpoint)).await?;
    let status_code = healthcheck_response.status();
    let app_state = healthcheck_response.json::<S>().await?;
    if status_code == StatusCode::OK {
        info!("Healthcheck success: {app_state:?}");
        Ok(())
    } else {
        Err(anyhow!("Healthcheck success: {app_state:?}"))
    }
}
