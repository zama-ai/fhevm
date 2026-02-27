use crate::tests::event_helpers::{
    allow_handle, insert_event, insert_trivial_encrypt, next_handle, setup_event_harness,
    wait_until_computed, zero_address,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{Handle, ScalarByte};
use serial_test::serial;

async fn fetch_ciphertext_row(
    pool: &sqlx::PgPool,
    handle: &Handle,
) -> Result<(i16, Vec<u8>), Box<dyn std::error::Error>> {
    let row = sqlx::query_as::<_, (i16, Vec<u8>)>(
        r#"
        SELECT ciphertext_type, ciphertext
        FROM ciphertexts
        WHERE handle = $1 AND ciphertext_version = $2
        LIMIT 1
        "#,
    )
    .bind(handle.to_vec())
    .bind(current_ciphertext_version())
    .fetch_one(pool)
    .await?;
    Ok(row)
}

#[tokio::test]
#[serial(db)]
async fn test_repeated_fhe_add_has_identical_ciphertext() -> Result<(), Box<dyn std::error::Error>>
{
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let lhs = next_handle();
    let rhs = next_handle();
    let out1 = next_handle();
    let out2 = next_handle();

    let mut tx = harness.listener_db.new_transaction().await?;
    insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 41, 4, lhs, true).await?;
    allow_handle(&harness.listener_db, &mut tx, &lhs).await?;
    insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 1, 4, rhs, true).await?;
    allow_handle(&harness.listener_db, &mut tx, &rhs).await?;

    for output in [out1, out2] {
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller: zero_address(),
                lhs,
                rhs,
                scalarByte: ScalarByte::from(0_u8),
                result: output,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &output).await?;
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;

    let (ty1, ct1) = fetch_ciphertext_row(&harness.pool, &out1).await?;
    let (ty2, ct2) = fetch_ciphertext_row(&harness.pool, &out2).await?;
    assert_eq!(ty1, ty2, "output ciphertext types differ");
    assert_eq!(
        ct1, ct2,
        "same operation with identical inputs must produce identical ciphertext bytes"
    );

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_reused_operands_across_transactions_are_deterministic(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;

    // Create two base ciphertext operands once.
    let lhs = next_handle();
    let rhs = next_handle();
    let seed_tx = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;
    // Insert in a non-canonical order to ensure arrival order does not matter.
    insert_trivial_encrypt(&harness.listener_db, &mut tx, seed_tx, 1, 4, rhs, true).await?;
    allow_handle(&harness.listener_db, &mut tx, &rhs).await?;
    insert_trivial_encrypt(&harness.listener_db, &mut tx, seed_tx, 41, 4, lhs, true).await?;
    allow_handle(&harness.listener_db, &mut tx, &lhs).await?;
    tx.commit().await?;
    wait_until_computed(&harness.app).await?;

    // Reuse the same operand handles in distinct transactions.
    let outputs = (0..4).map(|_| next_handle()).collect::<Vec<_>>();
    for output in &outputs {
        let mut tx = harness.listener_db.new_transaction().await?;
        insert_event(
            &harness.listener_db,
            &mut tx,
            next_handle(),
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller: zero_address(),
                lhs,
                rhs,
                scalarByte: ScalarByte::from(0_u8),
                result: *output,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, output).await?;
        tx.commit().await?;
    }

    wait_until_computed(&harness.app).await?;

    let (expected_type, expected_ct) = fetch_ciphertext_row(&harness.pool, &outputs[0]).await?;
    for output in outputs.iter().skip(1) {
        let (ct_type, ct) = fetch_ciphertext_row(&harness.pool, output).await?;
        assert_eq!(ct_type, expected_type, "output ciphertext types differ");
        assert_eq!(
            ct, expected_ct,
            "reused encrypted operands should produce identical ciphertext bytes across transactions"
        );
    }

    Ok(())
}
