use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::Eip712Domain;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::api::handlers::{
    get_ciphertext_handler, health_handler_v2, verify_input_handler, ApiState,
};
use crate::aws_s3::AwsS3Interface;
use crate::gw_listener::GatewayListener;
use crate::HealthStatus;
use alloy::{network::Ethereum, providers::Provider};

#[derive(Serialize)]
struct HealthResponse {
    status_code: String,
    status: String,
    database_connected: bool,
    blockchain_connected: bool,
    details: Option<String>,
}

impl From<HealthStatus> for HealthResponse {
    fn from(status: HealthStatus) -> Self {
        Self {
            status_code: if status.healthy { "200" } else { "503" }.to_string(),
            status: if status.healthy {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            database_connected: status.database_connected,
            blockchain_connected: status.blockchain_connected,
            details: status.details,
        }
    }
}

pub struct HttpServer<
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + Send + Sync + 'static,
> {
    listener: Arc<GatewayListener<P, A>>,
    port: u16,
    cancel_token: CancellationToken,
    db_pool: Option<Pool<Postgres>>,
    signer: Option<Arc<PrivateKeySigner>>,
    signer_address: Option<Address>,
    input_eip712_domain: Option<Eip712Domain>,
    ciphertext_eip712_domain: Option<Eip712Domain>,
}

impl<
        P: Provider<Ethereum> + Clone + Send + Sync + 'static,
        A: AwsS3Interface + Clone + Send + Sync + 'static,
    > HttpServer<P, A>
{
    pub fn new(
        listener: Arc<GatewayListener<P, A>>,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            listener,
            port,
            cancel_token,
            db_pool: None,
            signer: None,
            signer_address: None,
            input_eip712_domain: None,
            ciphertext_eip712_domain: None,
        }
    }

    pub fn with_v2_api(
        mut self,
        db_pool: Pool<Postgres>,
        signer: PrivateKeySigner,
        signer_address: Address,
        input_eip712_domain: Eip712Domain,
        ciphertext_eip712_domain: Eip712Domain,
    ) -> Self {
        self.db_pool = Some(db_pool);
        self.signer = Some(Arc::new(signer));
        self.signer_address = Some(signer_address);
        self.input_eip712_domain = Some(input_eip712_domain);
        self.ciphertext_eip712_domain = Some(ciphertext_eip712_domain);
        self
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let mut app = Router::new()
            .route("/healthz", get(health_handler))
            .route("/liveness", get(liveness_handler))
            .with_state(self.listener.clone());

        if let (
            Some(db_pool),
            Some(signer_address),
            Some(signer),
            Some(input_eip712_domain),
            Some(ciphertext_eip712_domain),
        ) = (
            &self.db_pool,
            &self.signer_address,
            &self.signer,
            &self.input_eip712_domain,
            &self.ciphertext_eip712_domain,
        )
        {
            let api_state = Arc::new(ApiState {
                listener: self.listener.clone(),
                db_pool: db_pool.clone(),
                signer: signer.clone(),
                input_eip712_domain: input_eip712_domain.clone(),
                ciphertext_eip712_domain: ciphertext_eip712_domain.clone(),
                signer_address: *signer_address,
                start_time: Instant::now(),
            });

            let v2_routes = Router::new()
                .route("/v1/verify-input", post(verify_input_handler::<P, A>))
                .route("/v1/ciphertext/:handle", get(get_ciphertext_handler::<P, A>))
                .route("/v1/health", get(health_handler_v2::<P, A>))
                .with_state(api_state);

            app = app.merge(v2_routes);
            info!("V2 API endpoints enabled: /v1/verify-input, /v1/ciphertext/:handle, /v1/health");
        }

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!(address = %addr, "Starting HTTP server");

        let cancel_token = self.cancel_token.clone();
        let shutdown = async move {
            cancel_token.cancelled().await;
        };

        let listener = TcpListener::bind(addr).await?;
        let server =
            axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown);

        if let Err(err) = server.await {
            error!(error = %err, "HTTP server error");
            return Err(anyhow::anyhow!("HTTP server error: {}", err));
        }

        Ok(())
    }
}

// Health handler returns appropriate HTTP status code based on health
async fn health_handler<
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + 'static,
>(
    State(listener): State<Arc<GatewayListener<P, A>>>,
) -> impl IntoResponse {
    let status = listener.health_check().await;
    let http_status = if status.healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    // Return HTTP status code that matches the health status
    (http_status, Json(HealthResponse::from(status)))
}

async fn liveness_handler<
    P: Provider<Ethereum> + Clone + Send + Sync + 'static,
    A: AwsS3Interface + Clone + 'static,
>(
    State(_listener): State<Arc<GatewayListener<P, A>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status_code": "200",
            "status": "alive"
        })),
    )
}
