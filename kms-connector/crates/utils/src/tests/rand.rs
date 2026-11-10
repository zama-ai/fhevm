use crate::types::{
    DEFAULT_EPOCH_ID, TESTING_KMS_CONTEXT,
    extra_data::extra_data_v2_payload,
    solana_request::{SolanaUserDecryptFields, SolanaUserDecryptionRequestV1},
};
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use fhevm_host_bindings::protocol_config::ProtocolConfig::{
    KmsNodeParams, KmsThresholds, PcrValues,
};
use kms_connector_api::{
    AttestationType, PAD_WITNESS_VERSION, PadDecryptionPayload, PadDecryptionRequest, PadEntry,
    PadWitness, RequestValidity,
};
use rand::Rng;
use sqlx::types::chrono::Utc;
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
        start_timestamp: Utc::now().timestamp() as u64 - 60,
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

/// A pad decryption request passing the endpoint validation, with two pads expiring in an hour.
///
/// Its signature is a placeholder.
pub fn rand_pad_request(chain_id: u64) -> PadDecryptionRequest {
    let now = Utc::now().timestamp() as u64;
    let prf_id = 0;
    // The user submitted the pads, so that it is authorized to decrypt them.
    let user = rand_address();
    let pads = [64, 2048]
        .into_iter()
        .map(|pad_length| {
            let witness = PadWitness {
                submitter: user,
                consumer: rand_address(),
                expirationTimestamp: now + 3600,
                decryptors: vec![rand_address()],
                nonce: FixedBytes::from(rand::rng().random::<[u8; 16]>()),
            };
            PadEntry {
                label: witness.label(PAD_WITNESS_VERSION, prf_id, chain_id),
                witness,
                padLength: pad_length,
            }
        })
        .collect();
    PadDecryptionRequest {
        attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
        payload: PadDecryptionPayload {
            witnessVersion: PAD_WITNESS_VERSION,
            prfId: prf_id,
            chainId: chain_id,
            pads,
            userAddress: user,
            publicKey: Bytes::from(vec![0x20; 32]),
            allowedContracts: vec![],
            requestValidity: RequestValidity {
                startTimestamp: now - 60,
                durationSeconds: 300,
            },
            extraData: Bytes::from(extra_data_v2_payload(TESTING_KMS_CONTEXT, DEFAULT_EPOCH_ID)),
        },
        signature: Bytes::from(vec![0x66; 65]),
    }
}
