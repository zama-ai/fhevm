use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, upsert_test_dcid, wait_for_error, zero_address, EventHarness,
    TEST_CHAIN_ID,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;
use sqlx::Row;

#[tokio::test]
#[serial(db)]
async fn test_coprocessor_input_errors() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db: _listener_db,
    } = setup_event_harness().await?;
    let output_handle = next_handle().to_vec();
    let tx_id = next_handle().to_vec();
    let dcid = next_handle().to_vec();
    // The chain row must describe the same block coordinates as the
    // computation row it schedules.
    let block_number = 0_i64;
    let producer_block_hash = vec![0xE0u8; 32];

    sqlx::query(
        r#"
        INSERT INTO computations_branch (
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
            block_number,
            producer_block_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $9, $10, $11)
        "#,
    )
    .bind(&output_handle)
    .bind(Vec::<Vec<u8>>::new())
    .bind(127_i16) // unknown operation
    .bind(false)
    .bind(&dcid)
    .bind(tx_id.clone())
    .bind(true)
    .bind(false)
    .bind(TEST_CHAIN_ID as i64)
    // A block-keyed producer hash requires a block number
    // (computations_branch_producer_block_number_check).
    .bind(block_number)
    .bind(&producer_block_hash)
    .execute(&pool)
    .await?;
    // The row is inserted directly (not through the listener helpers), so its
    // dependence chain must be marked schedulable explicitly.
    upsert_test_dcid(&pool, &dcid, block_number as u64, &producer_block_hash).await?;

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

/// Invalid operand arity is rejected while building the worker graph. It must
/// become terminal instead of aborting and retrying the complete worker cycle,
/// otherwise the block can never settle.
#[tokio::test]
#[serial(db)]
async fn test_invalid_operand_arity_becomes_terminal() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db: _listener_db,
    } = setup_event_harness().await?;
    let output_handle = next_handle().to_vec();
    let tx_id = next_handle().to_vec();
    let dcid = next_handle().to_vec();
    let block_number = 0_i64;
    let producer_block_hash = vec![0xE1u8; 32];

    sqlx::query(
        r#"
        INSERT INTO computations_branch (
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
            block_number,
            producer_block_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $9, $10, $11)
        "#,
    )
    .bind(&output_handle)
    .bind(Vec::<Vec<u8>>::new()) // FheAdd requires two operands.
    .bind(0_i16)
    .bind(false)
    .bind(&dcid)
    .bind(tx_id.clone())
    .bind(true)
    .bind(false)
    .bind(TEST_CHAIN_ID as i64)
    .bind(block_number)
    .bind(&producer_block_hash)
    .execute(&pool)
    .await?;
    upsert_test_dcid(&pool, &dcid, block_number as u64, &producer_block_hash).await?;

    let (is_error, msg) = wait_for_error(&pool, &output_handle, &tx_id).await?;
    assert!(
        is_error,
        "invalid operand arity must be terminal, last_error_message={msg:?}"
    );
    assert!(
        msg.as_deref()
            .is_some_and(|message| message.contains("unexpected operand count")),
        "expected operand-count error, got: {msg:?}"
    );
    Ok(())
}

/// FheSub on mismatched types (uint32 + uint64) fails at execution time with
/// `UnsupportedFheTypes`.  This is a reliable execution-time error on both CPU
/// and GPU (unlike Cast-to-invalid-type which panics on the GPU path during
/// memory reservation).
#[tokio::test]
#[serial(db)]
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

    let dependent_output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheNeg(TfheContract::FheNeg {
            caller: zero_address(),
            ct: output,
            result: dependent_output,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &dependent_output).await?;
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

    let (dependent_is_error, dependent_msg) =
        wait_for_error(&pool, dependent_output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        dependent_is_error,
        "dependent computation must become terminal when its root fails"
    );
    assert!(
        dependent_msg
            .as_deref()
            .is_some_and(|msg| msg.contains("blocked by terminal computation error")),
        "dependent computation must record a derived error, got: {dependent_msg:?}"
    );

    let root = sqlx::query(
        r#"
        SELECT error_root_output_handle,
               error_root_transaction_id,
               error_root_producer_block_hash
          FROM computations_branch
         WHERE output_handle = $1
           AND transaction_id = $2
           AND producer_block_hash <> ''::BYTEA
        "#,
    )
    .bind(dependent_output.as_slice())
    .bind(tx_id.as_slice())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        root.get::<Option<Vec<u8>>, _>("error_root_output_handle")
            .as_deref(),
        Some(output.as_slice())
    );
    assert_eq!(
        root.get::<Option<Vec<u8>>, _>("error_root_transaction_id")
            .as_deref(),
        Some(tx_id.as_slice())
    );
    assert!(
        root.get::<Option<Vec<u8>>, _>("error_root_producer_block_hash")
            .is_some(),
        "derived error must record the root block"
    );
    Ok(())
}

/// FheAdd on mismatched types (uint8 + uint16) passes validation in
/// `check_fhe_operand_types` but fails at execution time with `UnsupportedFheTypes`.
#[tokio::test]
#[serial(db)]
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
#[serial(db)]
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
#[serial(db)]
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
