use actix_web::http::StatusCode;
use alloy::providers::Provider;
use connector_utils::monitoring::health::{
    Healthcheck, database_healthcheck, rpc_node_healthcheck,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::Duration;

#[derive(Clone)]
/// The struct used to monitor the state of the `GatewayListener`.
pub struct State<GP, EP> {
    db_pool: Pool<Postgres>,
    gateway_provider: GP,
    ethereum_provider: EP,
    healthcheck_timeout: Duration,
}

impl<GP: Provider, EP: Provider> State<GP, EP> {
    pub fn new(
        db_pool: Pool<Postgres>,
        gateway_provider: GP,
        ethereum_provider: EP,
        healthcheck_timeout: Duration,
    ) -> Self {
        Self {
            db_pool,
            gateway_provider,
            ethereum_provider,
            healthcheck_timeout,
        }
    }
}

impl<GP: Provider, EP: Provider> Healthcheck for State<GP, EP> {
    async fn healthcheck(&self) -> actix_web::HttpResponse {
        let mut errors = vec![];
        let database_connected =
            database_healthcheck(&self.db_pool, self.healthcheck_timeout, &mut errors).await;
        let gateway_connected = rpc_node_healthcheck(
            &self.gateway_provider,
            self.healthcheck_timeout,
            &mut errors,
        )
        .await;
        let ethereum_connected = rpc_node_healthcheck(
            &self.ethereum_provider,
            self.healthcheck_timeout,
            &mut errors,
        )
        .await;

        let (status_code, healthy) = if errors.is_empty() {
            (StatusCode::OK, true)
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, false)
        };

        let status = HealthStatus {
            healthy,
            database_connected,
            gateway_connected,
            ethereum_connected,
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
    /// Ethereum provider connection status.
    pub ethereum_connected: bool,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}
