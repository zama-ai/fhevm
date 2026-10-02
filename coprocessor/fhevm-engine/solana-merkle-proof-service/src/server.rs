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
//! A signature names no recipient, so whoever received a request can resend it
//! to the other coprocessors while it is valid. The [`AnswerCache`] answers every
//! copy of a signed request with the answer to its first copy, so a server charges
//! the signer and reads the record at most once per signed request (DD-067).
//!
//! Each KMS tx-sender may ask for [`HttpServer::merkle_proofs`]'s leaves per
//! second and hold its bytes of requests in the cache. Proof reads take one
//! connection each and leave one of the pool to `/healthz`; a request waits up
//! to [`PROOF_READ_WAIT`] for its turn. These refusals are `rate_limited`,
//! before any database read, and the connector asks another coprocessor at
//! once.
//!
//! Bodies are CBOR (RFC 8949), served over HTTP/1.1 or HTTP/2 without TLS. The
//! wire contract is the committed OpenAPI document in `openapi/`; a test keeps
//! it in sync with the code. Errors use the RFC 033 body shape.

use std::{
    net::SocketAddr,
    num::NonZeroU32,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
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
use prometheus::{
    register_histogram, register_int_counter_vec, Histogram, IntCounterVec,
};
use request_authorization::Authorization;
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
    answer_cache::{Admission, AnswerCache},
    kms_tx_senders::KmsTxSenders,
    store::{find_leaf, load_proof, load_served_store, LeafKind},
    unix_now_secs,
};

/// Most leaves one request may ask for.
const MAX_LEAVES_PER_REQUEST: usize = 64;

/// Comfortably above [`MAX_LEAVES_PER_REQUEST`] queries.
const MAX_REQUEST_BYTES: usize = 64 * 1024;

pub const MERKLE_PROOFS_PATH: &str = "/v1/solana/merkle-proofs";

const CBOR: &str = "application/cbor";

/// How long a request waits for a database connection before it is refused. Below the KMS
/// connector's 250 ms hedge delay, so a refusal sends the connector to the next coprocessor no
/// later than the hedge would have.
pub const PROOF_READ_WAIT: Duration = Duration::from_millis(200);

static REQUESTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "solana_merkle_proof_server_requests_total",
        "Merkle proof requests answered, by status: ok, cached (a copy of a request whose \
         answer was ok) or the error code, which a copy of a refused request counts under too",
        &["status"]
    )
    .unwrap()
});

static REQUEST_DURATION: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram!(
        "solana_merkle_proof_server_request_duration_seconds",
        "Time to answer a Merkle proof request"
    )
    .unwrap()
});

static REQUESTS_BY_SIGNER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "solana_merkle_proof_server_requests_by_signer_total",
        "Authenticated Merkle proof requests, by the KMS tx-sender that signed them: each KMS \
         node's load on this server",
        &["signer"]
    )
    .unwrap()
});

