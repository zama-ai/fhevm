//! The canonical carriage of a Solana user-decryption request: the one byte layout between
//! the party that submits the gateway transaction and the party that authorizes it.
//!
//! The gateway's host-generic entry carries the whole request as one opaque field; the
//! gateway never reads a byte of it. This module is the single home of that layout on the
//! Rust side:
//!
//! ```text
//! bytes = SOLANA_REQUEST_VERSION (1 byte) ‖ borsh(body)
//! ```
//!
//! where the body mirrors [`SolanaUserDecryptRequestWire`] field for field over borsh
//! primitives. The mirror exists only as the codec definition — its field order IS the
//! layout — and the conversion in both directions destructures both structs exhaustively,
//! so adding a field to the wire form without deciding its place in the canon is a compile
//! error here, never a silent omission.
//!
//! There is one encoder and one decoder, and both consumers call them: a field added on
//! either side is a compile error on the other, rather than a byte diff discovered by a
//! fixture test.
//!
//! The gateway event also carries the handles, transport key, window and KMS routing in
//! cleartext. A consumer that authorizes an event uses those cleartext values in place of the
//! request's copies, so the handles that are budgeted, decrypted and authorized are the same list.

use crate::wire::{SolanaHandleEntryWire, SolanaUserDecryptRequestWire};
use borsh::{BorshDeserialize, BorshSerialize};
use zama_solana_permit::PermitWireFields;

/// The one known layout version byte. `0x01` carried client-built proofs and `0x02` named
/// per-value accounts; neither obsolete layout is decoded.
pub const SOLANA_REQUEST_VERSION: u8 = 0x03;

/// The borsh body, mirroring [`SolanaUserDecryptRequestWire`] field for field over
/// primitives. The field order below IS the canonical layout.
#[derive(BorshSerialize, BorshDeserialize)]
struct RequestBody {
    user_pubkey: Vec<u8>,
    transport_key: Vec<u8>,
    allowed_scopes: Vec<Vec<u8>>,
    start_timestamp: u64,
    duration_seconds: u64,
    verifying_program_id: Vec<u8>,
    chain_id: u64,
    extra_data: Vec<u8>,
    signature: Vec<u8>,
    handles: Vec<RequestBodyEntry>,
}

/// One handle entry of the body, mirroring [`SolanaHandleEntryWire`].
#[derive(BorshSerialize, BorshDeserialize)]
struct RequestBodyEntry {
    handle: Vec<u8>,
    allowed_key: Vec<u8>,
    encrypted_store: Vec<u8>,
}

impl From<&SolanaUserDecryptRequestWire> for RequestBody {
    fn from(wire: &SolanaUserDecryptRequestWire) -> Self {
        // Exhaustive destructuring on purpose: a field added to the wire form (or to the
        // permit's transport form) fails to compile HERE, forcing a decision about its
        // place in the canonical layout instead of a silent omission from it.
        let SolanaUserDecryptRequestWire {
            permit,
            signature,
            handles,
        } = wire;
        let PermitWireFields {
            user_pubkey,
            transport_key,
            allowed_scopes,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            chain_id,
            extra_data,
        } = permit;

        Self {
            user_pubkey: user_pubkey.clone(),
            transport_key: transport_key.clone(),
            allowed_scopes: allowed_scopes.clone(),
            start_timestamp: *start_timestamp,
            duration_seconds: *duration_seconds,
            verifying_program_id: verifying_program_id.clone(),
            chain_id: *chain_id,
            extra_data: extra_data.clone(),
            signature: signature.clone(),
            handles: handles.iter().map(RequestBodyEntry::from).collect(),
        }
    }
}

impl From<&SolanaHandleEntryWire> for RequestBodyEntry {
    fn from(entry: &SolanaHandleEntryWire) -> Self {
        let SolanaHandleEntryWire {
            handle,
            allowed_key,
            encrypted_store,
        } = entry;

        Self {
            handle: handle.clone(),
            allowed_key: allowed_key.clone(),
            encrypted_store: encrypted_store.clone(),
        }
    }
}

impl From<RequestBody> for SolanaUserDecryptRequestWire {
    fn from(body: RequestBody) -> Self {
        let RequestBody {
            user_pubkey,
            transport_key,
            allowed_scopes,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            chain_id,
            extra_data,
            signature,
            handles,
        } = body;

        Self {
            permit: PermitWireFields {
                user_pubkey,
                transport_key,
                allowed_scopes,
                start_timestamp,
                duration_seconds,
                verifying_program_id,
                chain_id,
                extra_data,
            },
            signature,
            handles: handles
                .into_iter()
                .map(SolanaHandleEntryWire::from)
                .collect(),
        }
    }
}

