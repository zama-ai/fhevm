use crate::monitoring::otlp::default_dispatcher;
use alloy::{
    providers::Provider,
    transports::http::reqwest::{self, StatusCode, Url},
};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres};
use std::{env, fmt::Debug, str::FromStr, time::Duration};
use tracing::{error, info};

/// Interface to perform the healthchecks of the different services of the KMS Connector.
pub trait Healthcheck {
    /// Returns an `HttpResponse` containing the health status of the service.
    fn healthcheck(&self) -> impl Future<Output = actix_web::HttpResponse>;

    /// Returns the name of the service.
    fn service_name() -> &'static str;
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
    endpoint: Option<Url>,
) -> anyhow::Result<()> {
    let _dispatcher_guard = tracing::dispatcher::set_default(&default_dispatcher());
    query_healthcheck_endpoint_inner::<S>(endpoint)
        .await
        .inspect_err(|e| error!("{e}"))
}

async fn query_healthcheck_endpoint_inner<S: Debug + DeserializeOwned>(
    endpoint: Option<Url>,
) -> anyhow::Result<()> {
    let healthz_endpoint = match endpoint {
        Some(endpoint) => endpoint,
        None => monitoring_endpoint_from_env()?,
    };
    let healthcheck_response = reqwest::get(healthz_endpoint).await?;
    let status_code = healthcheck_response.status();
    let app_state = healthcheck_response.json::<S>().await?;
    if status_code == StatusCode::OK {
        info!("Healthcheck success: {app_state:?}");
        Ok(())
    } else {
        Err(anyhow!("Healthcheck failed: {app_state:?}"))
    }
}

fn monitoring_endpoint_from_env() -> anyhow::Result<Url> {
    let str_endpoint = env::var("KMS_CONNECTOR_MONITORING_ENDPOINT")
        .map_err(|e| anyhow!("Failed to access KMS_CONNECTOR_MONITORING_ENDPOINT: {e}"))?;
    Url::from_str(&str_endpoint)
        .map_err(|e| anyhow!("Failed to parse monitoring endpoint url {str_endpoint}, {e}"))
}
