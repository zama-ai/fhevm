use crate::tests::event_helpers::{
    allow_handle, as_scalar_uint, decrypt_handles, insert_event, next_handle, setup_event_harness,
    to_ty, wait_until_computed, zero_address,
};
use alloy::primitives::FixedBytes;
use bigdecimal::num_bigint::BigInt;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;
use std::str::FromStr;

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_basic() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;
    let rand_type = 5; // FheUint64

    let output1 = next_handle();
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheRand(TfheContract::FheRand {
            caller: zero_address(),
            randType: to_ty(rand_type),
            seed: FixedBytes::from([0_u8; 16]),
            result: output1,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &output1).await?;

    let output2 = next_handle();
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheRand(TfheContract::FheRand {
            caller: zero_address(),
            randType: to_ty(rand_type),
            seed: FixedBytes::from([0_u8; 16]),
            result: output2,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &output2).await?;

    let output3 = next_handle();
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheRand(TfheContract::FheRand {
            caller: zero_address(),
            randType: to_ty(rand_type),
            seed: FixedBytes::from([1_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            result: output3,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &output3).await?;

    tx.commit().await?;
    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &[output1, output2, output3]).await?;

    assert_eq!(decrypted[0].output_type, rand_type as i16);
    assert_eq!(decrypted[1].output_type, rand_type as i16);
    assert_eq!(decrypted[2].output_type, rand_type as i16);
    assert_eq!(
        decrypted[0].value, decrypted[1].value,
        "random generation must be deterministic for same seed"
    );
    assert_ne!(
        decrypted[0].value, decrypted[2].value,
        "random generation must change when seed changes"
    );

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_bounded() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;
    let rand_type = 5; // FheUint64

    let output1 = next_handle();
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheRandBounded(TfheContract::FheRandBounded {
            caller: zero_address(),
            upperBound: as_scalar_uint(1),
            randType: to_ty(rand_type),
            seed: FixedBytes::from([1_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            result: output1,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &output1).await?;

    let output2 = next_handle();
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheRandBounded(TfheContract::FheRandBounded {
            caller: zero_address(),
            upperBound: as_scalar_uint(1024),
            randType: to_ty(rand_type),
            seed: FixedBytes::from([2_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            result: output2,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &output2).await?;

    tx.commit().await?;
    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &[output1, output2]).await?;

    assert_eq!(decrypted[0].output_type, rand_type as i16);
    assert_eq!(decrypted[1].output_type, rand_type as i16);
    assert_eq!(decrypted[0].value, "0");
    let bounded = BigInt::from_str(&decrypted[1].value)?;
    assert!(bounded >= BigInt::from(0_u8));
    assert!(bounded < BigInt::from(1024_u32));

    Ok(())
}
