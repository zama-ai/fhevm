use prometheus::{register_int_counter, register_int_gauge, IntCounter, IntGauge};
use sqlx::PgPool;
use std::sync::LazyLock;
use tokio::{task::JoinHandle, time::sleep};
use tracing::{error, info};

pub(crate) static VERIFY_PROOF_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_txn_sender_verify_proof_success_counter",
        "Number of successful verify or reject proof txns in transaction-sender"
    )
    .unwrap()
});

pub(crate) static VERIFY_PROOF_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_txn_sender_verify_proof_fail_counter",
        "Number of failed verify or reject proof txns requests in transaction-sender"
    )
    .unwrap()
});

pub(crate) static ADD_CIPHERTEXT_MATERIAL_SUCCESS_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_txn_sender_add_ciphertext_material_success_counter",
            "Number of successful add ciphertext material txns in transaction-sender"
        )
        .unwrap()
    });

pub(crate) static ADD_CIPHERTEXT_MATERIAL_FAIL_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_txn_sender_add_ciphertext_material_fail_counter",
            "Number of failed add ciphertext material txns requests in transaction-sender"
        )
        .unwrap()
    });

pub(crate) static ALLOW_HANDLE_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_txn_sender_allow_handle_success_counter",
        "Number of successful allow handle txns in transaction-sender"
    )
    .unwrap()
});

pub(crate) static ALLOW_HANDLE_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_txn_sender_allow_handle_fail_counter",
        "Number of failed allow handle txns requests in transaction-sender"
    )
    .unwrap()
});

pub(crate) static ALLOW_HANDLE_UNSENT: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_allow_handle_unsent_gauge",
        "Number of unsent allow handle transactions"
    )
    .unwrap()
});

pub(crate) static ADD_CIPHERTEXT_MATERIAL_UNSENT: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_add_ciphertext_material_unsent_gauge",
        "Number of unsent add ciphertext material transactions"
    )
    .unwrap()
});

pub fn spawn_gauge_update_routine(period: std::time::Duration, db_pool: PgPool) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match sqlx::query_scalar(
                "SELECT COUNT(*) FROM allowed_handles WHERE txn_is_sent = FALSE",
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(count) => {
                    info!(unsent_allow_handle_count = %count, "Fetched unsent allow handle count");
                    ALLOW_HANDLE_UNSENT.set(count);
                }
                Err(e) => {
                    error!(error = %e, "Failed to fetch unsent allow handle count");
                }
            }

            match sqlx::query_scalar(
                "SELECT COUNT(*) FROM ciphertext_digest WHERE txn_is_sent = FALSE",
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(count) => {
                    info!(unsent_add_ciphertext_material_count = %count, "Fetched unsent add ciphertext material count");
                    ADD_CIPHERTEXT_MATERIAL_UNSENT.set(count);
                }
                Err(e) => {
                    error!(error = %e, "Failed to fetch unsent add ciphertext material count");
                }
            }

            sleep(period).await;
        }
    })
}
