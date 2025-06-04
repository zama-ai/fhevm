use crate::config::settings::KeyUrl;
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::http::input_http_listener::{InputProofHandler, InputProofRequestJson};
use crate::http::keyurl_http_listener::KeyUrlResponseJson;
use crate::http::public_decrypt_http_listener::{PublicDecryptHandler, PublicDecryptRequestJson};
use crate::http::userdecrypt_http_listener::{UserDecryptHandler, UserDecryptRequestJson};
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::handler::{get, post};
use axum::Json;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run_http_server<D>(
    http_endpoint: SocketAddr,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    key_url: KeyUrl,
) where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    let api_version = ApiVersion::new(ApiCategory::PRODUCTION, 1);

    // Build our application with the POST endpoint '/input-proof'
    let input_proof_handler = Arc::new(InputProofHandler::new(orchestrator.clone(), api_version));
    let user_decrypt_handler = Arc::new(UserDecryptHandler::new(
        Arc::clone(&orchestrator),
        api_version,
    ));
    let public_decrypt_handler = Arc::new(PublicDecryptHandler::new(orchestrator, api_version));
    let app = Router::new()
        .route(
            format!("/{}/input-proof", api_version).as_str(),
            post({
                let handler = Arc::clone(&input_proof_handler);
                move |payload: Json<InputProofRequestJson>| {
                    let handler = Arc::clone(&handler);
                    async move {
                        http_metrics::with_http_metrics(
                            HttpEndpoint::InputProof,
                            HttpMethod::Post,
                            async move { handler.handle(payload).await },
                        )
                        .await
                    }
                }
            }),
        )
        .route(
            format!("/{}/public-decrypt", api_version).as_str(),
            post({
                let handler = Arc::new(public_decrypt_handler);
                move |payload: Json<PublicDecryptRequestJson>| {
                    let handler = Arc::clone(&handler);
                    async move {
                        http_metrics::with_http_metrics(
                            HttpEndpoint::PublicDecrypt,
                            HttpMethod::Post,
                            async move { handler.handle(payload).await },
                        )
                        .await
                    }
                }
            }),
        )
        .route(
            format!("/{}/user-decrypt", api_version).as_str(),
            post({
                let handler = Arc::clone(&user_decrypt_handler);
                move |payload: Json<UserDecryptRequestJson>| {
                    let handler = Arc::clone(&handler);
                    async move {
                        http_metrics::with_http_metrics(
                            HttpEndpoint::UserDecrypt,
                            HttpMethod::Post,
                            async move { handler.handle(payload).await },
                        )
                        .await
                    }
                }
            }),
        )
        .route(
            format!("/{}/keyurl", api_version).as_str(),
            get({
                let key_url_clone = key_url.clone();
                move || async {
                    http_metrics::with_http_metrics(
                        HttpEndpoint::KeyUrl,
                        HttpMethod::Get,
                        async move {
                            let keyurl_response = KeyUrlResponseJson::from(key_url_clone);
                            Json(keyurl_response)
                        },
                    )
                    .await
                }
            }),
        );

    println!("Server listening on http://{}", http_endpoint);

    // Start the server with hyper underneath.
    axum::Server::bind(&http_endpoint)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
