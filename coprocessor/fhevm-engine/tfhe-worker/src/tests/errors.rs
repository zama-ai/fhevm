use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, to_ty, wait_for_error, zero_address, EventHarness, TEST_CHAIN_ID,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::Handle;
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

    let input = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 42, 5, input, false).await?;

    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::Cast(TfheContract::Cast {
            caller: crate::tests::event_helpers::zero_address(),
            ct: input,
            toType: to_ty(255),
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
        "expected invalid cast target type to fail, last_error_message={msg:?}"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_circular_dependency_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db: _listener_db,
    } = setup_event_harness().await?;

    let handle_a = next_handle().to_vec();
    let handle_b = next_handle().to_vec();
    let handle_c = next_handle().to_vec();
    let tx_id = next_handle().to_vec();
    let dcid = next_handle().to_vec();

    // A depends on C, B depends on A, C depends on B  →  cycle
    for (output, dep) in [
        (&handle_a, &handle_c),
        (&handle_b, &handle_a),
        (&handle_c, &handle_b),
    ] {
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
        .bind(output)
        .bind(vec![dep.clone()])
        .bind(0_i16) // FheAdd
        .bind(false)
        .bind(&dcid)
        .bind(&tx_id)
        .bind(true)
        .bind(false)
        .bind(TEST_CHAIN_ID as i64)
        .execute(&pool)
        .await?;
    }

    let (is_error, msg) = wait_for_error(&pool, &handle_a, &tx_id).await?;
    assert!(
        is_error,
        "expected circular dependency to fail, last_error_message={msg:?}"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_too_many_inputs_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let tx_id = next_handle();
    let mut tx = listener_db.new_transaction().await?;

    let dep1 = next_handle();
    let dep2 = next_handle();
    let dep3 = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 1, 5, dep1, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 2, 5, dep2, false).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 3, 5, dep3, false).await?;
    tx.commit().await?;

    // Insert FheAdd (binary op) with 3 dependencies via raw SQL
    let output = next_handle().to_vec();
    let dcid = tx_id.to_vec();
    let tx_id_bytes = tx_id.to_vec();
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
    .bind(&output)
    .bind(vec![dep1.to_vec(), dep2.to_vec(), dep3.to_vec()])
    .bind(0_i16) // FheAdd
    .bind(false)
    .bind(&dcid)
    .bind(&tx_id_bytes)
    .bind(true)
    .bind(false)
    .bind(TEST_CHAIN_ID as i64)
    .execute(&pool)
    .await?;

    let (is_error, msg) = wait_for_error(&pool, &output, &tx_id_bytes).await?;
    assert!(
        is_error,
        "expected too many inputs to fail, last_error_message={msg:?}"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_scalar_division_by_zero_error() -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app: _app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let tx_id = next_handle();
    let mut tx = listener_db.new_transaction().await?;

    let lhs = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, tx_id, 42, 5, lhs, false).await?;

    // FheDiv with scalar RHS = all-zeros handle (zero value), scalar flag = true
    let output = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheDiv(TfheContract::FheDiv {
            caller: zero_address(),
            lhs,
            rhs: Handle::default(),
            scalarByte: scalar_flag(true),
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
        "expected scalar division by zero to fail, last_error_message={msg:?}"
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
