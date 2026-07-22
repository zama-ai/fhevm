//! Axum HTTP surface: liveness, readiness, metrics, OpenAPI, and MMR proof.

use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::{from_fn_with_state, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::{MakeSpan, TraceLayer};
use tracing::Span;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::chain::ChainFetcher;
use crate::ingest_health::IngestHealth;
use crate::metrics::{self, metrics_handler};
use crate::proof::{build_proof, ProofError, ProofSnapshotSource};
use crate::readiness::{evaluate_readiness, ReadinessClass, ReadinessQueryable, ReadinessReport};

/// Hard ceiling for a single proof HTTP request (includes RPC peak check).
const PROOF_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Proof response wire-format marker. Bump on a breaking DTO change; there is no
/// negotiation machinery in this PoC — consumers assert the version they expect.
const PROOF_FORMAT_VERSION: &str = "v1";

/// Shared application state. Generic over the chain fetcher and snapshot source
/// so handler tests can inject fakes without a live RPC node or Postgres.
pub struct AppState<C: ChainFetcher, S: ProofSnapshotSource> {
    pub store: S,
    pub fetcher: Arc<C>,
    pub ingest: Arc<IngestHealth>,
    /// Proof admission aligned with DB pool capacity minus reserved ops slots.
    pub proof_permits: Arc<Semaphore>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MmrProofQuery {
    /// Base58 lineage (`EncryptedValue` PDA) address.
    pub encrypted_value: String,
    pub leaf_index: u64,
}

/// Serializable mirror of `zama_solana_acl::mmr::MmrProof` (borsh-only on-chain),
/// siblings hex-encoded for JSON transport.
#[derive(Debug, Serialize, ToSchema)]
pub struct MmrProofDto {
    pub leaf_index: u64,
    pub siblings: Vec<String>,
}

impl From<&zama_solana_acl::mmr::MmrProof> for MmrProofDto {
    fn from(proof: &zama_solana_acl::mmr::MmrProof) -> Self {
        Self {
            leaf_index: proof.leaf_index,
            siblings: proof.siblings.iter().map(hex::encode).collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MmrProofResponse {
    pub mmr_proof: Option<MmrProofDto>,
    /// Lineage leaf count the proof was built against.
    pub leaf_count: u64,
    /// Confirmed Solana RPC context slot of the on-chain peak comparison.
    pub rpc_context_slot: u64,
    /// Durable ingest slot at which this lineage's served leaves were last written.
    /// Omitted when no snapshot backed the response (e.g. store not yet ingested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_last_slot: Option<u64>,
    /// Commitment level of the on-chain authorization reads.
    pub commitment: &'static str,
    /// Proof response wire-format version.
    pub proof_format_version: &'static str,
    pub verified: bool,
    pub status: &'static str,
}

/// Typed JSON error envelope for non-proof-shaped failures.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_count: Option<u64>,
}

fn error_json(status: StatusCode, code: &'static str, error: impl Into<String>) -> Response {
    (
        status,
        Json(ErrorResponse {
            error: error.into(),
            code: code.to_owned(),
            leaf_index: None,
            leaf_count: None,
        }),
    )
        .into_response()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LivenessResponse {
    pub status: &'static str,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("invalid encrypted_value address: {0}")]
    InvalidAddress(String),
    #[error(transparent)]
    Proof(#[from] ProofError),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::Proof(ProofError::Lagging {
                leaf_count,
                rpc_context_slot,
                lineage_last_slot,
            }) => {
                metrics::record_proof("lagging");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(MmrProofResponse {
                        mmr_proof: None,
                        leaf_count,
                        rpc_context_slot,
                        lineage_last_slot,
                        commitment: crate::chain::SOLANA_PROOF_COMMITMENT,
                        proof_format_version: PROOF_FORMAT_VERSION,
                        verified: false,
                        status: "lagging",
                    }),
                )
                    .into_response()
            }
            HttpError::Proof(ProofError::CorruptStore {
                leaf_count,
                rpc_context_slot,
                lineage_last_slot,
            }) => {
                metrics::record_proof("corrupt_cache");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MmrProofResponse {
                        mmr_proof: None,
                        leaf_count,
                        rpc_context_slot,
                        lineage_last_slot,
                        commitment: crate::chain::SOLANA_PROOF_COMMITMENT,
                        proof_format_version: PROOF_FORMAT_VERSION,
                        verified: false,
                        status: "corrupt_cache",
                    }),
                )
                    .into_response()
            }
            other => {
                let (status, code, leaf_index, leaf_count) = match &other {
                    HttpError::InvalidAddress(_) => {
                        (StatusCode::BAD_REQUEST, "invalid_address", None, None)
                    }
                    HttpError::Proof(ProofError::LineageNotFound) => {
                        (StatusCode::NOT_FOUND, "lineage_not_found", None, None)
                    }
                    HttpError::Proof(ProofError::LeafIndexOutOfRange {
                        leaf_index,
                        leaf_count,
                    }) => (
                        StatusCode::BAD_REQUEST,
                        "leaf_index_out_of_range",
                        Some(*leaf_index),
                        Some(*leaf_count),
                    ),
                    HttpError::Proof(ProofError::Chain(err)) => {
                        tracing::error!(error = %err, "proof chain fetch failed");
                        (StatusCode::INTERNAL_SERVER_ERROR, "chain_error", None, None)
                    }
                    HttpError::Proof(ProofError::Store(err)) => {
                        tracing::error!(error = %err, "proof store read failed");
                        (StatusCode::INTERNAL_SERVER_ERROR, "store_error", None, None)
                    }
                    HttpError::Proof(ProofError::Lagging { .. })
                    | HttpError::Proof(ProofError::CorruptStore { .. }) => {
                        unreachable!("lagging/corrupt handled above")
                    }
                };
                metrics::record_proof(code);
                let public_error = match status {
                    StatusCode::INTERNAL_SERVER_ERROR => "internal error".to_owned(),
                    _ => other.to_string(),
                };
                (
                    status,
                    Json(ErrorResponse {
                        error: public_error,
                        code: code.to_owned(),
                        leaf_index,
                        leaf_count,
                    }),
                )
                    .into_response()
            }
        }
    }
}

