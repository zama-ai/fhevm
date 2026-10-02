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
//! Each KMS tx-sender gets [`HttpServer::merkle_proofs`]'s requests per second,
//! and at most one request per database connection reads the record at a time.
//! Both refusals are `rateLimited` and come before any database read, so the
//! connector asks another coprocessor at once.
//!
//! Bodies are CBOR (RFC 8949), served over HTTP/1.1 or HTTP/2 without TLS. The
//! wire contract is the committed OpenAPI document in `openapi/`; a test keeps
//! it in sync with the code. Errors use the RFC 033 body shape.

use std::{
    net::SocketAddr,
    num::NonZeroU32,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::primitives::Address;
use axum::{
    body::Bytes,
    extract::{rejection::BytesRejection, DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_bytes::ByteArray;
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::Semaphore};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
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

const CBOR: &str = "application/cbor";

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    senders: KmsTxSenders,
    per_kms_tx_sender: Arc<DefaultKeyedRateLimiter<Address>>,
    proof_reads: Arc<Semaphore>,
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

    /// The health routes and the Merkle proof route, for the proof server. Each
    /// KMS tx-sender may send `requests_per_second`, in bursts of as many.
    pub fn merkle_proofs(
        pool: PgPool,
        senders: KmsTxSenders,
        requests_per_second: NonZeroU32,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        // A request reads one query at a time, so it holds one connection.
        let proof_reads = pool.options().get_max_connections() as usize;
        Self {
            router: merkle_proofs_router(
                pool,
                senders,
                requests_per_second,
                proof_reads,
            ),
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

fn merkle_proofs_router(
    pool: PgPool,
    senders: KmsTxSenders,
    requests_per_second: NonZeroU32,
    proof_reads: usize,
) -> Router {
    let state = AppState {
        pool,
        senders,
        per_kms_tx_sender: Arc::new(RateLimiter::keyed(Quota::per_second(
            requests_per_second,
        ))),
        proof_reads: Arc::new(Semaphore::new(proof_reads)),
    };
    Router::new()
        .route("/healthz", get(proof_server_healthz))
        .route("/liveness", get(liveness))
        .route(
            MERKLE_PROOFS_PATH,
            post(merkle_proofs).layer(DefaultBodyLimit::max(MAX_REQUEST_BYTES)),
        )
        .with_state(state)
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

/// One leaf to prove. Byte fields are 32-byte CBOR byte strings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeafQuery {
    /// The encrypted store account whose MMR holds the leaf.
    #[serde(with = "serde_bytes")]
    #[schema(value_type = String, format = Binary)]
    pub encrypted_store: [u8; 32],
    #[serde(with = "serde_bytes")]
    #[schema(value_type = String, format = Binary)]
    pub handle: [u8; 32],
    pub kind: LeafQueryKind,
    /// The allowed key; required for `allowed`, absent for `public`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_bytes"
    )]
    #[schema(value_type = Option<String>, format = Binary)]
    pub key: Option<[u8; 32]>,
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
        /// Authentication path from the leaf to its peak, 32-byte byte strings.
        #[schema(value_type = Vec<String>)]
        siblings: Vec<ByteArray<32>>,
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
    /// Malformed body, missing key, too many leaves.
    Malformed,
    /// The request signature is missing, malformed or expired, or its signer is
    /// not the tx-sender of a node in a live KMS context.
    SenderAuthenticationFailed,
    /// The leaf record could not be read or is inconsistent, or the KMS
    /// tx-sender set is not read yet; retry later.
    UpstreamTransient,
    /// The signer is over its requests per second, or every database
    /// connection is reading; ask another coprocessor or retry later.
    RateLimited,
}

impl ErrorCode {
    fn http_status(self) -> StatusCode {
        match self {
            Self::Malformed => StatusCode::BAD_REQUEST,
            Self::SenderAuthenticationFailed => StatusCode::UNAUTHORIZED,
            Self::UpstreamTransient => StatusCode::BAD_GATEWAY,
            Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn retryable(self) -> bool {
        matches!(self, Self::UpstreamTransient | Self::RateLimited)
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
            Cbor(ErrorResponse {
                code: self.code,
                message: self.message,
                retryable: self.code.retryable(),
            }),
        )
            .into_response()
    }
}

/// An `application/cbor` response body.
struct Cbor<T>(T);

impl<T: Serialize> IntoResponse for Cbor<T> {
    fn into_response(self) -> Response {
        ([(header::CONTENT_TYPE, CBOR)], encode_cbor(&self.0)).into_response()
    }
}

fn encode_cbor(value: &impl Serialize) -> Vec<u8> {
    let mut body = Vec::new();
    ciborium::into_writer(value, &mut body)
        .expect("the wire types have no fallible fields");
    body
}

/// Decodes exactly one CBOR item filling `body`.
fn decode_cbor<T: DeserializeOwned>(body: &[u8]) -> Result<T, HttpError> {
    let mut rest = body;
    let value = ciborium::from_reader(&mut rest)
        .map_err(|err| HttpError::malformed(format!("not CBOR: {err}")))?;
    if !rest.is_empty() {
        return Err(HttpError::malformed(format!(
            "{} bytes after the CBOR body",
            rest.len()
        )));
    }
    Ok(value)
}

// --- Merkle proofs -----------------------------------------------------------------

/// Builds the inclusion proof of each queried leaf from the leaf record.
#[utoipa::path(
    post,
    path = MERKLE_PROOFS_PATH,
    tag = "solana",
    request_body(content = MerkleProofRequest, content_type = "application/cbor"),
    responses(
        (status = 200, description = "One answer per queried leaf, in request order", body = MerkleProofResponse, content_type = "application/cbor"),
        (status = 400, description = "Malformed request", body = ErrorResponse, content_type = "application/cbor"),
        (status = 401, description = "Request signature refused", body = ErrorResponse, content_type = "application/cbor"),
        (status = 429, description = "Signer over its rate, or every database connection reading", body = ErrorResponse, content_type = "application/cbor"),
        (status = 502, description = "Leaf record or KMS tx-sender set unavailable", body = ErrorResponse, content_type = "application/cbor"),
    ),
    security(("request_authorization" = [])),
)]
async fn merkle_proofs(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Cbor<MerkleProofResponse>, HttpError> {
    let body =
        body.map_err(|rejection| HttpError::malformed(rejection.body_text()))?;
    let signer = authenticate(&state.senders, &headers, &body)?;
    if state.per_kms_tx_sender.check_key(&signer).is_err() {
        return Err(HttpError::new(
            ErrorCode::RateLimited,
            format!("{signer} is over its requests per second"),
        ));
    }
    let request: MerkleProofRequest = decode_cbor(&body)?;
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
    // Refused at once rather than queued: the connector asks the next
    // coprocessor after its hedge delay anyway.
    let Ok(_proof_read) = state.proof_reads.try_acquire() else {
        return Err(HttpError::new(
            ErrorCode::RateLimited,
            "every database connection is reading",
        ));
    };
    let mut proofs = Vec::with_capacity(queries.len());
    for query in queries {
        proofs.push(prove(&state.pool, &query).await?);
    }
    Ok(Cbor(MerkleProofResponse { proofs }))
}

/// Returns the KMS tx-sender that signed this exact body for this route.
fn authenticate(
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
        &set.registry,
        header,
        MERKLE_PROOFS_PATH,
        body,
        now,
    )
    .map_err(|err| refused(err.to_string()))?;
    if !set.senders.contains(&signer) {
        return Err(refused(format!(
            "{signer} is not the tx-sender of a node in a live KMS context of \
             ProtocolConfig {} on chain {}",
            set.registry.contract, set.registry.chain_id
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
        let (kind, key) = match (query.kind, query.key) {
            (LeafQueryKind::Allowed, Some(key)) => {
                (LeafKind::HistoricalAccess, Some(key))
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
            encrypted_store: query.encrypted_store,
            handle: query.handle,
            kind,
            key,
        })
    }
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
        siblings: proof.siblings.into_iter().map(ByteArray::new).collect(),
    })
}

// --- OpenAPI ----------------------------------------------------------------------

struct RequestAuthorizationScheme;

impl Modify for RequestAuthorizationScheme {
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
                         signature over this path and the exact body, by the tx-sender of a node \
                         in a live KMS context of the canonical ProtocolConfig, valid for at most \
                         {} s. The `shared/request-authorization` crate defines the typed data.",
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
    modifiers(&RequestAuthorizationScheme),
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kms_tx_senders::KmsTxSenderSet;
    use alloy::signers::local::PrivateKeySigner;
    use request_authorization::KeyRegistry;
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

    /// The server decodes the connector's request bytes and encodes the answer bytes the
    /// connector decodes.
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
        let hex_field = |name: &str| {
            hex::decode(fixture[name].as_str().expect("hex string"))
                .expect("hex")
        };
        let request: MerkleProofRequest = decode_cbor(&hex_field("request"))
            .ok()
            .expect("the fixture request decodes");
        assert_eq!(
            request.leaves,
            [
                LeafQuery {
                    encrypted_store: [0xAC; 32],
                    handle: [0x10; 32],
                    kind: LeafQueryKind::Allowed,
                    key: Some([0xA1; 32]),
                },
                LeafQuery {
                    encrypted_store: [0xAC; 32],
                    handle: [0x11; 32],
                    kind: LeafQueryKind::Public,
                    key: None,
                },
            ]
        );
        let response = MerkleProofResponse {
            proofs: vec![
                MerkleProofOutcome::Found {
                    leaf_index: 1,
                    leaf_count: 3,
                    siblings: vec![ByteArray::new([0x5B; 32])],
                },
                MerkleProofOutcome::NotFound { leaf_count: 3 },
                MerkleProofOutcome::UnknownAccount,
            ],
        };
        assert_eq!(
            hex::encode(encode_cbor(&response)),
            fixture["response"].as_str().expect("hex string")
        );
    }

    #[test]
    fn a_body_with_trailing_bytes_is_malformed() {
        let mut body = encode_cbor(&MerkleProofRequest { leaves: vec![] });
        assert!(decode_cbor::<MerkleProofRequest>(&body).is_ok());
        body.push(0);
        assert!(decode_cbor::<MerkleProofRequest>(&body).is_err());
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
    fn allowed_needs_a_key_and_public_refuses_one() {
        let mut query = LeafQuery {
            encrypted_store: [0xAC; 32],
            handle: [0xAC; 32],
            kind: LeafQueryKind::Allowed,
            key: None,
        };
        assert!(ParsedQuery::parse(&query).is_err());
        query.key = Some([0xAC; 32]);
        assert!(ParsedQuery::parse(&query).is_ok());
        query.kind = LeafQueryKind::Public;
        assert!(ParsedQuery::parse(&query).is_err());
        query.key = None;
        assert!(ParsedQuery::parse(&query).is_ok());
    }

    const REGISTRY: KeyRegistry = KeyRegistry {
        chain_id: 12345,
        contract: Address::repeat_byte(0xC0),
    };

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
            &REGISTRY,
            MERKLE_PROOFS_PATH,
            body,
            expires,
        )
        .await
        .expect("sign")
    }

    async fn error_of(response: reqwest::Response) -> ErrorResponse {
        assert_eq!(response.headers()[header::CONTENT_TYPE], CBOR);
        let body = response.bytes().await.expect("error body");
        decode_cbor(&body).ok().expect("a CBOR error body")
    }

    async fn spawn(
        senders: KmsTxSenders,
        requests_per_second: u32,
        proof_reads: usize,
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
                let router = merkle_proofs_router(
                    pool,
                    senders,
                    NonZeroU32::new(requests_per_second).expect("a rate"),
                    proof_reads,
                );
                serve(listener, router, cancel).await.expect("serve")
            }
        });
        (format!("http://{addr}"), cancel, server)
    }

    /// Authentication and body validation are answered before any database read,
    /// so a pool that never connects is enough to pin the error contract.
    #[tokio::test]
    async fn rejects_before_touching_the_database() {
        let connector = PrivateKeySigner::random();
        let (base, cancel, server) = spawn(
            KmsTxSenders::fixed(KmsTxSenderSet {
                registry: REGISTRY,
                senders: HashSet::from([connector.address()]),
            }),
            1000,
            1,
        )
        .await;
        let url = format!("{base}{MERKLE_PROOFS_PATH}");
        // As the KMS connector connects.
        let client = reqwest::Client::builder()
            .http2_prior_knowledge()
            .build()
            .expect("client");
        let leaf = LeafQuery {
            encrypted_store: [0xAC; 32],
            handle: [0x10; 32],
            kind: LeafQueryKind::Public,
            key: None,
        };
        let body = encode_cbor(&MerkleProofRequest {
            leaves: vec![leaf.clone()],
        });
        let post = |authorization: Option<String>, body: Vec<u8>| {
            let mut request = client.post(&url).body(body);
            if let Some(authorization) = authorization {
                request = request.header(header::AUTHORIZATION, authorization);
            }
            request.send()
        };
        let refused = |response: reqwest::Response| async move {
            assert_eq!(response.version(), reqwest::Version::HTTP_2);
            assert_eq!(response.status(), 401);
            let error = error_of(response).await;
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
        let for_other_body = signed(&connector, &[0xA0], now() + 60).await;
        refused(
            post(Some(for_other_body), body.clone())
                .await
                .expect("send"),
        )
        .await;

        let not_cbor = b"{\"leaves\":[]}".to_vec();
        let authorization = signed(&connector, &not_cbor, now() + 60).await;
        let response = post(Some(authorization), not_cbor).await.expect("send");
        assert_eq!(response.status(), 400);
        assert_eq!(error_of(response).await.code, ErrorCode::Malformed);

        let too_many = encode_cbor(&MerkleProofRequest {
            leaves: vec![leaf; MAX_LEAVES_PER_REQUEST + 1],
        });
        let authorization = signed(&connector, &too_many, now() + 60).await;
        let response = post(Some(authorization), too_many).await.expect("send");
        assert_eq!(response.status(), 400);

        let too_large = vec![0; MAX_REQUEST_BYTES + 1];
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
        let (base, cancel, server) = spawn(KmsTxSenders::unread(), 1, 1).await;
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{base}{MERKLE_PROOFS_PATH}"))
            .body(vec![0xA0])
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 502);
        let error = error_of(response).await;
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

    /// A signer over its rate, and a request finding every database connection
    /// reading, are refused before the database, as retryable.
    #[tokio::test]
    async fn refuses_over_the_rate_and_when_every_connection_reads() {
        let connector = PrivateKeySigner::random();
        let (base, cancel, server) = spawn(
            KmsTxSenders::fixed(KmsTxSenderSet {
                registry: REGISTRY,
                senders: HashSet::from([connector.address()]),
            }),
            1,
            0,
        )
        .await;
        let client = reqwest::Client::builder()
            .http2_prior_knowledge()
            .build()
            .expect("client");
        let body = encode_cbor(&MerkleProofRequest {
            leaves: vec![LeafQuery {
                encrypted_store: [0xAC; 32],
                handle: [0x10; 32],
                kind: LeafQueryKind::Public,
                key: None,
            }],
        });
        let authorization = signed(&connector, &body, now() + 60).await;
        let mut messages = Vec::new();
        for _ in 0..2 {
            let response = client
                .post(format!("{base}{MERKLE_PROOFS_PATH}"))
                .header(header::AUTHORIZATION, &authorization)
                .body(body.clone())
                .send()
                .await
                .expect("send");
            assert_eq!(response.status(), 429);
            let error = error_of(response).await;
            assert_eq!(error.code, ErrorCode::RateLimited);
            assert!(error.retryable);
            messages.push(error.message);
        }
        assert_eq!(messages[0], "every database connection is reading");
        assert_eq!(
            messages[1],
            format!("{} is over its requests per second", connector.address())
        );

        cancel.cancel();
        server.await.expect("join");
    }
}
