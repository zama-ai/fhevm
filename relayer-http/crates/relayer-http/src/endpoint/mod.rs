//! The relayer's HTTP layer: two sync decryption routes, the Kubernetes probes, one error model. Start with `docs.md`.

pub mod error;
pub mod flows;
mod ops;
pub mod summary;
pub mod validate;

use std::any::Any;
use std::cell::Cell;

use axum::Router;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tokio::time::Instant;
use tower_http::catch_panic::CatchPanicLayer;

pub use error::ApiError;

use crate::App;
use crate::metrics::{FlowName, INTERNAL_CODE};
use summary::Summary;

/// Every route of the process, on one port.
pub fn router(app: App) -> Router {
    let routes = Router::new()
        .route("/v4/user-decrypt", post(flows::user_decrypt::handle))
        .route("/v4/public-decrypt", post(flows::public_decrypt::handle))
        .route("/liveness", get(ops::liveness))
        .route("/healthz", get(ops::healthz))
        .fallback(error::not_found)
        .method_not_allowed_fallback(error::method_not_allowed);
    layered(routes, app)
}

/// The layers every route is served behind, innermost first: the body limit, the answer to a panic, the summary line.
fn layered(routes: Router<App>, app: App) -> Router {
    routes
        .layer(DefaultBodyLimit::max(app.http.max_body_bytes))
        .layer(CatchPanicLayer::custom(answer_panic))
        .layer(middleware::from_fn_with_state(app.clone(), record))
        .with_state(app)
}

/// What happens when a response is sent: the summary line and the request metrics, from the `Summary` the
/// terminal response carries. The probes carry none and are not requests. The request runs with its own
/// [`Panicked`] slot, which a panic fills while it unwinds.
async fn record(app: State<App>, request: Request, next: Next) -> Response {
    let started = Instant::now();
    let response = PANICKED.scope(Cell::default(), next.run(request)).await;
    if let Some(summary) = response.extensions().get::<Summary>() {
        summary.complete(Some(response.status().as_u16()), started, &app.metrics);
    }
    response
}

/// What a panic inside a request leaves for its answer, filled while it unwinds.
#[derive(Debug, Default)]
struct Panicked {
    /// Where it happened (`file:line:column`), from the panic hook.
    location: Option<String>,
    /// The request the `Guard` already logged; `None` if no guarded handler was running.
    logged: Option<String>,
}

tokio::task_local! {
    /// The panic of the request being served, if any: one slot per request, set by `record`.
    static PANICKED: Cell<Panicked>;
}

/// Runs `f` on the slot of the request being served; `None` outside a request.
fn with_panicked<R>(f: impl FnOnce(&mut Panicked) -> R) -> Option<R> {
    PANICKED
        .try_with(|cell| {
            let mut panicked = cell.take();
            let result = f(&mut panicked);
            cell.set(panicked);
            result
        })
        .ok()
}

/// The panic hook's part: keeps `location` for the request's `internal error` line. False outside a request: the
/// hook logs the panic itself.
pub fn note_panic(location: Option<String>) -> bool {
    with_panicked(|panicked| panicked.location = location).is_some()
}

/// The `Guard` logged the panicked request `request_id`: the answer must not log it again. Returns the location the
/// hook noted.
fn panic_logged(request_id: &str) -> Option<String> {
    with_panicked(|panicked| {
        panicked.logged = Some(request_id.to_owned());
        panicked.location.take()
    })
    .flatten()
}

/// The answer to a panic caught by `CatchPanicLayer`: `500 internal`, the one error body, with the request's id.
/// A request its `Guard` already logged carries no `Summary`; otherwise (a panic outside a guarded handler) the
/// answer's `Summary` is logged by `record` like any other. The payload is never read: it may hold request data.
fn answer_panic(_payload: Box<dyn Any + Send + 'static>) -> Response {
    let panicked = with_panicked(std::mem::take).unwrap_or_default();
    let mut summary = Summary::new(FlowName::None);
    if let Some(request_id) = &panicked.logged {
        summary.request_id.clone_from(request_id);
    }
    summary.panic_location = panicked.location;
    let mut response = ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        INTERNAL_CODE,
        &summary,
        summary::PANIC_REASON,
    )
    .into_response();
    if panicked.logged.is_some() {
        response.extensions_mut().remove::<Summary>();
    }
    response
}

