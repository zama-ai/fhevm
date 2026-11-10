#![allow(dead_code)]

use alloy::{
    primitives::{B256, Bytes, U256},
    transports::http::reqwest::{self, Client, RequestBuilder, Response, StatusCode},
};
use connector_utils::{
    tests::{
        db::{
            requests::{
                InsertRequestOptions, insert_pad_decryption_request,
                insert_rand_public_decryption_request, insert_rand_user_decryption_request_v2,
            },
            responses::{
                insert_rand_pad_decrypt_response, insert_rand_public_decrypt_response,
                insert_rand_user_decrypt_response, insert_test_pad_decrypt_error_response,
                insert_test_public_decrypt_error_response, insert_test_user_decrypt_error_response,
            },
        },
        rand::{rand_address, rand_pad_request},
        setup::{TestInstance, TestInstanceBuilder, pick_free_port},
    },
    types::{
        DEFAULT_EPOCH_ID, TESTING_KMS_CONTEXT,
        db::{OperationStatus, RequestSource},
        extra_data::extra_data_v2_payload,
    },
};
use endpoint::core::{Config, Endpoint};
use kms_connector_api::{
    AttestationType, ErrorCode, ErrorResponse, HandleEntry, PAD_DECRYPTION_ROUTE,
    PUBLIC_DECRYPTION_ROUTE, PadDecryptionRequest, PadDecryptionResponse, PublicDecryptionRequest,
    PublicDecryptionResponse, RequestValidity, USER_DECRYPTION_ROUTE, UserDecryptionPayload,
    UserDecryptionRequest, UserDecryptionResponse,
};
use sqlx::{Pool, Postgres, Row, types::chrono::Utc};
use std::{
    net::SocketAddr,
    str::FromStr,
    time::{Duration, Instant},
};
use tfhe::FheTypes;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub const CHAIN_ID: u64 = 31337;
pub const EUINT64: u8 = FheTypes::Uint64 as u8;
pub const PUBLIC_REQUESTS_TABLE: &str = "public_decryption_requests";
pub const USER_REQUESTS_TABLE: &str = "user_decryption_requests";
pub const PAD_REQUESTS_TABLE: &str = "pad_decryption_requests";

/// A running endpoint, the DB it works on and the test instance owning them.
pub struct RunningEndpoint {
    pub test_instance: TestInstance,
    pub db: Pool<Postgres>,
    base_url: String,
    cancel_token: CancellationToken,
    task: JoinHandle<anyhow::Result<()>>,
    client: Client,
}

pub async fn setup() -> anyhow::Result<RunningEndpoint> {
    setup_with(|config| config).await
}

pub async fn setup_with(
    configure: impl FnOnce(Config) -> Config,
) -> anyhow::Result<RunningEndpoint> {
    let mut test_instance = TestInstanceBuilder::db_setup().await?;
    let db = test_instance.db().clone();
    let config = configure(Config {
        database_url: test_instance.db_url().parse().unwrap(),
        database_pool_size: 3,
        http_endpoint: SocketAddr::from_str(&format!("127.0.0.1:{}", pick_free_port())).unwrap(),
        supported_chain_ids: vec![CHAIN_ID],
        ..Config::default()
    });

    let base_url = format!("http://{}", config.http_endpoint);
    let (endpoint, _state) = Endpoint::from_config(config).await?;
    let cancel_token = CancellationToken::new();
    let task = tokio::spawn(endpoint.start(cancel_token.clone()));
    test_instance.wait_for_log("HTTP server listening at").await;

    Ok(RunningEndpoint {
        test_instance,
        db,
        base_url,
        cancel_token,
        task,
        client: Client::new(),
    })
}

impl RunningEndpoint {
    pub fn url(&self, route: &str) -> String {
        format!("{}{}", self.base_url, route)
    }

    fn prepare_post(&self, request: &AnyRequest) -> RequestBuilder {
        self.client
            .post(self.url(request.route().path()))
            .json(&request.to_json())
    }

    pub async fn post(&self, request: impl Into<AnyRequest>) -> Response {
        self.prepare_post(&request.into())
            .send()
            .await
            .expect("request failed")
    }

