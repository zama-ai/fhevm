//! Tests of Unmask: a pad-based `VerifyInput` event, the ct64 that the pad-worker stores, and an
//! operation on the unmasked input. The tests write the ct64 in place of the pad-worker.

use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_event, next_handle, next_handle_with_type, scalar_flag,
    scalar_u128_handle, setup_event_harness, wait_for_error, wait_until_computed, zero_address,
    EventHarness, TEST_CHAIN_ID,
};
use alloy::primitives::{keccak256, Bytes, B256};
use fhevm_engine_common::pad::{mock::mock_pad, mock::MockOneTimePadMask, UNMASK_TRIGGERED_BY};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::Handle;
use std::time::Duration;

const PRF_ID: u16 = 1;

/// A package of `parts` with the values `plain` (each part LSB first), masked with the mock pad.
fn package(parts: &[u8], plain: &[u8]) -> (Vec<u8>, B256) {
    let mut witness = vec![0u8];
    witness.extend(PRF_ID.to_be_bytes());
    witness.extend(TEST_CHAIN_ID.to_be_bytes());
    witness.extend([0x11; 20]);
    witness.extend([0x22; 20]);
    witness.extend(u64::MAX.to_be_bytes());
    witness.push(0);
    // Each package gets a new nonce, so each test has its own label.
    static NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    witness.extend([0u8; 8]);
    witness.extend(
        NONCE
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            .to_be_bytes(),
    );
    let label = keccak256([b"FHE_padl".as_slice(), &witness].concat());
    let pad = mock_pad(PRF_ID, label, plain.len() * 8);
    let mut bytes = vec![0u8];
    bytes.extend(witness);
    bytes.extend(label);
    bytes.push(u8::try_from(parts.len()).unwrap());
    bytes.extend(parts);
    bytes.extend(plain.iter().zip(&pad).map(|(m, r)| m ^ r));
    (bytes, label)
}

/// The external handle of input `index`: `packageDigest[0..29] | index | type | version`.
fn external_handle(package: &[u8], index: u8, fhe_type: u8) -> Handle {
    let mut handle = keccak256([b"FHE_padp".as_slice(), package].concat()).0;
    handle[29] = index;
    handle[30] = fhe_type;
    handle[31] = 0;
    Handle::from(handle)
}

fn verify_input(package: &[u8], index: u8, fhe_type: u8, result: Handle) -> TfheContractEvents {
    TfheContractEvents::VerifyInput(TfheContract::VerifyInput {
        caller: zero_address(),
        inputHandle: external_handle(package, index, fhe_type),
        userAddress: zero_address(),
        inputProof: Bytes::from(package.to_vec()),
        inputType: fhe_type,
        result,
    })
}

/// Waits for the `pads` row that the tfhe-worker adds for a missing ct64.
async fn wait_for_pad_request(pool: &sqlx::PgPool, label: B256) -> i32 {
    for _ in 0..240 {
        let pad_length: Option<i32> = sqlx::query_scalar(
            "SELECT pad_length FROM pads WHERE label = $1 AND triggered_by = $2",
        )
        .bind(label.as_slice())
        .bind(UNMASK_TRIGGERED_BY.as_slice())
        .fetch_optional(pool)
        .await
        .unwrap();
        if let Some(pad_length) = pad_length {
            return pad_length;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    panic!("no pads row for the Unmask");
}

/// Writes the ct64 as the pad-worker does.
async fn store_ct64(pool: &sqlx::PgPool, label: B256, pad_length: i32) {
    let ct64 = MockOneTimePadMask::new_random(PRF_ID, label, usize::try_from(pad_length).unwrap())
        .unwrap()
        .compress()
        .into_bytes();
    sqlx::query(
        "INSERT INTO pad_ct64 (prf_instance, label, pad_length, ciphertext64, ciphertext_digest,
                               ciphertext128_digest, signer, signature, attestation_version)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1)",
    )
    .bind(i32::from(PRF_ID))
    .bind(label.as_slice())
    .bind(pad_length)
    .bind(&ct64)
    .bind(keccak256(&ct64).as_slice())
    .bind([0u8; 32].as_slice())
    .bind([0u8; 20].as_slice())
    .bind([0u8; 65].as_slice())
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn unmasks_pad_inputs_and_computes_with_them() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    // The values: euint64 40 and ebool true, each LSB first.
    let mut plain = 40u64.to_le_bytes().to_vec();
    plain.push(1);
    let (package, label) = package(&[5, 0], &plain);
    let tx_id = next_handle();
    let number = next_handle_with_type(5);
    let flag = next_handle_with_type(0);
    let sum = next_handle_with_type(5);

    let mut tx = harness
        .listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let event = |index, fhe_type, result| verify_input(&package, index, fhe_type, result);
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        event(0, 5, number),
        false,
    )
    .await?;
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        event(1, 0, flag),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &flag).await?;
    insert_event(
        &harness.listener_db,
        &mut tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: number,
            rhs: scalar_u128_handle(2),
            scalarByte: scalar_flag(true),
            result: sum,
        }),
        true,
    )
    .await?;
    allow_handle(&harness.listener_db, &mut tx, &sum).await?;
    tx.commit().await?;

    // No ct64 yet: the tfhe-worker asks the pad-worker for the 72 bits of the package.
    let pad_length = wait_for_pad_request(&harness.pool, label).await;
    assert_eq!(pad_length, 72);
    store_ct64(&harness.pool, label, pad_length).await;

    wait_until_computed(&harness.app).await?;
    let values = decrypt_handles(&harness.pool, &[sum, flag]).await?;
    let values: Vec<_> = values.iter().map(|v| v.value.as_str()).collect();
    assert_eq!(values, vec!["42", "true"]);
    Ok(())
}

