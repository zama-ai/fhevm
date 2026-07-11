use std::{path::PathBuf, sync::OnceLock};

use fhevm_engine_common::{
    tfhe_ops::perform_fhe_operation,
    types::{SupportedFheCiphertexts, SupportedFheOperations},
    utils::safe_deserialize_key,
};
use tfhe::{
    prelude::{FheDecrypt, FheTryEncrypt},
    xof_key_set::CompressedXofKeySet,
    ClientKey, FheBool, ServerKey,
};

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

fn install_server_key() -> &'static TestKeys {
    let keys = test_keys();
    tfhe::set_server_key(keys.server.clone());
    keys
}

fn run(
    operation: SupportedFheOperations,
    inputs: &[SupportedFheCiphertexts],
    output_type: i16,
) -> SupportedFheCiphertexts {
    perform_fhe_operation(operation as i16, inputs, 0, output_type)
        .unwrap_or_else(|error| panic!("{operation:?} failed: {error}"))
}

#[test]
fn bool_xor_uses_real_tfhe() {
    let keys = install_server_key();
    let lhs = FheBool::try_encrypt(true, &keys.client).expect("encrypt lhs");
    let rhs = FheBool::try_encrypt(false, &keys.client).expect("encrypt rhs");

    let result = run(
        SupportedFheOperations::FheBitXor,
        &[
            SupportedFheCiphertexts::FheBool(lhs),
            SupportedFheCiphertexts::FheBool(rhs),
        ],
        0,
    );

    assert_eq!(result.type_num(), 0);
    let SupportedFheCiphertexts::FheBool(result) = result else {
        panic!("expected FheBool result");
    };
    assert!(result.decrypt(&keys.client));
}
