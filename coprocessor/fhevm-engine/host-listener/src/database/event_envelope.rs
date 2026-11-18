#[path = "abi_envelope_preflight.rs"]
mod abi_envelope_preflight;

use alloy::primitives::Log;
use alloy::sol_types::SolEventInterface;

/// An ABI decoder may fail for either an unrelated signature or a malformed
/// payload of an event we own. The latter must stop block ingestion so that
/// the cursor cannot advance past a partially observed canonical block.
pub enum DecodedEvent<T> {
    Decoded(Log<T>),
    Unknown,
    MalformedKnown,
}

pub fn classify<T: SolEventInterface>(
    log: &Log,
    known_selectors: &[[u8; 32]],
) -> DecodedEvent<T> {
    let known = log
        .data
        .topics()
        .first()
        .is_some_and(|topic| known_selectors.contains(&topic.0));
    if known && !abi_envelope_preflight::dynamic_event_payload_fits(log) {
        return DecodedEvent::MalformedKnown;
    }
    if let Ok(event) = T::decode_log(log) {
        return DecodedEvent::Decoded(event);
    }
    if known {
        DecodedEvent::MalformedKnown
    } else {
        DecodedEvent::Unknown
    }
}
