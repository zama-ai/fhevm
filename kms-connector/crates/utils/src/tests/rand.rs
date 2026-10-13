use crate::types::solana_request::{SolanaUserDecryptFields, SolanaUserDecryptionRequestV1};
use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_host_bindings::protocol_config::ProtocolConfig::{
    KmsNodeParams, KmsThresholds, PcrValues,
};
use rand::Rng;
use zama_solana_acl::host_chain::{EVM_CHAIN_TYPE, solana_host_chain_id};
use zama_solana_request::HandleEntry;

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

/// A random handle of an EVM host chain: its chain id's type byte (byte 22) is zero.
pub fn rand_handle() -> FixedBytes<32> {
    let mut handle = rand::rng().random::<[u8; 32]>();
    handle[22] = EVM_CHAIN_TYPE;
    handle.into()
}

/// A random handle of the Solana localnet host chain.
pub fn rand_solana_handle() -> FixedBytes<32> {
    let mut handle = rand::rng().random::<[u8; 32]>();
    handle[22..30].copy_from_slice(&solana_host_chain_id(12345).to_be_bytes());
    handle.into()
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

/// A well-formed Solana user decryption request. Its signature is a placeholder, so it only
/// passes the checks that precede signature verification.
pub fn solana_user_decryption_request(
    decryption_id: U256,
    handle: FixedBytes<32>,
) -> SolanaUserDecryptionRequestV1 {
    let fields = SolanaUserDecryptFields {
        user_address: [1; 32],
        transport_key: vec![2; zama_solana_permit::TRANSPORT_KEY_LEN],
        allowed_scopes: vec![],
        start_timestamp: sqlx::types::chrono::Utc::now().timestamp() as u64 - 60,
        duration_seconds: 3600,
        verifying_program_id: [7; 32],
        extra_data: [vec![2], vec![1; 64]].concat(),
        signature: [0; 64],
        entries: vec![HandleEntry {
            handle: handle.0,
            owner_address: [1; 32],
            encrypted_store: [3; 32],
        }],
    };
    SolanaUserDecryptionRequestV1::new(decryption_id, fields).unwrap()
}
