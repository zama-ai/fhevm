use alloy::primitives::{keccak256, Address};
use ciphertext_attestation::{prf::PRF_ATTESTATION_HEADER, PrfOutputAttestation};
use prf_service::{Config, PrfService};
use reqwest::{Client, Method, Response, StatusCode, Version};
use std::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const LABEL: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

struct TestService {
    base_url: String,
    cancel_token: CancellationToken,
    task: JoinHandle<anyhow::Result<()>>,
}

impl TestService {
    fn start() -> Self {
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let cancel_token = CancellationToken::new();
        let service = PrfService::new(
            Config {
                http_endpoint: ([127, 0, 0, 1], port).into(),
            },
            cancel_token.clone(),
        )
        .unwrap();
        let task = tokio::spawn(service.start());
        Self {
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

fn attestation(response: &Response) -> PrfOutputAttestation {
    serde_json::from_str(response.headers()[PRF_ATTESTATION_HEADER].to_str().unwrap()).unwrap()
}

#[tokio::test]
async fn get_returns_body_and_attestation() {
    let service = TestService::start();
    let response = service
        .request(&Client::new(), Method::GET, &prf_path("0", LABEL, "64"))
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["content-type"],
        "application/octet-stream"
    );
    let attestation = attestation(&response);
    let body = response.bytes().await.unwrap();
    assert_eq!(body.len(), 32 * 32);
    assert_eq!(attestation.sns_ciphertext_digest, keccak256(&body));
    assert_eq!(attestation.signer, Address::ZERO);
    assert_eq!(attestation.signature, vec![0; 65]);

    service.stop().await;
}

#[tokio::test]
async fn head_returns_same_headers_without_body() {
    let service = TestService::start();
    let client = Client::new();
    let path = prf_path("0", LABEL, "7");
    let get = service.request(&client, Method::GET, &path).await;
    let head = service.request(&client, Method::HEAD, &path).await;

    assert_eq!(head.status(), StatusCode::OK);
    assert_eq!(
        head.headers()["content-length"],
        get.headers()["content-length"]
    );
    assert_eq!(attestation(&head), attestation(&get));
    assert!(head.bytes().await.unwrap().is_empty());

    drop(client);
    service.stop().await;
}

#[tokio::test]
async fn serves_healthz() {
    let service = TestService::start();
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
    let service = TestService::start();
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
