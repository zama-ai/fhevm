//! The HTTP routes of the Merkle proof service: the health routes both binaries serve, and the
//! Merkle proof route that only `solana_merkle_proof_server` adds (DD-064).
//!
//! The KMS connector asks for the inclusion proof of the leaf that authorizes a
//! decrypt (an allow of a key on a handle, or a handle made public) and verifies
//! it against the peaks of the on-chain account it read itself. This server only
//! reads the leaf record ([`crate::store`]) and makes no decrypt authorization
//! decision. It answers only callers in [`KmsTxSenders`]: each request carries an
//! `FhevmSig` signature (RFC 038) by a KMS node's tx-sender, for this coprocessor's
//! signer address as audience.
//!
//! The [`AnswerCache`] answers every copy of a signed request with the answer to
//! its first copy, so a server charges the signer and reads the record at most
//! once per signed request (DD-067). An `overloaded` answer read nothing, so it
//! is not kept: a copy sent after it is admitted again.
//!
//! Each KMS tx-sender may ask for [`HttpServer::merkle_proofs`]'s leaves per
//! second and hold its bytes of requests in the cache; past either it is refused
//! `rate_limited` (429). Proof reads take one connection each and leave one of
//! the pool to `/healthz`; a request waits up to [`PROOF_READ_WAIT`] for its
//! turn, then is refused `overloaded` (503). The connector asks another
//! coprocessor at once after either refusal.
//!
//! Request and success bodies are CBOR (RFC 8949), served over HTTP/1.1 or HTTP/2
//! without TLS. Error bodies are the JSON `{ code, message, retryable }` of RFC 033.
//! The wire contract is the committed OpenAPI document in `openapi/`; a test keeps
//! it in sync with the code.

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
    http::{header, HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use prometheus::{
    register_histogram, register_int_counter_vec, register_int_gauge,
    Histogram, IntCounterVec, IntGauge,
};
use request_authorization::{Authorization, AuthorizationError};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::Semaphore};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use zama_solana_acl::mmr_verify;
use zama_solana_merkle_proofs::{
    ErrorCode, ErrorResponse, LeafQuery, LeafQueryKind, MerkleProofOutcome,
    MerkleProofRequest, MerkleProofResponse, MAX_LEAVES_PER_REQUEST,
    MERKLE_PROOFS_PATH,
};

use crate::{
    answer_cache::{Admission, AnswerCache},
    kms_tx_senders::KmsTxSenders,
    store::{
        find_leaf, leaf_commitment, load_proof, load_served_store, LeafKind,
    },
    unix_now_secs,
};

/// Comfortably above [`MAX_LEAVES_PER_REQUEST`] queries.
const MAX_REQUEST_BYTES: usize = 64 * 1024;

const CBOR: &str = "application/cbor";

/// How long a request waits for a database connection before it is refused. Below the KMS
/// connector's 250 ms hedge delay, so a refusal sends the connector to the next coprocessor no
/// later than the hedge would have.
pub const PROOF_READ_WAIT: Duration = Duration::from_millis(200);

/// The `Retry-After` of a `rate_limited` or `overloaded` refusal. The connector asks another
/// coprocessor at once and retries this one on its next round.
const RETRY_AFTER_SECS: u64 = 1;

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

static PROOF_READS_WAITING: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "solana_merkle_proof_server_proof_reads_waiting",
        format!(
            "Requests waiting for a database connection. No request waits longer than \
             {PROOF_READ_WAIT:?} (PROOF_READ_WAIT). One that would is refused overloaded, so \
             {PROOF_READ_WAIT:?} bounds the age of the oldest waiting request"
        )
    )
    .unwrap()
});

