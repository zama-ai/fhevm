use alloy::providers::Provider;
use sqlx::{Pool, Postgres};
use std::time::Duration;

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
