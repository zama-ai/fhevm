use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, wait_until_computed, zero_address, EventHarness,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

/// Under the minted-in-transaction handle discriminant, a consumer's
/// representation is pinned by its handle: operands minted in the consuming
/// transaction are read raw, everything else is read in its canonical
/// persisted form — and a representation-mixing alias mints a DIFFERENT
/// handle, so no byte-equality obligation ties the two variants below
/// together (on chain they could never collide). What this pins instead:
/// both sourcing shapes compute correctly through the scheduler — a
/// cross-transaction boundary consumer (canonical decompression path) and a
/// local recompute whose add reads its in-transaction trivials raw — and
/// deterministic trivial encrypts persist byte-identical ciphertexts from
/// either transaction.
#[tokio::test]
#[serial(db)]
async fn boundary_and_local_sourcing_both_compute_and_persist(
) -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let caller = zero_address();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    // Variant ONE: `combined` consumes persisted, allowed boundary inputs
    // produced by a DIFFERENT transaction.
    let produce_tx = next_handle();
    let boundary_b = next_handle();
    let boundary_c = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, produce_tx, 7, 5, boundary_b, true).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, produce_tx, 5, 5, boundary_c, true).await?;
    allow_handle(&listener_db, &mut tx, &boundary_b).await?;
    allow_handle(&listener_db, &mut tx, &boundary_c).await?;

    let storage_tx = next_handle();
    let combined_from_boundaries = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        storage_tx,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: boundary_b,
            rhs: boundary_c,
            scalarByte: scalar_flag(false),
            result: combined_from_boundaries,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &combined_from_boundaries).await?;

    // Variant TWO: one transaction recomputes the same inputs locally and
    // combines them. The inputs are allowed (persisted), but the add was
    // minted in the same transaction as its operands, so it consumes the
    // raw working values; on chain its handle folds zero boundary bits and
    // cannot collide with variant ONE's.
    let local_tx = next_handle();
    let local_b = next_handle();
    let local_c = next_handle();
    let combined_locally = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, local_tx, 7, 5, local_b, true).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, local_tx, 5, 5, local_c, true).await?;
    allow_handle(&listener_db, &mut tx, &local_b).await?;
    allow_handle(&listener_db, &mut tx, &local_c).await?;
    insert_event(
        &listener_db,
        &mut tx,
        local_tx,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: local_b,
            rhs: local_c,
            scalarByte: scalar_flag(false),
            result: combined_locally,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &combined_locally).await?;

    tx.commit().await?;
    wait_until_computed(&app).await?;

    let fetch = |handle: Vec<u8>| {
        let pool = pool.clone();
        async move {
            sqlx::query_scalar::<_, Vec<u8>>(
                "SELECT ciphertext FROM ciphertexts WHERE handle = $1",
            )
            .bind(handle)
            .fetch_one(&pool)
            .await
        }
    };

    // The deterministic trivial encrypts must already persist identical
    // bytes; this isolates a failure below to the add's input sourcing.
    let boundary_b_bytes = fetch(boundary_b.to_vec()).await?;
    let local_b_bytes = fetch(local_b.to_vec()).await?;
    assert_eq!(
        boundary_b_bytes, local_b_bytes,
        "deterministic trivial encrypts must persist identical bytes"
    );

    // Both outputs persist (allowed) and decrypt correctly; their bytes are
    // intentionally NOT compared — the variants mint different handles on
    // chain, so each carries its own byte identity.
    let from_boundaries = fetch(combined_from_boundaries.to_vec()).await?;
    let from_local = fetch(combined_locally.to_vec()).await?;
    assert!(!from_boundaries.is_empty());
    assert!(!from_local.is_empty());

    let plaintexts = decrypt_handles(
        &pool,
        &[combined_from_boundaries, combined_locally],
    )
    .await?;
    assert_eq!(plaintexts[0].value, "12");
    assert_eq!(plaintexts[1].value, "12");
    Ok(())
}