static PROOF_READS_IN_FLIGHT: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "solana_merkle_proof_server_proof_reads_in_flight",
        "Requests reading the leaf record, at most the database pool size minus one"
    )
    .unwrap()
});

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    senders: KmsTxSenders,
    /// This coprocessor's signer address, the `FhevmSig` audience.
    audience: Address,
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

    /// The health routes and the Merkle proof route, for the proof server. A
    /// request must be signed for `audience`, this coprocessor's signer address.
    /// Each KMS tx-sender may ask for `leaves_per_second`, in bursts of as many,
    /// and the server remembers `answer_cache_bytes_per_signer` of each one's
    /// signed requests and their answers.
    /// The pool needs at least two connections.
    pub fn merkle_proofs(
        pool: PgPool,
        senders: KmsTxSenders,
        audience: Address,
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
                audience,
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
    audience: Address,
    leaves_per_second: NonZeroU32,
    answer_cache_bytes_per_signer: usize,
    proof_reads: usize,
) -> Router {
    // The outcomes alerts watch and the backlog gauges exist before the first
    // request.
    for outcome in ["inconsistent", "quarantined"] {
        LEAVES.with_label_values(&[outcome]);
    }
    LazyLock::force(&PROOF_READS_WAITING);
    LazyLock::force(&PROOF_READS_IN_FLIGHT);
    // A burst always fits the largest request.
    let burst = leaves_per_second
        .max(NonZeroU32::new(MAX_LEAVES_PER_REQUEST as u32).expect("not zero"));
    let state = AppState {
        pool,
        senders,
        audience,
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

// --- HTTP errors -----------------------------------------------------------------

fn http_status(code: ErrorCode) -> StatusCode {
    match code {
        ErrorCode::Malformed => StatusCode::BAD_REQUEST,
        ErrorCode::AuthExpired | ErrorCode::SenderAuthenticationFailed => {
            StatusCode::UNAUTHORIZED
        }
        ErrorCode::UpstreamTransient => StatusCode::BAD_GATEWAY,
        ErrorCode::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        ErrorCode::Overloaded => StatusCode::SERVICE_UNAVAILABLE,
    }
}

fn retryable(code: ErrorCode) -> bool {
    matches!(
        code,
        ErrorCode::AuthExpired
            | ErrorCode::UpstreamTransient
            | ErrorCode::RateLimited
            | ErrorCode::Overloaded
    )
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
        let mut response = (
            http_status(self.code),
            axum::Json(ErrorResponse {
                code: self.code,
                message: self.message,
                retryable: retryable(self.code),
            }),
        )
            .into_response();
        if matches!(self.code, ErrorCode::RateLimited | ErrorCode::Overloaded) {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, RETRY_AFTER_SECS.into());
        }
        response
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
        (status = 400, description = "Malformed request, or a query string", body = ErrorResponse, content_type = "application/json"),
        (status = 401, description = "Request signature expired (`auth_expired`, retryable) or refused", body = ErrorResponse, content_type = "application/json"),
        (status = 429, description = "Signer over its rate or holding as many signed requests as it may; `Retry-After` set", body = ErrorResponse, content_type = "application/json"),
        (status = 502, description = "Leaf record or KMS tx-sender set unavailable", body = ErrorResponse, content_type = "application/json"),
        (status = 503, description = "No database connection free in time; `Retry-After` set", body = ErrorResponse, content_type = "application/json"),
    ),
    security(("fhevm_sig" = [])),
)]
async fn merkle_proofs(
    State(state): State<AppState>,
    uri: Uri,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Response, HttpError> {
    let started = Instant::now();
    let answer = answer_merkle_proofs(state, &uri, &headers, body).await;
    let error_code;
    let status = match &answer {
        Ok(Served { repeat: true, .. }) => "cached",
        Ok(Served { repeat: false, .. }) => "ok",
        Err(error) => {
            error_code =
                serde_json::to_value(error.code).expect("serialize error code");
            error_code.as_str().expect("error code is a string")
        }
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
    uri: &Uri,
    headers: &HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Result<Served, HttpError> {
    // `FhevmSig` does not sign a query string.
    if uri.query().is_some() {
        return Err(HttpError::malformed("a query string is not signed"));
    }
    let body =
        body.map_err(|rejection| HttpError::malformed(rejection.body_text()))?;
    let now = unix_now_secs();
    let authorization =
        authenticate(&state.senders, state.audience, headers, &body, now)?;
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
        authorization.accepted_until(),
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
                let hash = &authorization.signing_hash;
                match &answer {
                    Err(error) if error.code == ErrorCode::Overloaded => {
                        state.answers.forget(hash)
                    }
                    answer => state
                        .answers
                        .settle(hash, answer.as_ref().map_or(0, Bytes::len)),
                }
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
                ErrorCode::AuthExpired,
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
    PROOF_READS_WAITING.inc();
    let proof_read =
        tokio::time::timeout(PROOF_READ_WAIT, state.proof_reads.acquire())
            .await;
    PROOF_READS_WAITING.dec();
    let Ok(Ok(_proof_read)) = proof_read else {
        return Err(HttpError::new(
            ErrorCode::Overloaded,
            format!("no database connection free within {PROOF_READ_WAIT:?}"),
        ));
    };
    PROOF_READS_IN_FLIGHT.inc();
    let answer = read_proofs(&state.pool, queries).await;
    PROOF_READS_IN_FLIGHT.dec();
    answer
}

async fn read_proofs(pool: &PgPool, queries: Vec<ParsedQuery>) -> Answer {
    let mut proofs = Vec::with_capacity(queries.len());
    for query in queries {
        let Ok(proof) = prove(pool, &query).await else {
            return Err(HttpError::new(
                ErrorCode::UpstreamTransient,
                "leaf record read failed",
            ));
        };
        proofs.push(proof);
    }
    let mut answer = encode_cbor(&MerkleProofResponse { proofs });
    // The cache counts the length; `Bytes` would keep the spare capacity too.
    answer.shrink_to_fit();
    Ok(Bytes::from(answer))
}

fn cbor_response(body: Bytes) -> Response {
    ([(header::CONTENT_TYPE, CBOR)], body).into_response()
}

/// Returns what the KMS tx-sender that signed this exact body for this route and
/// `audience` signed.
fn authenticate(
    senders: &KmsTxSenders,
    audience: Address,
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
        audience,
        now,
    )
    .map_err(|err| match err {
        AuthorizationError::Expired { .. } => {
            HttpError::new(ErrorCode::AuthExpired, err.to_string())
        }
        _ => refused(err.to_string()),
    })?;
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
) -> Result<MerkleProofOutcome, ReadFailed> {
    let outcome = prove_leaf(pool, query).await;
    let label = match &outcome {
        Ok(MerkleProofOutcome::Found { .. }) => "found",
        Ok(MerkleProofOutcome::NotFound { .. }) => "not_found",
        Ok(MerkleProofOutcome::UnknownAccount) => "unknown_store",
        Ok(MerkleProofOutcome::Inconsistent) => "inconsistent",
        Err(LeafError::Quarantined) => "quarantined",
        Err(LeafError::ReadFailed) => "read_failed",
    };
    LEAVES.with_label_values(&[label]).inc();
    match outcome {
        Ok(outcome) => Ok(outcome),
        Err(LeafError::Quarantined) => Ok(MerkleProofOutcome::Inconsistent),
        Err(LeafError::ReadFailed) => Err(ReadFailed),
    }
}

/// A database read failed: the whole request is refused `upstream_transient`.
struct ReadFailed;

/// Why [`prove_leaf`] has no outcome to count. A leaf of a quarantined store is answered
/// [`MerkleProofOutcome::Inconsistent`] but counted `quarantined`, so it pages once, through the
/// store check.
enum LeafError {
    /// The store check found this store's record disagrees with the chain.
    Quarantined,
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
    let inconsistent = |reason: &str| {
        error!(
            encrypted_store = %bs58::encode(account).into_string(),
            leaf_index,
            leaf_count,
            reason,
            "leaf record inconsistent"
        );
        Ok(MerkleProofOutcome::Inconsistent)
    };
    // The row is found by its handle and key, but only its commitment is proven: a row
    // whose columns disagree with it would get a valid path for another grant.
    if commitment
        != leaf_commitment(account, leaf_index, query.handle, query.key)
    {
        return inconsistent("the row does not match its commitment");
    }
    let proof = load_proof(pool, account, leaf_index, leaf_count)
        .await
        .map_err(read_failed)?;
    // A path that is missing or does not reach the recorded peaks comes from a record
    // that is wrong, not stale.
    let Some(proof) = proof.filter(|proof| {
        mmr_verify(&cursor.peaks, leaf_count, commitment, proof)
    }) else {
        return inconsistent(
            "the path is missing or misses the recorded peaks",
        );
    };
    Ok(MerkleProofOutcome::Found {
        leaf_index: proof.leaf_index,
        leaf_count,
        siblings: proof.siblings,
    })
}

// --- OpenAPI ----------------------------------------------------------------------

struct FhevmSigScheme;

impl Modify for FhevmSigScheme {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                "fhevm_sig",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                    "Authorization",
                    &format!(
                        "`{} expires=<unix seconds>, sig=0x<65 bytes>` (RFC 038): an EIP-712 \
                         signature over this path, the exact body, the expiry and this \
                         coprocessor's signer address as audience, by the tx-sender of a node \
                         in a live KMS context of the canonical ProtocolConfig. Accepted up to \
                         {} s after expiry and at most {} s ahead. The \
                         `shared/request-authorization` crate defines the typed data.",
                        request_authorization::SCHEME,
                        request_authorization::CLOCK_SKEW_SECS,
                        request_authorization::MAX_AUTH_VALIDITY_SECS
                            + request_authorization::CLOCK_SKEW_SECS,
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
        description = "Inclusion proofs from the leaf record of Solana encrypted stores, for the KMS connector.",
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
    modifiers(&FhevmSigScheme),
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
                    siblings: vec![[0x5B; 32]],
                },
                MerkleProofOutcome::NotFound { leaf_count: 3 },
                MerkleProofOutcome::UnknownAccount,
                MerkleProofOutcome::Inconsistent,
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

    /// The signer address of the coprocessor under test.
    const AUDIENCE: Address = Address::repeat_byte(0xCC);

    async fn signed_for(
        signer: &PrivateKeySigner,
        body: &[u8],
        expires: u64,
        audience: Address,
    ) -> String {
        request_authorization::authorize(
            signer,
            &REGISTRY,
            MERKLE_PROOFS_PATH,
            body,
            expires,
            audience,
        )
        .await
        .expect("sign")
    }

    async fn signed(
        signer: &PrivateKeySigner,
        body: &[u8],
        expires: u64,
    ) -> String {
        signed_for(signer, body, expires, AUDIENCE).await
    }

    async fn error_of(response: reqwest::Response) -> ErrorResponse {
        assert_eq!(
            response.headers()[header::CONTENT_TYPE],
            "application/json"
        );
        let body = response.bytes().await.expect("error body");
        serde_json::from_slice(&body).expect("a JSON error body")
    }

    async fn spawn(
        senders: KmsTxSenders,
        leaves_per_second: u32,
        answer_cache_bytes_per_signer: usize,
        proof_reads: usize,
    ) -> (String, CancellationToken, tokio::task::JoinHandle<()>) {
        // Nothing listens there, so a read fails once the acquire times out.
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_millis(100))
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
                    AUDIENCE,
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
        let for_other_body =
            signed(&connector, &[0xA0], unix_now_secs() + 30).await;
        refused(
            post(Some(for_other_body), body.clone())
                .await
                .expect("send"),
        )
        .await;
        // Signed for another coprocessor.
        let for_other_audience = signed_for(
            &connector,
            &body,
            unix_now_secs() + 30,
            Address::repeat_byte(0xCD),
        )
        .await;
        refused(
            post(Some(for_other_audience), body.clone())
                .await
                .expect("send"),
        )
        .await;
        let too_long_lived = signed(
            &connector,
            &body,
            unix_now_secs()
                + request_authorization::MAX_AUTH_VALIDITY_SECS
                + request_authorization::CLOCK_SKEW_SECS
                + 60,
        )
        .await;
        refused(
            post(Some(too_long_lived), body.clone())
                .await
                .expect("send"),
        )
        .await;

        // Past its expiry and the clock skew, a signature is refused as expired, which
        // the connector may sign again.
        let expired = signed(
            &connector,
            &body,
            unix_now_secs() - request_authorization::CLOCK_SKEW_SECS - 60,
        )
        .await;
        let response = post(Some(expired), body.clone()).await.expect("send");
        assert_eq!(response.status(), 401);
        let error = error_of(response).await;
        assert_eq!(error.code, ErrorCode::AuthExpired);
        assert!(error.retryable);

        // A query string is not signed, so a request that carries one is refused.
        let authorization =
            signed(&connector, &body, unix_now_secs() + 30).await;
        let response = client
            .post(format!("{url}?leaves=1"))
            .header(header::AUTHORIZATION, authorization)
            .body(body.clone())
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 400);
        assert_eq!(error_of(response).await.code, ErrorCode::Malformed);

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

    /// A refusal the connector takes to another coprocessor: its status, code and
    /// message. It is retryable, and a 429 or 503 sets `Retry-After`.
    async fn refusal(
        client: &reqwest::Client,
        url: &str,
        authorization: &str,
        body: Vec<u8>,
    ) -> (u16, ErrorCode, String) {
        let response = client
            .post(url)
            .header(header::AUTHORIZATION, authorization)
            .body(body)
            .send()
            .await
            .expect("send");
        let retry_after = response
            .headers()
            .get(header::RETRY_AFTER)
            .map(|value| value.to_str().expect("ASCII").to_owned());
        let status = response.status().as_u16();
        let error = error_of(response).await;
        assert!(error.retryable);
        let backs_off = matches!(
            error.code,
            ErrorCode::RateLimited | ErrorCode::Overloaded
        );
        assert_eq!(
            retry_after,
            backs_off.then(|| RETRY_AFTER_SECS.to_string())
        );
        (status, error.code, error.message)
    }

    fn no_connection() -> (u16, ErrorCode, String) {
        (
            503,
            ErrorCode::Overloaded,
            format!("no database connection free within {PROOF_READ_WAIT:?}"),
        )
    }

    fn read_failed() -> (u16, ErrorCode, String) {
        (
            502,
            ErrorCode::UpstreamTransient,
            "leaf record read failed".to_owned(),
        )
    }

    /// Every copy of a signed request, sent at once or later, gets the first copy's
    /// answer, here its failed read: only the first copy spends the signer's burst,
    /// and only a new request is over the rate.
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
            1,
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
        while let Some(answer) = copies.join_next().await {
            assert_eq!(answer.expect("join"), read_failed());
        }
        assert_eq!(
            refusal(&client, &url, &authorization, body).await,
            read_failed()
        );

        let other = full_request([0x11; 32]);
        let authorization =
            signed(&connector, &other, unix_now_secs() + 30).await;
        assert_eq!(
            refusal(&client, &url, &authorization, other).await,
            (
                429,
                ErrorCode::RateLimited,
                format!(
                    "{} is over its leaves per second",
                    connector.address()
                )
            )
        );

        cancel.cancel();
        server.await.expect("join");
    }

    /// An `overloaded` refusal read nothing, so a copy sent after it is admitted
    /// again: charged anew, which puts this signer at one leaf per second over its
    /// rate.
    #[tokio::test]
    async fn a_copy_after_an_overloaded_refusal_is_admitted_again() {
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
        let client = reqwest::Client::new();
        let body = full_request([0x10; 32]);
        let authorization =
            signed(&connector, &body, unix_now_secs() + 30).await;
        assert_eq!(
            refusal(&client, &url, &authorization, body.clone()).await,
            no_connection()
        );
        assert_eq!(
            refusal(&client, &url, &authorization, body).await,
            (
                429,
                ErrorCode::RateLimited,
                format!(
                    "{} is over its leaves per second",
                    connector.address()
                )
            )
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
            siblings: vec![[0x5B; 32]; 20],
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
            1,
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
        // A failed read is kept, so it holds Alice's room.
        assert_eq!(ask(&alice, 0x10).await, read_failed());
        // Alice's burst is spent too: the room is checked first.
        assert_eq!(
            ask(&alice, 0x11).await,
            (
                429,
                ErrorCode::RateLimited,
                format!(
                    "{} holds as many signed requests as it may",
                    alice.address()
                )
            )
        );
        assert_eq!(ask(&bob, 0x12).await, read_failed());

        cancel.cancel();
        server.await.expect("join");
    }
}
