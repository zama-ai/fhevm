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

/// Well-formed Solana request data; the signature is a placeholder for storage/routing tests.
pub fn solana_user_decryption_request(
    decryption_id: U256,
    handle: FixedBytes<32>,
) -> crate::types::solana_request::SolanaUserDecryptionRequestV1 {
    use crate::types::solana_request::{SolanaUserDecryptRequest, SolanaUserDecryptionRequestV1};
    use zama_solana_permit::{PermitWireFields, TRANSPORT_KEY_LEN};
    use zama_solana_request::{SolanaHandleEntryWire, SolanaUserDecryptRequestWire};
    let request = SolanaUserDecryptRequest::decode(&SolanaUserDecryptRequestWire {
        permit: PermitWireFields {
            user_pubkey: vec![1; 32],
            transport_key: vec![2; TRANSPORT_KEY_LEN],
            allowed_scopes: vec![],
            start_timestamp: sqlx::types::chrono::Utc::now().timestamp() as u64 - 60,
            duration_seconds: 3600,
            verifying_program_id: vec![7; 32],
            chain_id: crate::types::handle::extract_chain_id_from_handle(&handle).unwrap(),
            extra_data: [vec![2], vec![1; 64]].concat(),
        },
        signature: vec![0; 64],
        handles: vec![SolanaHandleEntryWire {
            handle: handle.to_vec(),
            allowed_key: vec![1; 32],
            encrypted_store: vec![3; 32],
        }],
    })
    .unwrap();
    SolanaUserDecryptionRequestV1 {
        decryption_id,
        request,
    }
}
