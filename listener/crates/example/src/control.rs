//! Runtime control plane — the half of the catchup lifecycle that a restart
//! cannot demonstrate.
//!
//! Boot-time reconciliation (see [`crate::catchup_state`]) covers "what should
//! this consumer be replaying when it starts". It does not cover an operator
//! deciding, while the process runs, that a backfill should stop. Before this
//! module the only way to reach [`catchup_state::reconcile`] was to restart
//! with a different `{PREFIX}_START` / `{PREFIX}_END`, which means a cancel
//! could not be issued at all without a deployment.
//!
//! Every route here funnels into [`catchup_state::reconcile_and_track`], the
//! same entry point the boot path uses. That is deliberate: a runtime cancel
//! and a restart-time cancel that are two separate implementations will drift,
//! and only one of them will be the one anybody tested.
//!
//! ```bash
//! curl -s localhost:8088/stats
//! curl -sX POST localhost:8088/catchup/request \
//!      -H 'content-type: application/json' -d '{"block_start":1,"block_end":500}'
//! curl -sX POST localhost:8088/catchup/cancel
//! ```
//!
//! There is no authentication. `CONTROL_ADDR` defaults to loopback for that
//! reason; a real deployment would put this behind whatever the rest of the
//! service uses.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use consumer::ListenerConsumer;
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tracing::{info, warn};
use uuid::Uuid;

use crate::catchup_state::{self, Flow, Range, Store};
use crate::stats::Stats;

/// Everything a route needs to change or report on this consumer's state.
#[derive(Clone)]
pub struct Control {
    consumer: ListenerConsumer,
    store: Store,
    stats: Arc<Stats>,
    catchup_active: Arc<watch::Sender<Option<Uuid>>>,
    final_catchup_active: Arc<watch::Sender<Option<Uuid>>>,
}

impl Control {
    pub fn new(
        consumer: ListenerConsumer,
        store: Store,
        stats: Arc<Stats>,
        catchup_active: Arc<watch::Sender<Option<Uuid>>>,
        final_catchup_active: Arc<watch::Sender<Option<Uuid>>>,
    ) -> Self {
        Self {
            consumer,
            store,
            stats,
            catchup_active,
            final_catchup_active,
        }
    }

    fn active(&self, flow: Flow) -> &watch::Sender<Option<Uuid>> {
        match flow {
            Flow::Catchup => &self.catchup_active,
            Flow::FinalCatchup => &self.final_catchup_active,
        }
    }
}

/// Serve the control plane until the process ends.
pub async fn serve(control: Control, addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/stats", get(stats))
        .route("/catchup/request", post(request_catchup))
        .route("/catchup/cancel", post(cancel_catchup))
        .route("/final-catchup/request", post(request_final_catchup))
        .route("/final-catchup/cancel", post(cancel_final_catchup))
        .with_state(control);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "control endpoint listening");
    axum::serve(listener, app).await?;
    Ok(())
}

// ── Routes ───────────────────────────────────────────────────────────────

async fn stats(State(control): State<Control>) -> Result<Json<StatsResponse>, ApiError> {
    let counters = control.stats.snapshot();
    Ok(Json(StatsResponse {
        consumer_id: control.consumer.consumer_id().to_string(),
        live: FlowReport {
            delivered: counters.live_delivered,
        },
        r#final: FlowReport {
            delivered: counters.final_delivered,
        },
        catchup: CatchupReport {
            active_id: control.store.current_id(Flow::Catchup).await?,
            delivered: counters.catchup.delivered,
            dropped_stale: counters.catchup.dropped_stale,
        },
        final_catchup: CatchupReport {
            active_id: control.store.current_id(Flow::FinalCatchup).await?,
            delivered: counters.final_catchup.delivered,
            dropped_stale: counters.final_catchup.dropped_stale,
        },
    }))
}

async fn request_catchup(
    State(control): State<Control>,
    Json(body): Json<RangeBody>,
) -> Result<Json<CatchupResponse>, ApiError> {
    set_desired(&control, Flow::Catchup, Some(body.into_range()?)).await
}

async fn request_final_catchup(
    State(control): State<Control>,
    Json(body): Json<RangeBody>,
) -> Result<Json<CatchupResponse>, ApiError> {
    set_desired(&control, Flow::FinalCatchup, Some(body.into_range()?)).await
}

