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
    if let Ok(event) = T::decode_log(log) {
        return DecodedEvent::Decoded(event);
    }
    if log
        .data
        .topics()
        .first()
        .is_some_and(|topic| known_selectors.contains(&topic.0))
    {
        DecodedEvent::MalformedKnown
    } else {
        DecodedEvent::Unknown
    }
}
