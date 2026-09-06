//! The leaf-proof HTTP interface of the Solana host listener.
//!
//! The KMS connector asks for the inclusion proof of the leaf that authorizes a
//! decrypt (an allow of a key on a handle, or a handle made public) and verifies
//! it against the peaks of the on-chain account it read itself. This server only
//! reads the leaf record (`database::solana_leaves`): it holds no chain client and
//! makes no authorization decision. Requests carry a bearer API key.
//!
//! The wire contract is the committed OpenAPI document in `openapi/`; a test keeps
//! it in sync with the code. Errors use the RFC 033 body shape.

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{rejection::JsonRejection, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use zama_solana_acl::{mmr_build_proof, mmr_peaks_from_leaves};

use crate::database::solana_leaves::{load_recorded_leaves, LeafKind};

/// Most leaves one request may ask for.
pub const MAX_LEAVES_PER_REQUEST: usize = 64;

pub const LEAF_PROOFS_PATH: &str = "/v1/solana/leaf-proofs";

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    api_key: Arc<str>,
}

pub struct HttpServer {
    state: AppState,
    port: u16,
    cancel_token: CancellationToken,
}

impl HttpServer {
    pub fn new(
        pool: PgPool,
        api_key: String,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            state: AppState {
                pool,
                api_key: api_key.into(),
            },
            port,
            cancel_token,
        }
    }

    /// Serves until the cancellation token fires.
    pub async fn start(&self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;
        info!("Starting HTTP server on {}", addr);
        serve(listener, self.state.clone(), self.cancel_token.clone()).await
    }
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/liveness", get(liveness))
        .route(LEAF_PROOFS_PATH, post(leaf_proofs))
        .with_state(state)
}

async fn serve(
    listener: TcpListener,
    state: AppState,
    cancel_token: CancellationToken,
) -> anyhow::Result<()> {
    let shutdown = async move { cancel_token.cancelled().await };
    axum::serve(listener, router(state).into_make_service())
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

async fn healthz(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            error!(error = %err, "database health check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
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
    /// The encrypted value account whose MMR holds the leaf.
    pub encrypted_value_account: String,
    pub handle: String,
    pub kind: LeafQueryKind,
    /// The allowed key; required for `allowed`, absent for `public`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeafProofRequest {
    /// At most [`MAX_LEAVES_PER_REQUEST`] entries; answered in order.
    pub leaves: Vec<LeafQuery>,
}

/// The answer for one queried leaf, in request order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum LeafProof {
    /// The leaf is recorded. `peaks` and `leafCount` are the record's state the
    /// proof was built against; the caller verifies against the on-chain account
    /// and retries when the record is behind the chain (`leafCount` smaller).
    Found {
        leaf_index: u64,
        leaf_count: u64,
        peaks: Vec<String>,
        /// Authentication path from the leaf to its peak.
        siblings: Vec<String>,
    },
    /// The account is recorded but no such leaf is, at `leafCount` leaves. Either
    /// it was never sealed or the record has not reached the block that sealed it.
    NotFound { leaf_count: u64 },
    /// The record never saw this account.
    UnknownAccount,
    /// The account was first seen through an update, so its earlier leaves are
    /// unknown and no proof can be served for it.
    HistoryIncomplete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeafProofResponse {
    pub proofs: Vec<LeafProof>,
}

/// Error codes, in the RFC 033 vocabulary.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, bad hex, missing key, too many leaves.
    Malformed,
    /// Missing or wrong bearer API key.
    SenderAuthenticationFailed,
    /// The leaf record could not be read or is inconsistent; retry later.
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

// --- Leaf proofs -----------------------------------------------------------------

/// Builds the inclusion proof of each queried leaf from the leaf record.
#[utoipa::path(
    post,
    path = LEAF_PROOFS_PATH,
    tag = "solana",
    request_body = LeafProofRequest,
    responses(
        (status = 200, description = "One answer per queried leaf, in request order", body = LeafProofResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 401, description = "Missing or wrong API key", body = ErrorResponse),
        (status = 502, description = "Leaf record unavailable", body = ErrorResponse),
    ),
    security(("bearer" = [])),
)]
async fn leaf_proofs(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Result<Json<LeafProofRequest>, JsonRejection>,
) -> Result<Json<LeafProofResponse>, HttpError> {
    authenticate(&headers, &state.api_key)?;
    let Json(request) =
        body.map_err(|rejection| HttpError::malformed(rejection.body_text()))?;
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
    Ok(Json(LeafProofResponse { proofs }))
}

