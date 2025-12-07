use super::super::types::keyurl::KeyUrlResponseJson;
use crate::config::settings::KeyUrl;
use crate::metrics::http::{self as http_metrics, HttpEndpoint, HttpMethod};
use axum::response::IntoResponse;
use axum::Json;

/// Create router with keyurl routes
pub fn routes(keyurl: KeyUrl) -> axum::Router {
    axum::Router::new().route(
        "/v1/keyurl",
        axum::routing::get(move || {
            let keyurl = keyurl.clone();
            async move { keyurl_v1(keyurl).await }
        }),
    )
}

/// Key URL
///
/// Returns the URLs to retrieve the public keys
#[utoipa::path(
get,
path = "/v1/keyurl",
responses(
    (status = 200, description = "Key URL", body = KeyUrlResponseJson),
),
)]
pub async fn keyurl_v1(keyurl: KeyUrl) -> impl IntoResponse {
    http_metrics::with_http_metrics(HttpEndpoint::KeyUrl, HttpMethod::Get, async move {
        let keyurl_response = KeyUrlResponseJson::from(keyurl);
        Json(keyurl_response)
    })
    .await
    .into_response()
}
