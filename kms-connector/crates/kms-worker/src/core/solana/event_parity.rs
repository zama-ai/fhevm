//! Parity between the gateway event's typed fields and the signed request they claim to carry.
//!
//! The host-generic gateway entry takes a handful of fields typed — because the gateway itself
//! consumes them — and everything else as one opaque blob it never reads. Every typed field
//! here also exists inside that blob, signed. The gateway cannot compare the two: the blob is
//! opaque to it. This module is where the comparison happens, and it is the only place it
//! happens.
//!
//! # Why one function rather than a check per site
//!
//! Each of these fields is unsigned on the event and signed in the permit, so a relayer can
//! substitute any of them without invalidating the signature. What a substitution costs
//! differs per field — a swapped window shifts the fee off the authorized request, a swapped
//! routing serves the request under a KMS pair the user never consented to, a swapped
//! transport key strands the response — but the shape of the defence is identical: the typed
//! value must equal the signed one, and a mismatch is wrong forever, so it is terminal.
//!
//! Collecting them here is not tidiness. Scattered across the request path, these checks
//! answered "is this field checked?" one call site at a time, and there was no place a reader
//! could look to answer "are they *all* checked?" — which is how the transport key came to be
//! the one that was not.
//!
//! # Why these four and no others
//!
//! The gateway entry carries exactly four fields that also exist inside the signed blob, and
//! all four are here. It used to carry two more — a declared ACL-scope length and a host-kind
//! discriminator — and neither was a duplicate worth comparing: the first could not widen
//! anything (a request was admitted only when the declaration equalled the signed list, so a
//! truthful one added nothing and a false one was refused), and the second had no signed
//! counterpart at all. Both are gone from the entry, together with the checks that read them.

use crate::core::solana_acl::HandleBytes;
use alloy::primitives::U256;
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_4 as UserDecryptionRequestV3;
use zama_solana_permit::PermitFields;
use zama_solana_request::{check_handle_list_parity, SolanaUserDecryptRequestWire};

/// Which duplicated field disagreed with the signed request, and how.
///
/// Every variant is terminal: no later observation makes a typed field start matching the
/// bytes a wallet already signed.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum EventParityFailure {
    /// The blob's handle list is not the event's typed `ctHandles`.
    #[error("host payload handles do not match the event handles: {reason}")]
    Handles {
        /// What the shared parity rule said.
        reason: String,
    },
    /// The event's `publicKey` is not the permit's signed transport key.
    #[error(
        "event publicKey 0x{event} is not the signed transport key 0x{signed}; the KMS seals \
         to the signed key and the gateway verifies the response against the event's, so the \
         response could never be accepted"
    )]
    TransportKey {
        /// The unsigned key on the event, hex.
        event: String,
        /// The signed key in the permit, hex.
        signed: String,
    },
    /// The event's `requestValidity` is not the permit's signed window.
    #[error(
        "event requestValidity (start {event_start}, duration {event_duration}) does not match \
         the signed permit window (start {signed_start}, duration {signed_duration})"
    )]
    ValidityWindow {
        /// Start timestamp on the event.
        event_start: U256,
        /// Duration on the event.
        event_duration: U256,
        /// Start timestamp signed in the permit.
        signed_start: u64,
        /// Duration signed in the permit.
        signed_duration: u64,
    },
    /// The event's `extraData` is not the permit's signed KMS routing.
    #[error("event extraData 0x{event} does not match the signed KMS routing 0x{signed}")]
    KmsRouting {
        /// The routing bytes on the event, hex.
        event: String,
        /// The canonical rendering of the signed routing, hex.
        signed: String,
    },
}

/// Compares every field the gateway event and the signed request both carry.
///
/// Called before authorization, so a mismatch costs no RPC read and no KMS work. It cannot
/// run before the fee: the gateway collected that without being able to read the blob at all.
pub fn check_event_permit_parity(
    event: &UserDecryptionRequestV3,
    ct_handles: &[HandleBytes],
    wire: &SolanaUserDecryptRequestWire,
    permit: &PermitFields,
) -> Result<(), EventParityFailure> {
    // The handle lists, in order and count. Load-bearing: the gateway enforces the bit budget
    // on the typed handles and the KMS response linker binds their exact order and count, so a
    // blob free to name other handles would be budgeted on one list and authorized on another.
    check_handle_list_parity(ct_handles, wire).map_err(|reason| EventParityFailure::Handles {
        reason: reason.to_string(),
    })?;

    // The transport key. The KMS seals the plaintext to the key signed in the permit, while
    // the gateway stores the event's `publicKey` at request time and verifies the KMS response
    // against that one. Diverge and the request is unfinishable: the response is valid, the
    // gateway rejects it, and the fee is already spent. Equality is also what lets the seal
    // target be taken from the permit without the two ever naming different keys.
    if event.publicKey.as_ref() != permit.transport_key().as_bytes() {
        return Err(EventParityFailure::TransportKey {
            event: alloy::hex::encode(&event.publicKey),
            signed: alloy::hex::encode(permit.transport_key().as_bytes()),
        });
    }

    // The validity window. The gateway charged its fee against the typed window and forwarded
    // it to monitoring and the SDK, but the window that authorizes is the signed one. If they
    // disagree, the fee, the readiness view and the authorization would each be about a
    // different window. The EVM path gets this parity for free — there the typed window is
    // inside the EIP-712 digest the signature covers; here it must be checked explicitly.
    if event.requestValidity.startTimestamp != U256::from(permit.start_timestamp())
        || event.requestValidity.durationSeconds != U256::from(permit.duration_seconds())
    {
        return Err(EventParityFailure::ValidityWindow {
            event_start: event.requestValidity.startTimestamp,
            event_duration: event.requestValidity.durationSeconds,
            signed_start: permit.start_timestamp(),
            signed_duration: permit.duration_seconds(),
        });
    }

    // The KMS routing. `prepare_decryption_request` parses the context/epoch pair out of the
    // typed `extraData` (and the gateway routed its fee by it), but the pair the user consented
    // to is signed in the permit. A mismatch would authorize under one pair and serve under the
    // other. Compared against the canonical rendering rather than the blob's raw bytes: the
    // signed routing has exactly one rendering, which is what an honest relayer copies over.
    let signed_routing = permit.extra_data().to_extra_data();
    if event.extraData.as_ref() != signed_routing.as_slice() {
        return Err(EventParityFailure::KmsRouting {
            event: alloy::hex::encode(&event.extraData),
            signed: alloy::hex::encode(&signed_routing),
        });
    }

    Ok(())
}