static LEAVES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "solana_merkle_proof_server_leaves_total",
        "Queried leaves, by outcome: found, not_found, unknown_store, quarantined, \
         inconsistent or read_failed",
        &["outcome"]
    )
    .unwrap()
});

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    senders: KmsTxSenders,
    answers: Arc<AnswerCache<Answer>>,
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
    /// KMS tx-sender may ask for `leaves_per_second`, in bursts of as many, and
    /// the server remembers `answer_cache_bytes_per_signer` of each one's signed
    /// requests and their answers.
    /// The pool needs at least two connections.
    pub fn merkle_proofs(
        pool: PgPool,
        senders: KmsTxSenders,
        leaves_per_second: NonZeroU32,
        answer_cache_bytes_per_signer: usize,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        // A request reads one query at a time, so it holds one connection. One
        // connection is left for `/healthz`.
        let proof_reads =
            pool.options().get_max_connections().saturating_sub(1) as usize;
        Self {
            router: merkle_proofs_router(
                pool,
                senders,
                leaves_per_second,
                answer_cache_bytes_per_signer,
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
    leaves_per_second: NonZeroU32,
    answer_cache_bytes_per_signer: usize,
    proof_reads: usize,
) -> Router {
    // The outcomes alerts watch exist before the first one is counted.
    for outcome in ["inconsistent", "quarantined"] {
        LEAVES.with_label_values(&[outcome]);
    }
    // A burst always fits the largest request.
    let burst = leaves_per_second
        .max(NonZeroU32::new(MAX_LEAVES_PER_REQUEST as u32).expect("not zero"));
    let state = AppState {
        pool,
        senders,
        answers: Arc::new(AnswerCache::new(answer_cache_bytes_per_signer)),
        per_kms_tx_sender: Arc::new(RateLimiter::keyed(
            Quota::per_second(leaves_per_second).allow_burst(burst),
        )),
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
    /// The signer is over its leaves per second or holds as many signed
    /// requests as it may, or no database connection freed in time; ask another
    /// coprocessor or retry later.
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

    /// The code as the wire spells it, for the request metric.
    fn label(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::SenderAuthenticationFailed => "sender_authentication_failed",
            Self::UpstreamTransient => "upstream_transient",
            Self::RateLimited => "rate_limited",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}

#[derive(Clone)]
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
        (status = 429, description = "Signer over its rate or holding as many signed requests as it may, or no database connection free in time", body = ErrorResponse, content_type = "application/cbor"),
        (status = 502, description = "Leaf record or KMS tx-sender set unavailable", body = ErrorResponse, content_type = "application/cbor"),
    ),
    security(("request_authorization" = [])),
)]
async fn merkle_proofs(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Response, HttpError> {
    let started = Instant::now();
    let answer = answer_merkle_proofs(state, &headers, body).await;
    let status = match &answer {
        Ok(Served { repeat: true, .. }) => "cached",
        Ok(Served { repeat: false, .. }) => "ok",
        Err(error) => error.code.label(),
    };
    REQUESTS.with_label_values(&[status]).inc();
    REQUEST_DURATION.observe(started.elapsed().as_secs_f64());
    answer.map(|served| served.response)
}

struct Served {
    response: Response,
    /// A copy of a request already admitted.
    repeat: bool,
}

async fn answer_merkle_proofs(
    state: AppState,
    headers: &HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Served, HttpError> {
    let body =
        body.map_err(|rejection| HttpError::malformed(rejection.body_text()))?;
    let now = unix_now_secs();
    let authorization = authenticate(&state.senders, headers, &body, now)?;
    REQUESTS_BY_SIGNER
        .with_label_values(&[&authorization.signer.to_string()])
        .inc();
    // Parsing costs neither the rate nor a read, so a malformed request is
    // refused each time and never remembered.
    let queries = parse_request(&body)?;
    let signer = authorization.signer;
    // An empty request costs one leaf. The burst fits the largest request, so
    // `check_key_n` never reports insufficient capacity.
    let leaves =
        NonZeroU32::new(queries.len() as u32).unwrap_or(NonZeroU32::MIN);
    let charge = || {
        matches!(
            state.per_kms_tx_sender.check_key_n(&signer, leaves),
            Ok(Ok(()))
        )
    };
    let admission = state.answers.admit(
        authorization.signing_hash,
        authorization.expires,
        signer,
        now,
        charge,
    );
    let repeat = matches!(admission, Admission::Repeat(_));
    let mut answer = match admission {
        Admission::First(sender) => {
            let answer = sender.subscribe();
            // Spawned, so a caller that disconnects does not cancel the answer
            // the request's other copies wait for.
            tokio::spawn(async move {
                let answer = answer_request(&state, queries).await;
                state.answers.settle(
                    &authorization.signing_hash,
                    answer.as_ref().map_or(0, Bytes::len),
                );
                sender.send_replace(Some(answer));
            });
            answer
        }
        Admission::Repeat(answer) => answer,
        Admission::Full => {
            return Err(HttpError::new(
                ErrorCode::RateLimited,
                format!("{signer} holds as many signed requests as it may"),
            ))
        }
        Admission::OverRate => {
            return Err(HttpError::new(
                ErrorCode::RateLimited,
                format!("{signer} is over its leaves per second"),
            ))
        }
        Admission::Expired => {
            return Err(HttpError::new(
                ErrorCode::SenderAuthenticationFailed,
                "request authorization expired",
            ))
        }
    };
    let answer = answer
        .wait_for(Option::is_some)
        .await
        .map_err(|_| {
            HttpError::new(
                ErrorCode::UpstreamTransient,
                "the request was not answered",
            )
        })?
        .clone()
        .expect("waited for an answer");
    answer.map(|body| Served {
        response: cbor_response(body),
        repeat,
    })
}

/// What the server answers a request's first copy: the CBOR body of a 200, or the
/// refusal.
type Answer = Result<Bytes, HttpError>;

fn parse_request(body: &[u8]) -> Result<Vec<ParsedQuery>, HttpError> {
    let request: MerkleProofRequest = decode_cbor(body)?;
    if request.leaves.len() > MAX_LEAVES_PER_REQUEST {
        return Err(HttpError::malformed(format!(
            "at most {MAX_LEAVES_PER_REQUEST} leaves per request, got {}",
            request.leaves.len()
        )));
    }
    request.leaves.iter().map(ParsedQuery::parse).collect()
}

async fn answer_request(state: &AppState, queries: Vec<ParsedQuery>) -> Answer {
    let Ok(Ok(_proof_read)) =
        tokio::time::timeout(PROOF_READ_WAIT, state.proof_reads.acquire())
            .await
    else {
        return Err(HttpError::new(
            ErrorCode::RateLimited,
            format!("no database connection free within {PROOF_READ_WAIT:?}"),
        ));
    };
    let mut proofs = Vec::with_capacity(queries.len());
    for query in queries {
        proofs.push(prove(&state.pool, &query).await?);
    }
    let mut answer = encode_cbor(&MerkleProofResponse { proofs });
    // The cache counts the length; `Bytes` would keep the spare capacity too.
    answer.shrink_to_fit();
    Ok(Bytes::from(answer))
}

fn cbor_response(body: Bytes) -> Response {
    ([(header::CONTENT_TYPE, CBOR)], body).into_response()
}

/// Returns what the KMS tx-sender that signed this exact body for this route signed.
fn authenticate(
    senders: &KmsTxSenders,
    headers: &HeaderMap,
    body: &[u8],
    now: u64,
) -> Result<Authorization, HttpError> {
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
    let authorization = request_authorization::recover_authorization(
        &set.registry,
        header,
        MERKLE_PROOFS_PATH,
        body,
        now,
    )
    .map_err(|err| refused(err.to_string()))?;
    if !set.senders.contains(&authorization.signer) {
        return Err(refused(format!(
            "{} is not the tx-sender of a node in a live KMS context of \
             ProtocolConfig {} on chain {}",
            authorization.signer, set.registry.contract, set.registry.chain_id
        )));
    }
    Ok(authorization)
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
    let outcome = prove_leaf(pool, query).await;
    let label = match &outcome {
        Ok(MerkleProofOutcome::Found { .. }) => "found",
        Ok(MerkleProofOutcome::NotFound { .. }) => "not_found",
        Ok(MerkleProofOutcome::UnknownAccount) => "unknown_store",
        Err(LeafError::Quarantined) => "quarantined",
        Err(LeafError::Inconsistent) => "inconsistent",
        Err(LeafError::ReadFailed) => "read_failed",
    };
    LEAVES.with_label_values(&[label]).inc();
    outcome.map_err(|err| {
        let message = match err {
            LeafError::ReadFailed => "leaf record read failed",
            LeafError::Quarantined | LeafError::Inconsistent => {
                "leaf record inconsistent"
            }
        };
        HttpError::new(ErrorCode::UpstreamTransient, message)
    })
}

/// Why a leaf has no answer. All are `upstream_transient`, which the connector retries on
/// another coprocessor.
enum LeafError {
    /// The store check found this store's record disagrees with the chain.
    Quarantined,
    /// A path that is missing or does not reach the recorded peaks.
    Inconsistent,
    ReadFailed,
}

async fn prove_leaf(
    pool: &PgPool,
    query: &ParsedQuery,
) -> Result<MerkleProofOutcome, LeafError> {
    let read_failed = |err: sqlx::Error| {
        error!(error = %err, "leaf record read failed");
        LeafError::ReadFailed
    };
    let account = query.encrypted_store;
    let Some(store) = load_served_store(pool, account)
        .await
        .map_err(read_failed)?
    else {
        return Ok(MerkleProofOutcome::UnknownAccount);
    };
    if store.quarantined {
        return Err(LeafError::Quarantined);
    }
    let cursor = store.cursor;
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
        return Err(LeafError::Inconsistent);
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
    use crate::{
        answer_cache::ENTRY_OVERHEAD_BYTES, kms_tx_senders::KmsTxSenderSet,
    };
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
        assert_eq!(fixture["path"], MERKLE_PROOFS_PATH);
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
        leaves_per_second: u32,
        answer_cache_bytes_per_signer: usize,
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
                    NonZeroU32::new(leaves_per_second).expect("a rate"),
                    answer_cache_bytes_per_signer,
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
            1 << 20,
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
        let by_stranger = signed(&stranger, &body, unix_now_secs() + 30).await;
        refused(post(Some(by_stranger), body.clone()).await.expect("send"))
            .await;
        let expired = signed(&connector, &body, unix_now_secs() - 1).await;
        refused(post(Some(expired), body.clone()).await.expect("send")).await;
        let for_other_body =
            signed(&connector, &[0xA0], unix_now_secs() + 30).await;
        refused(
            post(Some(for_other_body), body.clone())
                .await
                .expect("send"),
        )
        .await;

        let not_cbor = b"{\"leaves\":[]}".to_vec();
        let authorization =
            signed(&connector, &not_cbor, unix_now_secs() + 30).await;
        let response = post(Some(authorization), not_cbor).await.expect("send");
        assert_eq!(response.status(), 400);
        assert_eq!(error_of(response).await.code, ErrorCode::Malformed);

        let too_many = encode_cbor(&MerkleProofRequest {
            leaves: vec![leaf; MAX_LEAVES_PER_REQUEST + 1],
        });
        let authorization =
            signed(&connector, &too_many, unix_now_secs() + 30).await;
        let response = post(Some(authorization), too_many).await.expect("send");
        assert_eq!(response.status(), 400);

        let too_large = vec![0; MAX_REQUEST_BYTES + 1];
        let authorization =
            signed(&connector, &too_large, unix_now_secs() + 30).await;
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
        let (base, cancel, server) =
            spawn(KmsTxSenders::unread(), 1, 1 << 20, 1).await;
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

    fn full_request(handle: [u8; 32]) -> Vec<u8> {
        encode_cbor(&MerkleProofRequest {
            leaves: vec![
                LeafQuery {
                    encrypted_store: [0xAC; 32],
                    handle,
                    kind: LeafQueryKind::Public,
                    key: None,
                };
                MAX_LEAVES_PER_REQUEST
            ],
        })
    }

    async fn refusal(
        client: &reqwest::Client,
        url: &str,
        authorization: &str,
        body: Vec<u8>,
    ) -> String {
        let response = client
            .post(url)
            .header(header::AUTHORIZATION, authorization)
            .body(body)
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 429);
        let error = error_of(response).await;
        assert_eq!(error.code, ErrorCode::RateLimited);
        assert!(error.retryable);
        error.message
    }

    /// Every copy of a signed request, sent at once or later, gets the first copy's
    /// answer, here its refusal for want of a database connection: only the first
    /// copy spends the signer's burst, and only a new request is over the rate.
    #[tokio::test]
    async fn copies_of_a_request_share_its_first_answer_and_cost() {
        let connector = PrivateKeySigner::random();
        let (base, cancel, server) = spawn(
            KmsTxSenders::fixed(KmsTxSenderSet {
                registry: REGISTRY,
                senders: HashSet::from([connector.address()]),
            }),
            1,
            1 << 20,
            0,
        )
        .await;
        let url = format!("{base}{MERKLE_PROOFS_PATH}");
        let client = reqwest::Client::builder()
            .http2_prior_knowledge()
            .build()
            .expect("client");
        let body = full_request([0x10; 32]);
        let authorization =
            signed(&connector, &body, unix_now_secs() + 30).await;
        let mut copies = tokio::task::JoinSet::new();
        for _ in 0..10 {
            let (client, url, authorization, body) = (
                client.clone(),
                url.clone(),
                authorization.clone(),
                body.clone(),
            );
            copies.spawn(async move {
                refusal(&client, &url, &authorization, body).await
            });
        }
        let no_connection =
            format!("no database connection free within {PROOF_READ_WAIT:?}");
        while let Some(message) = copies.join_next().await {
            assert_eq!(message.expect("join"), no_connection);
        }
        assert_eq!(
            refusal(&client, &url, &authorization, body).await,
            no_connection
        );

        let other = full_request([0x11; 32]);
        let authorization =
            signed(&connector, &other, unix_now_secs() + 30).await;
        assert_eq!(
            refusal(&client, &url, &authorization, other).await,
            format!("{} is over its leaves per second", connector.address())
        );

        cancel.cancel();
        server.await.expect("join");
    }

    /// The size `--answer-cache-mib-per-kms-tx-sender` is documented with.
    #[test]
    fn a_64_leaf_answer_with_20_hash_paths_holds_under_48_kib() {
        let found = MerkleProofOutcome::Found {
            leaf_index: 999_999,
            leaf_count: 1_000_000,
            siblings: vec![ByteArray::new([0x5B; 32]); 20],
        };
        let answer = encode_cbor(&MerkleProofResponse {
            proofs: vec![found; MAX_LEAVES_PER_REQUEST],
        });
        assert!(
            answer.len() + ENTRY_OVERHEAD_BYTES < 48 * 1024,
            "{}",
            answer.len()
        );
    }

    /// A signer holding as many requests as it may is refused before any charge, and
    /// refuses only itself. A malformed request is never remembered.
    #[tokio::test]
    async fn a_signer_at_its_bytes_refuses_only_itself() {
        let (alice, bob) =
            (PrivateKeySigner::random(), PrivateKeySigner::random());
        let (base, cancel, server) = spawn(
            KmsTxSenders::fixed(KmsTxSenderSet {
                registry: REGISTRY,
                senders: HashSet::from([alice.address(), bob.address()]),
            }),
            1,
            ENTRY_OVERHEAD_BYTES,
            0,
        )
        .await;
        let url = format!("{base}{MERKLE_PROOFS_PATH}");
        let client = reqwest::Client::new();
        for not_cbor in [b"{}".to_vec(), b"[]".to_vec()] {
            let authorization =
                signed(&alice, &not_cbor, unix_now_secs() + 30).await;
            let response = client
                .post(&url)
                .header(header::AUTHORIZATION, authorization)
                .body(not_cbor)
                .send()
                .await
                .expect("send");
            assert_eq!(response.status(), 400);
        }
        let ask = |signer: &PrivateKeySigner, handle: u8| {
            let (client, url, signer) =
                (client.clone(), url.clone(), signer.clone());
            async move {
                let body = full_request([handle; 32]);
                let authorization =
                    signed(&signer, &body, unix_now_secs() + 30).await;
                refusal(&client, &url, &authorization, body).await
            }
        };
        let no_connection =
            format!("no database connection free within {PROOF_READ_WAIT:?}");
        assert_eq!(ask(&alice, 0x10).await, no_connection);
        // Alice's burst is spent too: the room is checked first.
        assert_eq!(
            ask(&alice, 0x11).await,
            format!(
                "{} holds as many signed requests as it may",
                alice.address()
            )
        );
        assert_eq!(ask(&bob, 0x12).await, no_connection);

        cancel.cancel();
        server.await.expect("join");
    }
}
