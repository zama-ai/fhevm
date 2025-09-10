use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use rand::Rng;

pub fn rand_u256() -> U256 {
    U256::from_le_bytes(rand::rng().random::<[u8; 32]>())
}

pub fn rand_address() -> Address {
    Address::from(rand::rng().random::<[u8; 20]>())
}

pub fn rand_public_key() -> Vec<u8> {
    rand::rng().random::<[u8; 32]>().to_vec()
}

pub fn rand_signature() -> Vec<u8> {
    rand::rng().random_iter().take(65).collect()
}

pub fn rand_digest() -> FixedBytes<32> {
    rand::rng().random::<[u8; 32]>().into()
}

pub fn rand_sns_ct() -> SnsCiphertextMaterial {
    SnsCiphertextMaterial {
        keyId: rand_u256(),
        ctHandle: rand::rng().random::<[u8; 32]>().into(),
        snsCiphertextDigest: rand::rng().random::<[u8; 32]>().into(),
        coprocessorTxSenderAddresses: vec![rand_address()],
    }
}
