//! The HTTP routes of the Merkle proof service: the health routes both binaries serve, and the
//! Merkle proof route that only `solana_merkle_proof_server` adds (DD-064).
//!
//! The KMS connector asks for the inclusion proof of the leaf that authorizes a
//! decrypt (an allow of a key on a handle, or a handle made public) and verifies
//! it against the peaks of the on-chain account it read itself. This server only
//! reads the leaf record ([`crate::store`]) and makes no decrypt authorization
//! decision. It answers only callers in [`KmsTxSenders`]: each request carries a
//! `request_authorization` signature by a KMS node's tx-sender.
//!
//! The wire contract is the committed OpenAPI document in `openapi/`; a test keeps
//! it in sync with the code. Errors use the RFC 033 body shape.

use std::{
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::primitives::Address;
use axum::{
    body::Bytes,
    extract::{rejection::BytesRejection, DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use zama_solana_acl::mmr_verify;

use crate::{
    kms_tx_senders::KmsTxSenders,
    store::{find_leaf, load_proof, load_store_cursor, LeafKind},
};

/// Most leaves one request may ask for.
const MAX_LEAVES_PER_REQUEST: usize = 64;

/// Comfortably above [`MAX_LEAVES_PER_REQUEST`] queries.
const MAX_REQUEST_BYTES: usize = 64 * 1024;

pub const MERKLE_PROOFS_PATH: &str = "/v1/solana/merkle-proofs";

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    senders: KmsTxSenders,
}

pub struct HttpServer {
    router: Router,
    port: u16,
    cancel_token: CancellationToken,
}

impl HttpServer {
    /// The health routes alone, for the indexer.
    pub fn health(
        pool: PgPool,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            router: health_router(pool),
            port,
            cancel_token,
        }
    }

    /// The health routes and the Merkle proof route, for the proof server.
    pub fn merkle_proofs(
        pool: PgPool,
        senders: KmsTxSenders,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            router: merkle_proofs_router(pool, senders),
            port,
            cancel_token,
        }
    }

    /// Serves until the cancellation token fires.
    pub async fn start(self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;
        info!("Starting HTTP server on {}", addr);
        serve(listener, self.router, self.cancel_token).await
    }
}

fn health_router(pool: PgPool) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/liveness", get(liveness))
        .with_state(pool)
}

fn merkle_proofs_router(pool: PgPool, senders: KmsTxSenders) -> Router {
    Router::new()
        .route("/healthz", get(proof_server_healthz))
        .route("/liveness", get(liveness))
        .route(
            MERKLE_PROOFS_PATH,
            post(merkle_proofs).layer(DefaultBodyLimit::max(MAX_REQUEST_BYTES)),
        )
        .with_state(AppState { pool, senders })
}

async fn serve(
    listener: TcpListener,
    router: Router,
    cancel_token: CancellationToken,
) -> anyhow::Result<()> {
    let shutdown = async move { cancel_token.cancelled().await };
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(|err| {
            error!("HTTP server error: {}", err);
            anyhow::anyhow!("HTTP server error: {}", err)
        })
}

// --- Health -------------------------------------------------------------------

async fn liveness() -> StatusCode {
    StatusCode::OK
}

