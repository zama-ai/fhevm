use crate::config::settings::{KeyUrl, RateLimitConfig};
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::http::health::{health_handler, liveness_handler, version_handler, HealthChecker};
use crate::http::input_http_listener::{
    InputProofErrorResponseJson, InputProofHandler, InputProofRequestJson, InputProofResponseJson,
};
use crate::http::keyurl_http_listener::KeyUrlResponseJson;
use crate::http::public_decrypt_http_listener::{
    PublicDecryptErrorResponseJson, PublicDecryptHandler, PublicDecryptRequestJson,
    PublicDecryptResponseJson,
};
use crate::http::userdecrypt_http_listener::{
    UserDecryptErrorResponseJson, UserDecryptHandler, UserDecryptRequestJson,
    UserDecryptResponseJson,
};
use crate::http::AppResponse;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::store::sql::repositories::{
    input_proof_repo::InputProofRepository, public_decrypt_repo::PublicDecryptRepository,
    user_decrypt_repo::UserDecryptRepository,
};
use chrono::Utc;
use rand::Rng;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use std::str::FromStr;

use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorError,
    GovernorLayer,
};
use utoipa::{OpenApi, ToSchema};
use utoipa_redoc::{Redoc, Servable};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HTTPApiVersion {
    V1,
}

/// Represents the error response from the endpoint for input proof.
#[derive(Debug, Serialize, Clone, Deserialize, ToSchema)]
pub struct VersionErrorResponseJson {
    pub message: String,
}

impl FromStr for HTTPApiVersion {
    type Err = ();

    fn from_str(input: &str) -> Result<HTTPApiVersion, Self::Err> {
        match input {
            "v1" => Ok(HTTPApiVersion::V1),
            _ => Err(()),
        }
    }
}

/// Custom error handler for rate limiting that returns structured JSON responses.
fn create_rate_limit_error_handler(
    config: &RateLimitConfig,
) -> impl Fn(GovernorError) -> axum::response::Response + Clone {
    let base_retry_after_seconds = config.retry_after_seconds;
    let jitter_max_ms = config.jitter_max_ms;

    move |_err: GovernorError| {
        // Calculate retry-after with millisecond precision jitter
        let base_ms = base_retry_after_seconds * 1000;
        let total_ms = if jitter_max_ms > 0 {
            let jitter = rand::rng().random_range(0..=jitter_max_ms);
            base_ms + jitter
        } else {
            base_ms
        };

        // Generate RFC 7231 timestamp indicating when client should retry
        // Uses absolute timestamp instead of relative seconds for cache-safety
        let retry_time = Utc::now() + chrono::Duration::milliseconds(total_ms as i64);
        let retry_after_timestamp = retry_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        AppResponse::<()>::rate_limited(
            "Too many requests at relayer HTTP server",
            &retry_after_timestamp,
        )
        .into_response()
    }
}

