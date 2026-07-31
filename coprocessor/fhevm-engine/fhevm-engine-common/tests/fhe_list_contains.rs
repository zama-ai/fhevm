//! Execution-level coverage for the `FheIsIn` ("list contains") dispatch
//! arms, focused on the two widest supported types (FheUint128 /
//! FheUint160) which had no execution coverage anywhere in the repo (only
//! `check_fhe_operand_types` type-checking is exercised in
//! `tfhe_ops::fhe_is_in_tests`).
//!
//! All operands are *trivial* ciphertexts (`trivial_encrypt_be_bytes`), the
//! same technique `tfhe-worker`'s `carry_residue` tests use to exercise real
//! FHE dispatch without a full encrypt/decrypt round trip. The server key is
//! loaded from the test-harness's precomputed fixture keys (`ImportMode`
//! that skips SnS) rather than generated fresh, to keep this from paying for
//! CRS/bootstrapping-key generation on every run.
//!
//! Slow in debug: real FHE comparisons over FheUint128/FheUint160, even on
//! trivial operands, are not free. Run with `--release` if debug is too
//! slow; see the crate-level test notes in CLAUDE.local.md-style docs for
//! this repo (sns/tfhe-worker FHE-heavy tests need `--release` locally).

use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::tfhe_ops::{perform_fhe_operation, trivial_encrypt_be_bytes};
use fhevm_engine_common::types::{FhevmError, SupportedFheCiphertexts, SupportedFheOperations};
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::{setup_test_db, ImportMode};

const TYPE_UINT128: i16 = 6;
const TYPE_UINT160: i16 = 7;
const TYPE_UINT64: i16 = 5;
const FHE_IS_IN: i16 = SupportedFheOperations::FheIsIn as i16;

/// Loads a real (fixture-backed) server key and sets it for the current
/// thread. Must run inside `spawn_blocking`: tfhe's server key is
/// thread-local.
///
/// Importing the fixture key set into a fresh DB (`ImportMode::WithKeysNoSns`
/// under `COPROCESSOR_TEST_LOCALHOST_RESET`) takes on the order of a couple
/// of minutes, so it is done at most once per test binary run and shared
/// (the key material itself is immutable and `ServerKey` is `Clone`).
async fn server_key() -> tfhe::ServerKey {
    static SERVER_KEY: tokio::sync::OnceCell<tfhe::ServerKey> = tokio::sync::OnceCell::const_new();
    SERVER_KEY
        .get_or_init(|| async {
            let db = setup_test_db(ImportMode::WithKeysNoSns)
                .await
                .expect("setup db with keys");
            let pool = PgPoolOptions::new()
                .max_connections(2)
                .connect(db.db_url())
                .await
                .expect("connect pool");
            DbKeyCache::new(1)
                .expect("db key cache")
                .fetch_latest_from_pool(&pool)
                .await
                .expect("fetch latest db key")
                .sks
        })
        .await
        .clone()
}

fn be_bytes(width: usize, value: u64) -> Vec<u8> {
    let mut out = vec![0u8; width];
    let value_bytes = value.to_be_bytes();
    let copy_len = value_bytes.len().min(width);
    out[width - copy_len..].copy_from_slice(&value_bytes[value_bytes.len() - copy_len..]);
    out
}

#[tokio::test]
#[serial(db)]
async fn uint128_contains_present_value() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, 42)).unwrap();
        let set = [7u64, 42, 99]
            .iter()
            .map(|v| trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, *v)).unwrap())
            .collect::<Vec<_>>();
        let operands: Vec<SupportedFheCiphertexts> = std::iter::once(value).chain(set).collect();

        let result = perform_fhe_operation(FHE_IS_IN, &operands, 0, 0).unwrap();
        let SupportedFheCiphertexts::FheBool(b) = result else {
            panic!("FheIsIn must return FheBool");
        };
        assert!(b.try_decrypt_trivial().unwrap(), "42 is present in the set");
    })
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uint128_contains_absent_value() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, 1000)).unwrap();
        let set = [7u64, 42, 99]
            .iter()
            .map(|v| trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, *v)).unwrap())
            .collect::<Vec<_>>();
        let operands: Vec<SupportedFheCiphertexts> = std::iter::once(value).chain(set).collect();

        let result = perform_fhe_operation(FHE_IS_IN, &operands, 0, 0).unwrap();
        let SupportedFheCiphertexts::FheBool(b) = result else {
            panic!("FheIsIn must return FheBool");
        };
        assert!(
            !b.try_decrypt_trivial().unwrap(),
            "1000 is absent from the set"
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uint128_contains_rejects_mixed_type_set_element() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, 42)).unwrap();
        // Wrong-type set element: FheUint64 mixed into a FheUint128 set.
        let wrong_type = trivial_encrypt_be_bytes(TYPE_UINT64, &be_bytes(8, 42)).unwrap();
        let operands = vec![value, wrong_type];

        let is_type_err = matches!(
            perform_fhe_operation(FHE_IS_IN, &operands, 0, 0),
            Err(FhevmError::UnsupportedFheTypes { .. })
        );
        assert!(is_type_err, "mixed-type set element must be a type error");
    })
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uint160_contains_present_value() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT160, &be_bytes(20, 42)).unwrap();
        let set = [7u64, 42, 99]
            .iter()
            .map(|v| trivial_encrypt_be_bytes(TYPE_UINT160, &be_bytes(20, *v)).unwrap())
            .collect::<Vec<_>>();
        let operands: Vec<SupportedFheCiphertexts> = std::iter::once(value).chain(set).collect();

        let result = perform_fhe_operation(FHE_IS_IN, &operands, 0, 0).unwrap();
        let SupportedFheCiphertexts::FheBool(b) = result else {
            panic!("FheIsIn must return FheBool");
        };
        assert!(b.try_decrypt_trivial().unwrap(), "42 is present in the set");
    })
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uint160_contains_absent_value() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT160, &be_bytes(20, 1000)).unwrap();
        let set = [7u64, 42, 99]
            .iter()
            .map(|v| trivial_encrypt_be_bytes(TYPE_UINT160, &be_bytes(20, *v)).unwrap())
            .collect::<Vec<_>>();
        let operands: Vec<SupportedFheCiphertexts> = std::iter::once(value).chain(set).collect();

        let result = perform_fhe_operation(FHE_IS_IN, &operands, 0, 0).unwrap();
        let SupportedFheCiphertexts::FheBool(b) = result else {
            panic!("FheIsIn must return FheBool");
        };
        assert!(
            !b.try_decrypt_trivial().unwrap(),
            "1000 is absent from the set"
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn uint160_contains_rejects_mixed_type_set_element() {
    let sks = server_key().await;
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(sks);

        let value = trivial_encrypt_be_bytes(TYPE_UINT160, &be_bytes(20, 42)).unwrap();
        let wrong_type = trivial_encrypt_be_bytes(TYPE_UINT128, &be_bytes(16, 42)).unwrap();
        let operands = vec![value, wrong_type];

        let is_type_err = matches!(
            perform_fhe_operation(FHE_IS_IN, &operands, 0, 0),
            Err(FhevmError::UnsupportedFheTypes { .. })
        );
        assert!(is_type_err, "mixed-type set element must be a type error");
    })
    .await
    .unwrap();
}
