use std::sync::Arc;

use axum::http::{HeaderMap, StatusCode};
use axum::{response::IntoResponse, Json};
use tokio::sync::watch;
use tracing::info;

use super::super::types::error::RelayerV2ResponseFailed;
use super::super::types::keyurl::KeyUrlResponseJson;
use crate::metrics::{
    http::{self as http_metrics, HttpEndpoint, HttpMethod},
    HttpApiVersion,
};

/// HTTP handler for the `/v2/keyurl` endpoint.
///
/// Reads from a watch channel that always holds a value: the host-chain poller seeds it at
/// startup under `source: chain`, and static config seeds it under `source: config`.
pub struct KeyUrlHandler {
    /// Receiver for the served KeyUrl response; updated on each rotation in `chain` mode.
    keyurl_rx: watch::Receiver<KeyUrlResponseJson>,
}

impl KeyUrlHandler {
    pub fn new(keyurl_rx: watch::Receiver<KeyUrlResponseJson>) -> Arc<Self> {
        Arc::new(Self { keyurl_rx })
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
        let response = self.keyurl_rx.borrow().clone();

        http_metrics::with_http_metrics(
            HttpEndpoint::KeyUrl,
            HttpMethod::Get,
            HttpApiVersion::V2,
            headers,
            async move {
                let status_code = StatusCode::OK;
                info!(http_status = status_code.as_u16(), "HTTP response");
                (status_code, Json(response)).into_response()
            },
        )
        .await
        .into_response()
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
