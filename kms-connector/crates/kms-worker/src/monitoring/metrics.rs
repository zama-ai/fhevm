use prometheus::{IntCounter, IntCounterVec, register_int_counter, register_int_counter_vec};
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