// There are TODO commands to remove argument bloat, after that this can be removed.
#[allow(clippy::too_many_arguments)]
pub async fn run_http_server<D>(
    http_endpoint: SocketAddr,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    key_url: KeyUrl,         // TODO: move else where to reduce argument bloat
    gateway_rpc_url: String, // TOOD: Move health checker to individual components. to reduce argument bloat.
    rate_limit_on_post_endpoints: RateLimitConfig,
    input_proof_repo: Arc<InputProofRepository>,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    user_decrypt_repo: Arc<UserDecryptRepository>,
) -> SocketAddr
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    // Initialize health checker
    let health_checker = Arc::new(HealthChecker::new(gateway_rpc_url));

    // Build our application with the POST endpoint '/input-proof'
    let input_proof_handler = Arc::new(InputProofHandler::new(
        orchestrator.clone(),
        api_version,
        input_proof_repo,
    ));

    /// Input proof
    ///
    /// Requests input proof verification
    #[utoipa::path(
    post,
    path = "/input-proof",
    request_body = InputProofRequestJson,
    responses(
        (status = 200, description = "Successfully verified input proof", body = InputProofResponseJson),
        (status = 400, description = "Bad request (wrong version)", body = VersionErrorResponseJson),
        (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
    ),
)]
    async fn input_proof_documented<D>(
        Path(api_version): Path<String>,
        Extension(input_proof_handler): Extension<Arc<InputProofHandler<D>>>,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse
    where
        D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
    {
        if let Ok(version) = HTTPApiVersion::from_str(api_version.as_str()) {
            match version {
                HTTPApiVersion::V1 => http_metrics::with_http_metrics(
                    HttpEndpoint::InputProof,
                    HttpMethod::Post,
                    async move { input_proof_handler.handle(req, &()).await },
                )
                .await
                .into_response(),
            }
        } else {
            let error_response = VersionErrorResponseJson {
                message: format!("Unsupported version: {api_version}, only v1 supported"),
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }

    let user_decrypt_handler = Arc::new(UserDecryptHandler::new(
        Arc::clone(&orchestrator),
        api_version,
        user_decrypt_repo,
    ));
    /// User decryption
    ///
    /// Requests a Private decryption
    #[utoipa::path(
    post,
    path = "/user-decrypt",
    request_body = UserDecryptRequestJson,
    responses(
        (status = 200, description = "Successfully decrypted", body = UserDecryptResponseJson),
        (status = 400, description = "Bad request (wrong version)", body = VersionErrorResponseJson),
        (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
    ),
)]
    async fn user_decrypt_documented<D>(
        Path(api_version): Path<String>,
        Extension(user_decrypt_handler): Extension<Arc<UserDecryptHandler<D>>>,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse
    where
        D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
    {
        if let Ok(version) = HTTPApiVersion::from_str(api_version.as_str()) {
            match version {
                HTTPApiVersion::V1 => http_metrics::with_http_metrics(
                    HttpEndpoint::UserDecrypt,
                    HttpMethod::Post,
                    async move { user_decrypt_handler.handle(req, &()).await },
                )
                .await
                .into_response(),
            }
        } else {
            let error_response = VersionErrorResponseJson {
                message: format!("Unsupported version: {api_version}, only v1 supported"),
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }

    // Public decryption
    let public_decrypt_handler = Arc::new(PublicDecryptHandler::new(
        orchestrator,
        api_version,
        public_decrypt_repo,
    ));

    /// Public decryption
    ///
    /// Requests a Public decryption
    #[utoipa::path(
    post,
    path = "/public-decrypt",
    request_body = PublicDecryptRequestJson,
    responses(
        (status = 200, description = "Successfully decrypted", body = PublicDecryptResponseJson),
        (status = 400, description = "Bad request (wrong version)", body = VersionErrorResponseJson),
        (status = 400, description = "Malformed JSON or validation failed", body = crate::http::ErrorResponse),
        (status = 429, description = "Too many requests", body = crate::http::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::http::ErrorResponse),
    ),
)]
    async fn public_decrypt_documented<D>(
        Path(api_version): Path<String>,
        Extension(public_decrypt_handler): Extension<Arc<PublicDecryptHandler<D>>>,
        req: Request<axum::body::Body>,
    ) -> impl IntoResponse
    where
        D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
    {
        if let Ok(version) = HTTPApiVersion::from_str(api_version.as_str()) {
            match version {
                HTTPApiVersion::V1 => http_metrics::with_http_metrics(
                    HttpEndpoint::PublicDecrypt,
                    HttpMethod::Post,
                    async move { public_decrypt_handler.handle(req, &()).await },
                )
                .await
                .into_response(),
            }
        } else {
            let error_response = VersionErrorResponseJson {
                message: format!("Unsupported version: {api_version}, only v1 supported"),
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }

    /// Key URL
    ///
    /// Returns the URLs to retrieve the public keys
    #[utoipa::path(
    get,
    path = "/keyurl",
    responses(
        (status = 200, description = "Key URL", body = KeyUrlResponseJson),
        (status = 400, description = "Bad request (non-existing version)", body = VersionErrorResponseJson),
    ),
)]
    async fn keyurl_documented(
        Path(api_version): Path<String>,
        Extension(keyurl): Extension<KeyUrl>,
    ) -> impl IntoResponse {
        if let Ok(version) = HTTPApiVersion::from_str(api_version.as_str()) {
            match version {
                HTTPApiVersion::V1 => http_metrics::with_http_metrics(
                    HttpEndpoint::KeyUrl,
                    HttpMethod::Get,
                    async move {
                        let keyurl_response = KeyUrlResponseJson::from(keyurl);
                        Json(keyurl_response)
                    },
                )
                .await
                .into_response(),
            }
        } else {
            let error_response = VersionErrorResponseJson {
                message: format!("Unsupported version: {api_version}, only v1 supported"),
            };
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }

    // OpenAPI documentation
    #[derive(OpenApi)]
    #[openapi(
        servers((url = "/v1", description = "FHEVM Relayer API v1")),
    paths(
        public_decrypt_documented,
        user_decrypt_documented,
        input_proof_documented,
        keyurl_documented,
    ),
    components(
        schemas(PublicDecryptRequestJson, PublicDecryptResponseJson, PublicDecryptErrorResponseJson),
        schemas(UserDecryptRequestJson, UserDecryptResponseJson, UserDecryptErrorResponseJson),
        schemas(InputProofRequestJson, InputProofResponseJson, InputProofErrorResponseJson),
        schemas(KeyUrlResponseJson),
        schemas(crate::http::keyurl_http_listener::Response, crate::http::keyurl_http_listener::FheKeyInfo, crate::http::keyurl_http_listener::KeyData),
        schemas(VersionErrorResponseJson),
        schemas(crate::http::ErrorResponse, crate::http::ApiError, crate::http::ErrorDetail, crate::http::ErrorLabel),
        schemas(crate::http::userdecrypt_http_listener::HandleContractPairJson, crate::http::userdecrypt_http_listener::RequestValidityJson, crate::http::userdecrypt_http_listener::UserDecryptResponsePayloadJson),
        schemas(crate::http::input_http_listener::InputProofResponsePayloadJson),
        schemas(crate::http::public_decrypt_http_listener::PublicDecryptResponsePayloadJson),
        schemas(crate::http::ChainId),
    ),
    tags(
        (name = "FHEVM Relayer API", description = "FHEVM Relayer API")
    )
)]
    struct ApiDoc;

    // Configure rate limiting using settings (configurable via rate_limit_on_post_endpoints config section)
    // Convert RPS to milliseconds per token: 1000ms / RPS = ms per token
    let ms_per_token = 1000 / rate_limit_on_post_endpoints.requests_per_second;
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(ms_per_token as u64)
        .burst_size(rate_limit_on_post_endpoints.burst_size)
        .methods([Method::POST].to_vec())
        .key_extractor(GlobalKeyExtractor)
        .finish()
        .unwrap();

    let app =
        Router::new()
            .route("/liveness", get(liveness_handler))
            .route(
                "/healthz",
                get(move || async move { health_handler(health_checker).await }),
            )
            .route("/version", get(version_handler))
            // Apply rate limiting to POST endpoints
            .route(
                "/{api_version}/input-proof",
                post(input_proof_documented::<D>),
            )
            .route(
                "/{api_version}/public-decrypt",
                post(public_decrypt_documented::<D>),
            )
            .route(
                "/{api_version}/user-decrypt",
                post(user_decrypt_documented::<D>),
            )
            .layer(GovernorLayer::new(governor_conf).error_handler(
                create_rate_limit_error_handler(&rate_limit_on_post_endpoints),
            ))
            .layer(Extension(input_proof_handler))
            .layer(Extension(public_decrypt_handler))
            .layer(Extension(user_decrypt_handler))
            // GET endpoint without rate limiting
            .route("/{api_version}/keyurl", get(keyurl_documented))
            .layer(Extension(key_url))
            .merge(Redoc::with_url("/docs", ApiDoc::openapi()));

    // The port in http_endpoint can either be explicitly specified or set to :0 (in which case
    // listener will bind to free port assigned by OS). Fetch the actual address & return it.
    let listener = tokio::net::TcpListener::bind(http_endpoint).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    println!("Server listening on http://{actual_addr}");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    actual_addr
}
