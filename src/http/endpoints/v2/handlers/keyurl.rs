use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::http::{HeaderMap, StatusCode};
use axum::{response::IntoResponse, Json};
use tokio::{sync::watch, time::timeout};
use tracing::{error, info};

use super::super::types::error::RelayerV2ResponseFailed;
use super::super::types::keyurl::KeyUrlResponseJson;
use crate::{
    core::event::{KeyUrlEventData, KeyUrlEventId, RelayerEvent, RelayerEventData},
    metrics::{
        http::{self as http_metrics, HttpEndpoint, HttpMethod},
        HttpApiVersion,
    },
    orchestrator::{traits::EventHandler, Orchestrator},
};

/// HTTP handler for `/v2/keyurl` endpoint.
///
/// Tracks the latest KeyUrl value in a tokio::watch channel, which can be
/// updated and read multiple times lock-free.
///
/// The value starts as None and is updated on first orchestrator event.
/// Further updates are seen immediately by all readers.
///
/// See `handle_event()` for updates and `keyurl_v2()` for reads.
pub struct KeyUrlHandler {
    /// Sender for posting KeyUrl updates from orchestrator events
    keyurl_tx: watch::Sender<Option<KeyUrlResponseJson>>,

    /// Receiver for reading KeyUrl data in HTTP requests
    keyurl_rx: watch::Receiver<Option<KeyUrlResponseJson>>,
}

impl KeyUrlHandler {
    pub fn new(orchestrator: Arc<Orchestrator>) -> Arc<Self> {
        let (keyurl_tx, keyurl_rx) = watch::channel::<Option<KeyUrlResponseJson>>(None);
        let handler = Arc::new(Self {
            keyurl_tx,
            keyurl_rx,
        });

        // Self-register for KeyDataUpdated events
        orchestrator.register_handler(
            &[KeyUrlEventId::KeyDataUpdated.into()],
            handler.clone() as Arc<dyn EventHandler<RelayerEvent>>,
        );

        info!("KeyUrlHandler (v2) registered for KeyDataUpdated events");

        handler
    }

    /// Create router with keyurl routes
    pub fn routes(self: Arc<Self>) -> axum::Router {
        axum::Router::new().route(
            "/v2/keyurl",
            axum::routing::get({
                let handler = self.clone();
                move |header: HeaderMap| async move {
                    let handler = handler.clone();
                    keyurl_v2(handler, header).await
                }
            }),
        )
    }

    pub async fn keyurl_v2(&self, headers: HeaderMap) -> impl IntoResponse {
        let mut rx = self.keyurl_rx.clone();

        // If not initialized, wait up to 5 seconds for the first update
        if rx.borrow_and_update().is_none() {
            let wait_result = timeout(Duration::from_secs(5), rx.changed()).await;

            // Re-read after potential change
            if wait_result.is_ok() {
                let _ = rx.borrow_and_update();
            }
        }

        let response = rx.borrow().clone();

        http_metrics::with_http_metrics(
            HttpEndpoint::KeyUrl,
            HttpMethod::Get,
            HttpApiVersion::V2,
            headers,
            async move {
                match response {
                    Some(keyurl_response) => {
                        let status_code = StatusCode::OK;

                        info!(http_status = status_code.as_u16(), "HTTP response");

                        (status_code, Json(keyurl_response)).into_response()
                    }
                    None => {
                        error!("key url not configured");
                        RelayerV2ResponseFailed::internal_server_error_simple(
                            "Key URL not yet initialized",
                        )
                        .into_response()
                    }
                }
            },
        )
        .await
        .into_response()
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for KeyUrlHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        if let RelayerEventData::KeyUrl(KeyUrlEventData::KeyDataUpdated { key_data }) = event.data {
            info!("KeyUrl handler (v2) received KeyDataUpdated event");

            let response = KeyUrlResponseJson::from(key_data);

            if self.keyurl_tx.send(Some(response)).is_err() {
                error!("Failed to update KeyUrl data - no receivers listening");
            } else {
                info!("KeyUrl data (v2) updated successfully");
            }
        }
    }
}

/// Retrieve FHE key material URLs.
#[utoipa::path(
get,
path = "/v2/keyurl",
responses(
    (status = 200, description = "FHE public key URLs.", body = KeyUrlResponseJson),
    (status = 500, description = "Internal server error", body = RelayerV2ResponseFailed),
),
tag = "Key URL"
)]
pub async fn keyurl_v2(handler: Arc<KeyUrlHandler>, headers: HeaderMap) -> impl IntoResponse {
    handler.keyurl_v2(headers).await
}