fn decode_solana_address(address: &str) -> Result<[u8; 32], String> {
    let bytes = bs58::decode(address)
        .into_vec()
        .map_err(|err| format!("invalid base58: {err}"))?;
    bytes
        .try_into()
        .map_err(|bytes: Vec<u8>| format!("address must decode to 32 bytes, got {}", bytes.len()))
}

/// GET `/internal/solana/mmr-proof?encrypted_value=<base58>&leaf_index=<u64>`
#[utoipa::path(
    get,
    path = "/internal/solana/mmr-proof",
    params(
        ("encrypted_value" = String, Query, description = "Base58 EncryptedValue PDA"),
        ("leaf_index" = u64, Query, description = "MMR leaf index")
    ),
    responses(
        (status = 200, description = "Verified proof", body = MmrProofResponse),
        (status = 503, description = "Store lagging chain (MmrProofResponse) or proof overloaded (ErrorResponse)"),
        (status = 500, description = "Corrupt cache / integrity", body = MmrProofResponse),
        (status = 408, description = "Proof request timed out", body = ErrorResponse),
        (status = 400, description = "Invalid address or leaf index", body = ErrorResponse),
        (status = 404, description = "Lineage not found on chain", body = ErrorResponse),
    ),
    tag = "Proof"
)]
pub async fn mmr_proof_handler<C: ChainFetcher, S: ProofSnapshotSource>(
    State(state): State<Arc<AppState<C, S>>>,
    Query(query): Query<MmrProofQuery>,
) -> Result<Json<MmrProofResponse>, HttpError> {
    let lineage =
        decode_solana_address(&query.encrypted_value).map_err(HttpError::InvalidAddress)?;
    let result = build_proof(
        state.fetcher.as_ref(),
        &state.store,
        lineage,
        query.leaf_index,
    )
    .await?;
    metrics::record_proof("verified");
    Ok(Json(MmrProofResponse {
        mmr_proof: result.mmr_proof.as_ref().map(MmrProofDto::from),
        leaf_count: result.leaf_count,
        rpc_context_slot: result.rpc_context_slot,
        lineage_last_slot: result.lineage_last_slot,
        commitment: crate::chain::SOLANA_PROOF_COMMITMENT,
        proof_format_version: PROOF_FORMAT_VERSION,
        verified: result.verified,
        status: "verified",
    }))
}

#[utoipa::path(
    get,
    path = "/health/liveness",
    responses((status = 200, description = "Process can respond", body = LivenessResponse)),
    tag = "Health"
)]
pub async fn liveness_handler() -> Json<LivenessResponse> {
    Json(LivenessResponse { status: "alive" })
}

