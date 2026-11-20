//! The tfhe-rs evaluator with the `fhevm-keys` key set.

use alloy::primitives::B256;
use fhevm_engine_common::{db_keys::DbKeyCache, utils::safe_deserialize};
use pad_worker::evaluator::{PadEvaluator, TfheEvaluator};
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::{setup_test_db, ImportMode};
use tfhe::{
    prelude::FheDecrypt, transciphering::OneTimePadPlainState, CompressedOneTimePadFheSecretMask,
    CompressedSquashedNoiseOneTimePadFheSecretMask,
};

mod common;

/// Same input, same bytes. The ct128 and the decompressed ct64 decrypt to one pad, rounded up to
/// whole bytes.
fn round_trip(pad_length: u16) {
    let keys = common::keys();
    let evaluator = common::evaluator();
    let label = B256::repeat_byte(0xab);
    let outputs = evaluator.evaluate(0, label, pad_length).unwrap();
    assert_eq!(outputs, evaluator.evaluate(0, label, pad_length).unwrap());

    let ct64: CompressedOneTimePadFheSecretMask = safe_deserialize(&outputs.ct64).unwrap();
    let ct128: CompressedSquashedNoiseOneTimePadFheSecretMask =
        safe_deserialize(&outputs.ct128).unwrap();
    // The decompression key is in the server key.
    tfhe::set_server_key(keys.server.clone());
    let pad = ct64.decompress().unwrap().decrypt(&keys.client);
    assert!(pad == ct128.decrypt(&keys.client));
    let bits = OneTimePadPlainState::new(pad).remaining_bits();
    assert_eq!(bits, u64::from(pad_length).div_ceil(8) * 8);
    assert_ne!(
        outputs,
        evaluator
            .evaluate(0, B256::repeat_byte(0xcd), pad_length)
            .unwrap()
    );
}

#[test]
fn round_trip_of_a_1_bit_pad() {
    round_trip(1);
}

#[test]
fn round_trip_of_a_64_bit_pad() {
    round_trip(64);
}

#[test]
fn round_trip_of_a_2048_bit_pad() {
    if common::merge_queue_only("round_trip_of_a_2048_bit_pad") {
        return;
    }
    round_trip(2048);
}

/// The full key computes a pad, the tfhe-worker key does not, and a legacy key is refused.
#[tokio::test]
async fn the_key_from_the_database_keeps_the_noise_squashing_keys() {
    let instance = setup_test_db(ImportMode::WithAllKeys)
        .await
        .expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .unwrap();
    let key_id: Vec<u8> = sqlx::query_scalar("SELECT key_id FROM keys")
        .fetch_one(&pool)
        .await
        .unwrap();
    let label = B256::repeat_byte(1);

    let key = DbKeyCache::new_keeping_noise_squashing(1)
        .unwrap()
        .fetch(&key_id, &pool)
        .await
        .unwrap();
    TfheEvaluator::new(key.sks).evaluate(0, label, 8).unwrap();

    let stripped = DbKeyCache::new(1)
        .unwrap()
        .fetch(&key_id, &pool)
        .await
        .unwrap();
    assert!(TfheEvaluator::new(stripped.sks)
        .evaluate(0, label, 8)
        .is_err());

    sqlx::query("UPDATE keys SET compressed_xof_keyset = NULL")
        .execute(&pool)
        .await
        .unwrap();
    let legacy = DbKeyCache::new_keeping_noise_squashing(1)
        .unwrap()
        .fetch(&key_id, &pool)
        .await;
    assert!(legacy.is_err());
}
