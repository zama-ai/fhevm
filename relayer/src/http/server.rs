use crate::config::settings::Settings;
use crate::core::event::{ApiCategory, ApiVersion};
use crate::gateway::throttlers::BouncerThrottlers;
use crate::host::{HostChainIdChecker, UserDecryptSignaturePreChecker};
use crate::http::admin::AdminConfigRegistry;
use crate::http::endpoints::{
    admin, health_handler, liveness_handler,
    v2::handlers::{
        InputProofHandler as InputProofHandlerV2, KeyUrlHandler as KeyUrlHandlerV2,
        PublicDecryptHandler as PublicDecryptHandlerV2, UserDecryptHandler as UserDecryptHandlerV2,
    },
    v3::handlers::UserDecryptHandler as UserDecryptHandlerV3,
    version_handler,
};
use crate::http::openapi_middleware;
use crate::http::retry_after::RetryAfterState;
use crate::http::utils::BounceChecker;
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

pub async fn run_http_server(
    settings: &Settings,
    orchestrator: Arc<Orchestrator>,
    repositories: Arc<Repositories>,
    bouncer_throttlers: BouncerThrottlers,
    host_chain_id_checker: Arc<HostChainIdChecker>,
    signature_prechecker: Arc<UserDecryptSignaturePreChecker>,
    keyurl_rx: tokio::sync::watch::Receiver<
        crate::http::endpoints::v2::types::keyurl::KeyUrlResponseJson,
    >,
) -> SocketAddr {
    // Read the HTTP-relevant config off the shared Settings so the server keeps a single
    // config parameter rather than one per scalar it needs.
    let http = &settings.http;
    let user_decrypt_shares_threshold = settings.gateway.contracts.user_decrypt_shares_threshold;

    let http_endpoint: SocketAddr = http
        .endpoint
        .as_ref()
        .expect("HTTP endpoint must be configured")
        .parse()
        .expect("Invalid http-endpoint address");
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    // Create RetryAfterState directly from config
    let retry_after_state = Arc::new(RetryAfterState::new(&http.retry_after));

    // Create AdminConfigRegistry for TPS throttling (separate from retry-after)
    let admin_registry = Arc::new(AdminConfigRegistry::new(
        HashMap::new(),
        bouncer_throttlers.input_proof_throttler_control_tx.clone(),
        bouncer_throttlers.user_decrypt_throttler_control_tx.clone(),
        bouncer_throttlers
            .public_decrypt_throttler_control_tx
            .clone(),
    ));

    // Initialize v2 handlers with dynamic retry-after state
    let input_proof_handler_v2 = Arc::new(InputProofHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
        http.api_retry_after_seconds,
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
        BounceChecker::new(
            bouncer_throttlers
                .tx_throttlers
                .user_decrypt_tx_throttler
                .clone(),
            bouncer_throttlers
                .readiness_throttling_senders
                .user_decrypt_readiness_throttler
                .clone(),
            http.api_retry_after_seconds,
        ),
        retry_after_state.clone(),
        host_chain_id_checker.clone(),
    ));

    let public_decrypt_handler_v2 = Arc::new(PublicDecryptHandlerV2::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
        BounceChecker::new(
            bouncer_throttlers
                .tx_throttlers
                .public_decrypt_tx_throttler
                .clone(),
            bouncer_throttlers
                .readiness_throttling_senders
                .public_decrypt_readiness_throttler
                .clone(),
            http.api_retry_after_seconds,
        ),
        retry_after_state.clone(),
        host_chain_id_checker.clone(),
    ));

    // v3 (unified EIP-712) user-decrypt handler. Shares orchestrator + repo + queue
    // state with v2; GET delegates to the v2 handler since the response
    // schema is unchanged. Uses a distinct API version tag so dashboards
    // separate v2 and v3 traffic.
    let api_version_v3 = ApiVersion::new(ApiCategory::PRODUCTION, 2);
    let user_decrypt_handler_v3 = Arc::new(UserDecryptHandlerV3::new(
        orchestrator.clone(),
        api_version_v3,
        repositories.user_decrypt.clone(),
        BounceChecker::new(
            bouncer_throttlers
                .tx_throttlers
                .user_decrypt_tx_throttler
                .clone(),
            bouncer_throttlers
                .readiness_throttling_senders
                .user_decrypt_readiness_throttler
                .clone(),
            http.api_retry_after_seconds,
        ),
        retry_after_state.clone(),
        host_chain_id_checker.clone(),
        signature_prechecker,
        user_decrypt_handler_v2.clone(),
    ));

    // Clone orchestrator for health endpoint before using it
    let orchestrator_for_health = orchestrator.clone();

    // Create KeyUrlHandler - reads the latest chain-sourced value from the poller's watch channel
    let keyurl_handler_v2 = KeyUrlHandlerV2::new(keyurl_rx);

    // Create the router by merging all handler routers
    let mut app = Router::new()
        // Health and info endpoints
        .route("/liveness", get(liveness_handler))
        .route(
            "/healthz",
            get(move || async move { health_handler(orchestrator_for_health.clone()).await }),
        )
        .route("/version", get(version_handler))
        // Merge handler routers
        .merge(input_proof_handler_v2.routes())
        .merge(public_decrypt_handler_v2.routes())
        .merge(user_decrypt_handler_v2.routes())
        .merge(user_decrypt_handler_v3.routes())
        // Add keyurl routes (no rate limiting for GET)
        .merge(keyurl_handler_v2.routes())
        // Add OpenAPI documentation
        .merge(openapi_middleware());

    // Admin endpoints configuration
    // When enabled, pass both registry (for TPS) and retry-after state
    // When disabled, None is passed so handler returns 403
    let (admin_registry_option, retry_after_option): (
        Option<Arc<AdminConfigRegistry>>,
        Option<Arc<RetryAfterState>>,
    ) = if http.enable_admin_endpoint {
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
