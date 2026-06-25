use crate::monitoring::OPERATION_TABLES;
use connector_utils::types::KmsResponse;
use prometheus::{
    HistogramVec, IntCounterVec, IntGaugeVec, register_histogram_vec, register_int_counter_vec,
    register_int_gauge_vec,
};
use sqlx::types::chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use std::{sync::LazyLock, time::Duration};
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::error;

pub static RESPONSE_RECEIVED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_response_received_counter",
        "Number of responses received by the TransactionSender",
        &["response_type"]
    )
    .expect("Failed to register kms_connector_tx_sender_response_received_counter metric")
});

pub static RESPONSE_RECEIVED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_response_received_errors",
        "Number of errors encountered by the TransactionSender while listening for responses",
        &["response_type"]
    )
    .expect("Failed to register kms_connector_tx_sender_response_received_errors metric")
});

pub static GATEWAY_TX_SENT_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_gateway_tx_sent_counter",
        "Number of transactions sent by the TransactionSender to the Gateway",
        &["response_type"]
    )
    .expect("Failed to register kms_connector_tx_sender_gateway_tx_sent_counter metric")
});

pub static GATEWAY_TX_SENT_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_gateway_tx_sent_errors",
        "Number of errors encountered by the TransactionSender while sending transactions to the Gateway",
        &["response_type"]
    )
    .expect("Failed to register kms_connector_tx_sender_gateway_tx_sent_errors metric")
});

const RESPONSE_FORWARDING_LATENCY_BUCKETS: &[f64] = &[0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 15.0, 30.0];

pub static RESPONSE_FORWARDING_LATENCY_HISTOGRAM: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "kms_connector_tx_sender_response_forwarding_latency_seconds",
        "Latency from response creation in DB to successful blockchain transaction confirmation",
        &["response_type"],
        RESPONSE_FORWARDING_LATENCY_BUCKETS.to_vec()
    )
    .expect("Failed to register kms_connector_tx_sender_response_forwarding_latency_seconds metric")
});

pub fn register_response_forwarding_latency(response: &KmsResponse) {
    let elapsed = Utc::now() - response.created_at;
    RESPONSE_FORWARDING_LATENCY_HISTOGRAM
        .with_label_values(&[response.kind.as_str()])
        .observe(elapsed.as_seconds_f64());
}

// Definition of gauges are in the tx-sender because it is a single entity, contrary to kms-worker
// or gw-listener.
// So the benefit is that a single background job updating these gauges will be spawned.

static UNPROCESSED_OPERATIONS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "kms_connector_unprocessed_operations",
        "Number of operations not yet processed (status `pending` or `under_process`).",
        &["table", "status"]
    )
    .expect("Failed to register kms_connector_unprocessed_operations metric")
});

pub fn spawn_gauge_update_routine(
    period: Duration,
    db_pool: Pool<Postgres>,
    cancel_token: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        select! {
            _ = run_gauge_update_routine(period, db_pool) => {}
            _ = cancel_token.cancelled() => {}
        }
    })
}

/// The operation statuses tracked by the unprocessed gauge, kept bounded by excluding the terminal
/// `completed`/`failed` statuses (which grow forever now that operations are not deleted).
const PENDING_STATUS: &str = "pending";
const UNDER_PROCESS_STATUS: &str = "under_process";

async fn run_gauge_update_routine(period: Duration, db_pool: Pool<Postgres>) {
    loop {
        for table in OPERATION_TABLES.iter().copied() {
            if let Some((pending, under_process)) = count_unprocessed(&db_pool, table).await {
                UNPROCESSED_OPERATIONS
                    .with_label_values(&[table, PENDING_STATUS])
                    .set(pending);
                UNPROCESSED_OPERATIONS
                    .with_label_values(&[table, UNDER_PROCESS_STATUS])
                    .set(under_process);
            }
        }

        tokio::time::sleep(period).await;
    }
}

/// Returns the `(pending, under_process)` row counts for the given table, or `None` if the query
/// failed.
///
/// `table` must be a trusted, hardcoded table name.
async fn count_unprocessed(db_pool: &Pool<Postgres>, table: &str) -> Option<(i64, i64)> {
    let query = format!(
        "SELECT
            COUNT(*) FILTER (WHERE status = 'pending') AS pending,
            COUNT(*) FILTER (WHERE status = 'under_process') AS under_process
        FROM {table}"
    );
    match sqlx::query(&query).fetch_one(db_pool).await {
        Ok(row) => Some((
            row.try_get::<i64, _>("pending").unwrap_or(0),
            row.try_get::<i64, _>("under_process").unwrap_or(0),
        )),
        Err(e) => {
            error!(error = %e, "Failed to count unprocessed rows in {table}");
            None
        }
    }
}
