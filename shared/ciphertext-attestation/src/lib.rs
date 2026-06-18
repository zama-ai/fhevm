//! Off-chain ciphertext attestation primitives shared by Coprocessor and KMS Connector.
//!
//! Both producer and consumer must encode, sign, and verify attestations byte-identically.
//! This crate is the single source of truth for that encoding.
//!
//! Producer flow:
//!
//! ```ignore
//! let attestation: CiphertextAttestation =
//!     CiphertextAttestationPayload::new(version, handle, key_id, ctx, ct, sns, format)
//!         .sign(&signer)
//!         .await?;
//! s3_push(url, serde_json::to_string(&attestation)?);
//! ```
//!
//! Verifier flow:
//!
//! ```ignore
//! let attestation: CiphertextAttestation = serde_json::from_str(&s3_metadata)?;
//! attestation.verify(handle, coprocessor_context_id)?;
//! ```
//!
//! See RFC-023 (Off-chain ciphertext commits handling).

use alloy_primitives::{Address, B256, U256};
use serde::{Deserialize, Serialize};

pub mod sign;

/// Domain separator for the canonical signed payload. Scopes the keccak hash to
/// "FHEVM CT Attestation" and prevents collisions with any other hash computed
/// over similar-looking inputs.
pub const DOMAIN_TAG: [u8; 8] = *b"FHEVMCTA";

/// S3 user-defined metadata header that carries the JSON-serialized
/// [`CiphertextAttestation`] on every ciphertext object.
pub const S3_METADATA_ATTESTATION_HEADER: &str = "x-amz-meta-ct-attestation";

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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum CiphertextFormat {
    UncompressedOnCpu = 10,
    CompressedOnCpu = 11,
    UncompressedOnGpu = 20,
    CompressedOnGpu = 21,
}

/// The full set of fields bound by an attestation signature. Construct, optionally
/// inspect via [`Self::canonical_bytes`] / [`Self::canonical_digest`], then call
/// [`Self::sign`] to produce a [`CiphertextAttestation`] for the wire.
///
/// Not serializable: `handle` and `coprocessor_context_id` are intentionally
/// stripped from the wire form (the verifier reconstructs them from the S3
/// lookup path).
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

/// Signed wire form persisted as the S3 metadata header
/// [`S3_METADATA_ATTESTATION_HEADER`].
///
/// `handle` and `coprocessor_context_id` are intentionally absent — the verifier
/// reconstructs them from the S3 lookup path and supplies them to
/// [`Self::verify`]. Both are bound by the signature, so any path/attestation
/// mismatch surfaces as a signature failure.
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
