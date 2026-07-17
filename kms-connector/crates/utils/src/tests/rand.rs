use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_host_bindings::protocol_config::{
    IProtocolConfig::KmsThresholds,
    ProtocolConfig::{KmsNodeParams, PcrValues},
};
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

pub fn rand_handle() -> FixedBytes<32> {
    rand::rng().random::<[u8; 32]>().into()
}

pub fn rand_kms_thresholds() -> KmsThresholds {
    KmsThresholds {
        publicDecryption: rand_u256(),
        userDecryption: rand_u256(),
        kmsGen: rand_u256(),
        mpc: rand_u256(),
    }
}

pub fn rand_kms_node_params() -> KmsNodeParams {
    KmsNodeParams {
        partyId: rand::rng().random::<i32>(),
        signerAddress: rand_address(),
        txSenderAddress: rand_address(),
        ..Default::default()
    }
}

pub fn rand_pcr_values() -> PcrValues {
    PcrValues {
        pcr0: rand_digest().into(),
        pcr1: rand_digest().into(),
        pcr2: rand_digest().into(),
    }
}
