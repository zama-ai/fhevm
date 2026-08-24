//! The canonical Solana `hostPayload` carriage: the one byte layout of a user-decryption
//! request between the gateway event and this connector.
//!
//! The gateway's host-generic entry (`userDecryptionRequestV2`) carries everything
//! host-specific as one opaque `hostPayload`; the gateway never reads a byte of it. This
//! module is the single home of that payload's byte layout on the Rust side:
//!
//! ```text
//! hostPayload = HOST_PAYLOAD_VERSION (1 byte) ‖ borsh(body)
//! ```
//!
//! where the body mirrors [`SolanaUserDecryptRequestWire`] field for field over borsh
//! primitives. The mirror exists only as the codec definition — its field order IS the
//! layout — and the conversion in both directions destructures both structs exhaustively,
//! so adding a field to the wire form without deciding its place in the canon is a compile
//! error here, never a silent omission. The canonical bytes themselves are frozen by the
//! normative connector-auth vectors; the client-side encoder is borsh-js over the same
//! schema (hand-mirrored across languages — the two layouts must change together).
//!
//! The typed handle list of the gateway event is NOT part of the payload: the payload
//! carries its own entries, and [`check_handle_list_parity`] admits a payload only when its
//! handle list matches the typed `ctHandles` exactly, in order and count. That parity is
//! load-bearing: the gateway's bit budget is enforced on the typed handles, and the KMS
//! response linker binds their exact order and count — a payload that named different
//! handles would be budgeted on one list and authorized on another.

use super::request::{SolanaHandleEntryWire, SolanaUserDecryptRequestWire};
use crate::core::solana_acl::HandleBytes;
use borsh::{BorshDeserialize, BorshSerialize};
use zama_solana_permit::PermitWireFields;

/// The one known `hostPayload` version byte.
pub const HOST_PAYLOAD_VERSION: u8 = 0x01;

/// The borsh body of a `hostPayload`, mirroring [`SolanaUserDecryptRequestWire`] field for
/// field over primitives. The field order below IS the canonical layout — the borsh-js
/// schema on the client side is written against this struct, and the normative
/// connector-auth vectors freeze the resulting bytes.
#[derive(BorshSerialize, BorshDeserialize)]
struct HostPayloadBody {
    user_pubkey: Vec<u8>,
    transport_key: Vec<u8>,
    allowed_acl_domain_keys: Vec<Vec<u8>>,
    start_timestamp: u64,
    duration_seconds: u64,
    verifying_program_id: Vec<u8>,
    chain_id: u64,
    extra_data: Vec<u8>,
    signature: Vec<u8>,
    handles: Vec<HostPayloadEntry>,
}

/// One handle entry of the body, mirroring [`SolanaHandleEntryWire`].
#[derive(BorshSerialize, BorshDeserialize)]
struct HostPayloadEntry {
    handle: Vec<u8>,
    owner: Vec<u8>,
    encrypted_value_id: Vec<u8>,
    proof_leaf_count: u64,
    access_proof: Vec<u8>,
}

impl From<&SolanaUserDecryptRequestWire> for HostPayloadBody {
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
            allowed_acl_domain_keys,
            start_timestamp,
            duration_seconds,
            verifying_program_id,
            chain_id,
            extra_data,
        } = permit;

        Self {
            user_pubkey: user_pubkey.clone(),
            transport_key: transport_key.clone(),
            allowed_acl_domain_keys: allowed_acl_domain_keys.clone(),
            start_timestamp: *start_timestamp,
            duration_seconds: *duration_seconds,
            verifying_program_id: verifying_program_id.clone(),
            chain_id: *chain_id,
            extra_data: extra_data.clone(),
            signature: signature.clone(),
            handles: handles.iter().map(HostPayloadEntry::from).collect(),
        }
    }
}

impl From<&SolanaHandleEntryWire> for HostPayloadEntry {
    fn from(entry: &SolanaHandleEntryWire) -> Self {
        let SolanaHandleEntryWire {
            handle,
            owner,
            encrypted_value_id,
            proof_leaf_count,
            access_proof,
        } = entry;

        Self {
            handle: handle.clone(),
            owner: owner.clone(),
            encrypted_value_id: encrypted_value_id.clone(),
            proof_leaf_count: *proof_leaf_count,
            access_proof: access_proof.clone(),
        }
    }
}

