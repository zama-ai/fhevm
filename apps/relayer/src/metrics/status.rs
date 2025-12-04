use once_cell::sync::OnceCell;
use prometheus::{register_gauge_vec_with_registry, GaugeVec, Opts, Registry};

use crate::store::sql::models::req_status_enum_model::ReqStatus;

#[derive(Debug)]
struct InternalMetrics {
    // Count how many requests are in each statuses.
    pub request_status_count: GaugeVec,
}

static STATUS_METRICS: OnceCell<InternalMetrics> = OnceCell::new();

pub fn init_internal_metrics(registry: &Registry) {
    STATUS_METRICS.get_or_init(|| InternalMetrics {
        request_status_count: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_request_count",
                "Number of request by table and statuses"
            ),
            &["table", "status"],
            registry,
        )
        .unwrap(),
    });
}

pub enum Table {
    UserDecryptReq,
    PublicDecryptReq,
    InputProofReq,
}

impl Table {
    pub fn as_str(&self) -> &'static str {
        match self {
            Table::UserDecryptReq => "user_decrypt_req",
            Table::PublicDecryptReq => "public_decrypt_req",
            Table::InputProofReq => "input_proof_req",
        }
    }
}

pub fn increment_req_status_count(table: Table, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), status.as_str()])
        .inc();
}

pub fn decrement_req_status_count(table: Table, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), status.as_str()])
        .dec();
}
