use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};

fn main() {
    let keys = FhevmKeys::new();
    let ser_keys: SerializedFhevmKeys = keys.into();
    ser_keys.save_to_disk();
}
