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

const RANDOM_SUPPORTED_TYPES_CPU: &[i32] = &[
    0,  // bool
    1,  // 4 bit
    2,  // 8 bit
    3,  // 16 bit
    4,  // 32 bit
    5,  // 64 bit
    6,  // 128 bit
    7,  // 160 bit
    8,  // 256 bit
    9,  // 512 bit
    10, // 1024 bit
    11, // 2048 bit
];

const RANDOM_SUPPORTED_TYPES_GPU: &[i32] = &[
    0, // bool
    1, // 4 bit
    2, // 8 bit
    3, // 16 bit
    4, // 32 bit
    5, // 64 bit
    6, // 128 bit
    7, // 160 bit
    8, // 256 bit
];

fn random_test_supported_types() -> &'static [i32] {
    if cfg!(feature = "gpu") {
        RANDOM_SUPPORTED_TYPES_GPU
    } else {
        RANDOM_SUPPORTED_TYPES_CPU
    }
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_basic() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut handles = Vec::new();
    let mut rand_types = Vec::new();

    for &rand_type in random_test_supported_types() {
        let tx_id = next_handle();
        let mut tx = harness.listener_db.new_transaction().await?;

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

        rand_types.push(rand_type);
        handles.extend([output1, output2, output3]);
    }

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &handles).await?;

    for (idx, rand_type) in rand_types.iter().enumerate() {
        let base = idx * 3;
        let first = &decrypted[base];
        let second = &decrypted[base + 1];
        let third = &decrypted[base + 2];
        assert_eq!(first.output_type, *rand_type as i16);
        assert_eq!(second.output_type, *rand_type as i16);
        assert_eq!(third.output_type, *rand_type as i16);
        assert_eq!(
            first.value, second.value,
            "random generation must be deterministic for same seed"
        );
        if *rand_type != 0 {
            assert_ne!(
                first.value, third.value,
                "random generation must change when seed changes"
            );
        }
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_bounded() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut handles = Vec::new();
    let mut rand_types = Vec::new();

    for &rand_type in random_test_supported_types() {
        let tx_id = next_handle();
        let mut tx = harness.listener_db.new_transaction().await?;

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

        rand_types.push(rand_type);
        handles.extend([output1, output2]);
    }

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &handles).await?;

    for (idx, rand_type) in rand_types.iter().enumerate() {
        let base = idx * 2;
        let unbounded = &decrypted[base];
        let bounded = &decrypted[base + 1];
        assert_eq!(unbounded.output_type, *rand_type as i16);
        assert_eq!(bounded.output_type, *rand_type as i16);

        let unbounded_num = if unbounded.value == "true" {
            BigInt::from(1_u8)
        } else if unbounded.value == "false" {
            BigInt::from(0_u8)
        } else {
            BigInt::from_str(&unbounded.value)?
        };
        assert_eq!(unbounded_num, BigInt::from(0_u8));

        let bounded_num = if bounded.value == "true" {
            BigInt::from(1_u8)
        } else if bounded.value == "false" {
            BigInt::from(0_u8)
        } else {
            BigInt::from_str(&bounded.value)?
        };
        assert!(bounded_num >= BigInt::from(0_u8));
        assert!(bounded_num < BigInt::from(1024_u32));
    }

    Ok(())
}