impl From<HostPayloadBody> for SolanaUserDecryptRequestWire {
    fn from(body: HostPayloadBody) -> Self {
        let HostPayloadBody {
            user_pubkey,
            transport_key,
            allowed_acl_domain_keys,
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
                allowed_acl_domain_keys,
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

impl From<HostPayloadEntry> for SolanaHandleEntryWire {
    fn from(entry: HostPayloadEntry) -> Self {
        let HostPayloadEntry {
            handle,
            owner,
            encrypted_value_id,
            proof_leaf_count,
            access_proof,
        } = entry;

        Self {
            handle,
            owner,
            encrypted_value_id,
            proof_leaf_count,
            access_proof,
        }
    }
}

/// Why a `hostPayload` blob was refused, or why it does not belong to its event.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HostPayloadError {
    /// The payload is empty or its version byte names an unknown layout.
    #[error("host payload version {version:?} is not a known layout")]
    UnknownVersion {
        /// The received version byte; `None` for an empty payload.
        version: Option<u8>,
    },
    /// The body after the version byte does not decode as the canonical layout.
    #[error("host payload body does not decode as the canonical layout: {reason}")]
    MalformedBody {
        /// What the decoder tripped over.
        reason: String,
    },
    /// Valid body followed by bytes the layout does not account for.
    #[error("host payload carries {trailing} trailing byte(s) after the canonical body")]
    TrailingBytes {
        /// How many bytes remained.
        trailing: usize,
    },
    /// The payload's handle list is not the event's typed handle list.
    #[error(
        "host payload names {payload_handles} handle(s) but the event carries \
         {event_handles}; the lists must match exactly, in order and count"
    )]
    HandleListMismatch {
        /// Entry count inside the payload.
        payload_handles: usize,
        /// Typed handle count on the event.
        event_handles: usize,
    },
}

/// Why the canonical `hostPayload` bytes could not be produced.
#[derive(Debug, thiserror::Error)]
pub enum HostPayloadEncodeError {
    /// The borsh body refused to serialize. The `Vec` writer itself cannot fail, so in
    /// practice this is a collection longer than borsh's `u32` length prefix.
    #[error("host payload body does not serialize: {reason}")]
    BodySerialization {
        /// What the serializer tripped over.
        reason: String,
    },
}

/// Encodes a wire request into the canonical `hostPayload` bytes.
///
/// The encoder lives next to the decoder so the normative vector generator and the tests
/// exercise the same layout the decoder reads — there is no second implementation to drift.
///
/// The only failure borsh can produce for this body is a collection longer than its
/// `u32` length prefix. Every list is bounded far below that upstream, but the bound
/// lives in other modules, so the failure is propagated rather than assumed away.
pub fn encode_host_payload(
    wire: &SolanaUserDecryptRequestWire,
) -> Result<Vec<u8>, HostPayloadEncodeError> {
    let body = HostPayloadBody::from(wire);
    let mut bytes = vec![HOST_PAYLOAD_VERSION];
    borsh::to_writer(&mut bytes, &body).map_err(|source| {
        HostPayloadEncodeError::BodySerialization {
            reason: source.to_string(),
        }
    })?;
    Ok(bytes)
}

/// Decodes canonical `hostPayload` bytes into the wire request, strictly: one known
/// version, a body that consumes every remaining byte, nothing tolerated after it.
pub fn decode_host_payload(bytes: &[u8]) -> Result<SolanaUserDecryptRequestWire, HostPayloadError> {
    let (version, mut body_bytes) = match bytes.split_first() {
        Some((version, body)) => (*version, body),
        None => return Err(HostPayloadError::UnknownVersion { version: None }),
    };
    if version != HOST_PAYLOAD_VERSION {
        return Err(HostPayloadError::UnknownVersion {
            version: Some(version),
        });
    }

    let body = HostPayloadBody::deserialize(&mut body_bytes).map_err(|decode_error| {
        HostPayloadError::MalformedBody {
            reason: decode_error.to_string(),
        }
    })?;
    if !body_bytes.is_empty() {
        return Err(HostPayloadError::TrailingBytes {
            trailing: body_bytes.len(),
        });
    }

    Ok(SolanaUserDecryptRequestWire::from(body))
}

/// Admits a decoded payload only when its handle list is exactly the event's typed
/// `ctHandles` — same handles, same order, same count (duplicates included).
pub fn check_handle_list_parity(
    ct_handles: &[HandleBytes],
    wire: &SolanaUserDecryptRequestWire,
) -> Result<(), HostPayloadError> {
    let mismatch = || HostPayloadError::HandleListMismatch {
        payload_handles: wire.handles.len(),
        event_handles: ct_handles.len(),
    };

    if wire.handles.len() != ct_handles.len() {
        return Err(mismatch());
    }
    for (entry, ct_handle) in wire.handles.iter().zip(ct_handles) {
        if entry.handle.as_slice() != ct_handle.as_slice() {
            return Err(mismatch());
        }
    }
    Ok(())
}
