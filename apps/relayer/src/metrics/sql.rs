// 1. put the queries latencies (Hist)
// 2. SQL pool wait time (average mesure of the connection pool), and active/idle connections.
// 3. TODO: Metric to mesure the error rate (After error handling on sql.)
// TODO add on readiness and liveness health for DB.

use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_histogram_vec_with_registry, CounterVec,
    HistogramOpts, HistogramVec, Opts, Registry,
};

use crate::config::settings::MetricsConfig;

#[derive(Debug)]
struct DbMetrics {
    // 1. Query Latency
    query_duration_seconds: HistogramVec,
    // 2. Errors
    db_errors_total: CounterVec,
}

static DB_METRICS: OnceCell<DbMetrics> = OnceCell::new();

pub fn init_db_metrics(registry: &Registry, config: MetricsConfig) {
    DB_METRICS.get_or_init(|| DbMetrics {
        query_duration_seconds: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_db_query_duration_seconds",
                "Time taken to execute SQL queries",
            )
            .buckets(config.query_duration_histogram_bucket.clone()),
            &["table"],
            registry
        )
        .unwrap(),
        db_errors_total: register_counter_vec_with_registry!(
            Opts::new("relayer_db_errors_total", "Total DB errors"),
            &["table"],
            registry,
        )
        .unwrap(),
    });
}

pub enum Table {
    UserDecryptReq,
    UserDecryptShares,
    PublicDecryptReq,
    InputProofReq,
    GatewayBlockNumberStore,
}

impl Table {
    pub fn as_str(&self) -> &'static str {
        match self {
            Table::UserDecryptReq => "user_decrypt_req",
            Table::UserDecryptShares => "user_decrypt_shares",
            Table::PublicDecryptReq => "public_decrypt_req",
            Table::InputProofReq => "input_proof_req",
            Table::GatewayBlockNumberStore => "gateway_block_number_store",
        }
    }
}

// --- API ---
pub fn observe_query(table: Table, duration: std::time::Duration) {
    let metrics = DB_METRICS.get().expect("DB Metrics not initialized");
    metrics
        .query_duration_seconds
        .with_label_values(&[table.as_str()])
        .observe(duration.as_secs_f64());
}

pub fn increment_error(table: Table) {
    let metrics = DB_METRICS.get().expect("DB Metrics not initialized");
    metrics
        .db_errors_total
        .with_label_values(&[table.as_str()])
        .inc();
}
