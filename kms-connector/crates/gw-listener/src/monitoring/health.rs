use actix_web::http::StatusCode;
use alloy::providers::Provider;
use connector_utils::monitoring::health::{Healthcheck, database_healthcheck, gateway_healthcheck};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::Duration;

#[derive(Clone)]
/// The struct used to monitor the state of the `GatewayListener`.
pub struct State<P> {
    db_pool: Pool<Postgres>,
    provider: P,
    healthcheck_timeout: Duration,
}

impl<P: Provider> State<P> {
    pub fn new(db_pool: Pool<Postgres>, provider: P, healthcheck_timeout: Duration) -> Self {
        Self {
            db_pool,
            provider,
            healthcheck_timeout,
        }
    }
}

impl<P: Provider> Healthcheck for State<P> {
    async fn healthcheck(&self) -> actix_web::HttpResponse {
        let mut errors = vec![];
        let database_connected =
            database_healthcheck(&self.db_pool, self.healthcheck_timeout, &mut errors).await;
        let gateway_connected =
            gateway_healthcheck(&self.provider, self.healthcheck_timeout, &mut errors).await;

        let (status_code, healthy) = if errors.is_empty() {
            (StatusCode::OK, true)
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, false)
        };

        let status = HealthStatus {
            healthy,
            database_connected,
            gateway_connected,
            details: errors.join("; "),
        };

        actix_web::HttpResponse::build(status_code).json(status)
    }

    fn service_name() -> &'static str {
        "kms-connector-gw-listener"
    }
}

/// Serializable representation of `GatewayListener`'s health status.
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    /// Overall health of the service.
    pub healthy: bool,
    /// Database connection status.
    pub database_connected: bool,
    /// Gateway provider connection status.
    pub gateway_connected: bool,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}