#[tokio::test]
async fn the_listener_writes_an_unmask_for_a_pad_input_only(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let (package, _) = package(&[3], &7u16.to_le_bytes());
    let tx_id = next_handle();
    let pad_result = next_handle_with_type(3);
    let regular_result = next_handle_with_type(3);
    // A regular input proof starts with its handle count, which is never 0.
    let mut regular = verify_input(&package, 0, 3, regular_result);
    if let TfheContractEvents::VerifyInput(event) = &mut regular {
        event.inputProof = Bytes::from(vec![1u8, 0]);
    }

    let mut tx = harness
        .listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let pad = verify_input(&package, 0, 3, pad_result);
    insert_event(&harness.listener_db, &mut tx, tx_id, pad, true).await?;
    insert_event(&harness.listener_db, &mut tx, tx_id, regular, true).await?;
    tx.commit().await?;

    // One Unmask, with the package and the input index. The ct64 comes from the tfhe-worker.
    let rows: Vec<(Vec<u8>, i16, Vec<Vec<u8>>)> = sqlx::query_as(
        "SELECT output_handle, fhe_operation, dependencies FROM computations
          WHERE transaction_id = $1",
    )
    .bind(tx_id.to_vec())
    .fetch_all(&harness.pool)
    .await?;
    assert_eq!(
        rows,
        vec![(pad_result.to_vec(), 33, vec![package, vec![0]])]
    );
    Ok(())
}

#[tokio::test]
async fn unmasks_with_a_pad_that_is_already_stored() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let (package, label) = package(&[3], &0xbeefu16.to_le_bytes());
    // The KMS asked for a longer pad before: it starts with the same 16 bits.
    store_ct64(&harness.pool, label, 64).await;
    let tx_id = next_handle();
    let value = next_handle_with_type(3);

    let mut tx = harness
        .listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let event = verify_input(&package, 0, 3, value);
    insert_event(&harness.listener_db, &mut tx, tx_id, event, true).await?;
    allow_handle(&harness.listener_db, &mut tx, &value).await?;
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let values = decrypt_handles(&harness.pool, &[value]).await?;
    assert_eq!(values[0].value, "48879");
    // The tfhe-worker did not ask the pad-worker for the pad.
    let requests: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pads WHERE label = $1")
        .bind(label.as_slice())
        .fetch_one(&harness.pool)
        .await?;
    assert_eq!(requests, 0);
    Ok(())
}

/// Adds an event for the only input (euint16) of `package`, and returns the error of its Unmask.
async fn unmask_error(harness: &EventHarness, package: &[u8]) -> String {
    let tx_id = next_handle();
    let value = next_handle_with_type(3);
    let mut tx = harness
        .listener_db
        .new_transaction()
        .await
        .unwrap()
        .expect("new_transaction() returns Some on a live stack");
    let event = verify_input(package, 0, 3, value);
    insert_event(&harness.listener_db, &mut tx, tx_id, event, true)
        .await
        .unwrap();
    allow_handle(&harness.listener_db, &mut tx, &value)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let (is_error, message) = wait_for_error(&harness.pool, value.as_slice(), tx_id.as_slice())
        .await
        .unwrap();
    assert!(is_error);
    message.unwrap_or_default()
}

#[tokio::test]
async fn fails_the_unmask_when_the_pad_worker_failed_the_pad(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let (package, label) = package(&[3], &7u16.to_le_bytes());
    sqlx::query(
        "INSERT INTO pads (prf_instance, label, pad_length, triggered_by, is_error)
         VALUES ($1, $2, 16, $3, TRUE)",
    )
    .bind(i32::from(PRF_ID))
    .bind(label.as_slice())
    .bind([7u8; 20].as_slice())
    .execute(&harness.pool)
    .await?;

    assert!(unmask_error(&harness, &package)
        .await
        .contains("the pad-worker failed it"));
    Ok(())
}

#[tokio::test]
async fn fails_the_unmask_when_the_ct64_does_not_match_its_digest(
) -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let (package, label) = package(&[3], &7u16.to_le_bytes());
    store_ct64(&harness.pool, label, 16).await;
    sqlx::query("UPDATE pad_ct64 SET ciphertext_digest = $1 WHERE label = $2")
        .bind([0u8; 32].as_slice())
        .bind(label.as_slice())
        .execute(&harness.pool)
        .await?;

    let message = unmask_error(&harness, &package).await;
    assert!(message.contains("does not match its digest"));
    Ok(())
}

#[test]
fn the_test_package_passes_the_listener_checks() {
    let (package, _) = package(&[5, 0], &[0; 9]);
    let mut result = [0u8; 32];
    result[21] = 0xff;
    result[22..30].copy_from_slice(&TEST_CHAIN_ID.to_be_bytes());
    result[30] = 0;
    let external = external_handle(&package, 1, 0);
    fhevm_engine_common::pad::validate_pad_input(&external.0, &result, 0, &package, TEST_CHAIN_ID)
        .unwrap();
    assert!(fhevm_engine_common::pad::validate_pad_input(
        &external.0,
        &result,
        0,
        &package,
        TEST_CHAIN_ID + 1
    )
    .is_err());
}
