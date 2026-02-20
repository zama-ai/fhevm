use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, setup_event_harness, to_ty,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

async fn wait_for_error(
    pool: &sqlx::PgPool,
    output_handle: &[u8],
    tx_id: &[u8],
) -> Result<(bool, Option<String>), Box<dyn std::error::Error>> {
    let mut last_error = None;
    for _ in 0..80 {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        let row = sqlx::query_as::<_, (bool, bool, Option<String>)>(
            r#"SELECT is_error, is_completed, error_message
               FROM computations
               WHERE output_handle = $1 AND transaction_id = $2"#,
        )
        .bind(output_handle)
        .bind(tx_id)
        .fetch_optional(pool)
        .await?;
        if let Some((is_error, is_completed, msg)) = row {
            last_error = msg;
            if is_error || is_completed {
                return Ok((is_error, last_error));
            }
        }
    }
    Ok((false, last_error))
}

#[tokio::test]
#[serial(db)]
async fn test_coprocessor_input_errors() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
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
    .bind(42_i64)
    .execute(&harness.pool)
    .await?;

    let (is_error, msg) = wait_for_error(&harness.pool, &output_handle, &tx_id).await?;
    assert!(
        is_error,
        "expected unknown operation to fail, last_error_message={msg:?}"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_coprocessor_computation_errors() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;

    let input = next_handle();
    insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 42, 5, input, false).await?;

    let output = next_handle();
    insert_event(
        &harness.listener_db,
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
    allow_handle(&harness.listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    let (is_error, msg) = wait_for_error(&harness.pool, output.as_ref(), tx_id.as_ref()).await?;
    assert!(
        is_error,
        "expected invalid cast target type to fail, last_error_message={msg:?}"
    );
    Ok(())
}
