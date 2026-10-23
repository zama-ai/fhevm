//! The answers of prf-service that need no pad. prf-standins/tests/five_parties.rs covers the
//! outputs, with the stand-in pad-worker.

use alloy::primitives::Address;
use prf_service::{AuthConfig, Config, PrfService};
use reqwest::{Client, Method, Response, StatusCode, Version};
use std::{net::TcpListener, time::Duration};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const LABEL: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

struct TestService {
    _db: DBInstance,
    base_url: String,
    cancel_token: CancellationToken,
    task: JoinHandle<anyhow::Result<()>>,
}

impl TestService {
    async fn start() -> Self {
        let db = setup_test_db(ImportMode::None).await.expect("test db");
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let cancel_token = CancellationToken::new();
        let service = PrfService::new(
            Config {
                http_endpoint: ([127, 0, 0, 1], port).into(),
                database_url: db.db_url().to_owned(),
                database_pool_size: 2,
                max_pending_bits_per_connector: 4096,
                max_adds_per_connector: 1,
                pad_poll_interval: Duration::from_secs(1),
                retry_after: Duration::from_secs(1),
                auth: AuthConfig {
                    ethereum_chain_id: 1,
                    protocol_config_address: Address::ZERO,
                    coprocessor_signer_address: Address::ZERO,
                },
            },
            cancel_token.clone(),
        )
        .await
        .unwrap();
        let task = tokio::spawn(service.start());
        Self {
            _db: db,
            base_url: format!("http://127.0.0.1:{port}"),
            cancel_token,
            task,
        }
    }

    async fn request(&self, client: &Client, method: Method, path: &str) -> Response {
        client
            .request(method, format!("{}{path}", self.base_url))
            .send()
            .await
            .unwrap()
    }

    async fn stop(self) {
        self.cancel_token.cancel();
        self.task.await.unwrap().unwrap();
    }
}

fn prf_path(prf: &str, label: &str, pad_length: &str) -> String {
    format!("/v1/prf/{prf}/labels/{label}/{pad_length}")
}

#[tokio::test]
async fn serves_healthz() {
    let service = TestService::start().await;
    let client = Client::new();
    for method in [Method::GET, Method::HEAD] {
        let health = service.request(&client, method, "/healthz").await;
        assert_eq!(health.version(), Version::HTTP_11);
        assert_eq!(health.status(), StatusCode::OK);
        assert!(health.bytes().await.unwrap().is_empty());
    }

    drop(client);
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
        let response = service.request(&client, Method::GET, &path).await;
        assert_eq!(response.status().as_u16(), status, "{path}");
        assert_eq!(response.headers()["cache-control"], "no-store", "{path}");
        let body: serde_json::Value = response.json().await.unwrap();
        assert_eq!(body["code"], code, "{path}");
        assert_eq!(body["retryable"], false, "{path}");
    }

    for path in [canonical, "/v1/prf/0".to_string()] {
        let response = service.request(&client, Method::POST, &path).await;
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED, "{path}");
        assert_eq!(response.headers()["allow"], "GET, HEAD", "{path}");
        assert!(response.bytes().await.unwrap().is_empty(), "{path}");
    }

    drop(client);
    service.stop().await;
}