    pub fn spawn_post(&self, request: impl Into<AnyRequest>) -> JoinHandle<Response> {
        let post = self.prepare_post(&request.into());
        tokio::spawn(async move { post.send().await.expect("request failed") })
    }

    /// Posts a raw JSON body to `route`.
    pub async fn post_raw(&self, route: &str, body: impl Into<reqwest::Body>) -> Response {
        self.client
            .post(self.url(route))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .expect("raw request failed")
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.cancel_token.cancel();
        self.task.await?
    }
}

/// A decryption route, to run the tests of the shared request flow on each of them.
#[derive(Clone, Copy, Debug)]
pub enum Route {
    Public,
    User,
    Pad,
}

impl Route {
    pub fn path(self) -> &'static str {
        match self {
            Route::Public => PUBLIC_DECRYPTION_ROUTE,
            Route::User => USER_DECRYPTION_ROUTE,
            Route::Pad => PAD_DECRYPTION_ROUTE,
        }
    }

    pub fn request(self) -> AnyRequest {
        match self {
            Route::Public => rand_public_request().into(),
            Route::User => rand_user_request().into(),
            Route::Pad => rand_pad_request(CHAIN_ID).into(),
        }
    }

    pub fn requests_table(self) -> &'static str {
        match self {
            Route::Public => PUBLIC_REQUESTS_TABLE,
            Route::User => USER_REQUESTS_TABLE,
            Route::Pad => PAD_REQUESTS_TABLE,
        }
    }

    pub fn responses_table(self) -> &'static str {
        match self {
            Route::Public => "public_decryption_responses",
            Route::User => "user_decryption_responses",
            Route::Pad => "pad_decryption_responses",
        }
    }

    /// The response column holding the decryption result.
    pub fn payload_column(self) -> &'static str {
        match self {
            Route::Public => "decrypted_result",
            Route::User => "user_decrypted_shares",
            Route::Pad => "encrypted_payload",
        }
    }

    /// Plays the kms-worker: stores a completed decryption for `id`.
    pub async fn complete(self, db: &Pool<Postgres>, id: B256) -> anyhow::Result<()> {
        match self {
            Route::Public => complete_public(db, id).await.map(drop),
            Route::User => complete_udec(db, id).await.map(drop),
            Route::Pad => complete_pad(db, id).await.map(drop),
        }
    }

    /// Plays the kms-worker: stores (or overrides) a decryption error for `id`.
    pub async fn fail(
        self,
        db: &Pool<Postgres>,
        id: B256,
        code: &str,
        details: &str,
    ) -> anyhow::Result<()> {
        let id = Some(db_id(id));
        match self {
            Route::Public => insert_test_public_decrypt_error_response(db, id, code, details).await,
            Route::User => insert_test_user_decrypt_error_response(db, id, code, details).await,
            Route::Pad => insert_test_pad_decrypt_error_response(db, id, code, details).await,
        }?;
        Ok(())
    }

    /// The `decryption_id` of a `200` response.
    pub async fn decryption_id(self, response: Response) -> B256 {
        assert_eq!(response.status(), StatusCode::OK);
        match self {
            Route::Public => {
                let body: PublicDecryptionResponse = response.json().await.unwrap();
                body.decryption_id
            }
            Route::User => {
                let body: UserDecryptionResponse = response.json().await.unwrap();
                body.decryption_id
            }
            Route::Pad => {
                let body: PadDecryptionResponse = response.json().await.unwrap();
                body.decryption_id
            }
        }
    }
}

/// A request of any route.
#[derive(Clone)]
pub enum AnyRequest {
    Public(PublicDecryptionRequest),
    User(UserDecryptionRequest),
    Pad(PadDecryptionRequest),
}

impl AnyRequest {
    pub fn route(&self) -> Route {
        match self {
            AnyRequest::Public(_) => Route::Public,
            AnyRequest::User(_) => Route::User,
            AnyRequest::Pad(_) => Route::Pad,
        }
    }

