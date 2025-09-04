use prometheus::{IntCounter, register_int_counter};
use std::sync::LazyLock;

pub static RESPONSE_RECEIVED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_tx_sender_response_received_counter",
        "Number of responses received by the TransactionSender"
    )
    .unwrap()
});

pub static RESPONSE_RECEIVED_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_tx_sender_response_received_errors",
        "Number of errors encountered by the TransactionSender while listening for responses"
    )
    .unwrap()
});

pub static GATEWAY_TX_SENT_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_tx_sender_gateway_tx_sent_counter",
        "Number of transactions sent by the TransactionSender to the Gateway"
    )
    .unwrap()
});

pub static GATEWAY_TX_SENT_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_tx_sender_gateway_tx_sent_errors",
        "Number of errors encountered by the TransactionSender while sending transactions to the Gateway"
    )
    .unwrap()
});
