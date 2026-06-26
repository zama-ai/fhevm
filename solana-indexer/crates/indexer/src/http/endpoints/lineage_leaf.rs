//! `GET /lineage/{value_key}/leaf?subject=&handle=` — finds the historical-access
//! leaf index for a `(subject, handle)` pair within the reconstructed list, so the
//! SDK can pick the `leaf_index` to pass to `/build_proof`.

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use utoipa::{IntoParams, ToSchema};

use crate::http::endpoints::build_proof::parse_key;
use crate::http::error::AppError;
use crate::http::AppState;

#[derive(Debug, Deserialize, IntoParams)]
pub struct LeafQuery {
    /// 32-byte subject pubkey, hex-encoded.
    pub subject: String,
    /// 32-byte historical handle, hex-encoded.
    pub handle: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LeafResponse {
    pub leaf_index: u64,
    pub leaf_count: u64,
}

pub struct LineageLeafHandler;

impl LineageLeafHandler {
    pub fn routes(&self, _state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new().route("/lineage/{value_key}/leaf", get(handle))
    }
}

#[utoipa::path(
    get,
    path = "/lineage/{value_key}/leaf",
    params(("value_key" = String, Path, description = "32-byte acl_nonce_key, hex"), LeafQuery),
    responses(
        (status = 200, body = LeafResponse),
        (status = 404, description = "lineage_not_found | lineage_has_no_leaves | leaf_not_found"),
    )
)]
pub async fn handle(
    State(state): State<Arc<AppState>>,
    Path(value_key): Path<String>,
    Query(query): Query<LeafQuery>,
) -> Result<impl IntoResponse, AppError> {
    let value_key = parse_key(&value_key, "value_key")?;
    let subject = parse_key(&query.subject, "subject")?;
    let handle = parse_key(&query.handle, "handle")?;

    let pda = state
        .repo
        .pda_for_value_key(&value_key)
        .await?
        .ok_or(AppError::LineageNotFound)?;

    // Distinguish "known lineage that has never been rotated" (no historical
    // leaves yet) from "rotated, but this (subject, handle) is not among the
    // leaves" — both would otherwise collapse to a confusing `leaf_not_found`.
    if state.repo.events_for_pda(&pda).await?.is_empty() {
        return Err(AppError::LineageHasNoLeaves);
    }

    let (leaf_index, leaf_count) = state
        .repo
        .leaf_for_subject_handle(&pda, &subject, &handle)
        .await?
        .ok_or(AppError::LeafNotFound)?;

    info!(
        pda = hex::encode(pda),
        leaf_index, leaf_count, "leaf lookup served"
    );
    Ok(Json(LeafResponse {
        leaf_index,
        leaf_count,
    }))
}
