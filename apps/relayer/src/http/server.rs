use crate::config::settings::{KeyUrl, RateLimitConfig};
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::http::endpoints::{
    health_handler, liveness_handler,
    v1::handlers::{keyurl, InputProofHandler, PublicDecryptHandler, UserDecryptHandler},
    version_handler,
};
use crate::http::{openapi_middleware, with_rate_limiting, HealthChecker};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::{
    input_proof_repo::InputProofRepository, public_decrypt_repo::PublicDecryptRepository,
    user_decrypt_repo::UserDecryptRepository,
};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;

// There are TODO commands to remove argument bloat, after that this can be removed.
#[allow(clippy::too_many_arguments)]
pub async fn run_http_server<D>(
    http_endpoint: SocketAddr,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    key_url: KeyUrl,
    health_checker: Arc<HealthChecker>,
    rate_limit_on_post_endpoints: RateLimitConfig,
    input_proof_repo: Arc<InputProofRepository>,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    user_decrypt_repo: Arc<UserDecryptRepository>,
) -> SocketAddr
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    // Initialize handlers
    let input_proof_handler = Arc::new(InputProofHandler::new(
        orchestrator.clone(),
        api_version,
        input_proof_repo,
    ));

    let user_decrypt_handler = Arc::new(UserDecryptHandler::new(
        Arc::clone(&orchestrator),
        api_version,
        user_decrypt_repo,
    ));

    let public_decrypt_handler = Arc::new(PublicDecryptHandler::new(
        orchestrator,
        api_version,
        public_decrypt_repo,
    ));

    // Create the router by merging all handler routers
    let app = Router::new()
        // Health and info endpoints
        .route("/liveness", get(liveness_handler))
        .route(
            "/healthz",
            get(move || async move { health_handler(health_checker).await }),
        )
        .route("/version", get(version_handler))
        // Merge handler routers with rate limiting applied to POST endpoints
        .merge(with_rate_limiting(
            Router::new()
                .merge(input_proof_handler.routes())
                .merge(public_decrypt_handler.routes())
                .merge(user_decrypt_handler.routes()),
            &rate_limit_on_post_endpoints,
        ))
        // Add keyurl route (no rate limiting for GET)
        .merge(keyurl::routes(key_url))
        // Add OpenAPI documentation
        .merge(openapi_middleware());

    // Setup TCP listener and start server
    let listener = tokio::net::TcpListener::bind(http_endpoint).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    println!("Server listening on http://{actual_addr}");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    actual_addr
}