async fn healthz(State(pool): State<PgPool>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            error!(error = %err, "database health check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

/// Not ready until the KMS tx-sender set is read, so a rollout waits for it.
async fn proof_server_healthz(State(state): State<AppState>) -> StatusCode {
    if !state.senders.is_loaded() {
        warn!("KMS tx-sender set not read yet");
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    healthz(State(state.pool)).await
}

// --- Wire types -----------------------------------------------------------------

/// Which leaf authorizes the decrypt.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LeafQueryKind {
    /// A historical-access leaf: `key` was allowed on `handle`.
    Allowed,
    /// A public-decrypt leaf: `handle` was made public.
    Public,
}

/// One leaf to prove. Byte fields are 32 bytes as lowercase hex, `0x` optional.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeafQuery {
    /// The encrypted store account whose MMR holds the leaf.
    pub encrypted_store: String,
    pub handle: String,
    pub kind: LeafQueryKind,
    /// The allowed key; required for `allowed`, absent for `public`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProofRequest {
    /// At most [`MAX_LEAVES_PER_REQUEST`] entries; answered in order.
    pub leaves: Vec<LeafQuery>,
}

/// The answer for one queried leaf, in request order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum MerkleProofOutcome {
    /// The leaf is recorded. `leafCount` is the history the record had sealed
    /// when it built the proof; the caller verifies the path against the
    /// on-chain account's peaks and retries when the record is behind the
    /// chain (`leafCount` smaller).
    #[serde(rename_all = "camelCase")]
    Found {
        leaf_index: u64,
        leaf_count: u64,
        /// Authentication path from the leaf to its peak.
        siblings: Vec<String>,
    },
    /// The account is recorded but no such leaf is, at `leafCount` leaves. Either
    /// it was never sealed or the record has not reached the block that sealed it.
    #[serde(rename_all = "camelCase")]
    NotFound { leaf_count: u64 },
    /// The record never saw this account.
    UnknownAccount,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MerkleProofResponse {
    pub proofs: Vec<MerkleProofOutcome>,
}

/// Error codes, in the RFC 033 vocabulary.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, bad hex, missing key, too many leaves.
    Malformed,
    /// The request signature is missing, malformed or expired, or its signer is
    /// not the tx-sender of a node in a live KMS context.
    SenderAuthenticationFailed,
    /// The leaf record could not be read or is inconsistent, or the KMS
    /// tx-sender set is not read yet; retry later.
    UpstreamTransient,
}

impl ErrorCode {
    fn http_status(self) -> StatusCode {
        match self {
            Self::Malformed => StatusCode::BAD_REQUEST,
            Self::SenderAuthenticationFailed => StatusCode::UNAUTHORIZED,
            Self::UpstreamTransient => StatusCode::BAD_GATEWAY,
        }
    }

    fn retryable(self) -> bool {
        matches!(self, Self::UpstreamTransient)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}

pub struct HttpError {
    code: ErrorCode,
    message: String,
}

impl HttpError {
    fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    fn malformed(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Malformed, message)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (
            self.code.http_status(),
            Json(ErrorResponse {
                code: self.code,
                message: self.message,
                retryable: self.code.retryable(),
            }),
        )
            .into_response()
    }
}

// --- Merkle proofs -----------------------------------------------------------------

