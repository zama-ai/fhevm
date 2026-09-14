//! The relayer's HTTP layer: two sync decryption routes, the Kubernetes probes, one error model. Start with `docs.md`.

pub mod error;
pub mod flows;
mod ops;
pub mod validate;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tokio::net::TcpListener;

pub use error::ApiError;

use crate::App;

/// Every route of the process, on one port.
pub fn router(app: App) -> Router {
    Router::new()
        .route("/v4/exp/user-decrypt", post(flows::user_decrypt::handle))
        .route(
            "/v4/exp/public-decrypt",
            post(flows::public_decrypt::handle),
        )
        .route("/liveness", get(ops::liveness))
        .route("/healthz", get(ops::healthz))
        .fallback(error::not_found)
        .method_not_allowed_fallback(error::method_not_allowed)
        .layer(DefaultBodyLimit::max(app.http.max_body_bytes))
        .with_state(app)
}

/// Binds `http.endpoint` and serves until the shutdown token is cancelled, then drains the in-flight requests
/// (bounded by `call.timeout`: every running aggregation ends with `Cancelled`).
pub async fn serve(app: App) -> std::io::Result<()> {
    let listener = TcpListener::bind(app.http.endpoint).await?;
    let shutdown = app.shutdown.clone();
    axum::serve(listener, router(app))
        .with_graceful_shutdown(shutdown.cancelled_owned())
        .await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use kms_connector_api::ErrorCode;
    use serde_json::Value;
    use tokio_util::sync::CancellationToken;
    use tower::ServiceExt;

    use super::*;
    use crate::kms_aggregator::mock::{Fixed, MockClient, Reply};
    use crate::kms_aggregator::{Aggregator, UserChecks};
    use crate::settings::HttpConfig;

    /// 13 mock nodes answering `reply`, thresholds 9 (user) and 5 (public), a 2 s deadline.
    fn app(reply: Reply, shutdown: CancellationToken) -> App {
        let mock = MockClient::new(vec![vec![reply]; 13], Duration::from_millis(1));
        let caller = mock.caller(Duration::from_secs(2), 0);
        App {
            user_decrypt: Arc::new(Aggregator::new(
                caller.clone(),
                9,
                UserChecks::default(),
                shutdown.clone(),
            )),
            public_decrypt: Arc::new(Aggregator::new(caller, 5, (), shutdown.clone())),
            http: Arc::new(HttpConfig {
                endpoint: "127.0.0.1:0".parse().unwrap(),
                max_body_bytes: 4096,
                supported_chain_ids: vec![1, 137],
            }),
            shutdown,
        }
    }

    fn healthy() -> App {
        app(Reply::Fixed(Fixed::Ok), CancellationToken::new())
    }

    async fn call(app: App, method: &str, uri: &str, body: &str) -> (StatusCode, String, Value) {
        let mut request = Request::builder().method(method).uri(uri);
        if method == "POST" {
            request = request.header(header::CONTENT_TYPE, "application/json");
        }
        let response = router(app)
            .oneshot(request.body(Body::from(body.to_owned())).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .map(|v| v.to_str().unwrap().to_owned())
            .unwrap_or_default();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, request_id, serde_json::from_slice(&bytes).unwrap())
    }

    fn user_wire() -> String {
        flows::user_decrypt::tests::wire(&format!("\"{}\"", flows::now() - 10), "\"600\"")
    }

    #[tokio::test]
    async fn public_decrypt_answers_the_envelope() {
        let (status, request_id, body) = call(
            healthy(),
            "POST",
            "/v4/exp/public-decrypt",
            &flows::public_decrypt::tests::wire(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["status"], "succeeded");
        assert_eq!(body["requestId"], request_id);
        assert_eq!(body["result"]["signatures"].as_array().unwrap().len(), 13);
        assert!(body["result"]["decryptedValue"].is_string());
        assert_eq!(body["result"]["extraData"], "0x00");
    }

    #[tokio::test]
    async fn user_decrypt_answers_the_envelope() {
        let (status, request_id, body) =
            call(healthy(), "POST", "/v4/exp/user-decrypt", &user_wire()).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["requestId"], request_id);
        assert_eq!(body["result"]["result"].as_array().unwrap().len(), 13);
        let share = &body["result"]["result"][0];
        assert!(share["payload"].is_string() && share["signature"].is_string());
        assert_eq!(share["extraData"], "0x00");
    }

    #[tokio::test]
    async fn bad_bodies_are_400_malformed_with_the_field() {
        let cases = [
            ("{ not json", "Failed to parse the request body as JSON"),
            (
                r#"{"ciphertextHandles": [], "extraData": "0x00"}"#,
                "ciphertextHandles: must not be empty",
            ),
            (
                r#"{"ciphertextHandles": ["0x12"], "extraData": "0x00"}"#,
                "ciphertextHandles",
            ),
            (
                r#"{"ciphertextHandles": [], "extraData": "0x00", "x": 1}"#,
                "unknown field",
            ),
        ];
        for (payload, expected) in cases {
            let (status, id, body) =
                call(healthy(), "POST", "/v4/exp/public-decrypt", payload).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "{payload}");
            assert_eq!(body["code"], "malformed");
            assert!(
                body["message"].as_str().unwrap().contains(expected),
                "{body}"
            );
            assert_eq!(body["requestId"], id);
        }
    }

    #[tokio::test]
    async fn oversized_body_and_wrong_content_type_are_400() {
        let big = format!(
            r#"{{"ciphertextHandles": [], "extraData": "0x{}"}}"#,
            "00".repeat(3000)
        );
        let (status, _, body) = call(healthy(), "POST", "/v4/exp/public-decrypt", &big).await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::BAD_REQUEST, "malformed")
        );

        let response = router(healthy())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v4/exp/public-decrypt")
                    .header(header::CONTENT_TYPE, "text/plain")
                    .body(Body::from(flows::public_decrypt::tests::wire()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn unknown_route_and_method() {
        let (status, _, body) = call(healthy(), "GET", "/nope", "").await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::NOT_FOUND, "not_found")
        );
        let (status, _, body) = call(healthy(), "GET", "/v4/exp/public-decrypt", "").await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed")
        );
    }

    #[tokio::test]
    async fn aggregation_failure_is_the_dominant_connector_error() {
        let app = app(Reply::Error(ErrorCode::AclDenied), CancellationToken::new());
        let (status, _, body) = call(app, "POST", "/v4/exp/user-decrypt", &user_wire()).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
        assert_eq!(body["code"], "acl_denied");
        assert!(body["message"].as_str().unwrap().contains("0 of 9"));
    }

    #[tokio::test]
    async fn shutdown_answers_503_everywhere() {
        let shutdown = CancellationToken::new();
        let app = app(Reply::Fixed(Fixed::Hang), shutdown.clone());
        let (status, _, body) = call(app.clone(), "GET", "/healthz", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::OK, "ready")
        );
        shutdown.cancel();
        let (status, _, body) = call(app.clone(), "GET", "/healthz", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::SERVICE_UNAVAILABLE, "shutting_down")
        );
        let (status, _, body) = call(
            app,
            "POST",
            "/v4/exp/public-decrypt",
            &flows::public_decrypt::tests::wire(),
        )
        .await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::SERVICE_UNAVAILABLE, "shutting_down")
        );
    }

    #[tokio::test]
    async fn liveness_is_always_alive() {
        let (status, _, body) = call(healthy(), "GET", "/liveness", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::OK, "alive")
        );
    }
}
