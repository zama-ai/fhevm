use prometheus::{register_int_counter, IntCounter};
use std::sync::LazyLock;

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
pub(crate) static DELEGATE_USER_DECRYPT_SUCCESS_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_txn_sender_delegation_user_decrypt_success_counter",
            "Number of successful delegate user decrypt txns in transaction-sender"
        )
        .unwrap()
    });

pub(crate) static DELEGATE_USER_DECRYPT_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_txn_sender_delegation_user_decrypt_fail_counter",
        "Number of failed delegate user decrypt txns requests in transaction-sender"
    )
    .unwrap()
});
