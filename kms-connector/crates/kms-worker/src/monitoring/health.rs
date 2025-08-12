use actix_web::http::StatusCode;
use alloy::providers::Provider;
use connector_utils::monitoring::health::{Healthcheck, database_healthcheck, gateway_healthcheck};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::time::timeout;
use tonic::{Status, transport::Channel};
use tonic_health::pb::{HealthCheckRequest, HealthCheckResponse, health_client::HealthClient};

#[derive(Clone)]
/// The struct used to monitor the state of the `KmsWorker`.
pub struct State<P> {
    db_pool: Pool<Postgres>,
    provider: P,
    kms_health_client: KmsHealthClient,
    healthcheck_timeout: Duration,
}

impl<P: Provider> State<P> {
    pub fn new(
        db_pool: Pool<Postgres>,
        provider: P,
        kms_health_client: KmsHealthClient,
        healthcheck_timeout: Duration,
    ) -> Self {
        Self {
            db_pool,
            provider,
            kms_health_client,
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

        let kms_core_connected =
            match timeout(self.healthcheck_timeout, self.kms_health_client.check()).await {
                Ok(Ok(_)) => true,
                Ok(Err(e)) => {
                    errors.push(format!("KMS Core connection failed: {e}"));
                    false
                }
                Err(e) => {
                    errors.push(format!("KMS Core connection timed out: {e}"));
                    false
                }
            };

        let (status_code, healthy) = if errors.is_empty() {
            (StatusCode::OK, true)
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, false)
        };

        let status = HealthStatus {
            healthy,
            database_connected,
            gateway_connected,
            kms_core_connected,
            details: errors.join("; "),
        };

        actix_web::HttpResponse::build(status_code).json(status)
    }

    fn service_name() -> &'static str {
        "kms-connector-kms-worker"
    }
}

/// Serializable representation of `KmsWorker`'s health status.
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    /// Overall health of the service.
    pub healthy: bool,
    /// Database connection status.
    pub database_connected: bool,
    /// Gateway provider connection status.
    pub gateway_connected: bool,
    /// KMS Core connection status.
    pub kms_core_connected: bool,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}

#[derive(Clone)]
/// KMS Core GRPC healthcheck client wrapper.
pub struct KmsHealthClient {
    /// The inner GRPC client used to perform the healthcheck of the KMS Core connection.
    inner: HealthClient<Channel>,
}

impl KmsHealthClient {
    /// Connects the GRPC client used to perform the healthcheck of the KMS Core connection.
    pub async fn connect(endpoint: &str) -> anyhow::Result<Self> {
        let channel = Channel::from_shared(endpoint.to_string())?
            .connect()
            .await?;
        let inner = HealthClient::new(channel);
        Ok(Self { inner })
    }

    /// Performs the healthcheck of the KMS Core connection.
    async fn check(&self) -> std::result::Result<tonic::Response<HealthCheckResponse>, Status> {
        let service =
            kms_grpc::kms_service::v1::core_service_endpoint_server::SERVICE_NAME.to_string();
        let request = HealthCheckRequest { service };
        self.inner.clone().check(request).await
    }
}
