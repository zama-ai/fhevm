//! Axum HTTP surface: liveness, readiness, metrics, OpenAPI, and MMR proof.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::chain::ChainFetcher;
use crate::ingest_health::IngestHealth;
use crate::metrics::{self, metrics_handler};
use crate::proof::{build_proof, ProofError, ProofSnapshotSource};
use crate::readiness::{evaluate_readiness, ReadinessClass, ReadinessQueryable, ReadinessReport};

/// Shared application state. Generic over the chain fetcher and snapshot source
/// so handler tests can inject fakes without a live RPC node or Postgres.
pub struct AppState<C: ChainFetcher, S: ProofSnapshotSource> {
    pub store: S,
    pub fetcher: Arc<C>,
    pub ingest: Arc<IngestHealth>,
    pub max_ingest_silence: std::time::Duration,
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
    pub leaf_count: u64,
    pub proof_slot: u64,
    pub verified: bool,
    pub status: &'static str,
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
            HttpError::Proof(ProofError::Lagging { leaf_count }) => {
                metrics::record_proof("lagging");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(MmrProofResponse {
                        mmr_proof: None,
                        leaf_count,
                        proof_slot: leaf_count,
                        verified: false,
                        status: "lagging",
                    }),
                )
                    .into_response()
            }
            HttpError::Proof(ProofError::CorruptStore { leaf_count }) => {
                // Wire name preserved for relayer DTO parity until #1721.
                metrics::record_proof("corrupt_cache");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MmrProofResponse {
                        mmr_proof: None,
                        leaf_count,
                        proof_slot: leaf_count,
                        verified: false,
                        status: "corrupt_cache",
                    }),
                )
                    .into_response()
            }
            other => {
                let (status, label) = match &other {
                    HttpError::InvalidAddress(_) => (StatusCode::BAD_REQUEST, "invalid_address"),
                    HttpError::Proof(ProofError::LineageNotFound) => {
                        (StatusCode::NOT_FOUND, "lineage_not_found")
                    }
                    HttpError::Proof(ProofError::LeafIndexOutOfRange { .. }) => {
                        (StatusCode::BAD_REQUEST, "leaf_index_out_of_range")
                    }
                    HttpError::Proof(ProofError::Chain(_))
                    | HttpError::Proof(ProofError::Store(_)) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "proof_error")
                    }
                    HttpError::Proof(ProofError::Lagging { .. })
                    | HttpError::Proof(ProofError::CorruptStore { .. }) => {
                        unreachable!("lagging/corrupt handled above")
                    }
                };
                metrics::record_proof(label);
                (status, other.to_string()).into_response()
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
        (status = 503, description = "Store lagging chain", body = MmrProofResponse),
        (status = 500, description = "Corrupt cache / integrity", body = MmrProofResponse),
        (status = 400, description = "Invalid address or leaf index"),
        (status = 404, description = "Lineage not found on chain"),
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
        proof_slot: result.proof_slot,
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
    let report = evaluate_readiness(&state.store, &state.ingest, state.max_ingest_silence).await;
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
    )),
    tags(
        (name = "Health", description = "Liveness and derived readiness"),
        (name = "Proof", description = "Internal MMR proof API")
    ),
    info(
        title = "Solana proof service",
        description = "Internal-only PoC API. TODO(prod): add auth / mTLS / rate limits before any public exposure.",
        version = "0.1.0"
    )
)]
pub struct ApiDoc;

/// Builds the HTTP router. Mounts OpenAPI + swagger UI.
pub fn router<C, S>(state: Arc<AppState<C, S>>) -> Router
where
    C: ChainFetcher + 'static,
    S: ProofSnapshotSource + ReadinessQueryable + 'static,
{
    let api = Router::new()
        .route("/health/liveness", get(liveness_handler))
        .route("/health/readiness", get(readiness_handler::<C, S>))
        .route("/metrics", get(metrics_handler))
        .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C, S>))
        .with_state(state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api)
}

/// Proof-only router for unit tests (no readiness / Postgres dependency).
#[cfg(test)]
fn proof_test_router<C, S>(state: Arc<AppState<C, S>>) -> Router
where
    C: ChainFetcher + 'static,
    S: ProofSnapshotSource + 'static,
{
    Router::new()
        .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C, S>))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    use std::time::Duration;

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
            max_ingest_silence: Duration::from_secs(60),
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
        chain.set(lineage, OnChainLineageState { peaks, leaf_count });
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
        assert_eq!(json["proof_slot"], 1);
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
        chain.set(lineage, OnChainLineageState { peaks, leaf_count });
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
        chain.set(lineage, OnChainLineageState { peaks, leaf_count });
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
        chain.set(lineage, OnChainLineageState { peaks, leaf_count });
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
    async fn handler_lineage_missing_is_404() {
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
    }

    #[tokio::test]
    async fn handler_does_not_write_or_catch_up() {
        let lineage = pk(0x16);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let chain = FakeChain::new();
        chain.set(lineage, OnChainLineageState { peaks, leaf_count });
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
        let response = HttpError::Proof(ProofError::Lagging { leaf_count: 3 }).into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = json_body(response).await;
        assert_eq!(body["status"], "lagging");
        assert_eq!(body["verified"], false);
        assert_eq!(body["leaf_count"], 3);
    }

    #[tokio::test]
    async fn corrupt_store_maps_to_500_corrupt_cache_json_body() {
        let response = HttpError::Proof(ProofError::CorruptStore { leaf_count: 3 }).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = json_body(response).await;
        assert_eq!(body["status"], "corrupt_cache");
        assert_eq!(body["verified"], false);
        assert_eq!(body["leaf_count"], 3);
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
