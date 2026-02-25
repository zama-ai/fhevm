use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, wait_for_error, zero_address, EventHarness, TEST_CHAIN_ID,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

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
            host_chain_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW(), $8, $9)
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
    .execute(&pool)
    .await?;

    let (is_error, msg) = wait_for_error(&pool, &output_handle, &tx_id).await?;
    assert!(
        is_error,
        "expected unknown operation to fail, last_error_message={msg:?}"
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
    let mut tx = listener_db.new_transaction().await?;

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
    let mut tx = listener_db.new_transaction().await?;

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
    let mut tx = listener_db.new_transaction().await?;

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
    let mut tx = listener_db.new_transaction().await?;

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
    Ok(())
}
