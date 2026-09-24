//! The canonical carriage of a Solana user-decryption request blob: the one byte layout between
//! the party that submits the gateway transaction and the party that authorizes it.
//!
//! The gateway's host-generic entry types the handles, transport key, validity window and extra
//! data, and carries the rest of the request as one opaque field it never reads. This module is
//! the single home of that field's layout on the Rust side:
//!
//! ```text
//! bytes = SOLANA_REQUEST_VERSION (1 byte) ‖ borsh(body)
//! ```
//!
//! where the body is [`SolanaRequestBlob`] itself: its field order is the layout.
//!
//! The blob carries no field the gateway types, so the two can never disagree:
//! [`crate::assemble_solana_request`] joins them into the full request.

use crate::assemble::SolanaRequestBlob;
use borsh::BorshDeserialize;

/// The one known layout version byte. `0x01` carried client-built proofs, `0x02` named
/// per-value accounts, and `0x03` repeated the fields the gateway types; no obsolete layout is
/// decoded.
pub const SOLANA_REQUEST_VERSION: u8 = 0x04;

/// Why a request blob was refused.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SolanaRequestDecodeError {
    /// The blob is empty or its version byte names an unknown layout.
    #[error("solana request version {version:?} is not a known layout")]
    UnknownVersion {
        /// The received version byte; `None` for an empty blob.
        version: Option<u8>,
    },
    /// The body after the version byte does not decode as the canonical layout.
    #[error("solana request body does not decode as the canonical layout: {reason}")]
    MalformedBody {
        /// What the decoder tripped over.
        reason: String,
    },
    /// Valid body followed by bytes the layout does not account for.
    #[error("solana request carries {trailing} trailing byte(s) after the canonical body")]
    TrailingBytes {
        /// How many bytes remained.
        trailing: usize,
    },
}

/// Why the canonical bytes could not be produced.
#[derive(Debug, thiserror::Error)]
pub enum SolanaRequestEncodeError {
    /// The borsh body refused to serialize. The `Vec` writer itself cannot fail, so in
    /// practice this is a collection longer than borsh's `u32` length prefix.
    #[error("solana request body does not serialize: {reason}")]
    BodySerialization {
        /// What the serializer tripped over.
        reason: String,
    },
}

/// Encodes a request blob into the canonical bytes.
///
/// The only failure borsh can produce for this body is a collection longer than its
/// `u32` length prefix. Every list is bounded far below that upstream, but the bound
/// lives in other modules, so the failure is propagated rather than assumed away.
pub fn encode_solana_request(
    blob: &SolanaRequestBlob,
) -> Result<Vec<u8>, SolanaRequestEncodeError> {
    let mut bytes = vec![SOLANA_REQUEST_VERSION];
    borsh::to_writer(&mut bytes, blob).map_err(|source| {
        SolanaRequestEncodeError::BodySerialization {
            reason: source.to_string(),
        }
    })?;
    Ok(bytes)
}

/// Decodes canonical bytes into the request blob, strictly: one known version, a body that
/// consumes every remaining byte, nothing tolerated after it.
pub fn decode_solana_request(bytes: &[u8]) -> Result<SolanaRequestBlob, SolanaRequestDecodeError> {
    let (version, mut body_bytes) = match bytes.split_first() {
        Some((version, body)) => (*version, body),
        None => return Err(SolanaRequestDecodeError::UnknownVersion { version: None }),
    };
    if version != SOLANA_REQUEST_VERSION {
        return Err(SolanaRequestDecodeError::UnknownVersion {
            version: Some(version),
        });
    }

    let blob = SolanaRequestBlob::deserialize(&mut body_bytes).map_err(|decode_error| {
        SolanaRequestDecodeError::MalformedBody {
            reason: decode_error.to_string(),
        }
    })?;
    if !body_bytes.is_empty() {
        return Err(SolanaRequestDecodeError::TrailingBytes {
            trailing: body_bytes.len(),
        });
    }

    Ok(blob)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assemble::SolanaEntryClaims;

    /// Every field distinct and non-empty, so a field the encoder dropped changes nothing.
    fn blob() -> SolanaRequestBlob {
        let entry = |seed: u8| SolanaEntryClaims {
            owner_address: vec![seed; 32],
            encrypted_store: vec![seed + 1; 32],
        };
        SolanaRequestBlob {
            user_address: vec![1; 32],
            allowed_scopes: vec![vec![3; 64]],
            verifying_program_id: vec![6; 32],
            signature: vec![9; 64],
            entries: vec![entry(10), entry(20)],
        }
    }

    #[test]
    fn every_blob_field_reaches_the_canonical_bytes() {
        let base = blob();
        let baseline = encode_solana_request(&base).unwrap();
        assert_eq!(baseline[0], SOLANA_REQUEST_VERSION);
        assert_eq!(decode_solana_request(&baseline), Ok(base.clone()));

        let mutations: [fn(&mut SolanaRequestBlob); 6] = [
            |b| b.user_address[0] ^= 1,
            |b| b.allowed_scopes[0][0] ^= 1,
            |b| b.verifying_program_id[0] ^= 1,
            |b| b.signature[0] ^= 1,
            |b| b.entries[0].owner_address[0] ^= 1,
            |b| b.entries[1].encrypted_store[0] ^= 1,
        ];
        for (index, mutate) in mutations.iter().enumerate() {
            let mut variant = base.clone();
            mutate(&mut variant);
            let bytes = encode_solana_request(&variant).unwrap();
            assert_ne!(bytes, baseline, "mutation {index} left the bytes unchanged");
            assert_eq!(decode_solana_request(&bytes), Ok(variant));
        }
    }

    #[test]
    fn the_decoder_is_strict() {
        let bytes = encode_solana_request(&blob()).unwrap();
        assert_eq!(
            decode_solana_request(&[]),
            Err(SolanaRequestDecodeError::UnknownVersion { version: None })
        );
        let mut obsolete = bytes.clone();
        obsolete[0] = 0x03;
        assert_eq!(
            decode_solana_request(&obsolete),
            Err(SolanaRequestDecodeError::UnknownVersion { version: Some(3) })
        );
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(
            decode_solana_request(&trailing),
            Err(SolanaRequestDecodeError::TrailingBytes { trailing: 1 })
        );
        // The user address's length prefix follows the version byte.
        let mut length_lie = bytes.clone();
        length_lie[1..5].copy_from_slice(&u32::MAX.to_le_bytes());
        for malformed in [
            &bytes[..bytes.len() / 2],
            &bytes[..bytes.len() - 1],
            &length_lie,
        ] {
            assert!(matches!(
                decode_solana_request(malformed),
                Err(SolanaRequestDecodeError::MalformedBody { .. })
            ));
        }
    }
}
