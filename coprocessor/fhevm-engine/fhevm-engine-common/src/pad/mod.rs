//! Pad-based inputs: the input package, and the unmask of its parts with the pad ct64.
//!
//! The package parser is the real one. The one-time pad of tfhe-rs is not released yet, so
//! [`mock`] replaces it, with the expected API and byte layout.

pub mod mock;

use crate::types::{FhevmError, SupportedFheCiphertexts};
use alloy::primitives::{keccak256, B256};

/// The most pad bits of one package.
pub const PAD_MAX_PAD_BITS: usize = 2048;

const PAD_PACKAGE_TYPE: u8 = 0x00;
const PAD_WITNESS_VERSION: u8 = 0x00;
const PAD_LABEL_DOMAIN_SEPARATOR: &[u8; 8] = b"FHE_padl";
const PAD_PACKAGE_DOMAIN_SEPARATOR: &[u8; 8] = b"FHE_padp";
// Byte offsets of the fixed part of the package.
const OFFSET_WITNESS_VERSION: usize = 1;
const OFFSET_PRF: usize = 2;
const OFFSET_CHAIN_ID: usize = 4;
const OFFSET_DECRYPTOR_COUNT: usize = 60;
const OFFSET_DECRYPTORS: usize = 61;
const NONCE_BYTES: usize = 16;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PadPackageError {
    #[error("malformed package")]
    Malformed,
    #[error("unsupported package type {0}")]
    UnsupportedPackageType(u8),
    #[error("unsupported witness version {0}")]
    UnsupportedWitnessVersion(u8),
    #[error("no parts")]
    NoParts,
    #[error("part {index} has the unknown type {part_type}")]
    UnknownPartType { index: usize, part_type: u8 },
    #[error("{0} pad bits, more than {PAD_MAX_PAD_BITS}")]
    TooLarge(usize),
    #[error("ciphertext of {actual} bytes, the parts need {expected}")]
    CiphertextLengthMismatch { actual: usize, expected: usize },
    #[error("label {label} is not the hash of the witness {expected}")]
    LabelMismatch { label: B256, expected: B256 },
}

#[derive(Debug, thiserror::Error)]
pub enum PadError {
    #[error("bad ct64: {0}")]
    BadCt64(&'static str),
    #[error("pad length {0}, not 1 to {PAD_MAX_PAD_BITS}")]
    BadPadLength(usize),
    #[error("the part has an unknown type, or not the bytes of its type")]
    BadPart,
    #[error("the pad has {pad_bits} bits, the package needs {needed}")]
    PadTooShort { pad_bits: usize, needed: usize },
    #[error("encryption: {0:?}")]
    Encryption(FhevmError),
}

/// One input of a package: its FHE type and its bytes in the ciphertext.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PadPart {
    pub fhe_type: i16,
    /// Byte offset in the ciphertext.
    pub offset: usize,
    /// `⌈width/8⌉` bytes, least significant byte first.
    pub len: usize,
}

/// An input package of version 0, with the fields that Unmask uses. Big-endian integers:
/// `packageType 1 | witnessVersion 1 | prfId 2 | chainId 8 | submitter 20 | consumer 20 |
/// expirationTimestamp 8 | decryptorCount 1 | decryptors 20×count | nonce 16 | label 32 |
/// partCount 1 | parts 1×partCount | ciphertext`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PadPackage {
    pub prf_id: u16,
    pub chain_id: u64,
    pub label: B256,
    pub parts: Vec<PadPart>,
    /// `c = m ⊕ r`: the parts one after the other.
    pub ciphertext: Vec<u8>,
}

