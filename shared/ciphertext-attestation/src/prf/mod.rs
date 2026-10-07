//! RFC-038 PRF output attestation: wire types, signing and consensus material.

use crate::{AttestationError, hex_bytes};
use alloy_primitives::{Address, B256};
use serde::{Deserialize, Serialize};

pub mod consensus;
pub mod sign;

/// Domain separator for the canonical signed payload.
pub const PRF_DOMAIN_TAG: [u8; 8] = *b"FHEVMPRF";

/// HTTP response header that carries the JSON-serialized [`PrfOutputAttestation`].
pub const PRF_ATTESTATION_HEADER: &str = "x-prf-attestation";

/// Versioned encoding of the PRF output attestation.
///
/// The version byte is part of the signed payload, so a stripped or downgraded `version`
/// field flips signature recovery and is caught at verification time.
///
/// Independent of the ciphertext attestation versioning.
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

/// The full set of fields bound by a PRF output attestation signature. [`Self::sign`] produces a
/// [`PrfOutputAttestation`] for the wire.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrfOutputAttestationPayload {
    pub version: Version,
    /// The RFC-030 PRF instance identifier.
    pub prf_id: u16,
    pub label: B256,
    /// Keccak-256 of the raw PRF output, as served in the response body.
    pub digest: B256,
}

impl PrfOutputAttestationPayload {
    pub fn new(version: Version, prf_id: u16, label: B256, digest: B256) -> Self {
        Self {
            version,
            prf_id,
            label,
            digest,
        }
    }
}

/// Signed wire form served as the [`PRF_ATTESTATION_HEADER`] response header.
///
/// `prf_id` and `label` are intentionally absent: the verifier reconstructs them from the
/// request path and supplies them to [`Self::verify`]. Both are bound by the signature, so any
/// path/attestation mismatch surfaces as a signature failure.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrfOutputAttestation {
    pub version: Version,
    pub digest: B256,
    pub signer: Address,
    #[serde(with = "hex_bytes")]
    pub signature: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256};

    fn sample_attestation() -> PrfOutputAttestation {
        PrfOutputAttestation {
            version: Version::V1,
            digest: b256!("1111111111111111111111111111111111111111111111111111111111111111"),
            signer: address!("00112233445566778899aabbccddeeff00112233"),
            signature: vec![0xab; 65],
        }
    }

    #[test]
    fn json_round_trip() {
        let att = sample_attestation();
        let json = serde_json::to_string(&att).unwrap();
        let back: PrfOutputAttestation = serde_json::from_str(&json).unwrap();
        assert_eq!(att, back);
    }

    #[test]
    fn json_wire_shape() {
        let json = serde_json::to_value(sample_attestation()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "version": 1,
                "digest": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "signer": "0x00112233445566778899aabbccddeeff00112233",
                "signature": format!("0x{}", "ab".repeat(65)),
            })
        );
    }

    #[test]
    fn json_rejects_unknown_version() {
        let mut value = serde_json::to_value(sample_attestation()).unwrap();
        value["version"] = serde_json::Value::from(99u8);
        let err = serde_json::from_value::<PrfOutputAttestation>(value).unwrap_err();
        assert!(err.to_string().contains("unsupported attestation version"));
    }
}
