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

/// Verifies FheRandBounded produces values within the requested bounds.
///
/// Uses per-type bounds that match the old gRPC test to avoid edge cases
/// (e.g. upper_bound=1 produces 0 random bits, which behaves differently
/// on GPU).
#[tokio::test]
#[serial(db)]
async fn test_fhe_random_bounded() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let mut handles = Vec::new();
    let mut rand_types = Vec::new();
    let mut bounds = Vec::new();

    // Per-type bounds matching the old gRPC test to avoid GPU edge cases.
    let type_bounds: &[(i32, &str)] = &[
        (0, "2"),
        (1, "4"),
        (2, "128"),
        (3, "16384"),
        (4, "1073741824"),
        (5, "4611686018427387904"),
        (6, "85070591730234615865843651857942052864"),
        (7, "365375409332725729550921208179070754913983135744"),
        (
            8,
            "28948022309329048855892746252171976963317496166410141009864396001978282409984",
        ),
    ];

    for &(rand_type, bound_str) in type_bounds {
        if !random_test_supported_types().contains(&rand_type) {
            continue;
        }
        let bound = BigInt::from_str(bound_str)?;

        let tx_id = next_handle();
        let mut tx = harness.listener_db.new_transaction().await?;

        let output = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheRandBounded(TfheContract::FheRandBounded {
                caller: zero_address(),
                upperBound: as_scalar_uint(&bound),
                randType: to_ty(rand_type),
                seed: FixedBytes::from([1_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                result: output,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &output).await?;
        tx.commit().await?;

        rand_types.push(rand_type);
        bounds.push(bound);
        handles.push(output);
    }

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &handles).await?;

    for (idx, rand_type) in rand_types.iter().enumerate() {
        let result = &decrypted[idx];
        assert_eq!(result.output_type, *rand_type as i16);

        // Bool is special: only valid values are 0/1, and bound=2 means [0,2).
        if *rand_type == 0 {
            assert!(
                result.value == "true" || result.value == "false",
                "bool rand_bounded should be true or false, got: {}",
                result.value
            );
            continue;
        }

        let result_num = BigInt::from_str(&result.value)?;
        assert!(
            result_num >= BigInt::from(0_u8),
            "type {rand_type}: rand_bounded result should be >= 0, got {result_num}"
        );
        assert!(
            result_num < bounds[idx],
            "type {rand_type}: rand_bounded result {result_num} should be < bound {}",
            bounds[idx]
        );
    }

    Ok(())
}
