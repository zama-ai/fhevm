use connector_utils::types::{
    db::EventType,
    kms_response::{PUBLIC_DECRYPTION_RESPONSE_STR, USER_DECRYPTION_RESPONSE_STR},
};
use prometheus::{IntCounterVec, IntGaugeVec, register_int_counter_vec, register_int_gauge_vec};
use sqlx::{Pool, Postgres};
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
    .unwrap()
});

pub static RESPONSE_RECEIVED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_response_received_errors",
        "Number of errors encountered by the TransactionSender while listening for responses",
        &["response_type"]
    )
    .unwrap()
});

pub static GATEWAY_TX_SENT_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_gateway_tx_sent_counter",
        "Number of transactions sent by the TransactionSender to the Gateway",
        &["response_type"]
    )
    .unwrap()
});

pub static GATEWAY_TX_SENT_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_tx_sender_gateway_tx_sent_errors",
        "Number of errors encountered by the TransactionSender while sending transactions to the Gateway",
        &["response_type"]
    )
    .unwrap()
});

// Definition of gauges are in the tx-sender because it is a single entity, contrary to kms-worker
// or gw-listener.
// So the benefit is that a single background job updating these gauges will be spawned.

static PENDING_EVENTS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "kms_connector_pending_events",
        "Number of Gateway events not yet processed by any KmsWorker",
        &["event_type"]
    )
    .unwrap()
});

static PENDING_RESPONSES: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "kms_connector_pending_responses",
        "Number of KMS responses not yet processed by the TransactionSender",
        &["response_type"]
    )
    .unwrap()
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

async fn run_gauge_update_routine(period: Duration, db_pool: Pool<Postgres>) {
    loop {
        match sqlx::query_scalar!("SELECT COUNT(decryption_id) FROM public_decryption_requests")
            .fetch_one(&db_pool)
            .await
        {
            Ok(Some(count)) => PENDING_EVENTS
                .with_label_values(&[EventType::PublicDecryptionRequest.as_str()])
                .set(count),
            Ok(None) => error!("Public decryption requests count is None"),
            Err(e) => error!(error = %e, "Failed to fetch public decryption requests count"),
        }

        match sqlx::query_scalar!("SELECT COUNT(decryption_id) FROM public_decryption_responses")
            .fetch_one(&db_pool)
            .await
        {
            Ok(Some(count)) => PENDING_RESPONSES
                .with_label_values(&[PUBLIC_DECRYPTION_RESPONSE_STR])
                .set(count),
            Ok(None) => error!("Public decryption responses count is None"),
            Err(e) => error!(error = %e, "Failed to fetch public decryption responses count"),
        }

        match sqlx::query_scalar!("SELECT COUNT(decryption_id) FROM user_decryption_requests")
            .fetch_one(&db_pool)
            .await
        {
            Ok(Some(count)) => PENDING_EVENTS
                .with_label_values(&[EventType::UserDecryptionRequest.as_str()])
                .set(count),
            Ok(None) => error!("User decryption requests count is None"),
            Err(e) => error!(error = %e, "Failed to fetch user decryption requests count"),
        }

        match sqlx::query_scalar!("SELECT COUNT(decryption_id) FROM user_decryption_responses")
            .fetch_one(&db_pool)
            .await
        {
            Ok(Some(count)) => PENDING_RESPONSES
                .with_label_values(&[USER_DECRYPTION_RESPONSE_STR])
                .set(count),
            Ok(None) => error!("User decryption responses count is None"),
            Err(e) => error!(error = %e, "Failed to fetch user decryption responses count"),
        }

        tokio::time::sleep(period).await;
    }
}
