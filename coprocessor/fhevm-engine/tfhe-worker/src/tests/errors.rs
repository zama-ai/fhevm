use std::ops::DerefMut;

use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, next_handle_with_type,
    scalar_flag, setup_event_harness, wait_for_error, zero_address, EventHarness, TEST_CHAIN_ID,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;

#[tokio::test]
async fn test_coprocessor_input_errors() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db: _listener_db,
    } = setup_event_harness().await?;
    let output_handle = next_handle().to_vec();
    let tx_id = next_handle().to_vec();
    let dcid = next_handle().to_vec();

    sqlx::query(
        r#"
        INSERT INTO computations (
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            dependence_chain_id,
            transaction_id,
            is_allowed,
            created_at,
            schedule_order,
            is_completed,
            host_chain_id,
            operand_boundary_mask
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $9, $10)
        "#,
    )
    .bind(&output_handle)
    .bind(Vec::<Vec<u8>>::new())
    .bind(127_i16) // unknown operation
    .bind(false)
    .bind(dcid)
    .bind(tx_id.clone())
    .bind(true)
    .bind(false)
    .bind(TEST_CHAIN_ID as i64)
    // This fixture bypasses ordered listener ingestion. Its invalid opcode
    // has no operands, so the executor-compatible authoritative mask is the
    // all-zero uint256 value.
    .bind(vec![0_u8; 32])
    .execute(&pool)
    .await?;

    let (is_error, msg) = wait_for_error(&pool, &output_handle, &tx_id).await?;
    assert!(
        is_error,
        "expected unknown operation to fail, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("Unknown fhe operation"),
        "expected 'Unknown fhe operation' error, got: {error_msg}"
    );
    Ok(())
}

/// FheSub on mismatched types (uint32 + uint64) fails at execution time with
/// `UnsupportedFheTypes`.  This is a reliable execution-time error on both CPU
/// and GPU (unlike Cast-to-invalid-type which panics on the GPU path during
/// memory reservation).
#[tokio::test]
async fn test_coprocessor_computation_errors() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    let lhs = next_handle();
    let rhs = next_handle();
    // lhs is uint32 (type 4), rhs is uint64 (type 5)
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 10, 4, lhs, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 20, 5, rhs, false).await?;

    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheSub(TfheContract::FheSub {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: scalar_flag(false),
            result: output,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "expected FheSub on mismatched types to fail, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("UnsupportedFheTypes"),
        "expected UnsupportedFheTypes error, got: {error_msg}"
    );
    Ok(())
}

/// An INTERNAL producer's execution-time verdict lands on its allowed
/// consumer, through the real scheduler.
///
/// The listener stores a non-allowed row `is_completed = TRUE`, so the
/// producer's own row can never carry a verdict; the worker records it on the
/// allowed rows downstream instead, found from the dataflow graph. That lookup
/// has to be answered from a snapshot taken BEFORE `schedule()`: the scheduler
/// takes each transaction's inner graph out of its node at dispatch, and a walk
/// over the live graph afterwards returns nothing -- which left the consumer
/// pending and the transaction re-executing forever. A unit test that hands the
/// blocked set to `set_computation_error` by hand cannot see that, so this one
/// drives the whole path.
#[tokio::test]
async fn internal_producer_execution_error_lands_on_its_allowed_consumer(
) -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    let lhs = next_handle();
    let rhs = next_handle();
    // lhs is uint32 (type 4), rhs is uint64 (type 5): FheSub passes
    // `check_fhe_operand_types` and fails at execution with
    // UnsupportedFheTypes, deterministically, on both CPU and GPU.
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 10, 4, lhs, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 20, 5, rhs, false).await?;

    // The failing op is an INTERNAL producer: not allowed, stored completed.
    let producer = next_handle_with_type(4);
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheSub(TfheContract::FheSub {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: scalar_flag(false),
            result: producer,
        }),
        false,
    )
    .await?;
    // Its allowed consumer is the only row of this transaction that can
    // carry a verdict.
    let consumer = next_handle_with_type(4);
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheNot(TfheContract::FheNot {
            caller: zero_address(),
            ct: producer,
            result: consumer,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &consumer).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, consumer.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "the allowed consumer must carry its internal producer's verdict; \
         left pending, the transaction re-executes forever. \
         last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("UnsupportedFheTypes"),
        "expected the producer's UnsupportedFheTypes verdict, got: {error_msg}"
    );
    assert!(
        !error_msg.contains("RETRYABLE"),
        "a type verdict is terminal and must not be stamped retryable: {error_msg}"
    );

    // The producer row itself is untouched: completed, not errored. The
    // verdict lives where the work window and the demotion predicates read.
    let (producer_completed, producer_errored): (bool, bool) = sqlx::query_as(
        "SELECT is_completed, is_error FROM computations
         WHERE output_handle = $1 AND transaction_id = $2",
    )
    .bind(producer.to_vec())
    .bind(tx_id.to_vec())
    .fetch_one(&pool)
    .await?;
    assert!(
        producer_completed,
        "an internal producer is stored completed"
    );
    assert!(
        !producer_errored,
        "an internal producer's own row cannot carry the verdict"
    );
    Ok(())
}

