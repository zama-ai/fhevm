use crate::types::{DEFAULT_EPOCH_ID, TESTING_KMS_CONTEXT, extra_data::extra_data_v2_payload};
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