/// Binds `http.endpoint` and serves until the shutdown token is cancelled, then drains the in-flight requests
/// (bounded by `http.body_read_timeout` plus `call.timeout`: an aggregation is never cut by the shutdown).
pub async fn serve(app: App) -> std::io::Result<()> {
    let listener = TcpListener::bind(app.http.endpoint).await?;
    let shutdown = app.shutdown.clone();
    axum::serve(listener, router(app))
        .with_graceful_shutdown(shutdown.cancelled_owned())
        .await
}

#[cfg(test)]
pub(crate) mod tests {
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
    use crate::kms_aggregator::mock::{EXTRA_DATA_V2, Fixed, MockClient, Reply};
    use crate::kms_aggregator::{Aggregator, UserChecks};
    use crate::logging::capture::capture_runs;
    use crate::metrics::tests::{histogram_sum, total, value};
    use crate::metrics::{FlowName, Metrics};

    /// An app over `mock`: thresholds 9 (user) and 5 (public), a 2 s deadline, `max_retries` retries.
    fn app_with(mock: Arc<MockClient>, max_retries: u32, shutdown: CancellationToken) -> App {
        let caller = mock.caller(Duration::from_secs(2), max_retries);
        let metrics = caller.metrics().clone();
        App {
            user_decrypt: Arc::new(Aggregator::new(caller.clone(), 9, UserChecks::default())),
            public_decrypt: Arc::new(Aggregator::new(caller, 5, ())),
            http: Arc::new(HttpConfig::for_tests(&[1, 137])),
            metrics,
            shutdown,
        }
    }

    /// 13 mock nodes answering `reply`, no retries.
    fn app(reply: Reply, shutdown: CancellationToken) -> App {
        let mock = MockClient::new(vec![vec![reply]; 13], Duration::from_millis(1));
        app_with(mock, 0, shutdown)
    }

    pub(crate) fn healthy() -> App {
        app(Reply::Fixed(Fixed::Ok), CancellationToken::new())
    }

