//! actix-web wiring of the `v1` routes.

pub mod decrypt;
pub mod public_decrypt;
pub mod user_decrypt;
pub mod version;

use crate::core::{Config, Waiters};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    dev::Server,
    error::{InternalError, JsonPayloadError},
    web::{self, Data, JsonConfig},
};
use anyhow::anyhow;
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
};
use sqlx::{Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

/// Shared state of the HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Pool<Postgres>,
    pub waiters: Arc<Waiters>,
    /// In-flight decryptions backpressure, sized by `Config::max_in_flight_decryptions`.
    pub in_flight_limiter: Arc<Semaphore>,
}

impl AppState {
    pub fn new(config: Arc<Config>, db_pool: Pool<Postgres>, waiters: Arc<Waiters>) -> Self {
        let in_flight_limiter = Arc::new(Semaphore::new(config.max_in_flight_decryptions));
        Self {
            config,
            db_pool,
            waiters,
            in_flight_limiter,
        }
    }
}

pub fn run_server(state: AppState, endpoint: SocketAddr) -> anyhow::Result<Server> {
    HttpServer::new(move || {
        App::new().configure(|cfg| {
            cfg.app_data(json_config(state.config.max_body_bytes))
                .app_data(Data::new(state.clone()))
                .route(VERSION_ROUTE, web::get().to(version::version))
                .route(
                    PUBLIC_DECRYPTION_ROUTE,
                    web::post().to(public_decrypt::public_decrypt),
                )
                .route(
                    USER_DECRYPTION_ROUTE,
                    web::post().to(user_decrypt::user_decrypt),
                );
        })
    })
    // Treat a client EOF as a disconnect, to release its in-flight permit.
    .h1_allow_half_closed(false)
    .bind(endpoint)
    .map_err(|e| anyhow!("Failed to bind HTTP server to {endpoint}: {e}"))
    .map(|server| server.run())
}

/// JSON extractor configuration: body size limit and `400 malformed` error body.
fn json_config(max_body_bytes: usize) -> JsonConfig {
    JsonConfig::default()
        .limit(max_body_bytes)
        .error_handler(json_error_handler)
}

fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    // The id cannot be derived from a body that failed to deserialize.
    let body = ErrorResponse::new(ErrorCode::Malformed, err.to_string(), None);
    let response = HttpResponse::BadRequest().json(body);
    InternalError::from_response(err, response).into()
}