impl PadPackage {
    /// Parses the package and checks `label = hash(witness)`. `PaddedInputter` checked the other
    /// witness fields. The caller checks `chain_id` against the chain of the event.
    pub fn parse(bytes: &[u8]) -> Result<Self, PadPackageError> {
        let slice = |offset: usize, len: usize| {
            bytes
                .get(offset..offset + len)
                .ok_or(PadPackageError::Malformed)
        };
        let byte = |offset: usize| slice(offset, 1).map(|b| b[0]);

        let package_type = byte(0)?;
        if package_type != PAD_PACKAGE_TYPE {
            return Err(PadPackageError::UnsupportedPackageType(package_type));
        }
        let witness_version = byte(OFFSET_WITNESS_VERSION)?;
        if witness_version != PAD_WITNESS_VERSION {
            return Err(PadPackageError::UnsupportedWitnessVersion(witness_version));
        }
        let prf_id = u16::from_be_bytes(slice(OFFSET_PRF, 2)?.try_into().expect("2 bytes"));
        let chain_id = u64::from_be_bytes(slice(OFFSET_CHAIN_ID, 8)?.try_into().expect("8 bytes"));
        let decryptor_count = usize::from(byte(OFFSET_DECRYPTOR_COUNT)?);
        let witness_end = OFFSET_DECRYPTORS + 20 * decryptor_count + NONCE_BYTES;
        let label = B256::from_slice(slice(witness_end, 32)?);
        let part_count = usize::from(byte(witness_end + 32)?);
        let parts_offset = witness_end + 33;
        let part_types = slice(parts_offset, part_count)?;
        let ciphertext = &bytes[parts_offset + part_count..];

        if part_count == 0 {
            return Err(PadPackageError::NoParts);
        }
        let mut parts = Vec::with_capacity(part_count);
        let mut offset = 0;
        for (index, &part_type) in part_types.iter().enumerate() {
            let width = part_bit_width(i16::from(part_type))
                .ok_or(PadPackageError::UnknownPartType { index, part_type })?;
            let len = width.div_ceil(8);
            parts.push(PadPart {
                fhe_type: i16::from(part_type),
                offset,
                len,
            });
            offset += len;
        }
        if offset * 8 > PAD_MAX_PAD_BITS {
            return Err(PadPackageError::TooLarge(offset * 8));
        }
        if ciphertext.len() != offset {
            return Err(PadPackageError::CiphertextLengthMismatch {
                actual: ciphertext.len(),
                expected: offset,
            });
        }
        let expected = keccak256([PAD_LABEL_DOMAIN_SEPARATOR, &bytes[1..witness_end]].concat());
        if expected != label {
            return Err(PadPackageError::LabelMismatch { label, expected });
        }

        Ok(Self {
            prf_id,
            chain_id,
            label,
            parts,
            ciphertext: ciphertext.to_vec(),
        })
    }

    /// `N`: the pad bits that the package uses, so the `pad_length` of its pad.
    pub fn pad_bits(&self) -> usize {
        self.ciphertext.len() * 8
    }
}

/// The width in bits of an FHE type that a package can carry, or `None`.
pub fn part_bit_width(fhe_type: i16) -> Option<usize> {
    match fhe_type {
        0 => Some(1),
        2 => Some(8),
        3 => Some(16),
        4 => Some(32),
        5 => Some(64),
        6 => Some(128),
        7 => Some(160),
        8 => Some(256),
        _ => None,
    }
}

/// Unmasks the parts of a package with the pad ct64 of its label.
pub trait PadUnmasker: Send + Sync {
    /// Returns `m = c ⊕ r` for input `index`, as an FHE value of its type. It uses only the pad
    /// bits of that input, so all the inputs of a package use each pad bit one time. `ct64` is
    /// the stored pad of `(package.prf_id, package.label)`, with at least `package.pad_bits()`
    /// bits: a longer pad starts with the same bits. Call it with the server key set on this
    /// thread.
    fn unmask(
        &self,
        ct64: &[u8],
        package: &PadPackage,
        index: usize,
    ) -> Result<SupportedFheCiphertexts, PadError>;
}

/// Whether the input proof of a `VerifyInput` event is a pad-based input package. The executor
/// uses the same check.
pub fn is_pad_package(proof: &[u8]) -> bool {
    proof.first() == Some(&PAD_PACKAGE_TYPE)
}

/// The checks of a pad-based `VerifyInput` event, as `PaddedInputter` makes them. The external
/// handle is `packageDigest[0..29] | inputIndex | type | version`.
pub fn validate_pad_input(
    external_handle: &[u8; 32],
    result: &[u8; 32],
    input_type: u8,
    package: &[u8],
    chain_id: u64,
) -> Result<(), &'static str> {
    let digest = keccak256([PAD_PACKAGE_DOMAIN_SEPARATOR, package].concat());
    if digest[..29] != external_handle[..29] {
        return Err("pad input handle is not from its package");
    }
    if external_handle[31] != 0 {
        return Err("pad input handle has an unknown version");
    }
    let package = PadPackage::parse(package).map_err(|_| "bad pad package")?;
    if package.chain_id != chain_id {
        return Err("pad package has the wrong chain id");
    }
    let part = package
        .parts
        .get(usize::from(external_handle[29]))
        .ok_or("pad package has no input at the handle index")?;
    let fhe_type = i16::from(input_type);
    if part.fhe_type != fhe_type
        || i16::from(external_handle[30]) != fhe_type
        || i16::from(result[30]) != fhe_type
    {
        return Err("pad input has the wrong type");
    }
    Ok(())
}

/// The `triggered_by` of a `pads` row that the tfhe-worker adds for a missing ct64. No KMS
/// connector has the zero address.
pub const UNMASK_TRIGGERED_BY: [u8; 20] = [0; 20];

/// The unmasker of this build: the mock, until a tfhe-rs release has the one-time pad.
fn unmasker() -> impl PadUnmasker {
    mock::MockPadUnmasker
}

