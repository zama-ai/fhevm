use actix_web::http::StatusCode;
use alloy::providers::Provider;
use connector_utils::monitoring::health::{Healthcheck, database_healthcheck, gateway_healthcheck};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::{
    task::JoinSet,
    time::{error::Elapsed, timeout},
};
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

        let mut kms_core_connected = true;
        let kms_healtcheck_results = self.kms_health_client.check(self.healthcheck_timeout).await;
        for (i, kms_shard_connected) in kms_healtcheck_results.iter().enumerate() {
            match kms_shard_connected {
                Ok(Ok(_)) => (),
                Ok(Err(e)) => {
                    errors.push(format!("KMS Core connection #{i} failed: {e}"));
                    kms_core_connected = false;
                }
                Err(e) => {
                    errors.push(format!("KMS Core connection #{i} timed out: {e}"));
                    kms_core_connected = false;
                }
            }
        }

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
    /// KMS Core connections status.
    pub kms_core_connected: bool,
    /// Details about any issues encountered during healthcheck.
    pub details: String,
}

#[derive(Clone)]
/// KMS Core GRPC healthcheck client wrapper.
pub struct KmsHealthClient {
    /// The inner GRPC clients used to perform the healthcheck of the KMS Core connections.
    inners: Vec<HealthClient<Channel>>,
}

impl KmsHealthClient {
    /// Connects the GRPC clients used to perform the healthcheck of the KMS Core connections.
    pub async fn connect(endpoints: &[String]) -> anyhow::Result<Self> {
        let mut inners = vec![];
        for endpoint in endpoints {
            let channel = Channel::from_shared(endpoint.clone())?.connect().await?;
            inners.push(HealthClient::new(channel));
        }
        Ok(Self { inners })
    }

    /// Performs the healthcheck of the KMS Core connections.
    async fn check(
        &self,
        healthcheck_timeout: Duration,
    ) -> Vec<Result<Result<tonic::Response<HealthCheckResponse>, Status>, Elapsed>> {
        let service =
            kms_grpc::kms_service::v1::core_service_endpoint_server::SERVICE_NAME.to_string();
        let request = HealthCheckRequest { service };

        let mut tasks = JoinSet::new();
        for inner in &self.inners {
            let mut client = inner.clone();
            let request = request.clone();
            tasks.spawn(async move { timeout(healthcheck_timeout, client.check(request)).await });
        }

        tasks.join_all().await
    }
}
