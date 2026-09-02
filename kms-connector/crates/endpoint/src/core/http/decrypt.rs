//! Request flow shared by `POST /v1/public-decrypt` and `POST /v1/user-decrypt`.
//!
//! 1. deserialize + validate,
//! 2. derive the content id,
//! 3. backpressure: take an in-flight permit or answer `503 overloaded`,
//! 4. register the waiter *before* reading the DB so no response can be missed,
//! 5. in one transaction: serve an existing payload or non-retryable error row, otherwise store the
//!    request (insert it, or re-arm it to `pending` if it previously `failed`; any other conflict
//!    means the kms-worker is already on it and we attach),
//! 6. wait for the response listener to wake the waiter, then read the response row.

use crate::core::http::AppState;
use actix_web::HttpResponse;
use alloy::primitives::B256;
use anyhow::anyhow;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{ErrorCode, ErrorResponse};
use sqlx::{PgExecutor, Postgres, Transaction, postgres::PgQueryResult};
use std::{future::Future, sync::Arc};
use tracing::{debug, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub trait DecryptionRoute {
    type Request;
    type ResponseRow;

    fn read_response<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
    ) -> impl Future<Output = sqlx::Result<Option<Self::ResponseRow>>> + Send;

    fn upsert_request<'e>(
        executor: impl PgExecutor<'e>,
        id: B256,
        request: &Self::Request,
        otlp_ctx: &PropagationContext,
    ) -> impl Future<Output = anyhow::Result<PgQueryResult>> + Send;

    /// The `error_code` of an error row, `None` for a payload row.
    fn error_code(response_row: &Self::ResponseRow) -> Option<ErrorCode>;

    /// `200` body or mapped error from a response row.
    fn build_response(
        id: B256,
        response_row: Self::ResponseRow,
    ) -> Result<HttpResponse, ErrorResponse>;
}

/// Handles a validated decryption request.
pub async fn handle<R: DecryptionRoute>(
    state: &AppState,
    id: B256,
    request: &R::Request,
) -> Result<HttpResponse, ErrorResponse> {
    // Backpressure: no DB access when the replica is at its in-flight cap.
    let in_flight_limiter = Arc::clone(&state.in_flight_limiter);
    let Ok(permit) = in_flight_limiter.try_acquire_owned() else {
        return Err(ErrorResponse::new(
            ErrorCode::Overloaded,
            "too many in-flight requests on this replica",
            Some(id),
        ));
    };
    let (_guard, receiver) = state.waiters.register(id, permit);

    let response_row = find_response_or_store_request::<R>(state, id, request)
        .await
        .map_err(|e| {
            warn!(decryption_id = %id, "Failed to find response or store request: {e}");
            upstream_transient("temporary storage error", id)
        })?;

    let row = match response_row {
        Some(row) => {
            debug!(decryption_id = %id, "Serving existing response row");
            row
        }
        None => wait_and_read_response::<R>(state, id, receiver).await?,
    };

    R::build_response(id, row)
}

/// Returns the response row to serve, or `None` once the request is stored for (re)processing.
async fn find_response_or_store_request<R: DecryptionRoute>(
    state: &AppState,
    id: B256,
    request: &R::Request,
) -> anyhow::Result<Option<R::ResponseRow>> {
    let mut tx: Transaction<'_, Postgres> = state.db_pool.begin().await?;

    let mut retrying = false;
    if let Some(row) = R::read_response(&mut *tx, id).await? {
        match R::error_code(&row) {
            Some(code) if code.retryable() => {
                debug!(decryption_id = %id, "Ignoring retryable `{}` error row", code.as_str());
                retrying = true;
            }
            _ => {
                tx.commit().await?;
                return Ok(Some(row));
            }
        }
    }

    let otlp_ctx = PropagationContext::inject(&tracing::Span::current().context());
    let result = R::upsert_request(&mut *tx, id, request, &otlp_ctx).await?;
    match (result.rows_affected(), retrying) {
        (1, true) => info!(decryption_id = %id, "Retrying failed decryption request"),
        (1, false) => info!(decryption_id = %id, "Decryption request stored in DB"),
        (0, _) => debug!(decryption_id = %id, "Decryption request already in progress, attaching"),
        (n, _) => return Err(anyhow!("unexpected upsert result: {n} rows affected")),
    }

    tx.commit().await?;
    Ok(None)
}

async fn wait_and_read_response<R: DecryptionRoute>(
    state: &AppState,
    id: B256,
    receiver: tokio::sync::oneshot::Receiver<()>,
) -> Result<R::ResponseRow, ErrorResponse> {
    let timeout = state.config.decryption_timeout;
    let wake_signal = tokio::time::timeout(timeout, receiver).await.map_err(|_| {
        warn!(decryption_id = %id, "No response within {timeout:?}, answering timeout");
        ErrorResponse::new(
            ErrorCode::Timeout,
            format!("no response within {timeout:?}"),
            Some(id),
        )
    })?;
    wake_signal.map_err(|_| {
        warn!(decryption_id = %id, "Response listener gone while waiting");
        upstream_transient("connection to the response stream was lost", id)
    })?;
    R::read_response(&state.db_pool, id)
        .await
        .map_err(|e| {
            warn!(decryption_id = %id, "Failed to read notified response row: {e}");
            upstream_transient("temporary storage error", id)
        })?
        .ok_or_else(|| {
            warn!(decryption_id = %id, "Notified response row not found");
            ErrorResponse::new(
                ErrorCode::Unknown,
                "response was announced but could not be read",
                Some(id),
            )
        })
}

fn upstream_transient(message: impl Into<String>, id: B256) -> ErrorResponse {
    ErrorResponse::new(ErrorCode::UpstreamTransient, message, Some(id))
}

/// Maps the error stored by the kms-worker in a response row.
pub fn error_from_row(error_code: &str, error_details: Option<String>, id: B256) -> ErrorResponse {
    let code = error_code.parse().unwrap_or(ErrorCode::Unknown);
    let message = error_details.unwrap_or_else(|| error_code.to_string());
    ErrorResponse::new(code, message, Some(id))
}
