//! API error type rendered as `{label, message}` with a structured log per
//! response, mirroring the relayer's error-body shape.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tracing::{info, warn};
use utoipa::ToSchema;

/// The JSON error body returned for every non-2xx response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    /// Stable machine-readable label (e.g. `lineage_not_found`).
    pub label: String,
    /// Human-readable detail.
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    /// value_key has no mapped PDA — the lineage is unknown. 404.
    LineageNotFound,
    /// The lineage exists (initialized) but has no historical leaves yet (no
    /// rotation/mark has occurred). Distinct from `LineageNotFound` so a caller
    /// that has just observed the `initialize` can tell "unknown" from "seen but
    /// not rotated yet" and not retry forever. 404.
    LineageHasNoLeaves,
    /// `(subject, handle)` not present in the reconstructed leaf list. 404.
    LeafNotFound,
    /// `leaf_index >= leaf_count`. 422.
    LeafIndexOutOfRange,
    /// Malformed request (bad hex, wrong length). 400.
    BadRequest(String),
    /// Unexpected server-side failure. 500.
    Internal(String),
}

/// Fixed client-facing message for any 500 — the real cause is logged server-side
/// but never serialised into the response body (it can carry DB/query internals).
const INTERNAL_CLIENT_MESSAGE: &str = "internal server error";

impl AppError {
    /// The `(status, label, client_message)` returned to the caller. For
    /// `Internal`, `client_message` is the fixed generic string; the stored detail
    /// is logged separately in `into_response`, never sent to the client.
    fn parts(&self) -> (StatusCode, &'static str, String) {
        match self {
            AppError::LineageNotFound => (
                StatusCode::NOT_FOUND,
                "lineage_not_found",
                "no lineage for the given value_key".to_string(),
            ),
            AppError::LineageHasNoLeaves => (
                StatusCode::NOT_FOUND,
                "lineage_has_no_leaves",
                "lineage exists but has no historical leaves yet (not rotated)".to_string(),
            ),
            AppError::LeafNotFound => (
                StatusCode::NOT_FOUND,
                "leaf_not_found",
                "no leaf for the given (subject, handle)".to_string(),
            ),
            AppError::LeafIndexOutOfRange => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "leaf_index_out_of_range",
                "leaf_index is beyond leaf_count".to_string(),
            ),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, "bad_request", m.clone()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                INTERNAL_CLIENT_MESSAGE.to_string(),
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, label, message) = self.parts();
        if status.is_server_error() {
            // Log the real (possibly DB-internal) detail server-side; the response
            // body carries only the fixed generic `message`.
            let detail = match &self {
                AppError::Internal(m) => m.as_str(),
                _ => message.as_str(),
            };
            warn!(label, %detail, status = status.as_u16(), "request failed");
        } else {
            info!(label, %message, status = status.as_u16(), "request rejected");
        }
        (
            status,
            Json(ErrorBody {
                label: label.to_string(),
                message,
            }),
        )
            .into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        // Capture the full chain (logged server-side in `into_response`); the
        // client only ever sees the fixed `INTERNAL_CLIENT_MESSAGE`.
        AppError::Internal(format!("{err:#}"))
    }
}
