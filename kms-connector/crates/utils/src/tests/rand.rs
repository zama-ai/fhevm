use crate::types::solana_request::{
    SolanaEntryClaims, SolanaGatewayFields, SolanaRequestBlob, SolanaUserDecryptionRequestV1,
};
use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_gateway_bindings::decryption::{
    Decryption::UserDecryptionRequest_4, IDecryption::RequestValiditySeconds,
};
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

/// A well-formed Solana user decryption event. Its signature is a placeholder, so it only passes
/// the checks that precede signature verification.
pub fn solana_user_decryption_event(
    decryption_id: U256,
    handle: FixedBytes<32>,
) -> UserDecryptionRequest_4 {
    let (gateway, blob) = solana_user_decryption_parts(handle);
    solana_user_decryption_event_for(decryption_id, &gateway, &blob)
}

/// The two carriers of a well-formed Solana request naming `handle`, with a placeholder
/// signature.
pub fn solana_user_decryption_parts(
    handle: FixedBytes<32>,
) -> (SolanaGatewayFields, SolanaRequestBlob) {
    let gateway = SolanaGatewayFields {
        handles: vec![handle.to_vec()],
        transport_key: vec![2; zama_solana_permit::TRANSPORT_KEY_LEN],
        start_timestamp: sqlx::types::chrono::Utc::now().timestamp() as u64 - 60,
        duration_seconds: 3600,
        extra_data: [vec![2], vec![1; 64]].concat(),
    };
    let blob = SolanaRequestBlob {
        user_address: vec![1; 32],
        allowed_scopes: vec![],
        verifying_program_id: vec![7; 32],
        signature: vec![0; 64],
        entries: vec![SolanaEntryClaims {
            owner_address: vec![1; 32],
            encrypted_store: vec![3; 32],
        }],
    };
    (gateway, blob)
}

/// The Gateway event carrying `gateway` as its typed fields and `blob` as its opaque request.
pub fn solana_user_decryption_event_for(
    decryption_id: U256,
    gateway: &SolanaGatewayFields,
    blob: &SolanaRequestBlob,
) -> UserDecryptionRequest_4 {
    UserDecryptionRequest_4 {
        decryptionId: decryption_id,
        ctHandles: gateway
            .handles
            .iter()
            .map(|handle| FixedBytes::from_slice(handle))
            .collect(),
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(gateway.start_timestamp),
            durationSeconds: U256::from(gateway.duration_seconds),
        },
        publicKey: gateway.transport_key.clone().into(),
        extraData: gateway.extra_data.clone().into(),
        solanaRequest: zama_solana_request::encode_solana_request(blob)
            .unwrap()
            .into(),
    }
}

pub fn solana_user_decryption_request(
    decryption_id: U256,
    handle: FixedBytes<32>,
) -> SolanaUserDecryptionRequestV1 {
    solana_user_decryption_event(decryption_id, handle)
        .try_into()
        .unwrap()
}
