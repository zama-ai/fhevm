//! Axum HTTP surface: liveness, readiness, metrics, OpenAPI, and MMR proof.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use solana_proof_store::SqlProofStore;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::chain::ChainFetcher;
use crate::ingest_health::IngestHealth;
use crate::metrics::{self, metrics_handler};
use crate::proof::{build_proof, ProofError};
use crate::readiness::{evaluate_readiness, ReadinessClass, ReadinessReport};

/// Shared application state. Generic over the chain fetcher so handler tests can
/// inject fakes without a live RPC node.
pub struct AppState<C: ChainFetcher> {
    pub store: SqlProofStore,
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
                metrics::record_proof("corrupt_store");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MmrProofResponse {
                        mmr_proof: None,
                        leaf_count,
                        proof_slot: leaf_count,
                        verified: false,
                        status: "corrupt_store",
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
        (status = 500, description = "Corrupt store / integrity", body = MmrProofResponse),
        (status = 400, description = "Invalid address or leaf index"),
        (status = 404, description = "Lineage not found on chain"),
    ),
    tag = "Proof"
)]
pub async fn mmr_proof_handler<C: ChainFetcher>(
    State(state): State<Arc<AppState<C>>>,
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
pub async fn readiness_handler<C: ChainFetcher>(
    State(state): State<Arc<AppState<C>>>,
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
pub fn router<C: ChainFetcher + 'static>(state: Arc<AppState<C>>) -> Router {
    let api = Router::new()
        .route("/health/liveness", get(liveness_handler))
        .route("/health/readiness", get(readiness_handler::<C>))
        .route("/metrics", get(metrics_handler))
        .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C>))
        .with_state(state);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    /// Router smoke test without a live Postgres: only liveness is exercised.
    #[tokio::test]
    async fn liveness_responds_without_infra() {
        // Build a minimal router fragment (liveness has no state dependency).
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
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "alive");
    }

    #[test]
    fn lagging_maps_to_503_json() {
        let response = HttpError::Proof(ProofError::Lagging { leaf_count: 3 }).into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn corrupt_store_maps_to_500_json() {
        let response = HttpError::Proof(ProofError::CorruptStore { leaf_count: 3 }).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
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