/// FheAdd on mismatched types (uint8 + uint16) passes validation in
/// `check_fhe_operand_types` but fails at execution time with `UnsupportedFheTypes`.
#[tokio::test]
async fn test_type_mismatch_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    let lhs = next_handle();
    let rhs = next_handle();
    // lhs is uint8 (type 2), rhs is uint16 (type 3)
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 1, 2, lhs, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 1, 3, rhs, false).await?;

    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: scalar_flag(false),
            result: output,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "expected FheAdd on mismatched types to fail, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("UnsupportedFheTypes"),
        "expected UnsupportedFheTypes error, got: {error_msg}"
    );
    Ok(())
}

#[tokio::test]
async fn test_binary_boolean_inputs_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    let lhs = next_handle();
    let rhs = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 1, 0, lhs, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 0, 0, rhs, false).await?;

    // FheAdd on bool inputs → UnsupportedFheTypes
    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: scalar_flag(false),
            result: output,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "expected FheAdd on bool inputs to fail, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("UnsupportedFheTypes"),
        "expected UnsupportedFheTypes error, got: {error_msg}"
    );
    Ok(())
}

#[tokio::test]
async fn test_unary_boolean_inputs_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let tx_id = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    let input = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 1, 0, input, false).await?;

    // FheNeg on bool input → UnsupportedFheTypes
    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheNeg(TfheContract::FheNeg {
            caller: zero_address(),
            ct: input,
            result: output,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "expected FheNeg on bool input to fail, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("UnsupportedFheTypes"),
        "expected UnsupportedFheTypes error, got: {error_msg}"
    );
    Ok(())
}

/// A cross-transaction consumer of a terminally-errored producer must drain
/// terminally itself instead of deferring as MissingInputs forever: the
/// producer's bytes can never exist (re-execution of a deterministic error
/// fails identically), so retrying the consumer re-anchors its chain without
/// ever making progress.
#[tokio::test]
async fn errored_producer_drains_cross_transaction_consumer(
) -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    // Producer transaction: FheSub on mismatched types (uint32 + uint64)
    // fails deterministically at execution time.
    let producer_tx = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let lhs = next_handle();
    let rhs = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, producer_tx, 10, 4, lhs, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, producer_tx, 20, 5, rhs, false).await?;
    let producer = next_handle_with_type(5);
    insert_event(
        &listener_db,
        &mut tx,
        producer_tx,
        TfheContractEvents::FheSub(TfheContract::FheSub {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: scalar_flag(false),
            result: producer,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &producer).await?;
    tx.commit().await?;
    let (is_error, msg) = wait_for_error(&pool, producer.as_ref(), producer_tx.as_ref()).await?;
    assert!(
        is_error,
        "expected producer to fail terminally, last_error_message={msg:?}"
    );

    // Consumer transaction: boundary-consumes the errored producer's handle
    // (not minted here, so its mask bit obligates the canonical persisted
    // form — which will never exist).
    let consumer_tx = next_handle();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let local = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, consumer_tx, 1, 5, local, false).await?;
    let consumer = next_handle_with_type(5);
    insert_event(
        &listener_db,
        &mut tx,
        consumer_tx,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: producer,
            rhs: local,
            scalarByte: scalar_flag(false),
            result: consumer,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &consumer).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&pool, consumer.as_ref(), consumer_tx.as_ref()).await?;
    assert!(
        is_error,
        "expected consumer of a dead boundary input to drain terminally, last_error_message={msg:?}"
    );
    let error_msg = msg.as_deref().unwrap_or("");
    assert!(
        error_msg.contains("dead boundary input"),
        "expected dead-boundary-input error, got: {error_msg}"
    );
    Ok(())
}

/// A retryable (panic) stamp is not a dead end: the work window re-selects
/// the stamped row, re-executes it, and a successful retry heals the stamp
/// through the completion path — bytes present, is_error cleared. Modeled by
/// stamping a perfectly computable row with an ExecutionPanic message in the
/// same transaction that inserts it, so the worker only ever sees the
/// stamped state.
#[tokio::test]
async fn retryable_panic_stamp_is_retried_and_healed() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let tx_id = next_handle();
    let output = next_handle_with_type(5);
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 42, 5, output, true).await?;
    allow_handle(&listener_db, &mut tx, &output).await?;
    // Stamp it as a transient panic BEFORE the commit the worker can see.
    sqlx::query(
        r#"UPDATE computations
           SET is_error = true,
               error_message = 'RETRYABLE Coprocessor scheduler error: ExecutionPanic("simulated device pressure")'
           WHERE output_handle = $1 AND transaction_id = $2"#,
    )
    .bind(output.as_slice())
    .bind(tx_id.as_slice())
    .execute(tx.deref_mut())
    .await?;
    tx.commit().await?;

    for _ in 0..240 {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        let row = sqlx::query_as::<_, (bool, bool)>(
            "SELECT is_completed, is_error FROM computations
             WHERE output_handle = $1 AND transaction_id = $2",
        )
        .bind(output.as_slice())
        .bind(tx_id.as_slice())
        .fetch_one(&pool)
        .await?;
        if row.0 {
            assert!(!row.1, "a healed row must not stay errored");
            let bytes: Option<Vec<u8>> =
                sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
                    .bind(output.as_slice())
                    .fetch_optional(&pool)
                    .await?;
            assert!(bytes.is_some_and(|b| !b.is_empty()));
            return Ok(());
        }
    }
    panic!("panic-stamped row was never retried and healed");
}
