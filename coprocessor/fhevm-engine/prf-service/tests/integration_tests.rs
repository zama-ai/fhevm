mod common;

use alloy::primitives::{Address, B256};
use ciphertext_attestation::{PrfOutputAttestation, PrfOutputRef};
use common::{CONNECTOR, DEADLINE};
use coprocessor_api::PRF_ATTESTATION_HEADER;
use prf_service::{Config, PrfService, SIGNER_HEADER};
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode, Version};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    net::TcpListener,
    num::{NonZeroU32, NonZeroUsize},
    time::Duration,
};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::{task::JoinHandle, time::timeout};
use tokio_util::sync::CancellationToken;

const LABEL: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
const OTHER_LABEL: &str = "ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100";
const BUDGET: i64 = 64;

struct TestService {
    base_url: String,
    /// Admin pool: it plays the pad-worker.
    pool: PgPool,
    cancel_token: CancellationToken,
    draining: CancellationToken,
    task: JoinHandle<anyhow::Result<()>>,
    _db: DBInstance,
}

impl TestService {
    async fn start() -> Self {
        Self::start_with_body_budget(64 << 20, 16 << 20).await
    }

    /// With a body budget of `pod_bytes` for the pod and `share_bytes` for each connector.
    async fn start_with_body_budget(pod_bytes: u32, share_bytes: u32) -> Self {
        let db = setup_test_db(ImportMode::None).await.expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(db.db_url())
            .await
            .expect("pool");
        let service_pool = common::prf_service_pool(&pool, db.db_url(), 8).await;
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let cancel_token = CancellationToken::new();
        let draining = CancellationToken::new();
        let service = PrfService::new(
            Config {
                http_endpoint: ([127, 0, 0, 1], port).into(),
                max_pending_bits: BUDGET,
                max_in_flight_per_connector: NonZeroUsize::new(4).unwrap(),
                max_adds_per_connector: NonZeroUsize::new(2).unwrap(),
                pad_poll_interval: Duration::from_secs(3600),
                max_body_bytes: NonZeroU32::new(pod_bytes).unwrap(),
                max_body_bytes_per_connector: NonZeroU32::new(share_bytes).unwrap(),
                pad_pending_max_age: Duration::from_secs(3600),
            },
            service_pool,
            cancel_token.clone(),
            draining.clone(),
        )
        .unwrap();
        let task = tokio::spawn(service.start());
        Self {
            base_url: format!("http://127.0.0.1:{port}"),
            pool,
            cancel_token,
            draining,
            task,
            _db: db,
        }
    }

    /// A request as the proxy sends it, with the signer that it verified.
    fn request(&self, client: &Client, method: Method, path: &str) -> RequestBuilder {
        self.request_as(client, CONNECTOR, method, path)
    }

    fn request_as(
        &self,
        client: &Client,
        connector: Address,
        method: Method,
        path: &str,
    ) -> RequestBuilder {
        client
            .request(method, format!("{}{path}", self.base_url))
            .header(SIGNER_HEADER, connector.to_checksum(None))
    }

    /// Adds a complete row with a body of `length` bytes, then sends a `GET` for it as `connector`,
    /// on a new connection.
    async fn get_output(&self, connector: Address, label: &str, length: usize) -> Response {
        let path = output_ref(label, 64);
        sqlx::query(
            "INSERT INTO pads (prf_instance, label, pad_length, triggered_by) VALUES (0, $1, 64, $2)",
        )
        .bind(path.label.as_slice())
        .bind(connector.as_slice())
        .execute(&self.pool)
        .await
        .unwrap();
        common::set_output_of_length(&self.pool, &path, 1, length).await;
        self.request_as(
            &Client::new(),
            connector,
            Method::GET,
            &prf_path("0", label, "64"),
        )
        .send()
        .await
        .unwrap()
    }

    async fn get(&self, client: &Client, path: &str) -> Response {
        self.request(client, Method::GET, path)
            .send()
            .await
            .unwrap()
    }

    /// Sends a `GET` in a task, on its own connection.
    fn spawn_get(&self, path: &str) -> JoinHandle<Response> {
        let request = self.request(&Client::new(), Method::GET, path);
        tokio::spawn(async move { request.send().await.unwrap() })
    }