#[utoipa::path(
    get,
    path = "/health/readiness",
    responses(
        (status = 200, description = "Proof-ready", body = ReadinessReport),
        (status = 503, description = "Not proof-ready", body = ReadinessReport),
    ),
    tag = "Health"
)]
pub async fn readiness_handler<C: ChainFetcher, S: ProofSnapshotSource + ReadinessQueryable>(
    State(state): State<Arc<AppState<C, S>>>,
) -> Result<Json<ReadinessReport>, (StatusCode, Json<ReadinessReport>)> {
    let report = evaluate_readiness(&state.store, &state.ingest).await;
    metrics::record_readiness(report.status.as_str());
    if report.ready {
        Ok(Json(report))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, Json(report)))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(liveness_handler, readiness_handler, mmr_proof_handler),
    components(schemas(
        LivenessResponse,
        ReadinessReport,
        ReadinessClass,
        MmrProofQuery,
        MmrProofDto,
        MmrProofResponse,
        ErrorResponse,
    )),
    tags(
        (name = "Health", description = "Liveness and derived readiness"),
        (name = "Proof", description = "Internal MMR proof API")
    ),
    info(
        title = "Solana proof service",
        description = "Internal-only PoC API. TODO(prod): add auth / mTLS / rate limits before any public exposure. Proof construction currently loads the full lineage leaf history (O(n)); production needs persisted MMR nodes or checkpoints.",
        version = "0.1.0"
    )
)]
pub struct ApiDoc;

#[derive(Clone, Copy, Debug)]
struct RequestIdMakeSpan;

impl<B> MakeSpan<B> for RequestIdMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .and_then(|id| id.header_value().to_str().ok())
            .unwrap_or("");
        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            request_id = %request_id,
        )
    }
}

/// Proof-only: shed when saturated (no unbounded queue) and enforce a typed timeout.
///
/// Admission capacity is sized to leave reserved DB connections for ingest and
/// readiness on the shared pool (see [`crate::config::DatabaseConfig::proof_admission_limit`]).
async fn proof_admission<C: ChainFetcher, S: ProofSnapshotSource>(
    State(state): State<Arc<AppState<C, S>>>,
    req: Request,
    next: Next,
) -> Response {
    let Ok(permit) = state.proof_permits.try_acquire() else {
        metrics::record_proof("overloaded");
        return error_json(
            StatusCode::SERVICE_UNAVAILABLE,
            "overloaded",
            "proof concurrency limit reached",
        );
    };

    let response = match tokio::time::timeout(PROOF_REQUEST_TIMEOUT, next.run(req)).await {
        Ok(response) => response,
        Err(_) => {
            metrics::record_proof("timeout");
            error_json(
                StatusCode::REQUEST_TIMEOUT,
                "timeout",
                "proof request timed out",
            )
        }
    };
    drop(permit);
    response
}

fn request_id_layers<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router
        .layer(TraceLayer::new_for_http().make_span_with(RequestIdMakeSpan))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

/// Builds the HTTP router. Mounts OpenAPI + swagger UI.
///
/// Operational endpoints (liveness / readiness / metrics) are outside the proof
/// concurrency and timeout gate. Proof admission is also capped below the DB
/// pool size so ingest/readiness retain reserved connection slots.
pub fn router<C, S>(state: Arc<AppState<C, S>>) -> Router
where
    C: ChainFetcher + 'static,
    S: ProofSnapshotSource + ReadinessQueryable + 'static,
{
    let ops = Router::new()
        .route("/health/liveness", get(liveness_handler))
        .route("/health/readiness", get(readiness_handler::<C, S>))
        .route("/metrics", get(metrics_handler))
        .with_state(Arc::clone(&state));

    let proof = Router::new()
        .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C, S>))
        .layer(from_fn_with_state(
            Arc::clone(&state),
            proof_admission::<C, S>,
        ))
        .with_state(state);

    request_id_layers(
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(ops)
            .merge(proof),
    )
}

