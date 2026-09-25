use crate::types::solana_request::{
    SolanaEntryClaims, SolanaRequestBlob, SolanaUserDecryptFields, SolanaUserDecryptionRequestV1,
};
use alloy::primitives::{Address, FixedBytes, U256};
use fhevm_gateway_bindings::decryption::{
    Decryption::SolanaUserDecryptionRequest, IDecryption::RequestValiditySeconds,
};
use fhevm_host_bindings::protocol_config::{
    IProtocolConfig::KmsThresholds,
    ProtocolConfig::{KmsNodeParams, PcrValues},
};
use rand::Rng;
use zama_solana_request::host_chain::solana_host_chain_id;

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

/// A well-formed Solana user decryption event. Its signature is a placeholder, so it only passes
/// the checks that precede signature verification.
pub fn solana_user_decryption_event(
    decryption_id: U256,
    handle: FixedBytes<32>,
) -> SolanaUserDecryptionRequest {
    let (fields, blob) = solana_user_decryption_parts(handle);
    solana_user_decryption_event_for(decryption_id, &fields, &blob)
}

/// The two carriers of a well-formed Solana request naming `handle`, with a placeholder
/// signature.
pub fn solana_user_decryption_parts(
    handle: FixedBytes<32>,
) -> (SolanaUserDecryptFields, SolanaRequestBlob) {
    let fields = SolanaUserDecryptFields {
        handles: vec![handle.0],
        transport_key: vec![2; zama_solana_permit::TRANSPORT_KEY_LEN],
        start_timestamp: sqlx::types::chrono::Utc::now().timestamp() as u64 - 60,
        duration_seconds: 3600,
        extra_data: [vec![2], vec![1; 64]].concat(),
    };
    let blob = SolanaRequestBlob {
        user_address: [1; 32],
        allowed_scopes: vec![],
        verifying_program_id: [7; 32],
        signature: [0; 64],
        entries: vec![SolanaEntryClaims {
            owner_address: [1; 32],
            encrypted_store: [3; 32],
        }],
    };
    (fields, blob)
}

/// The Gateway event carrying `fields` as its typed fields and `blob` as its opaque request.
pub fn solana_user_decryption_event_for(
    decryption_id: U256,
    fields: &SolanaUserDecryptFields,
    blob: &SolanaRequestBlob,
) -> SolanaUserDecryptionRequest {
    SolanaUserDecryptionRequest {
        decryptionId: decryption_id,
        ctHandles: fields
            .handles
            .iter()
            .map(|handle| FixedBytes::from(*handle))
            .collect(),
        requestValidity: RequestValiditySeconds {
            startTimestamp: U256::from(fields.start_timestamp),
            durationSeconds: U256::from(fields.duration_seconds),
        },
        publicKey: fields.transport_key.clone().into(),
        extraData: fields.extra_data.clone().into(),
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
