use prometheus::{IntCounter, register_int_counter};
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_event_received_counter",
        "Number of events received by the KmsWorker"
    )
    .unwrap()
});

pub static EVENT_RECEIVED_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_event_received_errors",
        "Number of errors encountered by the KmsWorker while listening for events"
    )
    .unwrap()
});

pub static CORE_REQUEST_SENT_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_core_request_sent_counter",
        "Number of requests sent by the KmsWorker to the KMS Core"
    )
    .unwrap()
});

pub static CORE_REQUEST_SENT_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_core_request_sent_errors",
        "Number of errors encountered by the KmsWorker while sending requests to the KMS Core"
    )
    .unwrap()
});

pub static CORE_RESPONSE_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_core_response_counter",
        "Number of responses received by the KmsWorker from the KMS Core"
    )
    .unwrap()
});

pub static CORE_RESPONSE_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_worker_core_response_errors",
        "Number of errors encountered by the KmsWorker while receiving responses from the KMS Core"
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
