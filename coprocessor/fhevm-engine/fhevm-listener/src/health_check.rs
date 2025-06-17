use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use std::sync::Arc;

use alloy::providers::{ProviderBuilder, WsConnect};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::{error, info};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;



/// Represents the health status of the transaction sender service
#[derive(Clone, Debug, Serialize)]
pub struct Health {
    pub status_code: u16,
    pub status: &'static str,
    /// Any error is unhealthy
    pub healthy: bool,
    /// Database connection status
    pub database_connected: bool,
    /// Blockchain provider connection status
    pub blockchain_connected: bool,
    pub message: String,
    pub last_tick: u64,
}

const HEALTHY: &str = "healthy";
const UNHEALTHY: &str = "unhealthy";
const OK: u16 = StatusCode::OK.as_u16();
const INTERNAL_SERVER_ERROR: u16 = StatusCode::INTERNAL_SERVER_ERROR.as_u16();

impl Health {

    pub fn initial() -> Self {
        Self {
            database_connected: false,
            blockchain_connected: false,
            message: "Not connected".to_string(),
            status_code: OK,
            status: UNHEALTHY,
            healthy: false,
            last_tick: 0,
        }
    }

    pub fn connected(&mut self) {
        self.status = HEALTHY;
        self.message = "Only connected".to_string();
        self.status_code = OK;
        self.healthy = true;
        self.database_connected = true;
        self.blockchain_connected = true;
        self.tick();
    }

    pub fn reset(&mut self) {
        if self.last_tick == 0 {
            return;
        }
        self.status = HEALTHY;
        self.status_code = OK;
        self.healthy = true;
        self.database_connected = true;
        self.blockchain_connected = true;
        self.message = "".to_string();
    }

    pub fn tick(&mut self) {
        self.last_tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    pub fn unhealthy(&mut self) {
        self.healthy = false;
        self.status = UNHEALTHY;
        self.status_code = INTERNAL_SERVER_ERROR;
    }

    pub fn check_last_tick(&mut self) {
        if self.last_tick == 0 {
            // not connected yet
            return;
        }
        let timestamp_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let elapsed_time = timestamp_now - self.last_tick;
        if elapsed_time > 30 {
            self.message += &format!("Last tick is too old {elapsed_time}.\n");
            self.unhealthy();
        }
    }

    pub async fn check_database_connected(&mut self, database_url: &str) {
        let Ok(options) = database_url.parse::<PgConnectOptions>() else {
            self.message += &format!("Bad Database url.\n");
            self.database_connected = false;
            self.unhealthy();
            return;
        };
        let pool = PgPoolOptions::new().max_connections(1).connect_with(options.clone()).await;
        if let Err(_err) = pool {
            self.message += &format!("Database connection failed.\n");
            self.database_connected = false;
            self.unhealthy();
            return;
        };
        self.database_connected = true;
    }

    pub async fn check_blockchain_connected(&mut self, blockchain_url: &str) {
        let ws = WsConnect::new(blockchain_url);
        let provider = ProviderBuilder::new()
            .connect_ws(ws)
            .await;
        if let Err(_err) = provider {
            self.message += &format!("Blockchain connection failed.\n");
            self.blockchain_connected = false;
            self.unhealthy();
            return;
        };
        self.blockchain_connected = true;
    }
}

#[derive(Clone)]
pub struct HealthStateContent {
    pub status: Health,
    database_url: String,
    blockchain_url: String,
}

impl HealthStateContent {
    pub fn tick(&mut self) {
        self.status.tick();
    }
}

pub type HealthState = Arc<RwLock<HealthStateContent>>;

#[derive(Clone)]
pub struct HealthCheck {
    pub health_state: HealthState,
    port: u16,
    pub cancel_token: CancellationToken,
}


impl HealthCheck {
    pub fn new(
        port: u16,
        cancel_token: CancellationToken,
        database_url: &String,
        blockchain_url: &String,
    ) -> Self {
        let health_state = HealthStateContent {
                status: Health::initial(),
                database_url: database_url.clone(),
                blockchain_url: blockchain_url.clone(),
        };
        Self {
            health_state: Arc::new(RwLock::new(health_state)),
            port,
            cancel_token,
        }
    }

    pub async fn start_http_server(&self) -> anyhow::Result<()> {
        let cancel_token = self.cancel_token.clone();
        let shutdown = async move {
            cancel_token.cancelled().await;
        };
        let health_state = self.health_state.clone();
        let port = self.port;
        let app = Router::new()
            .route("/healthz", get(health_handler))
            .route("/liveness", get(liveness_handler))
            .with_state(health_state);

        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        let listener = TcpListener::bind(addr).await?;
        let server =
            axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown);
        info!("HealthCheck server started on {}", addr);
        if let Err(err) = server.await {
            error!("HTTP server error: {}", err);
            return Err(anyhow::anyhow!("HTTP server error: {}", err));
        }
        Ok(())
    }

    pub async fn connected(&self) {
        self.health_state.write().await.status.connected()
    }

}

async fn health_handler(
    State(state_health): State<Arc<RwLock<HealthStateContent>>>,
) -> impl IntoResponse {
    let mut health_state = state_health.read().await.clone();
    health_state.status.check_last_tick();
    health_state.status.check_database_connected(&health_state.database_url).await;
    health_state.status.check_blockchain_connected(&health_state.blockchain_url).await;
    let health = health_state.status;
    let result = (
        StatusCode::from_u16(health.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(serde_json::json!({
            "status_code": health.status_code,
            "status": health.status,
            "details": *Json(health),
        })),
    );
    state_health.write().await.status.reset();
    result
}

async fn liveness_handler(
    State(_): State<Arc<RwLock<HealthStateContent>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status_code": 200,
            "status": "alive"
        })),
    )
}