/// Builds the inclusion proof of each queried leaf from the leaf record.
#[utoipa::path(
    post,
    path = MERKLE_PROOFS_PATH,
    tag = "solana",
    request_body = MerkleProofRequest,
    responses(
        (status = 200, description = "One answer per queried leaf, in request order", body = MerkleProofResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 401, description = "Request signature refused", body = ErrorResponse),
        (status = 502, description = "Leaf record or KMS tx-sender set unavailable", body = ErrorResponse),
    ),
    security(("request_authorization" = [])),
)]
async fn merkle_proofs(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Json<MerkleProofResponse>, HttpError> {
    let body =
        body.map_err(|rejection| HttpError::malformed(rejection.body_text()))?;
    authorize(&state.senders, &headers, &body)?;
    let request: MerkleProofRequest = serde_json::from_slice(&body)
        .map_err(|err| HttpError::malformed(err.to_string()))?;
    if request.leaves.len() > MAX_LEAVES_PER_REQUEST {
        return Err(HttpError::malformed(format!(
            "at most {MAX_LEAVES_PER_REQUEST} leaves per request, got {}",
            request.leaves.len()
        )));
    }
    let queries = request
        .leaves
        .iter()
        .map(ParsedQuery::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let mut proofs = Vec::with_capacity(queries.len());
    for query in queries {
        proofs.push(prove(&state.pool, &query).await?);
    }
    Ok(Json(MerkleProofResponse { proofs }))
}

/// Returns the KMS tx-sender that signed this exact body for this route.
fn authorize(
    senders: &KmsTxSenders,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<Address, HttpError> {
    let refused = |message: String| {
        HttpError::new(ErrorCode::SenderAuthenticationFailed, message)
    };
    let Some(set) = senders.current() else {
        return Err(HttpError::new(
            ErrorCode::UpstreamTransient,
            "KMS tx-sender set not read yet",
        ));
    };
    let header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let signer = request_authorization::recover_signer(
        set.chain_id,
        header,
        MERKLE_PROOFS_PATH,
        body,
        now,
    )
    .map_err(|err| refused(err.to_string()))?;
    if !set.senders.contains(&signer) {
        return Err(refused(format!(
            "{signer} is not the tx-sender of a node in a live KMS context"
        )));
    }
    Ok(signer)
}

struct ParsedQuery {
    encrypted_store: [u8; 32],
    handle: [u8; 32],
    kind: LeafKind,
    key: Option<[u8; 32]>,
}

impl ParsedQuery {
    fn parse(query: &LeafQuery) -> Result<Self, HttpError> {
        let (kind, key) = match (query.kind, &query.key) {
            (LeafQueryKind::Allowed, Some(key)) => {
                (LeafKind::HistoricalAccess, Some(parse_hex32("key", key)?))
            }
            (LeafQueryKind::Allowed, None) => {
                return Err(HttpError::malformed("allowed leaf needs a key"))
            }
            (LeafQueryKind::Public, None) => (LeafKind::PublicDecrypt, None),
            (LeafQueryKind::Public, Some(_)) => {
                return Err(HttpError::malformed("public leaf takes no key"))
            }
        };
        Ok(Self {
            encrypted_store: parse_hex32(
                "encryptedStore",
                &query.encrypted_store,
            )?,
            handle: parse_hex32("handle", &query.handle)?,
            kind,
            key,
        })
    }
}

fn parse_hex32(field: &str, value: &str) -> Result<[u8; 32], HttpError> {
    let digits = value.strip_prefix("0x").unwrap_or(value);
    hex::decode(digits)
        .ok()
        .and_then(|bytes| <[u8; 32]>::try_from(bytes).ok())
        .ok_or_else(|| {
            HttpError::malformed(format!("{field}: expected 32 bytes as hex"))
        })
}

/// Three indexed reads: the state row, the first matching leaf below its `leaf_count`,
/// and that leaf's path.
async fn prove(
    pool: &PgPool,
    query: &ParsedQuery,
) -> Result<MerkleProofOutcome, HttpError> {
    let read_failed = |err: sqlx::Error| {
        error!(error = %err, "leaf record read failed");
        HttpError::new(ErrorCode::UpstreamTransient, "leaf record read failed")
    };
    let account = query.encrypted_store;
    let Some(cursor) = load_store_cursor(pool, account)
        .await
        .map_err(read_failed)?
    else {
        return Ok(MerkleProofOutcome::UnknownAccount);
    };
    let leaf_count = cursor.leaf_count;
    let Some((leaf_index, commitment)) = find_leaf(
        pool,
        account,
        query.kind,
        query.handle,
        query.key,
        leaf_count,
    )
    .await
    .map_err(read_failed)?
    else {
        return Ok(MerkleProofOutcome::NotFound { leaf_count });
    };
    let proof = load_proof(pool, account, leaf_index, leaf_count)
        .await
        .map_err(read_failed)?;
    // A path that is missing or does not reach the recorded peaks comes from a record
    // that is wrong, not stale.
    let Some(proof) = proof.filter(|proof| {
        mmr_verify(&cursor.peaks, leaf_count, commitment, proof)
    }) else {
        error!(
            encrypted_store = %bs58::encode(account).into_string(),
            leaf_index,
            leaf_count,
            "leaf record inconsistent"
        );
        return Err(HttpError::new(
            ErrorCode::UpstreamTransient,
            "leaf record inconsistent",
        ));
    };
    Ok(MerkleProofOutcome::Found {
        leaf_index: proof.leaf_index,
        leaf_count,
        siblings: proof.siblings.iter().map(hex::encode).collect(),
    })
}

// --- OpenAPI ----------------------------------------------------------------------

struct RequestAuthorization;

impl Modify for RequestAuthorization {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                "request_authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                    "Authorization",
                    &format!(
                        "`{} expires=<unix seconds>, signature=0x<65 bytes>`: an EIP-712 \
                         `RequestAuthorization(string path, bytes32 bodyDigest, uint64 expires)` \
                         signature, domain `{{name: \"zama-request-authorization\", version: \"1\", \
                         chainId: <canonical ProtocolConfig chain>}}`, by the tx-sender of a node \
                         in a live KMS context. `bodyDigest` is the keccak-256 of the exact body; \
                         `expires` is at most {} s ahead.",
                        request_authorization::SCHEME,
                        request_authorization::MAX_VALIDITY_SECS,
                    ),
                ))),
            );
    }
}

