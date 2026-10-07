//! Off-chain attestation primitives shared by Coprocessor and KMS Connector.
//!
//! Both producer and consumer must encode, sign, and verify attestations byte-identically.
//! This crate is the single source of truth for that encoding.
//!
//! See RFC-023 (Off-chain ciphertext commits handling).

use alloy_primitives::{Address, B256};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

pub mod ciphertext;
pub mod consensus;
pub mod sign;

pub use ciphertext::{
    COPROCESSOR_CONTEXT_ID_V1, CiphertextAttestation, CiphertextAttestationPayload,
    CiphertextFormat, DOMAIN_TAG, MAX_SNS_CIPHERTEXT_SERIALIZED_SIZE, S3_CT64_KEY_PREFIX,
    S3_CT128_KEY_PREFIX, S3_METADATA_ATTESTATION_HEADER, S3_METADATA_ATTESTATION_KEY,
    consensus::{CiphertextRef, ConsensusMaterial},
    s3_ct64_key, s3_ct128_key,
};
pub use consensus::{
    Attestation, ConsensusCheckError, ConsensusOutcome, ConsensusRound, CoprocessorEntry,
    ResolvedConsensus,
};

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "client")]
pub use client::{
    BoundedClient, CoprocessorRegistry, CoprocessorRegistrySnapshot, CriticalFailurePolicy,
    FetchAttestationError, FetchCiphertextError, RegistryError,
    fetch_attestations_and_check_consensus,
};

/// Versioned encoding of the attestation. The version byte is part of the signed
/// payload, so a stripped or downgraded `version` field flips signature recovery
/// and is caught at verification time.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
#[repr(u8)]
pub enum Version {
    V1 = 1,
}

impl TryFrom<u8> for Version {
    type Error = AttestationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Version::V1),
            other => Err(AttestationError::UnsupportedVersion(other)),
        }
    }
}

impl From<Version> for u8 {
    fn from(v: Version) -> u8 {
        v as u8
    }
}

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
