use crate::types::db::SnsCiphertextMaterialDbItem;
use alloy::primitives::{Address, FixedBytes, U256};
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

pub fn rand_sns_ct() -> SnsCiphertextMaterialDbItem {
    SnsCiphertextMaterialDbItem {
        key_id: rand::rng().random::<[u8; 32]>(),
        ct_handle: rand::rng().random::<[u8; 32]>(),
        sns_ciphertext_digest: rand::rng().random::<[u8; 32]>(),
        storage_urls: vec!["http://localhost:9000/ct/128".to_string()],
    }
}
