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

mod dependence_chain;
mod errors;
mod events;
mod health_check;
mod inputs_from_compact_list;
mod operators;
mod operators_from_events;
mod random_events;
mod scheduling_bench;
mod utils;

fn as_clear_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

#[tokio::test]
#[serial(db)]
async fn test_smoke_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let listener = listener_db(&app).await;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let transaction_id = next_handle();
    let h1 = next_handle();
    let h2 = next_handle();
    let h3 = next_handle();

    let ct_type = 4; // u32
    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();

    let mut tx = listener.new_transaction().await?;
    insert_tfhe_event(
        &listener,
        &mut tx,
        tfhe_log(
            TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
                caller,
                pt: as_clear_uint(&BigInt::from(123u32)),
                toType: to_ty(ct_type),
                result: h1,
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
                pt: as_clear_uint(&BigInt::from(124u32)),
                toType: to_ty(ct_type),
                result: h2,
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
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: h1,
                rhs: h2,
                scalarByte: ScalarByte::from(0u8),
                result: h3,
            }),
            transaction_id,
        ),
        true,
    )
    .await?;
    allow_handle(&listener, &mut tx, h3.as_ref()).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, 1, vec![h3.to_vec()]).await?;
    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].output_type, ct_type as i16);
    assert_eq!(resp[0].value, "247");

    Ok(())
}

#[tokio::test]
#[ignore]
// custom test to run against local instance for decrypting custom ciphertexts
async fn test_custom_function() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect("postgresql://postgres:postgres@127.0.0.1:5432/coprocessor")
        .await?;

    let res = utils::decrypt_ciphertexts(
        &pool,
        1,
        vec![
            hex::decode("de2c33227b24ca797f7ad88495648446c70612c17f416d27513c77f2d0810200")
                .unwrap(),
            hex::decode("51d1d882d1e5ce54f15523558edd2746766c14cd5177faeb659418c57cec0200")
                .unwrap(),
            hex::decode("e3935354c48514fdfb0cbd965ad506d8865a2c88efffffca94dc9e0cecec0300")
                .unwrap(),
            hex::decode("3eed1ad1d1aa030b3bb3d3587ece4661a56945affcdee6bbdc02e28779380200")
                .unwrap(),
            hex::decode("55fe0c4283fbad83dc6fab91c3f85c098ada7a70ca8089e3076043efc9c60200")
                .unwrap(),
            hex::decode("3b42e61e197b88c083b4a2ab4b0ec542775e2282bebcc574e45d09f9779a0200")
                .unwrap(),
            hex::decode("9718b490a41e20fecaa90a7ab75e74de0c4105213ac3e5d8b5368ab813160200")
                .unwrap(),
            hex::decode("164c6d678ddf95f12bfa6b0fee7fd8b12e6221bd0c587640ae61dfc624f20200")
                .unwrap(),
            hex::decode("52a01af58c3d2b8ed1d04cd846706c1d214b72e079bd0930827628cb69180200")
                .unwrap(),
            hex::decode("d8d493764d46b62187b6a42917d58e297922d9ebab0dee306324e8c78a130200")
                .unwrap(),
        ],
    )
    .await?;
    println!("decrypted ciphertexts: {:?}", res);
    Ok(())
}
