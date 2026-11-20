//! The `fhevm-keys` key set: the tests make no keys.

use fhevm_engine_common::utils::safe_deserialize_key;
use pad_worker::evaluator::TfheEvaluator;
use std::{fs, sync::LazyLock};
use tfhe::{xof_key_set::CompressedXofKeySet, ClientKey, ServerKey};

pub struct TestKeys {
    #[allow(dead_code)] // Each test binary uses its own part.
    pub client: ClientKey,
    pub server: ServerKey,
}

/// The PRF instance of the tests.
#[allow(dead_code)] // Each test binary uses its own part.
pub const INSTANCE: u16 = 1;

static KEYS: LazyLock<TestKeys> = LazyLock::new(load);

pub fn keys() -> &'static TestKeys {
    &KEYS
}

/// True on a PR (`PAD_WORKER_TEST_MATRIX=local`): the test runs only in the merge queue.
#[allow(dead_code)] // Each test binary uses its own part.
pub fn merge_queue_only(test: &str) -> bool {
    let local = std::env::var("PAD_WORKER_TEST_MATRIX")
        .is_ok_and(|mode| mode.eq_ignore_ascii_case("local"));
    if local {
        println!("{test}: skipped, runs in the merge queue");
    }
    local
}

/// A tfhe-rs evaluator with the test keys.
pub fn evaluator() -> TfheEvaluator {
    TfheEvaluator::new(keys().server.clone())
}

fn load() -> TestKeys {
    let keyset_bytes = fs::read("../fhevm-keys/xof-keyset").expect("fhevm-keys/xof-keyset");
    let keyset: CompressedXofKeySet = safe_deserialize_key(&keyset_bytes).unwrap();
    let (_public_key, server) = keyset.decompress().into_raw_parts();
    let client: ClientKey =
        safe_deserialize_key(&fs::read("../fhevm-keys/xof-cks").expect("fhevm-keys/xof-cks"))
            .unwrap();
    TestKeys { client, server }
}
