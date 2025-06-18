use alloy::primitives::{Address, U256};
use rand::Rng;

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
    rand::thread_rng().r#gen::<[u8; 32]>().to_vec()
}
