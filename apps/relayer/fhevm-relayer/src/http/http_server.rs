use crate::config::settings::KeyUrl;
use crate::core::event::{ApiCategory, ApiVersion, RelayerEvent};
use crate::http::input_http_listener::{InputProofHandler, InputProofRequestJson};
use crate::http::keyurl_http_listener::KeyUrlResponseJson;
use crate::http::public_decrypt_http_listener::{PublicDecryptHandler, PublicDecryptRequestJson};
use crate::http::userdecrypt_http_listener::{UserDecryptHandler, UserDecryptRequestJson};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::handler::{get, post};
use axum::Json;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub async fn run_http_server<D>(
    addr: SocketAddr,
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
    let app =
        Router::new()
            .route(
                format!("/{}/input-proof", api_version).as_str(),
                post({
                    info!("Enabling handler for POST request to '/input-proof'");
                    let handler = Arc::new(input_proof_handler);
                    move |payload: Json<InputProofRequestJson>| async move {
                        handler.handle(payload).await
                    }
                }),
            )
            .route(
                format!("/{}/public-decrypt", api_version).as_str(),
                post({
                    info!("Enabling handler for POST request to '/public-decrypt'");
                    let handler = Arc::new(public_decrypt_handler);
                    move |payload: Json<PublicDecryptRequestJson>| async move {
                        handler.handle(payload).await
                    }
                }),
            )
            .route(
                format!("/{}/user-decrypt", api_version).as_str(),
                post({
                    info!("Enabling handler for POST request to '/user-decrypt'");
                    let handler = Arc::new(user_decrypt_handler);
                    move |payload: Json<UserDecryptRequestJson>| async move {
                        handler.handle(payload).await
                    }
                }),
            )
            .route(
                format!("/{}/keyurl", api_version).as_str(),
                get(|| async {
                    info!("Enabling handler for GET request to '/keyurl'");
                    let keyurl_response = KeyUrlResponseJson::from(key_url);
                    Json(keyurl_response)
                }),
            );

    // Define the socket address for the server to listen on.
    println!("Server listening on http://{}", addr);

    // Start the server with hyper underneath.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
