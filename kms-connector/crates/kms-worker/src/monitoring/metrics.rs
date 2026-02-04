use connector_utils::types::{GatewayEvent, GatewayEventKind, db::EventType};
use prometheus::{
    HistogramOpts, HistogramVec, IntCounter, IntCounterVec, register_histogram_vec,
    register_int_counter, register_int_counter_vec,
};
use sqlx::types::chrono::Utc;
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_received_counter",
        "Number of events received by the KmsWorker",
        &["event_type"]
    )
    .unwrap()
});

pub static EVENT_RECEIVED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_received_errors",
        "Number of errors encountered by the KmsWorker while listening for events",
        &["event_type"]
    )
    .unwrap()
});

pub static GRPC_REQUEST_SENT_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_request_sent_counter",
        "Number of successful GRPC requests sent by the KmsWorker to the KMS Core",
        &["event_type"]
    )
    .unwrap()
});

pub static GRPC_REQUEST_SENT_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_request_sent_errors",
        "Number of errors encountered by the KmsWorker while sending grpc requests to the KMS Core",
        &["event_type"]
    )
    .unwrap()
});

pub static GRPC_RESPONSE_POLLED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_response_polled_counter",
        "Number of responses successfully polled from the KMS Core via GRPC",
        &["event_type"]
    )
    .unwrap()
});

pub static GRPC_RESPONSE_POLLED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_response_polled_errors",
        "Number of errors encountered by the KmsWorker while polling responses from the KMS Core",
        &["event_type"]
    )
    .unwrap()
});

pub static S3_CIPHERTEXT_RETRIEVAL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_s3_ciphertext_retrieval_counter",
        "Number of ciphertexts retrieved from S3 by the KmsWorker"
    )
    .unwrap()
});

pub static S3_CIPHERTEXT_RETRIEVAL_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_s3_ciphertext_retrieval_errors",
        "Number of errors encountered by the KmsWorker while retrieving ciphertexts from S3"
    )
    .unwrap()
});

/// Histogram bucket boundaries (in seconds) for decryption latency measurements.
/// Ranges from 10ms to 30s to capture both fast and slow decryption.
const DECRYPTION_LATENCY_BUCKETS: &[f64] = &[0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0];

pub static DECRYPTION_LATENCY_HISTOGRAM: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        HistogramOpts::new(
            "kms_connector_worker_decryption_latency_seconds",
            "Latency of decryptions at the KmsWorker level"
        )
        .buckets(DECRYPTION_LATENCY_BUCKETS.to_vec()),
        &["event_type"]
    )
    .unwrap()
});

pub fn register_event_latency(event: &GatewayEvent) {
    if matches!(
        event.kind,
        GatewayEventKind::PublicDecryption(_) | GatewayEventKind::UserDecryption(_)
    ) {
        let elapsed = Utc::now() - event.created_at;
        DECRYPTION_LATENCY_HISTOGRAM
            .with_label_values(&[EventType::from(&event.kind).as_str()])
            .observe(elapsed.as_seconds_f64());
    }
}
