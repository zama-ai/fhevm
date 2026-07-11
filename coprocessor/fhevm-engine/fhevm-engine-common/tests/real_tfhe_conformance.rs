use std::{
    path::PathBuf,
    sync::{Mutex, MutexGuard, OnceLock},
};

use fhevm_engine_common::{
    tfhe_ops::perform_fhe_operation,
    types::{SupportedFheCiphertexts, SupportedFheOperations},
    utils::safe_deserialize_key,
};
use tfhe::{
    prelude::{FheDecrypt, FheTryEncrypt},
    xof_key_set::CompressedXofKeySet,
    ClientKey, FheBool, FheUint64, FheUint8, ServerKey,
};

const BOOL_TYPE: i16 = 0;
const U8_TYPE: i16 = 2;
const U64_TYPE: i16 = 5;

struct TestKeys {
    client: ClientKey,
    server: ServerKey,
}

fn test_keys() -> &'static TestKeys {
    static KEYS: OnceLock<TestKeys> = OnceLock::new();
    KEYS.get_or_init(|| {
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fhevm-keys");
        let keyset: CompressedXofKeySet = safe_deserialize_key(
            &std::fs::read(fixtures.join("xof-keyset")).expect("read xof-keyset fixture"),
        )
        .expect("deserialize xof-keyset fixture");
        let (_, server) = keyset
            .decompress()
            .expect("decompress xof-keyset fixture")
            .into_raw_parts();
        let client = safe_deserialize_key(
            &std::fs::read(fixtures.join("xof-cks")).expect("read xof-cks fixture"),
        )
        .expect("deserialize xof-cks fixture");
        TestKeys { client, server }
    })
}

fn install_server_key() -> (MutexGuard<'static, ()>, &'static TestKeys) {
    static EXECUTION: Mutex<()> = Mutex::new(());
    let execution = EXECUTION
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let keys = test_keys();
    tfhe::set_server_key(keys.server.clone());
    (execution, keys)
}

fn run(
    operation: SupportedFheOperations,
    inputs: &[SupportedFheCiphertexts],
    output_type: i16,
) -> SupportedFheCiphertexts {
    perform_fhe_operation(operation as i16, inputs, 0, output_type)
        .unwrap_or_else(|error| panic!("{operation:?} failed: {error}"))
}

fn encrypted_bool(value: bool, keys: &TestKeys) -> SupportedFheCiphertexts {
    SupportedFheCiphertexts::FheBool(
        FheBool::try_encrypt(value, &keys.client).expect("encrypt FheBool"),
    )
}

fn encrypted_u8(value: u8, keys: &TestKeys) -> SupportedFheCiphertexts {
    SupportedFheCiphertexts::FheUint8(
        FheUint8::try_encrypt(value, &keys.client).expect("encrypt FheUint8"),
    )
}

fn encrypted_u64(value: u64, keys: &TestKeys) -> SupportedFheCiphertexts {
    SupportedFheCiphertexts::FheUint64(
        FheUint64::try_encrypt(value, &keys.client).expect("encrypt FheUint64"),
    )
}

fn scalar(value: impl Into<Vec<u8>>) -> SupportedFheCiphertexts {
    SupportedFheCiphertexts::Scalar(value.into())
}

fn decrypt_bool(result: SupportedFheCiphertexts, keys: &TestKeys) -> bool {
    assert_eq!(result.type_num(), BOOL_TYPE);
    let SupportedFheCiphertexts::FheBool(result) = result else {
        panic!("expected FheBool result");
    };
    result.decrypt(&keys.client)
}

fn decrypt_u8(result: SupportedFheCiphertexts, keys: &TestKeys) -> u8 {
    assert_eq!(result.type_num(), U8_TYPE);
    let SupportedFheCiphertexts::FheUint8(result) = result else {
        panic!("expected FheUint8 result");
    };
    result.decrypt(&keys.client)
}

fn decrypt_u64(result: SupportedFheCiphertexts, keys: &TestKeys) -> u64 {
    assert_eq!(result.type_num(), U64_TYPE);
    let SupportedFheCiphertexts::FheUint64(result) = result else {
        panic!("expected FheUint64 result");
    };
    result.decrypt(&keys.client)
}

