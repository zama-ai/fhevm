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
    use crate::config::HttpConfig;
    use crate::kms_aggregator::mock::{Fixed, MockClient, Reply};
    use crate::kms_aggregator::{Aggregator, UserChecks};
    use crate::logging::Log;
    use crate::logging::capture::{Sink, capture};

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
                body_read_timeout: Duration::from_secs(2),
                supported_chain_ids: vec![1, 137],
            }),
            shutdown,
        }
    }

    fn healthy() -> App {
        app(Reply::Fixed(Fixed::Ok), CancellationToken::new())
    }

    #[tokio::test(start_paused = true)]
    async fn body_not_received_in_time_is_malformed() {
        // A body that never completes: the handler gives up after `body_read_timeout` (2 s here, instant when paused).
        let never = futures_util::stream::pending::<Result<bytes::Bytes, std::io::Error>>();
        let request = Request::builder()
            .method("POST")
            .uri("/v4/exp/public-decrypt")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from_stream(never))
            .unwrap();
        let response = router(healthy()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["code"], "malformed");
        assert_eq!(body["message"], "request body not received within 2s");
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

    /// Installs the JSON capture; the probe emits every `Log` event once.
    async fn capture_logs() -> (tracing::subscriber::DefaultGuard, Sink) {
        capture("request ", 7, || {
            let mut probe = Log::new("probe");
            probe.received(vec![]);
            probe.body_rejected("probe");
            probe.validation_failed("probe", "probe");
            probe.forwarded(alloy::primitives::B256::ZERO);
            probe.succeeded(200);
            probe.rejected(400, "probe", "probe");
            probe.failed(500, "probe");
        })
        .await
    }

    #[tokio::test]
    async fn every_endpoint_log_line_carries_the_identifiers() {
        let (_guard, sink) = capture_logs().await;

        let (_, ok_id, _) = call(healthy(), "POST", "/v4/exp/user-decrypt", &user_wire()).await;
        let no_key = user_wire().replace(r#""publicKey": "0x20002000""#, r#""publicKey": "0x""#);
        call(healthy(), "POST", "/v4/exp/user-decrypt", &no_key).await;
        call(healthy(), "POST", "/v4/exp/public-decrypt", r#"{"x":1}"#).await;

        let lines = sink.lines("request ");
        let messages: Vec<&str> = lines
            .iter()
            .map(|l| l["fields"]["message"].as_str().unwrap())
            .collect();
        assert_eq!(
            messages,
            [
                "request received",
                "request forwarded",
                "request succeeded",
                "request received",
                "request validation failed",
                "request rejected",
                "request body rejected",
                "request rejected",
            ]
        );
        for line in &lines {
            for field in ["request_id", "flow", "handles", "decryption_id"] {
                assert!(line["fields"][field].is_string(), "{field} missing: {line}");
            }
        }

        // Success: the handles from the first line on, the decryption id once forwarded, one request id.
        let success = &lines[..3];
        let handle = validate::tests::handle(1, 4).to_string();
        for line in success {
            assert_eq!(line["fields"]["request_id"], ok_id);
            assert_eq!(line["fields"]["flow"], "user_decrypt");
            assert!(
                line["fields"]["handles"]
                    .as_str()
                    .unwrap()
                    .contains(&handle)
            );
        }
        assert_eq!(success[0]["fields"]["decryption_id"], "none");
        let id = success[1]["fields"]["decryption_id"].as_str().unwrap();
        assert!(id.starts_with("0x") && id.len() == 66, "{id}");
        assert_eq!(success[2]["fields"]["decryption_id"], id);
        assert_eq!(success[2]["fields"]["status"], 200);

        // Validation failure: explicit, with the field and the issue, nothing forwarded.
        let invalid = &lines[4];
        assert_eq!(invalid["fields"]["field"], "payload.publicKey");
        assert_eq!(invalid["fields"]["issue"], "must not be empty");
        assert_eq!(invalid["fields"]["decryption_id"], "none");
        assert_eq!(lines[5]["fields"]["status"], 400);
        assert_eq!(lines[5]["fields"]["code"], "malformed");

        // unparsable body: nothing is known but the request id and the flow.
        let bad = &lines[6];
        assert_eq!(bad["fields"]["flow"], "public_decrypt");
        assert_eq!(bad["fields"]["handles"], "none");
        assert!(
            bad["fields"]["reason"]
                .as_str()
                .unwrap()
                .contains("unknown field")
        );
    }
}
