use crate::config::settings::HttpConfig;
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::gateway::throttlers::BouncerThrottlers;
use crate::http::admin::AdminConfigRegistry;
use crate::http::endpoints::{
    admin, health_handler, liveness_handler,
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
use crate::http::retry_after::RetryAfterState;
use crate::http::{openapi_middleware, with_rate_limiting};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::Repositories;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

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
    bouncer_throttlers: BouncerThrottlers,
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

    // Create RetryAfterState directly from config
    let retry_after_state = Arc::new(RetryAfterState::new(&config.retry_after));

    // Create AdminConfigRegistry for TPS throttling (separate from retry-after)
    let admin_registry = Arc::new(AdminConfigRegistry::new(
        HashMap::new(),
        bouncer_throttlers.input_proof_throttler_control_tx.clone(),
        bouncer_throttlers.user_decrypt_throttler_control_tx.clone(),
        bouncer_throttlers
            .public_decrypt_throttler_control_tx
            .clone(),
    ));

    // Initialize v1 handlers
    let input_proof_handler_v1 = Arc::new(InputProofHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .input_proof_tx_throttler
            .clone(),
    ));

    let user_decrypt_handler_v1 = Arc::new(UserDecryptHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.user_decrypt.clone(),
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .user_decrypt_tx_throttler
            .clone(),
        bouncer_throttlers
            .readiness_throttling_senders
            .user_decrypt_readiness_throttler
            .clone(),
    ));

    let public_decrypt_handler_v1 = Arc::new(PublicDecryptHandlerV1::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .public_decrypt_tx_throttler
            .clone(),
        bouncer_throttlers
            .readiness_throttling_senders
            .public_decrypt_readiness_throttler
            .clone(),
    ));

    // Initialize v2 handlers with dynamic retry-after state
    let input_proof_handler_v2 = Arc::new(InputProofHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .input_proof_tx_throttler
            .clone(),
        retry_after_state.clone(),
    ));

    let user_decrypt_handler_v2 = Arc::new(UserDecryptHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.user_decrypt.clone(),
        user_decrypt_shares_threshold,
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .user_decrypt_tx_throttler
            .clone(),
        bouncer_throttlers
            .readiness_throttling_senders
            .user_decrypt_readiness_throttler
            .clone(),
        retry_after_state.clone(),
    ));

    let public_decrypt_handler_v2 = Arc::new(PublicDecryptHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
        config.api_retry_after_seconds,
        bouncer_throttlers
            .tx_throttlers
            .public_decrypt_tx_throttler
            .clone(),
        bouncer_throttlers
            .readiness_throttling_senders
            .public_decrypt_readiness_throttler
            .clone(),
        retry_after_state.clone(),
    ));

    // Clone orchestrator for health endpoint before using it
    let orchestrator_for_health = orchestrator.clone();

    // Create KeyUrlHandlers - they self-register with orchestrator
    let keyurl_handler_v1 = KeyUrlHandlerV1::new(orchestrator.clone());
    let keyurl_handler_v2 = KeyUrlHandlerV2::new(orchestrator.clone());

    // Create the router by merging all handler routers
    let mut app = Router::new()
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

    // Admin endpoints configuration
    // When enabled, pass both registry (for TPS) and retry-after state
    // When disabled, None is passed so handler returns 403
    let (admin_registry_option, retry_after_option): (
        Option<Arc<AdminConfigRegistry>>,
        Option<Arc<RetryAfterState>>,
    ) = if config.enable_admin_endpoint {
        info!("Admin endpoints enabled at /admin/config");
        (Some(admin_registry), Some(retry_after_state))
    } else {
        info!("Admin endpoints disabled");
        (None, None)
    };

    app = app
        .route("/admin/config", post(admin::update_config))
        .route("/admin/config", get(admin::get_config))
        .layer(Extension(admin_registry_option))
        .layer(Extension(retry_after_option));

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
