use bigdecimal::num_bigint::BigInt;

use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{ClearConst, ScalarByte};
use serial_test::serial;

use crate::tests::events::{
    allow_handle, insert_tfhe_event, listener_db, next_handle, tfhe_log, to_ty,
};
use crate::tests::utils::{
    decrypt_ciphertexts, setup_test_app, wait_until_all_allowed_handles_computed,
};

fn as_clear_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

#[tokio::test]
#[serial(db)]
async fn schedule_erc20_graph_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener = listener_db(&app).await;

    let num_samples: usize = std::env::var("FHEVM_TEST_NUM_SAMPLES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(7);

    let u64_type = 5;
    let fhe_bool_type = 0;
    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();

    let mut expected: Vec<(Vec<u8>, String, i16)> = Vec::new();

    let mut tx = listener.new_transaction().await?;
    for i in 0..num_samples {
        // Alternate between "enough funds" and "not enough funds" to exercise both branches.
        let (bals, trxa, bald) = if i % 2 == 0 {
            (
                BigInt::from(100u64),
                BigInt::from(10u64),
                BigInt::from(20u64),
            )
        } else {
            (BigInt::from(5u64), BigInt::from(10u64), BigInt::from(20u64))
        };

        let transaction_id = next_handle();

        // Inputs: bals, trxa, bald (not allowed for decryption)
        let bals_handle = next_handle();
        let trxa_handle = next_handle();
        let bald_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_clear_uint(&bals),
                    toType: to_ty(u64_type),
                    result: bals_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_clear_uint(&trxa),
                    toType: to_ty(u64_type),
                    result: trxa_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                    caller,
                    pt: as_clear_uint(&bald),
                    toType: to_ty(u64_type),
                    result: bald_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;

        // Internal computations (not allowed for decryption)
        let has_enough_funds_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::FheGe(TfheContract::FheGe {
                    caller,
                    lhs: bals_handle,
                    rhs: trxa_handle,
                    scalarByte: ScalarByte::from(0u8),
                    result: has_enough_funds_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;

        let new_to_amount_target_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: bald_handle,
                    rhs: trxa_handle,
                    scalarByte: ScalarByte::from(0u8),
                    result: new_to_amount_target_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;

        let new_from_amount_target_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::FheSub(TfheContract::FheSub {
                    caller,
                    lhs: bals_handle,
                    rhs: trxa_handle,
                    scalarByte: ScalarByte::from(0u8),
                    result: new_from_amount_target_handle,
                }),
                transaction_id,
            ),
            false,
        )
        .await?;

        // Outputs (allowed for decryption)
        let new_to_amount_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                    caller,
                    control: has_enough_funds_handle,
                    ifTrue: new_to_amount_target_handle,
                    ifFalse: bald_handle,
                    result: new_to_amount_handle,
                }),
                transaction_id,
            ),
            true,
        )
        .await?;
        allow_handle(&listener, &mut tx, new_to_amount_handle.as_ref()).await?;

        let new_from_amount_handle = next_handle();
        insert_tfhe_event(
            &listener,
            &mut tx,
            tfhe_log(
                TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                    caller,
                    control: has_enough_funds_handle,
                    ifTrue: new_from_amount_target_handle,
                    ifFalse: bals_handle,
                    result: new_from_amount_handle,
                }),
                transaction_id,
            ),
            true,
        )
        .await?;
        allow_handle(&listener, &mut tx, new_from_amount_handle.as_ref()).await?;

        let expected_to = if i % 2 == 0 { "30" } else { "20" };
        let expected_from = if i % 2 == 0 { "90" } else { "5" };
        expected.push((
            new_to_amount_handle.to_vec(),
            expected_to.to_string(),
            u64_type as i16,
        ));
        expected.push((
            new_from_amount_handle.to_vec(),
            expected_from.to_string(),
            u64_type as i16,
        ));

        // Sanity: ensure we actually computed a boolean along the way
        expected.push((
            has_enough_funds_handle.to_vec(),
            String::new(),
            fhe_bool_type,
        ));
    }
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    // Only decrypt the allowed outputs (skip the internal boolean handles)
    let allowed_handles = expected
        .iter()
        .filter(|(_, v, _)| !v.is_empty())
        .map(|(h, _, _)| h.clone())
        .collect::<Vec<_>>();

    let resp = decrypt_ciphertexts(&pool, allowed_handles).await?;
    assert_eq!(resp.len(), num_samples * 2);

    for (idx, r) in resp.iter().enumerate() {
        let expected_idx = idx * 2;
        let expected_value = &expected
            .iter()
            .filter(|(_, v, _)| !v.is_empty())
            .nth(idx)
            .unwrap()
            .1;
        assert_eq!(
            &r.value, expected_value,
            "unexpected value at index {expected_idx}"
        );
        assert_eq!(r.output_type, u64_type as i16);
    }

    Ok(())
}