/// Proof-only router for unit tests (no readiness / Postgres dependency).
#[cfg(test)]
fn proof_test_router<C, S>(state: Arc<AppState<C, S>>) -> Router
where
    C: ChainFetcher + 'static,
    S: ProofSnapshotSource + 'static,
{
    request_id_layers(
        Router::new()
            .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C, S>))
            .layer(from_fn_with_state(
                Arc::clone(&state),
                proof_admission::<C, S>,
            ))
            .with_state(state),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use solana_proof_store::{ProofSnapshot, StoreError};
    use tower::ServiceExt;
    use zama_solana_acl::mmr::{mmr_append, mmr_peaks_from_leaves};

    use crate::chain::{ChainError, OnChainLineageState};

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn snapshot_with_leaves(lineage: [u8; 32], leaves: Vec<[u8; 32]>) -> ProofSnapshot {
        let peaks = mmr_peaks_from_leaves(&leaves);
        ProofSnapshot {
            lineage,
            current_handle: None,
            subjects: vec![],
            leaf_count: leaves.len() as u64,
            peaks,
            leaves,
            last_slot: 1,
        }
    }

    struct FakeChain {
        states: Mutex<HashMap<[u8; 32], OnChainLineageState>>,
    }

    impl FakeChain {
        fn new() -> Self {
            Self {
                states: Mutex::new(HashMap::new()),
            }
        }

        fn set(&self, lineage: [u8; 32], state: OnChainLineageState) {
            self.states.lock().unwrap().insert(lineage, state);
        }
    }

    #[async_trait]
    impl ChainFetcher for FakeChain {
        async fn get_lineage_state(
            &self,
            address: [u8; 32],
        ) -> Result<Option<OnChainLineageState>, ChainError> {
            Ok(self.states.lock().unwrap().get(&address).cloned())
        }
    }

    /// Read-only fake store: no write / catch-up surface.
    struct FakeStore {
        snapshots: Mutex<HashMap<[u8; 32], ProofSnapshot>>,
        reads: AtomicUsize,
        inconsistent: Mutex<HashMap<[u8; 32], (u64, u64)>>,
    }

    impl FakeStore {
        fn new() -> Self {
            Self {
                snapshots: Mutex::new(HashMap::new()),
                reads: AtomicUsize::new(0),
                inconsistent: Mutex::new(HashMap::new()),
            }
        }

        fn insert(&self, snapshot: ProofSnapshot) {
            self.snapshots
                .lock()
                .unwrap()
                .insert(snapshot.lineage, snapshot);
        }

        fn mark_inconsistent(&self, lineage: [u8; 32], leaf_count: u64, leaf_rows: u64) {
            self.inconsistent
                .lock()
                .unwrap()
                .insert(lineage, (leaf_count, leaf_rows));
        }
    }

    #[async_trait]
    impl ProofSnapshotSource for FakeStore {
        async fn proof_snapshot(
            &self,
            lineage: [u8; 32],
        ) -> Result<Option<ProofSnapshot>, StoreError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if let Some(&(leaf_count, leaf_rows)) = self.inconsistent.lock().unwrap().get(&lineage)
            {
                return Err(StoreError::SnapshotInconsistent {
                    leaf_count,
                    leaf_rows,
                });
            }
            Ok(self.snapshots.lock().unwrap().get(&lineage).cloned())
        }
    }

    async fn json_body(response: Response) -> serde_json::Value {
        let body = axum::body::to_bytes(response.into_body(), 64 * 1024)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    fn proof_uri(lineage: [u8; 32], leaf_index: u64) -> String {
        format!(
            "/internal/solana/mmr-proof?encrypted_value={}&leaf_index={}",
            bs58::encode(lineage).into_string(),
            leaf_index
        )
    }

    fn app_state(chain: FakeChain, store: FakeStore) -> Arc<AppState<FakeChain, FakeStore>> {
        Arc::new(AppState {
            store,
            fetcher: Arc::new(chain),
            ingest: IngestHealth::new(),
            proof_permits: Arc::new(Semaphore::new(8)),
        })
    }

    /// Router smoke test without a live Postgres: only liveness is exercised.
    #[tokio::test]
    async fn liveness_responds_without_infra() {
        let app = Router::new().route("/health/liveness", get(liveness_handler));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health/liveness")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let json = json_body(response).await;
        assert_eq!(json["status"], "alive");
    }

    #[tokio::test]
    async fn handler_verified_json_body() {
        let lineage = pk(0x11);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(lineage, vec![leaf]));

        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let json = json_body(response).await;
        assert_eq!(json["status"], "verified");
        assert_eq!(json["verified"], true);
        assert_eq!(json["leaf_count"], 1);
        assert_eq!(json["rpc_context_slot"], 4242);
        // snapshot_with_leaves pins last_slot = 1.
        assert_eq!(json["lineage_last_slot"], 1);
        assert_eq!(json["commitment"], "confirmed");
        assert_eq!(json["proof_format_version"], "v1");
        assert!(json.get("proof_slot").is_none());
        assert!(json["mmr_proof"].is_object());
    }

    #[tokio::test]
    async fn handler_lagging_json_body() {
        let lineage = pk(0x12);
        let leaf0 = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let leaf1 = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 1, pk(0x11));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf0).unwrap();
        mmr_append(&mut peaks, &mut leaf_count, leaf1).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(lineage, vec![leaf0]));

        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let json = json_body(response).await;
        assert_eq!(json["status"], "lagging");
        assert_eq!(json["verified"], false);
        assert_eq!(json["leaf_count"], 2);
        assert!(json["mmr_proof"].is_null());
    }

    #[tokio::test]
    async fn handler_corrupt_cache_json_body() {
        let lineage = pk(0x13);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let other = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0xAA));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, other).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(lineage, vec![leaf]));

        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let json = json_body(response).await;
        assert_eq!(json["status"], "corrupt_cache");
        assert_eq!(json["verified"], false);
        assert_eq!(json["leaf_count"], 1);
        assert!(json["mmr_proof"].is_null());
    }

    #[tokio::test]
    async fn handler_snapshot_inconsistency_returns_corrupt_cache_json() {
        let lineage = pk(0x14);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.mark_inconsistent(lineage, 1, 0);

        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let json = json_body(response).await;
        assert_eq!(json["status"], "corrupt_cache");
        assert_eq!(json["verified"], false);
        assert_eq!(json["leaf_count"], 1);
    }

    #[tokio::test]
    async fn handler_lineage_missing_is_404_json_error() {
        let lineage = pk(0x15);
        let chain = FakeChain::new();
        let store = FakeStore::new();
        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(response.headers().get("x-request-id").is_some());
        let json = json_body(response).await;
        assert_eq!(json["code"], "lineage_not_found");
        assert!(json["error"].is_string());
    }

    #[tokio::test]
    async fn handler_leaf_index_out_of_range_is_400_json_error() {
        let lineage = pk(0x17);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(lineage, vec![leaf]));

        let app = proof_test_router(app_state(chain, store));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 100))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let json = json_body(response).await;
        assert_eq!(json["code"], "leaf_index_out_of_range");
        assert_eq!(json["leaf_index"], 100);
        assert_eq!(json["leaf_count"], 1);
    }

    #[tokio::test]
    async fn handler_does_not_write_or_catch_up() {
        let lineage = pk(0x16);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(
            lineage,
            OnChainLineageState {
                peaks,
                leaf_count,
                rpc_context_slot: 4242,
            },
        );
        let store = FakeStore::new();
        store.insert(snapshot_with_leaves(lineage, vec![leaf]));

        let state = app_state(chain, store);
        let app = proof_test_router(Arc::clone(&state));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(proof_uri(lineage, 0))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        // Only snapshot reads; FakeStore exposes no write/catch-up methods.
        assert_eq!(state.store.reads.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn lagging_maps_to_503_json_body() {
        let response = HttpError::Proof(ProofError::Lagging {
            leaf_count: 3,
            rpc_context_slot: 77,
            lineage_last_slot: Some(9),
        })
        .into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = json_body(response).await;
        assert_eq!(body["status"], "lagging");
        assert_eq!(body["verified"], false);
        assert_eq!(body["leaf_count"], 3);
        assert_eq!(body["rpc_context_slot"], 77);
        assert_eq!(body["lineage_last_slot"], 9);
        assert_eq!(body["commitment"], "confirmed");
        assert_eq!(body["proof_format_version"], "v1");
    }

    #[tokio::test]
    async fn corrupt_store_maps_to_500_corrupt_cache_json_body() {
        let response = HttpError::Proof(ProofError::CorruptStore {
            leaf_count: 3,
            rpc_context_slot: 77,
            lineage_last_slot: None,
        })
        .into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = json_body(response).await;
        assert_eq!(body["status"], "corrupt_cache");
        assert_eq!(body["verified"], false);
        assert_eq!(body["leaf_count"], 3);
        assert_eq!(body["rpc_context_slot"], 77);
        // lineage_last_slot omitted when no snapshot backed the error.
        assert!(body.get("lineage_last_slot").is_none());
    }

    #[test]
    fn invalid_address_decode() {
        assert!(decode_solana_address("not-base58!!!").is_err());
        let pk = [7u8; 32];
        let encoded = bs58::encode(pk).into_string();
        assert_eq!(decode_solana_address(&encoded).unwrap(), pk);
    }

    #[test]
    fn openapi_documents_proof_route() {
        let doc = ApiDoc::openapi();
        let json = serde_json::to_value(doc).unwrap();
        assert!(json["paths"]["/internal/solana/mmr-proof"].is_object());
        assert!(json["paths"]["/health/readiness"].is_object());
    }
}