async fn cancel_catchup(State(control): State<Control>) -> Result<Json<CancelResponse>, ApiError> {
    cancel(&control, Flow::Catchup).await
}

async fn cancel_final_catchup(
    State(control): State<Control>,
) -> Result<Json<CancelResponse>, ApiError> {
    cancel(&control, Flow::FinalCatchup).await
}

/// Reconcile to `desired` and report the id that resulted.
async fn set_desired(
    control: &Control,
    flow: Flow,
    desired: Option<Range>,
) -> Result<Json<CatchupResponse>, ApiError> {
    reconcile(control, flow, desired).await?;

    Ok(Json(CatchupResponse {
        catchup_id: control.store.current_id(flow).await?,
    }))
}

/// Retire whatever `flow` owns, and report which id was retired.
///
/// The answer comes from the tombstone rather than from a read taken before
/// the reconcile, so it reports what was persisted: `cancelled_id` is null
/// when the flow owned nothing and the cancel was a no-op. `catchup_id` is
/// always null here — that is the point of a cancel — and is echoed so the
/// two routes have a shape in common.
async fn cancel(control: &Control, flow: Flow) -> Result<Json<CancelResponse>, ApiError> {
    reconcile(control, flow, None).await?;

    Ok(Json(CancelResponse {
        cancelled_id: control.store.cancelled_id(flow).await?,
        catchup_id: control.store.current_id(flow).await?,
    }))
}

async fn reconcile(control: &Control, flow: Flow, desired: Option<Range>) -> Result<(), ApiError> {
    catchup_state::reconcile_and_track(
        &control.consumer,
        &control.store,
        flow,
        desired,
        control.active(flow),
    )
    .await?;
    Ok(())
}

// ── Wire types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RangeBody {
    block_start: u64,
    block_end: u64,
}

impl RangeBody {
    fn into_range(self) -> Result<Range, ApiError> {
        if self.block_start > self.block_end {
            return Err(ApiError::BadRequest(format!(
                "block_start ({}) is above block_end ({})",
                self.block_start, self.block_end
            )));
        }
        Ok(Range {
            block_start: self.block_start,
            block_end: self.block_end,
        })
    }
}

#[derive(Debug, Serialize)]
struct FlowReport {
    delivered: u64,
}

#[derive(Debug, Serialize)]
struct CatchupReport {
    active_id: Option<Uuid>,
    delivered: u64,
    dropped_stale: u64,
}

#[derive(Debug, Serialize)]
struct StatsResponse {
    consumer_id: String,
    live: FlowReport,
    r#final: FlowReport,
    catchup: CatchupReport,
    final_catchup: CatchupReport,
}

#[derive(Debug, Serialize)]
struct CatchupResponse {
    catchup_id: Option<Uuid>,
}

/// Answer to a cancel: the id that was retired, and the id now owned (none).
#[derive(Debug, Serialize)]
struct CancelResponse {
    cancelled_id: Option<Uuid>,
    catchup_id: Option<Uuid>,
}

// ── Errors ───────────────────────────────────────────────────────────────

enum ApiError {
    BadRequest(String),
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError::Internal(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::Internal(e) => {
                // A failed reconcile may have left a `retiring` id on disk;
                // the next boot replays it. Say so rather than dropping the
                // detail into a 500 with no trace.
                warn!(error = %e, "control request failed");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_inverted_range_is_rejected_before_it_reaches_the_listener() {
        let body = RangeBody {
            block_start: 500,
            block_end: 1,
        };
        assert!(matches!(body.into_range(), Err(ApiError::BadRequest(_))));
    }

    #[test]
    fn a_single_block_range_is_accepted() {
        let body = RangeBody {
            block_start: 7,
            block_end: 7,
        };
        let range = body.into_range().ok().expect("single-block range");
        assert_eq!(range.block_start, 7);
        assert_eq!(range.block_end, 7);
    }

    #[test]
    fn a_cancel_reports_the_retired_id_and_an_empty_current() {
        let retired = Uuid::now_v7();
        let body = serde_json::to_value(CancelResponse {
            cancelled_id: Some(retired),
            catchup_id: None,
        })
        .unwrap();

        assert_eq!(body["cancelled_id"], serde_json::json!(retired.to_string()));
        assert_eq!(body["catchup_id"], serde_json::Value::Null);
    }
}