/// The `Unmask` operation: input `index` (1 byte) of `package`, unmasked with `ct64`, as a value
/// of `output_type`. An empty `ct64` means that there is no valid pad.
pub fn unmask_input(
    package: &[u8],
    index: &[u8],
    ct64: &[u8],
    output_type: i16,
) -> Result<SupportedFheCiphertexts, FhevmError> {
    let (package, index) = package_input(package, index)?;
    let fhe_type = package.parts[index].fhe_type;
    if fhe_type != output_type {
        return Err(FhevmError::PadUnmask(format!(
            "input {index} has type {fhe_type}, the result has type {output_type}"
        )));
    }
    if ct64.is_empty() {
        return Err(FhevmError::PadUnmask(
            "no valid pad: the pad-worker failed it, or its ct64 does not match its digest"
                .to_string(),
        ));
    }
    unmasker()
        .unmask(ct64, &package, index)
        .map_err(|e| FhevmError::PadUnmask(e.to_string()))
}

/// Parses the package, and checks that it has input `index`.
pub fn package_input(package: &[u8], index: &[u8]) -> Result<(PadPackage, usize), FhevmError> {
    let package = PadPackage::parse(package).map_err(|e| FhevmError::PadUnmask(e.to_string()))?;
    let &[index] = index else {
        return Err(FhevmError::PadUnmask(format!(
            "input index of {} bytes",
            index.len()
        )));
    };
    let index = usize::from(index);
    if index >= package.parts.len() {
        return Err(FhevmError::PadUnmask(format!("no input {index}")));
    }
    Ok((package, index))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A package with `parts` and `ciphertext`, and a label from its witness.
    pub(crate) fn package_bytes(prf_id: u16, parts: &[u8], ciphertext: &[u8]) -> Vec<u8> {
        let mut witness = vec![PAD_WITNESS_VERSION];
        witness.extend(prf_id.to_be_bytes());
        witness.extend(12345u64.to_be_bytes());
        witness.extend([0x11; 20]);
        witness.extend([0x22; 20]);
        witness.extend(1_900_000_000u64.to_be_bytes());
        witness.push(2);
        witness.extend([0x33; 40]);
        witness.extend([0x55; NONCE_BYTES]);
        let label = keccak256([PAD_LABEL_DOMAIN_SEPARATOR, witness.as_slice()].concat());
        let mut bytes = vec![PAD_PACKAGE_TYPE];
        bytes.extend(witness);
        bytes.extend(label);
        bytes.push(u8::try_from(parts.len()).unwrap());
        bytes.extend(parts);
        bytes.extend(ciphertext);
        bytes
    }

    #[test]
    fn parses_a_package() {
        // ebool, euint16, eaddress: 1 + 2 + 20 bytes.
        let bytes = package_bytes(7, &[0, 3, 7], &[0xaa; 23]);
        let package = PadPackage::parse(&bytes).unwrap();
        assert_eq!(package.prf_id, 7);
        assert_eq!(package.chain_id, 12345);
        assert_eq!(package.label, B256::from_slice(&bytes[117..149]));
        let parts: Vec<_> = package
            .parts
            .iter()
            .map(|p| (p.fhe_type, p.offset, p.len))
            .collect();
        assert_eq!(parts, vec![(0, 0, 1), (3, 1, 2), (7, 3, 20)]);
        assert_eq!(package.ciphertext, vec![0xaa; 23]);
        assert_eq!(package.pad_bits(), 184);
    }

    #[test]
    fn refuses_bad_packages() {
        let good = package_bytes(7, &[5], &[0; 8]);
        let parse = |bytes: &[u8]| PadPackage::parse(bytes).unwrap_err();

        let mut bad = good.clone();
        bad[0] = 1;
        assert_eq!(parse(&bad), PadPackageError::UnsupportedPackageType(1));
        bad = good.clone();
        bad[OFFSET_WITNESS_VERSION] = 1;
        assert_eq!(parse(&bad), PadPackageError::UnsupportedWitnessVersion(1));
        // A changed witness field no longer gives the label.
        bad = good.clone();
        bad[OFFSET_CHAIN_ID] ^= 1;
        assert!(matches!(parse(&bad), PadPackageError::LabelMismatch { .. }));
        assert_eq!(
            parse(&good[..good.len() - 1]),
            PadPackageError::CiphertextLengthMismatch {
                actual: 7,
                expected: 8
            }
        );
        assert_eq!(parse(&good[..40]), PadPackageError::Malformed);
        assert_eq!(
            parse(&package_bytes(7, &[1], &[0])),
            PadPackageError::UnknownPartType {
                index: 0,
                part_type: 1
            }
        );
        assert_eq!(parse(&package_bytes(7, &[], &[])), PadPackageError::NoParts);
        assert_eq!(
            parse(&package_bytes(7, &[8; 9], &[0; 288])),
            PadPackageError::TooLarge(2304)
        );
    }
}