#[test]
fn bool_xor_uses_real_tfhe() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheBitXor,
        &[encrypted_bool(true, keys), encrypted_bool(false, keys)],
        BOOL_TYPE,
    );
    assert!(decrypt_bool(result, keys));
}

#[test]
fn bool_not_uses_real_tfhe() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheNot,
        &[encrypted_bool(true, keys)],
        BOOL_TYPE,
    );
    assert!(!decrypt_bool(result, keys));
}

#[test]
fn uint8_encrypted_add_wraps() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheAdd,
        &[encrypted_u8(250, keys), encrypted_u8(10, keys)],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 4);
}

#[test]
fn uint8_scalar_subtraction_wraps() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheSub,
        &[encrypted_u8(3, keys), scalar(vec![5])],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 254);
}

#[test]
fn uint8_comparison_returns_bool() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheLt,
        &[encrypted_u8(7, keys), encrypted_u8(9, keys)],
        BOOL_TYPE,
    );
    assert!(decrypt_bool(result, keys));
}

#[test]
fn uint8_scalar_shift_discards_high_bit() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheShl,
        &[encrypted_u8(0b1000_0001, keys), scalar(vec![1])],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 0b0000_0010);
}

#[test]
fn uint8_scalar_rotate_preserves_high_bit() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheRotl,
        &[encrypted_u8(0b1000_0001, keys), scalar(vec![1])],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 0b0000_0011);
}

#[test]
fn uint64_encrypted_multiply_uses_real_tfhe() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheMul,
        &[encrypted_u64(7, keys), encrypted_u64(9, keys)],
        U64_TYPE,
    );
    assert_eq!(decrypt_u64(result, keys), 63);
}

#[test]
fn uint64_scalar_division_is_supported() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheDiv,
        &[
            encrypted_u64(29, keys),
            scalar(5_u64.to_be_bytes().to_vec()),
        ],
        U64_TYPE,
    );
    assert_eq!(decrypt_u64(result, keys), 5);
}

#[test]
fn uint64_scalar_remainder_is_supported() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheRem,
        &[
            encrypted_u64(29, keys),
            scalar(5_u64.to_be_bytes().to_vec()),
        ],
        U64_TYPE,
    );
    assert_eq!(decrypt_u64(result, keys), 4);
}

#[test]
fn uint64_unary_negation_wraps() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheNeg,
        &[encrypted_u64(1, keys)],
        U64_TYPE,
    );
    assert_eq!(decrypt_u64(result, keys), u64::MAX);
}

#[test]
fn uint8_cast_to_uint64_uses_big_endian_type_id() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheCast,
        &[
            encrypted_u8(200, keys),
            scalar((U64_TYPE as u16).to_be_bytes().to_vec()),
        ],
        U64_TYPE,
    );
    assert_eq!(decrypt_u64(result, keys), 200);
}

#[test]
fn if_then_else_selects_uint8_branch() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheIfThenElse,
        &[
            encrypted_bool(false, keys),
            encrypted_u8(10, keys),
            encrypted_u8(20, keys),
        ],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 20);
}

#[test]
fn non_empty_uint8_sum_wraps() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheSum,
        &[
            encrypted_u8(250, keys),
            encrypted_u8(10, keys),
            encrypted_u8(1, keys),
        ],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 5);
}

#[test]
fn non_empty_uint8_is_in_returns_bool() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheIsIn,
        &[
            encrypted_u8(7, keys),
            encrypted_u8(3, keys),
            encrypted_u8(7, keys),
            encrypted_u8(11, keys),
        ],
        BOOL_TYPE,
    );
    assert!(decrypt_bool(result, keys));
}

#[test]
fn uint8_mul_div_uses_fused_real_tfhe_operation() {
    let (_execution, keys) = install_server_key();
    let result = run(
        SupportedFheOperations::FheMulDiv,
        &[
            encrypted_u8(10, keys),
            encrypted_u8(9, keys),
            scalar(vec![6]),
        ],
        U8_TYPE,
    );
    assert_eq!(decrypt_u8(result, keys), 15);
}