    #[tokio::test(start_paused = true)]
    async fn body_not_received_in_time_is_malformed() {
        // A body that never completes: the handler gives up after `body_read_timeout` (2 s here, instant when paused).
        let never = futures_util::stream::pending::<Result<bytes::Bytes, std::io::Error>>();
        let request = Request::builder()
            .method("POST")
            .uri("/v4/public-decrypt")
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

    pub(crate) fn user_wire() -> String {
        flows::user_decrypt::tests::wire(&format!("\"{}\"", flows::now() - 10), "\"600\"")
    }

    #[tokio::test]
    async fn public_decrypt_answers_the_envelope() {
        let (status, request_id, body) = call(
            healthy(),
            "POST",
            "/v4/public-decrypt",
            &flows::public_decrypt::tests::wire(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["status"], "succeeded");
        assert_eq!(body["requestId"], request_id);
        assert_eq!(body["result"]["signatures"].as_array().unwrap().len(), 13);
        assert!(body["result"]["decryptedValue"].is_string());
        assert_eq!(body["result"]["extraData"], EXTRA_DATA_V2);
    }

    #[tokio::test]
    async fn user_decrypt_answers_the_envelope() {
        let (status, request_id, body) =
            call(healthy(), "POST", "/v4/user-decrypt", &user_wire()).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["requestId"], request_id);
        assert_eq!(body["result"]["result"].as_array().unwrap().len(), 13);
        let share = &body["result"]["result"][0];
        assert!(share["payload"].is_string() && share["signature"].is_string());
        assert_eq!(share["extraData"], EXTRA_DATA_V2);
    }

    #[tokio::test]
    async fn bad_bodies_are_400_malformed_with_the_field() {
        let cases = [
            ("{ not json".to_owned(), "invalid JSON"),
            (
                format!(r#"{{"ciphertextHandles": [], "extraData": "{EXTRA_DATA_V2}"}}"#),
                "ciphertextHandles: must not be empty",
            ),
            (
                format!(r#"{{"ciphertextHandles": ["0x12"], "extraData": "{EXTRA_DATA_V2}"}}"#),
                "ciphertextHandles",
            ),
            (
                format!(r#"{{"ciphertextHandles": [], "extraData": "{EXTRA_DATA_V2}", "x": 1}}"#),
                "unknown field",
            ),
        ];
        for (payload, expected) in cases {
            let (status, id, body) = call(healthy(), "POST", "/v4/public-decrypt", &payload).await;
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
    async fn extra_data_v0_and_v1_are_400_malformed() {
        let v1 = format!("0x01{}", "07".repeat(32));
        let routes = [
            (
                "/v4/public-decrypt",
                flows::public_decrypt::tests::wire(),
                "extraData",
            ),
            ("/v4/user-decrypt", user_wire(), "payload.extraData"),
        ];
        for (uri, valid, field) in routes {
            for extra_data in ["0x", "0x00", v1.as_str()] {
                let payload = valid.replace(EXTRA_DATA_V2, extra_data);
                let (status, id, body) = call(healthy(), "POST", uri, &payload).await;
                assert_eq!(status, StatusCode::BAD_REQUEST, "{uri} {extra_data}");
                assert_eq!(body["code"], "malformed");
                assert_eq!(
                    body["message"],
                    format!("{field}: must be 0x02 + context id + epoch id"),
                    "{uri} {extra_data}"
                );
                assert_eq!(body["requestId"], id);
            }
        }
    }

    #[tokio::test]
    async fn oversized_body_and_wrong_content_type_are_400() {
        let big = format!(
            r#"{{"ciphertextHandles": [], "extraData": "0x{}"}}"#,
            "00".repeat(3000)
        );
        let (status, _, body) = call(healthy(), "POST", "/v4/public-decrypt", &big).await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::BAD_REQUEST, "malformed")
        );

        let response = router(healthy())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v4/public-decrypt")
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
        let (status, _, body) = call(healthy(), "GET", "/v4/public-decrypt", "").await;
        assert_eq!(
            (status, body["code"].as_str().unwrap()),
            (StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed")
        );
    }

    #[tokio::test]
    async fn aggregation_failure_is_the_dominant_connector_error() {
        let app = app(Reply::Error(ErrorCode::AclDenied), CancellationToken::new());
        let (status, _, body) = call(app, "POST", "/v4/user-decrypt", &user_wire()).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
        assert_eq!(body["code"], "acl_denied");
        assert!(body["message"].as_str().unwrap().contains("0 of 9"));
    }

    #[tokio::test]
    async fn healthz_is_503_once_the_shutdown_started() {
        let shutdown = CancellationToken::new();
        let app = app(Reply::Fixed(Fixed::Ok), shutdown.clone());
        let (status, _, body) = call(app.clone(), "GET", "/healthz", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::OK, "ready")
        );
        shutdown.cancel();
        let (status, _, body) = call(app, "GET", "/healthz", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::SERVICE_UNAVAILABLE, "shutting_down")
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_in_flight_when_the_shutdown_starts_still_gets_its_answer() {
        // 13 nodes answering after 500 ms; the shutdown starts at 100 ms.
        let shutdown = CancellationToken::new();
        let mock = MockClient::new(
            vec![vec![Reply::Fixed(Fixed::Ok)]; 13],
            Duration::from_millis(500),
        );
        let app = app_with(mock, 0, shutdown.clone());
        let wire = user_wire();
        let ((status, _, body), ()) =
            tokio::join!(call(app, "POST", "/v4/user-decrypt", &wire), async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                shutdown.cancel();
            });
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["status"], "succeeded");
        assert_eq!(body["result"]["result"].as_array().unwrap().len(), 13);
    }

    #[tokio::test]
    async fn liveness_is_always_alive() {
        let (status, _, body) = call(healthy(), "GET", "/liveness", "").await;
        assert_eq!(
            (status, body["status"].as_str().unwrap()),
            (StatusCode::OK, "alive")
        );
    }

    /// 13 mock nodes running `scripts` (one per node) with `max_retries` retries.
    pub(crate) fn scripted(scripts: Vec<Vec<Reply>>, max_retries: u32) -> (App, Arc<MockClient>) {
        let mock = MockClient::new(scripts, Duration::from_millis(1));
        let app = app_with(mock.clone(), max_retries, CancellationToken::new());
        (app, mock)
    }

    /// The lines of one request, the summary line among them, whichever call sites it reaches.
    pub(crate) async fn logged(
        make: impl Fn() -> App,
        uri: &str,
        body: &str,
        also: &[&str],
    ) -> (StatusCode, Vec<Value>) {
        let status = std::cell::Cell::new(StatusCode::OK);
        let mut expected = vec!["request completed"];
        expected.extend_from_slice(also);
        let lines = capture_runs(&expected, || async {
            let (s, _, _) = call(make(), "POST", uri, body).await;
            status.set(s);
        })
        .await;
        (status.get(), lines)
    }

    fn summaries(lines: &[Value]) -> Vec<&Value> {
        lines
            .iter()
            .filter(|l| l["fields"]["message"] == "request completed")
            .collect()
    }

    /// The summary line of the one request in `lines`: exactly one, INFO, with exactly the defined fields.
    fn the_summary(lines: &[Value]) -> &Value {
        let found = summaries(lines);
        assert_eq!(found.len(), 1, "one summary line per request: {lines:?}");
        let line = found[0];
        assert_eq!(line["level"], "INFO");
        let mut got: Vec<&str> = line["fields"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .filter(|k| *k != "message")
            .collect();
        let mut defined: Vec<&str> = summary::FIELDS.iter().map(|f| f.name).collect();
        got.sort_unstable();
        defined.sort_unstable();
        assert_eq!(got, defined, "the fields of the line are the defined ones");
        line
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_a_success() {
        let body = user_wire();
        let (status, lines) = logged(healthy, "/v4/user-decrypt", &body, &[]).await;
        assert_eq!(status, StatusCode::OK);
        let f = &the_summary(&lines)["fields"];
        assert_eq!(f["flow"], "user_decrypt");
        assert_eq!(
            (f["outcome"].as_str(), f["status"].as_str()),
            (Some("succeeded"), Some("200"))
        );
        assert_eq!(
            (f["code"].as_str(), f["reason"].as_str()),
            (Some("ok"), Some("none"))
        );
        assert!(uuid::Uuid::parse_str(f["request_id"].as_str().unwrap()).is_ok());
        assert!(f["elapsed_ms"].is_u64());
        assert_eq!(f["handles"], validate::tests::handle(1, 4).to_string());
        assert_eq!(
            f["contract_addresses"],
            "0x3333333333333333333333333333333333333333"
        );
        assert_eq!(
            f["allowed_contracts"],
            "0x3333333333333333333333333333333333333333"
        );
        assert_eq!(
            f["user_address"],
            "0x5555555555555555555555555555555555555555"
        );
        assert_eq!(f["chain_id"], "1");
        assert_eq!(f["body_len"], body.len());
        let id = f["decryption_id"].as_str().unwrap();
        assert!(id.starts_with("0x") && id.len() == 66, "{id}");
        assert_eq!(
            (f["counted"].as_u64(), f["threshold"].as_u64()),
            (Some(13), Some(9))
        );
        // The 9th answer reached the threshold: a number of milliseconds, as text.
        let reached = f["threshold_reached_ms"].as_str().unwrap();
        assert!(reached.parse::<u64>().unwrap() <= f["elapsed_ms"].as_u64().unwrap());
        for nodes in [
            "failed_kms_nodes",
            "rejected_kms_nodes",
            "cancelled_kms_nodes",
        ] {
            assert_eq!(f[nodes], "none", "{nodes}");
        }
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_a_public_decrypt_names_no_user_or_contract() {
        let body = flows::public_decrypt::tests::wire();
        let (_, lines) = logged(healthy, "/v4/public-decrypt", &body, &[]).await;
        let f = &the_summary(&lines)["fields"];
        assert_eq!(f["flow"], "public_decrypt");
        assert_eq!(
            (f["user_address"].as_str(), f["contract_addresses"].as_str()),
            (Some("none"), Some("none"))
        );
        assert_eq!(f["allowed_contracts"], "none");
        assert_eq!(f["chain_id"], "137");
        assert_eq!(f["threshold"], 5);
        assert_eq!(f["handles"].as_str().unwrap().split(',').count(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_an_invalid_request_bounds_its_lists_and_the_value_goes_to_debug() {
        // One handle over the default limit of 50: only the limit's worth is listed.
        let handles: Vec<String> = (0..=50)
            .map(|_| format!("\"{}\"", validate::tests::handle(137, 0)))
            .collect();
        let body = format!(
            r#"{{ "ciphertextHandles": [{}], "extraData": "{EXTRA_DATA_V2}" }}"#,
            handles.join(",")
        );
        let (status, lines) =
            logged(healthy, "/v4/public-decrypt", &body, &["request invalid"]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let f = &the_summary(&lines)["fields"];
        let listed: Vec<&str> = f["handles"].as_str().unwrap().split(',').collect();
        assert_eq!(listed.len(), 51);
        assert_eq!(listed.last(), Some(&"+1 more"));
        let debug = lines
            .iter()
            .find(|l| l["fields"]["message"] == "request invalid")
            .unwrap();
        assert_eq!(debug["level"], "DEBUG");
        assert_eq!(debug["fields"]["field"], "ciphertextHandles");
        assert_eq!(debug["fields"]["value"], "51 handles");
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_a_rejected_request_says_why_and_forwards_nothing() {
        let no_key = user_wire().replace(r#""publicKey": "0x20002000""#, r#""publicKey": "0x""#);
        let (status, lines) = logged(healthy, "/v4/user-decrypt", &no_key, &[]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let f = &the_summary(&lines)["fields"];
        assert_eq!(
            (f["outcome"].as_str(), f["code"].as_str()),
            (Some("rejected"), Some("malformed"))
        );
        assert_eq!(f["reason"], "payload.publicKey: must not be empty");
        assert_eq!(f["decryption_id"], "none");
        assert_eq!(f["threshold_reached_ms"], "none");
        assert_eq!(f["handles"], validate::tests::handle(1, 4).to_string());
        assert_eq!(
            (f["counted"].as_u64(), f["threshold"].as_u64()),
            (Some(0), Some(0))
        );
        assert_eq!(f["body_len"], no_key.len());
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_an_unreadable_body_has_no_handles_and_the_error_class() {
        let cases = [
            (r#"{"x":1}"#, "invalid body: unknown field"),
            ("{ not json", "invalid JSON"),
            ("", "invalid JSON"),
            ("[1]", "invalid body: [0]"),
            (
                r#"{"ciphertextHandles": ["0x12"], "extraData": "0x00"}"#,
                "invalid body: ciphertextHandles[0]",
            ),
        ];
        for (body, reason) in cases {
            let (status, lines) = logged(healthy, "/v4/public-decrypt", body, &[]).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
            let (_, _, answer) = call(healthy(), "POST", "/v4/public-decrypt", body).await;
            assert_eq!(answer["message"], reason, "the client gets the logged text");
            let f = &the_summary(&lines)["fields"];
            assert_eq!(f["handles"], "none", "{body}");
            assert_eq!(f["reason"], reason, "{body}");
            assert_eq!(f["body_len"], body.len());
        }
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_a_failure_names_the_nodes_and_stays_info() {
        let make = || {
            scripted(
                vec![vec![Reply::Error(ErrorCode::UpstreamTransient)]; 13],
                0,
            )
            .0
        };
        let (status, lines) = logged(
            make,
            "/v4/public-decrypt",
            &flows::public_decrypt::tests::wire(),
            &["call failed"],
        )
        .await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        let f = &the_summary(&lines)["fields"];
        assert_eq!(
            (f["outcome"].as_str(), f["status"].as_str()),
            (Some("failed"), Some("502"))
        );
        assert_eq!(f["code"], "upstream_transient");
        assert!(
            f["reason"]
                .as_str()
                .unwrap()
                .contains("KMS threshold not reached")
        );
        let failed = f["failed_kms_nodes"].as_str().unwrap();
        assert!(failed.split(',').count() >= 9, "{failed}");
        assert!(failed.split(',').all(|n| n.starts_with("node-")));
        assert!(f["decryption_id"].as_str().unwrap().starts_with("0x"));
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_a_timeout_names_the_cut_nodes() {
        let mut scripts = vec![vec![Reply::Fixed(Fixed::Ok)]; 8];
        scripts.extend(vec![vec![Reply::Fixed(Fixed::Hang)]; 5]);
        let make = || scripted(scripts.clone(), 0).0;
        let (status, lines) = logged(make, "/v4/user-decrypt", &user_wire(), &["call cut"]).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
        let f = &the_summary(&lines)["fields"];
        assert_eq!(
            (f["code"].as_str(), f["counted"].as_u64()),
            (Some("timeout"), Some(8))
        );
        assert_eq!(f["threshold_reached_ms"], "none");
        assert_eq!(
            f["cancelled_kms_nodes"],
            "node-8,node-9,node-10,node-11,node-12"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn summary_line_of_the_router_fallbacks() {
        let lines = capture_runs(&["request completed"], || async {
            call(healthy(), "GET", "/nope", "").await;
        })
        .await;
        let f = &the_summary(&lines)["fields"];
        assert_eq!(f["flow"], "none");
        assert_eq!(
            (f["outcome"].as_str(), f["code"].as_str()),
            (Some("rejected"), Some("not_found"))
        );
        assert_eq!(
            (f["handles"].as_str(), f["decryption_id"].as_str()),
            (Some("none"), Some("none"))
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_probes_are_not_requests() {
        let lines = capture_runs(&["request completed"], || async {
            call(healthy(), "GET", "/liveness", "").await;
            call(healthy(), "GET", "/nope", "").await;
        })
        .await;
        assert_eq!(summaries(&lines).len(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn the_metrics_follow_the_summary() {
        let app = healthy();
        let metrics = app.metrics.clone();
        call(app.clone(), "POST", "/v4/user-decrypt", &user_wire()).await;
        call(app.clone(), "POST", "/v4/public-decrypt", r#"{"x":1}"#).await;
        call(app, "GET", "/nope", "").await;
        let count = |flow: &str, outcome: &str, code: &str| {
            let labels = [("flow", flow), ("outcome", outcome), ("code", code)];
            value(&metrics, "relayer_http_requests_total", &labels)
        };
        assert_eq!(count("user_decrypt", "succeeded", "ok"), Some(1));
        assert_eq!(count("public_decrypt", "rejected", "malformed"), Some(1));
        assert_eq!(count("none", "rejected", "not_found"), Some(1));
        assert_eq!(total(&metrics, "relayer_http_request_duration_seconds"), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_client_that_hangs_up_still_gets_its_summary_line() {
        let body = user_wire();
        let app = scripted(vec![vec![Reply::Fixed(Fixed::Hang)]; 13], 0).0;
        let metrics = app.metrics.clone();
        let lines = capture_runs(&["request completed", "call cut"], || async {
            let request = Request::builder()
                .method("POST")
                .uri("/v4/user-decrypt")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.clone()))
                .unwrap();
            // The client gives up after 100 ms: the handler's future is dropped mid-flight.
            let gave_up = tokio::time::timeout(
                Duration::from_millis(100),
                router(app.clone()).oneshot(request),
            )
            .await;
            assert!(gave_up.is_err());
            // The aborted node tasks drop their calls on the runtime's next turn.
            tokio::time::sleep(Duration::from_millis(1)).await;
        })
        .await;
        let f = &the_summary(&lines)["fields"];
        assert_eq!(f["outcome"], "cancelled");
        assert_eq!(
            (f["status"].as_str(), f["code"].as_str()),
            (Some("none"), Some("client_closed"))
        );
        assert_eq!(f["reason"], "client closed");
        assert_eq!(f["flow"], "user_decrypt");
        assert_eq!(f["handles"], validate::tests::handle(1, 4).to_string());
        assert_eq!(f["body_len"], body.len());
        assert!(f["decryption_id"].as_str().unwrap().starts_with("0x"));
        assert_eq!(
            (f["counted"].as_u64(), f["threshold"].as_u64()),
            (Some(0), Some(9))
        );
        let cut: Vec<String> = (0..13).map(|i| format!("node-{i}")).collect();
        assert_eq!(f["cancelled_kms_nodes"], cut.join(","));
        let closed = [("outcome", "cancelled"), ("code", "client_closed")];
        assert!(value(&metrics, "relayer_http_requests_total", &closed) >= Some(1));
        // Each node's call was cut at the hangup: its attempt is recorded as cut, as slow as it was held, and its
        // call is counted, exactly as at a deadline.
        for node in &cut {
            let cancelled = [("kms_node", node.as_str()), ("code", "cancelled")];
            let node = [("kms_node", node.as_str())];
            assert!(
                value(
                    &metrics,
                    "relayer_http_kms_attempt_errors_total",
                    &cancelled
                ) >= Some(1)
            );
            assert!(value(&metrics, "relayer_http_kms_attempt_duration_seconds", &node) >= Some(1));
            assert!(value(&metrics, "relayer_http_kms_attempts", &node) >= Some(1));
        }
        let held = histogram_sum(&metrics, "relayer_http_kms_attempt_duration_seconds");
        assert!(held >= 13.0 * 0.099 - 1e-9, "{held}");
        // One WARN per cut node, naming the cause.
        let warned: Vec<&str> = lines
            .iter()
            .filter(|l| l["fields"]["message"] == "call cut")
            .inspect(|l| {
                assert_eq!(l["level"], "WARN");
                assert_eq!(l["fields"]["cause"], "client_closed");
                assert_eq!(l["fields"]["request_id"], f["request_id"]);
            })
            .map(|l| l["fields"]["kms_node"].as_str().unwrap())
            .collect();
        assert_eq!(warned, cut.iter().map(String::as_str).collect::<Vec<_>>());
    }

    /// The summary line stays INFO; the defect is one more line, ERROR, with the request id and the reason.
    fn assert_internal_error(lines: &[Value], reason: &str) {
        let f = &the_summary(lines)["fields"];
        assert_eq!(
            (
                f["outcome"].as_str(),
                f["status"].as_str(),
                f["code"].as_str()
            ),
            (Some("failed"), Some("500"), Some("internal"))
        );
        let errors: Vec<&Value> = lines.iter().filter(|l| l["level"] == "ERROR").collect();
        assert_eq!(errors.len(), 1, "{lines:?}");
        let fields = &errors[0]["fields"];
        assert_eq!(fields["message"], "internal error");
        assert_eq!(fields["request_id"], f["request_id"]);
        assert_eq!(fields["reason"], reason);
    }

    #[tokio::test(start_paused = true)]
    async fn an_internal_error_is_an_error_line_next_to_its_info_summary() {
        let metrics = Metrics::new().unwrap();
        let lines = capture_runs(&["request completed", "internal error"], || async {
            Summary::new(FlowName::UserDecrypt)
                .ended("internal", "internal: no output")
                .complete(Some(500), Instant::now(), &metrics);
        })
        .await;
        assert_internal_error(&lines, "internal: no output");
        let labels = [
            ("flow", "user_decrypt"),
            ("outcome", "failed"),
            ("code", "internal"),
        ];
        assert_eq!(
            value(&metrics, "relayer_http_requests_total", &labels),
            Some(1)
        );
    }

    /// A guarded handler that panics after an await, with a node call in flight.
    async fn guarded_panic(app: State<App>) -> Response {
        let mut guard = summary::Guard::new(FlowName::PublicDecrypt, app.metrics.clone());
        guard.summary.tally.pending = vec!["node-3".to_owned()];
        tokio::task::yield_now().await;
        panic!("the handler panics with {MARKER}");
    }

    /// A handler that panics with no `Guard`.
    async fn bare_panic() -> Response {
        panic!("a bare panic with {MARKER}");
    }

    /// `uri` on the production layers over the two panicking routes, with the production panic hook: the answer and
    /// the lines.
    async fn panicked(app: &App, uri: &str) -> (StatusCode, String, Value, Vec<Value>) {
        crate::logging::init_panic_hook();
        let answer = std::cell::RefCell::new(None);
        let lines = capture_runs(&["request completed", "internal error"], || async {
            let routes = Router::new()
                .route("/guarded", post(guarded_panic))
                .route("/bare", post(bare_panic));
            let request = Request::builder()
                .method("POST")
                .uri(uri)
                .body(Body::empty())
                .unwrap();
            let response = layered(routes, app.clone()).oneshot(request).await.unwrap();
            let status = response.status();
            let id = response.headers()["x-request-id"]
                .to_str()
                .unwrap()
                .to_owned();
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let body: Value = serde_json::from_slice(&bytes).unwrap();
            *answer.borrow_mut() = Some((status, id, body));
        })
        .await;
        let _ = std::panic::take_hook();
        let (status, id, body) = answer.take().unwrap();
        (status, id, body, lines)
    }

    #[tokio::test(start_paused = true)]
    async fn a_panicking_handler_answers_500_internal_with_one_error_line() {
        let app = healthy();
        let (status, id, body, lines) = panicked(&app, "/guarded").await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body["code"], "internal");
        assert_eq!(body["message"], "handler panicked");
        assert_eq!(body["requestId"], id);
        assert_internal_error(&lines, "handler panicked");
        let summary = &the_summary(&lines)["fields"];
        assert_eq!(summary["request_id"], id.as_str());
        assert_eq!(summary["flow"], "public_decrypt");
        assert_eq!(summary["cancelled_kms_nodes"], "node-3");
        // The ERROR line names where, never what: the payload may hold request data.
        let error = lines.iter().find(|l| l["level"] == "ERROR").unwrap();
        assert!(
            error["fields"]["location"]
                .as_str()
                .unwrap()
                .contains("endpoint/mod.rs")
        );
        let text: String = lines.iter().map(ToString::to_string).collect();
        assert!(!text.contains(MARKER), "the panic payload reached the logs");
        assert!(!body.to_string().contains(MARKER));
        let cut = lines
            .iter()
            .find(|l| l["fields"]["message"] == "call cut")
            .unwrap();
        assert_eq!(cut["fields"]["cause"], "panic");
        let labels = [
            ("flow", "public_decrypt"),
            ("outcome", "failed"),
            ("code", "internal"),
        ];
        assert!(value(&app.metrics, "relayer_http_requests_total", &labels) >= Some(1));
    }

    #[tokio::test(start_paused = true)]
    async fn a_panic_outside_a_guarded_handler_is_answered_and_logged_once_too() {
        let (status, id, body, lines) = panicked(&healthy(), "/bare").await;
        assert_eq!(
            (status, body["code"].as_str()),
            (StatusCode::INTERNAL_SERVER_ERROR, Some("internal"))
        );
        assert_internal_error(&lines, "handler panicked");
        let summary = &the_summary(&lines)["fields"];
        assert_eq!(
            (summary["request_id"].as_str(), summary["flow"].as_str()),
            (Some(id.as_str()), Some("none"))
        );
        let error = lines.iter().find(|l| l["level"] == "ERROR").unwrap();
        assert!(error["fields"]["location"].is_string());
        let text: String = lines.iter().map(ToString::to_string).collect();
        assert!(!text.contains(MARKER));
    }

    #[tokio::test(start_paused = true)]
    async fn a_body_over_the_limit_is_400_and_says_so() {
        let big = format!(
            r#"{{"ciphertextHandles": [], "extraData": "0x{}"}}"#,
            "00".repeat(3000)
        );
        let (status, lines) = logged(healthy, "/v4/public-decrypt", &big, &[]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let f = &the_summary(&lines)["fields"];
        assert_eq!(f["reason"], "body exceeds 4096 bytes");
        assert_eq!(f["body_len"], 0);
        let (_, _, answer) = call(healthy(), "POST", "/v4/public-decrypt", &big).await;
        assert_eq!(answer["message"], "body exceeds 4096 bytes");
    }

    /// A marker the request carries everywhere it can: body, signature, headers.
    const MARKER: &str = "4d41524b45522d534543524554";

    #[tokio::test(start_paused = true)]
    async fn no_line_at_debug_carries_what_the_request_carried() {
        let signed = user_wire().replace(&"66".repeat(65), MARKER);
        let bodies = [
            ("/v4/user-decrypt", signed.clone()),
            (
                "/v4/user-decrypt",
                signed.replace(r#""publicKey": "0x20002000""#, r#""publicKey": "0x""#),
            ),
            (
                "/v4/user-decrypt",
                signed.replace(r#""userAddress""#, &format!(r#""{MARKER}""#)),
            ),
            (
                "/v4/public-decrypt",
                format!(r#"{{"ciphertextHandles": ["{MARKER}"], "extraData": "0x00"}}"#),
            ),
            (
                "/v4/public-decrypt",
                format!(r#"{{"ciphertextHandles": [], "extraData": "{MARKER}"}}"#),
            ),
            (
                "/v4/public-decrypt",
                format!(r#"{{"ciphertextHandles": {{"{MARKER}": 1}}, "extraData": "0x00"}}"#),
            ),
            ("/v4/public-decrypt", format!("not json {MARKER}")),
        ];
        // A failing and a succeeding node set: every path of the aggregator logs.
        let mut mixed = vec![
            vec![
                Reply::Error(ErrorCode::RateLimited),
                Reply::Fixed(Fixed::Ok)
            ];
            4
        ];
        mixed.extend(vec![vec![Reply::Error(ErrorCode::AclDenied)]; 3]);
        mixed.extend(vec![vec![Reply::Fixed(Fixed::BadSignature)]; 2]);
        mixed.extend(vec![vec![Reply::Fixed(Fixed::Refused)]; 2]);
        mixed.extend(vec![vec![Reply::Fixed(Fixed::Hang)]; 2]);
        let mocks = std::cell::RefCell::new(Vec::new());
        let lines = capture_runs(
            &[
                "request completed",
                "attempt failed",
                "call cut",
                "call failed",
                "response rejected",
            ],
            || async {
                mocks.borrow_mut().clear();
                for (uri, body) in &bodies {
                    let (app, mock) = scripted(mixed.clone(), 1);
                    mocks.borrow_mut().push(mock);
                    let request = Request::builder()
                        .method("POST")
                        .uri(*uri)
                        .header(header::CONTENT_TYPE, "application/json")
                        .header(header::AUTHORIZATION, format!("Bearer {MARKER}"))
                        .header("x-marker", MARKER)
                        .body(Body::from(body.clone()))
                        .unwrap();
                    router(app).oneshot(request).await.unwrap();
                }
                let wrong_type = Request::builder()
                    .method("POST")
                    .uri("/v4/public-decrypt")
                    .header(header::CONTENT_TYPE, MARKER)
                    .body(Body::from(MARKER))
                    .unwrap();
                router(healthy()).oneshot(wrong_type).await.unwrap();
            },
        )
        .await;
        let text: String = lines.iter().map(ToString::to_string).collect();
        assert!(!text.contains(MARKER), "a request value reached the logs");
        assert!(!text.contains("localhost:3000"), "a URL reached the logs");
        let mut sent = 0;
        for mock in mocks.borrow().iter() {
            for node in 0..13 {
                for header in mock.authorizations(node) {
                    sent += 1;
                    assert!(
                        !text.contains(header.to_str().unwrap()),
                        "an outgoing header reached the logs"
                    );
                }
            }
        }
        assert!(sent > 0, "the nodes received signed requests");
        assert_eq!(summaries(&lines).len(), bodies.len() + 1);
    }
}