    /// Writes the output of a row, as the pad-worker does.
    async fn complete(&self, label: &str, pad_length: i32, byte: u8) {
        common::set_output(&self.pool, &output_ref(label, pad_length), byte).await;
    }

    async fn fail(&self, label: &str, pad_length: i32) {
        common::set_error(&self.pool, &output_ref(label, pad_length)).await;
    }

    /// Waits until the request for `label` adds its row.
    async fn wait_for_row(&self, label: &str) {
        let has_row = || async {
            sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM pads WHERE label = $1)")
                .bind(label_bytes(label))
                .fetch_one(&self.pool)
                .await
                .unwrap()
        };
        timeout(DEADLINE, async {
            while !has_row().await {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("the request adds its row");
    }

    async fn stop(self) {
        self.cancel_token.cancel();
        self.task.await.unwrap().unwrap();
    }
}

fn prf_path(prf: &str, label: &str, pad_length: &str) -> String {
    format!("/v1/prf/{prf}/labels/{label}/{pad_length}")
}

fn label_bytes(label: &str) -> Vec<u8> {
    hex::decode(label).unwrap()
}

fn output_ref(label: &str, pad_length: i32) -> PrfOutputRef {
    let label = B256::from_slice(&label_bytes(label));
    PrfOutputRef::new(0, label, u16::try_from(pad_length).unwrap())
}

fn output(byte: u8) -> PrfOutputAttestation {
    common::attestation(byte)
}

fn attestation(response: &Response) -> PrfOutputAttestation {
    serde_json::from_str(response.headers()[PRF_ATTESTATION_HEADER].to_str().unwrap()).unwrap()
}

/// Checks an error answer: its status, its JSON body, and that it is not cached.
async fn assert_error(response: Response, status: StatusCode, code: &str, retryable: bool) {
    assert_eq!(response.status(), status, "{code}");
    assert_eq!(response.headers()["cache-control"], "no-store", "{code}");
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["code"], code);
    assert_eq!(body["retryable"], retryable, "{code}");
}

#[tokio::test]
async fn first_request_waits_for_the_output_and_head_gives_its_headers() {
    let service = TestService::start().await;
    let path = prf_path("0", LABEL, "64");
    let request = service.spawn_get(&path);
    service.wait_for_row(LABEL).await;
    assert!(!request.is_finished());

    service.complete(LABEL, 64, 1).await;
    let response = timeout(DEADLINE, request).await.unwrap().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    // coprocessor-proxy forwards over HTTP/1.1.
    assert_eq!(response.version(), Version::HTTP_11);
    assert_eq!(
        response.headers()["content-type"],
        "application/octet-stream"
    );
    assert_eq!(attestation(&response), output(1));
    assert_eq!(response.headers()["content-length"], "32");
    assert_eq!(response.bytes().await.unwrap().as_ref(), [1; 32]);

    let head = service
        .request(&Client::new(), Method::HEAD, &path)
        .send()
        .await
        .unwrap();
    assert_eq!(head.status(), StatusCode::OK);
    assert_eq!(head.headers()["content-length"], "32");
    assert_eq!(attestation(&head), output(1));
    assert!(head.bytes().await.unwrap().is_empty());

    service.stop().await;
}

#[tokio::test]
async fn failed_row_gives_prf_evaluation_failed() {
    let service = TestService::start().await;
    let path = prf_path("0", LABEL, "64");
    let request = service.spawn_get(&path);
    service.wait_for_row(LABEL).await;

    service.fail(LABEL, 64).await;
    let response = timeout(DEADLINE, request).await.unwrap().unwrap();
    assert_error(
        response,
        StatusCode::INTERNAL_SERVER_ERROR,
        "prf_evaluation_failed",
        false,
    )
    .await;

    // A later request reads the failed row and does not wait.
    let response = timeout(DEADLINE, service.get(&Client::new(), &path))
        .await
        .unwrap();
    assert_error(
        response,
        StatusCode::INTERNAL_SERVER_ERROR,
        "prf_evaluation_failed",
        false,
    )
    .await;

    service.stop().await;
}

#[tokio::test]
async fn over_budget_and_in_flight_limits_give_concurrency_limited() {
    let service = TestService::start().await;
    let client = Client::new();
    let first = service.spawn_get(&prf_path("0", LABEL, "64"));
    service.wait_for_row(LABEL).await;

    // The same connector and label, while the first request is in progress.
    let same_label = service.get(&client, &prf_path("0", LABEL, "64")).await;
    assert_eq!(same_label.headers()["retry-after"], "1");
    assert_error(
        same_label,
        StatusCode::TOO_MANY_REQUESTS,
        "concurrency_limited",
        true,
    )
    .await;

    // The first row uses all the budget of the connector.
    let over_budget = service.get(&client, &prf_path("0", OTHER_LABEL, "1")).await;
    assert_eq!(over_budget.headers()["retry-after"], "1");
    assert_error(
        over_budget,
        StatusCode::TOO_MANY_REQUESTS,
        "concurrency_limited",
        true,
    )
    .await;

    service.complete(LABEL, 64, 1).await;
    let response = timeout(DEADLINE, first).await.unwrap().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    service.stop().await;
}

#[tokio::test]
async fn a_disconnected_request_frees_its_place() {
    let service = TestService::start().await;
    let path = prf_path("0", LABEL, "64");
    let first = service.spawn_get(&path);
    service.wait_for_row(LABEL).await;
    first.abort();
    let _ = first.await;

    // Until the server sees the disconnect, a new request gets `429`. Then it waits for the row.
    let deadline = tokio::time::Instant::now() + DEADLINE;
    let second = loop {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the disconnect frees the place"
        );
        let second = service.spawn_get(&path);
        tokio::time::sleep(Duration::from_millis(100)).await;
        if !second.is_finished() {
            break second;
        }
        let refused = second.await.unwrap();
        assert_eq!(refused.status(), StatusCode::TOO_MANY_REQUESTS);
    };
    service.complete(LABEL, 64, 1).await;
    let response = timeout(DEADLINE, second).await.unwrap().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    service.stop().await;
}

