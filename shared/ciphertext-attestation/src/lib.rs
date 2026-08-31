//! Off-chain ciphertext attestation primitives shared by Coprocessor and KMS Connector.
//!
//! Both producer and consumer must encode, sign, and verify attestations byte-identically.
//! This crate is the single source of truth for that encoding.
//!
//! See RFC-023 (Off-chain ciphertext commits handling).

use alloy_primitives::{Address, B256, U256};
use serde::{Deserialize, Serialize};

pub mod consensus;
pub mod sign;
pub mod tracker;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "client")]
pub use client::{
    BoundedClient, ConsensusCheckError, CoprocessorEntry, CoprocessorRegistry,
    CoprocessorRegistrySnapshot, RegistryError, ResolvedConsensus,
    fetch_attestations_and_check_consensus,
};

/// Domain separator for the canonical signed payload.
pub const DOMAIN_TAG: [u8; 8] = *b"FHEVMCTA";

/// Ceiling on the serialized size of an SNS ciphertext in bytes.
pub const MAX_SNS_CIPHERTEXT_SERIALIZED_SIZE: u64 = 66 * 1024 * 1024;

/// S3 user-defined metadata key that carries the JSON-serialized
/// [`CiphertextAttestation`] on every ciphertext object.
///
/// AWS SDK metadata APIs expect this key without the `x-amz-meta-` HTTP
/// header prefix.
pub const S3_METADATA_ATTESTATION_KEY: &str = "ct-attestation";

/// S3 user-defined metadata header that carries the JSON-serialized
/// [`CiphertextAttestation`] on every ciphertext object.
pub const S3_METADATA_ATTESTATION_HEADER: &str = "x-amz-meta-ct-attestation";

/// Key prefix of ct128 (SNS) ciphertext objects in Coprocessor buckets.
pub const S3_CT128_KEY_PREFIX: &str = "ct128";

/// Key prefix of compressed ct64 ciphertext objects in Coprocessor buckets.
pub const S3_CT64_KEY_PREFIX: &str = "ct64";

/// S3 key of a ct128 (SNS) ciphertext object: `ct128/{hex(handle)}/{context_id}`.
pub fn s3_ct128_key(handle: &[u8], coprocessor_context_id: U256) -> String {
    format!(
        "{S3_CT128_KEY_PREFIX}/{}/{coprocessor_context_id}",
        hex::encode(handle)
    )
}

/// S3 key of a compressed ct64 ciphertext object: `ct64/{hex(handle)}/{context_id}`.
pub fn s3_ct64_key(handle: &[u8], coprocessor_context_id: U256) -> String {
    format!(
        "{S3_CT64_KEY_PREFIX}/{}/{coprocessor_context_id}",
        hex::encode(handle)
    )
}

/// Coprocessor context id for RFC-023 V1 deployments. Consensus-critical global state: it is
/// signed into the attestation payload and baked into the object URL, so it is a wire-format
/// constant, not per-service config. Retires when `GatewayConfig` gains Coprocessor contexts.
pub const COPROCESSOR_CONTEXT_ID_V1: U256 = U256::ONE;

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

/// Ciphertext storage format.
///
/// The JSON representation is the snake_case variant name; unknown strings are rejected at
/// deserialization. The canonical bytes encode the discriminant as `uint8`.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum CiphertextFormat {
    UncompressedOnCpu = 10,
    CompressedOnCpu = 11,
    UncompressedOnGpu = 20,
    CompressedOnGpu = 21,
}

/// The full set of fields bound by an attestation signature. [`Self::sign`] produces a
/// [`CiphertextAttestation`] for the wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiphertextAttestationPayload {
    pub version: Version,
    pub handle: B256,
    pub key_id: U256,
    pub coprocessor_context_id: U256,
    pub ciphertext_digest: B256,
    pub sns_ciphertext_digest: B256,
    pub format: CiphertextFormat,
}

impl CiphertextAttestationPayload {
    pub fn new(
        version: Version,
        handle: B256,
        key_id: U256,
        coprocessor_context_id: U256,
        ciphertext_digest: B256,
        sns_ciphertext_digest: B256,
        format: CiphertextFormat,
    ) -> Self {
        Self {
            version,
            handle,
            key_id,
            coprocessor_context_id,
            ciphertext_digest,
            sns_ciphertext_digest,
            format,
        }
    }
}

/// Signed wire form persisted as the S3 metadata header [`S3_METADATA_ATTESTATION_HEADER`].
///
/// `handle` and `coprocessor_context_id` are intentionally absent — the verifier reconstructs
/// them from the S3 lookup path and supplies them to [`Self::verify`]. Both are bound by the
/// signature, so any path/attestation mismatch surfaces as a signature failure.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiphertextAttestation {
    pub version: Version,
    pub key_id: U256,
    pub ciphertext_digest: B256,
    pub sns_ciphertext_digest: B256,
    pub format: CiphertextFormat,
    pub signer: Address,
    #[serde(with = "hex_bytes")]
    pub signature: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum AttestationError {
    #[error("unsupported attestation version: {0}")]
    UnsupportedVersion(u8),
    #[error("malformed signature: {0}")]
    MalformedSignature(String),
    #[error("signature recovery failed: {0}")]
    Recovery(String),
    #[error("signer mismatch: recovered {recovered}, expected {expected}")]
    SignerMismatch {
        recovered: Address,
        expected: Address,
    },
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("signer error: {0}")]
    Signer(#[from] alloy_signer::Error),
    /// The signature is genuine, but for a different bucket's key.
    #[error("signer {embedded} is not the registered signer {registered} for this bucket")]
    SignerNotRegisteredForBucket {
        embedded: Address,
        registered: Address,
    },
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};

    fn sample_attestation() -> CiphertextAttestation {
        CiphertextAttestation {
            version: Version::V1,
            key_id: U256::from(69),
            ciphertext_digest: b256!(
                "1111111111111111111111111111111111111111111111111111111111111111"
            ),
            sns_ciphertext_digest: b256!(
                "2222222222222222222222222222222222222222222222222222222222222222"
            ),
            format: CiphertextFormat::UncompressedOnCpu,
            signer: address!("00112233445566778899aabbccddeeff00112233"),
            signature: vec![0xab; 65],
        }
    }

    #[test]
    fn json_round_trip() {
        let att = sample_attestation();
        let json = serde_json::to_string(&att).unwrap();
        let back: CiphertextAttestation = serde_json::from_str(&json).unwrap();
        assert_eq!(att, back);
    }

    #[test]
    fn json_rejects_unknown_version() {
        let mut value = serde_json::to_value(sample_attestation()).unwrap();
        value["version"] = serde_json::Value::from(99u8);
        let err = serde_json::from_value::<CiphertextAttestation>(value).unwrap_err();
        assert!(err.to_string().contains("unsupported attestation version"));
    }

    #[test]
    fn json_rejects_unknown_format() {
        let mut value = serde_json::to_value(sample_attestation()).unwrap();
        value["format"] = serde_json::Value::from("uncompressed_on_quantum");
        let err = serde_json::from_value::<CiphertextAttestation>(value).unwrap_err();
        assert!(err.to_string().contains("unknown variant"));
    }

    #[test]
    fn json_serializes_format_as_snake_case() {
        let att = sample_attestation();
        let json = serde_json::to_value(&att).unwrap();
        assert_eq!(
            json["format"],
            serde_json::Value::from("uncompressed_on_cpu")
        );
    }
}
