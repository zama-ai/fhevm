use crate::config::settings::HttpConfig;
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::gateway::arbitrum::transaction::throttler::{GatewayTxTask, ThrottlingSender};
use crate::http::endpoints::{
    health_handler, liveness_handler,
    v1::handlers::{
        InputProofHandler as InputProofHandlerV1, KeyUrlHandler as KeyUrlHandlerV1,
        PublicDecryptHandler as PublicDecryptHandlerV1, UserDecryptHandler as UserDecryptHandlerV1,
    },
    v2::handlers::{
        InputProofHandler as InputProofHandlerV2, KeyUrlHandler as KeyUrlHandlerV2,
        PublicDecryptHandler as PublicDecryptHandlerV2, UserDecryptHandler as UserDecryptHandlerV2,
    },
    version_handler,
};
use crate::http::{openapi_middleware, with_rate_limiting};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::Repositories;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

async fn wait_for_ready(addr: SocketAddr) -> anyhow::Result<()> {
    const MAX_RETRIES: u32 = 10;
    let url = format!("http://{}/liveness", addr);
    for _ in 0..MAX_RETRIES {
        if reqwest::get(&url)
            .await
            .is_ok_and(|r| r.status().is_success())
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    Err(anyhow::anyhow!("HTTP server failed to start"))
}

pub async fn run_http_server<D>(
    config: &HttpConfig,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    repositories: Arc<Repositories>,
    user_decrypt_shares_threshold: u16,
    tx_throttler: ThrottlingSender<GatewayTxTask>,
) -> SocketAddr
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    let http_endpoint: SocketAddr = config
        .endpoint
        .as_ref()
        .expect("HTTP endpoint must be configured")
        .parse()
        .expect("Invalid http-endpoint address");
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    // Initialize v1 handlers
    let input_proof_handler_v1 = Arc::new(InputProofHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    let user_decrypt_handler_v1 = Arc::new(UserDecryptHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.user_decrypt.clone(),
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    let public_decrypt_handler_v1 = Arc::new(PublicDecryptHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    // Initialize v2 handlers
    let input_proof_handler_v2 = Arc::new(InputProofHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    let user_decrypt_handler_v2 = Arc::new(UserDecryptHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.user_decrypt.clone(),
        user_decrypt_shares_threshold,
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    let public_decrypt_handler_v2 = Arc::new(PublicDecryptHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
        config.api_retry_after_seconds,
        tx_throttler.clone(),
    ));

    // Clone orchestrator for health endpoint before using it
    let orchestrator_for_health = orchestrator.clone();

    // Create KeyUrlHandlers - they self-register with orchestrator
    let keyurl_handler_v1 = KeyUrlHandlerV1::new(orchestrator.clone());
    let keyurl_handler_v2 = KeyUrlHandlerV2::new(orchestrator.clone());

    // Create the router by merging all handler routers
    let app = Router::new()
        // Health and info endpoints
        .route("/liveness", get(liveness_handler))
        .route(
            "/healthz",
            get(move || async move { health_handler(orchestrator_for_health.clone()).await }),
        )
        .route("/version", get(version_handler))
        // Merge handler routers with rate limiting applied to POST endpoints
        .merge(with_rate_limiting(
            Router::new()
                // v1 routes
                .merge(input_proof_handler_v1.routes())
                .merge(public_decrypt_handler_v1.routes())
                .merge(user_decrypt_handler_v1.routes())
                // v2 routes
                .merge(input_proof_handler_v2.routes())
                .merge(public_decrypt_handler_v2.routes())
                .merge(user_decrypt_handler_v2.routes()),
            &config.rate_limit_post_endpoints,
        ))
        // Add keyurl routes (no rate limiting for GET)
        .merge(keyurl_handler_v1.routes())
        .merge(keyurl_handler_v2.routes())
        // Add OpenAPI documentation
        .merge(openapi_middleware());

    // Setup TCP listener and start server
    let listener = tokio::net::TcpListener::bind(http_endpoint).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    println!("Server listening on http://{actual_addr}");

    // Use orchestrator's task manager instead of raw tokio::spawn
    let addr_for_readiness = actual_addr;
    orchestrator
        .spawn_task_and_wait_ready(
            "http_server_axum",
            async move {
                axum::serve(listener, app).await.unwrap();
            },
            async move {
                // Wait for HTTP server to be ready with actual health check
                wait_for_ready(addr_for_readiness).await
            },
        )
        .await
        .unwrap();

    actual_addr
}
