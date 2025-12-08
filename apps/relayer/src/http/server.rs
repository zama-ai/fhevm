use crate::config::settings::HttpConfig;
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::http::endpoints::{
    health_handler, liveness_handler,
    v1::handlers::{InputProofHandler, KeyUrlHandler, PublicDecryptHandler, UserDecryptHandler},
    version_handler,
};
use crate::http::{openapi_middleware, with_rate_limiting};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::Repositories;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run_http_server<D>(
    config: &HttpConfig,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    repositories: Arc<Repositories>,
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

    // Initialize handlers
    let input_proof_handler = Arc::new(InputProofHandler::new(
        orchestrator.clone(),
        api_version,
        repositories.input_proof.clone(),
    ));

    let user_decrypt_handler = Arc::new(UserDecryptHandler::new(
        orchestrator.clone(),
        api_version,
        repositories.user_decrypt.clone(),
    ));

    let public_decrypt_handler = Arc::new(PublicDecryptHandler::new(
        orchestrator.clone(),
        api_version,
        repositories.public_decrypt.clone(),
    ));

    // Clone orchestrator for health endpoint before using it
    let orchestrator_for_health = orchestrator.clone();

    // Create KeyUrlHandler - it self-registers with orchestrator
    let keyurl_handler = KeyUrlHandler::new(orchestrator);

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
                .merge(input_proof_handler.routes())
                .merge(public_decrypt_handler.routes())
                .merge(user_decrypt_handler.routes()),
            &config.rate_limit_post_endpoints,
        ))
        // Add keyurl route (no rate limiting for GET)
        .merge(keyurl_handler.routes())
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
