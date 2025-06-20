use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_gateway_rust_bindings::decryption::Decryption::SnsCiphertextMaterial;
use rand::{Rng, distributions::Standard};

pub fn rand_u256() -> U256 {
    U256::from_le_bytes(rand::thread_rng().r#gen::<[u8; 32]>())
}

pub fn rand_address() -> Address {
    Address::from(rand::thread_rng().r#gen::<[u8; 20]>())
}

pub fn rand_public_key() -> Vec<u8> {
    rand::thread_rng().r#gen::<[u8; 32]>().to_vec()
}

pub fn rand_signature() -> Vec<u8> {
    rand::thread_rng().sample_iter(Standard).take(65).collect()
}

pub fn rand_digest() -> FixedBytes<32> {
    rand::thread_rng().r#gen::<[u8; 32]>().into()
}

pub fn rand_sns_ct() -> SnsCiphertextMaterial {
    SnsCiphertextMaterial {
        keyId: rand_u256(),
        ctHandle: rand::thread_rng().r#gen::<[u8; 32]>().into(),
        snsCiphertextDigest: rand::thread_rng().r#gen::<[u8; 32]>().into(),
        coprocessorTxSenderAddresses: vec![rand_address()],
    }
}