#[tokio::test]
async fn database_error_gives_unavailable() {
    let service = TestService::start().await;
    sqlx::query("ALTER TABLE pads RENAME TO pads_hidden")
        .execute(&service.pool)
        .await
        .unwrap();
    let response = service
        .get(&Client::new(), &prf_path("0", LABEL, "64"))
        .await;
    assert_error(
        response,
        StatusCode::SERVICE_UNAVAILABLE,
        "unavailable",
        true,
    )
    .await;

    service.stop().await;
}

#[tokio::test]
async fn returns_rfc_errors() {
    let service = TestService::start().await;
    let client = Client::new();
    let canonical = prf_path("0", LABEL, "64");
    for (path, status, code) in [
        (prf_path("00", LABEL, "64"), 400, "non_canonical_path"),
        (prf_path("0", LABEL, "2049"), 400, "non_canonical_path"),
        (prf_path("1", LABEL, "64"), 404, "unknown_prf"),
        (format!("{canonical}/"), 400, "non_canonical_path"),
        (format!("{canonical}?x=1"), 400, "non_canonical_path"),
        (format!("{canonical}?"), 400, "non_canonical_path"),
        ("/v1/prf/0".to_string(), 400, "non_canonical_path"),
    ] {
        let response = service.get(&client, &path).await;
        assert_error(response, StatusCode::from_u16(status).unwrap(), code, false).await;
    }

    let url = format!("{}{canonical}", service.base_url);
    for (request, code) in [
        (client.get(&url), "auth_missing"),
        (
            client.get(&url).header(SIGNER_HEADER, "0x11"),
            "auth_malformed",
        ),
    ] {
        let response = request.send().await.unwrap();
        assert_eq!(
            response.headers()["www-authenticate"],
            format!("FhevmSig error=\"{code}\"").as_str()
        );
        assert_error(response, StatusCode::UNAUTHORIZED, code, false).await;
    }
    // A refused request adds no row.
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM pads")
        .fetch_one(&service.pool)
        .await
        .unwrap();
    assert_eq!(rows, 0);

    for path in [canonical, "/v1/prf/0".to_string()] {
        let response = service
            .request(&client, Method::POST, &path)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED, "{path}");
        assert_eq!(response.headers()["allow"], "GET, HEAD", "{path}");
        assert!(response.bytes().await.unwrap().is_empty(), "{path}");
    }

    service.stop().await;
}

