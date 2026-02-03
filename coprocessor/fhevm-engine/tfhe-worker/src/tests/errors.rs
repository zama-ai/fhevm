use bigdecimal::num_bigint::BigInt;

use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::ClearConst;
use serial_test::serial;
use sqlx::Row;

use crate::tests::events::{
    allow_handle, insert_tfhe_event, listener_db, next_handle, tfhe_log, to_ty,
};
use crate::tests::utils::{setup_test_app, wait_until_all_allowed_handles_computed};

fn as_clear_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

#[tokio::test]
#[serial(db)]
async fn test_invalid_cast_marks_error() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener = listener_db(&app).await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let transaction_id = next_handle();
    let input_handle = next_handle();
    let output_handle = next_handle();
    let fhe_type_u64 = 5;
    let invalid_fhe_type = 255;

    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    let mut tx = listener.new_transaction().await?;

    // ct = 10 (not allowed for decryption)
    insert_tfhe_event(
        &listener,
        &mut tx,
        tfhe_log(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: as_clear_uint(&BigInt::from(10)),
                toType: to_ty(fhe_type_u64),
                result: input_handle,
            }),
            transaction_id,
        ),
        false,
    )
    .await?;

    // output = cast(ct, invalid_type)
    insert_tfhe_event(
        &listener,
        &mut tx,
        tfhe_log(
            TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: input_handle,
                toType: to_ty(invalid_fhe_type),
                result: output_handle,
            }),
            transaction_id,
        ),
        true,
    )
    .await?;
    allow_handle(&listener, &mut tx, output_handle.as_ref()).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let row = sqlx::query(
        "SELECT is_error, error_message FROM computations WHERE tenant_id = $1 AND output_handle = $2 AND transaction_id = $3",
    )
    .bind(1_i32)
    .bind(output_handle.to_vec())
    .bind(transaction_id.to_vec())
    .fetch_one(&pool)
    .await?;

    let is_error: bool = row.try_get("is_error")?;
    let msg: Option<String> = row.try_get("error_message")?;
    assert!(is_error);
    let msg = msg.unwrap_or_default();
    assert!(
        msg.contains("UnknownCastType") || msg.contains("UnknownFheType"),
        "unexpected error message: {msg}"
    );

    Ok(())
}
