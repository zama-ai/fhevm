use connector_utils::types::{ProtocolEvent, ProtocolEventKind, db::EventType};
use prometheus::{
    HistogramOpts, HistogramVec, IntCounter, IntCounterVec, register_histogram_vec,
    register_int_counter, register_int_counter_vec,
};
use sqlx::types::chrono::Utc;
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_received_counter",
        "Number of events picked from DB by the KmsWorker (retries included)",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_event_received_counter metric")
});

pub static EVENT_COMPLETED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_completed_counter",
        "Number of events successfully processed by the KmsWorker (response stored in DB)",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_event_completed_counter metric")
});

pub static EVENT_FAILED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_failed_counter",
        "Number of events the KmsWorker gave up on, by terminal status (failed or aborted)",
        &["event_type", "reason"]
    )
    .expect("Failed to register kms_connector_worker_event_failed_counter metric")
});

pub static EVENT_RETRIED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_retried_counter",
        "Number of events put back to pending by the KmsWorker to be retried",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_event_retried_counter metric")
});

pub static EVENT_RECEIVED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_event_received_errors",
        "Number of errors encountered by the KmsWorker while listening for events",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_event_received_errors metric")
});

pub static GRPC_REQUEST_SENT_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_request_sent_counter",
        "Number of successful GRPC requests sent by the KmsWorker to the KMS Core",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_grpc_request_sent_counter metric")
});

pub static GRPC_REQUEST_SENT_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_request_sent_errors",
        "Number of errors encountered by the KmsWorker while sending grpc requests to the KMS Core",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_grpc_request_sent_errors metric")
});

pub static GRPC_RESPONSE_POLLED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_response_polled_counter",
        "Number of responses successfully polled from the KMS Core via GRPC",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_grpc_response_polled_counter metric")
});

pub static GRPC_RESPONSE_POLLED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_grpc_response_polled_errors",
        "Number of errors encountered by the KmsWorker while polling responses from the KMS Core",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_worker_grpc_response_polled_errors metric")
});

pub static S3_CIPHERTEXT_RETRIEVAL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_s3_ciphertext_retrieval_counter",
        "Number of ciphertexts retrieved from S3 by the KmsWorker"
    )
    .expect("Failed to register kms_connector_worker_s3_ciphertext_retrieval_counter metric")
});

pub static S3_CIPHERTEXT_RETRIEVAL_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_s3_ciphertext_retrieval_errors",
        "Number of errors encountered by the KmsWorker while retrieving ciphertexts from S3"
    )
    .expect("Failed to register kms_connector_worker_s3_ciphertext_retrieval_errors metric")
});

pub static REQUEST_CHECK_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_worker_request_check_errors",
        "Counts request pre-flight check failures, by check family",
        &["check_type"]
    )
    .expect("Failed to register kms_connector_worker_request_check_errors metric")
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
    .expect("Failed to register kms_connector_worker_decryption_latency_seconds metric")
});

pub fn inc_event_completed_counter(event: &ProtocolEvent) {
    EVENT_COMPLETED_COUNTER
        .with_label_values(&[EventType::from(&event.kind).as_str()])
        .inc();
}

pub fn inc_event_failed_counter(event: &ProtocolEvent, reason: &str) {
    EVENT_FAILED_COUNTER
        .with_label_values(&[EventType::from(&event.kind).as_str(), reason])
        .inc();
}

pub fn inc_event_retried_counter(event: &ProtocolEvent) {
    EVENT_RETRIED_COUNTER
        .with_label_values(&[EventType::from(&event.kind).as_str()])
        .inc();
}

pub fn register_event_latency(event: &ProtocolEvent) {
    if matches!(
        event.kind,
        ProtocolEventKind::PublicDecryption(_)
            | ProtocolEventKind::UserDecryption(_)
            | ProtocolEventKind::UserDecryptionV2(_)
    ) {
        let elapsed = Utc::now() - event.created_at;
        DECRYPTION_LATENCY_HISTOGRAM
            .with_label_values(&[EventType::from(&event.kind).as_str()])
            .observe(elapsed.as_seconds_f64());
    }
}
