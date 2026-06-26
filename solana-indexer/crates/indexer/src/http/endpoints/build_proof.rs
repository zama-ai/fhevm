//! `POST /build_proof` — serves a mode-prefixed MMR inclusion proof for a leaf.
//!
//! Resolves `value_key` -> PDA via the initialize mapping, loads the ordered
//! events, reconstructs the leaf list, and builds the proof. When an RPC client
//! is configured it cross-checks the reconstruction against the live PDA's
//! on-chain `(peaks, leaf_count)`; otherwise the proof is returned with
//! `verified = false` and a warning is logged.

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};
use utoipa::ToSchema;

use crate::http::error::AppError;
use crate::http::AppState;
use crate::lineage::proof::{self, ProofError};

#[derive(Debug, Deserialize, ToSchema)]
pub struct BuildProofRequest {
    /// 32-byte `acl_nonce_key`, hex-encoded.
    pub value_key: String,
    pub leaf_index: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BuildProofResponse {
    /// `mode_prefix (0x01 historical / 0x02 public) ‖ Borsh(MmrProof)`, hex-encoded.
    pub mmr_proof_bytes: String,
    pub leaf_count: u64,
    /// Whether the proof was cross-checked against the live on-chain account.
    pub verified: bool,
}

pub struct BuildProofHandler;

impl BuildProofHandler {
    pub fn routes(&self, _state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new().route("/build_proof", post(handle))
    }
}

#[utoipa::path(
    post,
    path = "/build_proof",
    request_body = BuildProofRequest,
    responses(
        (status = 200, body = BuildProofResponse),
        (status = 404, description = "lineage_not_found | lineage_has_no_leaves"),
        (status = 422, description = "leaf_index_out_of_range"),
    )
)]
pub async fn handle(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BuildProofRequest>,
) -> Result<impl IntoResponse, AppError> {
    let value_key = parse_key(&req.value_key, "value_key")?;

    let pda = state
        .repo
        .pda_for_value_key(&value_key)
        .await?
        .ok_or(AppError::LineageNotFound)?;

    let events = state.repo.events_for_pda(&pda).await?;
    if events.is_empty() {
        // The lineage is known (PDA resolved) but no rotation/mark has produced a
        // leaf yet — distinct from an unknown value_key.
        return Err(AppError::LineageHasNoLeaves);
    }

    // Best-effort on-chain cross-check.
    let on_chain = match &state.rpc {
        Some(rpc) => match rpc.account_data(pda).await {
            Some(data) => proof::on_chain_peaks_from_account(&data),
            None => {
                state.metrics.rpc_verify_failures.inc();
                warn!(
                    pda = hex::encode(pda),
                    "on-chain cross-check unavailable; serving unverified proof"
                );
                None
            }
        },
        None => None,
    };

    let built = proof::build(pda, &events, req.leaf_index, on_chain).map_err(|e| match e {
        ProofError::LeafIndexOutOfRange(..) => AppError::LeafIndexOutOfRange,
        ProofError::Reconstruct(inner) => {
            // Detail is logged server-side; the client sees only a generic 500.
            AppError::Internal(format!("reconstruct failed: {inner:?}"))
        }
        ProofError::Encode(inner) => AppError::Internal(format!("proof encode failed: {inner:#}")),
    })?;

    state.metrics.proofs_served.inc();
    info!(
        pda = hex::encode(pda),
        leaf_index = req.leaf_index,
        leaf_count = built.leaf_count,
        verified = built.verified,
        "build_proof served"
    );

    Ok(Json(BuildProofResponse {
        mmr_proof_bytes: hex::encode(built.bytes),
        leaf_count: built.leaf_count,
        verified: built.verified,
    }))
}

/// Hex-decodes a 32-byte key, accepting an optional `0x` prefix.
pub fn parse_key(raw: &str, field: &str) -> Result<[u8; 32], AppError> {
    let stripped = raw.strip_prefix("0x").unwrap_or(raw);
    let bytes = hex::decode(stripped)
        .map_err(|_| AppError::BadRequest(format!("{field} is not valid hex")))?;
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| AppError::BadRequest(format!("{field} must be 32 bytes")))
}
