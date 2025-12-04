use once_cell::sync::OnceCell;
use prometheus::{register_gauge_vec_with_registry, GaugeVec, Opts, Registry};

#[derive(Debug)]
struct InternalMetrics {
    // Count how many requests are in each statuses.
    pub req_count: GaugeVec,
}

pub static DB_METRICS: OnceCell<InternalMetrics> = OnceCell::new();

pub fn init_internal_metrics(registry: &Registry) {
    DB_METRICS.get_or_init(|| InternalMetrics {
        req_count: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_db_req_count",
                "Number of request by table and statuses"
            ),
            &["table", "status"],
            registry,
        )
        .unwrap(),
    });
}