#[tokio::test]
async fn draining_closes_each_connection_after_its_response() {
    let service = TestService::start().await;
    let url = format!("{}{}", service.base_url, prf_path("0", LABEL, "64"));
    let client = Client::new();

    // No signer header: an answer at once.
    let before = client.get(&url).send().await.unwrap();
    assert_eq!(before.status(), StatusCode::UNAUTHORIZED);
    assert!(before.headers().get("connection").is_none());

    service.draining.cancel();
    let after = client.get(&url).send().await.unwrap();
    assert_eq!(after.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(after.headers()["connection"], "close");

    service.stop().await;
}

#[tokio::test]
async fn stop_serves_a_row_completed_during_the_drain() {
    let service = TestService::start().await;
    let request = service.spawn_get(&prf_path("0", LABEL, "64"));
    service.wait_for_row(LABEL).await;

    // The server drains held requests for up to 5 s after the stop.
    service.cancel_token.cancel();
    service.complete(LABEL, 64, 1).await;
    let response = timeout(Duration::from_secs(2), request)
        .await
        .expect("the waiter wakes the request during the drain")
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    service.task.await.unwrap().unwrap();
}

/// More than the socket buffers: a client that reads nothing keeps the body in prf-service.
const LARGE_BODY: usize = 24 << 20;
const OTHER_CONNECTOR: Address = Address::repeat_byte(0x22);

fn label(byte: u8) -> String {
    hex::encode([byte; 32])
}

#[tokio::test]
async fn a_body_of_several_chunks_is_served_whole() {
    let service = TestService::start().await;
    let length = 100_000;
    let response = service.get_output(CONNECTOR, &label(1), length).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length(), Some(length as u64));
    assert_eq!(response.bytes().await.unwrap(), vec![1u8; length]);
    service.stop().await;
}

#[tokio::test]
async fn a_connector_above_its_body_share_gets_concurrency_limited() {
    let service = TestService::start_with_body_budget(64 << 20, 32 << 20).await;
    // The client does not read this body, so it keeps its part of the share.
    let held = service.get_output(CONNECTOR, &label(1), LARGE_BODY).await;
    assert_eq!(held.status(), StatusCode::OK);

    let refused = service.get_output(CONNECTOR, &label(2), LARGE_BODY).await;
    assert_eq!(refused.headers()["retry-after"], "1");
    assert_error(
        refused,
        StatusCode::TOO_MANY_REQUESTS,
        "concurrency_limited",
        true,
    )
    .await;
    // Another connector has its own share.
    let other = service
        .get_output(OTHER_CONNECTOR, &label(3), LARGE_BODY)
        .await;
    assert_eq!(other.status(), StatusCode::OK);

    drop((held, other));
    service.stop().await;
}

#[tokio::test]
async fn a_full_pod_body_budget_gives_unavailable_until_a_body_is_released() {
    let service = TestService::start_with_body_budget(32 << 20, 32 << 20).await;
    let held = service.get_output(CONNECTOR, &label(1), LARGE_BODY).await;
    assert_eq!(held.status(), StatusCode::OK);

    // No room in the pod: the request waits 5 s, then gets `503`.
    let refused = service
        .get_output(OTHER_CONNECTOR, &label(2), LARGE_BODY)
        .await;
    assert_eq!(refused.headers()["retry-after"], "1");
    assert_error(
        refused,
        StatusCode::SERVICE_UNAVAILABLE,
        "unavailable",
        true,
    )
    .await;

    // The client of the held body disconnects: its budget comes back.
    drop(held);
    let served = timeout(DEADLINE, async {
        loop {
            let response = service
                .request_as(
                    &Client::new(),
                    OTHER_CONNECTOR,
                    Method::GET,
                    &prf_path("0", &label(2), "64"),
                )
                .send()
                .await
                .unwrap();
            if response.status() == StatusCode::OK {
                return response;
            }
        }
    })
    .await
    .expect("the body budget comes back after the disconnect");
    assert_eq!(served.bytes().await.unwrap().len(), LARGE_BODY);
    service.stop().await;
}