fn authenticate(headers: &HeaderMap, api_key: &str) -> Result<(), HttpError> {
    let presented = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| {
            HttpError::new(
                ErrorCode::SenderAuthenticationFailed,
                "missing bearer API key",
            )
        })?;
    if constant_time_eq(presented.as_bytes(), api_key.as_bytes()) {
        Ok(())
    } else {
        Err(HttpError::new(
            ErrorCode::SenderAuthenticationFailed,
            "wrong API key",
        ))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

struct ParsedQuery {
    encrypted_value_account: [u8; 32],
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
            encrypted_value_account: parse_hex32(
                "encryptedValueAccount",
                &query.encrypted_value_account,
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

async fn prove(
    pool: &PgPool,
    query: &ParsedQuery,
) -> Result<LeafProof, HttpError> {
    let recorded = load_recorded_leaves(pool, query.encrypted_value_account)
        .await
        .map_err(|err| {
            error!(error = %err, "leaf record read failed");
            HttpError::new(
                ErrorCode::UpstreamTransient,
                "leaf record read failed",
            )
        })?;
    let Some(recorded) = recorded else {
        return Ok(LeafProof::UnknownAccount);
    };
    if !recorded.state.history_complete {
        return Ok(LeafProof::HistoryIncomplete);
    }
    let leaf_count = recorded.state.leaf_count;
    let Some(leaf) = recorded.leaves.iter().find(|leaf| {
        leaf.kind == query.kind
            && leaf.handle == query.handle
            && leaf.key == query.key
    }) else {
        return Ok(LeafProof::NotFound { leaf_count });
    };
    let commitments: Vec<[u8; 32]> =
        recorded.leaves.iter().map(|leaf| leaf.commitment).collect();
    // The record stores every leaf below `leaf_count` and the peaks those leaves
    // imply; a proof from a record that fails this check would be wrong, not stale.
    let inconsistent = || {
        error!(
            encrypted_value_account = %bs58::encode(query.encrypted_value_account).into_string(),
            leaf_count,
            recorded_leaves = commitments.len(),
            "leaf record inconsistent"
        );
        HttpError::new(ErrorCode::UpstreamTransient, "leaf record inconsistent")
    };
    if commitments.len() as u64 != leaf_count
        || mmr_peaks_from_leaves(&commitments) != recorded.state.peaks
    {
        return Err(inconsistent());
    }
    let proof = mmr_build_proof(&commitments, leaf.leaf_index)
        .ok_or_else(inconsistent)?;
    Ok(LeafProof::Found {
        leaf_index: proof.leaf_index,
        leaf_count,
        peaks: recorded.state.peaks.iter().map(hex::encode).collect(),
        siblings: proof.siblings.iter().map(hex::encode).collect(),
    })
}

// --- OpenAPI ----------------------------------------------------------------------

struct BearerApiKey;

impl Modify for BearerApiKey {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
                "bearer",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
    }
}

/// The committed document lives at `openapi/solana_leaf_proofs.json`.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Solana host listener leaf proofs",
        description = "Inclusion proofs from the RFC 035 leaf record of Solana encrypted value accounts, for the KMS connector.",
        version = "1.0.0",
    ),
    paths(leaf_proofs),
    components(schemas(
        LeafQueryKind,
        LeafQuery,
        LeafProofRequest,
        LeafProof,
        LeafProofResponse,
        ErrorCode,
        ErrorResponse,
    )),
    modifiers(&BearerApiKey),
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    const OPENAPI_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/openapi/solana_leaf_proofs.json"
    );

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
            "openapi/solana_leaf_proofs.json is stale; rerun with UPDATE_OPENAPI=1"
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
            encrypted_value_account: bare.clone(),
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

    /// Authentication and body validation are answered before any database read,
    /// so a pool that never connects is enough to pin the error contract.
    #[tokio::test]
    async fn rejects_before_touching_the_database() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://nobody@127.0.0.1:1/none")
            .expect("lazy pool");
        let state = AppState {
            pool,
            api_key: "secret".into(),
        };
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let cancel = CancellationToken::new();
        let server = tokio::spawn(serve(listener, state, cancel.clone()));
        let url = format!("http://{addr}{LEAF_PROOFS_PATH}");
        let client = reqwest::Client::new();
        let body = LeafProofRequest {
            leaves: vec![LeafQuery {
                encrypted_value_account: "ac".repeat(32),
                handle: "10".repeat(32),
                kind: LeafQueryKind::Public,
                key: None,
            }],
        };

        let response =
            client.post(&url).json(&body).send().await.expect("send");
        assert_eq!(response.status(), 401);
        let error: ErrorResponse = response.json().await.expect("error body");
        assert_eq!(error.code, ErrorCode::SenderAuthenticationFailed);
        assert!(!error.retryable);

        let response = client
            .post(&url)
            .bearer_auth("wrong")
            .json(&body)
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 401);

        let response = client
            .post(&url)
            .bearer_auth("secret")
            .header(header::CONTENT_TYPE, "application/json")
            .body("{not json")
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 400);
        let error: ErrorResponse = response.json().await.expect("error body");
        assert_eq!(error.code, ErrorCode::Malformed);

        let too_many = LeafProofRequest {
            leaves: vec![body.leaves[0].clone(); MAX_LEAVES_PER_REQUEST + 1],
        };
        let response = client
            .post(&url)
            .bearer_auth("secret")
            .json(&too_many)
            .send()
            .await
            .expect("send");
        assert_eq!(response.status(), 400);

        let liveness = client
            .get(format!("http://{addr}/liveness"))
            .send()
            .await
            .expect("send");
        assert_eq!(liveness.status(), 200);

        cancel.cancel();
        server.await.expect("join").expect("serve");
    }
}
