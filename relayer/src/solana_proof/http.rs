//! Internal HTTP endpoint exposing `proof::build_proof`.
//!
//! There is no existing Solana user-decrypt HTTP path in the relayer yet (the
//! gateway/host modules are EVM-only today), so this stands alone as an
//! internal route rather than plugging into an existing handler. Mount
//! `router(service)` from `http/server.rs` (e.g.
//! `.merge(solana_proof::http::router(service))`) once a Solana request path
//! exists; until then it is reachable but not wired into `run_http_server`.
//! In-process callers (the future orchestrator) should call
//! [`crate::solana_proof::build_proof`] directly instead of going through HTTP.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::solana_proof::chain::{ChainFetcher, RpcChainFetcher};
use crate::solana_proof::proof::{build_proof, ProofError};
use crate::solana_proof::store::{FileLeafStore, LeafStore};

/// Bundles the fetcher/store/program-id a running deployment needs to answer
/// proof requests, generic over both so tests can inject fakes.
pub struct SolanaProofService<C: ChainFetcher, S: LeafStore> {
    pub fetcher: C,
    pub store: S,
    pub program_id: [u8; 32],
}

pub type DefaultSolanaProofService = SolanaProofService<RpcChainFetcher, FileLeafStore>;

#[derive(Debug, Deserialize)]
pub struct MmrProofQuery {
    /// Base58 lineage (`EncryptedValue` PDA) address.
    pub encrypted_value: String,
    pub leaf_index: u64,
}

/// Serializable mirror of `zama_solana_acl::mmr::MmrProof` (which only derives
/// `borsh`, not `serde`), siblings hex-encoded for JSON transport.
#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct MmrProofResponse {
    pub mmr_proof: Option<MmrProofDto>,
    pub leaf_count: u64,
    pub verified: bool,
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
        let status = match &self {
            HttpError::InvalidAddress(_) => axum::http::StatusCode::BAD_REQUEST,
            HttpError::Proof(ProofError::LineageNotFound) => axum::http::StatusCode::NOT_FOUND,
            HttpError::Proof(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

pub async fn mmr_proof_handler<C: ChainFetcher, S: LeafStore>(
    State(service): State<Arc<SolanaProofService<C, S>>>,
    Query(query): Query<MmrProofQuery>,
) -> Result<Json<MmrProofResponse>, HttpError> {
    let lineage = crate::http::utils::decode_solana_address(&query.encrypted_value)
        .map_err(|e| HttpError::InvalidAddress(e.to_string()))?;
    let result = build_proof(
        &service.fetcher,
        &service.store,
        service.program_id,
        lineage,
        query.leaf_index,
    )
    .await?;
    Ok(Json(MmrProofResponse {
        mmr_proof: result.mmr_proof.as_ref().map(MmrProofDto::from),
        leaf_count: result.leaf_count,
        verified: result.verified,
    }))
}

/// Router exposing `GET /internal/solana/mmr-proof?encrypted_value=<base58>&leaf_index=<u64>`.
pub fn router<C: ChainFetcher + 'static, S: LeafStore + 'static>(
    service: Arc<SolanaProofService<C, S>>,
) -> Router {
    Router::new()
        .route("/internal/solana/mmr-proof", get(mmr_proof_handler::<C, S>))
        .with_state(service)
}