/// The committed document lives at `openapi/solana_merkle_proofs.json`.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Solana Merkle proofs",
        description = "Inclusion proofs from the RFC 035 leaf record of Solana encrypted stores, for the KMS connector.",
        version = "1.0.0",
    ),
    paths(merkle_proofs),
    components(schemas(
        LeafQueryKind,
        LeafQuery,
        MerkleProofRequest,
        MerkleProofOutcome,
        MerkleProofResponse,
        ErrorCode,
        ErrorResponse,
    )),
    modifiers(&RequestAuthorization),
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kms_tx_senders::KmsTxSenderSet;
    use alloy::signers::local::PrivateKeySigner;
    use sqlx::postgres::PgPoolOptions;
    use std::collections::HashSet;

    const OPENAPI_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/openapi/solana_merkle_proofs.json"
    );

    /// The shared spelling of the wire; the connector pins its own against the same file.
    const MERKLE_PROOFS_FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../solana/test-fixtures/merkle-proofs/merkle_proofs_v1.json"
    );

    #[test]
    fn wire_matches_the_shared_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(MERKLE_PROOFS_FIXTURE)
                .expect("read fixture"),
        )
        .expect("fixture is json");
        assert_eq!(
            fixture["maxLeavesPerRequest"],
            serde_json::json!(MAX_LEAVES_PER_REQUEST)
        );
        let request: MerkleProofRequest =
            serde_json::from_value(fixture["request"].clone())
                .expect("the fixture request decodes");
        assert_eq!(
            serde_json::to_value(&request).expect("serialize"),
            fixture["request"]
        );
        assert_eq!(request.leaves.len(), 2);
        for proof in fixture["proofs"].as_array().expect("proofs") {
            let decoded: MerkleProofOutcome =
                serde_json::from_value(proof.clone())
                    .expect("every fixture proof decodes");
            assert_eq!(
                serde_json::to_value(&decoded).expect("serialize"),
                *proof
            );
        }
    }

    /// Regenerate with `UPDATE_OPENAPI=1`.
    #[test]
    fn openapi_document_is_committed() {
        let generated =
            ApiDoc::openapi().to_pretty_json().expect("serialize") + "\n";
        if std::env::var_os("UPDATE_OPENAPI").is_some() {
            std::fs::write(OPENAPI_PATH, &generated).expect("write openapi");
        }
        let committed =
            std::fs::read_to_string(OPENAPI_PATH).expect("read openapi");
        assert_eq!(
            committed, generated,
            "openapi/solana_merkle_proofs.json is stale; rerun with UPDATE_OPENAPI=1"
        );
    }

    #[test]
    fn hex_fields_accept_optional_prefix_and_reject_wrong_length() {
        let bare = "ac".repeat(32);
        assert_eq!(parse_hex32("handle", &bare).ok(), Some([0xAC; 32]));
        assert_eq!(
            parse_hex32("handle", &format!("0x{bare}")).ok(),
            Some([0xAC; 32])
        );
        assert!(parse_hex32("handle", "ac").is_err());
        assert!(parse_hex32("handle", "zz").is_err());
    }

    #[test]
    fn allowed_needs_a_key_and_public_refuses_one() {
        let bare = "ac".repeat(32);
        let mut query = LeafQuery {
            encrypted_store: bare.clone(),
            handle: bare.clone(),
            kind: LeafQueryKind::Allowed,
            key: None,
        };
        assert!(ParsedQuery::parse(&query).is_err());
        query.key = Some(bare);
        assert!(ParsedQuery::parse(&query).is_ok());
        query.kind = LeafQueryKind::Public;
        assert!(ParsedQuery::parse(&query).is_err());
        query.key = None;
        assert!(ParsedQuery::parse(&query).is_ok());
    }

    const CHAIN_ID: u64 = 12345;

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    async fn signed(
        signer: &PrivateKeySigner,
        body: &[u8],
        expires: u64,
    ) -> String {
        request_authorization::authorize(
            signer,
            CHAIN_ID,
            MERKLE_PROOFS_PATH,
            body,
            expires,
        )
        .await
        .expect("sign")
    }

    async fn spawn(
        senders: KmsTxSenders,
    ) -> (String, CancellationToken, tokio::task::JoinHandle<()>) {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://nobody@127.0.0.1:1/none")
            .expect("lazy pool");
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let cancel = CancellationToken::new();
        let server = tokio::spawn({
            let cancel = cancel.clone();
            async move {
                serve(listener, merkle_proofs_router(pool, senders), cancel)
                    .await
                    .expect("serve")
            }
        });
        (format!("http://{addr}"), cancel, server)
    }

    /// Authentication and body validation are answered before any database read,
    /// so a pool that never connects is enough to pin the error contract.
    #[tokio::test]
    async fn rejects_before_touching_the_database() {
        let connector = PrivateKeySigner::random();
        let (base, cancel, server) =
            spawn(KmsTxSenders::fixed(KmsTxSenderSet {
                chain_id: CHAIN_ID,
                senders: HashSet::from([connector.address()]),
            }))
            .await;
        let url = format!("{base}{MERKLE_PROOFS_PATH}");
        let client = reqwest::Client::new();
        let body = serde_json::to_vec(&MerkleProofRequest {
            leaves: vec![LeafQuery {
                encrypted_store: "ac".repeat(32),
                handle: "10".repeat(32),
                kind: LeafQueryKind::Public,
                key: None,
            }],
        })
        .expect("encode");
        let post = |authorization: Option<String>, body: Vec<u8>| {
            let mut request = client.post(&url).body(body);
            if let Some(authorization) = authorization {
                request = request.header(header::AUTHORIZATION, authorization);
            }
            request.send()
        };
        let refused = |response: reqwest::Response| async move {
            assert_eq!(response.status(), 401);
            let error: ErrorResponse =
                response.json().await.expect("error body");
            assert_eq!(error.code, ErrorCode::SenderAuthenticationFailed);
            assert!(!error.retryable);
        };

        refused(post(None, body.clone()).await.expect("send")).await;
        let stranger = PrivateKeySigner::random();
        let by_stranger = signed(&stranger, &body, now() + 60).await;
        refused(post(Some(by_stranger), body.clone()).await.expect("send"))
            .await;
        let expired = signed(&connector, &body, now() - 1).await;
        refused(post(Some(expired), body.clone()).await.expect("send")).await;
        let for_other_body = signed(&connector, b"{}", now() + 60).await;
        refused(
            post(Some(for_other_body), body.clone())
                .await
                .expect("send"),
        )
        .await;

        let not_json = b"{not json".to_vec();
        let authorization = signed(&connector, &not_json, now() + 60).await;
        let response = post(Some(authorization), not_json).await.expect("send");
        assert_eq!(response.status(), 400);
        let error: ErrorResponse = response.json().await.expect("error body");
        assert_eq!(error.code, ErrorCode::Malformed);

        let leaf: serde_json::Value =
            serde_json::from_slice::<serde_json::Value>(&body).unwrap()
                ["leaves"][0]
                .clone();
        let too_many = serde_json::to_vec(&serde_json::json!({
            "leaves": vec![leaf; MAX_LEAVES_PER_REQUEST + 1]
        }))
        .unwrap();
        let authorization = signed(&connector, &too_many, now() + 60).await;
        let response = post(Some(authorization), too_many).await.expect("send");
        assert_eq!(response.status(), 400);

        let too_large = vec![b' '; MAX_REQUEST_BYTES + 1];
        let authorization = signed(&connector, &too_large, now() + 60).await;
        let response =
            post(Some(authorization), too_large).await.expect("send");
        assert_eq!(response.status(), 400);

        let liveness = client
            .get(format!("{base}/liveness"))
            .send()
            .await
            .expect("send");
        assert_eq!(liveness.status(), 200);

        cancel.cancel();
        server.await.expect("join");
    }

    /// Until the KMS tx-sender set is read, the server is not ready and asks callers to retry.
    #[tokio::test]
    async fn refuses_until_the_kms_tx_senders_are_read() {
        let (base, cancel, server) = spawn(KmsTxSenders::unread()).await;
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{base}{MERKLE_PROOFS_PATH}"))
            .body("{}")
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 502);
        let error: ErrorResponse = response.json().await.expect("error body");
        assert_eq!(error.code, ErrorCode::UpstreamTransient);
        assert!(error.retryable);
        let healthz = client
            .get(format!("{base}/healthz"))
            .send()
            .await
            .expect("send");
        assert_eq!(healthz.status(), 503);

        cancel.cancel();
        server.await.expect("join");
    }
}