impl From<RequestBodyEntry> for SolanaHandleEntryWire {
    fn from(entry: RequestBodyEntry) -> Self {
        let RequestBodyEntry {
            handle,
            allowed_key,
            encrypted_store,
        } = entry;

        Self {
            handle,
            allowed_key,
            encrypted_store,
        }
    }
}

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

/// Encodes a wire request into the canonical bytes.
///
/// The only failure borsh can produce for this body is a collection longer than its
/// `u32` length prefix. Every list is bounded far below that upstream, but the bound
/// lives in other modules, so the failure is propagated rather than assumed away.
pub fn encode_solana_request(
    wire: &SolanaUserDecryptRequestWire,
) -> Result<Vec<u8>, SolanaRequestEncodeError> {
    let body = RequestBody::from(wire);
    let mut bytes = vec![SOLANA_REQUEST_VERSION];
    borsh::to_writer(&mut bytes, &body).map_err(|source| {
        SolanaRequestEncodeError::BodySerialization {
            reason: source.to_string(),
        }
    })?;
    Ok(bytes)
}

/// Decodes canonical bytes into the wire request, strictly: one known version, a body that
/// consumes every remaining byte, nothing tolerated after it.
pub fn decode_solana_request(
    bytes: &[u8],
) -> Result<SolanaUserDecryptRequestWire, SolanaRequestDecodeError> {
    let (version, mut body_bytes) = match bytes.split_first() {
        Some((version, body)) => (*version, body),
        None => return Err(SolanaRequestDecodeError::UnknownVersion { version: None }),
    };
    if version != SOLANA_REQUEST_VERSION {
        return Err(SolanaRequestDecodeError::UnknownVersion {
            version: Some(version),
        });
    }

    let body = RequestBody::deserialize(&mut body_bytes).map_err(|decode_error| {
        SolanaRequestDecodeError::MalformedBody {
            reason: decode_error.to_string(),
        }
    })?;
    if !body_bytes.is_empty() {
        return Err(SolanaRequestDecodeError::TrailingBytes {
            trailing: body_bytes.len(),
        });
    }

    Ok(SolanaUserDecryptRequestWire::from(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SolanaHandleEntryWire;

    /// Every field distinct and non-empty, so a field the encoder dropped changes nothing.
    fn wire() -> SolanaUserDecryptRequestWire {
        let entry = |seed: u8| SolanaHandleEntryWire {
            handle: vec![seed; 32],
            allowed_key: vec![seed + 1; 32],
            encrypted_store: vec![seed + 2; 32],
        };
        SolanaUserDecryptRequestWire {
            permit: PermitWireFields {
                user_pubkey: vec![1; 32],
                transport_key: vec![2; 32],
                allowed_scopes: vec![vec![3; 64]],
                start_timestamp: 4,
                duration_seconds: 5,
                verifying_program_id: vec![6; 32],
                chain_id: 7,
                extra_data: vec![8; 65],
            },
            signature: vec![9; 64],
            handles: vec![entry(10), entry(20)],
        }
    }

    #[test]
    fn every_wire_field_reaches_the_canonical_bytes() {
        let base = wire();
        let baseline = encode_solana_request(&base).unwrap();
        assert_eq!(baseline[0], SOLANA_REQUEST_VERSION);
        assert_eq!(decode_solana_request(&baseline), Ok(base.clone()));

        let mutations: [fn(&mut SolanaUserDecryptRequestWire); 12] = [
            |w| w.permit.user_pubkey[0] ^= 1,
            |w| w.permit.transport_key[0] ^= 1,
            |w| w.permit.allowed_scopes[0][0] ^= 1,
            |w| w.permit.start_timestamp += 1,
            |w| w.permit.duration_seconds += 1,
            |w| w.permit.verifying_program_id[0] ^= 1,
            |w| w.permit.chain_id += 1,
            |w| w.permit.extra_data[0] ^= 1,
            |w| w.signature[0] ^= 1,
            |w| w.handles[0].handle[0] ^= 1,
            |w| w.handles[0].allowed_key[0] ^= 1,
            |w| w.handles[1].encrypted_store[0] ^= 1,
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
        let bytes = encode_solana_request(&wire()).unwrap();
        assert_eq!(
            decode_solana_request(&[]),
            Err(SolanaRequestDecodeError::UnknownVersion { version: None })
        );
        let mut obsolete = bytes.clone();
        obsolete[0] = 0x02;
        assert_eq!(
            decode_solana_request(&obsolete),
            Err(SolanaRequestDecodeError::UnknownVersion { version: Some(2) })
        );
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(
            decode_solana_request(&trailing),
            Err(SolanaRequestDecodeError::TrailingBytes { trailing: 1 })
        );
        // The user pubkey's length prefix follows the version byte.
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
