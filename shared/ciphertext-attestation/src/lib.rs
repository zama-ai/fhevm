//! Off-chain attestation primitives shared by Coprocessor and KMS Connector.
//!
//! Both producer and consumer must encode, sign, and verify attestations byte-identically.
//! This crate is the single source of truth for that encoding.

use alloy_primitives::{Address, B256};
use sha3::{Digest, Keccak256};

pub mod ciphertext;
pub mod consensus;
pub mod prf;

pub use ciphertext::{
    COPROCESSOR_CONTEXT_ID_V1, CiphertextAttestation, CiphertextAttestationPayload,
    CiphertextFormat, MAX_SNS_CIPHERTEXT_SERIALIZED_SIZE, S3_CT64_KEY_PREFIX, S3_CT128_KEY_PREFIX,
    S3_METADATA_ATTESTATION_HEADER, S3_METADATA_ATTESTATION_KEY,
    consensus::{CiphertextRef, ConsensusMaterial},
    s3_ct64_key, s3_ct128_key,
};
pub use consensus::{
    Attestation, ConsensusCheckError, ConsensusOutcome, ConsensusRound, CoprocessorEntry,
    ResolvedConsensus,
};
pub use prf::{
    PrfOutputAttestation, PrfOutputAttestationPayload,
    consensus::{PrfMaterial, PrfOutputRef},
};

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "client")]
pub use client::{
    BoundedClient, CoprocessorRegistry, CoprocessorRegistrySnapshot, CriticalFailurePolicy,
    FetchAttestationError, FetchCiphertextError, RegistryError,
    fetch_attestations_and_check_consensus,
};

#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    #[error("unsupported attestation version: {0}")]
    UnsupportedVersion(u8),
    #[error("malformed signature: {0}")]
    MalformedSignature(String),
    #[error("signature recovery failed: {0}")]
    Recovery(String),
    /// The signature does not recover to the signer the attestation claims.
    #[error("signer mismatch: recovered {recovered}, attestation claims {claimed}")]
    SignerMismatch {
        recovered: Address,
        claimed: Address,
    },
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("signer error: {0}")]
    Signer(#[from] alloy_signer::Error),
    /// The attestation claims a signer other than the one the caller expects.
    #[error("unexpected signer: attestation claims {claimed}, expected {expected}")]
    UnexpectedSigner { claimed: Address, expected: Address },
}

pub(crate) mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(bytes: &Vec<u8>, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(de)?;
        let stripped = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(stripped).map_err(D::Error::custom)
    }
}

/// Keccak-256 of arbitrary bytes.
pub fn keccak_b256(bytes: &[u8]) -> B256 {
    B256::from_slice(&Keccak256::digest(bytes))
}
