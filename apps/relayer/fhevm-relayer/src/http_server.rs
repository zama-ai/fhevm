use crate::input_http_listener::{InputProofHandler, InputProofRequestJson};
use crate::orchestrator::traits::{EventDispatcher, HandlerRegistry};
use crate::orchestrator::Orchestrator;
use crate::relayer_event::RelayerEvent;
use axum::handler::post;
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
    let input_proof_handler = Arc::new(InputProofHandler::new(orchestrator));
    let app = Router::new().route(
        "/input-proof",
        post({
            info!("Received POST request to '/input-proof'");
            let handler = Arc::new(input_proof_handler);
            move |payload: Json<InputProofRequestJson>| async move { handler.handle(payload).await }
        }),
    );

    // Define the socket address for the server to listen on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    // Start the server with hyper underneath.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
