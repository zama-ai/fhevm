use crate::config::settings::Settings;
use crate::core::event::RelayerEvent;
use crate::http::input_http_listener::{InputProofHandler, InputProofRequestJson};
use crate::http::keyurl_http_listener::KeyUrlResponseJson;
use crate::http::userdecrypt_http_listener::{UserDecryptHandler, UserDecryptRequestJson};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use axum::handler::{get, post};
use axum::Json;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub async fn run_http_server<D>(orchestrator: Arc<Orchestrator<D, RelayerEvent>>)
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent> + 'static,
{
    // Build our application with the POST endpoint '/input-proof'
    let input_proof_handler = Arc::new(InputProofHandler::new(orchestrator.clone()));
    let user_decrypt_handler = Arc::new(UserDecryptHandler::new(orchestrator));
    let app =
        Router::new()
            .route(
                "/input-proof",
                post({
                    info!("Enabling handler for POST request to '/input-proof'");
                    let handler = Arc::new(input_proof_handler);
                    move |payload: Json<InputProofRequestJson>| async move {
                        handler.handle(payload).await
                    }
                }),
            )
            .route(
                "/user-decrypt",
                post({
                    info!("Enabling handler for POST request to '/user-decrypt'");
                    let handler = Arc::new(user_decrypt_handler);
                    move |payload: Json<UserDecryptRequestJson>| async move {
                        handler.handle(payload).await
                    }
                }),
            )
            .route(
                "/keyurl",
                get(|| async {
                    info!("Enabling handler for GET request to '/keyurl'");
                    let keyurl_response = KeyUrlResponseJson::from_settings();
                    Json(keyurl_response)
                }),
            );

    // Define the socket address for the server to listen on.
    let settings = Settings::new()
        .map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))
        .unwrap(); // TODO(mano): Handle error properly.
    let addr: SocketAddr = settings.inputproof.url.parse().expect("Invalid address");
    println!("Server listening on http://{}", addr);

    // Start the server with hyper underneath.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
