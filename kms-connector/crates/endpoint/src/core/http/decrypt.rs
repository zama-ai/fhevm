//! Request flow shared by `POST /v1/public-decrypt` and `POST /v1/user-decrypt`.
//!
//! 1. deserialize + validate,
//! 2. derive the content id,
//! 3. backpressure: take an in-flight permit or answer `503 overloaded`,
//! 4. register the waiter *before* reading the DB so no response can be missed,
//! 5. in one transaction: serve an existing payload or an irrecoverable error within its serve window,
//!    otherwise store the request (insert it, or reset its status to `pending` if it previously
//!    `failed`; any other
//!    conflict means the kms-worker is already on it and we attach),
//! 6. wait for the response listener to wake the waiter, then read the response row.

use crate::core::{http::AppState, waiters::Waiter};
use crate::monitoring::metrics;
use actix_web::HttpResponse;
use alloy::primitives::B256;
use anyhow::anyhow;
use connector_utils::monitoring::otlp::PropagationContext;
use kms_connector_api::{ErrorCode, ErrorResponse};
use sqlx::{
    PgExecutor, Postgres, Transaction,
    postgres::PgQueryResult,
    types::chrono::{DateTime, Utc},
};
use std::{future::Future, sync::Arc};
use tokio::time::Instant;
use tracing::{debug, info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub trait DecryptionRoute {
    const ROUTE: &'static str;
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

    /// The `created_at` of a response row.
    fn created_at(response_row: &Self::ResponseRow) -> DateTime<Utc>;

    /// `200` body or mapped error from a response row.
    fn build_response(
        id: B256,
        response_row: Self::ResponseRow,
    ) -> Result<HttpResponse, ErrorResponse>;
}

/// Outcome of the lookup transaction of [`find_response_or_store_request`].
enum LookupOutcome<Row> {
    /// A payload or an irrecoverable error within its serve window to serve as is.
    Serve(Row),
    /// The response has to be waited for.
    Wait {
        /// How the request was handled, for metrics: `stored` (new request), `retried` (request reset after an old
        /// error), or `already_in_progress` (already being processed).
        outcome: &'static str,
        /// The `created_at` of the old error row skipped by the lookup, if any. Waking up to that same row again means
        /// the new response is not there yet. Set for `retried`, and also for `already_in_progress` when another
        /// request already reset the request after that error.
        old_response_created_at: Option<DateTime<Utc>>,
    },
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
    let mut waiter = state.waiter_registry.register(id, permit);

    let lookup = find_response_or_store_request::<R>(state, id, request)
        .await
        .map_err(|e| {
            warn!(decryption_id = %id, "Failed to find response or store request: {e}");
            upstream_transient("temporary storage error", id)
        })?;

    let row = match lookup {
        LookupOutcome::Serve(row) => {
            metrics::DECRYPTION_LOOKUP_COUNTER
                .with_label_values(&[R::ROUTE, "served"])
                .inc();
            row
        }
        LookupOutcome::Wait {
            outcome,
            old_response_created_at,
        } => {
            metrics::DECRYPTION_LOOKUP_COUNTER
                .with_label_values(&[R::ROUTE, outcome])
                .inc();
            wait_and_read_response::<R>(state, id, &mut waiter, old_response_created_at).await?
        }
    };

    R::build_response(id, row)
}

/// Serves an existing response row, or stores the request for (re)processing.
async fn find_response_or_store_request<R: DecryptionRoute>(
    state: &AppState,
    id: B256,
    request: &R::Request,
) -> anyhow::Result<LookupOutcome<R::ResponseRow>> {
    let mut tx: Transaction<'_, Postgres> = state.db_pool.begin().await?;

    let mut old_response_created_at = None;
    if let Some(row) = R::read_response(&mut *tx, id).await? {
        match R::error_code(&row) {
            Some(code)
                if code.retryable()
                    || Utc::now()
                        .signed_duration_since(R::created_at(&row))
                        .to_std()
                        .is_ok_and(|age| age >= state.config.error_serve_window) =>
            {
                debug!(decryption_id = %id, "Reprocessing recoverable or expired `{}` error row", code.as_str());
                old_response_created_at = Some(R::created_at(&row));
            }
            _ => {
                tx.commit().await?;
                debug!(decryption_id = %id, "Serving existing response row");
                return Ok(LookupOutcome::Serve(row));
            }
        }
    }

    let otlp_ctx = PropagationContext::inject(&tracing::Span::current().context());
    let result = R::upsert_request(&mut *tx, id, request, &otlp_ctx).await?;
    let outcome = match (result.rows_affected(), old_response_created_at.is_some()) {
        (1, true) => {
            info!(decryption_id = %id, "Retrying failed decryption request");
            "retried"
        }
        (1, false) => {
            info!(decryption_id = %id, "Decryption request stored in DB");
            "stored"
        }
        (0, _) => {
            debug!(decryption_id = %id, "Decryption request already in progress, attaching");
            "already_in_progress"
        }
        (n, _) => return Err(anyhow!("unexpected upsert result: {n} rows affected")),
    };

    tx.commit().await?;
    Ok(LookupOutcome::Wait {
        outcome,
        old_response_created_at,
    })
}

/// Waits for the response of `id` and reads it, within `Config::decryption_timeout` overall.
///
/// A wake-up that only leads to the stale row identified by `old_response_created_at` keeps
/// waiting.
async fn wait_and_read_response<R: DecryptionRoute>(
    state: &AppState,
    id: B256,
    waiter: &mut Waiter,
    old_response_created_at: Option<DateTime<Utc>>,
) -> Result<R::ResponseRow, ErrorResponse> {
    let started_at = Instant::now();
    let timeout = state.config.decryption_timeout;
    let deadline = started_at + timeout;
    loop {
        let wake_signal = tokio::time::timeout_at(deadline, waiter.wait())
            .await
            .map_err(|_| {
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

        let row = R::read_response(&state.db_pool, id)
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
            })?;

        if old_response_created_at == Some(R::created_at(&row)) {
            debug!(decryption_id = %id, "Woken up by the stale error row, waiting again");
            continue;
        }
        metrics::DECRYPTION_WAIT_LATENCY_HISTOGRAM
            .with_label_values(&[R::ROUTE])
            .observe(started_at.elapsed().as_secs_f64());
        return Ok(row);
    }
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