    pub fn id(&self) -> B256 {
        match self {
            AnyRequest::Public(r) => r.id(),
            AnyRequest::User(r) => r.id(),
            AnyRequest::Pad(r) => r.id(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            AnyRequest::Public(r) => serde_json::to_value(r),
            AnyRequest::User(r) => serde_json::to_value(r),
            AnyRequest::Pad(r) => serde_json::to_value(r),
        }
        .expect("request serialization failed")
    }

    /// Sets the RFC-016 validity window, if the route has one.
    pub fn with_validity(mut self, start_timestamp: u64, duration_seconds: u64) -> Self {
        let validity = RequestValidity {
            startTimestamp: start_timestamp,
            durationSeconds: duration_seconds,
        };
        match &mut self {
            AnyRequest::Public(_) => {}
            AnyRequest::User(r) => r.payload.requestValidity = validity,
            AnyRequest::Pad(r) => r.payload.requestValidity = validity,
        }
        self
    }

    /// Stores an HTTP-sourced request row with this request's id and `status`.
    pub async fn insert(&self, db: &Pool<Postgres>, status: OperationStatus) -> anyhow::Result<()> {
        let options = InsertRequestOptions::new()
            .with_id(db_id(self.id()))
            .with_status(status)
            .with_source(RequestSource::Http);
        match self {
            AnyRequest::Public(_) => {
                insert_rand_public_decryption_request(db, options).await?;
            }
            AnyRequest::User(_) => {
                insert_rand_user_decryption_request_v2(db, options).await?;
            }
            AnyRequest::Pad(r) => {
                insert_pad_decryption_request(db, r, status).await?;
            }
        }
        Ok(())
    }
}

impl From<PublicDecryptionRequest> for AnyRequest {
    fn from(request: PublicDecryptionRequest) -> Self {
        AnyRequest::Public(request)
    }
}

impl From<UserDecryptionRequest> for AnyRequest {
    fn from(request: UserDecryptionRequest) -> Self {
        AnyRequest::User(request)
    }
}

impl From<PadDecryptionRequest> for AnyRequest {
    fn from(request: PadDecryptionRequest) -> Self {
        AnyRequest::Pad(request)
    }
}

/// Builds a well-formed handle for `chain_id` with a random prefix.
pub fn rand_handle(chain_id: u64) -> B256 {
    let mut bytes: [u8; 32] = rand::random();
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    bytes[30] = EUINT64;
    bytes[31] = 0;
    B256::from(bytes)
}

/// A well-formed v2 `extraData` targeting the testing KMS context and default epoch.
fn extra_data() -> Bytes {
    Bytes::from(extra_data_v2_payload(TESTING_KMS_CONTEXT, DEFAULT_EPOCH_ID))
}

pub fn rand_public_request() -> PublicDecryptionRequest {
    PublicDecryptionRequest {
        ctHandles: vec![rand_handle(CHAIN_ID), rand_handle(CHAIN_ID)],
        extraData: extra_data(),
    }
}

/// Its public key and signature are placeholders.
pub fn rand_user_request() -> UserDecryptionRequest {
    let contract = rand_address();
    UserDecryptionRequest {
        attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
        payload: UserDecryptionPayload {
            handles: vec![HandleEntry {
                handle: rand_handle(CHAIN_ID),
                contractAddress: contract,
                ownerAddress: rand_address(),
            }],
            userAddress: rand_address(),
            publicKey: Bytes::from(vec![0x20; 32]),
            allowedContracts: vec![contract],
            requestValidity: RequestValidity {
                startTimestamp: Utc::now().timestamp() as u64 - 60,
                durationSeconds: 300,
            },
            extraData: extra_data(),
        },
        signature: Bytes::from(vec![0x66; 65]),
    }
}

/// The `U256` the worker-side helpers use for an interface id.
pub fn db_id(id: B256) -> U256 {
    U256::from_be_bytes(id.0)
}

/// Plays the kms-worker: stores a completed public decryption for `id`.
pub async fn complete_public(
    db: &Pool<Postgres>,
    id: B256,
) -> anyhow::Result<connector_utils::types::PublicDecryptionResponse> {
    insert_rand_public_decrypt_response(
        db,
        Some(db_id(id)),
        Some(OperationStatus::Completed),
        RequestSource::Http,
    )
    .await
}

/// Plays the kms-worker: stores a completed user decryption for `id`.
pub async fn complete_udec(
    db: &Pool<Postgres>,
    id: B256,
) -> anyhow::Result<connector_utils::types::UserDecryptionResponse> {
    insert_rand_user_decrypt_response(
        db,
        Some(db_id(id)),
        Some(OperationStatus::Completed),
        RequestSource::Http,
    )
    .await
}

/// Plays the kms-worker: stores a completed pad decryption for `id`.
pub async fn complete_pad(db: &Pool<Postgres>, id: B256) -> anyhow::Result<PadDecryptionResponse> {
    insert_rand_pad_decrypt_response(db, Some(db_id(id)), Some(OperationStatus::Completed)).await
}

/// Plays a successful worker retry, replacing an error payload in place.
pub async fn replace_error_with_success(
    db: &Pool<Postgres>,
    table: &str,
    payload_column: &str,
    id: B256,
) -> anyhow::Result<()> {
    let updated = sqlx::query(&format!(
        "UPDATE {table} SET {payload_column} = $1, signature = $1, error_code = NULL,
        error_details = NULL, created_at = $2 WHERE decryption_id = $3"
    ))
    .bind(vec![0x42_u8; 65])
    .bind(Utc::now())
    .bind(db_id(id).as_le_slice().to_vec())
    .execute(db)
    .await?;
    assert_eq!(updated.rows_affected(), 1);
    Ok(())
}

/// Ages a result without changing its identity or notifying waiters.
pub async fn age_response(
    db: &Pool<Postgres>,
    table: &str,
    id: B256,
    age: Duration,
) -> anyhow::Result<()> {
    let updated = sqlx::query(&format!(
        "UPDATE {table} SET created_at = $1 WHERE decryption_id = $2"
    ))
    .bind(Utc::now() - age)
    .bind(db_id(id).as_le_slice())
    .execute(db)
    .await?;
    assert_eq!(updated.rows_affected(), 1);
    Ok(())
}

pub async fn count_rows(db: &Pool<Postgres>, table: &str) -> i64 {
    sqlx::query(&format!("SELECT COUNT(*) AS n FROM {table}"))
        .fetch_one(db)
        .await
        .unwrap()
        .get::<i64, _>("n")
}

pub async fn wait_for_request_row(
    db: &Pool<Postgres>,
    table: &str,
    id: B256,
) -> sqlx::postgres::PgRow {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let row = sqlx::query(&format!("SELECT * FROM {table} WHERE decryption_id = $1"))
            .bind(db_id(id).as_le_slice().to_vec())
            .fetch_optional(db)
            .await
            .unwrap();
        if let Some(row) = row {
            return row;
        }
        assert!(Instant::now() < deadline, "request row never appeared");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// The request row's status, `None` if the row does not exist (yet).
pub async fn request_status(db: &Pool<Postgres>, table: &str, id: B256) -> Option<String> {
    sqlx::query(&format!(
        "SELECT status::TEXT AS status FROM {table} WHERE decryption_id = $1"
    ))
    .bind(db_id(id).as_le_slice().to_vec())
    .fetch_optional(db)
    .await
    .unwrap()
    .map(|row| row.get::<String, _>("status"))
}

pub async fn wait_for_request_status(db: &Pool<Postgres>, table: &str, id: B256, expected: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if request_status(db, table, id).await.as_deref() == Some(expected) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "request never reached `{expected}`"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

pub async fn error_body(response: Response) -> ErrorResponse {
    response.json::<ErrorResponse>().await.unwrap()
}

/// Asserts the status, code, `retryable` flag and id of an error response, and returns its body
/// for further checks.
pub async fn assert_error(
    response: Response,
    status: StatusCode,
    code: ErrorCode,
    id: B256,
) -> ErrorResponse {
    assert_eq!(response.status(), status);
    let body = error_body(response).await;
    assert_eq!(body.code, code);
    assert_eq!(body.retryable, code.retryable());
    assert_eq!(body.decryption_id, Some(id));
    body
}
